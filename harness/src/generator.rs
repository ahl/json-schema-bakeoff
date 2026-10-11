//! The generator contract: a directory under `generators/` with a
//! `generator.toml` naming three commands. The harness never links a
//! generator; it runs these as subprocesses so a generator can be written
//! in any language.
//!
//! ```toml
//! name = "quicktype"
//! description = "What the tool is and what it's designed for."
//! language = "rust"                         # or per target via [languages]
//! kind = "validator"                        # optional; default "generator"
//! targets = ["rust", "go", "typescript"]   # optional; omitted = one target
//! [commands]
//! version  = "..."   # stdout: identity string, e.g. "quicktype 26.0.0 go"
//! generate = "..."   # stdin: [{key, draft, schema}]; writes {out_dir}/<key>.<ext>;
//!                    # stdout: [{key, status: ok|error|panic, message?}]
//! build    = "..."   # stdin: [key]; stdout: {outcomes: {key: "ok"|{fail}},
//!                    #                       runner: [exe, args...]}
//! ```
//!
//! Commands are run through `sh -c` with cwd = the generator directory.
//! Placeholders: `{root}` (repo root), `{target}`, `{out_dir}`, `{gen_dir}`,
//! `{batch_dir}`. Environment: `CARGO_TARGET_DIR=<root>/target`,
//! `BAKEOFF_ROOT`, `BAKEOFF_TARGET`, and for generate/build `BAKEOFF_DRAFT`
//! (the suite directory name, e.g. `draft7`). stderr is inherited so build
//! noise and progress reach the terminal; stdout is the protocol.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Manifest {
    name: String,
    /// A paragraph on what the tool is and its apparent design center,
    /// shown on the scoreboard and the generator's page.
    #[serde(default)]
    description: String,
    #[serde(default)]
    targets: Vec<String>,
    /// Language of the generated code (`rust`, `go`, `typescript`, ...).
    /// With targets, each target's language defaults to the target name;
    /// `[languages]` can override per target.
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    languages: BTreeMap<String, String>,
    /// `generator` (default) or `validator`: a validator row checks
    /// instances against the schema at run time and generates no types.
    #[serde(default)]
    kind: Option<String>,
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
    pub description: String,
    /// `generator` or `validator`.
    pub kind: String,
    language: Option<String>,
    languages: BTreeMap<String, String>,
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
    Ok(Generator {
        name: m.name,
        description: m.description.trim().to_string(),
        kind: m.kind.unwrap_or_else(|| "generator".to_string()),
        language: m.language,
        languages: m.languages,
        dir: dir.to_path_buf(),
        root: root.to_path_buf(),
        targets: m.targets,
        commands: m.commands,
    })
}

impl Generator {
    /// The language of `target`'s generated code.
    pub fn language(&self, target: &str) -> String {
        if let Some(l) = self.languages.get(target) {
            return l.clone();
        }
        if let Some(l) = &self.language {
            return l.clone();
        }
        // No declared language: a target name is the best guess; otherwise
        // leave it empty rather than label the row with a made-up word.
        target.to_string()
    }

    /// A hash of the generator's own sources, so an edit to its glue code
    /// invalidates stored records the way a version bump does. Covers every
    /// regular file under the generator directory except documentation
    /// (`*.md`), dotfiles, and `node_modules`/`target`.
    pub fn source_hash(&self) -> Result<String> {
        use sha2::{Digest, Sha256};
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
            let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            entries.sort();
            for path in entries {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') || name == "node_modules" || name == "target" {
                    continue;
                }
                if path.is_dir() {
                    walk(&path, out)?;
                } else if path.is_file() && !name.ends_with(".md") {
                    out.push(path);
                }
            }
            Ok(())
        }
        let mut files = Vec::new();
        walk(&self.dir, &mut files)?;
        let mut h = Sha256::new();
        for f in files {
            h.update(f.strip_prefix(&self.dir).unwrap_or(&f).to_string_lossy().as_bytes());
            h.update([0]);
            h.update(std::fs::read(&f).with_context(|| format!("reading {}", f.display()))?);
            h.update([0]);
        }
        Ok(hex::encode(h.finalize()))
    }

    /// Identity string for `target`, e.g. `typify 0.10.0-alpha.2`.
    pub fn version(&self, target: &str) -> Result<String> {
        let out = self.run(&self.commands.version, target, &[], None)?;
        let v = out.trim().to_string();
        anyhow::ensure!(!v.is_empty(), "{}: version command printed nothing", self.name);
        Ok(v)
    }

    /// Generate code for every group (all from `draft`) into `out_dir`.
    pub fn generate(&self, target: &str, draft: &str, out_dir: &Path, groups: &[(String, serde_json::Value)]) -> Result<Vec<GenStatus>> {
        std::fs::create_dir_all(out_dir)?;
        let input: Vec<serde_json::Value> = groups
            .iter()
            .map(|(key, schema)| serde_json::json!({ "key": key, "draft": draft, "schema": schema }))
            .collect();
        let out = self.run_in_draft(
            &self.commands.generate,
            target,
            draft,
            &[("{out_dir}", &out_dir.to_string_lossy())],
            Some(&serde_json::to_vec(&input)?),
        )?;
        let statuses: Vec<GenStatus> =
            serde_json::from_str(&out).with_context(|| format!("{}: generate printed something other than a status array", self.name))?;
        Ok(statuses)
    }

    /// Build the generated modules named by `keys` (all from `draft`) into a runner.
    pub fn build(&self, target: &str, draft: &str, gen_dir: &Path, batch_dir: &Path, keys: &[String]) -> Result<BuildResult> {
        std::fs::create_dir_all(batch_dir)?;
        let out = self.run_in_draft(
            &self.commands.build,
            target,
            draft,
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
        self.run_in_draft(template, target, "", subst, stdin)
    }

    fn run_in_draft(&self, template: &str, target: &str, draft: &str, subst: &[(&str, &str)], stdin: Option<&[u8]>) -> Result<String> {
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
            .env("BAKEOFF_DRAFT", draft)
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
