//! Orchestration: load -> (cache?) -> generate -> compile -> run -> report.

use crate::compile::{self, BatchInput, CompileOutcome};
use crate::generate::{self, GenOutcome};
use crate::results::{Bucket, CaseOutcome, CaseResult, GroupResult, Results};
use crate::runner::{self, RunCase, RunOutcome};
use crate::suite::{self, Group};
use crate::summary;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Bump when the pipeline changes in a way that invalidates cached results
/// (e.g. the runner's deserialize path or the bucket rules).
const HARNESS_REV: &str = "1";

const GENERATE_TIMEOUT: Duration = Duration::from_secs(60);
const RUN_TIMEOUT: Duration = Duration::from_secs(120);

pub struct RunOpts {
    pub version: &'static str,
    pub draft: String,
    pub filter: Option<String>,
    pub optional: bool,
    pub no_cache: bool,
    pub out: Option<PathBuf>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn typify_version(label: &str) -> &'static str {
    match label {
        "typify1" => generate::TYPIFY1_VERSION,
        other => panic!("unknown version label {other}"),
    }
}

/// Cache key for a group: everything that determines its result,
/// including its id (two files can hold byte-identical groups).
fn group_hash(group: &Group, typify: &str, cases: &[(usize, &suite::Case)]) -> String {
    let mut h = Sha256::new();
    h.update(HARNESS_REV);
    h.update([0]);
    h.update(typify);
    h.update([0]);
    h.update(group.id());
    h.update([0]);
    // serde_json::Value serializes object keys sorted, so this is canonical.
    h.update(serde_json::to_vec(&group.schema).unwrap());
    h.update([0]);
    for (i, c) in cases {
        h.update(i.to_string());
        h.update([0]);
        h.update(serde_json::to_vec(&c.data).unwrap());
        h.update([c.valid as u8]);
    }
    hex::encode(h.finalize())
}

/// Module key for a group's generated code, which depends only on the
/// schema and the typify version. Identical schemas share one module.
fn module_key(group: &Group, typify: &str) -> String {
    let mut h = Sha256::new();
    h.update(typify);
    h.update([0]);
    h.update(serde_json::to_vec(&group.schema).unwrap());
    format!("g_{}", &hex::encode(h.finalize())[..16])
}

/// Per-group state as it flows through the pipeline.
struct Item<'a> {
    group: &'a Group,
    /// (index within the group, case) for the cases selected by the filter.
    cases: Vec<(usize, &'a suite::Case)>,
    hash: String,
    key: String,
    generated: Option<GenOutcome>,
    result: Option<GroupResult>,
}

