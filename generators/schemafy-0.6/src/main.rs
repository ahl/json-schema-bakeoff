//! schemafy 0.6 generator for the bakeoff harness.
//!
//!   version                → "schemafy <schemafy_lib version>"
//!   generate <out_dir>     → stdin: JSON array of {key, draft, schema};
//!                             writes <out_dir>/<key>.rs (module +
//!                             __conformance_check); stdout: JSON array of
//!                             {key, status, message?}
//!
//! Building and running the generated Rust is delegated to `rust-batch`
//! (see generator.toml).
//!
//! `draft` is accepted and ignored: schemafy has one dialect (draft 4, the
//! meta-schema its `Schema` type was generated from) and no setting to
//! choose another, so its results are identical in every draft directory.
//!
//! Unlike the typify generators, `generate` is a supervisor over a worker
//! subprocess (`generate-worker`, a private third verb) rather than doing
//! the work in process. schemafy resolves `$ref` and `allOf` by plain
//! recursion with no cycle check, so some schemas overflow the stack — five
//! groups in the suite do, all of them a root with `type: "object"`,
//! `properties`, and an `allOf` whose merged subschema contributes no
//! properties of its own — and a stack overflow aborts the process no
//! matter how big the stack or how careful the `catch_unwind`. The worker
//! streams one status line per group, and a
//! worker that dies costs exactly one group (recorded as a `panic`, with
//! the runtime's message) instead of the whole draft. This is the same
//! restart-past-the-cursor shape the harness uses for runner processes,
//! down to the `ready` handshake that tells a crash on a schema apart from
//! a worker that never got going.

mod generate;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

#[path = "../../../rust-batch/src/crashguard.inc.rs"]
mod crashguard;

#[derive(Deserialize)]
struct Group {
    key: String,
    /// The suite directory the schema came from. schemafy has no dialect
    /// setting, so this is accepted and otherwise unused.
    #[allow(dead_code)]
    draft: Option<String>,
    schema: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize)]
struct Status {
    key: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

/// The worker's stdout protocol. Indices are absolute positions in the
/// group list so a restart can't shift the supervisor's cursor.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Msg {
    /// Printed once, as soon as the worker has the group list. Without it
    /// the supervisor can't tell "schemafy crashed the process on this
    /// group" from "the worker never started" (an unreadable group file, a
    /// broken executable) — and blaming the latter on groups one at a time
    /// would quietly turn it into a draft full of panics.
    Ready { start: usize },
    /// The status for the group at `i`.
    Done {
        i: usize,
        #[serde(flatten)]
        status: Status,
    },
    /// The group at `i` timed out and the worker is exiting after this
    /// line: the abandoned generation thread holds the panic-hook lock and
    /// may yet overflow the stack, so the process is spent (see
    /// `generate::generate`). The group is reported, so the supervisor
    /// resumes at `i + 1` and blames nobody for the exit.
    Timeout {
        i: usize,
        #[serde(flatten)]
        status: Status,
    },
}

/// How long one group gets before its generation is abandoned.
const TIMEOUT: Duration = Duration::from_secs(60);

/// How long the supervisor waits for any word from a worker before
/// concluding the worker itself is wedged, killing it, and blaming the
/// group at the cursor. A worker abandons a generation after `TIMEOUT`, so
/// this only has to be comfortably longer than that.
const STALL: Duration = Duration::from_secs(90);

/// How much of a dead worker's stderr to keep in the panic text.
const STDERR_TAIL: usize = 400;

/// Where the supervisor parks the group list for its workers.
const GROUPS_FILE: &str = ".groups.json";

/// What the generated code needs beyond the batch crate's standing
/// serde/serde_json/regress/chrono/uuid.
///
/// * `schemafy_core` holds `one_or_many`, which schemafy's output names in
///   `#[serde(with = "::schemafy_core::one_or_many")]` for an `anyOf` of
///   `T` and `array of T`.
/// * `serde_repr` backs the `Deserialize_repr`/`Serialize_repr` derives
///   schemafy emits for an integer `enum` with `enumNames`.
///
/// Neither is reached by the suite as it stands (`enumNames` is a
/// non-standard keyword, and schemafy's `one_or_many` shortcut needs an
/// `anyOf` of `T` and `array of T`), but both are things schemafy's output
/// names, so the batch crate carries them.
const BATCH_DEPS: &str = "\
schemafy_core = \"=0.6.0\"\n\
serde_repr = \"=0.1.21\"\n";

