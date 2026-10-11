---
name: add-generator
description: Onboard a new code generator or validator into the JSON Schema bakeoff — create generators/<name>/ with a generator.toml and the version/generate/build commands, wire it through rust-batch or a language runner, run it on every draft, verify idempotence, write its description, and commit. Use when asked to add, evaluate, or onboard a tool (typify, quicktype, zod, pydantic, ajv, ...) to the bakeoff.
---

# Add a generator to the bakeoff

Read `DESIGN.md` first; it is the contract. This skill is the checklist and
the list of traps we have already fallen into. Do not change the harness
to suit one tool; the contract is process-based precisely so that a tool in
any language fits without touching Rust.

## 1. Decide the shape

- **Directory name** is per *integration*, named for the API line:
  `typify-0.10`, `schemafy-0.6`, `quicktype` (targets inside). The exact
  version comes from the tool at run time and names `results/<id>/`.
- **Targets**: one directory with `targets = [...]` when one pinned tool
  emits several languages (quicktype). Otherwise no `targets`.
- **Validator or generator?** A validator (jsonschema, ajv) fits the same
  contract: its "module" embeds the schema and `__conformance_check`
  validates. Validator rows calibrate the harness and may resolve
  `localhost:1234` remotes from `suite/remotes/` (never the network).
  Generator rows leave remote refs to fail; record what the tool does.
- **Measure the tool's default behavior.** Don't enable stricter modes
  (Go's `DisallowUnknownFields`, a validator's format assertion) to make
  the row look better; don't strip the tool's output to hide a compile
  error it really produces. Document every option you set and why.

## 2. Pick the pattern to copy

| tool language | copy | build step |
|---|---|---|
| Rust crate | `generators/typify-0.10/` (crate outside the workspace, own `Cargo.lock`, `build.rs` reads the pin, `generate.rs` = catch_unwind + big stack + timeout + crash guard) | `rust-batch` via the same `build` command; extra crates go in `<out_dir>/batch-deps.toml` written by generate |
| Node package | `generators/quicktype/` (`deps.mjs`: `npm ci` on first use, stdout muzzle, `writeAll`; `generate.mjs`; `build.mjs`) | your own: tsc or plain JS, probe-load each module to attribute failures, node runner |
| Go tool | `generators/quicktype/build.mjs` go target, or `generators/go-jsonschema/` | per-key package dirs, `main.go` runner, `go build`, attribute `<key>/<key>.go:L:C:` |

Copy helpers into your directory; generators do not import each other.

## 3. Write `generator.toml`

```toml
name = "<dir name>"
description = """
What the tool is and its apparent design center, in a paragraph. This is
rendered on RESULTS.md and the tool's page; say what it optimizes for and
what it therefore doesn't do, not just features.
"""
targets = ["rust", "go"]   # only if applicable

[commands]
version  = "..."   # prints the identity string, e.g. "quicktype 26.0.0 go"
generate = "... {out_dir}"
build    = "... {gen_dir} {batch_dir}"
```

Commands run via `sh -c` with cwd = the generator dir and
`CARGO_TARGET_DIR=<repo>/target`, `BAKEOFF_ROOT`, `BAKEOFF_TARGET`,
`BAKEOFF_DRAFT` set. Read the version from the pin (Cargo.lock,
node_modules/<pkg>/package.json, go.mod), never hardcode it twice.

## 4. Implement `generate`

- stdin: JSON array of `{key, draft, schema}`; one long-lived process, never
  one spawn per group (1,700 groups × 0.5 s is 15 minutes).
- Root type named `Root` where the tool lets you choose; otherwise detect
  what it emitted and target that in the check function.
- Write `<out_dir>/<key>.<ext>` containing the tool's output plus the check
  entry point (Rust: `pub fn __conformance_check(v: ::serde_json::Value) ->
  Result<(), String>`; Go: `func ConformanceCheck(data []byte) error`; JS:
  `export function __conformanceCheck(json)`). Keep the tool's output
  verbatim except for what is syntactically invalid inside a module
  (`extern crate`, package clauses); say what you stripped in the README.