pub fn run(opts: RunOpts) -> Result<()> {
    let started = Instant::now();
    let typify = typify_version(opts.version);
    let work = repo_root().join("work");
    let target_dir = work.join("target");
    let version_dir = work.join(opts.version);
    let cache_dir = version_dir.join("cache");
    let gen_dir = version_dir.join("generated");
    std::fs::create_dir_all(&cache_dir)?;
    std::fs::create_dir_all(&gen_dir)?;

    let groups = suite::load_draft(&opts.draft, opts.optional)?;
    let filter = opts.filter.as_deref();
    let selected: Vec<&Group> = groups.iter().filter(|g| filter.is_none_or(|f| g.matches(f))).collect();
    anyhow::ensure!(!selected.is_empty(), "no groups match");
    eprintln!(
        "{}: {} of {} groups selected ({} cases)",
        opts.draft,
        selected.len(),
        groups.len(),
        selected.iter().map(|g| g.filtered_cases(filter).count()).sum::<usize>()
    );

    let mut items: Vec<Item> = selected
        .iter()
        .map(|g| {
            let cases: Vec<_> = g.filtered_cases(filter).collect();
            let hash = group_hash(g, typify, &cases);
            Item { group: g, cases, key: module_key(g, typify), hash, generated: None, result: None }
        })
        .collect();

    // Cache lookup.
    let mut cached = 0;
    if !opts.no_cache {
        for item in &mut items {
            let path = cache_dir.join(format!("{}.json", item.hash));
            if let Ok(r) = std::fs::read_to_string(&path).map_err(|_| ()).and_then(|t| serde_json::from_str::<GroupResult>(&t).map_err(|_| ())) {
                item.result = Some(r);
                cached += 1;
            }
        }
    }
    eprintln!("cache: {cached} hit, {} to run", items.len() - cached);

    // Step 1: generate. (typify's panic capture is serialized on a lock,
    // and generation is fast, so this is a plain loop.)
    let t = Instant::now();
    let mut inputs = Vec::new();
    let mut seen_keys = std::collections::BTreeSet::new();
    for item in items.iter_mut().filter(|i| i.result.is_none()) {
        let outcome = generate::generate(item.group, GENERATE_TIMEOUT);
        if let GenOutcome::Code { code, root } = &outcome {
            // Keep a copy for humans: work/<version>/generated/<key>.rs.
            let header = format!("// {}\n// root type: {}\n\n", item.group.id(), root);
            std::fs::write(gen_dir.join(format!("{}.rs", item.key)), format!("{header}{code}"))?;
            if seen_keys.insert(item.key.clone()) {
                inputs.push(BatchInput { key: item.key.clone(), code: code.clone(), root: root.clone() });
            }
        }
        item.generated = Some(outcome);
    }
    let n_gen = items.iter().filter(|i| i.generated.is_some()).count();
    eprintln!("generate: {n_gen} groups, {} produced code ({:.1?})", inputs.len(), t.elapsed());

    // Step 2: compile.
    let t = Instant::now();
    let compiled = if inputs.is_empty() {
        None
    } else {
        Some(compile::compile_batch(&version_dir.join("batch"), &target_dir, &inputs)?)
    };
    if let Some(c) = &compiled {
        let ok = c.outcomes.values().filter(|o| **o == CompileOutcome::Ok).count();
        eprintln!("compile: {ok} of {} modules compiled ({:.1?})", c.outcomes.len(), t.elapsed());
    }

    // Step 3: run.
    let t = Instant::now();
    let mut run_cases = Vec::new();
    let mut case_slots: Vec<(usize, usize)> = Vec::new(); // (item index, position in item.cases)
    if let Some(c) = &compiled {
        for (ii, item) in items.iter().enumerate() {
            if c.outcomes.get(&item.key) != Some(&CompileOutcome::Ok) {
                continue;
            }
            for (pos, (_, case)) in item.cases.iter().enumerate() {
                run_cases.push(RunCase { key: item.key.clone(), data: case.data.clone() });
                case_slots.push((ii, pos));
            }
        }
    }
    let mut outcomes: BTreeMap<(usize, usize), RunOutcome> = BTreeMap::new();
    if let Some(runner_bin) = compiled.as_ref().and_then(|c| c.runner.as_ref()) {
        let results = runner::run_cases(runner_bin, &version_dir, &run_cases, RUN_TIMEOUT)?;
        anyhow::ensure!(results.len() == run_cases.len(), "runner returned {} outcomes for {} cases", results.len(), run_cases.len());
        for (slot, outcome) in case_slots.into_iter().zip(results) {
            outcomes.insert(slot, outcome);
        }
    }
    eprintln!("run: {} cases ({:.1?})", run_cases.len(), t.elapsed());

    // Assemble results for everything that wasn't cached.
    let mut fresh = 0;
    for (ii, item) in items.iter_mut().enumerate() {
        if item.result.is_some() {
            continue;
        }
        let group = item.group;
        let not_run = |cases: &[(usize, &suite::Case)]| -> Vec<CaseResult> {
            cases
                .iter()
                .map(|(_, c)| CaseResult { description: c.description.clone(), valid: c.valid, outcome: CaseOutcome::NotRun, error: None })
                .collect()
        };
        let (bucket, error, cases) = match item.generated.as_ref().expect("generated") {
            GenOutcome::Panic(msg) => (Bucket::GenerationPanic, Some(msg.clone()), not_run(&item.cases)),
            GenOutcome::Error(msg) => (Bucket::GenerationError, Some(msg.clone()), not_run(&item.cases)),
            GenOutcome::Code { .. } => {
                let compiled = compiled.as_ref().expect("compiled batch");
                match compiled.outcomes.get(&item.key) {
                    Some(CompileOutcome::Fail(msg)) => (Bucket::CompileFail, Some(msg.clone()), not_run(&item.cases)),
                    Some(CompileOutcome::Ok) => {
                        let cases: Vec<CaseResult> = item
                            .cases
                            .iter()
                            .enumerate()
                            .map(|(pos, (_, c))| {
                                let (outcome, error) = match outcomes.remove(&(ii, pos)).expect("every case has an outcome") {
                                    RunOutcome::Accept if c.valid => (CaseOutcome::Match, None),
                                    RunOutcome::Accept => (CaseOutcome::FalseAccept, None),
                                    RunOutcome::Reject(e) if c.valid => (CaseOutcome::FalseReject, Some(e)),
                                    RunOutcome::Reject(e) => (CaseOutcome::Match, Some(e)),
                                    RunOutcome::Panic(e) => (CaseOutcome::Panic, Some(e)),
                                };
                                CaseResult { description: c.description.clone(), valid: c.valid, outcome, error }
                            })
                            .collect();
                        let bucket = GroupResult::bucket_from_cases(&cases);
                        let error = cases.iter().find(|c| c.outcome != CaseOutcome::Match).map(|c| {
                            match c.outcome {
                                CaseOutcome::FalseAccept => format!("accepted `{}` though valid: false", c.description),
                                _ => format!("{}: {}", c.description, c.error.clone().unwrap_or_default()),
                            }
                        });
                        (bucket, error, cases)
                    }
                    None => anyhow::bail!("compile step reported nothing for {}", group.id()),
                }
            }
        };
        let result = GroupResult {
            id: group.id(),
            draft: group.draft.clone(),
            file: group.file.clone(),
            optional: group.optional,
            group: group.description.clone(),
            bucket,
            error,
            cases,
        };
        std::fs::write(cache_dir.join(format!("{}.json", item.hash)), serde_json::to_string(&result)?)?;
        item.result = Some(result);
        fresh += 1;
    }

    let results = Results {
        version: opts.version.to_string(),
        typify: typify.to_string(),
        generated_at: chrono_free_timestamp(),
        groups: items.into_iter().map(|i| i.result.expect("every item has a result")).collect(),
    };

    // A filtered run never overwrites the headline results; it goes to
    // work/<version>/filtered/ unless --out is given.
    let out_dir = match (&opts.out, filter) {
        (Some(o), _) => o.clone(),
        (None, None) => repo_root().join("results").join(opts.version),
        (None, Some(_)) => version_dir.join("filtered"),
    };
    results.save(&out_dir.join("results.json"))?;
    std::fs::write(out_dir.join("summary.md"), summary::render(&results))?;
    if filter.is_none() && opts.out.is_none() {
        write_scoreboard()?;
    }

    if filter.is_some() {
        print_detail(&results, &gen_dir, &items_keys(&results, &groups))?;
    }
    println!("{}", summary::render_terminal(&results));
    eprintln!(
        "{fresh} fresh, {cached} cached; wrote {} ({:.1?} total)",
        out_dir.join("results.json").display(),
        started.elapsed()
    );
    Ok(())
}

