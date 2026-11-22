//! `diff A B`: groups that moved between buckets.
//!
//! Groups are matched across the two results by `id`. We report a headline
//! line, a transition matrix, then Improved / Regressed /
//! Changed-within-bucket lists (ordering per `Bucket`'s derived `Ord`,
//! which runs worst-to-best, so `a.bucket < b.bucket` means things got
//! better), then any ids that only exist on one side. Each list is split by
//! tier, and each id carries the dialect it ran under.

use crate::results::{Bucket, CaseOutcome, GroupResult, Results, Tier};
use crate::summary::{TIERS, dialect_label, escape_code_span, first_line_trunc, labeled_id, tier_name};
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

/// `items`, in any order, as one list per tier under a `###`
/// heading, each item rendered by `line`. At most `cap` items appear in all.
fn tiered<T>(s: &mut String, items: &[T], tier: impl Fn(&T) -> Tier, line: impl Fn(&T) -> String, cap: usize) {
    if items.is_empty() {
        s.push_str("_none_\n\n");
        return;
    }
    let mut shown = 0;
    for t in TIERS {
        let in_tier: Vec<&T> = items.iter().filter(|i| tier(i) == t).collect();
        if in_tier.is_empty() || shown == cap {
            continue;
        }
        s.push_str(&format!("### {}\n\n", tier_name(t)));
        for item in in_tier.iter().take(cap - shown) {
            s.push_str(&line(item));
            shown += 1;
        }
        s.push('\n');
    }
    if items.len() > cap {
        s.push_str(&format!("… and {} more\n\n", items.len() - cap));
    }
}

/// The group label for a pair: both dialects, `a` then `b`, when they
/// differ, as in `ref.json::x (2020-12, then v1)`.
fn pair_label(p: &Pair) -> String {
    if p.a.draft == p.b.draft {
        return labeled_id(p.b);
    }
    escape_code_span(&format!(
        "{} ({}, then {})",
        p.b.id,
        dialect_label(&p.a.draft),
        dialect_label(&p.b.draft)
    ))
}

fn id_line(g: &&GroupResult) -> String {
    format!("- `{}`\n", labeled_id(g))
}