- Status per key: `ok`, `error` (the tool returned an error, including "I
  can't represent this"), `panic` (crash, internal assertion, or timeout).
  Isolate each group: catch_unwind / try-catch / recover, a timeout, and
  the crash guard (`rust-batch/src/crashguard.inc.rs`) in Rust so a
  SIGABRT doesn't raise the macOS crash dialog.
- `draft`: use it if the tool has a dialect setting (validators do);
  otherwise ignore it and say so.
- **stdout carries only the status array.** Muzzle anything the tool
  prints (quicktype's `deps.mjs` shows how). Progress goes to stderr.
- **Numbers**: the harness sends the suite's digits exactly. Parse with
  correctly rounded floats (serde_json `float_roundtrip`); if you embed
  schema text, embed the bytes you received, don't re-serialize.

## 5. Implement `build`

- stdin: JSON array of keys. Compile them together; attribute compiler
  diagnostics to keys by file name; drop failures; repeat until clean (cap
  ~10 passes); build a runner over survivors.
- stdout: `{"outcomes": {key: "ok" | {"fail": "<diagnostic>"}}, "runner":
  [exe, args...]}`. Nothing else on stdout.
- An error you cannot attribute to a module (toolchain missing, runner
  source broken) is a hard failure (non-zero exit), never N compile_fails.
- **Write stdout with a loop that handles short writes.** A single
  `write(2)` to a non-blocking pipe stops at 64 KiB and returns the short
  count silently; build results with many diagnostics exceed that.
- Rust: delegate to `rust-batch` (unique runner bin name per batch dir,
  `strip = "none"` profile, `batch-deps.toml` honored). Go: pin a
  conservative `go` directive (`1.21`) so a machine one release behind
  doesn't try to fetch a toolchain. Node: run the runner with
  `process.execPath`, not `"node"` from PATH.

## 6. The runner protocol

`runner <cases.json> <start>`: print `{"ready": <start>}` after parsing;
then for each case from `start`: `{"case": {"i": <absolute index>, "r":
"accept" | {"reject": msg} | {"panic": msg}}}`, flushed per line. Unknown
key → panic outcome. Each check isolated (a panic in one case must not
kill the rest); if the process dies anyway the harness blames the case at
the cursor and restarts from the next. Keep the case data's number text:
Go passes `json.RawMessage`; JS uses a `JSON.parse` reviver with
`context.source` to re-serialize; Rust parses with `float_roundtrip`.

## 7. Run it

```sh
cargo run -- generators                                   # it's listed
cargo run -- run --generator <name> --draft draft7 --filter 'type.json::integer type'   # per-case detail + generated file path
cargo run -- run --generator <name> --draft draft7
cargo run -- run --generator <name>                       # all drafts
cargo run -- run --generator <name>                       # again: must be "cache: N hit, 0 to run"
git status --short results/<id>/                          # must be clean after the rerun
```

Then look at the generated code for two passing and two failing groups
and make sure the outcome reflects the tool, not the glue. Expect to find
a harness bug or two the first time a new language goes through; the
jsonschema and ajv rows are the reference: if a validator row isn't 100%
on required groups, it's the harness or a documented policy.

## 8. Document and commit

- `generators/<name>/README.md`: version, invocation, options and why,
  root naming, what was stripped, batch deps, quirks found.
- Append the tool to the Generators section of the root README.md.
- Commit `generators/<name>/`, `results/<id>/`, `RESULTS.md`, README.md.
  Never commit `node_modules/` or anything under `target/`.
- If the tool exposed a harness limitation, fix the harness in its own
  commit, bump `HARNESS_REV` only when stored records must be recomputed,
  and rerun every generator.

## Traps we have hit

- `rust-objcopy` crash dialogs: a `debug = false` profile defaults to
  `strip = "debuginfo"`; set `strip = "none"`.
- Two batch crates with the same bin name in one target dir run each
  other's binary: bin names must be unique per batch directory.
- `serde_json` without `float_roundtrip` is off by one ULP on bignum
  inputs; without `arbitrary_precision` a 50-digit integer is already an
  f64 before the tool sees it (harness side is handled; be careful in glue).
- A validator's `default-features = false` can silently drop format
  support (idna) and read as "too loose".
- Remote-ref retrievers must refuse absolute paths and anything outside
  `suite/remotes/`; `Path::join` with an absolute path discards the base.
- A content-derived module key collides across suite files with identical
  schemas; the harness handles it, but your build must tolerate repeated
  keys (`outcomes` may be smaller than the input).
- Review before committing: have a second agent read the generator against
  this checklist and run it; every generator so far had at least one real
  bug found that way.
