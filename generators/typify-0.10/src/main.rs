//! typify 0.10 generator for the bakeoff harness.
//!
//!   version                → "typify <crate version>"
//!   generate <out_dir>     → stdin: JSON array of {key, schema}; writes
//!                             <out_dir>/<key>.rs (module + __conformance_check);
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
    let groups: Vec<Group> = serde_json::from_reader(std::io::stdin().lock()).context("reading groups from stdin")?;
    let mut statuses = Vec::with_capacity(groups.len());
    for g in groups {
        let status = match generate::generate(&g.schema, TIMEOUT) {
            generate::GenOutcome::Code { code, root } => {
                let source = format!(
                    "// root type: {root}\n\n{code}\n\
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
