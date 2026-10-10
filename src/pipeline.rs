//! Orchestration of generate -> compile -> run, with caching.
//! TODO(ahl/claude): implement.

use anyhow::Result;
use std::path::PathBuf;

pub struct RunOpts {
    pub version: &'static str,
    pub draft: String,
    pub filter: Option<String>,
    pub optional: bool,
    pub no_cache: bool,
    pub out: Option<PathBuf>,
}

pub fn run(opts: RunOpts) -> Result<()> {
    let _ = opts;
    anyhow::bail!("not implemented")
}
