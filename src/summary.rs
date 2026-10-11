//! `summary.md`: key, pass rate per draft, bucket chart per keyword file.
//!
//! This is the report Adam reads after a run. It has three parts:
//! headline tables (bucket counts and pass rate per draft, non-optional
//! and optional reported separately), a per-draft chart (one glyph per
//! group, one row per keyword file, in suite order), and a "Failures"
//! section with the first error and offending cases for every non-pass
//! group.

use crate::results::{Bucket, CaseOutcome, GroupResult, Results};
use std::collections::BTreeMap;

/// Canonical draft ordering for display; anything not listed here sorts
/// after, alphabetically.
const DRAFT_ORDER: &[&str] = &["draft4", "draft6", "draft7", "draft2019-09", "draft2020-12"];

/// Display rank of a draft. Shared with `diff` so both reports order
/// drafts the same way (and not lexically, where draft2019-09 would come
/// before draft4).
pub fn draft_rank(draft: &str) -> usize {
    DRAFT_ORDER.iter().position(|d| *d == draft).unwrap_or(DRAFT_ORDER.len())
}

/// Distinct drafts present in `results`, in canonical/display order.
fn sorted_drafts(results: &Results) -> Vec<String> {
    let mut drafts: Vec<String> = results.groups.iter().map(|g| g.draft.clone()).collect();
    drafts.sort();
    drafts.dedup();
    drafts.sort_by_key(|d| (draft_rank(d), d.clone()));
    drafts
}

/// Escape a plain-text value for use as a markdown table cell.
fn escape_cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Sanitize text before embedding it in a markdown code span: a stray
/// backtick would close the span. Suite case descriptions really do
/// contain one — see `optional/format/uri.json`, "invalid ` character" —
/// and typify's errors quote them back. Pipes are left alone (typify
/// panics love `||`); a code span inside a *table* cell additionally
/// needs `escape_cell`.
pub fn escape_code_span(s: &str) -> String {
    s.replace('`', "'")
}

/// First non-blank line of `s`, trimmed and capped at `max` characters.
pub fn first_line_trunc(s: &str, max: usize) -> String {
    let first = s.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("");
    if first.chars().count() > max {
        let truncated: String = first.chars().take(max).collect();
        format!("{truncated}…")
    } else {
        first.to_string()
    }
}

fn pass_rate(pass: usize, total: usize) -> String {
    if total == 0 {
        "-".to_string()
    } else {
        format!("{:.1}%", (pass as f64 / total as f64) * 100.0)
    }
}

/// Buckets that actually occur in `groups`, in `Bucket::ALL` (best-first)
/// order. Used to pick the count columns for a table so the glyph headers
/// always line up with the values underneath.
fn active_buckets(groups: &[&GroupResult]) -> Vec<Bucket> {
    Bucket::ALL.iter().copied().filter(|b| groups.iter().any(|g| g.bucket == *b)).collect()
}

fn count(groups: &[&GroupResult], bucket: Bucket) -> usize {
    groups.iter().filter(|g| g.bucket == bucket).count()
}

struct FileGroup<'a> {
    file: String,
    groups: Vec<&'a GroupResult>,
}

/// Groups of one draft, split by `optional`, bucketed by file (files
/// sorted alphabetically; groups within a file keep the order they
/// appear in `results.groups`, i.e. suite order).
fn files_for_draft<'a>(results: &'a Results, draft: &str, optional: bool) -> Vec<FileGroup<'a>> {
    let mut map: BTreeMap<String, Vec<&'a GroupResult>> = BTreeMap::new();
    for g in &results.groups {
        if g.draft == draft && g.optional == optional {
            map.entry(g.file.clone()).or_default().push(g);
        }
    }
    map.into_iter().map(|(file, groups)| FileGroup { file, groups }).collect()
}

fn groups_for<'a>(results: &'a Results, draft: &str, optional: bool) -> Vec<&'a GroupResult> {
    results.groups.iter().filter(|g| g.draft == draft && g.optional == optional).collect()
}