/// Regenerate `RESULTS.md` from every `results/*/results.json`.
pub fn write_scoreboard() -> Result<()> {
    let results_dir = repo_root().join("results");
    let mut all = Vec::new();
    if results_dir.is_dir() {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(&results_dir)?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        dirs.sort();
        for dir in dirs {
            let path = dir.join("results.json");
            if path.is_file() {
                all.push(Results::load(&path)?);
            }
        }
    }
    std::fs::write(repo_root().join("RESULTS.md"), summary::render_scoreboard(&all))?;
    Ok(())
}

/// Map group id -> generated module key, for pointing humans at the file.
fn items_keys(results: &Results, groups: &[Group]) -> BTreeMap<String, String> {
    let by_id: BTreeMap<String, &Group> = groups.iter().map(|g| (g.id(), g)).collect();
    results
        .groups
        .iter()
        .filter_map(|r| by_id.get(&r.id).map(|g| (r.id.clone(), module_key(g, &results.typify))))
        .collect()
}

/// Per-group, per-case detail for filtered runs.
fn print_detail(results: &Results, gen_dir: &Path, keys: &BTreeMap<String, String>) -> Result<()> {
    for g in &results.groups {
        println!("{} {}  [{}]", g.bucket.glyph(), g.id, g.bucket.name());
        if let Some(k) = keys.get(&g.id) {
            let path = gen_dir.join(format!("{k}.rs"));
            if path.exists() {
                println!("    generated: {}", path.display());
            }
        }
        if let Some(e) = &g.error {
            println!("    error: {}", first_lines(e, 12));
        }
        for c in &g.cases {
            let mark = match c.outcome {
                CaseOutcome::Match => "ok ",
                CaseOutcome::FalseAccept => "🙈 ",
                CaseOutcome::FalseReject => "🚫 ",
                CaseOutcome::Panic => "🔥 ",
                CaseOutcome::NotRun => "-- ",
            };
            let valid = if c.valid { "valid" } else { "invalid" };
            print!("    {mark}{} ({valid})", c.description);
            if let (false, Some(e)) = (c.outcome == CaseOutcome::Match, &c.error) {
                print!(": {}", first_lines(e, 1));
            }
            println!();
        }
        println!();
    }
    Ok(())
}

fn first_lines(s: &str, n: usize) -> String {
    let lines: Vec<&str> = s.lines().take(n).collect();
    let mut out = lines.join("\n    ");
    if s.lines().count() > n {
        out.push_str("\n    …");
    }
    out
}

fn chrono_free_timestamp() -> String {
    // RFC 3339 UTC without pulling in chrono for the harness itself.
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let days = secs / 86400;
    let (h, m, s) = ((secs % 86400) / 3600, (secs % 3600) / 60, secs % 60);
    // Civil-from-days (Howard Hinnant's algorithm).
    let z = days as i64 + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}
