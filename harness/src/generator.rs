//! The generator contract: a directory under `generators/` with a
//! `generator.toml` naming three commands. The harness never links a
//! generator; it runs these as subprocesses so a generator can be written
//! in any language.
//!
//! ```toml
//! name = "quicktype"
//! targets = ["rust", "go", "typescript"]   # optional; omitted = one target
//! [commands]
//! version  = "..."   # stdout: identity string, e.g. "quicktype 26.0.0 go"
//! generate = "..."   # stdin: [{key, schema}]; writes {out_dir}/<key>.<ext>;
//!                    # stdout: [{key, status: ok|error|panic, message?}]
//! build    = "..."   # stdin: [key]; stdout: {outcomes: {key: "ok"|{fail}},
//!                    #                       runner: [exe, args...]}
//! ```
//!
//! Commands are run through `sh -c` with cwd = the generator directory.
//! Placeholders: `{root}` (repo root), `{target}`, `{out_dir}`, `{gen_dir}`,
//! `{batch_dir}`. Environment: `CARGO_TARGET_DIR=<root>/target`,
//! `BAKEOFF_ROOT`, `BAKEOFF_TARGET`. stderr is inherited so build noise and
//! progress reach the terminal; stdout is the protocol.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    #[serde(default)]
    targets: Vec<String>,
    commands: Commands,
}

#[derive(Debug, Deserialize)]
struct Commands {
    version: String,
    generate: String,
    build: String,
}

#[derive(Debug)]
pub struct Generator {
    pub name: String,
    pub dir: PathBuf,
    root: PathBuf,
    /// Targets from the manifest; empty means the generator has one
    /// implicit target and `{target}` expands to "".
    pub targets: Vec<String>,
    commands: Commands,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenStatus {
    pub key: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildOutcome {
    Ok,
    Fail(String),
}

#[derive(Debug)]
pub struct BuildResult {
    pub outcomes: BTreeMap<String, BuildOutcome>,
    /// Command line for the runner (empty if nothing survived).
    pub runner: Vec<String>,
}

/// Every generator under `<root>/generators/`, by directory name.
pub fn discover(root: &Path) -> Result<Vec<Generator>> {
    let dir = root.join("generators");
    let mut out = Vec::new();
    if dir.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        entries.sort();
        for path in entries {
            if path.join("generator.toml").is_file() {
                out.push(load_dir(root, &path)?);
            }
        }
    }
    Ok(out)
}

/// The generator at `<root>/generators/<name>/`.
pub fn load(root: &Path, name: &str) -> Result<Generator> {
    let dir = root.join("generators").join(name);
    anyhow::ensure!(dir.join("generator.toml").is_file(), "no generator named {name:?} (looked for {})", dir.join("generator.toml").display());
    load_dir(root, &dir)
}

fn load_dir(root: &Path, dir: &Path) -> Result<Generator> {
    let path = dir.join("generator.toml");
    let text = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let m: Manifest = toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(Generator { name: m.name, dir: dir.to_path_buf(), root: root.to_path_buf(), targets: m.targets, commands: m.commands })
}

impl Generator {
    /// Identity string for `target`, e.g. `typify 0.10.0-alpha.2`.
    pub fn version(&self, target: &str) -> Result<String> {
        let out = self.run(&self.commands.version, target, &[], None)?;
        let v = out.trim().to_string();
        anyhow::ensure!(!v.is_empty(), "{}: version command printed nothing", self.name);
        Ok(v)
    }

    /// Generate code for every group into `out_dir`.
    pub fn generate(&self, target: &str, out_dir: &Path, groups: &[(String, serde_json::Value)]) -> Result<Vec<GenStatus>> {
        std::fs::create_dir_all(out_dir)?;
        let input: Vec<serde_json::Value> =
            groups.iter().map(|(key, schema)| serde_json::json!({ "key": key, "schema": schema })).collect();
        let out = self.run(
            &self.commands.generate,
            target,
            &[("{out_dir}", &out_dir.to_string_lossy())],
            Some(&serde_json::to_vec(&input)?),
        )?;
        let statuses: Vec<GenStatus> =
            serde_json::from_str(&out).with_context(|| format!("{}: generate printed something other than a status array", self.name))?;
        Ok(statuses)
    }

    /// Build the generated modules named by `keys` into a runner.
    pub fn build(&self, target: &str, gen_dir: &Path, batch_dir: &Path, keys: &[String]) -> Result<BuildResult> {
        std::fs::create_dir_all(batch_dir)?;
        let out = self.run(
            &self.commands.build,
            target,
            &[("{gen_dir}", &gen_dir.to_string_lossy()), ("{batch_dir}", &batch_dir.to_string_lossy())],
            Some(&serde_json::to_vec(keys)?),
        )?;
        #[derive(Deserialize)]
        struct Raw {
            outcomes: BTreeMap<String, serde_json::Value>,
            #[serde(default)]
            runner: Vec<String>,
        }
        let raw: Raw = serde_json::from_str(&out).with_context(|| format!("{}: build printed something other than a result object", self.name))?;
        let mut outcomes = BTreeMap::new();
        for (k, v) in raw.outcomes {
            let o = match &v {
                serde_json::Value::String(s) if s == "ok" => BuildOutcome::Ok,
                serde_json::Value::Object(m) if m.contains_key("fail") => {
                    BuildOutcome::Fail(m["fail"].as_str().unwrap_or("").to_string())
                }
                other => anyhow::bail!("{}: build outcome for {k:?} is neither \"ok\" nor {{fail}}: {other}", self.name),
            };
            outcomes.insert(k, o);
        }
        Ok(BuildResult { outcomes, runner: raw.runner })
    }

    fn run(&self, template: &str, target: &str, subst: &[(&str, &str)], stdin: Option<&[u8]>) -> Result<String> {
        let mut cmd_line = template.replace("{root}", &self.root.to_string_lossy()).replace("{target}", target);
        for (k, v) in subst {
            cmd_line = cmd_line.replace(k, v);
        }
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(&cmd_line)
            .current_dir(&self.dir)
            .env("CARGO_TARGET_DIR", self.root.join("target"))
            .env("BAKEOFF_ROOT", &self.root)
            .env("BAKEOFF_TARGET", target)
            .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        let mut child = cmd.spawn().with_context(|| format!("{}: spawning `{cmd_line}`", self.name))?;
        if let Some(bytes) = stdin {
            let mut pipe = child.stdin.take().expect("piped stdin");
            // The child may exit early (usage error) without draining stdin.
            let _ = pipe.write_all(bytes);
            drop(pipe);
        }
        let out = child.wait_with_output()?;
        anyhow::ensure!(out.status.success(), "{}: `{cmd_line}` failed with {}", self.name, out.status);
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}
