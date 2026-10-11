//! Emit generated modules into one batch crate, `cargo check` it, drop
//! modules that fail, and build a runner binary over the survivors.
//!
//! The batch crate is a plain binary crate with one module per generated
//! group plus a `main.rs` that holds a static dispatch table. Each module
//! (written by the generator) exposes `__conformance_check`, which
//! deserializes a `serde_json::Value` into that group's root type. Because rustc stops at phase boundaries
//! (parse, name resolution, typeck) we can't learn about every broken
//! module in one go, so we loop: attribute each error to a module by the
//! file name in its spans, drop those modules from `main.rs`, and check
//! again. Two or three passes is typical.

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

/// One generated module.
#[derive(Debug, Clone)]
pub struct BatchInput {
    /// Module name, a valid Rust identifier unique within the batch
    /// (e.g. `g_0123abcd4567`).
    pub key: String,
    /// Complete module source (items only), including
    /// `pub fn __conformance_check(v: ::serde_json::Value) -> Result<(), String>`.
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileOutcome {
    Ok,
    /// Rendered rustc diagnostic(s) for this module.
    Fail(String),
}

#[derive(Debug)]
pub struct CompiledBatch {
    pub outcomes: BTreeMap<String, CompileOutcome>,
    /// Path to the built runner binary (None if no module survived).
    pub runner: Option<PathBuf>,
}

/// Name of the per-module entry point every generator must define.
pub const CHECK_FN: &str = "__conformance_check";

/// Turns `SIGABRT` into a plain exit so a crash in the runner doesn't raise
/// the macOS Crash Reporter dialog. Embedded in the generated `main.rs`.
const CRASHGUARD: &str = include_str!("crashguard.inc.rs");

/// How many check/build passes we'll spend shaking out broken modules.
const MAX_PASSES: usize = 10;

/// Cap on the stored diagnostic text per module; these end up in
/// `results.json` and a single error can be enormous.
const MAX_DIAG_LEN: usize = 4000;

/// How many rendered errors to keep per module.
const MAX_DIAGS_PER_MODULE: usize = 3;

/// Build the batch crate at `crate_dir` (created/overwritten) using the
/// shared `target_dir`, and return per-module outcomes plus the runner.
///
/// Inputs that repeat a key with byte-identical code collapse into one
/// module (and one entry in `outcomes`); a key reused for different code
/// is an error.
pub fn compile_batch(crate_dir: &Path, target_dir: &Path, inputs: &[BatchInput], extra_deps: &str) -> Result<CompiledBatch> {
    let src = crate_dir.join("src");
    // Clear the sources but keep the crate directory: Cargo.lock lives
    // there, and re-resolving dependencies on every run both costs a
    // network round trip and risks a version bump that invalidates the
    // shared target dir.
    if src.exists() {
        std::fs::remove_dir_all(&src).with_context(|| format!("clearing {}", src.display()))?;
    }
    std::fs::create_dir_all(&src).with_context(|| format!("creating {}", src.display()))?;
    std::fs::create_dir_all(target_dir).with_context(|| format!("creating {}", target_dir.display()))?;

    // Batches share one target dir, and cargo uplifts every batch's binary
    // to `<target>/debug/<bin name>`, so the name has to be per-batch or
    // concurrent batches overwrite each other's runner.
    let bin = bin_name(crate_dir);
    std::fs::write(crate_dir.join("Cargo.toml"), cargo_toml(&bin, extra_deps))?;

    // Keys are content-derived upstream, so two identical groups (the same
    // schema in two different suite files, say) can share one. That's
    // harmless: they'd compile to the same module and the caller looks
    // results up by key. Only a key reused for *different* code is a bug.
    let mut seen: BTreeMap<&str, &BatchInput> = BTreeMap::new();
    for input in inputs {
        anyhow::ensure!(is_ident(&input.key), "module key {:?} is not a plain identifier", input.key);
        // `src/main.rs` is ours; a module claiming it would be silently
        // clobbered by (or clobber) the dispatch table.
        anyhow::ensure!(input.key != "main", "module key \"main\" collides with the batch crate's main.rs");
        match seen.get(input.key.as_str()) {
            Some(first) => anyhow::ensure!(
                first.source == input.source,
                "module key {:?} used for two different modules",
                input.key
            ),
            None => {
                std::fs::write(src.join(format!("{}.rs", input.key)), &input.source)?;
                seen.insert(&input.key, input);
            }
        }
    }
    let keys: BTreeSet<String> = seen.into_keys().map(str::to_string).collect();

    let mut alive = keys.clone();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();

    // Pass 1..n: `cargo check`, dropping whatever rustc complains about.
    converge(crate_dir, target_dir, &bin, &mut alive, &mut failures, &["check"])?;

    // Pass n+1..: the real build. `check` catches essentially everything,
    // but codegen and linking can still fail, so the same loop applies.
    let mut runner = None;
    if !alive.is_empty() {
        runner = converge(
            crate_dir,
            target_dir,
            &bin,
            &mut alive,
            &mut failures,
            &["build", "--bin", &bin],
        )?;
        anyhow::ensure!(
            runner.is_some() || alive.is_empty(),
            "cargo build reported no runner executable"
        );
        // The uplifted binary lives in the shared target dir, which the
        // next build (or a `cargo clean`) is free to churn. Keep a private
        // copy next to the batch's sources.
        if let Some(built) = runner.take() {
            let mine = crate_dir.join("runner-bin");
            std::fs::copy(&built, &mine)
                .with_context(|| format!("copying {} to {}", built.display(), mine.display()))?;
            runner = Some(mine);
        }
    }
    if alive.is_empty() {
        runner = None;
    }

    let mut outcomes: BTreeMap<String, CompileOutcome> = BTreeMap::new();
    for key in keys {
        let outcome = match failures.remove(&key) {
            Some(text) => CompileOutcome::Fail(text),
            None => {
                debug_assert!(alive.contains(&key));
                CompileOutcome::Ok
            }
        };
        outcomes.insert(key, outcome);
    }

    Ok(CompiledBatch { outcomes, runner })
}

/// Re-run `cargo <args>` until it succeeds, removing the modules it blames
/// from `main.rs` between passes. Returns the runner executable if the
/// invocation produced one.
fn converge(
    crate_dir: &Path,
    target_dir: &Path,
    bin: &str,
    alive: &mut BTreeSet<String>,
    failures: &mut BTreeMap<String, String>,
    args: &[&str],
) -> Result<Option<PathBuf>> {
    let mut last_diag = String::new();
    for _ in 0..MAX_PASSES {
        write_main(crate_dir, alive)?;
        let pass = cargo_pass(crate_dir, target_dir, bin, args, alive)?;

        if pass.module_errors.is_empty() {
            if !pass.unattributed.is_empty() {
                // Nothing to drop, so this is our bug (bad main.rs, bad
                // Cargo.toml) rather than a bad schema.
                anyhow::bail!(
                    "cargo {} reported errors that don't belong to any module:\n{}",
                    args.join(" "),
                    pass.unattributed.join("\n")
                );
            }
            if !pass.success {
                anyhow::bail!(
                    "cargo {} failed with no error diagnostics (status {}):\n{}",
                    args.join(" "),
                    pass.status,
                    tail(&pass.stderr, 4000)
                );
            }
            return Ok(pass.runner);
        }

        // Keep one diagnostic from the most recent pass: if we run out of
        // passes, that bail is otherwise undebuggable.
        if let Some((key, text)) = pass.module_errors.first_key_value() {
            last_diag = format!("{key}: {}", first_lines(text, 10));
        }
        for (key, text) in pass.module_errors {
            alive.remove(&key);
            failures.entry(key).or_insert(text);
        }
        if alive.is_empty() {
            // Nothing left to compile; the caller skips the build.
            write_main(crate_dir, alive)?;
            return Ok(None);
        }
    }
    anyhow::bail!(
        "cargo {} still failing after {MAX_PASSES} passes ({} modules left); last diagnostic:\n{}",
        args.join(" "),
        alive.len(),
        last_diag
    )
}

struct Pass {
    /// Module key -> rendered diagnostics.
    module_errors: BTreeMap<String, String>,
    /// Rendered errors we couldn't blame on a module.
    unattributed: Vec<String>,
    success: bool,
    status: String,
    stderr: String,
    runner: Option<PathBuf>,
}

fn cargo_pass(
    crate_dir: &Path,
    target_dir: &Path,
    bin: &str,
    args: &[&str],
    keys: &BTreeSet<String>,
) -> Result<Pass> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.current_dir(crate_dir)
        .args(args)
        .arg("--message-format=json")
        .env("CARGO_TARGET_DIR", target_dir)
        // Our caller may be running under `cargo test`; don't inherit its
        // flags (`-D warnings` in particular) into generated code.
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_RUSTFLAGS")
        .env_remove("CARGO_PRIMARY_PACKAGE")
        .env_remove("CARGO_TARGET_DIR_OVERRIDE");
    let out = cmd
        .output()
        .with_context(|| format!("running cargo {} in {}", args.join(" "), crate_dir.display()))?;

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    let mut module_errors: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut unattributed: Vec<String> = Vec::new();
    let mut runner = None;

