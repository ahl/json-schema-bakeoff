# typebox 0.34.52 + schema2typebox 1.7.8

TypeBox is a TypeScript type builder: one `Type.Object({...})` definition is
both a JSON Schema fragment and a static type, and `TypeCompiler` checks
instances against it. Its design center is the TypeScript end of the wire,
so its checker refuses schemas it didn't build. Going from JSON Schema to
TypeBox needs a separate generator; this row uses schema2typebox, the one
TypeBox's readme points at. Most of what it can't express becomes
`Type.Unknown()`.

_generated 2026-11-22T02:41:30Z_

**59 of 540 standard groups pass (10.9%)**

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

## Standard

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 59 | 30 | 6 | 3 | 0 | 23 | 417 | 2 | 540 | 10.9% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 243 | 308 | 78.9% | 1664 | 14.6% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 3 | 4 | 0 | 12 | ❌❌❌🦀🦀🦀✅✅✅✅✅❌ |
| anchor-legacy.json | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 14 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 8 | ❌❌🦀🦀🦀❌✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 10 | 0 | 3 | 0 | 0 | 0 | 4 | 0 | 17 | ✅❌🚧❌✅✅🚧🚧❌❌✅✅✅✅✅✅✅ |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 0 | 21 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 0 | 21 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| enum.json | 11 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 17 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅✅✅ |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| id-legacy.json | 2 | 0 | 0 | 0 | 0 | 0 | 38 | 0 | 40 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌✅❌✅❌❌❌❌❌❌❌❌❌ |
| id.json | 0 | 0 | 0 | 0 | 0 | 0 | 25 | 0 | 25 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection-legacy.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 1 | 0 | 0 | 0 | 2 | 24 | 0 | 28 | ❌❌❌❌❌❌❌🙈🦀🦀✅❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 4 | 0 | 6 | ❌❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 9 | ✅✅✅❌✅🦀🦀✅❌ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 4 | 4 | 0 | 11 | ❌❌🦀🦀🦀🦀❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 2 | 0 | 0 | 0 | 2 | 0 | 4 | ❌❌🚧🚧 |
| patternProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 0 | 6 | ❌❌❌❌❌🙈 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| recursiveRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| ref-legacy.json | 0 | 0 | 0 | 1 | 0 | 2 | 11 | 0 | 14 | ❌❌❌❌❌🤪🦀🦀❌❌❌❌❌❌ |
| ref.json | 4 | 0 | 0 | 1 | 0 | 0 | 17 | 1 | 23 | 🤯❌❌❌❌❌❌❌🤪❌❌❌❌❌✅✅✅✅❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 7 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 12 | 🙈✅✅✅🙈✅✅✅✅🤪🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 46 | 0 | 47 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌🦀❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 6 | 23 | 0 | 0 | 0 | 1 | 14 | 1 | 45 | ✅🙈🙈🙈🙈🦀✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌✅✅🙈🙈🙈🙈🙈🙈🙈🤯❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 4 | ❌✅❌✅ |
| **Total** | **59** | **30** | **6** | **3** | **0** | **23** | **417** | **2** | **540** |  |

### Failures

#### 🤯 `generation_panic` (2)

- `ref.json::root pointer ref (2020-12)` — `TypeError: Converting circular structure to JSON`
- `unevaluatedProperties.json::unevaluatedProperties + single cyclic ref (2020-12)` — `RangeError: Maximum call stack size exceeded`

#### ❌ `generation_error` (417)

313 distinct first-line errors; most common:

