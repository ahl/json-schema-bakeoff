# Design

A bakeoff of JSON Schema code generators: each generator turns every schema
group in the official JSON Schema Test Suite into a type, the generated
code is compiled, and each test case is deserialized into that type. The
outcome is compared with the suite's `valid` flag.

The suite's files live in `suite/validation/tests/`; each file holds cases
(here, groups) that name the dialects they apply to. Every group runs once,
under the newest dialect it allows (see Dialects and tiers).

## Two actions

- `run` executes the suite through generators and stores data under
  `results/<id>/results.json`, then renders.
- `render` regenerates every `results/<id>/README.md` and the top-level
  `RESULTS.md` from stored data. Nothing else writes to `results/`.

Stored results never need regenerating when a generator is added or the
pages change. A generator's results are only recomputed when the generator
version, the dialect, the tier, the schema, the cases, or the harness
revision changes.

## Layout

```
harness/            orchestrator: suite loader, buckets, cache, runner protocol, render, diff
rust-batch/         shared build step for generators that emit Rust
generators/<name>/  one directory per generator integration (see below)
suite/              json-schema-org/JSON-Schema-Test-Suite, as a submodule
compound/tests/     our own groups, in the suite's file format
results/<id>/       committed data (results.json) and its rendered page (README.md)
RESULTS.md          rendered scoreboard: one row per results/<id>
archive/suite-1.041/ results against the old per-draft suite; never regenerated
target/             everything scratch: cargo output, generated sources, batch crates
```

## The generator contract

A generator is a directory under `generators/` holding a `generator.toml`:

```toml
name = "quicktype"
targets = ["rust", "go", "typescript"]   # optional; omitted = one implicit target

[commands]
version  = "node generate.mjs version {target}"
generate = "node generate.mjs generate {target} {out_dir}"
build    = "sh build.sh {target} {gen_dir} {batch_dir}"
```

The harness never links a generator. It runs the three commands as
subprocesses through `sh -c`, with cwd set to the generator directory,
`CARGO_TARGET_DIR=<repo>/target`, `BAKEOFF_ROOT`, and `BAKEOFF_TARGET` in the
environment, and `{root}`, `{target}`, `{out_dir}`, `{gen_dir}`,
`{batch_dir}` substituted. stdout is the protocol; stderr is inherited so
build noise reaches the terminal.

1. **`version`** prints an identity string, e.g. `typify 0.10.0-alpha.2` or
   `quicktype 26.0.0 go`. Slugged, it names the results directory and the
   scoreboard row, and it is part of every cache hash.
2. **`generate <out_dir>`** reads a JSON array of `{key, draft, schema,
   externalSchemas?}` on stdin in one long-lived process (never one process per
   group), writes `<out_dir>/<key>.<ext>` for each schema it can handle, and
   prints a JSON array of `{key, status, message?}` with status `ok`, `error`
   (the tool returned an error) or `panic` (the tool crashed or hung). The root
   type must be named `Root` where the tool lets the caller choose. `draft` is
   the dialect the group runs under (`draft4`, `draft6`, `draft7`,
   `draft2019-09`, `draft2020-12`, or `v1`); a generator that can select a
   dialect should use it, and one that can't should ignore it. A generator that
   can't handle `v1` reports status `error` for that item and keeps going.
   `externalSchemas` is present only when the group has any: an object from
   retrieval URI to schema, as the suite wrote it. A tool with a facility for
   supplying additional documents is given them; one without ignores the field
   (see Remote references). One `generate` call covers every selected group,
   so the array mixes dialects; `build` is likewise called once.
3. **`build <gen_dir> <batch_dir>`** reads a JSON array of keys on stdin,
   compiles those modules together, attributes compiler diagnostics back
   to keys, drops the failures, repeats until clean, and prints
   `{"outcomes": {key: "ok" | {"fail": diagnostic}}, "runner": [exe, args]}`.
   A failure that can't be attributed to one module (a missing toolchain,
   a broken dependency install, a bug in the runner source) must be a
   non-zero exit with nothing on stdout, never N `compile_fail`s. Probe-style
   builds should prove the environment first (a canary module) and emit a
   ready signal before attributing anything.

The runner the build step names speaks the harness's runner protocol:

```
runner <cases.json> <start-index>
```

`cases.json` is a JSON array of `{key, data}`. The runner prints
`{"ready": <start>}` once it has parsed the file, then for each case from
`start` onward one line `{"case": {"i": <index>, "r": "accept" | {"reject": msg} | {"panic": msg}}}`,
flushing after each. Each deserialize must be isolated so one failure
can't take the rest down (`catch_unwind`, `try/catch`, `recover`). If the
process dies anyway, the harness records the case at the cursor as a panic
and restarts the runner from the next index.

Every generator should parse JSON with correctly rounded floats (serde_json
needs the `float_roundtrip` feature); the suite's bignum tests depend on it.
The harness itself keeps numbers as their original text (serde_json's
`arbitrary_precision`), so `cases.json` carries the suite's digits exactly
and what a runner does with a 50-digit integer is the tool's behavior, not
the harness's.

Protocol payloads are unbounded (a build result can hold hundreds of 4 KB
diagnostics). Write them with a loop that handles short writes; a single
`write` syscall to a pipe can stop at 64 KiB without an error.

### Generators that emit Rust

`rust-batch build <gen_dir> <batch_dir>` implements the build step for any
generator whose modules are Rust. Each `<gen_dir>/<key>.rs` must define
`pub fn __conformance_check(v: ::serde_json::Value) -> Result<(), String>`.
The batch crate depends on serde, serde_json, regress, chrono and uuid; a
generator whose output needs more writes `<gen_dir>/batch-deps.toml` from
its generate step (lines for the `[dependencies]` table), and rust-batch
appends it.