    for line in stdout.lines() {
        let line = line.trim();
        if !line.starts_with('{') {
            continue;
        }
        let msg: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        match msg.get("reason").and_then(|r| r.as_str()) {
            Some("compiler-message") => {
                let message = &msg["message"];
                if message.get("level").and_then(|l| l.as_str()) != Some("error") {
                    continue;
                }
                let rendered = message
                    .get("rendered")
                    .and_then(|r| r.as_str())
                    .unwrap_or_else(|| message.get("message").and_then(|m| m.as_str()).unwrap_or("<no rendered error>"))
                    .trim_end()
                    .to_string();
                match attribute(message, keys) {
                    Some(key) => module_errors.entry(key).or_default().push(rendered),
                    // rustc's trailing "aborting due to N previous errors"
                    // and friends carry no spans and no information; don't
                    // mistake them for harness bugs.
                    None if is_summary_error(message) => {}
                    None => unattributed.push(rendered),
                }
            }
            Some("compiler-artifact") if msg["target"]["name"].as_str() == Some(bin) => {
                if let Some(exe) = msg.get("executable").and_then(|e| e.as_str()) {
                    runner = Some(PathBuf::from(exe));
                }
            }
            _ => {}
        }
    }

    let module_errors = module_errors
        .into_iter()
        .map(|(k, diags)| {
            let joined = diags.iter().take(MAX_DIAGS_PER_MODULE).cloned().collect::<Vec<_>>().join("\n");
            (k, truncate(&joined, MAX_DIAG_LEN))
        })
        .collect();