- **8×** `ResolverError: Error reading "<repo>/generators/typebox/folderInteger.json": the bakeoff forbids filesystem access`
- **6×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$recursiveRef":"#"}`
- **6×** `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- **5×** `ResolverError: Error reading "<repo>/generators/typebox/list": the bakeoff forbids filesystem access`
- **4×** `ResolverError: Error downloading http://localhost:1234/subSchemas.json`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"patternProperties":{"^v":{}},"properties":{"bar":{}…`
- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"patternProperties":{"^á":{}},"$id":"T"}`
- `additionalProperties.json::additionalProperties with schema (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"boolean"},"properties":{"bar":{},"foo":{}},"$id":…`
- `additionalProperties.json::additionalProperties can exist by itself (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"boolean"},"$id":"T"}`
- `additionalProperties.json::additionalProperties are allowed by default (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{},"foo":{}},"$id":"T"}`
- `additionalProperties.json::additionalProperties does not look in applicators (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{}}}`
- `additionalProperties.json::additionalProperties with null valued instance properties (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"null"},"$id":"T"}`
- `additionalProperties.json::additionalProperties with propertyNames (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"number"},"propertyNames":{"maxLength":5},"$id":"T…`
- `additionalProperties.json::dependentSchemas with additionalProperties (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"dependentSchemas":{"foo":{},"foo2":{"properties":{"…`
- `allOf.json::allOf (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"type":"integer"}},"required":["bar"]}`
- `allOf.json::allOf with base schema (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{"type":"string"}},"required":["foo"]}`
- `allOf.json::allOf simple types (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maximum":30}`
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"multipleOf":3}`
- `anchor-legacy.json::Location-independent identifier (7)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$ref":"#foo"}`
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"definitions":{"A":{"id":"#foo","type":"integer"}},"properties":{"foo":{"$ref":"#…`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `ResolverError: Error downloading http://localhost:1234/nested.json`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `ResolverError: Error downloading http://localhost:1234/nested.json`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `ResolverError: Error downloading https://example.com/schema-with-anchor`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `ResolverError: Error downloading https://example.com/schema-with-anchor`
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `TypeError: The argument 'url' urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something. Received 'Invalid port in url'`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `TypeError: The argument 'url' urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something. Received 'Invalid port in url'`
- `anchor-legacy.json::anchor within remote ref (7)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `ResolverError: Error downloading http://localhost:1234/detached-ref.json`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `ResolverError: Error downloading http://localhost:1234/detached-ref.json`
- `anchor.json::Location-independent identifier (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$defs":{"A":{"$anchor":"foo","type":"integer"}},"$ref":"#foo","$id":"T"}`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `ResolverError: Error downloading http://localhost:1234/bar`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `ResolverError: Error downloading http://localhost:1234/nested.json`
- `anchor.json::same $anchor with different base uri (2020-12)` — `ResolverError: Error reading "<repo>/generators/typebox/child1": the bakeoff forbids filesystem access`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `ResolverError: Error downloading https://example.com/schema-with-anchor`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `TypeError: The argument 'url' urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something. Received 'Invalid port in url'`
- `anchor.json::anchor within remote ref (2020-12)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `ResolverError: Error downloading http://localhost:1234/locationIndependentIdentifier.json`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `ResolverError: Error downloading http://localhost:1234/detached-ref.json`
- `anyOf.json::anyOf (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"minimum":2}`
- `anyOf.json::anyOf with base schema (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxLength":2}`
- `anyOf.json::anyOf complex types (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"type":"integer"}},"required":["bar"]}`
- `boolean_schema.json::boolean schema 'true' (2020-12)` — `ResolverError: Error reading "<repo>/generators/typebox/": the bakeoff forbids filesystem access`
- `boolean_schema.json::boolean schema 'false' (2020-12)` — `Error: Expected a file path, URL, or object. Got false`
- `const.json::const with object (2020-12)` — `SyntaxError: Argument expression expected. (6:28)`
- `const.json::const with null (2020-12)` — `SyntaxError: Argument expression expected. (6:28)`
- `const.json::const with {"a": false} does not match {"a": 0} (2020-12)` — `SyntaxError: Argument expression expected. (6:28)`
- `const.json::const with {"a": true} does not match {"a": 1} (2020-12)` — `SyntaxError: Argument expression expected. (6:28)`
- `contains.json::contains keyword validation (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contains":{"minimum":5},"$id":"T"}`
- `contains.json::contains keyword with const keyword (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contains":{"const":5},"$id":"T"}`
- `contains.json::contains keyword with boolean schema true (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contains":true,"$id":"T"}`
- `contains.json::contains keyword with boolean schema false (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contains":false,"$id":"T"}`
- `contains.json::items + contains (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contains":{"multipleOf":3},"items":{"multipleOf":2},"$id":"T"}`
- … and 367 more

#### 🦀 `compile_fail` (23)

23 distinct first-line errors; most common:

- **1×** `g_15eb9179f5086255.ts(14,27): error TS2345: Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'.`
- **1×** `g_1e8a293ea52fa9f5.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_2f296854df82b484.ts(12,41): error TS2345: Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'.`
- **1×** `g_2f906fee05d2629f.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_2fcdc92229a336a7.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`

Per group:

- `allOf.json::allOf with boolean schemas, all true (2020-12)` — `g_f0fe2cef59ac90ed.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `allOf.json::allOf with boolean schemas, some false (2020-12)` — `g_5624b7cd9a68a5bf.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `allOf.json::allOf with boolean schemas, all false (2020-12)` — `g_5c62acb2ae1997f7.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `g_b299a37e3fc7af52.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `g_adc44db4991b0ae7.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `g_7b301e9e3da8497b.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `enum.json::heterogeneous enum validation (2020-12)` — `g_b34998acf2b40351.ts(16,16): error TS2345: Argument of type 'undefined[]' is not assignable to parameter of type 'TSchema'.`
- `enum.json::enum with [false] does not match [0] (2020-12)` — `g_655bb30e54889f06.ts(12,41): error TS2345: Argument of type 'TLiteral<false>[]' is not assignable to parameter of type 'TSchema'.`
- `enum.json::enum with [true] does not match [1] (2020-12)` — `g_892d3ba33e2431b2.ts(12,41): error TS2345: Argument of type 'TLiteral<true>[]' is not assignable to parameter of type 'TSchema'.`
- `enum.json::enum with [0] does not match [false] (2020-12)` — `g_f780844c45d0ef11.ts(12,41): error TS2345: Argument of type 'TLiteral<0>[]' is not assignable to parameter of type 'TSchema'.`
- `enum.json::enum with [1] does not match [true] (2020-12)` — `g_2f296854df82b484.ts(12,41): error TS2345: Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'.`
- `items.json::items and subitems (2019) (2019-09)` — `g_15eb9179f5086255.ts(14,27): error TS2345: Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'.`
- `items.json::items and subitems (7) (7)` — `g_9bca0a22a508d20a.ts(16,9): error TS2345: Argument of type 'TObject<{ foo: TOptional<TNull>; }>' is not assignable to parameter of type 'TSchema[]'.`
- `not.json::forbid everything with boolean schema true (2020-12)` — `g_c0116031e28355ec.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `not.json::allow everything with boolean schema false (2020-12)` — `g_8ecb68b040209552.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `oneOf.json::oneOf with boolean schemas, all true (2020-12)` — `g_f45eece91576a5c4.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `oneOf.json::oneOf with boolean schemas, one true (2020-12)` — `g_2f906fee05d2629f.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `oneOf.json::oneOf with boolean schemas, more than one true (2020-12)` — `g_2fcdc92229a336a7.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `oneOf.json::oneOf with boolean schemas, all false (2020-12)` — `g_37593ec9196a739b.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `ref-legacy.json::$ref to boolean schema true (7)` — `g_1e8a293ea52fa9f5.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `ref-legacy.json::$ref to boolean schema false (7)` — `g_b7463fde41823089.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `unevaluatedItems.json::unevaluatedItems with boolean schemas (2020-12)` — `g_c20efb2813aa28c5.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `unevaluatedProperties.json::unevaluatedProperties with adjacent bool additionalProperties (2020-12)` — `g_7c9f81c5875a0314.ts(14,30): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`

#### 🤪 `mixed` (3)

- `ref-legacy.json::naive replacement of $ref with its destination is not correct (7)` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `ref.json::naive replacement of $ref with its destination is not correct (2020-12)` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `type.json::multiple types can be specified in an array (2020-12)` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)

#### 🚧 `false_reject` (6)

6 distinct first-line errors; most common:

- **1×** `ASCII letters match: Expected string to match '^\p{Letter}+$' at <root>`
- **1×** `[false] is valid: Expected false at <root>`
- **1×** `[true] is valid: Expected true at <root>`
- **1×** `a valid case with a Unicode letter: Expected string to match '^\p{Letter}+$' at <root>`
- **1×** `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`

Per group:

- `const.json::const with array (2020-12)` — `same array is valid: Expected object at <root>`
  - cases: same array is valid (rejected)
- `const.json::const with [false] does not match [0] (2020-12)` — `[false] is valid: Expected false at <root>`
  - cases: [false] is valid (rejected)
- `const.json::const with [true] does not match [1] (2020-12)` — `[true] is valid: Expected true at <root>`
  - cases: [true] is valid (rejected)
- `multipleOf.json::small multiple of large integer (2020-12)` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `pattern.json::pattern with Unicode property escape requires unicode mode (2020-12)` — `ASCII letters match: Expected string to match '^\p{Letter}+$' at <root>`
  - cases: ASCII letters match (rejected), Non-ASCII letters match (rejected)
- `pattern.json::pattern with Unicode property escape (2020-12)` — `a valid case with a Unicode letter: Expected string to match '^\p{Letter}+$' at <root>`
  - cases: a valid case with a Unicode letter (rejected), another valid case with multiple Unicode letters (rejected)

#### 🙈 `false_accept` (30)

15 distinct first-line errors; most common:

- **9×** `accepted 'with unevaluated properties' though valid: false`
- **3×** `accepted 'when if is true and has unevaluated properties' though valid: false`
- **2×** `accepted 'base case: both properties present' though valid: false`
- **2×** `accepted 'number is invalid' though valid: false`
- **2×** `accepted 'with additional properties' though valid: false`

Per group:

- `items.json::items and subitems (2020-12)` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted)
- `oneOf.json::oneOf with required (2020-12)` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `patternProperties.json::patternProperties with Unicode property escape (2020-12)` — `accepted 'Unicode letter property name matches but value type is incorrect' though valid: false`
  - cases: Unicode letter property name matches but value type is incorrect (accepted)
- `type.json::integer type matches integers (2020-12)` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `type.json::object type matches objects (2020-12)` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `type.json::type: array or object (2020-12)` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `type.json::type: array, object or null (2020-12)` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `unevaluatedProperties.json::unevaluatedProperties schema (2020-12)` — `accepted 'with invalid unevaluated properties' though valid: false`
  - cases: with invalid unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties false (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with adjacent properties (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with adjacent patternProperties (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with nested properties (2020-12)` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with nested patternProperties (2020-12)` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with anyOf (2020-12)` — `accepted 'when one matches and has unevaluated properties' though valid: false`
  - cases: when one matches and has unevaluated properties (accepted), when two match and has unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with oneOf (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with not (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with if/then/else (2020-12)` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with if/then/else, then not defined (2020-12)` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with if/then/else, else not defined (2020-12)` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has no unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with dependentSchemas (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with boolean schemas (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with $ref (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::unevaluatedProperties before $ref (2020-12)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties outside (2020-12)` — `accepted 'with no nested unevaluated properties' though valid: false`
  - cases: with no nested unevaluated properties (accepted), with nested unevaluated properties (accepted)
- `unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties inside (2020-12)` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)
- `unevaluatedProperties.json::cousin unevaluatedProperties, true and false, true with properties (2020-12)` — `accepted 'with no nested unevaluated properties' though valid: false`
  - cases: with no nested unevaluated properties (accepted), with nested unevaluated properties (accepted)
- `unevaluatedProperties.json::cousin unevaluatedProperties, true and false, false with properties (2020-12)` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)
- `unevaluatedProperties.json::property is evaluated in an uncle schema to unevaluatedProperties (2020-12)` — `accepted 'uncle keyword evaluation is not significant' though valid: false`
  - cases: uncle keyword evaluation is not significant (accepted)
- `unevaluatedProperties.json::in-place applicator siblings, allOf has unevaluated (2020-12)` — `accepted 'base case: both properties present' though valid: false`
  - cases: base case: both properties present (accepted), in place applicator siblings, foo is missing (accepted)
- `unevaluatedProperties.json::in-place applicator siblings, anyOf has unevaluated (2020-12)` — `accepted 'base case: both properties present' though valid: false`
  - cases: base case: both properties present (accepted), in place applicator siblings, bar is missing (accepted)

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 15 | 8 | 0 | 0 | 0 | 0 | 75 | 0 | 98 | 15.3% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 57 | 71 | 80.3% | 1006 | 5.7% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 4 | 2 | 0 | 0 | 0 | 0 | 8 | 0 | 14 | ✅✅✅❌❌❌❌❌❌🙈🙈✅❌❌ |
| optional/content.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| optional/format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/non-schemas.json | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 0 | 13 | ❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/zero-terminated-floats.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **15** | **8** | **0** | **0** | **0** | **0** | **75** | **0** | **98** |  |

### Failures

#### ❌ `generation_error` (75)

60 distinct first-line errors; most common:

- **7×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"regex","$id":"T"}`
- **3×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$ref":"#my_anchor"}`
- **3×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"hostname","$id":"T"}`
- **2×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"email","$id":"T"}`
- **2×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"idn-hostname","$id":"T"}`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"date-time","$id":"T"}`
- `format-date.json::validation of date strings (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"date","$id":"T"}`
- `format-duration.json::validation of duration strings (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"duration","$id":"T"}`
- `format-email.json::validation of e-mail addresses (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"email","$id":"T"}`
- `format-email.json::validation of e-mail addresses against the RFC 5321 Mailbox rule (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"email","$id":"T"}`
- `format-hostname.json::validation of host names (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"hostname","$id":"T"}`
- `format-hostname.json::validation of host names without IDNA rules (6)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"hostname","$id":"T"}`
- `format-hostname.json::validation of A-label (punycode) host names (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"hostname","$id":"T"}`
- `format-idn-email.json::validation of an internationalized e-mail addresses (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"idn-email","$id":"T"}`
- `format-idn-hostname.json::validation of internationalized host names (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"idn-hostname","$id":"T"}`
- `format-idn-hostname.json::validation of separators in internationalized host names (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"idn-hostname","$id":"T"}`
- `format-ipv4.json::validation of IP addresses (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"ipv4","$id":"T"}`
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"ipv6","$id":"T"}`
- `format-iri-reference.json::validation of IRI References (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"iri-reference","$id":"T"}`
- `format-iri.json::validation of IRIs (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"iri","$id":"T"}`
- `format-json-pointer.json::validation of JSON-pointers (JSON String Representation) (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"json-pointer","$id":"T"}`
- `format-regex.json::validation of regular expressions (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"regex","$id":"T"}`
- `format-relative-json-pointer.json::validation of Relative JSON Pointers (RJP) (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"relative-json-pointer","$id":"T"}`
- `format-time.json::validation of time strings (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"time","$id":"T"}`
- `format-uri-reference.json::validation of URI References (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"uri-reference","$id":"T"}`
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"uri-template","$id":"T"}`
- `format-uri.json::validation of URIs (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"uri","$id":"T"}`
- `format-uuid.json::uuid format (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"uuid","$id":"T"}`
- `optional/bignum.json::maximum integer comparison (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maximum":18446744073709552000,"$id":"T"}`
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"exclusiveMaximum":true,"maximum":9.727837981879871e+26,"$id":"T"}`
- `optional/bignum.json::float comparison with high precision (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"exclusiveMaximum":9.727837981879871e+26,"$id":"T"}`
- `optional/bignum.json::minimum integer comparison (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"minimum":-18446744073709552000,"$id":"T"}`
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"exclusiveMinimum":true,"minimum":-9.727837981879871e+26,"$id":"T"}`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"exclusiveMinimum":-9.727837981879871e+26,"$id":"T"}`
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"uniqueItems":true,"$id":"T"}`
- `optional/bignum.json::multipleOf with integers above 2^53 (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"multipleOf":3,"$id":"T"}`
- `optional/content.json::validation of string-encoded content based on media type (7)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contentMediaType":"application/json","$id":"T"}`
- `optional/content.json::validation of binary string-encoding (7)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contentEncoding":"base64","$id":"T"}`
- `optional/content.json::validation of binary-encoded media type documents (7)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"contentEncoding":"base64","contentMediaType":"application/json","$id":"T"}`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `ResolverError: Error downloading http://localhost:1234/draft2019-09/dependentRequired.json`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `ResolverError: Error downloading http://localhost:1234/draft2020-12/prefixItems.json`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `ResolverError: Error downloading http://localhost:1234/draft7/ignore-dependentRequired.json`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `ResolverError: Error downloading http://localhost:1234/draft2019-09/ignore-prefixItems.json`
- `optional/dependencies-compatibility.json::single dependency (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":["foo"]},"$id":"T"}`
- `optional/dependencies-compatibility.json::empty dependents (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":[]},"$id":"T"}`
- `optional/dependencies-compatibility.json::multiple dependents required (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"quux":["foo","bar"]},"$id":"T"}`
- `optional/dependencies-compatibility.json::dependencies with escaped characters (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"foo\nbar":["foo\rbar"],"foo\"bar":["foo'bar"]},"$id":"T"}`
- `optional/dependencies-compatibility.json::single schema dependency (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":{"properties":{"bar":{"type":"integer"},"foo":{"type":"inte…`
- `optional/dependencies-compatibility.json::boolean subschemas (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":false,"foo":true},"$id":"T"}`
- `optional/dependencies-compatibility.json::schema dependencies with escaped characters (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"foo\tbar":{"minProperties":4},"foo'bar":{"required":["foo\"bar"]…`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `ResolverError: Error reading "<repo>/generators/typebox/item": the bakeoff forbids filesystem access`
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"pattern":"\\p{Letter}cole","$id":"T"}`
- `optional/ecmascript-regex.json::\w in patterns matches [A-Za-z0-9_], not unicode letters (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"pattern":"\\wcole","$id":"T"}`
- `optional/ecmascript-regex.json::pattern with ASCII ranges (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"pattern":"[a-z]cole","$id":"T"}`
- `optional/ecmascript-regex.json::\d in pattern matches [0-9], not unicode digits (2020-12)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"pattern":"^\\d+$","$id":"T"}`
- … and 25 more

#### 🙈 `false_accept` (8)

5 distinct first-line errors; most common:

- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'literal unicode character in json string' though valid: false`
- **1×** `accepted 'a float is not an integer even without fractional part' though valid: false`
- **1×** `accepted 'unicode matching is case-sensitive' though valid: false`