fn chart_string(groups: &[&GroupResult]) -> String {
    groups.iter().map(|g| g.bucket.glyph()).collect::<String>()
}

fn case_stats(groups: &[&GroupResult]) -> (usize, usize, usize) {
    let mut matched = 0;
    let mut reached = 0;
    let mut total = 0;
    for g in groups {
        for c in &g.cases {
            total += 1;
            if c.outcome != CaseOutcome::NotRun {
                reached += 1;
            }
            if c.outcome == CaseOutcome::Match {
                matched += 1;
            }
        }
    }
    (matched, reached, total)
}

/// Bucket counts, total and pass rate for `optional` (or non-optional)
/// groups, one row per draft plus a totals row. `None` if there are no
/// matching groups at all.
fn groups_table(results: &Results, drafts: &[String], optional: bool) -> Option<String> {
    let all: Vec<&GroupResult> = results.groups.iter().filter(|g| g.optional == optional).collect();
    if all.is_empty() {
        return None;
    }
    let active = active_buckets(&all);

    let mut s = String::from("| Draft |");
    for b in &active {
        s.push_str(&format!(" {} |", b.glyph()));
    }
    s.push_str(" Total | Pass rate |\n|---|");
    for _ in &active {
        s.push_str("---:|");
    }
    s.push_str("---:|---:|\n");

    for draft in drafts {
        let groups = groups_for(results, draft, optional);
        if groups.is_empty() {
            continue;
        }
        s.push_str(&format!("| {} |", escape_cell(draft)));
        for b in &active {
            s.push_str(&format!(" {} |", count(&groups, *b)));
        }
        s.push_str(&format!(
            " {} | {} |\n",
            groups.len(),
            pass_rate(count(&groups, Bucket::Pass), groups.len())
        ));
    }

    s.push_str("| **Total** |");
    for b in &active {
        s.push_str(&format!(" **{}** |", count(&all, *b)));
    }
    s.push_str(&format!(
        " **{}** | **{}** |\n",
        all.len(),
        pass_rate(count(&all, Bucket::Pass), all.len())
    ));
    Some(s)
}

/// One small table: per-case match rate, reached-run and overall, one row
/// per draft plus a totals row. `None` if there are no cases.
fn case_rate_table(results: &Results, drafts: &[String], optional: bool) -> Option<String> {
    let mut rows = String::new();
    let mut tot_matched = 0;
    let mut tot_reached = 0;
    let mut tot_total = 0;
    for draft in drafts {
        let (matched, reached, total) = case_stats(&groups_for(results, draft, optional));
        if total == 0 {
            continue;
        }
        tot_matched += matched;
        tot_reached += reached;
        tot_total += total;
        rows.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            escape_cell(draft),
            matched,
            reached,
            pass_rate(matched, reached),
            total,
            pass_rate(matched, total)
        ));
    }
    if tot_total == 0 {
        return None;
    }
    let mut s = String::from(
        "| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |\n|---|---:|---:|---:|---:|---:|\n",
    );
    s.push_str(&rows);
    s.push_str(&format!(
        "| **Total** | **{}** | **{}** | **{}** | **{}** | **{}** |\n",
        tot_matched,
        tot_reached,
        pass_rate(tot_matched, tot_reached),
        tot_total,
        pass_rate(tot_matched, tot_total)
    ));
    Some(s)
}

