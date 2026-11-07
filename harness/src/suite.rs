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
    /// `optional/format/date.json`; for our own groups, `compound/` plus
    /// the path relative to `compound/tests/<draft>/`.
    pub file: String,
    pub optional: bool,
    /// One of ours, from `compound/tests/<draft>/`, not the official suite.
    pub compound: bool,
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
    crate::pipeline::repo_root().join("suite")
}

/// Root of our own groups (`compound/`), in the suite's format.
pub fn compound_root() -> PathBuf {
    crate::pipeline::repo_root().join("compound")
}

/// Load every group for `draft`, in file order then group order: the
/// official suite's files (`optional/` included when `optional`), then our
/// compound files (when `compound`). A draft with no compound directory
/// just has none.
pub fn load_draft(draft: &str, optional: bool, compound: bool) -> Result<Vec<Group>> {
    load_draft_from(&suite_root(), &compound_root(), draft, optional, compound)
}

fn load_draft_from(suite: &Path, compound_dir: &Path, draft: &str, optional: bool, compound: bool) -> Result<Vec<Group>> {
    let dir = suite.join("tests").join(draft);
    anyhow::ensure!(dir.is_dir(), "no such draft directory: {}", dir.display());
    let mut files = Vec::new();
    collect_json_files(&dir, &dir, optional, &mut files)?;
    // Plain path order, as stored results have always had it (so
    // `optional/` falls between `oneOf.json` and `pattern.json`).
    files.sort();
    let mut groups = Vec::new();
    for rel in files {
        let file = rel.to_string_lossy().replace('\\', "/");
        let is_optional = file.starts_with("optional/");
        load_file(&dir.join(&rel), draft, file, is_optional, false, &mut groups)?;
    }
    let cdir = compound_dir.join("tests").join(draft);
    if compound && cdir.is_dir() {
        let mut files = Vec::new();
        collect_json_files(&cdir, &cdir, true, &mut files)?;
        files.sort();
        for rel in files {
            let file = format!("compound/{}", rel.to_string_lossy().replace('\\', "/"));
            load_file(&cdir.join(&rel), draft, file, false, true, &mut groups)?;
        }
    }
    Ok(groups)
}

fn load_file(path: &Path, draft: &str, file: String, optional: bool, compound: bool, out: &mut Vec<Group>) -> Result<()> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let raw: Vec<RawGroup> = serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    for g in raw {
        out.push(Group {
            draft: draft.to_string(),
            file: file.clone(),
            optional,
            compound,
            description: g.description,
            schema: g.schema,
            cases: g.tests,
        });
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch tree with `suite/tests/draft7/{a.json, optional/b.json}`
    /// and, when `with_compound`, `compound/tests/draft7/sub/c.json`.
    fn tree(name: &str, with_compound: bool) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bakeoff-suite-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let group = |d: &str| format!(r#"[{{"description": "{d}", "schema": {{}}, "tests": [{{"description": "x", "data": 1, "valid": true}}]}}]"#);
        let core = root.join("suite/tests/draft7");
        std::fs::create_dir_all(core.join("optional")).unwrap();
        std::fs::write(core.join("a.json"), group("ga")).unwrap();
        std::fs::write(core.join("optional/b.json"), group("gb")).unwrap();
        if with_compound {
            let c = root.join("compound/tests/draft7/sub");
            std::fs::create_dir_all(&c).unwrap();
            std::fs::write(c.join("c.json"), group("gc")).unwrap();
        }
        root
    }

    #[test]
    fn compound_groups_follow_the_suite_with_their_own_ids() {
        let root = tree("compound", true);
        let groups = load_draft_from(&root.join("suite"), &root.join("compound"), "draft7", true, true).unwrap();
        let ids: Vec<String> = groups.iter().map(Group::id).collect();
        assert_eq!(ids, ["draft7/a.json::ga", "draft7/optional/b.json::gb", "draft7/compound/sub/c.json::gc"]);
        let c = &groups[2];
        assert_eq!(c.file, "compound/sub/c.json");
        assert!(c.compound && !c.optional);
        assert!(!groups[0].compound && !groups[1].compound && groups[1].optional);
        assert!(c.matches("compound/sub/c.json::gc"));

        // --no-compound leaves them out.
        let groups = load_draft_from(&root.join("suite"), &root.join("compound"), "draft7", true, false).unwrap();
        assert!(groups.iter().all(|g| !g.compound));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn missing_compound_directory_is_not_an_error() {
        let root = tree("nocompound", false);
        let groups = load_draft_from(&root.join("suite"), &root.join("compound"), "draft7", false, true).unwrap();
        let ids: Vec<String> = groups.iter().map(Group::id).collect();
        assert_eq!(ids, ["draft7/a.json::ga"]);
        let _ = std::fs::remove_dir_all(&root);
    }
}
