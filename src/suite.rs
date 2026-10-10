//! Loading the JSON Schema test suite from `suite/`.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Case {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct RawGroup {
    description: String,
    schema: serde_json::Value,
    tests: Vec<Case>,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub draft: String,
    /// Path relative to `suite/tests/<draft>/`, e.g. `allOf.json` or
    /// `optional/format/date.json`.
    pub file: String,
    pub optional: bool,
    pub description: String,
    pub schema: serde_json::Value,
    pub cases: Vec<Case>,
}

impl Group {
    /// `<draft>/<file>::<group description>`; unique within the suite.
    pub fn id(&self) -> String {
        format!("{}/{}::{}", self.draft, self.file, self.description)
    }

    /// Does this group (or any of its cases) match a `--filter` string?
    /// The filter is a substring match against `<file>::<group>::<case>`
    /// for each case, so `ref.json`, `ref.json::relative pointer`, and
    /// `allOf::mismatch second` all work.
    pub fn matches(&self, filter: &str) -> bool {
        self.cases
            .iter()
            .any(|c| format!("{}::{}::{}", self.file, self.description, c.description).contains(filter))
    }

    /// Cases matching the filter (all cases when the filter is `None`).
    pub fn filtered_cases<'a>(&'a self, filter: Option<&'a str>) -> impl Iterator<Item = (usize, &'a Case)> + 'a {
        self.cases.iter().enumerate().filter(move |(_, c)| match filter {
            None => true,
            Some(f) => format!("{}::{}::{}", self.file, self.description, c.description).contains(f),
        })
    }
}

/// Root of the vendored suite (`suite/`).
pub fn suite_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("suite")
}

/// Load every group for `draft`, in file order then group order. Optional
/// tests are included when `optional` is true.
pub fn load_draft(draft: &str, optional: bool) -> Result<Vec<Group>> {
    let dir = suite_root().join("tests").join(draft);
    anyhow::ensure!(dir.is_dir(), "no such draft directory: {}", dir.display());
    let mut files = Vec::new();
    collect_json_files(&dir, &dir, optional, &mut files)?;
    files.sort();
    let mut groups = Vec::new();
    for rel in files {
        let path = dir.join(&rel);
        let text = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let raw: Vec<RawGroup> =
            serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        let file = rel.to_string_lossy().replace('\\', "/");
        let is_optional = file.starts_with("optional/");
        for g in raw {
            groups.push(Group {
                draft: draft.to_string(),
                file: file.clone(),
                optional: is_optional,
                description: g.description,
                schema: g.schema,
                cases: g.tests,
            });
        }
    }
    Ok(groups)
}

fn collect_json_files(base: &Path, dir: &Path, optional: bool, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "optional") && !optional {
                continue;
            }
            collect_json_files(base, &path, optional, out)?;
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path.strip_prefix(base)?.to_path_buf());
        }
    }
    Ok(())
}
