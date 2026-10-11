//! json-schema-rs 0.0 generator for the bakeoff harness.
//!
//!   version                → "json-schema-rs <crate version>"
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
use std::path::PathBuf;
use std::time::Duration;

#[path = "../../../rust-batch/src/crashguard.inc.rs"]
mod crashguard;

#[derive(Deserialize)]
struct Group {
    key: String,
    /// Suite directory the schema came from (`draft7`, ...). Selects the
    /// dialect; absent or unrecognized is an `error` for the group.
    #[serde(default)]
    draft: Option<String>,
    schema: serde_json::Value,
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

fn generate_all(out_dir: PathBuf) -> Result<()> {
    std::fs::create_dir_all(&out_dir)?;
    // What the generated modules' derive needs; rust-batch appends this to
    // the batch crate's manifest.
    std::fs::write(out_dir.join("batch-deps.toml"), generate::BATCH_DEPS)?;
    let groups: Vec<Group> =
        serde_json::from_reader(std::io::stdin().lock()).context("reading groups from stdin")?;
    let mut statuses = Vec::with_capacity(groups.len());
    for g in groups {
        // A draft this integration has no `SpecVersion` for is an error for
        // the group rather than a generation against a guessed dialect.
        let spec = match generate::spec_version(g.draft.as_deref()) {
            Ok(spec) => spec,
            Err(message) => {
                statuses.push(Status { key: g.key, status: "error", message: Some(message) });
                continue;
            }
        };
        let status = match generate::generate(&g.schema, spec, TIMEOUT) {
            generate::GenOutcome::Code { code, root } => {
                // The generated buffer is used verbatim: it opens with a
                // `//!` doc comment and `use` items, both of which are fine
                // at the top of a module file, and it never emits
                // `extern crate` or a path rooted at `crate::`.
                let source = format!(
                    "{code}\n\
                     // root type: {root}\n\
                     pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {{\n\
                     \x20   ::serde_json::from_value::<{root}>(v).map(|_| ()).map_err(|e| e.to_string())\n\
                     }}\n"
                );
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