/// `<glyph> <name> → <glyph> <name>`, plus the first line of the new
/// error when there is one.
fn transition_line(p: &Pair) -> String {
    let mut s = format!(
        "- `{}`: {} {} → {} {}",
        pair_label(p),
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

/// Compare two results files by group id. Returns the markdown diff: a
/// headline line, the bucket key, the transition matrix, and the Improved,
/// Regressed, Changed-within-bucket and one-sided lists, each split by tier.
pub fn render(a: &Results, b: &Results) -> String {
    let a_map: HashMap<&str, &GroupResult> =
        a.groups.iter().map(|g| (g.id.as_str(), g)).collect();
    let b_map: HashMap<&str, &GroupResult> =
        b.groups.iter().map(|g| (g.id.as_str(), g)).collect();

    // Walk `a` in suite order and keep the groups present in both. `tiered`
    // groups them by tier at report time.
    let pairs: Vec<Pair> = a
        .groups
        .iter()
        .filter_map(|ag| b_map.get(ag.id.as_str()).map(|bg| Pair { a: ag, b: bg }))
        .collect();

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

    let only_a: Vec<&GroupResult> =
        a.groups.iter().filter(|g| !b_map.contains_key(g.id.as_str())).collect();
    let only_b: Vec<&GroupResult> =
        b.groups.iter().filter(|g| !a_map.contains_key(g.id.as_str())).collect();

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

    let pair_tier = |p: &&Pair| p.b.tier;

    s.push_str(&format!("## Improved ({})\n\n", improved.len()));
    tiered(&mut s, &improved, pair_tier, |p| transition_line(p), usize::MAX);

    s.push_str(&format!("## Regressed ({})\n\n", regressed.len()));
    tiered(&mut s, &regressed, pair_tier, |p| transition_line(p), usize::MAX);

    s.push_str(&format!("## Changed within the same bucket ({})\n\n", changed_same.len()));
    tiered(
        &mut s,
        &changed_same,
        pair_tier,
        |p| {
            let cases = changed_cases(p.a, p.b);
            const CASE_CAP: usize = 4;
            let shown = cases.iter().take(CASE_CAP).cloned().collect::<Vec<_>>().join("; ");
            let more = if cases.len() > CASE_CAP {
                format!("; … and {} more", cases.len() - CASE_CAP)
            } else {
                String::new()
            };
            format!(
                "- `{}`: {} {} — cases: {shown}{more}\n",
                pair_label(p),
                p.b.bucket.glyph(),
                p.b.bucket.name(),
            )
        },
        usize::MAX,
    );

    if !only_a.is_empty() {
        s.push_str(&format!("## Only in {} ({})\n\n", a.version, only_a.len()));
        tiered(&mut s, &only_a, |g| g.tier, id_line, ONLY_IN_CAP);
    }
    if !only_b.is_empty() {
        s.push_str(&format!("## Only in {} ({})\n\n", b.version, only_b.len()));
        tiered(&mut s, &only_b, |g| g.tier, id_line, ONLY_IN_CAP);
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::{CaseResult, Tier};

    fn case(description: &str, valid: bool, outcome: CaseOutcome) -> CaseResult {
        CaseResult { description: description.to_string(), valid, outcome, error: None }
    }

    /// A group from `<dialect>/<id>`, e.g. `draft7/ref.json::ref panics`.
    fn group(spec: &str, bucket: Bucket, error: Option<&str>, cases: Vec<CaseResult>) -> GroupResult {
        let (draft, id) = spec.split_once('/').unwrap();
        let (file, desc) = id.split_once("::").unwrap();
        GroupResult {
            id: id.to_string(),
            draft: draft.to_string(),
            file: file.to_string(),
            tier: if draft == "v1" {
                Tier::Upcoming
            } else if file.starts_with("compound/") {
                Tier::Compound
            } else if file.starts_with("optional/") {
                Tier::Optional
            } else {
                Tier::Standard
            },
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
            description: String::new(),
            language: String::new(),
            kind: String::new(),
            hidden: false,
            generator: String::new(),
            homepage: String::new(),
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
            description: String::new(),
            language: String::new(),
            kind: String::new(),
            hidden: false,
            generator: String::new(),
            homepage: String::new(),
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

        assert!(out.contains("`ref.json::ref panics (7)`: 🤯 generation_panic → ✅ pass"));
        assert!(out.contains("`allOf.json::allOf ok (7)`: ✅ pass → 🙈 false_accept"));
        // regression includes the first line of the new error.
        assert!(out.contains("now accepts garbage"));
        assert!(!out.contains("extra context"));
    }

    #[test]
    fn pair_label_shows_both_dialects_when_they_differ() {
        let a = results_a();
        let mut b = results_b();
        let g = b.groups.iter_mut().find(|g| g.id == "ref.json::ref panics").unwrap();
        g.draft = "v1".to_string();
        let out = render(&a, &b);
        assert!(
            out.contains("`ref.json::ref panics (7, then v1)`: 🤯 generation_panic → ✅ pass"),
            "got:\n{out}"
        );
        // Unchanged dialects keep the single label.
        assert!(out.contains("`allOf.json::allOf ok (7)`: "));
    }

    #[test]
    fn changed_within_same_bucket() {
        let out = render(&results_a(), &results_b());
        assert!(out.contains("## Changed within the same bucket (1)"));
        assert!(
            out.contains(
                "`enum.json::enum loose (7)`: 🙈 false_accept — cases: first: FalseAccept → Match; second: Match → FalseAccept"
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
        assert!(out.contains("- `const.json::const only a (7)`"));
        assert!(out.contains("## Only in typify2 (1)"));
        assert!(out.contains("- `if-then-else.json::only b (7)`"));
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
    fn lists_split_by_tier_in_report_order() {
        let mut a = results_a();
        let mut b = results_b();
        for (r, bucket) in [(&mut a, Bucket::CompileFail), (&mut b, Bucket::Pass)] {
            r.groups.insert(0, group("v1/proposals/p/p.json::upcoming", bucket, Some("x"), vec![]));
            r.groups.insert(0, group("draft2020-12/compound/c.json::compound", bucket, Some("x"), vec![]));
            r.groups.insert(0, group("draft4/optional/o.json::optional", bucket, Some("x"), vec![]));
        }
        let out = render(&a, &b);
        let improved = out.split("## Improved (4)\n\n").nth(1).unwrap();
        let improved = improved.split("\n## ").next().unwrap();
        let pos = |s: &str| improved.find(s).unwrap_or_else(|| panic!("no {s} in:\n{improved}"));
        assert!(pos("### Standard") < pos("ref.json::ref panics (7)"));
        assert!(pos("ref.json::ref panics (7)") < pos("### Optional"));
        assert!(pos("### Optional") < pos("optional/o.json::optional (4)"));
        assert!(pos("optional/o.json::optional (4)") < pos("### Compound"));
        assert!(pos("### Compound") < pos("compound/c.json::compound (2020-12)"));
        assert!(pos("compound/c.json::compound (2020-12)") < pos("### Upcoming"));
        assert!(pos("### Upcoming") < pos("proposals/p/p.json::upcoming (v1)"));
        // A list with groups in one tier only has one heading.
        let regressed = out.split("## Regressed (1)\n\n").nth(1).unwrap();
        assert!(regressed.starts_with("### Standard\n\n- `allOf.json::allOf ok (7)`"), "got:\n{regressed}");
        assert!(!regressed.split("\n## ").next().unwrap().contains("### Optional"));
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
