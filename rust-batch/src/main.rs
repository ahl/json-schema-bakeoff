//! `rust-batch build <gen_dir> <batch_dir>`: the shared build step for any
//! generator that emits Rust.
//!
//! Reads a JSON array of module keys on stdin. Each `<gen_dir>/<key>.rs` is
//! a complete module that defines
//! `pub fn __conformance_check(v: ::serde_json::Value) -> Result<(), String>`.
//! Compiles them all as one crate under `<batch_dir>`, drops the ones rustc
//! rejects, builds a runner over the survivors, and prints
//! `{"outcomes": {key: "ok" | {"fail": diagnostic}}, "runner": [exe]}`
//! on stdout. The runner speaks the harness's JSON-lines protocol.
//!
//! If `<gen_dir>/batch-deps.toml` exists, its contents are appended to the
//! batch crate's `[dependencies]` table, so a generator whose output needs
//! more than serde/serde_json/regress/chrono/uuid declares it there (one
//! `name = "version"` line per crate, written by its generate step).
//!
//! The crate's target dir is `$CARGO_TARGET_DIR` when set (the harness
//! points every generator at the repo's shared `target/`), else
//! `<batch_dir>/target`.

mod batch;

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (gen_dir, batch_dir) = match args.as_slice() {
        [_, cmd, gen_dir, batch_dir] if cmd == "build" => (PathBuf::from(gen_dir), PathBuf::from(batch_dir)),
        _ => anyhow::bail!("usage: rust-batch build <gen_dir> <batch_dir>  (keys as a JSON array on stdin)"),
    };
    let keys: Vec<String> = serde_json::from_reader(std::io::stdin().lock()).context("reading keys from stdin")?;
    let inputs = keys
        .iter()
        .map(|key| {
            let path = gen_dir.join(format!("{key}.rs"));
            let source = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            Ok(batch::BatchInput { key: key.clone(), source })
        })
        .collect::<Result<Vec<_>>>()?;
    let target_dir = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| batch_dir.join("target"));

    let extra_deps = std::fs::read_to_string(gen_dir.join("batch-deps.toml")).unwrap_or_default();
    let compiled = batch::compile_batch(&batch_dir, &target_dir, &inputs, &extra_deps)?;
    let outcomes: BTreeMap<String, serde_json::Value> = compiled
        .outcomes
        .into_iter()
        .map(|(k, o)| {
            let v = match o {
                batch::CompileOutcome::Ok => serde_json::json!("ok"),
                batch::CompileOutcome::Fail(text) => serde_json::json!({ "fail": text }),
            };
            (k, v)
        })
        .collect();
    let runner: Vec<String> = compiled.runner.into_iter().map(|p| p.to_string_lossy().into_owned()).collect();
    println!("{}", serde_json::to_string(&serde_json::json!({ "outcomes": outcomes, "runner": runner }))?);
    Ok(())
}