    Ok(Pass {
        module_errors,
        unattributed,
        success: out.status.success(),
        status: out.status.to_string(),
        stderr,
        runner,
    })
}

/// Blame a diagnostic on a module: the first file name it mentions that is
/// one of our module files. Primary spans first, then secondary spans, then
/// child diagnostics, following macro expansions.
fn attribute(message: &serde_json::Value, keys: &BTreeSet<String>) -> Option<String> {
    let mut files = Vec::new();
    collect_files(message, &mut files);
    files.iter().find_map(|f| key_for_file(f, keys))
}

fn collect_files(message: &serde_json::Value, out: &mut Vec<String>) {
    let spans = message.get("spans").and_then(|s| s.as_array());
    if let Some(spans) = spans {
        for primary in [true, false] {
            for span in spans {
                if span.get("is_primary").and_then(|p| p.as_bool()).unwrap_or(false) == primary {
                    collect_span_files(span, out);
                }
            }
        }
    }
    if let Some(children) = message.get("children").and_then(|c| c.as_array()) {
        for child in children {
            collect_files(child, out);
        }
    }
}

fn collect_span_files(span: &serde_json::Value, out: &mut Vec<String>) {
    if let Some(file) = span.get("file_name").and_then(|f| f.as_str()) {
        out.push(file.to_string());
    }
    if let Some(expansion) = span.get("expansion") {
        for field in ["span", "def_site_span"] {
            if let Some(inner) = expansion.get(field).filter(|v| v.is_object()) {
                collect_span_files(inner, out);
            }
        }
    }
}

