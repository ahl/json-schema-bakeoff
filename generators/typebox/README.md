# typebox

[TypeBox](https://github.com/sinclairzx81/typebox) 0.34, driven from JSON
Schema by [schema2typebox](https://github.com/xddq/schema2typebox) 1.7, both
pinned in `package.json`. The version string — which names
`results/<id>/` — is `typebox 0.34.52 + schema2typebox 1.7.8`.

This row answers the same question typify's rows do: **does the tool turn a
schema into a type, and does that type agree with the schema?** TypeBox is
the TypeScript library where those two halves are the same object, so it is
the closest analogue in the language.

## Why a second package is in the pipeline, and what that means

TypeBox builds types; it does not read them. `Type.Object({ x:
Type.Number() })` returns a value that is simultaneously a draft-7 JSON
Schema fragment and a static TypeScript type, and `TypeCompiler` / `Value`
check instances against it. But every checker dispatches on the
`Symbol.for('TypeBox.Kind')` property that the `Type.*` constructors
attach, so a schema TypeBox did not build is not a schema it can check:

```js
Value.Check({ type: 'string' }, 'x')              // throws "Unknown type"
TypeCompiler.Compile({ type: 'string' })          // throws "Preflight validation
                                                  //   check failed to guard for
                                                  //   the given schema"
Value.Check(Type.Unsafe({ type: 'string' }), 'x') // throws "Unknown type"
                                                  //   (Kind is 'Unsafe', which
                                                  //   nothing implements)
```

`Type.Unsafe` copies the schema and stamps `Kind: 'Unsafe'`, for which no
check function is registered; it is a hole for *declaring* a type to
TypeScript, not a way to run one. So there is no "hand TypeBox the raw
schema" route to measure, and `TypeSystemPolicy` doesn't change that.

Nor is there a JSON Schema front end in `@sinclair/typebox-codegen` 0.11.1,
which is where you'd look first. That package exports
`TypeScriptToTypeBox` and `TypeScriptToModel` — TypeScript *source* in — and
then `ModelToZod`, `ModelToValibot`, `ModelToIoTs`, `ModelToEffect`,
`ModelToArkType`, `ModelToYup`, `ModelToJavaScript`, `ModelToTypeScript`,
`ModelToValue` and `ModelToJsonSchema` out. There is no
`JsonSchemaToTypeBox` and no `JsonSchemaToModel`; it is **not installed
here**. `@sinclair/typemap`, from the same author, is likewise a TypeScript
*syntax* front end (`TypeBox('{ x: number }')`), not a JSON Schema one.

What exists is `schema2typebox`, listed in TypeBox's own readme ecosystem
table as "Creating TypeBox code from Json Schemas". It emits TypeScript:

```ts
import { Static, Type } from "@sinclair/typebox";

export type T = Static<typeof T>;
export const T = Type.Object({ "foo": Type.Number() }, { additionalProperties: false, $id: "T" });
```

So the route is **codegen, not raw**: schema2typebox writes the `Type.*`
source, `tsc` compiles it, TypeBox's compiler checks the cases. Read the row
accordingly — a ❌ is almost always schema2typebox declining to translate,
and everything from 🦀 rightwards is TypeBox's behavior on what it was
handed. The two are separated keyword by keyword below.

## Files

| file | what it is |
|---|---|
| `generator.toml` | the three commands and the row's description |
| `package.json` / `package-lock.json` | the pins: `@sinclair/typebox` 0.34.52, `schema2typebox` 1.7.8, `typescript` 5.9.3 |
| `deps.mjs` | `npm ci` bootstrap, stdout muzzle, short-write-safe `writeAll`, the network block |
| `generate.mjs` | `version` and `generate` |
| `build.mjs` | `tsc` pass loop, diagnostic attribution, the probe, the runner |
| `probe.mjs` | loads one compiled module per line so a module that throws on load is blamed on itself |

`typescript` is a direct dependency because the output is compiled (see
below); schema2typebox's own `typescript` 5.0.4 stays nested under it, so
`node_modules/.bin/tsc` is the pinned 5.9.3.

## Commands

```sh
node generate.mjs version                 # typebox 0.34.52 + schema2typebox 1.7.8
node generate.mjs generate <out_dir>      # stdin: [{key, draft, schema}] → <out_dir>/<key>.ts
node build.mjs <gen_dir> <batch_dir>      # stdin: [key] → {outcomes, runner}
```

Versions are read from `node_modules/<pkg>/package.json`, never hardcoded.

## generate

One long-lived process reads the whole array off stdin and calls
`schema2typebox({ input })` per group. There are no options to set:
the programmatic API takes the schema text and nothing else, prettier
formats the result (via `prettier-plugin-organize-imports`, which prettier
2 auto-discovers and which is what strips the unused names out of the
emitted import), and TypeBox's `TypeSystemPolicy` is left at its default —
JSON Schema assertion semantics rather than the TypeScript ones.

**Numbers.** The harness sends the suite's digits exactly
(serde_json `arbitrary_precision`), and schema2typebox takes the schema as
*text*, so `generate.mjs` parses stdin with a `JSON.parse` reviver that
keeps each number's `context.source` and writes it back verbatim. A
50-digit `minimum` reaches the tool as 50 digits; `1.0000000000000002`
reaches it as `1.0000000000000002`. What happens next is the tool's:
`parseSchemaOptions` does its own `JSON.parse`/`JSON.stringify`, so the
50-digit integer is emitted as `minimum: 1.2345678901234567e49` while the
float round-trips exactly.

**Root naming.** schema2typebox names its export after the root schema's
`title`, pascal-cased, falling back to `T`, and has no option for it.
Injecting a `title` would be editing the schema under measurement, so
instead the emitted name is read off the `export const` line and aliased:

```ts
// Appended by the bakeoff harness. Everything above is schema2typebox's.
import { TypeCompiler as __TypeCompiler } from "@sinclair/typebox/compiler";

export const Root = T;

const __checker = __TypeCompiler.Compile(Root);

export function __conformanceCheck(json: string): void {
  const value: unknown = JSON.parse(json);
  if (__checker.Check(value)) return;
  const first: any = __checker.Errors(value).First();
  if (first === undefined) throw new Error("the value does not match the type");
  throw new Error(`${first.message} at ${first.path === "" ? "<root>" : first.path}`);
}
```

Nothing is stripped from the tool's output; the block above is appended to
it. The alias is skipped when `title` already made the export `Root`.

`TypeCompiler.Compile` runs at **module scope**, deliberately: that is where
TypeBox refuses a type it can't build a checker for, and a type with no
checker can't answer any case. Failing there should be 🦀, not a rejection
of every case in the group — which is what the load probe in `build` is for.

### Statuses

| status | when |
|---|---|
| `error` | the tool threw. Mostly `Error: Unsupported schema. Did not match any type of the parsers` — 1015 of the 1216 ❌ groups counting core and optional together (1011 of them are core) — plus `ResolverError` for a `$ref` it may not retrieve, and `SyntaxError` from prettier when the tool emits text that isn't TypeScript |
| `panic` | `RangeError` (stack exhausted on a cyclic dereferenced schema), `TypeError: Converting circular structure to JSON` (the same cycle, reached through the tool's own error message), or the 30 s timeout |

Six of the seven 🤯 are the `TypeError`; exactly one — `draft2019-09`'s
`unevaluatedProperties + single cyclic ref` — exhausts the stack first. The
timeout is raced against the call but can only fire during the asynchronous
half: the tool awaits the dereference and then generates synchronously, so
a runaway recursion blocks the event loop. In practice the cycle is always
caught — all five drafts generate in about 3 s total, and no group has ever
taken the process down. That is luck rather than design, though: `generate`
writes its status array once, at the end, in one process, so an uncatchable
death (a V8 heap OOM on a cyclic structure) would cost the whole draft
rather than one group. The Rust rows supervise a worker for exactly this;
the Node rows, this one and `json-schema-to-zod`, do not.

### Dialect

Ignored, and nothing is lost by that. TypeBox targets draft 7 and has no
dialect setting; schema2typebox has none either and never reads `$schema`
(it passes it through as an inert schema option). The `draft` field arrives
on stdin and is dropped, so the per-draft columns differ only because the
suite's schemas differ.

### No retrieval, over the network or off the disk

schema2typebox dereferences `$ref` with
`@apidevtools/json-schema-ref-parser` 9.0.9 before generating anything, and
there is no way to configure that through the API — it calls
`$RefParser.dereference(schema)` with no options. Its default resolver set
is HTTP plus filesystem and `deps.mjs` shuts off both before the tool is
imported.

HTTP, because left alone it would fetch `http://localhost:1234/...` for
`refRemote.json` *and* `http://json-schema.org/draft-07/schema` for the
groups that `$ref` a meta-schema. The resolver holds the `http` and `https`
module objects and calls `protocol.get(...)`, so those two functions are
replaced:

```
ResolverError: Error downloading http://localhost:1234/integer.json
the bakeoff forbids network access, node:http.get
```

The filesystem, because the file resolver resolves a relative `$ref`
against the process's cwd — which for a generator command is *this
directory*. `{"$ref": "folderInteger.json"}` was an honest ENOENT, but
`{"$ref": "package.json"}` would have been read, an absolute `$ref` would
read anywhere on disk, and the row's results would quietly depend on what
sits next to this file. A code generator resolves nothing, so the
resolver's `read` is replaced with a `ResolverError` too (a `ResolverError`
specifically: `parse.js` relays one of those unchanged and replaces
anything else with a generic "Error reading file" that loses the reason):

```
ResolverError: Error reading ".../generators/typebox/folderInteger.json": the bakeoff forbids filesystem access
```

Both patches fail loudly if the resolver module isn't where it was, rather
than leaving the sandbox open. Nothing under `suite/remotes/` is reachable
either way; a generator row is supposed to forfeit the remotes.

Local pointers *are* inlined, but not reliably: a root `$ref` comes back
rewritten to `"#"`, so `{"$ref": "#/definitions/a", "definitions": {...}}`
ends up as `Unsupported schema ... {"definitions":{...},"$ref":"#"}`, and a
recursive `$ref` produces a cyclic object that the code generator walks
until the stack runs out.

## build: `tsc`, then a load probe

Two gates, both reported as 🦀:

1. **`tsc`.** schema2typebox's output is *not* erasable TypeScript: it
   writes `import { Kind, SchemaOptions, Static, TSchema, TUnion, Type,
   TypeRegistry } from "@sinclair/typebox"` with the type-only names on the
   value import, which node's type stripping turns into a runtime
   `SyntaxError: ... does not provide an export named 'SchemaOptions'`. So
   unlike the `json-schema-to-zod` row there is a real compiler here — and
   that is the right measurement anyway, because the generated value *is*
   the static type, so a type error in it is a result rather than a harness
   artifact. The settings match the quicktype TypeScript target
   (`strict: false`, `noImplicitAny: false`, `types: []`,
   `skipLibCheck: true`, `module: commonjs`), so what survives is about the
   generated code and not about the strictness dial. Diagnostics are
   attributed by the `file.ts(L,C):` prefix, the blamed modules are dropped,
   and the batch is compiled again until it is clean (cap 10 passes).
2. **the load probe** (`probe.mjs`). TypeBox validates at type construction
   and again inside `TypeCompiler.Compile`, and the generated modules do
   both in their body. Each compiled module is `require`d on its own, one
   line of output per module, flushed, so a module that takes the process
   down is blamed on itself and the probe restarts past it.

A `__canary.ts` is compiled and run first — it imports `@sinclair/typebox`,
compiles a `Type.String({ maxLength: 2 })` and checks two values. Every
generated module depends on exactly that much working, so a broken install,
a missing `node_modules/@sinclair` symlink or a bad tsconfig is a hard
failure of the build step instead of a few hundred meaningless 🦀.

The runner is plain CommonJS written straight into `<batch_dir>/out/`, not
TypeScript: it needs no types, and keeping it out of the `tsc` graph means
the surviving set can be fixed up after the probe without another compile.
It loads its modules by `require` inside a `try`, so a key that isn't there
is a `{"panic": "module not compiled"}` on its cases rather than a dead
process. Case data arrives through the same `JSON.parse` reviver as in
`generate` and is re-serialized with the original digits, so
`__conformanceCheck` does its own `JSON.parse` — a 50-digit integer is a
float64 by the time TypeBox sees it, which is what a user of this tool
would get.

In practice the two gates cost 86 groups — 6 in draft 4 and 20 in each
draft from 6 onward — and every one is a schema2typebox bug that `tsc`
catches. The whole distribution:

| groups | emitted | diagnostic | from |
|---:|---|---|---|
| 47 | `Type.Intersect([true])`, `Type.Union([false])` | `Type 'boolean' is not assignable to type 'TSchema'` | a boolean subschema inside `allOf`/`anyOf`/`oneOf` (and 3 inside `unevaluated*`): `collect` returns `JSON.stringify(schema)` for one, under a `// TODO: boolean schema support..?` |
| 20 | `Type.Union([Type.Array([Type.Literal(1)]), …])` | `Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'` | all in `enum.json`: `parseType` maps a JSON array *inside* an `enum` to `Type.Array([...])`, passing an array where a schema goes |
| 10 | `Type.Not(true)` | `Argument of type 'boolean' is not assignable to parameter of type 'TSchema'` | the same boolean hole under `not` (8 in `not.json`, 2 in `unevaluatedProperties.json`) |
| 5 | `Type.Array([])` | `Argument of type 'undefined[]' is not assignable to parameter of type 'TSchema'` | an *empty* array inside an `enum` |
| 4 | `Type.Array(Type.Union(Type.Unknown()))` | `Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'` | tuple-form `items`: `parseArray` writes `Type.Array(Type.Union(${code}))` without the brackets `Type.Union` needs. These four (`items.json::items and subitems`, one per draft from 6 on) are the only tuple-`items` schemas that get far enough to compile at all |

Nothing reaches the probe as a result, so the probe currently finds nothing.
It stays because it is the only gate for the things TypeBox itself refuses
at run time — `Type.Literal({...})`, and the `Preflight validation check
failed to guard for the given schema` that a raw sub-schema in `contains`
or a boolean `exclusiveMinimum` produces — each of which is one
schema2typebox fix away from being reachable.

## What TypeBox actually enforces

`parseSchemaOptions` is the reason this row has teeth: schema2typebox
consumes only `title`, `type`, `items`, `allOf`, `anyOf`, `oneOf`, `not`,
`properties`, `required`, `const` and `enum`, and passes **every other
keyword straight into the TypeBox options object**. So `minimum`,
`maxLength`, `pattern`, `uniqueItems`, `additionalProperties`, `format` and
also `propertyNames`, `patternProperties` and `unevaluatedProperties` all
arrive in the schema TypeBox checks. Whether they do anything is TypeBox's
decision, and `@sinclair/typebox/compiler` is the thing being measured.

But there is a hole in front of all of it. `collect` tests
`isObjectSchema` — `type === "object"` — **before** every combinator, and
`parseObject` returns a bare `Type.Unknown()` when `properties` is absent.
So `{"type": "object"}` is a checker that accepts everything, and
`{"type": "object", "oneOf": [...]}`, `{"type": "object", "minProperties":
1}` and `{"type": "object", "additionalProperties": {...}}` are *also*
exactly `Type.Unknown()`: the `type` keyword silently swallows the rest of
the schema. **49 of the 469 generated modules are literally
`export const T = Type.Unknown();`** — a sixth of everything that gets
past generation — and that, not any TypeBox gap, is the single largest
cause of 🙈. Read every row below as "when the shape reaches TypeBox at
all".

Enforced, with the semantics TypeBox gives them:

| keyword | on | enforced? |
|---|---|---|
| `type: string/number/boolean/null/array` | the kind | yes |
| `type: object` | `Type.Object` only with `properties`; otherwise `Type.Unknown()` | see above. `type.json::object type matches objects` is 🙈 in all five drafts for this reason |
| `type: integer` | — | **no**: schema2typebox maps `integer` to `Type.Number()`, not `Type.Integer()`, so `1.5` passes. 5 of the 49 🙈 groups (one per draft) are this one line |
| `minLength` / `maxLength` | `Type.String` | yes, by `value.length` — UTF-16 code units, where JSON Schema counts code points, so `"💩💩"` fails `maxLength: 2` |
| `pattern` | `Type.String` | yes: `new RegExp(schema.pattern).test(value)` — **no flags**, so an unanchored search (correct) in non-unicode mode (`\p{Letter}` is an Annex B identity escape) |
| `format` | `Type.String` | yes, as a *rejection*: `FormatRegistry.Has(format)` is consulted first and an unregistered format fails every value. The registry ships **empty** and is deliberately left empty, so any `format` that reached a checker here would be 🚧. None does — all 142 `optional/format/` group schemas across the five drafts declare no `type` (some carry `$schema`/`$id` as well as `format`), which schema2typebox refuses, so zero of the 469 generated modules contain a `format` |
| `minimum` / `maximum` / `exclusiveMinimum` / `exclusiveMaximum` | `Type.Number` | yes, in the numeric form. Draft 4's **boolean** `exclusiveMinimum` is not ignored, it is fatal: `Type.Number({minimum: 2, exclusiveMinimum: true})` fails `TypeCompiler.Compile`'s preflight guard, which would be 🦀. No suite group reaches it — all four draft-4 boolean-exclusive groups omit `type` |
| `multipleOf` | `Type.Number` | yes, by `value % multipleOf === 0`, so `1e-8` and `0.0001` behave like float remainders |
| `minItems` / `maxItems` | `Type.Array` | yes |
| `uniqueItems` | `Type.Array` | yes, structurally — two objects with equal contents count as one |
| `contains` | `Type.Array` | only with a TypeBox sub-schema. The tool passes the raw JSON sub-schema through `parseSchemaOptions`, and `Type.Array(…, {contains: {type: "number"}})` fails `Compile`'s preflight guard, so it would be 🦀. No suite group reaches it: every `contains` group omits `type: array` |
| `items: <schema>` | `Type.Array(inner)` | yes |
| `required` | `Type.Object` | yes — schema2typebox wraps every non-required property in `Type.Optional` |
| `additionalProperties: false` | `Type.Object` | yes **with `properties`**; with `properties: {}` the tool emits `Type.Record(Type.String(), false)`, which is 🦀, and with no `properties` key at all the whole schema is `Type.Unknown()` |
| `additionalProperties: <schema>`, no `properties` | `Type.Record(Type.String(), inner)` | yes, but only if the root isn't `type: "object"`, which short-circuits to `Type.Unknown()` first |
| `additionalProperties: <schema>`, with `properties` | `Type.Union([Type.Object({...}), Type.Record(...)])` | no — the union makes the record an *alternative* to the object, and every other option on that schema is dropped |
| `minProperties` / `maxProperties` | `Type.Object` | yes, and again only with `properties` present |
| `enum` / `const` of scalars | `Type.Union([Type.Literal(v), ...])` | yes |
| `anyOf` | `Type.Union` | yes |
| `allOf` | `Type.Intersect` | yes |
| `oneOf` | a registered `ExtendedOneOf` kind the tool emits, counting branches with `Value.Check` | yes, exactly one. `TypeCompiler` handles the custom kind |
| `not` | `Type.Not` | yes |

Reaches TypeBox and is ignored — `Type.Object` and `Type.Array` read only
the keywords above, so these are silently annotations:
`patternProperties`, `propertyNames`, `dependencies`, `dependentRequired`,
`dependentSchemas`, `if`/`then`/`else`, `unevaluatedProperties`,
`unevaluatedItems`, `prefixItems`, `additionalItems`, `contentEncoding`,
`contentMediaType`, `$schema`, `$comment`, `definitions`, `$defs`.
`unevaluatedProperties.json` is 23 of the 49 🙈, all of them in 2019-09 —
but only 12 of those 23 are TypeBox ignoring the keyword; the other 11 are
`Type.Unknown()` modules, where the keyword never got that far. 2020-12's
version of the same file mostly doesn't generate at all, which is the whole
reason the 🙈 column is 28 in one draft and 5–6 in the others.

### The `$id` collision

schema2typebox stamps `$id` on the root when the schema has none — and
`parseWithMultipleTypes` hands the *same* options object to every member of
the union it builds for `type: [a, b]`:

```ts
export const T = Type.Union([
  Type.Number({ $id: "T" }),
  Type.String({ $id: "T" }),
]);
```

`TypeCompiler` keys its generated functions by `$id`, so both branches
compile to one `check_T` and the union collapses to its first member:
`{"type": ["integer", "string"]}` accepts numbers and rejects strings. It
is also the one case where `Check` returns false while
`Errors(value).First()` is `undefined`, so the recorded rejection is
`__conformanceCheck`'s own fallback text, `the value does not match the
type`, rather than a TypeBox message. This is the one place where the
measurement depends on which TypeBox checker is used — `Value.Check` walks the schema and gets it right. `TypeCompiler` is
kept because it is TypeBox's recommended path (and the one its own
benchmarks publish); 10 of the 469 generated modules carry a duplicated
`$id`, so that is the ceiling on the difference.

### `const`, and what the tool does to it

`parseConst` reads an **array** `const` as a union of its elements, so
`{"const": [false]}` becomes "the value `false`", not "the array
`[false]`"; an **object** `const` emits the literal text
`Type.Todo(const with object)`, which prettier then refuses to parse
(`SyntaxError: Argument expression expected`, 18 groups); and a **string**
`const` is interpolated unescaped into `Type.Literal("...")`, so a const
containing a quote or a newline is `SyntaxError: Unterminated string
literal` (5 groups). Those three account for most of `const.json`.

## Results

| draft | core groups | pass | optional groups | pass |
|---|---:|---:|---:|---:|
| draft4 | 160 | 37 (23.1%) | 39 | 14 (35.9%) |
| draft6 | 233 | 46 (19.7%) | 44 | 14 (31.8%) |
| draft7 | 258 | 46 (17.8%) | 64 | 14 (21.9%) |
| 2019-09 | 373 | 51 (13.7%) | 76 | 14 (18.4%) |
| 2020-12 | 384 | 47 (12.2%) | 78 | 14 (17.9%) |
| **all** | **1408** | **227 (16.1%)** | 301 | 70 (23.3%) |

Core buckets across all drafts: ✅227 🙈49 🚧19 🤪9 🦀86 ❌1011 🤯7.

The shape of the row is one number: **486 of the 1709 groups produce a
module at all** (469 distinct `.ts` files, since groups with identical
schemas share a key). `collect` needs `type: "object"`, `type: "array"`, a
named scalar `type`, a combinator, `enum` or `const`, and the suite tests
one keyword at a time without a `type` (`{"maxLength": 2}`,
`{"properties": {...}}`, `{"uniqueItems": true}`, `{"required": ["foo"]}`).
Even the empty schema is refused: `isUnknownSchema` wants
`Object.keys(schema).length === 0`, but the tool stamps `$id` on the root
first, so `{}` arrives as `{"$id": "T"}` and throws `Unsupported schema`.
(`parseUnknown` therefore only ever fires for a *nested* `{}`.)

That is the same wall `json-schema-to-zod` hits — its dispatcher keys on
`type` too — and the difference is which column the fallthrough lands in.
zod answers `z.any()`, which accepts everything, so those groups are 🙈 or
accidentally ✅ and its row has no ❌ at all; schema2typebox throws for the
untyped case and they are ❌ here. It is not that this tool has no
accept-everything fallback — `Type.Unknown()` is one, and 49 modules are
exactly that — only that it is reached from `type: "object"` instead of
from a missing `type`.

Of the 304 core groups whose module compiles and loads, 227 pass — three in
four. `pattern`, `uniqueItems`, the length, bound and property-count
keywords, `additionalProperties: false`, `required`,
`anyOf`/`allOf`/`oneOf` and `not` are all enforced properly, which is more
than quicktype's TypeScript row models at all (it emits interfaces plus a
hand-written `Convert` that checks shape). 🙈 is 49 groups here against
quicktype TypeScript's 512 and zod's 774 — but on a tenth as many groups
that got that far, and a third of this row's 🙈 is `Type.Unknown()`, which
is the same `z.any()` failure wearing a different name.

Where it trails: everything that needs a front end. `$ref` (local or
remote), `if`/`then`/`else`, `dependencies`, `patternProperties`,
`propertyNames`, `contains`, tuple `items`, and boolean subschemas are all
❌ or 🦀 before TypeBox is involved — and `type: "object"` without
`properties` throws the rest of its own schema away.