Per group:

- `optional/bignum.json::enum comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::const comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties (2020-12)` — `accepted 'unicode matching is case-sensitive' though valid: false`
  - cases: unicode matching is case-sensitive (accepted)
- `optional/ecmascript-regex.json::\w in patternProperties matches [A-Za-z0-9_], not unicode letters (2020-12)` — `accepted 'literal unicode character in json string' though valid: false`
  - cases: literal unicode character in json string (accepted), unicode character in hex format in string (accepted), unicode matching is case-sensitive (accepted)
- `optional/ecmascript-regex.json::patternProperties with ASCII ranges (2020-12)` — `accepted 'literal unicode character in json string' though valid: false`
  - cases: literal unicode character in json string (accepted), unicode character in hex format in string (accepted)
- `optional/ecmascript-regex.json::\d in patternProperties matches [0-9], not unicode digits (2020-12)` — `accepted 'ascii non-digits' though valid: false`
  - cases: ascii non-digits (accepted), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (accepted)
- `optional/ecmascript-regex.json::patternProperties with non-ASCII digits (2020-12)` — `accepted 'ascii non-digits' though valid: false`
  - cases: ascii non-digits (accepted)
- `optional/zero-terminated-floats.json::some languages do not distinguish between different types of numeric value (4)` — `accepted 'a float is not an integer even without fractional part' though valid: false`
  - cases: a float is not an integer even without fractional part (accepted)

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 100.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 34 | 34 | 100.0% | 34 | 100.0% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| **Total** | **5** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

