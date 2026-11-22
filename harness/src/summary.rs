//! `README.md` per results file, `RESULTS.md`, and the terminal summary.
//!
//! The per-tool page is the report Adam reads after a run. After a short
//! header it has one section per tier present (Standard, Optional, Compound,
//! Upcoming), each with bucket counts, the per-case match rate, a chart (one
//! glyph per group, one row per keyword file, in suite order), and a
//! "Failures" list with the first error and offending cases for every
//! non-pass group.

use crate::results::{Bucket, CaseOutcome, GroupResult, Results, Tier};
use std::collections::BTreeMap;

/// Every tier, in the order the reports list them.
pub const TIERS: [Tier; 4] = [Tier::Standard, Tier::Optional, Tier::Compound, Tier::Upcoming];

/// Heading for a tier: `Standard`, `Optional`, `Compound`, `Upcoming`.
pub fn tier_name(tier: Tier) -> &'static str {
    match tier {
        Tier::Standard => "Standard",
        Tier::Optional => "Optional",
        Tier::Compound => "Compound",
        Tier::Upcoming => "Upcoming",
    }
}

/// Short form of a dialect for display next to a group id: `draft2020-12`
/// is `2020-12`, `draft7` is `7`, and `v1` stays `v1`.
pub fn dialect_label(draft: &str) -> &str {
    draft.strip_prefix("draft").unwrap_or(draft)
}

/// A group id followed by its dialect, e.g. `ref.json::root pointer ref
/// (2020-12)`, sanitized for a code span.
pub fn labeled_id(g: &GroupResult) -> String {
    escape_code_span(&format!("{} ({})", g.id, dialect_label(&g.draft)))
}

/// Escape a plain-text value for use as a markdown table cell.
fn escape_cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Sanitize text before embedding it in a markdown code span: a stray backtick
/// would close the span. Suite case descriptions really do contain one — see
/// `format-uri.json`, "invalid ` character" — and typify's errors quote them
/// back. Pipes are left alone (typify panics love `||`); a code span inside a
/// *table* cell additionally needs `escape_cell`.
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

/// Every bucket, in `Bucket::ALL` order, whether or not `groups` has any:
/// every table has the same columns, so pages compare side by side.
fn active_buckets(_groups: &[&GroupResult]) -> Vec<Bucket> {
    Bucket::ALL.to_vec()
}

fn count(groups: &[&GroupResult], bucket: Bucket) -> usize {
    groups.iter().filter(|g| g.bucket == bucket).count()
}

struct FileGroup<'a> {
    file: String,
    groups: Vec<&'a GroupResult>,
}

/// Groups of one tier, bucketed by file (files sorted alphabetically;
/// groups within a file keep the order they appear in `results.groups`,
/// i.e. suite order).
fn files_in<'a>(results: &'a Results, tier: Tier) -> Vec<FileGroup<'a>> {
    let mut map: BTreeMap<String, Vec<&'a GroupResult>> = BTreeMap::new();
    for g in &results.groups {
        if g.tier == tier {
            map.entry(g.file.clone()).or_default().push(g);
        }
    }
    map.into_iter().map(|(file, groups)| FileGroup { file, groups }).collect()
}

