//! Step 2: emit generated modules into one batch crate, `cargo check` it,
//! drop modules that fail, and build a runner binary over the survivors.
//!
//! TODO(agent): implement. See `src/runner.rs` for the runner protocol.

use anyhow::Result;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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

/// Build the batch crate at `crate_dir` (created/overwritten) using the
/// shared `target_dir`, and return per-module outcomes plus the runner.
pub fn compile_batch(crate_dir: &Path, target_dir: &Path, inputs: &[BatchInput]) -> Result<CompiledBatch> {
    let _ = (crate_dir, target_dir, inputs);
    anyhow::bail!("compile_batch not implemented")
}