/// `src/g_abc.rs` (or a longer path ending in it) -> `g_abc`, but only for
/// a `g_abc` we actually wrote: diagnostics routinely point at std and at
/// dependency sources, which also live under some `src/`.
fn key_for_file(file: &str, keys: &BTreeSet<String>) -> Option<String> {
    let path = Path::new(file);
    if path.extension()?.to_str()? != "rs" {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    if !keys.contains(stem) {
        return None;
    }
    if path.parent()?.file_name()?.to_str()? != "src" {
        return None;
    }
    Some(stem.to_string())
}

/// rustc's span-less wrap-up errors ("aborting due to 3 previous errors",
/// "could not compile ...").
fn is_summary_error(message: &serde_json::Value) -> bool {
    let text = message.get("message").and_then(|m| m.as_str()).unwrap_or("");
    message.get("code").map(|c| c.is_null()).unwrap_or(true)
        && (text.starts_with("aborting due to")
            || text.starts_with("could not compile")
            || text.starts_with("For more information"))
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with(|c: char| c.is_ascii_digit())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// A binary name unique to this batch directory, so concurrent batches
/// sharing a target dir don't uplift over each other's executable.
/// A bin name unique to this crate directory. Every batch shares one
/// `CARGO_TARGET_DIR`, and cargo uplifts each bin to `<target>/debug/<name>`
/// and skips the uplift when it thinks the unit is fresh, so two batches
/// with the same bin name would end up running each other's binary. The
/// name is derived from the absolute path so it's stable across runs and
/// distinct across generators.
fn bin_name(crate_dir: &Path) -> String {
    use std::hash::{Hash, Hasher};
    let abs = std::path::absolute(crate_dir).unwrap_or_else(|_| crate_dir.to_path_buf());
    // DefaultHasher::new() uses fixed keys, so this is deterministic.
    let mut h = std::collections::hash_map::DefaultHasher::new();
    abs.hash(&mut h);
    let tag: String = crate_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("runner_{tag}_{:016x}", h.finish())
}

fn first_lines(s: &str, n: usize) -> String {
    let mut out: String = s.lines().take(n).collect::<Vec<_>>().join("\n");
    if s.lines().count() > n {
        out.push_str("\n...");
    }
    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n... (truncated)", &s[..end])
}

fn tail(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut start = s.len() - max;
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    format!("... (truncated)\n{}", &s[start..])
}

fn write_main(crate_dir: &Path, alive: &BTreeSet<String>) -> Result<()> {
    let mut s = String::from(MAIN_PRELUDE);
    s.push_str("mod crashguard {\n");
    s.push_str(CRASHGUARD);
    s.push_str("}\n\n");
    for key in alive {
        s.push_str(&format!("mod {key};\n"));
    }
    s.push_str("\nstatic MODULES: &[(&str, Check)] = &[\n");
    for key in alive {
        s.push_str(&format!("    (\"{key}\", {key}::{CHECK_FN}),\n"));
    }
    s.push_str("];\n");
    s.push_str(MAIN_BODY);
    std::fs::write(crate_dir.join("src").join("main.rs"), s)?;
    Ok(())
}

/// The batch crate's manifest. `extra_deps` is the generator's
/// `batch-deps.toml` (lines for the `[dependencies]` table), so each
/// generator declares what its own output needs.
fn cargo_toml(bin: &str, extra_deps: &str) -> String {
    format!(
        r#"[package]
name = "conformance-batch"
version = "0.0.0"
edition = "2021"
publish = false

# Keep this crate out of any enclosing workspace.
[workspace]

[[bin]]
name = "{bin}"
path = "src/main.rs"

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
serde_json = {{ version = "1", features = ["float_roundtrip"] }}
regress = "0.12"
chrono = {{ version = "0.4", features = ["serde"] }}
uuid = {{ version = "1", features = ["serde"] }}
{extra_deps}
{CARGO_TOML_TAIL}"#
    )
}

const CARGO_TOML_TAIL: &str = r#"
# Generated code is noisy by construction; none of it is our problem.
[lints.rust]
warnings = "allow"

[profile.dev]
opt-level = 0
debug = false
# With debug = false cargo would default to strip = "debuginfo", which makes
# rustc shell out to rust-objcopy; that's wasted work here and, on a
# toolchain whose llvm-tools are broken, a crash dialog per build.
strip = "none"
incremental = false

[profile.dev.package."*"]
opt-level = 0
debug = false
"#;

const MAIN_PRELUDE: &str = r#"#![allow(warnings)]
//! Generated by typify-conformance. See src/runner.rs for the protocol.

type Check = fn(::serde_json::Value) -> ::std::result::Result<(), ::std::string::String>;

"#;

const MAIN_BODY: &str = r#"
static PANIC_MESSAGE: ::std::sync::Mutex<Option<String>> = ::std::sync::Mutex::new(None);

fn main() {
    crashguard::install();
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: runner <cases.json> <start-index>");
        ::std::process::exit(2);
    }
    let text = ::std::fs::read_to_string(&args[1]).expect("read cases file");
    let start: usize = args[2].parse().expect("parse start index");
    let cases: Vec<::serde_json::Value> = ::serde_json::from_str(&text).expect("parse cases file");

    // We got far enough to run cases; anything that kills us from here on
    // is a case's fault, not a setup problem. (println! flushes on the
    // newline, so this survives an abort on the very first case.)
    println!("{}", ::serde_json::json!({ "ready": start }));

    // Capture panic messages instead of letting the default hook write to
    // stderr; the harness only reads stdout.
    ::std::panic::set_hook(Box::new(|info| {
        let payload = info
            .payload()
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| info.payload().downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| "non-string panic payload".to_string());
        let location = info
            .location()
            .map(|l| format!(" at {}:{}", l.file(), l.line()))
            .unwrap_or_default();
        let message = format!("{payload}{location}");
        *PANIC_MESSAGE.lock().unwrap_or_else(|e| e.into_inner()) = Some(message);
    }));

    use ::std::io::Write;
    let stdout = ::std::io::stdout();
    for (i, case) in cases.iter().enumerate().skip(start) {
        let key = case["key"].as_str().unwrap_or("").to_string();
        let data = case["data"].clone();
        let outcome = match MODULES.iter().find(|(k, _)| *k == key) {
            Some((_, check)) => run_one(*check, data),
            None => ::serde_json::json!({ "panic": "module not compiled" }),
        };
        // The absolute case index goes on the wire so that stray output
        // (anything a generated deserializer prints) can't shift the
        // harness's cursor.
        let line = ::serde_json::json!({ "case": { "i": i, "r": outcome } });
        let mut lock = stdout.lock();
        let _ = writeln!(lock, "{}", ::serde_json::to_string(&line).unwrap());
        let _ = lock.flush();
    }
}

