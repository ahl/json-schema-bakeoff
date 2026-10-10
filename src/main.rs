mod compile;
mod diff;
mod generate;
mod pipeline;
mod results;
mod runner;
mod suite;
mod summary;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Run the JSON Schema test suite through typify")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Version {
    Typify1,
}

impl Version {
    fn label(self) -> &'static str {
        match self {
            Version::Typify1 => "typify1",
        }
    }
}

#[derive(Subcommand)]
enum Cmd {
    /// Generate, compile, and run; write results/<version>/{results.json,summary.md}.
    Run {
        #[arg(long, value_enum, default_value = "typify1")]
        version: Version,
        /// Draft directory under suite/tests (e.g. draft7).
        #[arg(long, default_value = "draft7")]
        draft: String,
        /// Substring filter on `<file>::<group>::<case>`.
        #[arg(long)]
        filter: Option<String>,
        /// Include tests under optional/ (reported separately).
        #[arg(long)]
        optional: bool,
        /// Ignore the cache and redo everything.
        #[arg(long)]
        no_cache: bool,
        /// Write results here instead of results/<version>/.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Compare two results.json files and list groups that moved buckets.
    Diff { a: PathBuf, b: PathBuf },
    /// Re-render summary.md from an existing results.json.
    Summary { results: PathBuf },
    /// Print the bucket key.
    Key,
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Run { version, draft, filter, optional, no_cache, out } => pipeline::run(pipeline::RunOpts {
            version: version.label(),
            draft,
            filter,
            optional,
            no_cache,
            out,
        }),
        Cmd::Diff { a, b } => {
            let a = results::Results::load(&a)?;
            let b = results::Results::load(&b)?;
            print!("{}", diff::render(&a, &b));
            Ok(())
        }
        Cmd::Summary { results } => {
            let r = results::Results::load(&results)?;
            print!("{}", summary::render(&r));
            Ok(())
        }
        Cmd::Key => {
            println!("{}", results::Bucket::key_text());
            Ok(())
        }
    }
}