/// File-by-file chart table: one count column per bucket that occurs in
/// this table (in `Bucket::ALL` order), then the chart itself — one glyph
/// per group in suite order. The chart goes last because it can be 40
/// glyphs wide, which would otherwise push the counts off the screen.
fn chart_table(file_groups: &[FileGroup]) -> String {
    let all: Vec<&GroupResult> =
        file_groups.iter().flat_map(|fg| fg.groups.iter().copied()).collect();
    let active = active_buckets(&all);

    let mut s = String::from("| File |");
    for b in &active {
        s.push_str(&format!(" {} |", b.glyph()));
    }
    s.push_str(" Total | Chart |\n|---|");
    for _ in &active {
        s.push_str("---:|");
    }
    s.push_str("---:|---|\n");

    for fg in file_groups {
        s.push_str(&format!("| {} |", escape_cell(&fg.file)));
        for b in &active {
            s.push_str(&format!(" {} |", count(&fg.groups, *b)));
        }
        s.push_str(&format!(" {} | {} |\n", fg.groups.len(), chart_string(&fg.groups)));
    }

    s.push_str("| **Total** |");
    for b in &active {
        s.push_str(&format!(" **{}** |", count(&all, *b)));
    }
    s.push_str(&format!(" **{}** |  |\n", all.len()));
    s
}

/// The offending cases of a group, as readable text (capped).
fn offending_cases(g: &GroupResult) -> Option<String> {
    const CASE_CAP: usize = 6;
    let offending: Vec<String> = g
        .cases
        .iter()
        .filter_map(|c| match c.outcome {
            CaseOutcome::FalseAccept => Some(format!("{} (accepted)", escape_code_span(&c.description))),
            CaseOutcome::FalseReject => Some(format!("{} (rejected)", escape_code_span(&c.description))),
            CaseOutcome::Panic => Some(format!("{} (panicked)", escape_code_span(&c.description))),
            _ => None,
        })
        .collect();
    if offending.is_empty() {
        return None;
    }
    let shown = offending.iter().take(CASE_CAP).cloned().collect::<Vec<_>>().join(", ");
    Some(if offending.len() > CASE_CAP {
        format!("{shown}, … and {} more", offending.len() - CASE_CAP)
    } else {
        shown
    })
}