/// Run one check on a fresh thread with a big stack (typify's generated
/// deserializers can recurse deeply). A genuine stack overflow still
/// aborts the process; the harness restarts us past the offending case.
fn run_one(check: Check, data: ::serde_json::Value) -> ::serde_json::Value {
    *PANIC_MESSAGE.lock().unwrap_or_else(|e| e.into_inner()) = None;
    let joined = ::std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(move || check(data))))
        .expect("spawn check thread")
        .join();
    match joined {
        Ok(Ok(Ok(()))) => ::serde_json::json!("accept"),
        Ok(Ok(Err(e))) => ::serde_json::json!({ "reject": e }),
        Ok(Err(_)) | Err(_) => {
            let message = PANIC_MESSAGE
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .unwrap_or_else(|| "unknown panic".to_string());
            ::serde_json::json!({ "panic": message })
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;
    use std::process::Stdio;

    /// A module body plus the check function a generator would append.
    fn module(code: &str, root: &str) -> String {
        format!(
            "{code}\npub fn {CHECK_FN}(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {{\n\
             \x20   ::serde_json::from_value::<{root}>(v).map(|_| ()).map_err(|e| e.to_string())\n}}\n"
        )
    }

    fn mk(key: &str, code: &str, root: &str) -> BatchInput {
        BatchInput { key: key.to_string(), source: module(code, root) }
    }

    fn inputs() -> Vec<BatchInput> {
        vec![
            mk("g_ok", "#[derive(::serde::Deserialize)]\npub struct Root {\n    pub a: i64,\n}\n", "Root"),
            mk("g_bad", "pub struct Root {\n    pub a: NoSuchType,\n}\n", "Root"),
            mk("g_builtin", "", "i64"),
            mk(
                "g_panic",
                "pub struct Root;\n\
                 impl<'de> ::serde::Deserialize<'de> for Root {\n\
                 fn deserialize<D: ::serde::Deserializer<'de>>(_: D) -> ::std::result::Result<Self, D::Error> { panic!(\"boom\") }\n\
                 }\n",
                "Root",
            ),
            mk(
                "g_abort",
                "pub struct Root;\n\
                 impl<'de> ::serde::Deserialize<'de> for Root {\n\
                 fn deserialize<D: ::serde::Deserializer<'de>>(_: D) -> ::std::result::Result<Self, D::Error> { ::std::process::abort() }\n\
                 }\n",
                "Root",
            ),
        ]
    }

    /// Drive the runner once from `start`, returning its parsed stdout
    /// lines and whether it exited cleanly.
    fn run_from(runner: &Path, cases: &Path, start: usize) -> (Vec<serde_json::Value>, bool) {
        let mut child = Command::new(runner)
            .arg(cases)
            .arg(start.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn runner");
        let lines: Vec<serde_json::Value> = std::io::BufReader::new(child.stdout.take().unwrap())
            .lines()
            .map(|l| serde_json::from_str(&l.unwrap()).expect("runner line is JSON"))
            .collect();
        let ok = child.wait().unwrap().success();
        (lines, ok)
    }

    #[test]
    fn batch_compile_and_run() {
        let dir = std::env::temp_dir().join(format!("rust-batch-test-{}", std::process::id()));
        let crate_dir = dir.join("batch");
        let target_dir = dir.join("target");
        let mut inputs = inputs();
        // A content-derived key can repeat when two suite groups are
        // identical; that must collapse rather than break the batch.
        inputs.push(inputs[0].clone());
        let batch = compile_batch(&crate_dir, &target_dir, &inputs, "").expect("compile batch");

        assert_eq!(batch.outcomes.len(), 5);
        assert_eq!(batch.outcomes.get("g_ok"), Some(&CompileOutcome::Ok));
        assert_eq!(batch.outcomes.get("g_builtin"), Some(&CompileOutcome::Ok));
        assert_eq!(batch.outcomes.get("g_panic"), Some(&CompileOutcome::Ok));
        assert_eq!(batch.outcomes.get("g_abort"), Some(&CompileOutcome::Ok));
        match batch.outcomes.get("g_bad") {
            Some(CompileOutcome::Fail(text)) => {
                assert!(text.contains("NoSuchType"), "unexpected diagnostic: {text}");
            }
            other => panic!("expected g_bad to fail to compile, got {other:?}"),
        }
        let runner_bin = batch.runner.clone().expect("runner binary");
        assert!(runner_bin.exists(), "{} does not exist", runner_bin.display());

        let case = |key: &str, data: serde_json::Value| serde_json::json!({ "key": key, "data": data });
        let cases = vec![
            case("g_ok", serde_json::json!({ "a": 1 })),
            case("g_ok", serde_json::json!("x")),
            case("g_builtin", serde_json::json!(5)),
            case("g_panic", serde_json::json!(null)),
            case("g_abort", serde_json::json!(null)),
            case("g_ok", serde_json::json!({ "a": 2 })),
        ];
        let cases_path = dir.join("cases.json");
        std::fs::write(&cases_path, serde_json::to_vec(&cases).unwrap()).unwrap();

        // First run: ready, four outcomes, then the abort kills the process
        // (the crash guard turns that into a plain non-zero exit).
        let (lines, ok) = run_from(&runner_bin, &cases_path, 0);
        assert!(!ok, "the abort case should have killed the runner");
        assert_eq!(lines[0], serde_json::json!({ "ready": 0 }));
        let outcome = |i: usize| lines[i + 1]["case"]["r"].clone();
        assert_eq!(lines[1]["case"]["i"], 0);
        assert_eq!(outcome(0), serde_json::json!("accept"));
        assert!(outcome(1).get("reject").is_some(), "{:?}", outcome(1));
        assert_eq!(outcome(2), serde_json::json!("accept"));
        assert!(outcome(3)["panic"].as_str().unwrap().contains("boom"), "{:?}", outcome(3));
        assert_eq!(lines.len(), 5, "no outcome should be printed for the aborting case");

        // Restart after the dead case: the following case still gets a real answer.
        let (lines, ok) = run_from(&runner_bin, &cases_path, 5);
        assert!(ok);
        assert_eq!(lines[0], serde_json::json!({ "ready": 5 }));
        assert_eq!(lines[1]["case"]["i"], 5);
        assert_eq!(lines[1]["case"]["r"], serde_json::json!("accept"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Breakage from three different compiler phases at once: a parse
    /// error, a name-resolution error, and a type error. Each has to be
    /// blamed on its own module and the healthy one has to survive.
    /// (Today's rustc recovers from the parse error and reports all three
    /// in one pass; if it ever stops at a phase boundary instead, the
    /// convergence loop has to pick up the rest on later passes.)
    #[test]
    fn errors_from_different_phases_converge() {
        let dir = std::env::temp_dir().join(format!("rust-batch-phases-{}", std::process::id()));
        let crate_dir = dir.join("batch");
        let target_dir = dir.join("target");
        let mk = |key: &str, code: &str| mk(key, code, "Root");
        let inputs = vec![
            mk("g_ok", "#[derive(::serde::Deserialize)]\npub struct Root {\n    pub a: i64,\n}\n"),
            mk("g_parse", "pub struct Root { pub a: ,,, }\n"),
            mk("g_resolve", "pub struct Root {\n    pub a: NoSuchType,\n}\n"),
            mk(
                "g_typeck",
                "#[derive(::serde::Deserialize)]\npub struct Root {\n    pub a: i64,\n}\n\
                 pub fn oops() -> i64 {\n    \"not an integer\"\n}\n",
            ),
        ];
        let batch = compile_batch(&crate_dir, &target_dir, &inputs, "").expect("compile batch");
        assert_eq!(batch.outcomes.get("g_ok"), Some(&CompileOutcome::Ok));
        for key in ["g_parse", "g_resolve", "g_typeck"] {
            match batch.outcomes.get(key) {
                Some(CompileOutcome::Fail(text)) => {
                    assert!(!text.is_empty(), "{key} failed with no diagnostic")
                }
                other => panic!("expected {key} to fail to compile, got {other:?}"),
            }
        }
        assert!(batch.runner.expect("runner binary").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn file_attribution() {
        let keys: BTreeSet<String> = ["g_abc".to_string()].into_iter().collect();
        assert_eq!(key_for_file("src/g_abc.rs", &keys).as_deref(), Some("g_abc"));
        assert_eq!(key_for_file("/tmp/x/batch/src/g_abc.rs", &keys).as_deref(), Some("g_abc"));
        assert_eq!(key_for_file("src/main.rs", &keys), None);
        assert_eq!(key_for_file("g_abc.rs", &keys), None);
        assert_eq!(key_for_file("src/g_abc.json", &keys), None);
        assert_eq!(key_for_file("/rustc/deadbeef/library/core/src/option.rs", &keys), None);
        assert_eq!(key_for_file("src/g_other.rs", &keys), None);
    }

    #[test]
    fn attribution_prefers_primary_span() {
        let keys: BTreeSet<String> = ["g_a".to_string(), "g_b".to_string()].into_iter().collect();
        let message = serde_json::json!({
            "level": "error",
            "message": "mismatched types",
            "rendered": "error[E0308]: mismatched types",
            "spans": [
                { "file_name": "src/g_a.rs", "is_primary": false },
                { "file_name": "src/g_b.rs", "is_primary": true },
            ],
            "children": [],
        });
        assert_eq!(attribute(&message, &keys).as_deref(), Some("g_b"));

        // Only a macro expansion names a module.
        let message = serde_json::json!({
            "level": "error",
            "message": "the trait bound is not satisfied",
            "rendered": "error[E0277]",
            "spans": [{
                "file_name": "/rustc/x/library/core/src/macros/mod.rs",
                "is_primary": true,
                "expansion": { "span": { "file_name": "src/g_a.rs", "is_primary": true } },
            }],
            "children": [],
        });
        assert_eq!(attribute(&message, &keys).as_deref(), Some("g_a"));

        // Wrap-up errors carry no spans and are not harness bugs.
        let message = serde_json::json!({
            "level": "error",
            "message": "aborting due to 2 previous errors",
            "rendered": "error: aborting due to 2 previous errors",
            "code": null,
            "spans": [],
            "children": [],
        });
        assert_eq!(attribute(&message, &keys), None);
        assert!(is_summary_error(&message));
    }
}
