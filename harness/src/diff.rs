//! `diff A B`: groups that moved between buckets.
//!
//! Groups are matched across the two results by `id`. We report a headline
//! line, a transition matrix, then Improved / Regressed /
//! Changed-within-bucket lists (ordering per `Bucket`'s derived `Ord`,
//! which runs worst-to-best, so `a.bucket < b.bucket` means things got
//! better), then any ids that only exist on one side.

use crate::results::{Bucket, CaseOutcome, GroupResult, Results};
use crate::summary::{draft_rank, escape_code_span, first_line_trunc};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// "Only in A" / "only in B" lists exist to explain a count mismatch, not
/// to be read end to end; cap them. The Improved/Regressed lists are the
/// point of the diff and stay uncapped.
const ONLY_IN_CAP: usize = 50;

/// A group present in both results.
struct Pair<'a> {
    a: &'a GroupResult,
    b: &'a GroupResult,
}

impl Pair<'_> {
    fn id(&self) -> &str {
        &self.b.id
    }
}

/// Per-case outcomes keyed by case description, so that reordering alone
/// doesn't register as a change. Error text is deliberately excluded:
/// only the outcome matters for "changed within the same bucket".
fn case_outcomes(group: &GroupResult) -> BTreeMap<&str, CaseOutcome> {
    group.cases.iter().map(|c| (c.description.as_str(), c.outcome)).collect()
}

/// Case-level outcome changes between two runs of the same group, as
/// readable text (capped).
fn changed_cases(a: &GroupResult, b: &GroupResult) -> Vec<String> {
    let (ao, bo) = (case_outcomes(a), case_outcomes(b));
    let descs: BTreeSet<&str> = ao.keys().chain(bo.keys()).copied().collect();
    descs
        .into_iter()
        .filter_map(|d| {
            let desc = escape_code_span(d);
            match (ao.get(d), bo.get(d)) {
                (Some(x), Some(y)) if x == y => None,
                (Some(x), Some(y)) => Some(format!("{desc}: {x:?} → {y:?}")),
                (Some(_), None) => Some(format!("{desc}: gone")),
                (None, Some(y)) => Some(format!("{desc}: new, {y:?}")),
                (None, None) => None,
            }
        })
        .collect()
}

fn id_list(s: &mut String, ids: &[&str], cap: usize) {
    for id in ids.iter().take(cap) {
        s.push_str(&format!("- `{}`\n", escape_code_span(id)));
    }
    if ids.len() > cap {
        s.push_str(&format!("- … and {} more\n", ids.len() - cap));
    }
    s.push('\n');
}

/// `<glyph> <name> → <glyph> <name>`, plus the first line of the new
/// error when there is one.
fn transition_line(p: &Pair) -> String {
    let mut s = format!(
        "- `{}`: {} {} → {} {}",
        escape_code_span(p.id()),
        p.a.bucket.glyph(),
        p.a.bucket.name(),
        p.b.bucket.glyph(),
        p.b.bucket.name()
    );
    if let Some(err) = &p.b.error {
        let snippet = escape_code_span(&first_line_trunc(err, 160));
        if !snippet.is_empty() {
            s.push_str(&format!(" — `{snippet}`"));
        }
    }
    s.push('\n');
    s
}

fn transition_matrix(pairs: &[Pair]) -> String {
    if pairs.is_empty() {
        return "_no common groups_\n\n".to_string();
    }
    let mut matrix: BTreeMap<(Bucket, Bucket), usize> = BTreeMap::new();
    for p in pairs {
        *matrix.entry((p.a.bucket, p.b.bucket)).or_insert(0) += 1;
    }
    let cell = |rb: Bucket, cb: Bucket| matrix.get(&(rb, cb)).copied().unwrap_or(0);

    let rows: Vec<Bucket> =
        Bucket::ALL.iter().copied().filter(|rb| pairs.iter().any(|p| p.a.bucket == *rb)).collect();
    let cols: Vec<Bucket> =
        Bucket::ALL.iter().copied().filter(|cb| pairs.iter().any(|p| p.b.bucket == *cb)).collect();

    let mut s = String::from("| before ↓ / after → |");
    for cb in &cols {
        s.push_str(&format!(" {} |", cb.glyph()));
    }
    s.push_str(" Total |\n|---|");
    for _ in &cols {
        s.push_str("---:|");
    }
    s.push_str("---:|\n");

    for rb in &rows {
        s.push_str(&format!("| {} |", rb.glyph()));
        for cb in &cols {
            s.push_str(&format!(" {} |", cell(*rb, *cb)));
        }
        s.push_str(&format!(" **{}** |\n", pairs.iter().filter(|p| p.a.bucket == *rb).count()));
    }

    s.push_str("| **Total** |");
    for cb in &cols {
        s.push_str(&format!(" **{}** |", pairs.iter().filter(|p| p.b.bucket == *cb).count()));
    }
    s.push_str(&format!(" **{}** |\n\n", pairs.len()));
    s
}

