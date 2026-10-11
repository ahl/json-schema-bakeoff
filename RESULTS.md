# JSON Schema codegen bakeoff

Each row links to that generator's page, which lists every failing group with its error.

| | bucket | meaning |
|---|---|---|
| ✅ | `pass` | every case matched |
| 🙈 | `false_accept` | deserialize succeeded but `valid: false` (too loose) |
| 🚧 | `false_reject` | deserialize failed but `valid: true` (too strict) |
| 🤪 | `mixed` | both false_accept and false_reject cases in one group |
| 💀 | `runtime_panic` | a test case panicked during deserialize |
| 🦀 | `compile_fail` | generated code doesn't compile |
| ❌ | `generation_error` | the generator returned an error |
| 🤯 | `generation_panic` | the generator crashed or hung |

## Rust

### Core

Pass rate per draft: groups where every test case matched, over all groups in the draft.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [json-schema-rs 0.0.6](results/json-schema-rs-0.0.6/README.md) | 1.2% | 0.9% | 0.8% | 1.3% | 1.3% | 2026-10-11T18:33:48Z |
| [jsonschema 0.58.6](results/jsonschema-0.58.6/README.md) [𝓥](#notes) | 100.0% | 100.0% | 100.0% | 100.0% | 100.0% | 2026-10-11T18:32:37Z |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 41.2% | 41.2% | 43.4% | 36.2% | 35.4% | 2026-10-11T18:31:54Z |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 27.5% | 23.2% | 25.6% | 21.7% | 20.8% | 2026-10-11T18:25:46Z |
| [typify 0.10.0-alpha.2](results/typify-0.10.0-alpha.2/README.md) | 38.1% | 34.3% | 31.0% | 27.6% | 27.9% | 2026-10-11T18:31:38Z |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 38.1% | 34.3% | 31.0% | 27.6% | 27.9% | 2026-10-11T18:31:22Z |

### Optional tests

`tests/<draft>/optional/` covers behavior the spec leaves optional (formats, bignum, regex dialects, ...). Reported here, never counted above.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [json-schema-rs 0.0.6](results/json-schema-rs-0.0.6/README.md) | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% | 2026-10-11T18:33:48Z |
| [jsonschema 0.58.6](results/jsonschema-0.58.6/README.md) [𝓥](#notes) | 100.0% | 100.0% | 100.0% | 68.4% | 69.2% | 2026-10-11T18:32:37Z |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 17.9% | 11.4% | 12.5% | 15.8% | 15.4% | 2026-10-11T18:31:54Z |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 17.9% | 11.4% | 12.5% | 11.8% | 11.5% | 2026-10-11T18:25:46Z |
| [typify 0.10.0-alpha.2](results/typify-0.10.0-alpha.2/README.md) | 56.4% | 45.5% | 31.2% | 27.6% | 26.9% | 2026-10-11T18:31:38Z |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 56.4% | 45.5% | 31.2% | 27.6% | 26.9% | 2026-10-11T18:31:22Z |

## Other languages

### Core

Pass rate per draft: groups where every test case matched, over all groups in the draft.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 97.5% | 98.3% | 98.4% | 96.8% | 90.6% | 2026-10-11T18:28:47Z |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 1.9% | 1.3% | 1.2% | 2.9% | 2.1% | 2026-10-11T18:30:38Z |
| [json-schema-to-zod 2.8.1 + zod 4.6.6 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.6/README.md) | 38.8% | 40.8% | 41.9% | 36.5% | 35.7% | 2026-10-11T18:35:39Z |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 25.0% | 24.9% | 24.0% | 18.8% | 18.8% | 2026-10-11T18:32:03Z |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 43.8% | 43.3% | 45.3% | 40.5% | 39.6% | 2026-10-11T18:32:18Z |

### Optional tests

`tests/<draft>/optional/` covers behavior the spec leaves optional (formats, bignum, regex dialects, ...). Reported here, never counted above.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 79.5% | 75.0% | 56.2% | 63.2% | 61.5% | 2026-10-11T18:28:47Z |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 23.1% | 18.2% | 12.5% | 11.8% | 10.3% | 2026-10-11T18:30:38Z |
| [json-schema-to-zod 2.8.1 + zod 4.6.6 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.6/README.md) | 46.2% | 40.9% | 32.8% | 30.3% | 29.5% | 2026-10-11T18:35:39Z |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 10.3% | 4.5% | 3.1% | 6.6% | 6.4% | 2026-10-11T18:32:03Z |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 17.9% | 15.9% | 15.6% | 18.4% | 17.9% | 2026-10-11T18:32:18Z |

## The tools

**[ajv 8.20.0](results/ajv-8.20.0/README.md)** — ajv is the JSON Schema validator for JavaScript, and for most of the
ecosystem it *is* JSON Schema: it compiles a schema into purpose-built
JavaScript source and runs that, which is why it is fast enough to sit in
request paths and why it covers draft-04 through 2020-12 keyword for
keyword. It produces no types — the schema stays the artifact, validation
happens at the boundary, and what flows past it is a plain parsed value —
so it answers the opposite question from a code generator: not "what shape
does this schema describe" but "does this document satisfy it". It is here
as the reference row for the Node path. It resolves the suite's
`http://localhost:1234/` remotes from `suite/remotes/`, which the code
generators are not allowed to do, so the difference between this row and
theirs on `refRemote.json` is the price of that policy. The one thing ajv
leaves at the door is `format`: the core package ships no format
implementations at all, and the `ajv-formats` package that does is
deliberately not installed here, so every `format` in the suite is an
annotation.

**[go-jsonschema v0.24.1](results/go-jsonschema-v0.24.1/README.md)** — go-jsonschema (github.com/omissis/go-jsonschema, module path
github.com/atombender/go-jsonschema) turns JSON Schema into Go structs plus
hand-rolled `UnmarshalJSON` methods that re-check the schema's validation
keywords as they decode. Its design center is a configuration file with a
known shape: a schema whose root declares `type: object` becomes a struct
whose fields carry `json`/`yaml`/`mapstructure` tags, and required fields,
enums, string lengths, patterns and numeric bounds are enforced in the
generated unmarshaller rather than left to the type system. The flip side is
that a schema whose root has no `type` keyword produces no root type at all,
and the combinators (`allOf`, `anyOf`, `oneOf`, `not`, `if`/`then`,
`const`) are reduced to `interface{}`.

**[json-schema-rs 0.0.6](results/json-schema-rs-0.0.6/README.md)** — A young Rust generator (0.0.x) aimed at hand-written config schemas: the root
must be an object with `properties` or it refuses outright. Constraints come
out as attributes for its own validator, which serde ignores, so it's
measured on the little it does model.

**[json-schema-to-zod 2.8.1 + zod 4.6.6](results/json-schema-to-zod-2.8.1-zod-4.6.6/README.md)** — json-schema-to-zod rewrites a JSON Schema into zod schema source: one zod
expression, as a string, for you to paste into a TypeScript project. Its
design center is developer convenience at the boundary between a published
schema and a zod-shaped codebase — get a usable validator and an inferred
TypeScript type out of a schema you didn't write. It is a *constraint*
translator rather than a type generator, so where quicktype and typify
model shape and drop the rest, this emits `.min()`, `.max()`, `.regex()`,
`.strict()` and `superRefine` closures, and keywords it has no parser for
(`$ref`, `multipleOf` outside a typed number, `propertyNames`,
`dependencies`) fall through to `z.any()`. Its own README says the output
is not meant to be used at runtime and recommends Ajv instead; we run it
anyway, because what it does translate it translates exactly. Deprecated
since March 2026.

**[jsonschema 0.58.6](results/jsonschema-0.58.6/README.md)** — A Rust validator, not a generator: it interprets the schema at run time, so
it can enforce every keyword, which a type generator structurally can't. It's
the reference row: the ceiling the suite allows, a check on the harness, and
a price tag on the policies the generators live with. Remote refs come from
`suite/remotes/`, the dialect from the suite directory, and `format` stays at
the library default (asserted through draft 7, annotation-only after).

**[quicktype 26.0.0 go](results/quicktype-26.0.0-go/README.md)** — Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

**[quicktype 26.0.0 rust](results/quicktype-26.0.0-rust/README.md)** — Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

**[quicktype 26.0.0 typescript](results/quicktype-26.0.0-typescript/README.md)** — Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

**[schemafy 0.6.0](results/schemafy-0.6.0/README.md)** — schemafy is the original Rust JSON Schema → serde generator (2017), usually
used through the `schemafy!` proc macro that drops types for a schema file
straight into a module. Its design center is a hand-written draft-4 schema
you own and keep working: walk `properties` and `definitions`, name a struct
or an alias for each, and otherwise get out of the way. It models only
structure — "no checking such as `min_value` is done" — so validation
keywords become comments at best, unknown keywords are dropped, and anything
that isn't a plain object graph (`oneOf` without `$ref`s, a boolean schema,
a `$ref` it can't find) panics rather than degrading. `schemafy_lib` is the
library behind the macro, and the generator here calls its `Expander`
directly.

**[typify 0.10.0-alpha.2](results/typify-0.10.0-alpha.2/README.md)** — Rust types for serde from JSON Schema, built to generate OpenAPI clients
(progenitor). Its design center is idiomatic Rust: structs, enums, newtypes,
`deny_unknown_fields` when the schema is closed. Validation is whatever the
type system can say; anything else is silently absent, and a construct it
can't model fails at generation rather than producing a wrong type. The 0.10
line is the in-progress rewrite on top of typespace.

**[typify 0.8.0](results/typify-0.8.0/README.md)** — The last typify release before the typespace rewrite. Same design center as
0.10; it's here so the rewrite has something to be measured against.

## Notes

𝓥 A validator, not a code generator: it checks each instance against the schema at run time and generates no types. Every other row deserializes into generated types, so a validator marks the ceiling the suite allows rather than a competing approach.

