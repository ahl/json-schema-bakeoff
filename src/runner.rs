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

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

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

/// How much of the dead runner's stderr to keep in the panic text.
const STDERR_TAIL: usize = 1000;

/// Run every case through the runner binary; the result has one entry per
/// input case, in order.
///
/// `timeout` is how long we'll wait for the *next* outcome line, i.e. it
/// bounds a single case rather than the whole batch; when it expires the
/// runner is killed, the case at the cursor is recorded as a panic, and a
/// fresh runner picks up from the following case.
pub fn run_cases(runner: &Path, work_dir: &Path, cases: &[RunCase], timeout: Duration) -> Result<Vec<RunOutcome>> {
    if cases.is_empty() {
        return Ok(Vec::new());
    }
    std::fs::create_dir_all(work_dir).with_context(|| format!("creating {}", work_dir.display()))?;
    let cases_path = work_dir.join("cases.json");
    std::fs::write(&cases_path, serde_json::to_vec(cases)?)
        .with_context(|| format!("writing {}", cases_path.display()))?;

    let mut outcomes: Vec<RunOutcome> = Vec::with_capacity(cases.len());
    while outcomes.len() < cases.len() {
        let start = outcomes.len();
        let mut child = Command::new(runner)
            .arg(&cases_path)
            .arg(start.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("spawning {}", runner.display()))?;

        let stdout = child.stdout.take().expect("piped stdout");
        let (tx, rx) = mpsc::channel::<String>();
        let reader = std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        let stderr = child.stderr.take().expect("piped stderr");
        let errors = std::thread::spawn(move || {
            let mut s = String::new();
            let _ = BufReader::new(stderr).read_to_string(&mut s);
            s
        });

        let mut timed_out = false;
        while outcomes.len() < cases.len() {
            match rx.recv_timeout(timeout) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    // Anything we can't parse is stray output, not an
                    // outcome; skipping keeps the cursor aligned.
                    if let Ok(outcome) = serde_json::from_str::<RunOutcome>(line) {
                        outcomes.push(outcome);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    timed_out = true;
                    break;
                }
                // The runner closed stdout: it either finished or died.
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }

        let status = reap(&mut child);
        drop(rx);
        let _ = reader.join();
        let stderr_text = errors.join().unwrap_or_default();

        if outcomes.len() < cases.len() {
            // The case at the cursor took the process down (or hung).
            let message = if timed_out {
                format!("runner timed out after {timeout:?}")
            } else {
                format!(
                    "runner process died: {status}, stderr tail: {}",
                    tail(stderr_text.trim(), STDERR_TAIL)
                )
            };
            outcomes.push(RunOutcome::Panic(message));
        }
        anyhow::ensure!(
            outcomes.len() > start,
            "runner made no progress at case {start} of {}",
            cases.len()
        );
    }

    Ok(outcomes)
}

/// Make sure the child is gone and describe how it ended.
fn reap(child: &mut Child) -> String {
    match child.try_wait() {
        // Already exited on its own.
        Ok(Some(status)) => return status.to_string(),
        Ok(None) => {}
        Err(e) => return format!("wait failed: {e}"),
    }
    let _ = child.kill();
    match child.wait() {
        Ok(status) => format!("killed ({status})"),
        Err(e) => format!("wait failed: {e}"),
    }
}

fn tail(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut start = s.len() - max;
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    format!("...{}", &s[start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The generated runner builds these JSON shapes by hand; keep them in
    /// sync with the serde encoding the harness parses.
    #[test]
    fn outcome_wire_format() {
        let round = |v: serde_json::Value| serde_json::from_value::<RunOutcome>(v).expect("parse outcome");
        assert_eq!(round(serde_json::json!("accept")), RunOutcome::Accept);
        assert_eq!(
            round(serde_json::json!({ "reject": "nope" })),
            RunOutcome::Reject("nope".to_string())
        );
        assert_eq!(
            round(serde_json::json!({ "panic": "boom" })),
            RunOutcome::Panic("boom".to_string())
        );
        assert_eq!(serde_json::to_string(&RunOutcome::Accept).unwrap(), "\"accept\"");
    }

    #[test]
    fn tail_keeps_the_end() {
        assert_eq!(tail("abcdef", 10), "abcdef");
        assert_eq!(tail("abcdef", 3), "...def");
    }
}
