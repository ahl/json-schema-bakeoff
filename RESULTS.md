# JSON Schema codegen bakeoff

Each row links to that generator's page, which lists every failing group with its error.

## Rust

### Core

Pass rate per draft: groups where every test case matched, over all groups in the draft.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [jsonschema 0.58.6](results/jsonschema-0.58.6/README.md) [𝓥](#notes) | 100.0% | 100.0% | 100.0% | 100.0% | 100.0% | 2026-10-11T18:52:11Z |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 41.2% | 41.2% | 43.4% | 36.2% | 35.4% | 2026-10-11T18:51:27Z |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 27.5% | 23.2% | 25.6% | 21.7% | 20.8% | 2026-10-11T18:53:33Z |
| [typify 0.10.0-alpha.2](results/typify-0.10.0-alpha.2/README.md) | 38.1% | 34.3% | 31.0% | 27.6% | 27.9% | 2026-10-11T18:51:12Z |
| [typify 0.3.0](results/typify-0.3.0/README.md) | 36.9% | 33.5% | — | — | — | 2026-10-11T19:26:33Z |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 38.1% | 34.3% | 31.0% | 27.6% | 27.9% | 2026-10-11T18:50:57Z |

### Optional tests

`tests/<draft>/optional/` covers behavior the spec leaves optional (formats, bignum, regex dialects, ...).

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [jsonschema 0.58.6](results/jsonschema-0.58.6/README.md) [𝓥](#notes) | 100.0% | 100.0% | 100.0% | 68.4% | 69.2% | 2026-10-11T18:52:11Z |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 17.9% | 11.4% | 12.5% | 15.8% | 15.4% | 2026-10-11T18:51:27Z |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 17.9% | 11.4% | 12.5% | 11.8% | 11.5% | 2026-10-11T18:53:33Z |
| [typify 0.10.0-alpha.2](results/typify-0.10.0-alpha.2/README.md) | 56.4% | 45.5% | 31.2% | 27.6% | 26.9% | 2026-10-11T18:51:12Z |
| [typify 0.3.0](results/typify-0.3.0/README.md) | 56.4% | 45.5% | — | — | — | 2026-10-11T19:26:33Z |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 56.4% | 45.5% | 31.2% | 27.6% | 26.9% | 2026-10-11T18:50:57Z |

## Other languages

### Core

Pass rate per draft: groups where every test case matched, over all groups in the draft.

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 97.5% | 98.3% | 98.4% | 96.8% | 90.6% | 2026-10-11T18:53:54Z |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 1.9% | 1.3% | 1.2% | 2.9% | 2.1% | 2026-10-11T18:50:16Z |
| [json-schema-to-zod 2.8.1 + zod 4.6.6 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.6/README.md) | 38.8% | 40.8% | 41.9% | 36.5% | 35.7% | 2026-10-11T18:53:39Z |
| [ogen v1.24.0 (go)](results/ogen-v1.24.0/README.md) | 21.2% | 17.6% | 19.8% | 20.4% | 18.8% | 2026-10-11T19:20:29Z |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 25.0% | 24.9% | 24.0% | 18.8% | 18.8% | 2026-10-11T18:51:36Z |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 43.8% | 43.3% | 45.3% | 40.5% | 39.6% | 2026-10-11T18:51:51Z |
| [typebox 0.34.52 + schema2typebox 1.7.8 (ts)](results/typebox-0.34.52-schema2typebox-1.7.8/README.md) | 23.1% | 19.7% | 17.8% | 13.7% | 12.2% | 2026-10-11T19:24:11Z |

### Optional tests

`tests/<draft>/optional/` covers behavior the spec leaves optional (formats, bignum, regex dialects, ...).

| | 4 | 6 | 7 | 2019-09 | 2020-12 | Generated |
|---|:---:|:---:|:---:|:---:|:---:|---|
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 79.5% | 75.0% | 56.2% | 63.2% | 61.5% | 2026-10-11T18:53:54Z |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 23.1% | 18.2% | 12.5% | 11.8% | 10.3% | 2026-10-11T18:50:16Z |
| [json-schema-to-zod 2.8.1 + zod 4.6.6 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.6/README.md) | 46.2% | 40.9% | 32.8% | 30.3% | 29.5% | 2026-10-11T18:53:39Z |
| [ogen v1.24.0 (go)](results/ogen-v1.24.0/README.md) | 56.4% | 34.1% | 28.1% | 25.0% | 24.4% | 2026-10-11T19:20:29Z |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 10.3% | 4.5% | 3.1% | 6.6% | 6.4% | 2026-10-11T18:51:36Z |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 17.9% | 15.9% | 15.6% | 18.4% | 17.9% | 2026-10-11T18:51:51Z |
| [typebox 0.34.52 + schema2typebox 1.7.8 (ts)](results/typebox-0.34.52-schema2typebox-1.7.8/README.md) | 35.9% | 31.8% | 21.9% | 18.4% | 17.9% | 2026-10-11T19:24:11Z |

## The tools

**ajv 8.20.0** — [ajv.js.org](https://ajv.js.org)  
The JSON Schema validator for JavaScript, and for much of the ecosystem the
reference implementation: it compiles a schema into purpose-built JS and runs
that at the boundary. No types come out; the schema stays the artifact. It's
the reference row for the Node path, resolving `localhost:1234` remotes from
`suite/remotes/` and shipping no formats, so `format` is ignored everywhere.

**go-jsonschema v0.24.1** — [GitHub](https://github.com/omissis/go-jsonschema)  
Go structs from JSON Schema, plus hand-written `UnmarshalJSON` methods that
re-check the schema's keywords while decoding. Its design center is a config
file with a known shape: a root that declares `type: object` becomes a tagged
struct, and `required`, `enum`, lengths, patterns and bounds are enforced in
the unmarshaller rather than left to the type system. The flip side is that a
root with no `type` keyword produces no root type at all, and the combinators
collapse into a merged type or `interface{}`.

**json-schema-to-zod 2.8.1 + zod 4.6.6** — [npm](https://www.npmjs.com/package/json-schema-to-zod)  
Turns a JSON Schema into a zod schema expression, so you get a static
TypeScript type and a runtime parser from one artifact. Its dispatcher keys
on `type`: a typed schema picks up bounds, lengths, patterns, enums and
`const`, while a schema without `type` becomes `z.any()` whatever else it
says. It has no `$ref` resolver. Unmaintained since March 2026; 2.8.1 is the
last release, and it runs on zod 4 here.

**jsonschema 0.58.6** — [crates.io](https://crates.io/crates/jsonschema)  
A Rust validator, not a generator: it interprets the schema at run time, so
it can enforce every keyword, which a type generator structurally can't. It's
the reference row: the ceiling the suite allows, a check on the harness, and
a price tag on the policies the generators live with. Remote refs come from
`suite/remotes/`, the dialect from the suite directory, and `format` stays at
the library default (asserted through draft 7, annotation-only after).

**ogen v1.24.0** — [ogen.dev](https://ogen.dev)  
An OpenAPI v3 generator for Go: it reads a whole API document and emits the
client, the server and — the part this row measures — a Go type per schema
with its JSON codec and its validator written out as code, against
`github.com/go-faster/jx` rather than `encoding/json` and reflection.
Decoding and validation are deliberately two methods: `Decode` enforces the
type and the required-field set, `Validate` enforces lengths, bounds,
patterns and `uniqueItems`. Because the input is an API document and not a
bare schema, each group here is wrapped as an OpenAPI 3.1 document whose
`components.schemas.Root` is the group's schema and whose one synthetic
operation takes it as a request body; 3.1 schema objects are JSON Schema
2020-12, so the schema itself goes in untranslated.

**quicktype 26.0.0** — [quicktype.io](https://quicktype.io)  
Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

**schemafy 0.6.0** — [crates.io](https://crates.io/crates/schemafy_lib)  
The original Rust JSON Schema → serde generator, usually used through the
`schemafy!` macro. Its design center is a hand-written draft-4 schema you own:
walk `properties` and `definitions`, name a struct or alias for each, and
get out of the way. It models structure only, so validation keywords vanish
and anything that isn't a plain object graph panics. The last release
(0.6.0) is from October 2021.

**typebox 0.34.52 + schema2typebox 1.7.8** — [GitHub](https://github.com/sinclairzx81/typebox)  
TypeBox is a TypeScript type builder: `Type.Object({ x: Type.Number() })`
returns a value that is at once a draft-7 Json Schema fragment and a static
TypeScript type, and `TypeCompiler`/`Value` check instances against it. Its
design center is the TypeScript end of the wire — one definition, written
in TypeScript, yielding both inference and validation — so its checkers
dispatch on a `Kind` symbol its own constructors attach and refuse a Json
Schema they didn't build (`Value.Check({ type: 'string' }, 'x')` throws
"Unknown type"; `Type.Unsafe` only relabels the problem). Coming the other
way needs a code generator, and `@sinclair/typebox-codegen` has no Json
Schema front end, so this row uses the one TypeBox's own readme points at:
schema2typebox emits the `Type.*` source, tsc compiles it, TypeBox's
compiler checks the cases. That measures both halves of the claim — types
generated from the schema, validation by the type — at the cost of
schema2typebox's narrow front end, which wants a recognizable `type` or
combinator at every level and refuses most of the suite outright.

**typify 0.10.0-alpha.2** — [crates.io](https://crates.io/crates/typify)  
Rust types for serde from JSON Schema, built to generate OpenAPI clients
(progenitor). Its design center is idiomatic Rust: structs, enums, newtypes,
`deny_unknown_fields` when the schema is closed. Validation is whatever the
type system can say; anything else is silently absent, and a construct it
can't model fails at generation rather than producing a wrong type. The 0.10
line is the in-progress rewrite on top of typespace.

**typify 0.3.0** — [crates.io](https://crates.io/crates/typify)  
typify 0.3.0, from December 2024: the version most typify users were on
before the 0.4 through 0.8 releases and well before the rewrite. Same design
center; it's here to show how much the last two years moved.

**typify 0.8.0** — [crates.io](https://crates.io/crates/typify)  
The last typify release before the typespace rewrite. Same design center as
0.10; it's here so the rewrite has something to be measured against.

## Notes

𝓥 A validator, not a code generator: it checks each instance against the schema at run time and generates no types. Every other row deserializes into generated types, so a validator marks the ceiling the suite allows rather than a competing approach.