fn groups_in(results: &Results, tier: Tier) -> Vec<&GroupResult> {
    results.groups.iter().filter(|g| g.tier == tier).collect()
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

/// Bucket counts, total and pass rate for `groups`, as a one-row table.
fn groups_table(groups: &[&GroupResult]) -> String {
    let active = active_buckets(groups);
    let mut s = String::from("|");
    for b in &active {
        s.push_str(&format!(" {} |", b.glyph()));
    }
    s.push_str(" Total | Pass rate |\n|");
    for _ in &active {
        s.push_str("---:|");
    }
    s.push_str("---:|---:|\n|");
    for b in &active {
        s.push_str(&format!(" {} |", count(groups, *b)));
    }
    s.push_str(&format!(
        " {} | {} |\n",
        groups.len(),
        pass_rate(count(groups, Bucket::Pass), groups.len())
    ));
    s
}

/// Per-case match rate for `groups`, reached-run and overall, as a one-row
/// table. `None` if there are no cases.
fn case_rate_table(groups: &[&GroupResult]) -> Option<String> {
    let (matched, reached, total) = case_stats(groups);
    if total == 0 {
        return None;
    }
    Some(format!(
        "| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |\n\
         |---:|---:|---:|---:|---:|\n\
         | {} | {} | {} | {} | {} |\n",
        matched,
        reached,
        pass_rate(matched, reached),
        total,
        pass_rate(matched, total)
    ))
}

/// File-by-file chart table: one count column per bucket (in `Bucket::ALL`
/// order), then the chart itself, one glyph per group in suite order. The chart
/// goes last because it can be 40 glyphs wide, which would otherwise push the
/// counts off the screen.
fn chart_table(file_groups: &[FileGroup]) -> String {
    let all: Vec<&GroupResult> =
        file_groups.iter().flat_map(|fg| fg.groups.iter().copied()).collect();
    let active = active_buckets(&all);

    let mut s = String::from("| File |");
    for b in &active {
        s.push_str(&format!(" {} |", b.glyph()));
    }
    s.push_str(" Total | Per group |\n|---|");
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

/// Every non-pass group in `groups`, worst bucket first, in suite order
/// within a bucket.
fn failures_section(groups: &[&GroupResult]) -> String {
    let mut s = String::from("### Failures\n\n");
    // worst-first: reverse of Bucket::ALL (best-to-worst), skipping Pass.
    let worst_first = Bucket::ALL.iter().rev().copied().filter(|b| *b != Bucket::Pass);

    const CAP: usize = 50;
    let mut any = false;
    for bucket in worst_first {
        let groups: Vec<&GroupResult> = groups.iter().copied().filter(|g| g.bucket == bucket).collect();
        if groups.is_empty() {
            continue;
        }
        any = true;
        s.push_str(&format!("#### {} `{}` ({})\n\n", bucket.glyph(), bucket.name(), groups.len()));
        if let Some(rollup) = error_rollup(&groups) {
            s.push_str(&rollup);
            s.push_str("Per group:\n\n");
        }
        for g in groups.iter().take(CAP) {
            let err = g.error.as_deref().unwrap_or("");
            let snippet = escape_code_span(&first_line_trunc(err, 160));
            s.push_str(&format!("- `{}` — `{}`\n", labeled_id(g), snippet));
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

/// Render the markdown page for one results file.
///
/// The header has the identity, description, generation time, the Standard
/// pass rate and the bucket key. Then each tier with any groups gets a
/// section, in `TIERS` order: bucket counts, the per-case match rate, the
/// per-file chart, and the failures. A group named in the failures carries
/// its dialect (`labeled_id`).
pub fn render(results: &Results) -> String {
    let mut s = String::new();
    s.push_str(&format!("# {}\n\n", results.typify));
    if !results.description.is_empty() {
        s.push_str(&format!("{}\n\n", results.description));
    }
    s.push_str(&format!("_generated {}_\n\n", results.generated_at));

    let standard = groups_in(results, Tier::Standard);
    s.push_str(&format!(
        "**{} of {} standard groups pass ({})**\n\n",
        count(&standard, Bucket::Pass),
        standard.len(),
        pass_rate(count(&standard, Bucket::Pass), standard.len()),
    ));

    s.push_str(&Bucket::key_markdown());
    s.push('\n');

    if results.groups.is_empty() {
        s.push_str("_no groups_\n");
    }
    for tier in TIERS {
        let groups = groups_in(results, tier);
        if groups.is_empty() {
            continue;
        }
        s.push_str(&format!("## {}\n\n", tier_name(tier)));
        s.push_str(&groups_table(&groups));
        s.push('\n');
        if let Some(t) = case_rate_table(&groups) {
            s.push_str("### Per-case match rate\n\n");
            s.push_str(&t);
            s.push('\n');
        }
        s.push_str("### Chart\n\n");
        s.push_str(&chart_table(&files_in(results, tier)));
        s.push('\n');
        s.push_str(&failures_section(&groups));
    }

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

/// Short terminal summary printed at the end of `run`: the bucket key, then
/// one block per tier with any groups, each a line per file and a totals
/// line.
pub fn render_terminal(results: &Results) -> String {
    let mut s = String::new();
    s.push_str(&Bucket::key_text());
    s.push_str("\n\n");
    for tier in TIERS {
        let files = files_in(results, tier);
        if files.is_empty() {
            continue;
        }
        s.push_str(&format!("{}:\n", tier_name(tier).to_ascii_lowercase()));
        render_terminal_files(&mut s, &files, "  ");
        s.push('\n');
    }
    s
}

/// Appended to a validator row's name; links to the note at the bottom.
const VALIDATOR_MARK: &str = " [𝓥](#notes)";

/// A row's name cell: a link to its page, with the language outside Rust
/// and the validator mark.
fn row_name(r: &Results, show_language: bool) -> String {
    let mut name = crate::results::display_name(&r.typify, &r.language);
    if show_language && !r.language.is_empty() {
        name = format!("{name} ({})", crate::results::language_label(&r.language));
    }
    let marker = if r.kind == "validator" { VALIDATOR_MARK } else { "" };
    format!("[{}](results/{}/README.md){marker}", escape_cell(&name), r.version)
}

/// One row per version, with the pass rate of each of the Standard,
/// Optional and Compound tiers and the generation date. A tier with no
/// groups leaves its cell blank; a row with none in any of the three is
/// left out. Empty if nothing qualifies.
fn scoreboard_headline(all: &[&Results], show_language: bool) -> String {
    let mut rows = String::new();
    for r in all {
        let cells: Vec<String> = [Tier::Standard, Tier::Optional, Tier::Compound]
            .into_iter()
            .map(|tier| {
                let groups = groups_in(r, tier);
                if groups.is_empty() {
                    String::new()
                } else {
                    pass_rate(count(&groups, Bucket::Pass), groups.len())
                }
            })
            .collect();
        if cells.iter().all(String::is_empty) {
            continue;
        }
        rows.push_str(&format!(
            "| {} | {} | {} |\n",
            row_name(r, show_language),
            cells.join(" | "),
            r.generated_at
        ));
    }
    if rows.is_empty() {
        return rows;
    }
    format!("| | Standard | Optional | Compound | Generated |\n|---|:---:|:---:|:---:|---|\n{rows}")
}

/// One row per version with Upcoming groups: the pass rate and one glyph
/// per Upcoming group. The glyphs run over the union of the shown rows'
/// Upcoming ids, files in alphabetical order and suite order within a file,
/// so rows line up column for column; a row without a group shows a middle
/// dot there. Empty if no row has any.
fn scoreboard_upcoming(sections: &[(Vec<&Results>, bool)]) -> String {
    let mut shown: Vec<(&Results, bool, Vec<&GroupResult>)> = Vec::new();
    let mut ids: Vec<(&str, &str)> = Vec::new();
    for (all, show_language) in sections {
        for r in all {
            let mut groups = groups_in(r, Tier::Upcoming);
            if groups.is_empty() {
                continue;
            }
            groups.sort_by(|a, b| a.file.cmp(&b.file));
            for g in &groups {
                let key = (g.file.as_str(), g.id.as_str());
                if !ids.contains(&key) {
                    ids.push(key);
                }
            }
            shown.push((r, *show_language, groups));
        }
    }
    ids.sort_by(|a, b| a.0.cmp(b.0));
    let mut rows = String::new();
    for (r, show_language, groups) in &shown {
        let chart: String = ids
            .iter()
            .map(|(_, id)| match groups.iter().find(|g| g.id == *id) {
                Some(g) => g.bucket.glyph(),
                None => "\u{b7}",
            })
            .collect();
        rows.push_str(&format!(
            "| {} | {} | {} |\n",
            row_name(r, *show_language),
            pass_rate(count(groups, Bucket::Pass), groups.len()),
            chart
        ));
    }
    if rows.is_empty() {
        return rows;
    }
    format!("| | Pass | Per group |\n|---|:---:|---|\n{rows}")
}

/// Scoreboard order: by tool, newest version first (`row_order`). Rows whose
/// versions tie differ only past the version, which for a git pin is the
/// build metadata (`0.0.6+<sha>`) semver says not to order by; the one first
/// measured last goes first. A record's `generated_at` survives reruns
/// through the cache, so a row's earliest one is when its pin was first run.
fn order_key(r: &Results) -> ((String, Vec<crate::results::VersionKey>), std::cmp::Reverse<String>) {
    let first = r.groups.iter().map(|g| g.generated_at.as_str()).min().unwrap_or("").to_string();
    (crate::results::row_order(&r.typify), std::cmp::Reverse(first))
}

/// Rows left off the scoreboard because a newer row from the same
/// generator directory and language has the same results on every shared
/// group (see `same_results`). `rows` must already be in `order_key` order,
/// newest first. Each entry is (superseded row, the newest identical row).
fn superseded<'a>(rows: &[&'a Results]) -> Vec<(&'a Results, &'a Results)> {
    let mut out = Vec::new();
    for (i, old) in rows.iter().enumerate() {
        if old.generator.is_empty() {
            continue;
        }
        let newer = rows[..i].iter().find(|r| {
            r.generator == old.generator
                && r.language == old.language
                && order_key(r) < order_key(old)
                && crate::results::same_results(r, old)
        });
        if let Some(by) = newer {
            out.push((*old, *by));
        }
    }
    out
}

/// `RESULTS.md`: the cross-version scoreboard at the repo root.
///
/// A "Rust" and an "Other languages" section each have one table, a row per
/// generator version linking to its page under `results/<version>/`, with
/// the pass rate of the Standard, Optional and Compound tiers. An
/// "Upcoming" section has one table over the rows of both, with the
/// Upcoming pass rate and per-group glyphs. Then "The tools" and "Notes".
pub fn render_scoreboard(all: &[Results]) -> String {
    let mut s = String::new();
    s.push_str("# JSON Schema codegen bakeoff\n\n");
    s.push_str("Each row links to that generator's page, which lists every failing group with its error.\n\n");

    let mut shown: Vec<&Results> = all.iter().filter(|r| !r.hidden).collect();
    shown.sort_by_key(|r| order_key(r));
    let superseded = superseded(&shown);
    shown.retain(|r| !superseded.iter().any(|(old, _)| std::ptr::eq(*old, *r)));
    let (rust, other): (Vec<&Results>, Vec<&Results>) = shown.iter().partition(|r| r.language == "rust");
    let sections = [(rust, false), (other, true)];
    let mut explained = false;
    for ((rows, show_language), title) in sections.iter().zip(["Rust", "Other languages"]) {
        let table = scoreboard_headline(rows, *show_language);
        if table.is_empty() {
            continue;
        }
        s.push_str(&format!("## {title}\n\n"));
        if !explained {
            s.push_str("Standard is the suite outside `optional/`, less the format cases tagged `format-assertion` and the `v1` cases; Optional is `optional/` plus the format cases the suite tags `format-assertion`; Compound is our own groups under `compound/`, which the validator rows must pass in full.\n\n");
            explained = true;
        }
        s.push_str(&table);
        s.push('\n');
    }
    let upcoming = scoreboard_upcoming(&sections);
    if !upcoming.is_empty() {
        s.push_str("## Upcoming\n\n");
        s.push_str("Cases the suite marks for features of the unreleased JSON Schema v1, run under dialect `v1`. They are not part of the tables above.\n\n");
        s.push_str(&upcoming);
        s.push('\n');
    }
    if shown.iter().any(|r| !r.description.is_empty()) {
        s.push_str("## The tools\n\n");
        // One entry per tool: a generator with several targets (quicktype)
        // gets one paragraph with a link to each target's page.
        let mut seen: Vec<&str> = Vec::new();
        for r in &shown {
            let tool = if r.generator.is_empty() { r.version.as_str() } else { r.generator.as_str() };
            if seen.contains(&tool) {
                continue;
            }
            seen.push(tool);
            let name = crate::results::display_name(&r.typify, &r.language);
            s.push_str(&format!("**{}**", escape_cell(&name)));
            if !r.homepage.is_empty() {
                s.push_str(&format!(" — [{}]({})", crate::results::homepage_label(&r.homepage), r.homepage));
            }
            s.push_str("  \n");
            s.push_str(&r.description);
            s.push_str("\n\n");
        }
    }
    let validators = shown.iter().any(|r| r.kind == "validator");
    if validators || !superseded.is_empty() {
        s.push_str("## Notes\n\n");
    }
    if !superseded.is_empty() {
        let names: Vec<String> = superseded
            .iter()
            .map(|(old, by)| {
                let old_name = crate::results::display_name(&old.typify, &old.language);
                let by_name = crate::results::display_name(&by.typify, &by.language);
                // Same tool, so name the newer one by its version alone.
                let tool = old_name.split_whitespace().next().unwrap_or("");
                let by_short = by_name
                    .strip_prefix(tool)
                    .and_then(|v| v.strip_prefix(' '))
                    .unwrap_or(&by_name);
                format!("{} (by {})", escape_cell(&old_name), escape_cell(by_short))
            })
            .collect();
        s.push_str(&format!(
            "Left off: same results as a newer version of the same tool on every group both ran: {}.\n\n",
            names.join(", ")
        ));
    }
    if validators {
        s.push_str("𝓥 A validator, not a code generator: it checks each instance against the schema at run time and generates no types. Every other row deserializes into generated types, so a validator marks the ceiling the suite allows on the Standard tier rather than a competing approach.\n\n");
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
            id: format!("{file}::{group_desc}"),
            draft: draft.to_string(),
            file: file.to_string(),
            tier: if draft == "v1" {
                Tier::Upcoming
            } else if file.starts_with("compound/") {
                Tier::Compound
            } else if optional {
                Tier::Optional
            } else {
                Tier::Standard
            },
            group: group_desc.to_string(),
            hash: String::new(),
            generated_at: String::new(),
            bucket,
            error: error.map(|e| e.to_string()),
            cases,
        }
    }

    /// Groups under two dialects, a couple of files each, one optional file,
    /// every bucket represented at least once.
    fn sample_results() -> Results {
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
        cells(line)
    }

    fn cells(line: &str) -> Vec<String> {
        line.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect()
    }

    /// The text of the section that starts with `heading`, up to the next
    /// heading of the same level.
    fn section<'a>(out: &'a str, heading: &str) -> &'a str {
        let start = out.find(&format!("{heading}\n")).unwrap_or_else(|| panic!("no {heading} in:\n{out}"));
        let level = heading.split(' ').next().unwrap();
        let rest = &out[start + heading.len()..];
        let end = rest.find(&format!("\n{level} ")).map_or(rest.len(), |e| e + 1);
        &rest[..end]
    }

    /// The first data row of the first table in `out`.
    fn first_data_row(out: &str) -> Vec<String> {
        let line = out.lines().filter(|l| l.starts_with('|')).nth(2).expect("a table row");
        cells(line)
    }

    #[test]
    fn chart_glyphs_follow_file_order() {
        let out = render(&sample_results());
        // Every bucket column, always (✅ 🙈 🚧 🤪 💀 🦀 ❌ 🤯), then Total,
        // then the chart — one glyph per group, in suite order.
        assert_eq!(row(&out, "allOf.json"), ["allOf.json", "1", "1", "0", "1", "0", "0", "0", "0", "3", "✅🙈🤪"]);
        assert_eq!(row(&out, "ref.json"), ["ref.json", "0", "0", "0", "0", "0", "0", "0", "1", "1", "🤯"]);
        assert_eq!(row(&out, "type.json"), ["type.json", "0", "0", "0", "0", "0", "1", "0", "0", "1", "🦀"]);
    }

    #[test]
    fn chart_table_headers_line_up_with_counts() {
        let out = render(&sample_results());
        // All eight bucket columns in Bucket::ALL (best-first) order,
        // zero columns included so tables compare side by side.
        let header = "| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |";
        let sep = "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|";
        assert!(out.contains(header), "missing header, got:\n{out}");
        assert!(out.contains(sep));
        // Every row of every table must have the same number of cells as
        // its header.
        let pipes = |line: &str| line.matches('|').count();
        let mut want = None;
        for line in out.lines() {
            if !line.starts_with('|') {
                want = None;
            } else if let Some(n) = want {
                assert_eq!(pipes(line), n, "misaligned row: {line}");
            } else {
                want = Some(pipes(line));
            }
        }
    }

    #[test]
    fn optional_chart_is_separate() {
        let out = render(&sample_results());
        let opt = section(&out, "## Optional");
        // The only optional group is a false_reject.
        assert_eq!(
            row(opt, "optional/format/date.json"),
            ["optional/format/date.json", "0", "0", "1", "0", "0", "0", "0", "0", "1", "🚧"]
        );
        assert!(!section(&out, "## Standard").contains("optional/format/date.json"));
    }

    #[test]
    fn headline_pass_rate() {
        let out = render(&sample_results());
        // Count columns ✅ 🙈 🚧 🤪 💀 🦀 ❌ 🤯, then Total and Pass rate.
        // Standard: 1 pass out of 7 groups, whatever the dialect.
        let std = section(&out, "## Standard");
        assert!(std.contains("| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |"));
        assert_eq!(first_data_row(std), ["1", "1", "0", "1", "1", "1", "1", "1", "7", "14.3%"]);
        assert!(out.contains("**1 of 7 standard groups pass (14.3%)**"));
        // Optional groups get their own section: 0 of 1.
        let opt = section(&out, "## Optional");
        assert_eq!(first_data_row(opt), ["0", "0", "1", "0", "0", "0", "0", "0", "1", "0.0%"]);
    }

    #[test]
    fn case_rate_excludes_optional_from_headline() {
        let out = render(&sample_results());
        // Standard cases: 5 (one match, two false_accept, one false_reject,
        // one panic); the optional date case must not be counted here.
        let std = section(section(&out, "## Standard"), "### Per-case match rate");
        assert_eq!(first_data_row(std), ["1", "5", "20.0%", "5", "20.0%"]);
        let opt = section(section(&out, "## Optional"), "### Per-case match rate");
        assert_eq!(first_data_row(opt), ["0", "1", "0.0%", "1", "0.0%"]);
    }

    #[test]
    fn failures_section_lists_cases_for_false_accept_reject_mixed() {
        let out = render(&sample_results());
        assert!(out.contains("#### 🙈 `false_accept` (1)"));
        assert!(out.contains("cases: extra property (accepted)"));
        assert!(out.contains("#### 🚧 `false_reject` (1)"));
        assert!(out.contains("cases: valid date (rejected)"));
        assert!(out.contains("#### 🤪 `mixed` (1)"));
        assert!(out.contains("cases: accepted bad (accepted), rejected good (rejected)"));
        // generation_panic doesn't get a case list, just the error.
        assert!(out.contains("#### 🤯 `generation_panic` (1)"));
        assert!(out.contains("unsupported '$ref'"));
        // runtime_panic names the case that blew up.
        assert!(out.contains("cases: weird data (panicked)"));
    }

    #[test]
    fn failures_label_each_group_with_its_dialect() {
        let mut r = sample_results();
        r.groups.push(group(
            "v1",
            "proposals/propertyDependencies/propertyDependencies.json",
            false,
            "single dependency",
            Bucket::GenerationError,
            Some("unknown dialect v1"),
            vec![],
        ));
        let out = render(&r);
        assert!(out.contains("- `allOf.json::allOf too loose (7)` — `accepted invalid data`"), "got:\n{out}");
        assert!(out.contains("- `allOf.json::allOf mixed (2019-09)` — `mismatch`"));
        let upcoming = section(&out, "## Upcoming");
        assert!(upcoming.contains(
            "- `proposals/propertyDependencies/propertyDependencies.json::single dependency (v1)` — `unknown dialect v1`"
        ));
        assert_eq!(dialect_label("draft2020-12"), "2020-12");
        assert_eq!(dialect_label("draft4"), "4");
        assert_eq!(dialect_label("v1"), "v1");
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
        assert!(out.contains("#### ❌ `generation_error` (1)"));
        let gen_err = out.split("#### ❌ `generation_error` (1)\n\n").nth(1).unwrap();
        assert!(gen_err.starts_with("- `enum.json::enum generation error (2019-09)`"), "got:\n{gen_err}");
    }

    #[test]
    fn worst_first_ordering_in_failures() {
        let out = render(&sample_results());
        let std = section(&out, "## Standard");
        let panic_pos = std.find("#### 🤯").unwrap();
        let error_pos = std.find("#### ❌").unwrap();
        let failures = std.find("### Failures").unwrap();
        assert!(failures < panic_pos);
        assert!(panic_pos < error_pos);
    }

    #[test]
    fn all_pass_results_have_no_failures_and_no_division_by_zero() {
        let results = Results {
            description: String::new(),
            language: String::new(),
            kind: String::new(),
            hidden: false,
            generator: String::new(),
            homepage: String::new(),
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
        assert!(out.contains("**1 of 1 standard groups pass (100.0%)**"));
        assert!(out.contains("_none: every group passed._"));
        // No optional section when there are no optional groups.
        assert!(!out.contains("## Optional"));
    }

    #[test]
    fn empty_results_render_without_panicking() {
        let results = Results {
            description: String::new(),
            language: String::new(),
            kind: String::new(),
            hidden: false,
            generator: String::new(),
            homepage: String::new(),
            version: "typify1".to_string(),
            typify: "t".to_string(),
            generated_at: "now".to_string(),
            groups: vec![],
        };
        let out = render(&results);
        assert!(out.contains("**0 of 0 standard groups pass (-)**"));
        assert!(out.contains("_no groups_"));
        assert!(render_terminal(&results).contains("generation_panic"));
    }

    #[test]
    fn tiers_render_in_order() {
        let mut r = with_compound();
        r.groups.insert(
            0,
            group("v1", "proposals/p/p.json", false, "upcoming", Bucket::Pass, None, vec![]),
        );
        let out = render(&r);
        let pos = |h: &str| out.find(&format!("\n{h}\n")).unwrap_or_else(|| panic!("no {h}"));
        assert!(pos("## Standard") < pos("## Optional"));
        assert!(pos("## Optional") < pos("## Compound"));
        assert!(pos("## Compound") < pos("## Upcoming"));
        // No dialect headings anywhere.
        assert!(!out.contains("draft7\n"));
        let term = render_terminal(&r);
        let tpos = |h: &str| term.find(&format!("\n{h}:\n")).unwrap_or_else(|| panic!("no {h}"));
        assert!(tpos("standard") < tpos("optional"));
        assert!(tpos("optional") < tpos("compound"));
        assert!(tpos("compound") < tpos("upcoming"));
    }

    #[test]
    fn render_terminal_has_key_and_glyphs() {
        let out = render_terminal(&sample_results());
        assert!(out.contains("generation_panic"));
        assert!(out.contains("standard:"));
        assert!(!out.contains("draft7"));
        assert!(out.contains("allOf.json"));
        assert!(out.contains("✅🙈🤪"));
        assert!(out.contains("optional:"));
        assert!(out.contains("totals:"));
        assert!(out.contains("1/3"));
        assert!(!out.contains("compound:"));
        assert!(!out.contains("upcoming:"));
    }

    fn with_compound() -> Results {
        let mut r = sample_results();
        r.groups.push(group(
            "draft7",
            "compound/union-one-required-key.json",
            false,
            "union inline",
            Bucket::GenerationPanic,
            Some("panicked"),
            vec![],
        ));
        r.groups.push(group(
            "draft2020-12",
            "compound/union-one-required-key.json",
            false,
            "union named",
            Bucket::Pass,
            None,
            vec![case("c", true, CaseOutcome::Match)],
        ));
        r
    }

    #[test]
    fn compound_page_sections_only_when_present() {
        let out = render(&sample_results());
        assert!(!out.contains("## Compound"));

        let out = render(&with_compound());
        // Standard numbers don't move.
        assert!(out.contains("**1 of 7 standard groups pass (14.3%)**"));
        let comp = section(&out, "## Compound");
        assert_eq!(first_data_row(comp), ["1", "0", "0", "0", "0", "0", "0", "1", "2", "50.0%"]);
        assert_eq!(
            row(comp, "compound/union-one-required-key.json"),
            ["compound/union-one-required-key.json", "1", "0", "0", "0", "0", "0", "0", "1", "2", "🤯✅"]
        );
    }

    /// Two rows from one generator directory: `typify 0.10.0-alpha.2`
    /// (sample_results) and a newer `typify 0.10.0-alpha.3` with the same
    /// outcomes but different error text, timestamps, and an extra group.
    fn bump_pair() -> (Results, Results) {
        let mut old = sample_results();
        old.generator = "typify-0.10".to_string();
        old.language = "rust".to_string();
        let mut new = old.clone();
        new.typify = "typify 0.10.0-alpha.3".to_string();
        new.version = "typify-0.10.0-alpha.3".to_string();
        new.generated_at = "2026-11-01T00:00:00Z".to_string();
        for g in &mut new.groups {
            g.error = g.error.as_ref().map(|e| format!("{e} (reworded)"));
            g.generated_at = new.generated_at.clone();
        }
        new.groups.push(group(
            "draft7",
            "compound/new.json",
            false,
            "only in the new row",
            Bucket::Pass,
            None,
            vec![case("c", true, CaseOutcome::Match)],
        ));
        (old, new)
    }

    fn table_has(out: &str, name: &str) -> bool {
        out.lines().any(|l| l.starts_with(&format!("| [{name}]")))
    }

    #[test]
    fn row_order_puts_newest_prerelease_first() {
        use crate::results::row_order;
        let mut ids = vec![
            "typify 0.3.0",
            "typify 0.10.0-alpha.2",
            "typify 0.8.0",
            "typify 0.10.0-alpha.10",
            "typify 0.10.0",
            "typify 0.10.0-alpha.3",
        ];
        ids.sort_by_key(|i| row_order(i));
        assert_eq!(
            ids,
            [
                "typify 0.10.0",
                "typify 0.10.0-alpha.10",
                "typify 0.10.0-alpha.3",
                "typify 0.10.0-alpha.2",
                "typify 0.8.0",
                "typify 0.3.0",
            ]
        );
        // Build metadata doesn't break the numeric part.
        assert!(row_order("json-schema-rs 0.0.7+abc") < row_order("json-schema-rs 0.0.6+def"));
        assert!(
            row_order("json-schema-to-zod 2.8.1 + zod 4.6.7") < row_order("json-schema-to-zod 2.8.1 + zod 4.6.6")
        );
        assert_eq!(row_order("quicktype 26.0.0 rust"), row_order("quicktype 26.0.0 go"));
    }

    /// Two git pins of one version differ only in build metadata, which
    /// `row_order` ignores; the pin measured later comes first even when its
    /// sha sorts after the other's.
    #[test]
    fn git_pins_order_by_first_measurement() {
        let mut old = sample_results();
        old.typify = "json-schema-rs 0.0.6+9999999".to_string();
        old.version = "json-schema-rs-0.0.6-9999999".to_string();
        for g in &mut old.groups {
            g.generated_at = "2026-11-03T00:00:00Z".to_string();
        }
        let mut new = old.clone();
        new.typify = "json-schema-rs 0.0.6+1111111".to_string();
        new.version = "json-schema-rs-0.0.6-1111111".to_string();
        new.groups[0].bucket = Bucket::FalseReject;
        new.groups[0].cases[0].outcome = CaseOutcome::FalseReject;
        for g in &mut new.groups {
            g.generated_at = "2026-11-16T00:00:00Z".to_string();
        }
        // Group order doesn't matter: the earliest record decides, and a
        // rerun that refreshed some records leaves it alone.
        new.groups[1].generated_at = "2026-11-20T00:00:00Z".to_string();
        old.groups[1].generated_at = "2026-11-21T00:00:00Z".to_string();
        let out = render_scoreboard(&[old, new]);
        assert!(table_has(&out, "json-schema-rs 0.0.6+1111111") && table_has(&out, "json-schema-rs 0.0.6+9999999"));
        assert!(out.find("0.0.6+1111111").unwrap() < out.find("0.0.6+9999999").unwrap(), "{out}");
    }

    #[test]
    fn identical_newer_row_supersedes_older() {
        let (old, new) = bump_pair();
        let out = render_scoreboard(&[old, new]);
        assert!(table_has(&out, "typify 0.10.0-alpha.3"));
        assert!(!table_has(&out, "typify 0.10.0-alpha.2"));
        assert!(out.contains(
            "## Notes\n\nLeft off: same results as a newer version of the same tool on every group both ran: typify 0.10.0-alpha.2 (by 0.10.0-alpha.3).\n"
        ));
    }

    #[test]
    fn different_bucket_on_shared_group_keeps_both() {
        let (old, mut new) = bump_pair();
        new.groups[0].bucket = Bucket::FalseReject;
        new.groups[0].cases[0].outcome = CaseOutcome::FalseReject;
        let out = render_scoreboard(&[old, new]);
        assert!(table_has(&out, "typify 0.10.0-alpha.3"));
        assert!(table_has(&out, "typify 0.10.0-alpha.2"));
        assert!(!out.contains("Left off"));
        assert!(!out.contains("## Notes"));
    }

    #[test]
    fn different_case_outcome_is_not_identical() {
        let (old, mut new) = bump_pair();
        assert!(crate::results::same_results(&old, &new));
        // Same bucket and the same cases, but two outcomes traded places.
        let g = &mut new.groups[5];
        let (a, b) = (g.cases[0].outcome, g.cases[1].outcome);
        assert_ne!(a, b);
        g.cases[0].outcome = b;
        g.cases[1].outcome = a;
        assert!(!crate::results::same_results(&old, &new));
    }

    #[test]
    fn different_generator_keeps_both() {
        let (mut old, new) = bump_pair();
        old.generator = "typify-0.8".to_string();
        let out = render_scoreboard(&[old, new]);
        assert!(table_has(&out, "typify 0.10.0-alpha.2"));
        assert!(!out.contains("Left off"));
        // An empty generator never participates either.
        let (mut old, mut new) = bump_pair();
        old.generator.clear();
        new.generator.clear();
        assert!(table_has(&render_scoreboard(&[old, new]), "typify 0.10.0-alpha.2"));
    }

    #[test]
    fn group_whose_cases_changed_is_ignored() {
        // A suite bump added a case to a group the retired row never reran
        // with; that group is not the same test, so it doesn't break the
        // match.
        let (old, mut new) = bump_pair();
        new.groups[0].cases.push(case("added by a suite bump", false, CaseOutcome::FalseAccept));
        new.groups[0].bucket = Bucket::FalseAccept;
        let out = render_scoreboard(&[old, new]);
        assert!(!table_has(&out, "typify 0.10.0-alpha.2"));
        assert!(out.contains("Left off"));
    }

    #[test]
    fn no_shared_groups_keeps_both() {
        let (old, mut new) = bump_pair();
        for g in &mut new.groups {
            g.id = format!("{} renamed", g.id);
        }
        let out = render_scoreboard(&[old, new]);
        assert!(table_has(&out, "typify 0.10.0-alpha.2"));
        assert!(!out.contains("Left off"));
    }

    /// The scoreboard line for the row named `name` in `table`.
    fn line_for<'a>(table: &'a str, name: &str) -> &'a str {
        table
            .lines()
            .find(|l| l.starts_with(&format!("| [{name}]")))
            .unwrap_or_else(|| panic!("no row for {name} in:\n{table}"))
    }

    #[test]
    fn scoreboard_compound_cell_blank_without_compound_groups() {
        let mut other = with_compound();
        other.typify = "zzz 1.0.0".to_string();
        other.version = "zzz-1.0.0".to_string();
        let out = render_scoreboard(&[sample_results(), other]);
        assert!(out.contains("| | Standard | Optional | Compound | Generated |\n|---|:---:|:---:|:---:|---|\n"));
        let rows = section(&out, "## Other languages");
        assert_eq!(
            cells(line_for(rows, "typify 0.10.0-alpha.2")),
            ["[typify 0.10.0-alpha.2](results/typify1/README.md)", "14.3%", "0.0%", "", "2026-10-10T00:00:00Z"]
        );
        assert_eq!(
            cells(line_for(rows, "zzz 1.0.0")),
            ["[zzz 1.0.0](results/zzz-1.0.0/README.md)", "14.3%", "0.0%", "50.0%", "2026-10-10T00:00:00Z"]
        );
        // No dialect columns and no Upcoming section without v1 groups.
        assert!(!out.contains("2020-12"));
        assert!(!out.contains("## Upcoming"));
    }

    #[test]
    fn scoreboard_upcoming_lists_rows_with_v1_groups() {
        let mut rust = sample_results();
        rust.typify = "zzz 1.0.0".to_string();
        rust.version = "zzz-1.0.0".to_string();
        rust.language = "rust".to_string();
        rust.groups.push(group("v1", "proposals/p/a.json", false, "one", Bucket::GenerationError, None, vec![]));
        rust.groups.push(group("v1", "proposals/p/b.json", false, "two", Bucket::Pass, None, vec![]));
        let mut ts = sample_results();
        ts.typify = "bbb 2.0.0".to_string();
        ts.version = "bbb-2.0.0".to_string();
        ts.language = "typescript".to_string();
        ts.groups.push(group("v1", "proposals/p/a.json", false, "one", Bucket::Pass, None, vec![]));
        let without = sample_results();
        let out = render_scoreboard(&[without, ts, rust]);
        let up = section(&out, "## Upcoming");
        assert!(up.contains("dialect `v1`"));
        assert!(up.contains("| | Pass | Per group |\n|---|:---:|---|\n"));
        assert_eq!(cells(line_for(up, "zzz 1.0.0")), ["[zzz 1.0.0](results/zzz-1.0.0/README.md)", "50.0%", "❌✅"]);
        assert_eq!(cells(line_for(up, "bbb 2.0.0 (ts)")), ["[bbb 2.0.0 (ts)](results/bbb-2.0.0/README.md)", "100.0%", "✅\u{b7}"]);
        // Rust rows first, as in the sections above.
        assert!(up.find("zzz 1.0.0").unwrap() < up.find("bbb 2.0.0").unwrap());
        assert!(!up.contains("typify"));
        // Upcoming groups never reach the headline tables.
        let headline = section(&out, "## Rust");
        assert_eq!(cells(line_for(headline, "zzz 1.0.0"))[1], "14.3%");
        assert!(out.find("## Other languages").unwrap() < out.find("## Upcoming").unwrap());
    }
}
