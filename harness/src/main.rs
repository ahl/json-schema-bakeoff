mod diff;
mod generator;
mod pipeline;
mod results;
mod runner;
mod suite;
mod summary;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Run the JSON Schema test suite through code generators and keep score")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the suite through generators; store results/<version>/results.json and render.
    Run {
        /// Generator directory under generators/ (default: every generator).
        #[arg(long)]
        generator: Option<String>,
        /// Target within the generator, e.g. go (default: every target it lists).
        #[arg(long)]
        target: Option<String>,
        /// Draft directory under suite/tests (e.g. draft7), or `all`.
        #[arg(long, default_value = "all")]
        draft: String,
        /// Substring filter on `<file>::<group>::<case>`.
        #[arg(long)]
        filter: Option<String>,
        /// Skip tests under optional/ (they are reported separately, never
        /// counted in headline numbers).
        #[arg(long)]
        no_optional: bool,
        /// Recompute every selected group even if the stored results
        /// already have a record with a matching hash.
        #[arg(long)]
        no_cache: bool,
        /// Write results here instead of results/<version>/.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Re-render every results/*/README.md and RESULTS.md from stored
    /// results, without running anything.
    Render,
    /// Compare two results.json files and list groups that moved buckets.
    Diff { a: PathBuf, b: PathBuf },
    /// List the generators and targets found under generators/.
    Generators,
    /// Print the bucket key.
    Key,
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Run { generator, target, draft, filter, no_optional, no_cache, out } => {
            let drafts: Vec<String> = if draft == "all" {
                summary::DRAFT_ORDER.iter().map(|d| d.to_string()).collect()
            } else {
                vec![draft]
            };
            pipeline::run(pipeline::RunOpts { generator, target, drafts, filter, optional: !no_optional, no_cache, out })
        }
        Cmd::Render => pipeline::render_all(),
        Cmd::Diff { a, b } => {
            let a = results::Results::load(&a)?;
            let b = results::Results::load(&b)?;
            print!("{}", diff::render(&a, &b));
            Ok(())
        }
        Cmd::Generators => {
            for g in generator::discover(&pipeline::repo_root())? {
                if g.targets.is_empty() {
                    println!("{}", g.name);
                } else {
                    println!("{}  targets: {}", g.name, g.targets.join(", "));
                }
            }
            Ok(())
        }
        Cmd::Key => {
            println!("{}", results::Bucket::key_text());
            Ok(())
        }
    }
}