/// The most common error messages in a bucket, with counts: dozens of
/// groups usually share a handful of root causes (e.g. 17 `format.json`
/// groups all panic with the same line), and that's the number worth
/// reading. `None` when the per-group list below is already short enough
/// to take in at a glance.
fn error_rollup(groups: &[&GroupResult]) -> Option<String> {
    const MIN_GROUPS: usize = 5;
    const MSG_CAP: usize = 5;
    if groups.len() <= MIN_GROUPS {
        return None;
    }
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for g in groups {
        let msg = first_line_trunc(g.error.as_deref().unwrap_or(""), 160);
        *counts.entry(msg).or_insert(0) += 1;
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    // Most common first; ties by message for a stable report.
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut s = format!("{} distinct first-line errors; most common:\n\n", ranked.len());
    for (msg, n) in ranked.iter().take(MSG_CAP) {
        s.push_str(&format!("- **{n}×** `{}`\n", escape_code_span(msg)));
    }
    s.push('\n');
    Some(s)
}

fn failures_section(results: &Results) -> String {
    let mut s = String::from("## Failures\n\n");
    // worst-first: reverse of Bucket::ALL (best-to-worst), skipping Pass.
    let worst_first = Bucket::ALL.iter().rev().copied().filter(|b| *b != Bucket::Pass);

    const CAP: usize = 50;
    let mut any = false;
    for bucket in worst_first {
        let mut groups: Vec<&GroupResult> =
            results.groups.iter().filter(|g| g.bucket == bucket).collect();
        if groups.is_empty() {
            continue;
        }
        any = true;
        // Required groups before optional ones, drafts in canonical order;
        // suite order within (sort_by_key is stable).
        groups.sort_by_key(|g| (g.optional, draft_rank(&g.draft)));
        s.push_str(&format!("### {} `{}` ({})\n\n", bucket.glyph(), bucket.name(), groups.len()));
        if let Some(rollup) = error_rollup(&groups) {
            s.push_str(&rollup);
            s.push_str("Per group:\n\n");
        }
        for g in groups.iter().take(CAP) {
            let err = g.error.as_deref().unwrap_or("");
            let snippet = escape_code_span(&first_line_trunc(err, 160));
            s.push_str(&format!("- `{}` — `{}`\n", escape_code_span(&g.id), snippet));
            if let Some(cases) = offending_cases(g) {
                s.push_str(&format!("  - cases: {cases}\n"));
            }
        }
        if groups.len() > CAP {
            s.push_str(&format!("- … and {} more\n", groups.len() - CAP));
        }
        s.push('\n');
    }
    if !any {
        s.push_str("_none: every group passed._\n\n");
    }
    s
}

/// Render the markdown summary for one results file.
pub fn render(results: &Results) -> String {
    let mut s = String::new();
    s.push_str(&format!("# Conformance summary — {} ({})\n\n", results.version, results.typify));
    s.push_str(&format!("_generated {}_\n\n", results.generated_at));

    let drafts = sorted_drafts(results);
    let required: Vec<&GroupResult> = results.groups.iter().filter(|g| !g.optional).collect();
    let optional_n = results.groups.len() - required.len();
    s.push_str(&format!(
        "**{} of {} non-optional groups pass ({})**{}\n\n",
        count(&required, Bucket::Pass),
        required.len(),
        pass_rate(count(&required, Bucket::Pass), required.len()),
        if optional_n > 0 {
            format!("; {optional_n} optional groups reported separately")
        } else {
            String::new()
        },
    ));

    s.push_str(&Bucket::key_markdown());
    s.push('\n');

    s.push_str("## Headline\n\n");
    s.push_str("### Groups\n\n");
    match groups_table(results, &drafts, false) {
        Some(t) => s.push_str(&t),
        None => s.push_str("_no groups_\n"),
    }
    s.push('\n');
    if let Some(t) = groups_table(results, &drafts, true) {
        s.push_str("### Optional groups\n\n");
        s.push_str(&t);
        s.push('\n');
    }
    if let Some(t) = case_rate_table(results, &drafts, false) {
        s.push_str("### Per-case match rate\n\n");
        s.push_str(&t);
        s.push('\n');
    }
    if let Some(t) = case_rate_table(results, &drafts, true) {
        s.push_str("### Per-case match rate (optional)\n\n");
        s.push_str(&t);
        s.push('\n');
    }

    s.push_str("## Chart\n\n");
    for draft in &drafts {
        s.push_str(&format!("### {draft}\n\n"));
        let non_opt = files_for_draft(results, draft, false);
        if non_opt.is_empty() {
            s.push_str("_no groups_\n\n");
        } else {
            s.push_str(&chart_table(&non_opt));
            s.push('\n');
        }
        let opt = files_for_draft(results, draft, true);
        if !opt.is_empty() {
            s.push_str("#### optional\n\n");
            s.push_str(&chart_table(&opt));
            s.push('\n');
        }
    }

    s.push_str(&failures_section(results));

    s
}

/// One `file  pass/total  chart` line per file, plus a totals line.
fn render_terminal_files(s: &mut String, files: &[FileGroup], indent: &str) {
    let name_width = files.iter().map(|f| f.file.chars().count()).max().unwrap_or(0);
    let counts: Vec<String> = files
        .iter()
        .map(|f| format!("{}/{}", count(&f.groups, Bucket::Pass), f.groups.len()))
        .collect();
    let count_width = counts.iter().map(|c| c.chars().count()).max().unwrap_or(0);

    let all: Vec<&GroupResult> = files.iter().flat_map(|f| f.groups.iter().copied()).collect();
    for (f, c) in files.iter().zip(&counts) {
        s.push_str(&format!(
            "{indent}{:<name_width$}  {:>count_width$}  {}\n",
            f.file,
            c,
            chart_string(&f.groups)
        ));
    }
    s.push_str(&format!("{indent}{}\n", totals_line(&all)));
}

fn totals_line(groups: &[&GroupResult]) -> String {
    let parts: Vec<String> = Bucket::ALL
        .iter()
        .filter_map(|b| match count(groups, *b) {
            0 => None,
            c => Some(format!("{}{}", b.glyph(), c)),
        })
        .collect();
    format!(
        "totals: {} ({} groups, {} pass)",
        parts.join(" "),
        groups.len(),
        pass_rate(count(groups, Bucket::Pass), groups.len())
    )
}

/// Short terminal summary printed at the end of `run`.
pub fn render_terminal(results: &Results) -> String {
    let mut s = String::new();
    s.push_str(&Bucket::key_text());
    s.push_str("\n\n");
    let drafts = sorted_drafts(results);
    for draft in &drafts {
        s.push_str(&format!("{draft}:\n"));
        let non_opt = files_for_draft(results, draft, false);
        if !non_opt.is_empty() {
            render_terminal_files(&mut s, &non_opt, "  ");
        }
        let opt = files_for_draft(results, draft, true);
        if !opt.is_empty() {
            s.push_str("  optional:\n");
            render_terminal_files(&mut s, &opt, "    ");
        }
        s.push('\n');
    }
    if drafts.len() > 1 {
        let required: Vec<&GroupResult> = results.groups.iter().filter(|g| !g.optional).collect();
        s.push_str(&format!("all drafts, non-optional: {}\n", totals_line(&required)));
    }
    s
}

/// `RESULTS.md`: the cross-version scoreboard at the repo root. One
/// headline row per (version, draft), then the chart tables per version,
/// pointing at `results/<version>/summary.md` for the failure detail.
pub fn render_scoreboard(all: &[Results]) -> String {
    let mut s = String::new();
    s.push_str("# typify conformance scoreboard\n\n");
    s.push_str("Regenerated by `cargo run -- run`; per-version detail (including every failing group and its error) is in `results/<version>/summary.md`.\n\n");
    s.push_str(&Bucket::key_markdown());
    s.push('\n');

    s.push_str("## Headline (non-optional groups)\n\n");
    s.push_str("| Version | Draft | Pass | Total | Rate | Cases matched | Generated |\n|---|---|---:|---:|---:|---:|---|\n");
    for r in all {
        for draft in sorted_drafts(r) {
            let groups = groups_for(r, &draft, false);
            if groups.is_empty() {
                continue;
            }
            let (matched, _reached, total_cases) = case_stats(&groups);
            s.push_str(&format!(
                "| {} ({}) | {} | {} | {} | {} | {} | {} |\n",
                escape_cell(&r.version),
                escape_cell(&r.typify),
                draft,
                count(&groups, Bucket::Pass),
                groups.len(),
                pass_rate(count(&groups, Bucket::Pass), groups.len()),
                pass_rate(matched, total_cases),
                r.generated_at,
            ));
        }
    }
    s.push('\n');

    for r in all {
        s.push_str(&format!("## {} ({})\n\n", r.version, r.typify));
        for draft in sorted_drafts(r) {
            let non_opt = files_for_draft(r, &draft, false);
            if non_opt.is_empty() {
                continue;
            }
            s.push_str(&format!("### {draft}\n\n"));
            s.push_str(&chart_table(&non_opt));
            s.push('\n');
            let opt = files_for_draft(r, &draft, true);
            if !opt.is_empty() {
                s.push_str("#### optional\n\n");
                s.push_str(&chart_table(&opt));
                s.push('\n');
            }
        }
        s.push_str(&format!("Details: [results/{0}/summary.md](results/{0}/summary.md)\n\n", r.version));
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

    fn group(
        draft: &str,
        file: &str,
        optional: bool,
        group_desc: &str,
        bucket: Bucket,
        error: Option<&str>,
        cases: Vec<CaseResult>,
    ) -> GroupResult {
        GroupResult {
            id: format!("{draft}/{file}::{group_desc}"),
            draft: draft.to_string(),
            file: file.to_string(),
            optional,
            group: group_desc.to_string(),
            bucket,
            error: error.map(|e| e.to_string()),
            cases,
        }
    }

    /// Two drafts, a couple of files each, one optional file, every
    /// bucket represented at least once.
    fn sample_results() -> Results {
        Results {
            version: "typify1".to_string(),
            typify: "typify 0.10.0-alpha.2".to_string(),
            generated_at: "2026-10-10T00:00:00Z".to_string(),
            groups: vec![
                group(
                    "draft7",
                    "allOf.json",
                    false,
                    "allOf passes",
                    Bucket::Pass,
                    None,
                    vec![case("match case", true, CaseOutcome::Match)],
                ),
                group(
                    "draft7",
                    "allOf.json",
                    false,
                    "allOf too loose",
                    Bucket::FalseAccept,
                    Some("accepted invalid data\nmore detail"),
                    vec![case("extra property", false, CaseOutcome::FalseAccept)],
                ),
                group(
                    "draft7",
                    "ref.json",
                    false,
                    "ref panics",
                    Bucket::GenerationPanic,
                    Some("panicked at 'unsupported `$ref`' src/foo.rs:1"),
                    vec![],
                ),
                group(
                    "draft7",
                    "type.json",
                    false,
                    "type compile fail",
                    Bucket::CompileFail,
                    Some("error[E0308]: mismatched types"),
                    vec![],
                ),
                group(
                    "draft7",
                    "optional/format/date.json",
                    true,
                    "date format too strict",
                    Bucket::FalseReject,
                    Some("missing field `foo`"),
                    vec![case("valid date", true, CaseOutcome::FalseReject)],
                ),
                group(
                    "draft2019-09",
                    "allOf.json",
                    false,
                    "allOf mixed",
                    Bucket::Mixed,
                    Some("mismatch"),
                    vec![
                        case("accepted bad", false, CaseOutcome::FalseAccept),
                        case("rejected good", true, CaseOutcome::FalseReject),
                    ],
                ),
                group(
                    "draft2019-09",
                    "const.json",
                    false,
                    "const runtime panic",
                    Bucket::RuntimePanic,
                    Some("thread panicked during deserialize"),
                    vec![case("weird data", true, CaseOutcome::Panic)],
                ),
                group(
                    "draft2019-09",
                    "enum.json",
                    false,
                    "enum generation error",
                    Bucket::GenerationError,
                    Some("unsupported construct: contains"),
                    vec![],
                ),
            ],
        }
    }

    #[test]
    fn render_includes_key() {
        let out = render(&sample_results());
        assert!(out.contains("generation_panic"));
        assert!(out.contains("false_accept"));
        assert!(out.contains("false_reject"));
        assert!(out.contains("🤯"));
        assert!(out.contains("🙈"));
        assert!(out.contains("🚧"));
    }

    /// The cells of the first table row whose first cell is `first`.
    fn row(out: &str, first: &str) -> Vec<String> {
        let want = format!("| {first} |");
        let line = out
            .lines()
            .find(|l| l.starts_with(&want))
            .unwrap_or_else(|| panic!("no row for {first} in:\n{out}"));
        line.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect()
    }

    #[test]
    fn chart_glyphs_follow_file_order() {
        let out = render(&sample_results());
        // draft7 non-optional: ✅ 🙈 🦀 🤯 count columns, then Total,
        // then the chart — one glyph per group, in suite order.
        assert_eq!(row(&out, "allOf.json"), ["allOf.json", "1", "1", "0", "0", "2", "✅🙈"]);
        assert_eq!(row(&out, "ref.json"), ["ref.json", "0", "0", "0", "1", "1", "🤯"]);
        assert_eq!(row(&out, "type.json"), ["type.json", "0", "0", "1", "0", "1", "🦀"]);
    }

    #[test]
    fn chart_table_headers_line_up_with_counts() {
        let out = render(&sample_results());
        // draft7 non-optional has Pass, FalseAccept, CompileFail,
        // GenerationPanic — in Bucket::ALL (best-first) order.
        let header = "| File | ✅ | 🙈 | 🦀 | 🤯 | Total | Chart |";
        let sep = "|---|---:|---:|---:|---:|---:|---|";
        assert!(out.contains(header), "missing header, got:\n{out}");
        assert!(out.contains(sep));
        // Every row of that table must have the same number of cells as
        // the header.
        let cells = |line: &str| line.matches('|').count();
        let table_start = out.find(header).unwrap();
        for line in out[table_start..].lines().take_while(|l| l.starts_with('|')) {
            assert_eq!(cells(line), cells(header), "misaligned row: {line}");
        }
    }

    #[test]
    fn optional_chart_is_separate() {
        let out = render(&sample_results());
        assert!(out.contains("#### optional"));
        // The only optional group is a false_reject, so that table has a
        // single 🚧 count column.
        assert_eq!(
            row(&out, "optional/format/date.json"),
            ["optional/format/date.json", "1", "1", "🚧"]
        );
    }

    #[test]
    fn headline_pass_rate() {
        let out = render(&sample_results());
        // Non-optional count columns: ✅ 🙈 🔀 💀 🦀 ❌ 🤯 (🚧 only
        // occurs in the optional table), then Total and Pass rate.
        // draft7: 1 pass out of 4 groups.
        assert_eq!(row(&out, "draft7"), ["draft7", "1", "1", "0", "0", "1", "0", "1", "4", "25.0%"]);
        assert_eq!(
            row(&out, "draft2019-09"),
            ["draft2019-09", "0", "0", "1", "1", "0", "1", "0", "3", "0.0%"]
        );
        assert_eq!(
            row(&out, "**Total**"),
            ["**Total**", "**1**", "**1**", "**1**", "**1**", "**1**", "**1**", "**1**", "**7**", "**14.3%**"]
        );
        assert!(out.contains("**1 of 7 non-optional groups pass (14.3%)**"));
        // Optional groups get their own table: draft7, 0 of 1.
        let opt = out.split("### Optional groups\n\n").nth(1).unwrap();
        assert_eq!(row(opt, "draft7"), ["draft7", "1", "1", "0.0%"]);
    }

    #[test]
    fn case_rate_excludes_optional_from_headline() {
        let out = render(&sample_results());
        // draft7 non-optional cases: 2 (one match, one false_accept); the
        // optional date case must not be counted here.
        let non_opt = out.split("### Per-case match rate\n\n").nth(1).expect("per-case table");
        assert_eq!(row(non_opt, "draft7"), ["draft7", "1", "2", "50.0%", "2", "50.0%"]);
        let opt = out.split("### Per-case match rate (optional)\n\n").nth(1).unwrap();
        assert_eq!(row(opt, "draft7"), ["draft7", "0", "1", "0.0%", "1", "0.0%"]);
    }

    #[test]
    fn failures_section_lists_cases_for_false_accept_reject_mixed() {
        let out = render(&sample_results());
        assert!(out.contains("### 🙈 `false_accept` (1)"));
        assert!(out.contains("cases: extra property (accepted)"));
        assert!(out.contains("### 🚧 `false_reject` (1)"));
        assert!(out.contains("cases: valid date (rejected)"));
        assert!(out.contains("### 🔀 `mixed` (1)"));
        assert!(out.contains("cases: accepted bad (accepted), rejected good (rejected)"));
        // generation_panic doesn't get a case list, just the error.
        assert!(out.contains("### 🤯 `generation_panic` (1)"));
        assert!(out.contains("unsupported '$ref'"));
        // runtime_panic names the case that blew up.
        assert!(out.contains("cases: weird data (panicked)"));
    }

    #[test]
    fn case_descriptions_with_backticks_do_not_break_markdown() {
        let mut r = sample_results();
        r.groups.push(group(
            "draft7",
            "optional/format/uri.json",
            true,
            "uri format",
            Bucket::FalseAccept,
            Some("accepted `invalid ` character` though valid: false"),
            vec![case("invalid ` character", false, CaseOutcome::FalseAccept)],
        ));
        let out = render(&r);
        assert!(!out.contains("invalid ` character"), "unescaped backtick leaked into markdown");
        assert!(out.contains("invalid ' character (accepted)"));
    }

    #[test]
    fn failures_roll_up_repeated_errors_and_keep_pipes_readable() {
        let mut r = sample_results();
        for i in 0..7 {
            r.groups.push(group(
                "draft7",
                "format.json",
                false,
                &format!("format {i}"),
                Bucket::GenerationPanic,
                Some("assertion failed: a.is_none() || b.is_none()"),
                vec![],
            ));
        }
        let out = render(&r);
        assert!(out.contains("2 distinct first-line errors; most common:"), "got:\n{out}");
        assert!(out.contains("- **7×** `assertion failed: a.is_none() || b.is_none()`"));
        assert!(out.contains("Per group:"));
        // One-off buckets don't get a rollup.
        assert!(out.contains("### ❌ `generation_error` (1)"));
        let gen_err = out.split("### ❌ `generation_error` (1)\n\n").nth(1).unwrap();
        assert!(gen_err.starts_with("- `draft2019-09/enum.json"), "got:\n{gen_err}");
    }

    #[test]
    fn worst_first_ordering_in_failures() {
        let out = render(&sample_results());
        let panic_pos = out.find("### 🤯").unwrap();
        let error_pos = out.find("### ❌").unwrap();
        let pass_section = out.find("## Failures").unwrap();
        assert!(pass_section < panic_pos);
        assert!(panic_pos < error_pos);
    }

    #[test]
    fn all_pass_results_have_no_failures_and_no_division_by_zero() {
        let results = Results {
            version: "typify1".to_string(),
            typify: "t".to_string(),
            generated_at: "now".to_string(),
            groups: vec![group(
                "draft7",
                "allOf.json",
                false,
                "ok",
                Bucket::Pass,
                None,
                vec![case("c", true, CaseOutcome::Match)],
            )],
        };
        let out = render(&results);
        assert!(out.contains("**1 of 1 non-optional groups pass (100.0%)**"));
        assert!(out.contains("_none: every group passed._"));
        // No optional sections when there are no optional groups.
        assert!(!out.contains("### Optional groups"));
        assert!(!out.contains("#### optional"));
    }

    #[test]
    fn empty_results_render_without_panicking() {
        let results = Results {
            version: "typify1".to_string(),
            typify: "t".to_string(),
            generated_at: "now".to_string(),
            groups: vec![],
        };
        let out = render(&results);
        assert!(out.contains("**0 of 0 non-optional groups pass (-)**"));
        assert!(out.contains("_no groups_"));
        assert!(render_terminal(&results).contains("generation_panic"));
    }

    #[test]
    fn drafts_render_in_canonical_order() {
        let out = render(&sample_results());
        let d7 = out.find("### draft7").unwrap();
        let d2019 = out.find("### draft2019-09").unwrap();
        assert!(d7 < d2019, "draft7 must come before draft2019-09");
        // Same in the headline table.
        let h7 = out.find("| draft7 |").unwrap();
        let h2019 = out.find("| draft2019-09 |").unwrap();
        assert!(h7 < h2019);
    }

    #[test]
    fn render_terminal_has_key_and_glyphs() {
        let out = render_terminal(&sample_results());
        assert!(out.contains("generation_panic"));
        assert!(out.contains("draft7:"));
        assert!(out.contains("allOf.json"));
        assert!(out.contains("✅🙈"));
        assert!(out.contains("optional:"));
        assert!(out.contains("totals:"));
        // pass/total column, and a cross-draft line when >1 draft.
        assert!(out.contains("1/2"));
        assert!(out.contains("all drafts, non-optional: totals:"));
    }
}
