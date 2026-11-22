//! Loading the JSON Schema test suite from `suite/validation/tests/` and our
//! own groups from `compound/tests/`.
//!
//! Each file is `{description, tests: [case]}`. A case's `compatibility`
//! names the dialects it applies to. We run each case once, under the newest
//! released dialect it allows, or under `v1` when it allows only unreleased
//! features (`9999`). A case that allows only draft 3 is skipped.

use crate::results::Tier;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Case {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}

#[derive(Debug, Deserialize)]
struct RawFile {
    tests: Vec<RawGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawGroup {
    description: String,
    #[serde(default)]
    compatibility: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    schema: serde_json::Value,
    #[serde(default)]
    external_schemas: BTreeMap<String, serde_json::Value>,
    tests: Vec<Case>,
}

#[derive(Debug, Clone)]
pub struct Group {
    /// Dialect the group runs under: `draft4` ... `draft2020-12`, or `v1`.
    pub draft: String,
    /// Path relative to `suite/validation/tests/`, e.g. `allOf.json` or
    /// `optional/bignum.json`; for our own groups, `compound/` plus the path
    /// relative to `compound/tests/`.
    pub file: String,
    pub tier: Tier,
    pub description: String,
    pub schema: serde_json::Value,
    /// Retrieval URI -> schema, from the case's `externalSchemas`.
    pub external_schemas: BTreeMap<String, serde_json::Value>,
    pub cases: Vec<Case>,
}

impl Group {
    /// `<file>::<group description>`; unique within the suite.
    pub fn id(&self) -> String {
        format!("{}::{}", self.file, self.description)
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

/// Released dialects we run, oldest first, as (`compatibility` number,
/// dialect string passed to tools).
const DIALECTS: &[(u32, &str)] =
    &[(4, "draft4"), (6, "draft6"), (7, "draft7"), (2019, "draft2019-09"), (2020, "draft2020-12")];

/// The `compatibility` number for features not yet in a release.
const UNRELEASED: u32 = 9999;

/// Does a `compatibility` value admit release `version`? `None` admits
/// every release. This follows the suite README's algorithm, including the
/// break once a constraint's release reaches `version`: the constraints are
/// ordered smallest to largest, so the first one at or above `version`
/// decides.
fn is_compatible(compatibility: Option<&str>, version: u32) -> Result<bool> {
    let Some(compatibility) = compatibility else {
        return Ok(true);
    };
    let mut ok = true;
    for constraint in compatibility.split(',') {
        let constraint = constraint.trim();
        let (op, number) = if let Some(n) = constraint.strip_prefix("<=") {
            ("<=", n)
        } else if let Some(n) = constraint.strip_prefix('=') {
            ("=", n)
        } else {
            ("", constraint)
        };
        let release: u32 =
            number.parse().with_context(|| format!("bad compatibility constraint {constraint:?} in {compatibility:?}"))?;
        ok = match op {
            "<=" => version <= release,
            "=" => version == release,
            _ => version >= release,
        };
        if version <= release {
            break;
        }
    }
    Ok(ok)
}

/// The dialect a case runs under: the newest released dialect it admits,
/// else `v1` if it admits `9999`, else `None` (draft 3 only).
fn dialect(compatibility: Option<&str>) -> Result<Option<&'static str>> {
    for (version, name) in DIALECTS.iter().rev() {
        if is_compatible(compatibility, *version)? {
            return Ok(Some(name));
        }
    }
    Ok(is_compatible(compatibility, UNRELEASED)?.then_some("v1"))
}

/// Root of the vendored suite's tests (`suite/validation/tests/`).
pub fn suite_root() -> PathBuf {
    crate::pipeline::repo_root().join("suite").join("validation").join("tests")
}

/// Root of our own groups (`compound/tests/`), in the suite's format.
pub fn compound_root() -> PathBuf {
    crate::pipeline::repo_root().join("compound").join("tests")
}

/// Which tiers a load includes; `Standard` is always included.
#[derive(Debug, Clone, Copy)]
pub struct Tiers {
    pub optional: bool,
    pub compound: bool,
    pub upcoming: bool,
}

impl Tiers {
    fn includes(self, tier: Tier) -> bool {
        match tier {
            Tier::Standard => true,
            Tier::Optional => self.optional,
            Tier::Compound => self.compound,
            Tier::Upcoming => self.upcoming,
        }
    }
}

/// Load every group in the selected tiers, in file order then group order:
/// the official suite's files, then our compound files.
pub fn load(tiers: Tiers) -> Result<Vec<Group>> {
    load_from(&suite_root(), &compound_root(), tiers)
}

fn load_from(suite: &Path, compound: &Path, tiers: Tiers) -> Result<Vec<Group>> {
    anyhow::ensure!(suite.is_dir(), "no such suite directory: {}", suite.display());
    let mut groups = Vec::new();
    for rel in json_files(suite)? {
        load_file(&suite.join(&rel), rel, false, &mut groups)?;
    }
    if compound.is_dir() {
        for rel in json_files(compound)? {
            load_file(&compound.join(&rel), format!("compound/{rel}"), true, &mut groups)?;
        }
    }
    let mut ids = BTreeSet::new();
    for g in &groups {
        anyhow::ensure!(ids.insert(g.id()), "duplicate group id {:?}", g.id());
    }
    groups.retain(|g| tiers.includes(g.tier));
    Ok(groups)
}

fn load_file(path: &Path, file: String, compound: bool, out: &mut Vec<Group>) -> Result<()> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let raw: RawFile = serde_json::from_str(&text)
        .with_context(|| format!("parsing {} (expected an object {{description, tests: [...]}})", path.display()))?;
    let optional_file = file.starts_with("optional/");
    for g in raw.tests {
        let compatibility = g.compatibility.as_deref();
        let draft = dialect(compatibility).with_context(|| format!("{file}::{}", g.description))?;
        let Some(draft) = draft else { continue };
        let tier = if compound {
            Tier::Compound
        } else if draft == "v1" {
            Tier::Upcoming
        } else if optional_file || g.tags.iter().any(|t| t == "format-assertion") {
            Tier::Optional
        } else {
            Tier::Standard
        };
        out.push(Group {
            draft: draft.to_string(),
            file: file.clone(),
            tier,
            description: g.description,
            schema: g.schema,
            external_schemas: g.external_schemas,
            cases: g.tests,
        });
    }
    Ok(())
}

/// Every `.json` file under `base`, as a `/`-separated relative path, sorted.
fn json_files(base: &Path) -> Result<Vec<String>> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                walk(base, &path, out)?;
            } else if path.extension().is_some_and(|e| e == "json") {
                out.push(path.strip_prefix(base)?.to_string_lossy().replace('\\', "/"));
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    walk(base, base, &mut out)?;
    // Plain path order, so `optional/` falls between `oneOf.json` and
    // `pattern.json`.
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: Tiers = Tiers { optional: true, compound: true, upcoming: true };

    /// A scratch tree with `suite/` and `compound/` roots, holding the given
    /// (relative path, file contents) pairs.
    fn tree(name: &str, suite: &[(&str, &str)], compound: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bakeoff-suite-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (dir, files) in [("suite", suite), ("compound", compound)] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            for (rel, text) in files {
                let path = root.join(dir).join(rel);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, text).unwrap();
            }
        }
        root
    }

    fn load_tree(root: &Path, tiers: Tiers) -> Result<Vec<Group>> {
        load_from(&root.join("suite"), &root.join("compound"), tiers)
    }

    /// A file holding one case per (description, extra fields) pair.
    fn file(cases: &[(&str, &str)]) -> String {
        let cases: Vec<String> = cases
            .iter()
            .map(|(d, extra)| {
                format!(
                    r#"{{"description": "{d}", {extra}"schema": {{}}, "tests": [{{"description": "x", "data": 1, "valid": true}}]}}"#
                )
            })
            .collect();
        format!(r#"{{"description": "f", "tests": [{}]}}"#, cases.join(", "))
    }

    /// The README's examples, checked at every release we know about.
    #[test]
    fn compatibility_follows_the_readme() {
        let releases = [3, 4, 6, 7, 2019, 2020, 9999];
        let admitted = |c: &str| -> Vec<u32> {
            releases.iter().copied().filter(|v| is_compatible(Some(c), *v).unwrap()).collect()
        };
        assert_eq!(admitted("7"), [7, 2019, 2020, 9999]);
        assert_eq!(admitted("<=2019"), [3, 4, 6, 7, 2019]);
        assert_eq!(admitted("6,<=2019"), [6, 7, 2019]);
        assert_eq!(admitted("=2020"), [2020]);
        assert_eq!(admitted("=3,7,<=2020"), [3, 7, 2019, 2020]);
        assert_eq!(admitted("<=3,7"), [3, 7, 2019, 2020, 9999]);
        assert_eq!(admitted("9999"), [9999]);
        assert_eq!(admitted("4,<=6,9999"), [4, 6, 9999]);
        assert!(releases.iter().all(|v| is_compatible(None, *v).unwrap()));
        assert!(is_compatible(Some("<7"), 7).is_err());
    }

    #[test]
    fn dialect_is_the_newest_release_then_v1() {
        assert_eq!(dialect(None).unwrap(), Some("draft2020-12"));
        assert_eq!(dialect(Some("<=2019")).unwrap(), Some("draft2019-09"));
        assert_eq!(dialect(Some("4,<=6,9999")).unwrap(), Some("draft6"));
        assert_eq!(dialect(Some("9999")).unwrap(), Some("v1"));
        assert_eq!(dialect(Some("=3")).unwrap(), None);
    }

    #[test]
    fn dialects_and_tiers() {
        let root = tree(
            "tiers",
            &[
                (
                    "a.json",
                    &file(&[
                        ("plain", ""),
                        ("old", r#""compatibility": "<=7", "#),
                        ("draft3 only", r#""compatibility": "=3", "#),
                        ("unreleased", r#""compatibility": "9999", "#),
                    ]),
                ),
                (
                    "format-x.json",
                    &file(&[
                        ("asserts", r#""tags": ["format-assertion"], "#),
                        ("annotates", r#""tags": ["format-annotation"], "#),
                    ]),
                ),
                ("optional/b.json", &file(&[("opt", r#""compatibility": "6", "#)])),
                ("proposals/p/p.json", &file(&[("prop", r#""compatibility": "9999", "#)])),
            ],
            &[("sub/c.json", &file(&[("ours", r#""compatibility": "2019", "#)]))],
        );
        let groups = load_tree(&root, ALL).unwrap();
        let got: Vec<(String, &str, Tier)> = groups.iter().map(|g| (g.id(), g.draft.as_str(), g.tier)).collect();
        let want = [
            ("a.json::plain", "draft2020-12", Tier::Standard),
            ("a.json::old", "draft7", Tier::Standard),
            ("a.json::unreleased", "v1", Tier::Upcoming),
            ("format-x.json::asserts", "draft2020-12", Tier::Optional),
            ("format-x.json::annotates", "draft2020-12", Tier::Standard),
            ("optional/b.json::opt", "draft2020-12", Tier::Optional),
            ("proposals/p/p.json::prop", "v1", Tier::Upcoming),
            ("compound/sub/c.json::ours", "draft2020-12", Tier::Compound),
        ];
        let want: Vec<(String, &str, Tier)> = want.iter().map(|(i, d, t)| (i.to_string(), *d, *t)).collect();
        assert_eq!(got, want);
        assert_eq!(groups[7].file, "compound/sub/c.json");
        assert!(groups[7].matches("compound/sub/c.json::ours"));

        // Each tier option leaves its tier out; Standard always stays.
        let groups = load_tree(&root, Tiers { optional: false, compound: false, upcoming: false }).unwrap();
        assert!(groups.iter().all(|g| g.tier == Tier::Standard));
        assert_eq!(groups.len(), 3);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn external_schemas_are_kept_per_case() {
        let ext = r#""externalSchemas": {"http://example.com/s.json": {"type": "integer"}}, "#;
        let root = tree("external", &[("ref.json", &file(&[("remote", ext), ("local", "")]))], &[]);
        let groups = load_tree(&root, ALL).unwrap();
        assert_eq!(
            groups[0].external_schemas.get("http://example.com/s.json"),
            Some(&serde_json::json!({"type": "integer"}))
        );
        assert!(groups[1].external_schemas.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn duplicate_id_is_an_error() {
        let root = tree("duplicate", &[("a.json", &file(&[("same", ""), ("same", r#""compatibility": "7", "#)]))], &[]);
        let err = load_tree(&root, ALL).unwrap_err();
        assert!(err.to_string().contains("duplicate group id \"a.json::same\""), "{err}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn missing_compound_directory_is_not_an_error() {
        let root = tree("nocompound", &[("a.json", &file(&[("ga", "")]))], &[]);
        std::fs::remove_dir_all(root.join("compound")).unwrap();
        let ids: Vec<String> = load_tree(&root, ALL).unwrap().iter().map(Group::id).collect();
        assert_eq!(ids, ["a.json::ga"]);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn old_compound_layout_is_an_error() {
        let old = r#"[{"description": "g", "schema": {}, "tests": []}]"#;
        let root = tree("oldcompound", &[("a.json", &file(&[("ga", "")]))], &[("draft7/c.json", old)]);
        let err = load_tree(&root, ALL).unwrap_err();
        assert!(format!("{err:#}").contains("expected an object"), "{err:#}");
        let _ = std::fs::remove_dir_all(&root);
    }
}