_none: every group passed._

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 0 | 0 | 0 | 0 | 0 | 25 | 0 | 25 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 0 | 0 | - | 90 | 0.0% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| proposals/propertyDependencies/propertyDependencies.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **0** | **0** | **0** | **0** | **0** | **0** | **25** | **0** | **25** |  |

### Failures

#### ❌ `generation_error` (25)

19 distinct first-line errors; most common:

- **3×** `ResolverError: Error reading "<repo>/generators/typebox/extendible-dynamic-ref.json": the bakeoff forbids filesystem access`
- **2×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"propertyDependencies":{"foo":{"bar":false}},"$id":"T"}`
- **2×** `ResolverError: Error reading "<repo>/generators/typebox/baseSchema": the bakeoff forbids filesystem access`
- **2×** `ResolverError: Error reading "<repo>/generators/typebox/item": the bakeoff forbids filesystem access`
- **2×** `ResolverError: Error reading "<repo>/generators/typebox/list": the bakeoff forbids filesystem access`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$dynamicRef":"items"}`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/list": the bakeoff forbids filesystem access`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/intermediate-scope": the bakeoff forbids filesystem access`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/list": the bakeoff forbids filesystem access`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/inner_scope": the bakeoff forbids filesystem access`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/tree.json": the bakeoff forbids filesystem access`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/extendible-dynamic-ref.json": the bakeoff forbids filesystem access`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/extendible-dynamic-ref.json": the bakeoff forbids filesystem access`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/extendible-dynamic-ref.json": the bakeoff forbids filesystem access`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `ResolverError: Error downloading http://localhost:1234/detached-dynamicref.json`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/item": the bakeoff forbids filesystem access`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/first": the bakeoff forbids filesystem access`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"date-time","$id":"T"}`
- `format-time.json::leap seconds are not valid times (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"time","$id":"T"}`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/item": the bakeoff forbids filesystem access`
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"properties":{"foo2":{}},"propertyDependencies":{"fo…`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/genericList": the bakeoff forbids filesystem access`
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/inner": the bakeoff forbids filesystem access`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-objects (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"propertyDependencies":{"foo":{"bar":false}},"$id":"T"}`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-string property values (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"propertyDependencies":{"foo":{"bar":false}},"$id":"T"}`
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"propertyDependencies":{"foo":{"bar":{"maxProperties":2,"minProperties":2},"baz":…`
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{"type":"string"}},"propertyDependencies":{"foo":{"foo1":{"pr…`
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo2":{}},"propertyDependencies":{"foo":{},"foo2":{"bar":{"propert…`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/baseSchema": the bakeoff forbids filesystem access`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `ResolverError: Error reading "<repo>/generators/typebox/baseSchema": the bakeoff forbids filesystem access`

