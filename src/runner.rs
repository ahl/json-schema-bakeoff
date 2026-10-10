//! Step 3: drive the runner binary produced by `compile.rs`.
//!
//! Runner protocol (the runner is a generated `main.rs` in the batch crate):
//!   runner <cases.json> <start-index>
//! `cases.json` is a JSON array of `RunCase`. The runner processes cases
//! from `start-index` onward, printing one JSON line (`RunOutcome`) per
//! case to stdout and flushing after each. Each deserialize is wrapped in
//! `catch_unwind` with a panic hook that captures the message. If the
//! process dies (stack overflow, abort), the harness records the case at
//! the cursor as a panic and restarts from the next one.
//!
//! TODO(agent): implement `run_cases`.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCase {
    /// Module key (`BatchInput::key`).
    pub key: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunOutcome {
    /// Deserialize succeeded.
    Accept,
    /// Deserialize returned an error.
    Reject(String),
    /// Deserialize panicked, or the runner process died on this case.
    Panic(String),
}

/// Run every case through the runner binary; the result has one entry per
/// input case, in order.
pub fn run_cases(runner: &Path, work_dir: &Path, cases: &[RunCase], timeout: std::time::Duration) -> Result<Vec<RunOutcome>> {
    let _ = (runner, work_dir, cases, timeout);
    anyhow::bail!("run_cases not implemented")
}
