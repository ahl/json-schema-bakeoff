//! Result records: what gets written to `results/<version>/results.json`.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Outcome of one schema group under one typify version. Exactly one per
/// group. Ordered worst-to-best so that `max` over cases is meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bucket {
    /// The generator crashed or hung.
    GenerationPanic,
    /// The generator returned an error (including "I can't represent this schema").
    GenerationError,
    /// Generated code didn't compile.
    CompileFail,
    /// A test case panicked (or aborted the process) during deserialize.
    RuntimePanic,
    /// Some case deserialized successfully but was `valid: false`, and
    /// some other case failed to deserialize but was `valid: true`.
    Mixed,
    /// Deserialize succeeded but `valid: false` (typify too loose).
    FalseAccept,
    /// Deserialize failed but `valid: true` (typify too strict).
    FalseReject,
    /// Every case matched.
    Pass,
}

impl Bucket {
    pub const ALL: [Bucket; 8] = [
        Bucket::Pass,
        Bucket::FalseAccept,
        Bucket::FalseReject,
        Bucket::Mixed,
        Bucket::RuntimePanic,
        Bucket::CompileFail,
        Bucket::GenerationError,
        Bucket::GenerationPanic,
    ];

    pub fn glyph(self) -> &'static str {
        match self {
            Bucket::Pass => "✅",
            Bucket::FalseAccept => "🙈",
            Bucket::FalseReject => "🚧",
            Bucket::Mixed => "🤪",
            Bucket::RuntimePanic => "💀",
            Bucket::CompileFail => "🦀",
            Bucket::GenerationError => "❌",
            Bucket::GenerationPanic => "🤯",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Bucket::Pass => "pass",
            Bucket::FalseAccept => "false_accept",
            Bucket::FalseReject => "false_reject",
            Bucket::Mixed => "mixed",
            Bucket::RuntimePanic => "runtime_panic",
            Bucket::CompileFail => "compile_fail",
            Bucket::GenerationError => "generation_error",
            Bucket::GenerationPanic => "generation_panic",
        }
    }

    pub fn describe(self) -> &'static str {
        match self {
            Bucket::Pass => "every case matched",
            Bucket::FalseAccept => "deserialize succeeded but `valid: false` (too loose)",
            Bucket::FalseReject => "deserialize failed but `valid: true` (too strict)",
            Bucket::Mixed => "both false_accept and false_reject cases in one group",
            Bucket::RuntimePanic => "a test case panicked during deserialize",
            Bucket::CompileFail => "generated code doesn't compile",
            Bucket::GenerationError => "the generator returned an error",
            Bucket::GenerationPanic => "the generator crashed or hung",
        }
    }

    /// The key, as markdown, printed at the top of every summary and run.
    pub fn key_markdown() -> String {
        let mut s = String::from("| | bucket | meaning |\n|---|---|---|\n");
        for b in Bucket::ALL {
            s.push_str(&format!("| {} | `{}` | {} |\n", b.glyph(), b.name(), b.describe()));
        }
        s
    }

    /// The key, as plain text, for terminal output.
    pub fn key_text() -> String {
        Bucket::ALL
            .iter()
            .map(|b| format!("{} {:<17} {}", b.glyph(), b.name(), b.describe()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Per-case outcome, recorded once the group reaches the run step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseOutcome {
    /// deserialize success/failure agreed with `valid`.
    Match,
    FalseAccept,
    FalseReject,
    /// deserialize panicked (or the runner process aborted on this case).
    Panic,
    /// the group never reached the run step.
    NotRun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseResult {
    pub description: String,
    pub valid: bool,
    pub outcome: CaseOutcome,
    /// The deserialize error (for a rejected case) or the panic text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupResult {
    /// Unique id: `<draft>/<file>::<group description>`, e.g.
    /// `draft7/allOf.json::allOf with base schema`.
    pub id: String,
    pub draft: String,
    /// Path relative to `suite/tests/<draft>/`, e.g. `allOf.json` or
    /// `optional/format/date.json`.
    pub file: String,
    /// True for anything under `optional/`.
    pub optional: bool,
    pub group: String,
    /// Hash of everything that determined this record (typify version,
    /// schema, cases, harness revision). A rerun reuses the record when
    /// the hash matches, so the results file doubles as the cache.
    #[serde(default)]
    pub hash: String,
    /// When this record was computed (RFC 3339, UTC).
    #[serde(default)]
    pub generated_at: String,
    pub bucket: Bucket,
    /// First error message or panic text, for every non-pass bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub cases: Vec<CaseResult>,
}

impl GroupResult {
    /// Derive the group bucket from per-case outcomes (for groups that
    /// reached the run step).
    pub fn bucket_from_cases(cases: &[CaseResult]) -> Bucket {
        let any = |o: CaseOutcome| cases.iter().any(|c| c.outcome == o);
        if any(CaseOutcome::Panic) {
            Bucket::RuntimePanic
        } else {
            match (any(CaseOutcome::FalseAccept), any(CaseOutcome::FalseReject)) {
                (true, true) => Bucket::Mixed,
                (true, false) => Bucket::FalseAccept,
                (false, true) => Bucket::FalseReject,
                (false, false) => Bucket::Pass,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Results {
    /// Directory name under `results/`, derived from the typify version,
    /// e.g. `typify-0.10.0-alpha.2`.
    pub version: String,
    /// The tool that produced these results, e.g. `typify 0.10.0-alpha.2`.
    pub typify: String,
    /// The generator's description from its manifest (may be empty).
    #[serde(default)]
    pub description: String,
    /// Language of the generated code (`rust`, `go`, `typescript`, ...).
    #[serde(default)]
    pub language: String,
    /// The newest `generated_at` among the groups.
    pub generated_at: String,
    pub groups: Vec<GroupResult>,
}

/// `results/<slug>/` for an identity string like `typify 0.10.0-alpha.2`
/// or `json-schema-to-zod 2.8.1 (zod 4.6.6)`: runs of anything but
/// alphanumerics, `.` and `-` collapse to one `-`, trimmed at the ends.
pub fn version_slug(typify: &str) -> String {
    let mut out = String::new();
    for c in typify.chars() {
        if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
            out.push(c);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Short label for a language, for row names like `ajv (js)`.
pub fn language_label(language: &str) -> &str {
    match language {
        "typescript" => "ts",
        "javascript" => "js",
        other => other,
    }
}

/// The identity string without a trailing language token (`quicktype
/// 26.0.0 rust` -> `quicktype 26.0.0`), for display next to a section or
/// suffix that already says the language.
pub fn display_name(identity: &str, language: &str) -> String {
    let lower = identity.to_ascii_lowercase();
    for token in [language.to_ascii_lowercase(), language_label(language).to_ascii_lowercase()] {
        if !token.is_empty() {
            if let Some(stripped) = lower.strip_suffix(&format!(" {token}")) {
                return identity[..stripped.len()].to_string();
            }
        }
    }
    identity.to_string()
}

impl Results {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let s = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading {}: {e}", path.display()))?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