/// Compare two results files by group id. Returns the markdown diff.
pub fn render(a: &Results, b: &Results) -> String {
    let a_map: HashMap<&str, &GroupResult> =
        a.groups.iter().map(|g| (g.id.as_str(), g)).collect();
    let b_map: HashMap<&str, &GroupResult> =
        b.groups.iter().map(|g| (g.id.as_str(), g)).collect();

    // Walk `a` in suite order and keep the groups present in both, then
    // put the drafts in canonical order (the sort is stable, so suite
    // order survives within a draft).
    let mut pairs: Vec<Pair> = a
        .groups
        .iter()
        .filter_map(|ag| b_map.get(ag.id.as_str()).map(|bg| Pair { a: ag, b: bg }))
        .collect();
    pairs.sort_by_key(|p| draft_rank(&p.a.draft));

    let mut improved: Vec<&Pair> = Vec::new();
    let mut regressed: Vec<&Pair> = Vec::new();
    let mut changed_same: Vec<&Pair> = Vec::new();
    let mut unchanged_bucket_count = 0usize;
    for p in &pairs {
        // Bucket's derived Ord runs worst-to-best, so a < b is better.
        match p.a.bucket.cmp(&p.b.bucket) {
            Ordering::Less => improved.push(p),
            Ordering::Greater => regressed.push(p),
            Ordering::Equal => {
                unchanged_bucket_count += 1;
                if case_outcomes(p.a) != case_outcomes(p.b) {
                    changed_same.push(p);
                }
            }
        }
    }

    let only_a: Vec<&str> =
        a.groups.iter().map(|g| g.id.as_str()).filter(|id| !b_map.contains_key(id)).collect();
    let only_b: Vec<&str> =
        b.groups.iter().map(|g| g.id.as_str()).filter(|id| !a_map.contains_key(id)).collect();

    let mut s = String::new();
    s.push_str(&format!(
        "# Diff: {} ({}) → {} ({})\n\n",
        a.version, a.typify, b.version, b.typify
    ));
    s.push_str(&format!(
        "**{} improved, {} regressed, {} unchanged**",
        improved.len(),
        regressed.len(),
        unchanged_bucket_count
    ));
    if !changed_same.is_empty() {
        s.push_str(&format!(
            " ({} unchanged with different case outcomes)",
            changed_same.len()
        ));
    }
    if !only_a.is_empty() || !only_b.is_empty() {
        s.push_str(&format!(
            "; {} only in {}, {} only in {}",
            only_a.len(),
            a.version,
            only_b.len(),
            b.version
        ));
    }
    s.push_str(".\n\n");
    s.push_str(&Bucket::key_markdown());
    s.push('\n');

    s.push_str("## Transition matrix\n\n");
    s.push_str(&transition_matrix(&pairs));

    s.push_str(&format!("## Improved ({})\n\n", improved.len()));
    if improved.is_empty() {
        s.push_str("_none_\n\n");
    } else {
        for p in &improved {
            s.push_str(&transition_line(p));
        }
        s.push('\n');
    }

    s.push_str(&format!("## Regressed ({})\n\n", regressed.len()));
    if regressed.is_empty() {
        s.push_str("_none_\n\n");
    } else {
        for p in &regressed {
            s.push_str(&transition_line(p));
        }
        s.push('\n');
    }

    s.push_str(&format!("## Changed within the same bucket ({})\n\n", changed_same.len()));
    if changed_same.is_empty() {
        s.push_str("_none_\n\n");
    } else {
        for p in &changed_same {
            let cases = changed_cases(p.a, p.b);
            const CASE_CAP: usize = 4;
            let shown = cases.iter().take(CASE_CAP).cloned().collect::<Vec<_>>().join("; ");
            let more = if cases.len() > CASE_CAP {
                format!("; … and {} more", cases.len() - CASE_CAP)
            } else {
                String::new()
            };
            s.push_str(&format!(
                "- `{}`: {} {} — cases: {shown}{more}\n",
                escape_code_span(p.id()),
                p.b.bucket.glyph(),
                p.b.bucket.name(),
            ));
        }
        s.push('\n');
    }

    if !only_a.is_empty() {
        s.push_str(&format!("## Only in {} ({})\n\n", a.version, only_a.len()));
        id_list(&mut s, &only_a, ONLY_IN_CAP);
    }
    if !only_b.is_empty() {
        s.push_str(&format!("## Only in {} ({})\n\n", b.version, only_b.len()));
        id_list(&mut s, &only_b, ONLY_IN_CAP);
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::CaseResult;

    fn case(description: &str, valid: bool, outcome: CaseOutcome) -> CaseResult {
        CaseResult { description: description.to_string(), valid, outcome, error: None }
    }

    fn group(id: &str, bucket: Bucket, error: Option<&str>, cases: Vec<CaseResult>) -> GroupResult {
        let (draft, rest) = id.split_once('/').unwrap();
        let (file, desc) = rest.split_once("::").unwrap();
        GroupResult {
            id: id.to_string(),
            draft: draft.to_string(),
            file: file.to_string(),
            optional: file.starts_with("optional/"),
            group: desc.to_string(),
            hash: String::new(),
            generated_at: String::new(),
            bucket,
            error: error.map(|e| e.to_string()),
            cases,
        }
    }

    fn results_a() -> Results {
        Results {
            version: "typify1".to_string(),
            typify: "typify 0.10.0-alpha.2".to_string(),
            generated_at: "2026-10-10T00:00:00Z".to_string(),
            groups: vec![
                // Will improve: panic -> pass.
                group("draft7/ref.json::ref panics", Bucket::GenerationPanic, Some("panicked"), vec![]),
                // Will regress: pass -> false_accept.
                group(
                    "draft7/allOf.json::allOf ok",
                    Bucket::Pass,
                    None,
                    vec![case("c1", true, CaseOutcome::Match)],
                ),
                // Stays pass, identical cases: unchanged.
                group(
                    "draft7/type.json::type ok",
                    Bucket::Pass,
                    None,
                    vec![case("c1", true, CaseOutcome::Match)],
                ),
                // Stays false_accept, but which case is offending changes:
                // changed within the same bucket.
                group(
                    "draft7/enum.json::enum loose",
                    Bucket::FalseAccept,
                    Some("accepted bad"),
                    vec![
                        case("first", false, CaseOutcome::FalseAccept),
                        case("second", true, CaseOutcome::Match),
                    ],
                ),
                // Only in A.
                group("draft7/const.json::const only a", Bucket::CompileFail, Some("boom"), vec![]),
            ],
        }
    }

    fn results_b() -> Results {
        Results {
            version: "typify2".to_string(),
            typify: "typify2 0.1.0".to_string(),
            generated_at: "2026-10-10T01:00:00Z".to_string(),
            groups: vec![
                group(
                    "draft7/ref.json::ref panics",
                    Bucket::Pass,
                    None,
                    vec![case("c1", true, CaseOutcome::Match)],
                ),
                group(
                    "draft7/allOf.json::allOf ok",
                    Bucket::FalseAccept,
                    Some("now accepts garbage\nextra context"),
                    vec![case("c1", false, CaseOutcome::FalseAccept)],
                ),
                group(
                    "draft7/type.json::type ok",
                    Bucket::Pass,
                    None,
                    vec![case("c1", true, CaseOutcome::Match)],
                ),
                group(
                    "draft7/enum.json::enum loose",
                    Bucket::FalseAccept,
                    Some("accepted bad"),
                    vec![
                        case("first", true, CaseOutcome::Match),
                        case("second", false, CaseOutcome::FalseAccept),
                    ],
                ),
                // Only in B.
                group("draft7/if-then-else.json::only b", Bucket::Pass, None, vec![]),
            ],
        }
    }

    #[test]
    fn header_and_key_present() {
        let out = render(&results_a(), &results_b());
        assert!(out.contains("typify1"));
        assert!(out.contains("typify2"));
        assert!(out.contains("generation_panic"));
        assert!(out.contains("🤯"));
    }

    #[test]
    fn improved_and_regressed_sections() {
        let out = render(&results_a(), &results_b());
        let improved_pos = out.find("## Improved").unwrap();
        let regressed_pos = out.find("## Regressed").unwrap();
        assert!(improved_pos < regressed_pos);

        assert!(out.contains("`draft7/ref.json::ref panics`: 🤯 generation_panic → ✅ pass"));
        assert!(out.contains("`draft7/allOf.json::allOf ok`: ✅ pass → 🙈 false_accept"));
        // regression includes the first line of the new error.
        assert!(out.contains("now accepts garbage"));
        assert!(!out.contains("extra context"));
    }

    #[test]
    fn changed_within_same_bucket() {
        let out = render(&results_a(), &results_b());
        assert!(out.contains("## Changed within the same bucket (1)"));
        assert!(
            out.contains(
                "`draft7/enum.json::enum loose`: 🙈 false_accept — cases: first: FalseAccept → Match; second: Match → FalseAccept"
            ),
            "got:\n{out}"
        );
        // the truly-identical pass group should not show up here.
        assert!(!out.contains("type ok`: "));
    }

    #[test]
    fn only_in_a_and_b() {
        let out = render(&results_a(), &results_b());
        assert!(out.contains("## Only in typify1 (1)"));
        assert!(out.contains("draft7/const.json::const only a"));
        assert!(out.contains("## Only in typify2 (1)"));
        assert!(out.contains("draft7/if-then-else.json::only b"));
    }

    #[test]
    fn headline_summary_line() {
        let out = render(&results_a(), &results_b());
        // 1 improved (ref), 1 regressed (allOf), unchanged = type ok +
        // enum loose = 2; plus the one-sided ids.
        assert!(out.contains("**1 improved, 1 regressed, 2 unchanged**"));
        assert!(out.contains("(1 unchanged with different case outcomes)"));
        assert!(out.contains("1 only in typify1, 1 only in typify2"));
        // The headline comes before the detail.
        assert!(out.find("1 improved").unwrap() < out.find("## Transition matrix").unwrap());
    }

    #[test]
    fn transition_matrix_rows_and_totals() {
        let out = render(&results_a(), &results_b());
        let table = out.split("## Transition matrix\n\n").nth(1).unwrap();
        // before-buckets ✅ and 🙈 and 🤯; after-buckets ✅ and 🙈.
        assert!(table.contains("| before ↓ / after → | ✅ | 🙈 | Total |"), "got:\n{table}");
        // ✅ before: one stayed pass, one became false_accept.
        assert!(table.contains("| ✅ | 1 | 1 | **2** |"), "got:\n{table}");
        assert!(table.contains("| 🙈 | 0 | 1 | **1** |"), "got:\n{table}");
        assert!(table.contains("| 🤯 | 1 | 0 | **1** |"), "got:\n{table}");
        assert!(table.contains("| **Total** | **2** | **2** | **4** |"), "got:\n{table}");
    }

    #[test]
    fn drafts_in_canonical_order_not_lexical() {
        let mut a = results_a();
        let mut b = results_b();
        for (r, bucket) in [(&mut a, Bucket::CompileFail), (&mut b, Bucket::Pass)] {
            r.groups.push(group("draft2019-09/ref.json::later draft", bucket, Some("x"), vec![]));
            r.groups.push(group("draft4/ref.json::early draft", bucket, Some("x"), vec![]));
        }
        let out = render(&a, &b);
        let d4 = out.find("draft4/ref.json::early draft").unwrap();
        let d7 = out.find("draft7/ref.json::ref panics").unwrap();
        let d2019 = out.find("draft2019-09/ref.json::later draft").unwrap();
        assert!(d4 < d7 && d7 < d2019, "improved list must run draft4, draft7, draft2019-09");
    }

    #[test]
    fn identical_results_report_no_movement() {
        let a = results_a();
        let out = render(&a, &a);
        assert!(out.contains("**0 improved, 0 regressed, 5 unchanged**"));
        assert!(!out.contains("only in"));
        assert!(out.contains("## Improved (0)"));
        assert!(out.contains("_none_"));
    }

    #[test]
    fn disjoint_results_have_no_common_groups() {
        let a = results_a();
        let mut b = results_b();
        b.groups = vec![group("draft6/x.json::x", Bucket::Pass, None, vec![])];
        let out = render(&a, &b);
        assert!(out.contains("_no common groups_"));
        assert!(out.contains("**0 improved, 0 regressed, 0 unchanged**"));
        assert!(out.contains("## Only in typify1 (5)"));
        assert!(out.contains("## Only in typify2 (1)"));
    }

    #[test]
    fn backticks_in_ids_and_cases_are_escaped() {
        let mut a = results_a();
        let mut b = results_b();
        let id = "draft7/optional/format/uri.json::uri";
        a.groups.push(group(
            id,
            Bucket::FalseAccept,
            Some("accepted `invalid ` character`"),
            vec![case("invalid ` character", false, CaseOutcome::FalseAccept)],
        ));
        b.groups.push(group(
            id,
            Bucket::FalseAccept,
            Some("accepted `invalid ` character`"),
            vec![case("invalid ` character", false, CaseOutcome::Match)],
        ));
        let out = render(&a, &b);
        assert!(!out.contains("invalid ` character"), "unescaped backtick leaked into markdown");
        assert!(out.contains("invalid ' character: FalseAccept → Match"));
    }
}
