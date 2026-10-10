//! Step 2: emit generated modules into one batch crate, `cargo check` it,
//! drop modules that fail, and build a runner binary over the survivors.
//!
//! The batch crate is a plain binary crate with one module per generated
//! group plus a `main.rs` that holds a static dispatch table. Each module
//! exposes `__conformance_check`, which deserializes a `serde_json::Value`
//! into that group's root type. Because rustc stops at phase boundaries
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
    /// Complete module body from typify (items only), already pretty-printed.
    pub code: String,
    /// Type path to deserialize into, relative to the module
    /// (e.g. `Root`, `i64`, `::std::vec::Vec<i64>`).
    pub root: String,
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

/// Name of the per-module entry point. Deliberately unlikely to collide
/// with anything typify emits.
const CHECK_FN: &str = "__conformance_check";

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
pub fn compile_batch(crate_dir: &Path, target_dir: &Path, inputs: &[BatchInput]) -> Result<CompiledBatch> {
    let src = crate_dir.join("src");
    if crate_dir.exists() {
        std::fs::remove_dir_all(crate_dir).with_context(|| format!("clearing {}", crate_dir.display()))?;
    }
    std::fs::create_dir_all(&src).with_context(|| format!("creating {}", src.display()))?;
    std::fs::create_dir_all(target_dir).with_context(|| format!("creating {}", target_dir.display()))?;

    std::fs::write(crate_dir.join("Cargo.toml"), CARGO_TOML)?;

    // Keys are content-derived upstream, so two identical groups (the same
    // schema in two different suite files, say) can share one. That's
    // harmless: they'd compile to the same module and the caller looks
    // results up by key. Only a key reused for *different* code is a bug.
    let mut seen: BTreeMap<&str, &BatchInput> = BTreeMap::new();
    for input in inputs {
        anyhow::ensure!(is_ident(&input.key), "module key {:?} is not a plain identifier", input.key);
        match seen.get(input.key.as_str()) {
            Some(first) => anyhow::ensure!(
                first.code == input.code && first.root == input.root,
                "module key {:?} used for two different modules",
                input.key
            ),
            None => {
                std::fs::write(src.join(format!("{}.rs", input.key)), module_source(input))?;
                seen.insert(&input.key, input);
            }
        }
    }
    let keys: BTreeSet<String> = seen.into_keys().map(str::to_string).collect();

    let mut alive = keys.clone();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();

    // Pass 1..n: `cargo check`, dropping whatever rustc complains about.
    converge(crate_dir, target_dir, &mut alive, &mut failures, &["check"])?;

    // Pass n+1..: the real build. `check` catches essentially everything,
    // but codegen and linking can still fail, so the same loop applies.
    let mut runner = None;
    if !alive.is_empty() {
        runner = converge(
            crate_dir,
            target_dir,
            &mut alive,
            &mut failures,
            &["build", "--bin", "runner"],
        )?;
        anyhow::ensure!(
            runner.is_some() || alive.is_empty(),
            "cargo build reported no runner executable"
        );
        // Batches share one target dir, so `target/debug/runner` belongs to
        // whichever batch built last. Keep a private copy.
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
    alive: &mut BTreeSet<String>,
    failures: &mut BTreeMap<String, String>,
    args: &[&str],
) -> Result<Option<PathBuf>> {
    for _ in 0..MAX_PASSES {
        write_main(crate_dir, alive)?;
        let pass = cargo_pass(crate_dir, target_dir, args, alive)?;

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
        "cargo {} still failing after {MAX_PASSES} passes ({} modules left)",
        args.join(" "),
        alive.len()
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

fn cargo_pass(crate_dir: &Path, target_dir: &Path, args: &[&str], keys: &BTreeSet<String>) -> Result<Pass> {
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
            Some("compiler-artifact") if msg["target"]["name"].as_str() == Some("runner") => {
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

fn module_source(input: &BatchInput) -> String {
    format!(
        "{code}\n\
         pub fn {CHECK_FN}(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {{\n\
         \x20   ::serde_json::from_value::<{root}>(v).map(|_| ()).map_err(|e| e.to_string())\n\
         }}\n",
        code = input.code,
        root = input.root,
    )
}

fn write_main(crate_dir: &Path, alive: &BTreeSet<String>) -> Result<()> {
    let mut s = String::from(MAIN_PRELUDE);
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

const CARGO_TOML: &str = r#"[package]
name = "conformance-batch"
version = "0.0.0"
edition = "2021"
publish = false

# Keep this crate out of any enclosing workspace.
[workspace]

[[bin]]
name = "runner"
path = "src/main.rs"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regress = "0.12"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde"] }

# Generated code is noisy by construction; none of it is our problem.
[lints.rust]
warnings = "allow"

[profile.dev]
opt-level = 0
debug = false
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
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: runner <cases.json> <start-index>");
        ::std::process::exit(2);
    }
    let text = ::std::fs::read_to_string(&args[1]).expect("read cases file");
    let start: usize = args[2].parse().expect("parse start index");
    let cases: Vec<::serde_json::Value> = ::serde_json::from_str(&text).expect("parse cases file");

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
        if let Ok(mut slot) = PANIC_MESSAGE.lock() {
            *slot = Some(format!("{payload}{location}"));
        }
    }));

    use ::std::io::Write;
    let stdout = ::std::io::stdout();
    for case in cases.iter().skip(start) {
        let key = case["key"].as_str().unwrap_or("").to_string();
        let data = case["data"].clone();
        let outcome = match MODULES.iter().find(|(k, _)| *k == key) {
            Some((_, check)) => run_one(*check, data),
            None => ::serde_json::json!({ "panic": "module not compiled" }),
        };
        let mut lock = stdout.lock();
        let _ = writeln!(lock, "{}", ::serde_json::to_string(&outcome).unwrap());
        let _ = lock.flush();
    }
}

/// Run one check on a fresh thread with a big stack (typify's generated
/// deserializers can recurse deeply). A genuine stack overflow still
/// aborts the process; the harness restarts us past the offending case.
fn run_one(check: Check, data: ::serde_json::Value) -> ::serde_json::Value {
    if let Ok(mut slot) = PANIC_MESSAGE.lock() {
        *slot = None;
    }
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
                .ok()
                .and_then(|mut slot| slot.take())
                .unwrap_or_else(|| "unknown panic".to_string());
            ::serde_json::json!({ "panic": message })
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{self, RunCase, RunOutcome};
    use std::time::Duration;

    fn inputs() -> Vec<BatchInput> {
        let mk = |key: &str, code: &str, root: &str| BatchInput {
            key: key.to_string(),
            code: code.to_string(),
            root: root.to_string(),
        };
        vec![
            mk(
                "g_ok",
                "#[derive(::serde::Deserialize)]\npub struct Root {\n    pub a: i64,\n}\n",
                "Root",
            ),
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

    #[test]
    fn batch_compile_and_run() {
        let dir = std::env::temp_dir().join(format!("typify-conformance-test-{}", std::process::id()));
        let crate_dir = dir.join("batch");
        let target_dir = dir.join("target");
        let mut inputs = inputs();
        // A content-derived key can repeat when two suite groups are
        // identical; that must collapse rather than break the batch.
        inputs.push(inputs[0].clone());
        let batch = compile_batch(&crate_dir, &target_dir, &inputs).expect("compile batch");

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

        let case = |key: &str, data: serde_json::Value| RunCase { key: key.to_string(), data };
        let cases = vec![
            case("g_ok", serde_json::json!({ "a": 1 })),
            case("g_ok", serde_json::json!("x")),
            case("g_builtin", serde_json::json!(5)),
            case("g_panic", serde_json::json!(null)),
            case("g_abort", serde_json::json!(null)),
            case("g_ok", serde_json::json!({ "a": 2 })),
        ];
        let outcomes =
            runner::run_cases(&runner_bin, &dir, &cases, Duration::from_secs(60)).expect("run cases");
        assert_eq!(outcomes.len(), cases.len());
        assert_eq!(outcomes[0], RunOutcome::Accept);
        assert!(matches!(outcomes[1], RunOutcome::Reject(_)), "{:?}", outcomes[1]);
        assert_eq!(outcomes[2], RunOutcome::Accept);
        match &outcomes[3] {
            RunOutcome::Panic(msg) => assert!(msg.contains("boom"), "unexpected panic text: {msg}"),
            other => panic!("expected a panic for g_panic, got {other:?}"),
        }
        match &outcomes[4] {
            RunOutcome::Panic(msg) => {
                assert!(msg.contains("runner process died"), "unexpected panic text: {msg}")
            }
            other => panic!("expected a process death for g_abort, got {other:?}"),
        }
        // The runner restarted and the following case still got a real answer.
        assert_eq!(outcomes[5], RunOutcome::Accept);

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