### Versioning

A generator directory is per *integration*, named for the API line it was
written against (`typify-0.10`), not the exact version. Bumping the pinned
version edits it in place; the old version's `results/<id>/` stays. Two
directories for the same tool exist only when the glue code differs
(`typify-0.8`, `typify2`).

A bump within one generator directory (and, for quicktype, one target
language) that leaves every shared group's bucket and case outcomes
unchanged adds nothing, so the older row is left off the scoreboard as
superseded and named in its Notes; its page stays. Only groups both rows
ran with the same cases count: groups one side lacks (compound groups
added after a version was retired) and groups whose cases a suite bump
changed are skipped, so the two rows' percentages can differ. Different
directories never supersede each other.

## Buckets

One per group, worst to best. `cargo run -- key` prints the glyphs.

| | bucket | meaning |
|---|---|---|
| 🤯 | generation_panic | the generator crashed or hung |
| ❌ | generation_error | the generator returned an error |
| 🦀 | compile_fail | generated code doesn't compile |
| 💀 | runtime_panic | a test case crashed during deserialize |
| 🤪 | mixed | both false_accept and false_reject cases in one group |
| 🚧 | false_reject | deserialize failed but `valid: true` (too strict) |
| 🙈 | false_accept | deserialize succeeded but `valid: false` (too loose) |
| ✅ | pass | every case matched |

`false_accept` and `false_reject` stay separate because they have different
causes and fixes. A tool that reports success but emits no root type (Go
tools that skip an untyped root, schemafy on a self-referential root) is
not a `generation_error`: the generator writes the module as emitted, and
the missing `Root` lands in `compile_fail` with the compiler's own
diagnostic. Two generators chose this independently; keep it consistent. The first error message or panic text is stored for every
non-pass group, plus per-case detail.

## Caching and idempotence

Each stored record carries a hash of the harness revision, the generator
identity, a hash of the generator directory's own sources (everything but
`*.md`, dotfiles, `node_modules/` and `target/`), the group id, the dialect, the
tier, the schema, the external schemas, and the selected cases. `run` reuses any
record whose hash matches and recomputes the rest; an unchanged rerun is a
byte-for-byte no-op, and an edit to a generator's glue code recomputes that
generator's records. A run writes `results.json` whole. `--no-cache` recomputes
everything selected. Bump `HARNESS_REV` in `harness/src/pipeline.rs` when the
pipeline's semantics change in a way that should invalidate stored records.

## Dialects and tiers

A group's `compatibility` says which dialects it applies to: a comma list
of `[op]N`, with op one of "" (N or newer), `<=` (N or older), `=` (only
N), and N one of 3, 4, 6, 7, 2019, 2020, 9999. Absent means every dialect.
The harness applies the suite README's matching rule exactly, early break
included.

Each group runs once, under the newest of draft4, draft6, draft7,
draft2019-09, draft2020-12 it allows. A group that allows none of those
but allows 9999 (an unreleased v1 feature) runs under `v1`. A group that
allows neither (draft 3 only) is skipped.

Four tiers, in the order they are reported:

- Standard: suite groups not in another tier. Only this tier feeds the
  headline number.
- Optional: groups from `optional/` or tagged `format-assertion`.
  `--no-optional` skips it.
- Compound: our own groups (below). `--no-compound` skips it.
- Upcoming: groups that run under `v1`, including everything under
  `proposals/`. `--no-upcoming` skips it. It is reported in its own
  section, never as a column in the main tables.

Optional, Compound and Upcoming never enter the headline number.

A group id is `<file>::<description>`, where `file` is relative to
`suite/validation/tests/` (`allOf.json`, `optional/bignum.json`,
`compound/externally-tagged-union.json`). The loader errors on a duplicate
id.

## Compound tier

The suite isolates one keyword per group and never composes a realistic
document, which is where generators fail: naming anonymous nested objects,
choosing a union representation. `compound/tests/<path>.json` holds
our own groups in the suite's exact file format (`{description, tests}`,
with `compatibility` per group and a `comment` per group that says what
it's probing). They load after the suite's files, by the same
compatibility rule, with `file` set to `compound/<path>`, so ids read
`compound/<path>::<group>` and can't collide with official ones. They run
by default and are reported in their own section. The validator rows are
the oracle: every compound group must pass on both, or the expected
verdicts are wrong. Groups are added sparingly, a file at a time, when a
generator turns up a failure worth pinning down.

## Remote references

A group's `externalSchemas` (the documents its `http://localhost:1234/...`
references need, keyed by retrieval URI) travel with the generate request.
Any tool with a facility for supplying additional documents is given them
through it: each entry registered under its key, scoped to the group, never
read from the network or the filesystem. The validator rows do this, and a
generator that accepts a document set should too; its README says how. A tool
with no such facility gets the schema alone, and the row records whatever it
does with the unresolvable reference (typify panics, schemafy panics looking
for a local definition). Validator rows leave `format` at the library default:
asserting it everywhere fails 19 required 2020-12 groups whose cases state
that an invalid string is only an annotation by default.

## Where number precision stops

The harness keeps numbers as the suite's text through `cases.json`. What
happens next is the tool's: `rust-batch`'s runner parses cases with
serde_json (correctly rounded, but an integer beyond `u64` becomes an
`f64`), Go's runner hands the raw bytes to the generated unmarshal, and the
TypeScript runner re-serializes from source text. So a bignum result
reflects the target language's JSON parser plus the tool, which is the
experience a user of that tool would have.
