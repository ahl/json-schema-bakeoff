# JSON Schema codegen bakeoff

Each row links to that generator's page, which lists every failing group with its error.

## Rust

Standard is the suite outside `optional/`, less the format cases tagged `format-assertion` and the `v1` cases; Optional is `optional/` plus the format cases the suite tags `format-assertion`; Compound is our own groups under `compound/`, which the validator rows must pass in full.

| | Standard | Optional | Compound | Generated |
|---|:---:|:---:|:---:|---|
| [json-schema-rs 0.0.6+8032443](results/json-schema-rs-0.0.6-8032443/README.md) | 56.3% | 46.9% | 80.0% | 2026-11-22T03:39:29Z |
| [jsonschema 0.60.2](results/jsonschema-0.60.2/README.md) [𝓥](#notes) | 100.0% | 65.3% | 100.0% | 2026-11-22T01:59:39Z |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 40.0% | 14.3% | 0.0% | 2026-11-22T03:39:13Z |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 23.3% | 10.2% | 0.0% | 2026-11-22T02:41:27Z |
| [typify 0.10.2](results/typify-0.10.2/README.md) | 28.1% | 28.6% | 20.0% | 2026-11-25T20:23:27Z |
| [typify 0.10.1](results/typify-0.10.1/README.md) | 25.0% | 28.6% | 20.0% | 2026-11-22T01:57:31Z |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 24.6% | 24.5% | 20.0% | 2026-11-22T01:57:36Z |

## Other languages

| | Standard | Optional | Compound | Generated |
|---|:---:|:---:|:---:|---|
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 92.2% | 48.0% | 100.0% | 2026-11-22T02:30:50Z |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 2.6% | 10.2% | 0.0% | 2026-11-22T03:50:44Z |
| [json-schema-to-zod 2.8.1 + zod 4.6.7 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.7/README.md) | 32.0% | 22.4% | 80.0% | 2026-11-22T02:41:31Z |
| [ogen v1.24.0 (go)](results/ogen-v1.24.0/README.md) | 19.1% | 25.5% | 80.0% | 2026-11-22T03:50:51Z |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 43.0% | 14.3% | 40.0% | 2026-11-22T03:39:20Z |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 23.0% | 9.2% | 0.0% | 2026-11-22T03:39:16Z |
| [typebox 0.34.52 + schema2typebox 1.7.8 (ts)](results/typebox-0.34.52-schema2typebox-1.7.8/README.md) | 10.9% | 15.3% | 100.0% | 2026-11-22T02:41:30Z |

## Upcoming

Cases the suite marks for features of the unreleased JSON Schema v1, run under dialect `v1`. They are not part of the tables above.

| | Pass | Per group |
|---|:---:|---|
| [json-schema-rs 0.0.6+8032443](results/json-schema-rs-0.0.6-8032443/README.md) | 0.0% | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| [jsonschema 0.60.2](results/jsonschema-0.60.2/README.md) [𝓥](#notes) | 0.0% | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| [quicktype 26.0.0](results/quicktype-26.0.0-rust/README.md) | 12.0% | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌🙈🙈🙈✅✅🙈🙈🙈🙈🙈 |
| [schemafy 0.6.0](results/schemafy-0.6.0/README.md) | 8.0% | 🙈🤯🤯🤯🙈🤯🤯🤯🤯🤯🦀🤯🙈🙈🦀🙈🙈🙈✅✅🙈🙈🙈🤯🤯 |
| [typify 0.10.2](results/typify-0.10.2/README.md) | 12.0% | 🙈🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯✅🤯🙈✅✅🙈🙈🙈🤯🤯 |
| [typify 0.10.1](results/typify-0.10.1/README.md) | 12.0% | 🙈🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯✅🤯🙈✅✅🙈🙈🙈🤯🤯 |
| [typify 0.8.0](results/typify-0.8.0/README.md) | 12.0% | 🙈🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯✅🤯🙈✅✅🙈🙈🙈🤯🤯 |
| [ajv 8.20.0 (js)](results/ajv-8.20.0/README.md) [𝓥](#notes) | 0.0% | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| [go-jsonschema v0.24.1 (go)](results/go-jsonschema-v0.24.1/README.md) | 0.0% | 🙈🦀❌🦀❌🦀🦀🦀🦀🦀❌🦀🦀🦀❌🦀❌❌❌❌❌🦀🦀🦀🦀 |
| [json-schema-to-zod 2.8.1 + zod 4.6.7 (ts)](results/json-schema-to-zod-2.8.1-zod-4.6.7/README.md) | 12.0% | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈 |
| [ogen v1.24.0 (go)](results/ogen-v1.24.0/README.md) | 8.0% | 🙈❌❌❌🙈❌❌❌❌🦀❌❌🦀🦀❌🙈🙈🙈✅✅🙈🙈🙈❌❌ |
| [quicktype 26.0.0 (ts)](results/quicktype-26.0.0-typescript/README.md) | 16.0% | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌✅🙈🙈✅✅🙈🚧🚧🙈🚧 |
| [quicktype 26.0.0 (go)](results/quicktype-26.0.0-go/README.md) | 4.0% | 🙈🙈🙈✅🦀🙈🙈🙈🙈🦀🙈🦀🦀🦀❌🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈 |
| [typebox 0.34.52 + schema2typebox 1.7.8 (ts)](results/typebox-0.34.52-schema2typebox-1.7.8/README.md) | 0.0% | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |

## The tools

**ajv 8.20.0** — [ajv.js.org](https://ajv.js.org)  
The JSON Schema validator for JavaScript, and for much of the ecosystem the
reference implementation: it compiles a schema into purpose-built JS and runs
that at the boundary. No types come out; the schema stays the artifact. It's
the reference row for the Node path, resolving `localhost:1234` remotes from
the case's own `externalSchemas` and shipping no formats, so `format` is
ignored everywhere.

**go-jsonschema v0.24.1** — [GitHub](https://github.com/omissis/go-jsonschema)  
Go structs from JSON Schema, plus hand-written `UnmarshalJSON` methods that
re-check the schema's keywords while decoding. Its design center is a config
file with a known shape: a root that declares `type: object` becomes a tagged
struct, and `required`, `enum`, lengths, patterns and bounds are enforced in
the unmarshaller rather than left to the type system. The flip side is that a
root with no `type` keyword produces no root type at all, and the combinators
collapse into a merged type or `interface{}`.

**json-schema-rs 0.0.6+8032443** — [GitHub](https://github.com/goddtriffin/json-schema-rs)  
A young Rust generator aimed at hand-written config schemas, pinned to an
unreleased main that is moving fast: a type-graph IR, then self-validating
types on by default. Constrained scalars are newtypes checked by the crate's
own validator, embedded in the output, and `oneOf` picks its variant by
validation, unless no branch has anything to check. But a schema with no
`type` is `serde_json::Value`, with no validator at all, so `not`,
`if`/`then`/`else`, `contains`, `propertyNames` and most of `unevaluated*`
accept anything; and a struct's `Deserialize` is shape-only (the validating
path is `TryFrom<Value>`). Written by Claude Code: the repo runs from a
CLAUDE.md, though the session tags its commits once carried are gone from
the history. The published 0.0.6 (results in `archive/suite-1.041/`) refused
any root that wasn't an object.

**json-schema-to-zod 2.8.1 + zod 4.6.7** — [npm](https://www.npmjs.com/package/json-schema-to-zod)  
Turns a JSON Schema into a zod schema expression, so you get a static
TypeScript type and a runtime parser from one artifact. Its dispatcher keys
on `type`: a typed schema picks up bounds, lengths, patterns, enums and
`const`, while a schema without `type` becomes `z.any()` whatever else it
says. It has no `$ref` resolver. Unmaintained since March 2026; 2.8.1 is the
last release, and it runs on zod 4 here.

**jsonschema 0.60.2** — [crates.io](https://crates.io/crates/jsonschema)  
A Rust validator, not a generator: it interprets the schema at run time, so
it can enforce every keyword, which a type generator structurally can't. It's
the reference row: the ceiling the suite allows, a check on the harness, and
a price tag on the policies the generators live with. Remote refs come from
the case's own `externalSchemas`, the dialect from the harness, and `format`
stays at the library default (asserted through draft 7, annotation-only
after), because 2020-12's required tests say so.

**ogen v1.24.0** — [ogen.dev](https://ogen.dev)  
An OpenAPI v3 generator for Go: its design center is an API document, and
it emits a type per schema with a code-generated JSON codec and validator on
top of `go-faster/jx`. Each group here is wrapped as an OpenAPI 3.1 document
with the schema as a component (3.1 schema objects are JSON Schema 2020-12,
so nothing is translated), and the check calls both `Decode` and
`Validate`.

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
TypeBox is a TypeScript type builder: one `Type.Object({...})` definition is
both a JSON Schema fragment and a static type, and `TypeCompiler` checks
instances against it. Its design center is the TypeScript end of the wire,
so its checker refuses schemas it didn't build. Going from JSON Schema to
TypeBox needs a separate generator; this row uses schema2typebox, the one
TypeBox's readme points at. Most of what it can't express becomes
`Type.Unknown()`.

**typify 0.10.2** — [crates.io](https://crates.io/crates/typify)  
Rust types for serde from JSON Schema, built to generate OpenAPI clients
(progenitor). Its design center is idiomatic Rust: structs, enums, newtypes,
`deny_unknown_fields` when the schema is closed. Validation is whatever the
type system can say; anything else is silently absent, and a construct it
can't model fails at generation rather than producing a wrong type. The 0.10
line is the rewrite on top of typespace.

**typify 0.8.0** — [crates.io](https://crates.io/crates/typify)  
The last typify release before the typespace rewrite. Same design center as
0.10; it's here so the rewrite has something to be measured against.

## Notes

𝓥 A validator, not a code generator: it checks each instance against the schema at run time and generates no types. Every other row deserializes into generated types, so a validator marks the ceiling the suite allows on the Standard tier rather than a competing approach.

