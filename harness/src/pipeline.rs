//! `run`: for each generator flavor, load the suite, reuse stored records
//! whose hash matches, generate → build → run the rest, store, render.

use crate::generator::{self, BuildOutcome, Generator};
use crate::results::{version_slug, Bucket, CaseOutcome, CaseResult, GroupResult, Results};
use crate::runner::{self, RunCase, RunOutcome};
use crate::suite::{self, Group};
use crate::summary;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Bump when the pipeline changes in a way that invalidates stored results
/// (e.g. the runner protocol semantics or the bucket rules).
const HARNESS_REV: &str = "3";

const RUN_TIMEOUT: Duration = Duration::from_secs(120);

pub struct RunOpts {
    /// Generator directory name; `None` runs every generator.
    pub generator: Option<String>,
    /// Target within the generator; `None` runs every target it lists.
    pub target: Option<String>,
    pub drafts: Vec<String>,
    pub filter: Option<String>,
    pub optional: bool,
    pub no_cache: bool,
    pub out: Option<PathBuf>,
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("harness/ is under the repo root").to_path_buf()
}

/// Cache key for a group: everything that determines its record,
/// including its id (two files can hold byte-identical groups) and the
/// generator's own glue sources.
fn group_hash(group: &Group, identity: &str, glue: &str, cases: &[(usize, &suite::Case)]) -> String {
    let mut h = Sha256::new();
    h.update(HARNESS_REV);
    h.update([0]);
    h.update(identity);
    h.update([0]);
    h.update(glue);
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

/// Module key for a group's generated code, which depends on the generator
/// identity, the draft (a generator may read it), and the schema. Identical
/// schemas within a draft share one module.
fn module_key(group: &Group, identity: &str) -> String {
    let mut h = Sha256::new();
    h.update(identity);
    h.update([0]);
    h.update(&group.draft);
    h.update([0]);
    h.update(serde_json::to_vec(&group.schema).unwrap());
    format!("g_{}", &hex::encode(h.finalize())[..16])
}

/// One (generator, target) pair to run.
struct Flavor {
    generator: Generator,
    target: String,
}

fn flavors(opts: &RunOpts) -> Result<Vec<Flavor>> {
    let root = repo_root();
    let generators = match &opts.generator {
        Some(name) => vec![generator::load(&root, name)?],
        None => generator::discover(&root)?,
    };
    anyhow::ensure!(!generators.is_empty(), "no generators found under {}", root.join("generators").display());
    let mut out = Vec::new();
    for g in generators {
        let targets: Vec<String> = match (&opts.target, g.targets.is_empty()) {
            (Some(t), false) => {
                anyhow::ensure!(g.targets.contains(t), "{}: no target {t:?} (has {:?})", g.name, g.targets);
                vec![t.clone()]
            }
            (Some(t), true) => anyhow::bail!("{}: has no targets, so --target {t:?} doesn't apply", g.name),
            (None, false) => g.targets.clone(),
            (None, true) => vec![String::new()],
        };
        // Loading a generator once per target keeps `Flavor` self-contained.
        for t in targets {
            out.push(Flavor { generator: generator::load(&root, g.dir.file_name().unwrap().to_str().unwrap())?, target: t });
        }
    }
    Ok(out)
}

pub fn run(opts: RunOpts) -> Result<()> {
    for flavor in flavors(&opts)? {
        let identity = flavor.generator.version(&flavor.target)?;
        let glue = flavor.generator.source_hash()?;
        let mut last = None;
        for draft in &opts.drafts {
            last = Some(run_one(&opts, &flavor, &identity, &glue, draft)?);
        }
        // One terminal summary per flavor, covering every draft stored.
        if let Some(results) = last {
            println!("{}", summary::render_terminal(&results));
        }
    }
    Ok(())
}

/// Per-group state as it flows through the pipeline.
struct Item<'a> {
    group: &'a Group,
    /// (index within the group, case) for the cases selected by the filter.
    cases: Vec<(usize, &'a suite::Case)>,
    hash: String,
    key: String,
    /// Generation status, once known: ("ok" | "error" | "panic", message).
    generated: Option<(String, Option<String>)>,
    result: Option<GroupResult>,
}

fn run_one(opts: &RunOpts, flavor: &Flavor, identity: &str, glue: &str, draft: &str) -> Result<Results> {
    let started = Instant::now();
    let root = repo_root();
    let slug = version_slug(identity);
    let scratch = root.join("target").join("bakeoff").join(&slug);
    let gen_dir = scratch.join("generated");
    let batch_dir = scratch.join("batch");
    std::fs::create_dir_all(&gen_dir)?;
    let headline_dir = root.join("results").join(&slug);

    let groups = suite::load_draft(draft, opts.optional)?;
    let filter = opts.filter.as_deref();
    let selected: Vec<&Group> = groups.iter().filter(|g| filter.is_none_or(|f| g.matches(f))).collect();
    anyhow::ensure!(!selected.is_empty(), "no groups match");
    eprintln!(
        "{draft} on {identity}: {} of {} groups selected ({} cases)",
        selected.len(),
        groups.len(),
        selected.iter().map(|g| g.filtered_cases(filter).count()).sum::<usize>()
    );

    let mut items: Vec<Item> = selected
        .iter()
        .map(|g| {
            let cases: Vec<_> = g.filtered_cases(filter).collect();
            let hash = group_hash(g, identity, glue, &cases);
            Item { group: g, cases, key: module_key(g, identity), hash, generated: None, result: None }
        })
        .collect();

    // The committed results file is the cache: a record whose hash matches
    // is reused as-is, so a rerun that changes nothing rewrites nothing.
    let previous = Results::load(&headline_dir.join("results.json")).ok();
    let mut cached = 0;
    if let Some(prev) = previous.as_ref().filter(|_| !opts.no_cache) {
        let by_hash: BTreeMap<&str, &GroupResult> = prev.groups.iter().map(|g| (g.hash.as_str(), g)).collect();
        for item in &mut items {
            if let Some(r) = by_hash.get(item.hash.as_str()) {
                item.result = Some((*r).clone());
                cached += 1;
            }
        }
    }
    eprintln!("cache: {cached} hit, {} to run", items.len() - cached);

    // Step 1: generate, one module per distinct key.
    let t = Instant::now();
    let mut to_generate: Vec<(String, serde_json::Value)> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for item in items.iter().filter(|i| i.result.is_none()) {
        if seen.insert(item.key.clone()) {
            to_generate.push((item.key.clone(), item.group.schema.clone()));
        }
    }
    let mut gen_status: BTreeMap<String, (String, Option<String>)> = BTreeMap::new();
    if !to_generate.is_empty() {
        for s in flavor.generator.generate(&flavor.target, draft, &gen_dir, &to_generate)? {
            gen_status.insert(s.key, (s.status, s.message));
        }
    }
    let mut ok_keys: Vec<String> = Vec::new();
    for item in items.iter_mut().filter(|i| i.result.is_none()) {
        let status = gen_status
            .get(&item.key)
            .cloned()
            .with_context(|| format!("{}: generate reported nothing for {}", flavor.generator.name, item.group.id()))?;
        if status.0 == "ok" && !ok_keys.contains(&item.key) {
            ok_keys.push(item.key.clone());
        }
        item.generated = Some(status);
    }
    eprintln!("generate: {} modules, {} produced code ({:.1?})", to_generate.len(), ok_keys.len(), t.elapsed());

    // Step 2: build.
    let t = Instant::now();
    let built = if ok_keys.is_empty() {
        None
    } else {
        Some(flavor.generator.build(&flavor.target, draft, &gen_dir, &batch_dir, &ok_keys)?)
    };
    if let Some(b) = &built {
        let ok = b.outcomes.values().filter(|o| **o == BuildOutcome::Ok).count();
        eprintln!("build: {ok} of {} modules compiled ({:.1?})", b.outcomes.len(), t.elapsed());
    }

    // Step 3: run.
    let t = Instant::now();
    let mut run_cases = Vec::new();
    let mut case_slots: Vec<(usize, usize)> = Vec::new(); // (item index, position in item.cases)
    if let Some(b) = &built {
        for (ii, item) in items.iter().enumerate() {
            if item.result.is_some() || b.outcomes.get(&item.key) != Some(&BuildOutcome::Ok) {
                continue;
            }
            for (pos, (_, case)) in item.cases.iter().enumerate() {
                run_cases.push(RunCase { key: item.key.clone(), data: case.data.clone() });
                case_slots.push((ii, pos));
            }
        }
    }
    let mut outcomes: BTreeMap<(usize, usize), RunOutcome> = BTreeMap::new();
    if let Some(b) = built.as_ref().filter(|b| !b.runner.is_empty() && !run_cases.is_empty()) {
        let results = runner::run_cases(&b.runner, &scratch, &run_cases, RUN_TIMEOUT)?;
        anyhow::ensure!(results.len() == run_cases.len(), "runner returned {} outcomes for {} cases", results.len(), run_cases.len());
        for (slot, outcome) in case_slots.into_iter().zip(results) {
            outcomes.insert(slot, outcome);
        }
    }
    eprintln!("run: {} cases ({:.1?})", run_cases.len(), t.elapsed());

    // Assemble records for everything that wasn't cached.
    let now = chrono_free_timestamp();
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
        let (status, message) = item.generated.clone().expect("generated");
        let (bucket, error, cases) = match status.as_str() {
            "panic" => (Bucket::GenerationPanic, message, not_run(&item.cases)),
            "error" => (Bucket::GenerationError, message, not_run(&item.cases)),
            "ok" => {
                let built = built.as_ref().expect("built batch");
                match built.outcomes.get(&item.key) {
                    Some(BuildOutcome::Fail(msg)) => (Bucket::CompileFail, Some(msg.clone()), not_run(&item.cases)),
                    Some(BuildOutcome::Ok) => {
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
                        let error = cases.iter().find(|c| c.outcome != CaseOutcome::Match).map(|c| match c.outcome {
                            CaseOutcome::FalseAccept => format!("accepted `{}` though valid: false", c.description),
                            _ => format!("{}: {}", c.description, c.error.clone().unwrap_or_default()),
                        });
                        (bucket, error, cases)
                    }
                    None => anyhow::bail!("build step reported nothing for {}", group.id()),
                }
            }
            other => anyhow::bail!("{}: unknown generate status {other:?} for {}", flavor.generator.name, group.id()),
        };
        item.result = Some(GroupResult {
            id: group.id(),
            draft: group.draft.clone(),
            file: group.file.clone(),
            optional: group.optional,
            group: group.description.clone(),
            hash: item.hash.clone(),
            generated_at: now.clone(),
            bucket,
            error,
            cases,
        });
        fresh += 1;
    }

    let mut results = Results {
        version: slug.clone(),
        typify: identity.to_string(),
        description: flavor.generator.description.clone(),
        language: flavor.generator.language(&flavor.target),
        kind: flavor.generator.kind.clone(),
        generated_at: String::new(),
        groups: items.iter().map(|i| i.result.clone().expect("every item has a result")).collect(),
    };

    // A filtered run never overwrites the headline results; it goes to
    // target/bakeoff/<slug>/filtered/ unless --out is given.
    let headline = opts.out.is_none() && filter.is_none();
    let out_dir = match (&opts.out, filter) {
        (Some(o), _) => o.clone(),
        (None, None) => headline_dir.clone(),
        (None, Some(_)) => scratch.join("filtered"),
    };
    if headline {
        // The headline file accumulates drafts: this run replaces only its
        // own draft's groups. Drafts are kept in canonical order.
        if let Some(prev) = previous {
            let mut kept: Vec<GroupResult> = prev.groups.into_iter().filter(|g| g.draft != draft).collect();
            kept.append(&mut results.groups);
            kept.sort_by_key(|g| summary::draft_rank(&g.draft));
            results.groups = kept;
        }
    }
    results.generated_at = results.groups.iter().map(|g| g.generated_at.clone()).max().unwrap_or_default();
    results.save(&out_dir.join("results.json"))?;
    std::fs::write(out_dir.join("README.md"), summary::render(&results))?;
    if headline {
        write_scoreboard()?;
    }

    if filter.is_some() {
        let keys: BTreeMap<String, String> = items.iter().map(|i| (i.group.id(), i.key.clone())).collect();
        print_detail(&results, &gen_dir, &keys);
    }
    eprintln!("{fresh} fresh, {cached} cached; wrote {} ({:.1?} total)", out_dir.join("results.json").display(), started.elapsed());
    Ok(results)
}

