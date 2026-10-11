//! jsonschema 0.58 generator for the bakeoff harness.
//!
//!   version                → "jsonschema <crate version>"
//!   generate <out_dir>     → stdin: JSON array of {key, draft, schema}; writes
//!                             <out_dir>/<key>.rs (module + __conformance_check)
//!                             and <out_dir>/batch-deps.toml;
//!                             stdout: JSON array of {key, status, message?}
//!
//! Building and running the generated Rust is delegated to `rust-batch`
//! (see generator.toml).

mod generate;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::path::PathBuf;
use std::time::Duration;

#[path = "../../../rust-batch/src/crashguard.inc.rs"]
mod crashguard;

#[derive(Deserialize)]
struct Group {
    key: String,
    /// The suite directory the group came from (`draft7`, ...). It picks
    /// the `jsonschema::Draft` the module compiles with. Also in the
    /// environment as `BAKEOFF_DRAFT`, but the per-item field is what we
    /// read: it is the one that travels with the schema.
    draft: String,
    /// Kept as raw JSON text so it is embedded byte for byte.
    schema: Box<RawValue>,
}

#[derive(Serialize)]
struct Status {
    key: String,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

const TIMEOUT: Duration = Duration::from_secs(60);

fn main() -> Result<()> {
    crashguard::install();
    let args: Vec<String> = std::env::args().collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        [_, "version"] => {
            println!("{}", generate::VERSION);
            Ok(())
        }
        [_, "generate", out_dir] => generate_all(PathBuf::from(out_dir)),
        _ => anyhow::bail!("usage: version | generate <out_dir>"),
    }
}

/// `suite/remotes` under `BAKEOFF_ROOT`, which the harness always sets.
fn remotes_dir() -> Result<String> {
    let root = std::env::var("BAKEOFF_ROOT").context("BAKEOFF_ROOT is not set")?;
    let dir = PathBuf::from(root).join("suite").join("remotes");
    anyhow::ensure!(dir.is_dir(), "{} is not a directory (is the suite submodule checked out?)", dir.display());
    Ok(dir.to_string_lossy().into_owned())
}

fn generate_all(out_dir: PathBuf) -> Result<()> {
    std::fs::create_dir_all(&out_dir)?;
    let remotes = remotes_dir()?;
    let groups: Vec<Group> = serde_json::from_reader(std::io::stdin().lock()).context("reading groups from stdin")?;

    // The batch crate doesn't know about jsonschema; tell it, with the same
    // pin and the same features the generator itself was built against.
    std::fs::write(
        out_dir.join("batch-deps.toml"),
        format!(
            "jsonschema = {{ version = \"={}\", default-features = false, features = [\"idna\"] }}\n",
            generate::CRATE_VERSION
        ),
    )?;

    let mut statuses = Vec::with_capacity(groups.len());
    for g in groups {
        let status = match generate::generate(g.schema.get(), &g.draft, &remotes, TIMEOUT) {
            generate::GenOutcome::Code(source) => {
                std::fs::write(out_dir.join(format!("{}.rs", g.key)), source)?;
                Status { key: g.key, status: "ok", message: None }
            }
            generate::GenOutcome::Error(m) => Status { key: g.key, status: "error", message: Some(m) },
            generate::GenOutcome::Panic(m) => Status { key: g.key, status: "panic", message: Some(m) },
        };
        statuses.push(status);
    }
    println!("{}", serde_json::to_string(&statuses)?);
    Ok(())
}
