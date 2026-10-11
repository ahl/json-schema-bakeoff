# quicktype

[quicktype](https://github.com/glideapps/quicktype) via its library,
[`quicktype-core`](https://www.npmjs.com/package/quicktype-core), pinned in
`package.json` with a `package-lock.json`. Three targets: `rust`, `go`,
`typescript`.

```sh
cargo run -- run --generator quicktype --target rust --draft draft7
cargo run -- run --generator quicktype --target go --draft draft7
cargo run -- run --generator quicktype --target typescript --draft draft7
```

## Files

| | |
|---|---|
| `generator.toml` | the three commands the harness runs |
| `deps.mjs` | `npm ci` if `node_modules` is missing; version lookup; stdout muzzle; the looping protocol write |
| `generate.mjs` | `version` and `generate`: one process, every group |
| `build.mjs` | `build` for all three targets, each with its own compiler loop |
| `package.json`, `package-lock.json` | the pin: `quicktype-core` and `typescript` |

`node_modules/` is gitignored. Any command installs it on first use, so a
fresh clone needs no setup step; `npm ci` output goes to stderr because
stdout is the harness protocol. The guard is a plain existence check with
no lock, which is fine because the harness runs targets one at a time — two
concurrent `npm ci` runs in this directory would race. If the harness ever
parallelizes flavors, the install wants to move out of the generate/build
commands.

The identity string is `quicktype <version> <target>`, read from the
installed package's own `package.json`, so bumping the pin moves the
results directory (`results/quicktype-26.0.0-go/`) by itself.

## Generation

Each group's schema becomes a `JSONSchemaInput` with a single source named
`Root`, which is what makes the top-level type `Root` in every language.
Nothing else is registered: a group's `definitions`/`$defs` are reachable
through the document itself.

References are resolved against a `JSONSchemaStore` whose `fetch` always
returns `undefined`. That is the no-network policy, and returning
`undefined` rather than throwing is deliberate — it is how quicktype
learns an address is unresolvable, and it comes back as a labelled
`SchemaFetchError` instead of an opaque exception. Every
`http://localhost:1234/...` remote is therefore a `generation_error`: 14
groups in draft 7, all of `refRemote.json` among them.

Exceptions are bucketed by what they are, not by where they came from:

- a `QuickTypeError` is quicktype saying "I can't represent this" →
  `generation_error` (`SchemaFalseNotSupported`, `IRNoEmptyUnions`,
  `SchemaFetchError`, ...);
- a `QuickTypeError` whose `messageName` is `InternalError` is what
  `assert()` and `panic()` raise from inside the type graph — a crash
  wearing an error's clothes → `generation_panic`;
- anything that isn't a `QuickTypeError` → `generation_panic`. In draft 7
  this is three `ref.json` groups with `urn:` base URIs, where `urijs`
  throws `URNs do not have any generally defined hierarchical components`
  straight out through quicktype;
- a 30s timeout → `generation_panic`. The timer is a `Promise.race`, so it
  catches a hang in quicktype's async plumbing but not a synchronous
  infinite loop; nothing in the suite has hit either so far.

A self-referential root (`{"type": "array", "items": {"$ref": "#"}}`)
raises `RangeError: Maximum call stack size exceeded`, which is also a
`generation_panic`. It is catchable in Node, so it doesn't take the process
down.

## Renderer options

Only four options are set; everything else is quicktype's default, because
the default is what the bakeoff is measuring.

| target | option | why |
|---|---|---|
| rust | `visibility: public` | the batch crate reaches `Root` from a sibling module |
| rust | `leading-comments: false` | drops the commented-out `extern crate` example block — noise, and a trap if a future version emits it uncommented |
| go | `package: <key>` | each group is its own Go package directory, named for its module key |
| go | `just-types: false` | keeps `UnmarshalRoot`, the entry point the runner calls |
| typescript | `just-types: false`, `runtime-typecheck: true` | both are already the default, and both are the whole point: with `just-types` on, TypeScript interfaces check nothing at runtime. These stay explicit so a default change can't quietly turn the target into a no-op |

Rust's `integer-type` stays `conservative` (so whole numbers keep the i64
range rather than being forced to i32) and `density` stays `normal`.

## What the check entry point is, per target

Appended to the rendered module rather than written into the runner, so a
module whose root type came out unusable fails to compile on its own and
the diagnostic is attributed to its own key.

- **rust**: `pub fn __conformance_check(v: serde_json::Value) -> Result<(), String>`,
  per the shared `rust-batch` contract.
- **go**: `func ConformanceCheck(data []byte) error`, wrapping
  `UnmarshalRoot`. `UnmarshalRoot(data []byte) (Root, error)` is emitted
  for a struct root *and* a primitive one (`type Root int64`), so one
  wrapper covers both.
- **typescript**: `export function __conformanceCheck(json: string): void`,
  calling `Convert.toRoot`, which throws on a mismatch.

## Build and run, per target

**rust** delegates to the repo's `rust-batch`, handing it `<batch_dir>`
as the crate directory and relaying its result object verbatim. rust-batch
derives the runner's binary name from that directory's absolute path, so
two generators sharing one `CARGO_TARGET_DIR` can't uplift over each
other's executable.

**go** builds a module at `<batch_dir>` (`module bakeoff`, `go 1.27`, no
external dependencies) with one package directory per group and a
`main.go` that imports the live ones; `go build -o runner-bin .` compiles
only what `main.go` imports. Diagnostics (`# bakeoff/g_abc` headers and
`g_abc/g_abc.go:12:3: ...` spans) are attributed by directory, the blamed
packages are dropped from `main.go`, and the loop runs again — the same
shape as rust-batch's converge loop, capped at ten passes.

`encoding/json` is left exactly as it comes: no `DisallowUnknownFields`.
Unknown fields are ignored and missing ones zero-valued, which is Go's
default behavior and part of what is being measured. Go has no exceptions
for ordinary failures, so a returned error is a `reject` and only a
recovered panic is a `panic`; a stack overflow is fatal and unrecoverable
in Go, and the harness notices the dead process on its own.

**typescript** compiles the modules plus a generated `runner.ts` with the
pinned `tsc` (`--pretty false`, `target: es2020`, `module: commonjs`,
`strict` off, `types: []`), attributing `g_abc.ts(12,3): error TSxxxx:`
diagnostics by file stem and dropping the failures from `runner.ts` the
same way. The tsconfig lists its files explicitly, so a dropped module
isn't compiled again on the next pass. The runner declares `require` and
`process` instead of importing them, which is why no `@types/node` is
needed.

Everything thrown by `__conformanceCheck` is a `reject`: quicktype's
`Convert` reports a type mismatch by throwing a plain `Error`, and nothing
distinguishes that from a genuine crash. Only a dead process is a panic,
and that one the harness sees for itself.

## quicktype quirks worth knowing

**Go output doesn't compile when the root type is `interface{}`.** This is
the big one: 147 of draft 7's 322 groups. quicktype emits

```go
type Root interface{}

func (r *Root) Marshal() ([]byte, error) { return json.Marshal(r) }
```

and Go rejects a method on a pointer-to-interface:
`invalid receiver type Root (pointer or interface type)`. Every schema that
constrains nothing about its root's *type* lands here — `maxLength.json`,
`minimum.json`, `pattern.json`, `format.json`, all of `if-then-else.json`,
`not.json`, `propertyNames.json`. It is quicktype's bug, not the harness's,
so it is recorded as `compile_fail` and not worked around; dropping the
three-line `Marshal` method from the Go output would turn those 147 groups
into whatever `json.Unmarshal` into `interface{}` does (almost all of them
`false_accept`), which is a different question than "does the generated
code compile".

**`Cannot read properties of null (reading 'literal')` is a quicktype bug
in its own error formatter, not a lost verdict.** 66 of the TypeScript
target's stored case errors read that way. quicktype's `prettyTypeName`
does `typeof typ === "object" && typ.literal !== undefined`, and
`typeof null` is `"object"`, so a `null` member of a union crashes it. It
is only ever reached from `invalidValue`, i.e. *after* `cast` has already
decided the value doesn't match — so the reject is real and the bucket is
right; all that's lost is a readable message. Don't read those rejects as
runtime crashes.

**TypeScript index signatures can conflict with declared properties.** Two
draft-7 groups (`additionalProperties with schema`,
`properties, patternProperties, additionalProperties interaction`) emit an
interface whose `[property: string]: boolean` index type contradicts the
properties above it: `error TS2411`.

**Rust output is module-safe as it stands.** It opens with
`use serde::{Serialize, Deserialize};` — valid inside a non-root module in
edition 2018+ — and otherwise uses `serde_json::Value`,
`std::collections::HashMap` and `Option`, all of which the batch crate
already provides. The `extern crate` lines quicktype knows about are inside
the leading-comment block, which `leading-comments: false` suppresses;
`generate.mjs` filters uncommented `extern crate` lines anyway, as
insurance. Nothing else is stripped, and draft 7 produces zero Rust
compile failures.

**`Root` is always `Root`.** A `title` on the schema does not rename the
top-level type; the source name wins in all three languages.

**Number precision.** The Rust runner gets `serde_json` with
`float_roundtrip` from rust-batch, and Go's `encoding/json` is correctly
rounded — the Go runner keeps each case's `data` as a `json.RawMessage` so
`json.Unmarshal` sees the digits `cases.json` holds. TypeScript is the
awkward one, because `Convert.toRoot` takes a JSON *string*: the runner
reads `cases.json` with a `JSON.parse` reviver that captures each number's
`context.source` and re-serializes from that text, so a number never makes
a round trip through a float64 inside the runner. That is what keeps
`9007199254740992.0` from arriving as `9007199254740992`.

The limit is upstream of all three: the harness writes `cases.json` from a
`serde_json::Value`, and without `arbitrary_precision` an integer too big
for `i64`/`u64` is already an `f64` by then —
`optional/bignum.json`'s `12345678910111213141516171819202122232425262728293031`
reaches every runner as `1.2345678910111214e+52`. No runner can recover
the digits the suite wrote; those four bignum groups are measuring the
harness, not the generator.

**Nothing validates but the type.** quicktype models types, not
constraints, so `maxLength`, `minimum`, `multipleOf`, `pattern`, `format`,
`uniqueItems`, `not`, `if`/`then`/`else` and friends are all absent from
the output. That is why `false_accept` dominates: 93 groups for rust, 88
for typescript. It is the finding, not a defect in the integration.
