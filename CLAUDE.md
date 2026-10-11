# JSON Schema codegen bakeoff

A scoreboard for JSON Schema code generators (and two validators as
reference rows), scored against the official JSON Schema Test Suite. The
repo started as a typify-only conformance harness; it is now a place to keep
evaluation glue for any generator, in any language, without re-running
stored results when a new tool is added. Adam reads `RESULTS.md`.

Don't change any tool's source here; the point is to measure tools as
shipped, with their default behavior. Don't make a row look better by
enabling stricter modes or stripping output a tool really produces.

## Two actions

- `cargo run -- run [--generator <name>] [--target <t>] [--draft <d>|all] [--filter <s>] [--no-cache]`
  executes the suite and stores `results/<tool version>/results.json`, then
  renders. A filtered run prints per-case detail and writes under `target/`.
- `cargo run -- render` regenerates every `results/*/README.md` and
  `RESULTS.md` from stored data. Nothing else writes to `results/`.

`cargo run -- diff A B` compares two results files; `cargo run -- key`
prints the bucket glyphs; `cargo run -- generators` lists what's installed.

## Layout and contract

`DESIGN.md` is the contract: the generator directory shape
(`generators/<name>/generator.toml` with `version`, `generate`, `build`
commands run as subprocesses), the runner protocol, the buckets, the record
cache (hash of harness revision, tool identity, generator sources, group,
schema, cases), and the policies (remote refs left to fail for generators,
resolved from `suite/remotes/` only by validator rows; `format` at library
defaults (asserting it fails required 2020-12 tests); dialect passed as
`draft`).

`harness/` orchestrates. `rust-batch/` is the shared build step for Rust
output. Everything scratch is under `target/`; Adam deletes it freely.

To add a tool, use the `add-generator` skill (`.claude/skills/add-generator/`):
it is the checklist plus every trap found so far. Every generator so far had
at least one real bug found by a second agent reviewing it against that
checklist; keep doing that.

## Conventions that are settled

- Buckets, worst to best: 🤯 generation_panic, ❌ generation_error,
  🦀 compile_fail, 💀 runtime_panic, 🤪 mixed, 🚧 false_reject,
  🙈 false_accept, ✅ pass. A tool that succeeds but emits no root type is
  `compile_fail`, not `generation_error`.
- `RESULTS.md`: Rust section then Other languages, each with Core and
  Optional tables; rows are "name (lang)" outside Rust; validator rows carry
  𝓥 linking to a Notes footnote; "The tools" has one paragraph per tool with
  a homepage link; no bucket key and no regeneration instructions there
  (the key is on each per-tool page; the README has the commands). Rows sort
  by tool name, then newest version first. Manifest `hidden = true` keeps a
  row off the tables.
- Per-tool pages show all eight bucket columns in every table; the
  per-group glyph column is "Per group".
- Descriptions in `generator.toml` are short and in Adam's voice (the
  `writing-like-ahl` skill); manifest metadata doesn't feed the record hash.
- Results are keyed by the exact tool version; bumping a pin starts a new
  directory and old results stay. A results directory may outlive its
  generator (typify 0.3.0).
- `.github/`: Dependabot bumps generator pins; `results.yml` reruns the
  changed generator on the PR and commits the results; `ci.yml` checks that
  stored results are current. Needs a `RESULTS_TOKEN` secret on GitHub.

## Working style

Delegate builds and reviews to agents and pick the model per task: opus for
anything with protocol or correctness subtlety, sonnet for mechanical or
well-specified work. Commit as you go. Keep the full-suite run fast (today
about ten seconds per generator cached, a few minutes cold).
