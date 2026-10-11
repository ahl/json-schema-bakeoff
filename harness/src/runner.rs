//! Step 3: drive the runner binary produced by `compile.rs`.
//!
//! Runner protocol (the runner is a generated `main.rs` in the batch crate):
//!   runner <cases.json> <start-index>
//! `cases.json` is a JSON array of `RunCase`. The runner prints one JSON
//! line per event to stdout, flushing after each: `{"ready":<start>}` once
//! it has read the cases, then `{"case":{"i":<index>,"r":<outcome>}}` for
//! every case from `start-index` onward. Indices are absolute so that
//! stray output can't shift the harness's cursor, and the `ready` line
//! distinguishes "this case killed the process" from "the runner never
//! started". Each deserialize is wrapped in `catch_unwind` with a panic
//! hook that captures the message. If the process dies (stack overflow,
//! abort) or hangs, the harness records the case at the cursor as a panic
//! and restarts from the next one.

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

/// One line of the runner's stdout protocol.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Line {
    /// The runner read its cases and is about to run from this index.
    Ready(usize),
    /// One case outcome, by absolute index into `cases`.
    Case { i: usize, r: RunOutcome },
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
pub fn run_cases(runner: &[String], work_dir: &Path, cases: &[RunCase], timeout: Duration) -> Result<Vec<RunOutcome>> {
    anyhow::ensure!(!runner.is_empty(), "no runner command");
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
        let mut child = Command::new(&runner[0])
            .args(&runner[1..])
            .arg(&cases_path)
            .arg(start.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("spawning {}", runner.join(" ")))?;

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

        let mut ready: Option<usize> = None;
        let mut timed_out = false;
        while outcomes.len() < cases.len() {
            match rx.recv_timeout(timeout) {
                Ok(line) => record(&mut outcomes, cases.len(), &mut ready, &line),
                Err(RecvTimeoutError::Timeout) => {
                    timed_out = true;
                    break;
                }
                // The runner closed stdout: it either finished or died.
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }

        let status = reap(&mut child);
        // The channel is unbounded, so the reader never blocks on send and
        // joining it here can't deadlock. Draining afterwards matters for
        // the timeout path: an outcome that landed while we were deciding
        // to give up is a real answer, not a hang.
        let _ = reader.join();
        let stderr_text = errors.join().unwrap_or_default();
        while let Ok(line) = rx.try_recv() {
            record(&mut outcomes, cases.len(), &mut ready, &line);
        }

        // No `ready` line means the runner never got as far as its first
        // case: that's a setup problem (bad binary, unreadable cases file),
        // and respawning it once per remaining case would just repeat it a
        // few thousand times.
        match ready {
            Some(at) if at == start => {}
            Some(at) => anyhow::bail!("runner started at case {at}, not {start}"),
            None => anyhow::bail!(
                "runner {} never started: {} with cases {} from index {start}; stderr: {}",
                runner.join(" "),
                if timed_out { format!("no output within {timeout:?}") } else { format!("status {status}") },
                cases_path.display(),
                tail(stderr_text.trim(), STDERR_TAIL)
            ),
        }

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

/// Fold one protocol line into `outcomes`. Lines the runner didn't write
/// (anything a generated deserializer prints to stdout) don't parse and are
/// ignored; because every outcome carries its own index, ignoring them
/// can't shift later cases onto the wrong result.
fn record(outcomes: &mut Vec<RunOutcome>, total: usize, ready: &mut Option<usize>, line: &str) {
    match serde_json::from_str::<Line>(line.trim()) {
        Ok(Line::Ready(at)) => *ready = Some(at),
        Ok(Line::Case { i, r }) => {
            // Already answered (a restart that re-ran a case), or past the
            // end: nothing to do.
            if i < outcomes.len() || i >= total {
                return;
            }
            // A gap means a line went missing; say so rather than silently
            // attributing this outcome to an earlier case.
            while outcomes.len() < i {
                outcomes.push(RunOutcome::Panic(
                    "the runner reported no outcome for this case".to_string(),
                ));
            }
            outcomes.push(r);
        }
        Err(_) => {}
    }
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

        // The line envelope, as the generated runner prints it.
        assert!(matches!(
            serde_json::from_str::<Line>(r#"{"ready":7}"#).expect("parse ready"),
            Line::Ready(7)
        ));
        match serde_json::from_str::<Line>(r#"{"case":{"i":2,"r":{"reject":"no"}}}"#).expect("parse case") {
            Line::Case { i, r } => {
                assert_eq!(i, 2);
                assert_eq!(r, RunOutcome::Reject("no".to_string()));
            }
            other => panic!("expected a case line, got {other:?}"),
        }
    }

    #[test]
    fn record_uses_the_reported_index() {
        let mut outcomes = Vec::new();
        let mut ready = None;
        let lines = [
            r#"{"ready":0}"#,
            // Stray output from generated code: ignored, cursor unmoved.
            "hello from a deserializer",
            r#"{"something":"else"}"#,
            "",
            r#"{"case":{"i":0,"r":"accept"}}"#,
            // A duplicate (a restart that re-ran a case) shifts nothing.
            r#"{"case":{"i":0,"r":{"panic":"x"}}}"#,
            // A gap is filled explicitly rather than silently shifting.
            r#"{"case":{"i":2,"r":"accept"}}"#,
            // Past the end: dropped.
            r#"{"case":{"i":9,"r":"accept"}}"#,
        ];
        for line in lines {
            record(&mut outcomes, 4, &mut ready, line);
        }
        assert_eq!(ready, Some(0));
        assert_eq!(outcomes.len(), 3);
        assert_eq!(outcomes[0], RunOutcome::Accept);
        assert!(matches!(outcomes[1], RunOutcome::Panic(_)));
        assert_eq!(outcomes[2], RunOutcome::Accept);
    }

    #[cfg(unix)]
    fn stub_runner(dir: &Path, body: &str) -> std::path::PathBuf {
        use std::os::unix::fs::PermissionsExt;
        std::fs::create_dir_all(dir).unwrap();
        let path = dir.join("stub-runner");
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[cfg(unix)]
    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("typify-conformance-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    /// Stray stdout from generated code must not shift later outcomes onto
    /// the wrong case.
    #[cfg(unix)]
    #[test]
    fn stray_output_does_not_shift_the_cursor() {
        let dir = scratch("stray");
        let runner = stub_runner(
            &dir,
            r#"echo '{"ready":0}'
echo 'chatter from a deserializer'
echo '{"case":{"i":0,"r":"accept"}}'
echo 'not json either'
echo '{"case":{"i":1,"r":{"reject":"no"}}}'
echo '{"case":{"i":2,"r":{"panic":"boom"}}}'"#,
        );
        let cases: Vec<RunCase> = (0..3)
            .map(|i| RunCase { key: format!("g_{i}"), data: serde_json::json!(i) })
            .collect();
        let outcomes = run_cases(&[runner.to_string_lossy().into_owned()], &dir, &cases, Duration::from_secs(30)).expect("run cases");
        assert_eq!(
            outcomes,
            vec![
                RunOutcome::Accept,
                RunOutcome::Reject("no".to_string()),
                RunOutcome::Panic("boom".to_string()),
            ]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A runner that never gets as far as its first case is a setup bug: we
    /// must not respawn it once per case.
    #[cfg(unix)]
    #[test]
    fn a_runner_that_never_starts_is_an_error() {
        let dir = scratch("nostart");
        let runner = stub_runner(&dir, "echo 'cannot read cases' >&2\nexit 3");
        let cases: Vec<RunCase> = (0..100)
            .map(|i| RunCase { key: format!("g_{i}"), data: serde_json::json!(i) })
            .collect();
        let err = run_cases(&[runner.to_string_lossy().into_owned()], &dir, &cases, Duration::from_secs(30))
            .expect_err("expected an error, not 100 panics");
        let text = err.to_string();
        assert!(text.contains("never started"), "unexpected error: {text}");
        assert!(text.contains("cannot read cases"), "stderr not reported: {text}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn tail_keeps_the_end() {
        assert_eq!(tail("abcdef", 10), "abcdef");
        assert_eq!(tail("abcdef", 3), "...def");
    }
}