fn main() -> Result<()> {
    crashguard::install();
    let args: Vec<String> = std::env::args().collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        [_, "version"] => {
            println!("{}", generate::VERSION);
            Ok(())
        }
        [_, "generate", out_dir] => supervise(Path::new(out_dir)),
        [_, "generate-worker", out_dir, start] => work(Path::new(out_dir), start.parse()?),
        _ => anyhow::bail!("usage: version | generate <out_dir>"),
    }
}

/// Run the whole group list through worker processes, restarting past any
/// group that kills one, and print the status array the harness expects.
fn supervise(out_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(out_dir)?;
    std::fs::write(out_dir.join("batch-deps.toml"), BATCH_DEPS)?;

    let mut groups = Vec::new();
    std::io::stdin().lock().read_to_end(&mut groups)?;
    let keys: Vec<String> = serde_json::from_slice::<Vec<Group>>(&groups)
        .context("reading groups from stdin")?
        .into_iter()
        .map(|g| g.key)
        .collect();
    std::fs::write(out_dir.join(GROUPS_FILE), &groups)?;

    let exe = std::env::current_exe().context("locating our own executable")?;
    let mut statuses: Vec<Option<Status>> = vec![None; keys.len()];
    let mut cursor = 0;
    while cursor < keys.len() {
        match run_worker(&exe, out_dir, cursor, &mut statuses)? {
            End::Reported => break,
            // A timeout: the group is already recorded, and the worker
            // exited on purpose so the next one starts clean.
            End::Resume(next) => cursor = next,
            End::Died(next, death) => {
                // The worker died on the group at `next` without reporting
                // it. That is schemafy crashing the process, which is a
                // panic bucket exactly like one `catch_unwind` caught.
                statuses[next] = Some(Status {
                    key: keys[next].clone(),
                    status: "panic".to_string(),
                    message: Some(death),
                });
                cursor = next + 1;
            }
        }
    }

    let statuses = statuses
        .into_iter()
        .zip(&keys)
        .map(|(s, key)| s.with_context(|| format!("no worker reported a status for {key}")))
        .collect::<Result<Vec<Status>>>()?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(serde_json::to_string(&statuses)?.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

/// How a worker run ended.
enum End {
    /// It reported every group through the end of the list.
    Reported,
    /// It reported up to `next` and asked to be restarted there.
    Resume(usize),
    /// It died at `next` without reporting it.
    Died(usize, String),
}

/// What a worker has told us so far.
struct Progress {
    /// One past the last group reported: where a restart resumes.
    next: usize,
    /// Has the worker said it got going at all?
    ready: bool,
    /// Did the last thing it said ask for a restart?
    resume: bool,
}

impl Progress {
    fn apply(&mut self, line: &str, start: usize, statuses: &mut [Option<Status>]) -> Result<()> {
        let msg: Msg = serde_json::from_str(line).with_context(|| format!("worker printed {line:?}"))?;
        let (i, status) = match msg {
            Msg::Ready { start: s } => {
                anyhow::ensure!(s == start, "worker started at group {s}, not {start}");
                self.ready = true;
                return Ok(());
            }
            Msg::Done { i, status } => {
                self.resume = false;
                (i, status)
            }
            Msg::Timeout { i, status } => {
                self.resume = true;
                (i, status)
            }
        };
        anyhow::ensure!(i < statuses.len(), "worker reported out-of-range index {i}");
        statuses[i] = Some(status);
        self.next = i + 1;
        Ok(())
    }
}

/// Spawn one worker from `start` and fill in the statuses it reports.
fn run_worker(exe: &Path, out_dir: &Path, start: usize, statuses: &mut [Option<Status>]) -> Result<End> {
    let mut child = Command::new(exe)
        .arg("generate-worker")
        .arg(out_dir)
        .arg(start.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning {} generate-worker", exe.display()))?;

    // Both pipes get their own thread: a worker that filled one while we
    // read the other would deadlock, and reading stdout on a thread is also
    // what lets us put a deadline on a wedged worker.
    let mut err = child.stderr.take().expect("piped stderr");
    let errors = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = err.read_to_string(&mut s);
        s
    });
    let out = child.stdout.take().expect("piped stdout");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(out).lines() {
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut p = Progress { next: start, ready: false, resume: false };
    let stalled = loop {
        match rx.recv_timeout(STALL) {
            Ok(line) => p.apply(&line.context("reading worker stdout")?, start, statuses)?,
            // Nothing for STALL: the worker is stuck somewhere its own
            // per-group timeout doesn't cover. Kill it and blame the group
            // it was on, the same as a crash.
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let _ = child.kill();
                break true;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break false,
        }
    };
    // Anything the reader queued between the deadline and the kill.
    while let Ok(line) = rx.try_recv() {
        p.apply(&line.context("reading worker stdout")?, start, statuses)?;
    }

    let status = child.wait()?;
    let stderr = tail(errors.join().unwrap_or_default().trim(), STDERR_TAIL);
    if p.next == statuses.len() {
        return Ok(End::Reported);
    }
    // A worker that stopped early without ever announcing itself, or that
    // stopped early and exited cleanly, didn't crash on a schema: something
    // is wrong with us (a bad group file, a missing executable). Blaming
    // groups one at a time would quietly turn that into a draft full of
    // panics.
    anyhow::ensure!(p.ready, "generate-worker never reported itself ready ({status}): {stderr}");
    if p.resume {
        anyhow::ensure!(p.next > start, "generate-worker asked to restart at group {start} without any progress");
        return Ok(End::Resume(p.next));
    }
    anyhow::ensure!(
        !status.success(),
        "generate-worker stopped at group {} of {} but exited cleanly: {stderr}",
        p.next,
        statuses.len()
    );
    let what = if stalled {
        format!("the generator process produced nothing for {STALL:?} and was killed ({status})")
    } else {
        format!("schemafy crashed the generator process ({status})")
    };
    let death = if stderr.is_empty() { what } else { format!("{what}: {stderr}") };
    Ok(End::Died(p.next, death))
}

/// The worker: generate groups from `start` onward, writing each module and
/// reporting its status on stdout as it goes.
fn work(out_dir: &Path, start: usize) -> Result<()> {
    let groups: Vec<Group> =
        serde_json::from_slice(&std::fs::read(out_dir.join(GROUPS_FILE)).context("reading the group list")?)?;
    let mut stdout = std::io::stdout().lock();
    // Flush every line: the supervisor reads what arrived to decide who to
    // blame, and a process killed by the crash guard loses its buffers.
    writeln!(stdout, "{}", serde_json::to_string(&Msg::Ready { start })?)?;
    stdout.flush()?;
    for (i, g) in groups.into_iter().enumerate().skip(start) {
        let mut spent = false;
        let status = match generate::generate(&g.schema, TIMEOUT) {
            generate::GenOutcome::Code { code, root } => {
                let source = format!(
                    "// root type: {root}\n\n{code}\n\
                     pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {{\n\
                     \x20   ::serde_json::from_value::<{root}>(v).map(|_| ()).map_err(|e| e.to_string())\n\
                     }}\n"
                );
                std::fs::write(out_dir.join(format!("{}.rs", g.key)), source)?;
                Status { key: g.key, status: "ok".to_string(), message: None }
            }
            generate::GenOutcome::Error(m) => Status { key: g.key, status: "error".to_string(), message: Some(m) },
            generate::GenOutcome::Panic(m) => Status { key: g.key, status: "panic".to_string(), message: Some(m) },
            // A timeout leaves a thread running that holds the panic-hook
            // lock and may still overflow the stack, so this process can't
            // be trusted with another group: report it and quit.
            generate::GenOutcome::Timeout(m) => {
                spent = true;
                Status { key: g.key, status: "panic".to_string(), message: Some(m) }
            }
        };
        let msg = if spent { Msg::Timeout { i, status } } else { Msg::Done { i, status } };
        writeln!(stdout, "{}", serde_json::to_string(&msg)?)?;
        stdout.flush()?;
        if spent {
            return Ok(());
        }
    }
    Ok(())
}

fn tail(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut start = s.len() - max;
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    format!("... {}", &s[start..])
}