/// Every `results/*/results.json`, in directory order, with its directory.
fn all_results() -> Result<Vec<(PathBuf, Results)>> {
    let results_dir = repo_root().join("results");
    let mut all = Vec::new();
    if results_dir.is_dir() {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(&results_dir)?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        dirs.sort();
        for dir in dirs {
            let path = dir.join("results.json");
            if path.is_file() {
                all.push((dir, Results::load(&path)?));
            }
        }
    }
    Ok(all)
}

/// Regenerate `RESULTS.md` from every `results/*/results.json`.
pub fn write_scoreboard() -> Result<()> {
    let all: Vec<Results> = all_results()?.into_iter().map(|(_, r)| r).collect();
    std::fs::write(repo_root().join("RESULTS.md"), summary::render_scoreboard(&all))?;
    Ok(())
}

/// Re-render every `results/*/README.md` and `RESULTS.md` from the stored
/// results, without running anything.
pub fn render_all() -> Result<()> {
    for (dir, r) in all_results()? {
        std::fs::write(dir.join("README.md"), summary::render(&r))?;
        eprintln!("rendered {}", dir.join("README.md").display());
    }
    write_scoreboard()?;
    eprintln!("rendered {}", repo_root().join("RESULTS.md").display());
    Ok(())
}

/// Per-group, per-case detail for filtered runs.
fn print_detail(results: &Results, gen_dir: &Path, keys: &BTreeMap<String, String>) {
    for g in &results.groups {
        println!("{} {}  [{}]", g.bucket.glyph(), g.id, g.bucket.name());
        if let Some(k) = keys.get(&g.id) {
            // Whatever extension the generator used.
            if let Some(path) = std::fs::read_dir(gen_dir)
                .ok()
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .find(|p| p.file_stem().is_some_and(|s| s == k.as_str()))
            {
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
                CaseOutcome::FalseReject => "🚧 ",
                CaseOutcome::Panic => "💀 ",
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
