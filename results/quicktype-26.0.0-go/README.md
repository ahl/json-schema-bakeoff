# quicktype 26.0.0 go

Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

_generated 2026-11-22T02:30:57Z_

**101 of 540 standard groups pass (18.7%)**

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
| 101 | 130 | 13 | 8 | 0 | 183 | 92 | 13 | 540 | 18.7% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 503 | 741 | 67.9% | 1664 | 30.2% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 2 | 2 | 0 | 0 | 0 | 6 | 2 | 0 | 12 | 🙈🙈🦀🦀❌❌🦀🦀✅✅🦀🦀 |
| anchor-legacy.json | 3 | 0 | 0 | 0 | 0 | 0 | 9 | 2 | 14 | ✅❌✅❌✅❌🤯🤯❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 1 | 9 | ❌❌❌❌❌🤯❌❌❌ |
| anyOf.json | 1 | 1 | 0 | 0 | 0 | 4 | 2 | 0 | 8 | 🦀🙈🦀❌❌✅🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| const.json | 4 | 6 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈🙈🙈🙈 |
| contains.json | 0 | 1 | 0 | 0 | 0 | 20 | 0 | 0 | 21 | 🦀🦀🦀🦀🙈🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| content.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| dependentSchemas.json | 0 | 2 | 0 | 0 | 0 | 6 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🙈🙈 |
| dynamicRef.json | 4 | 9 | 0 | 0 | 0 | 3 | 5 | 0 | 21 | 🙈🙈🙈✅🦀❌❌❌❌❌🙈🦀🙈🙈🙈✅✅🙈✅🦀🙈 |
| enum.json | 5 | 6 | 1 | 0 | 0 | 0 | 5 | 0 | 17 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈❌🙈🙈 |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| id-legacy.json | 16 | 0 | 0 | 1 | 0 | 0 | 17 | 6 | 40 | ❌❌❌❌❌🤪❌✅❌✅❌✅❌🤯🤯🤯✅✅✅✅✅✅✅✅🤯🤯✅❌✅✅✅✅❌🤯❌❌❌❌❌❌ |
| id.json | 11 | 0 | 0 | 0 | 0 | 1 | 9 | 4 | 25 | ❌❌❌✅✅✅🦀❌✅🤯🤯🤯✅✅✅✅🤯✅✅✅❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 8 | 10 | 2 | 3 | 0 | 2 | 3 | 0 | 28 | ✅🤪🤪✅❌❌❌🙈🙈🙈✅✅✅✅✅✅🙈🙈🦀🚧🚧🙈🙈🙈🤪🙈🙈🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 6 | 🦀🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 0 | 8 | 0 | 0 | 9 | 🦀🦀🦀🙈🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 0 | 4 | 3 | 0 | 11 | 🦀🙈🦀❌❌❌🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 4 | 🦀🦀🙈🙈 |
| patternProperties.json | 0 | 1 | 0 | 0 | 0 | 5 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🙈 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| recursiveRef.json | 1 | 6 | 1 | 0 | 0 | 2 | 0 | 0 | 10 | 🙈🙈✅🙈🙈🙈🙈🦀🦀🚧 |
| ref-legacy.json | 5 | 0 | 0 | 0 | 0 | 1 | 8 | 0 | 14 | ✅❌✅✅✅❌🦀❌❌✅❌❌❌❌ |
| ref.json | 9 | 3 | 0 | 0 | 0 | 1 | 10 | 0 | 23 | 🙈✅✅❌✅🙈✅✅❌🦀❌❌❌🙈✅✅✅✅❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 4 | 6 | 1 | 0 | 0 | 1 | 0 | 0 | 12 | 🙈🚧🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| unevaluatedItems.json | 2 | 23 | 6 | 2 | 0 | 14 | 0 | 0 | 47 | 🦀🦀🦀✅🙈🙈🚧🚧✅🦀🙈🙈🦀🦀🚧🚧🚧🚧🙈🙈🙈🙈🙈🙈🤪🤪🦀🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🦀🦀🙈🙈 |
| unevaluatedProperties.json | 7 | 34 | 0 | 0 | 0 | 4 | 0 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🙈🙈🙈🙈 |
| uniqueItems.json | 0 | 4 | 2 | 2 | 0 | 2 | 0 | 0 | 10 | 🦀🦀🤪🤪🙈🙈🚧🚧🙈🙈 |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 4 | ❌✅❌✅ |
| **Total** | **101** | **130** | **13** | **8** | **0** | **183** | **92** | **13** | **540** |  |

### Failures

#### 🤯 `generation_panic` (13)

1 distinct first-line errors; most common:

- **13×** `Error: URNs do not have any generally defined hierarchical components`

Per group:

- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `Error: URNs do not have any generally defined hierarchical components`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::simple URN base URI with $ref via the URN (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::simple tag URI base URI with $ref via the tag URI (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::simple base URI using an arbitrary URI scheme with $ref to that URI (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::URN base URI with URN and JSON pointer ref (7)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::URN base URI with URN and JSON pointer ref (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `id-legacy.json::URN ref with nested pointer ref (id) (4)` — `Error: URNs do not have any generally defined hierarchical components`
- `id.json::simple URN base URI with $ref via the URN (2020-12)` — `Error: URNs do not have any generally defined hierarchical components`
- `id.json::simple tag URI base URI with $ref via the tag URI (2020-12)` — `Error: URNs do not have any generally defined hierarchical components`
- `id.json::simple base URI using an arbitrary URI scheme with $ref to that URI (2020-12)` — `Error: URNs do not have any generally defined hierarchical components`
- `id.json::URN base URI with URN and JSON pointer ref (2020-12)` — `Error: URNs do not have any generally defined hierarchical components`

#### ❌ `generation_error` (92)

54 distinct first-line errors; most common:

- **14×** `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- **6×** `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- **4×** `SchemaFetchError: Could not fetch schema http://localhost:1234/subSchemas.json, referred to from Root#`
- **3×** `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-ref.json, referred to from Root#`
- **3×** `SchemaFetchError: Could not fetch schema http://localhost:1234/different-id-ref-string.json, referred to from Root#`

Per group:

- `allOf.json::allOf with boolean schemas, some false (2020-12)` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/allOf/1`
- `allOf.json::allOf with boolean schemas, all false (2020-12)` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/allOf/0`
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `SchemaKeyNotInObject: Key foo not in schema object at Root#foo`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/nested.json, referred to from Root#/properties/foo`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `SchemaFetchError: Could not fetch schema https://example.com/schema-with-anchor, referred to from Root#/properties/foo`
- `anchor-legacy.json::anchor within remote ref (7)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-ref.json, referred to from Root#`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-ref.json, referred to from Root#`
- `anchor.json::Location-independent identifier (2020-12)` — `SchemaKeyNotInObject: Key foo not in schema object at Root#foo`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/bar, referred to from Root#`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/nested.json, referred to from Root#`
- `anchor.json::same $anchor with different base uri (2020-12)` — `SchemaFetchError: Could not fetch schema child1, referred to from Root#`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `SchemaFetchError: Could not fetch schema https://example.com/schema-with-anchor, referred to from Root#/allOf/0`
- `anchor.json::anchor within remote ref (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/locationIndependentIdentifier.json, referred to from Root#`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-ref.json, referred to from Root#`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/anyOf/1`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/anyOf/0`
- `boolean_schema.json::boolean schema 'false' (2020-12)` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#`
- `const.json::const with object (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with array (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with null (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with [false] does not match [0] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with [true] does not match [1] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with {"a": false} does not match {"a": 0} (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `const.json::const with {"a": true} does not match {"a": 1} (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema tree.json, referred to from Root#`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/0`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/1`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-dynamicref.json, referred to from Root#`
- `enum.json::enum with [false] does not match [0] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `enum.json::enum with [true] does not match [1] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `enum.json::enum with [0] does not match [false] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `enum.json::enum with [1] does not match [true] (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `enum.json::empty enum (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `id-legacy.json::base URI change (4)` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/items/items`
- `id-legacy.json::base URI change - change folder (7)` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/items`
- `id-legacy.json::base URI change - change folder (id) (4)` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/items`
- `id-legacy.json::base URI change - change folder in subschema (7)` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/definitions/bar/items`
- `id-legacy.json::base URI change - change folder in subschema (id) (4)` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/definitions/bar/items`
- `id-legacy.json::$ref prevents a sibling id from changing the base uri (4)` — `SchemaFetchError: Could not fetch schema foo.json, referred to from Root#/properties/foo`
- `id-legacy.json::refs with relative uris and defs (id) (4)` — `SchemaKeyNotInObject: Key definitions not in schema object at Root#definitions/inner`
- `id-legacy.json::relative refs with absolute uris and defs (id) (4)` — `SchemaKeyNotInObject: Key definitions not in schema object at Root#definitions/inner`
- `id-legacy.json::id must be resolved against nearest parent, not just immediate parent (4)` — `SchemaFetchError: Could not fetch schema http://example.com/b/d.json, referred to from Root#/properties/foo`
- `id-legacy.json::ref with absolute-path-reference (id) (4)` — `SchemaFetchError: Could not fetch schema /absref/foobar.json, referred to from Root#/properties/foo`
- `id-legacy.json::URN ref with nested pointer ref (7)` — `SchemaKeyNotInObject: Key bar not in schema object at Root#bar`
- … and 42 more

#### 🦀 `compile_fail` (183)

163 distinct first-line errors; most common:

- **2×** `g_076bb24c932b67d2/g_076bb24c932b67d2.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_1115b2c99c539742/g_1115b2c99c539742.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_25adedebc677c818/g_25adedebc677c818.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_32486e353c4617f0/g_32486e353c4617f0.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_33c7956b174a5daa/g_33c7956b174a5daa.go:19:10: invalid receiver type Root (pointer or interface type)`

Per group:

- `allOf.json::allOf simple types (2020-12)` — `g_510fe1d3fa91b2b1/g_510fe1d3fa91b2b1.go:19:10: invalid receiver type Root (pointer or interface type)`
- `allOf.json::allOf with boolean schemas, all true (2020-12)` — `g_f511992bddf00c9c/g_f511992bddf00c9c.go:19:10: invalid receiver type Root (pointer or interface type)`
- `allOf.json::allOf with one empty schema (2020-12)` — `g_eba834605a0a18ea/g_eba834605a0a18ea.go:19:10: invalid receiver type Root (pointer or interface type)`
- `allOf.json::allOf with two empty schemas (2020-12)` — `g_4e1cb26ca1763d1e/g_4e1cb26ca1763d1e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `allOf.json::nested allOf, to check validation semantics (2020-12)` — `g_2086508c092ac78e/g_2086508c092ac78e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `g_26ec0e2647acf8e8/g_26ec0e2647acf8e8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `anyOf.json::anyOf (2020-12)` — `g_f4cb13c854103a3b/g_f4cb13c854103a3b.go:19:10: invalid receiver type Root (pointer or interface type)`
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `g_9cff28eb9e019baa/g_9cff28eb9e019baa.go:19:10: invalid receiver type Root (pointer or interface type)`
- `anyOf.json::anyOf with one empty schema (2020-12)` — `g_128c36a829472d01/g_128c36a829472d01.go:19:10: invalid receiver type Root (pointer or interface type)`
- `anyOf.json::nested anyOf, to check validation semantics (2020-12)` — `g_f7cb680bd00c7876/g_f7cb680bd00c7876.go:19:10: invalid receiver type Root (pointer or interface type)`
- `boolean_schema.json::boolean schema 'true' (2020-12)` — `g_35522a610b175778/g_35522a610b175778.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains keyword validation (2020-12)` — `g_5adc48dbeba149bd/g_5adc48dbeba149bd.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains keyword with const keyword (2020-12)` — `g_c1b64ad63d47d3b2/g_c1b64ad63d47d3b2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains keyword with boolean schema true (2020-12)` — `g_cebfd9a071c703a0/g_cebfd9a071c703a0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains keyword with boolean schema false (2020-12)` — `g_8f270ed8f974179b/g_8f270ed8f974179b.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains with false if subschema (2020-12)` — `g_51b0ed75662b5bdf/g_51b0ed75662b5bdf.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::contains with null instance elements (2020-12)` — `g_232110d594b107b3/g_232110d594b107b3.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains without contains is ignored (2020-12)` — `g_1c47d62b8a07bb8a/g_1c47d62b8a07bb8a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains=1 with contains (2020-12)` — `g_38557bd50bc6b5a0/g_38557bd50bc6b5a0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains=2 with contains (2020-12)` — `g_9af84708e3f81f55/g_9af84708e3f81f55.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains=2 with contains with a decimal value (2020-12)` — `g_c74157afa51f0af0/g_c74157afa51f0af0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains = minContains (2020-12)` — `g_997c6a761041674d/g_997c6a761041674d.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains < minContains (2020-12)` — `g_f0b60be3241a3f2e/g_f0b60be3241a3f2e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains = 0 (2020-12)` — `g_5a01906f18f2dd81/g_5a01906f18f2dd81.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains = 0 with no maxContains (2019-09)` — `g_9ea4dc960593cf26/g_9ea4dc960593cf26.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains = 0 with maxContains (2020-12)` — `g_c8bfb1fa1292adcf/g_c8bfb1fa1292adcf.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains without contains is ignored (2020-12)` — `g_4cf2ff3466d10686/g_4cf2ff3466d10686.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains with contains (2020-12)` — `g_53c09a431cda49ea/g_53c09a431cda49ea.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains with contains, value with a decimal (2020-12)` — `g_7175a7ea16e7ef24/g_7175a7ea16e7ef24.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::minContains < maxContains (2020-12)` — `g_a8e01c6da8aae9c0/g_a8e01c6da8aae9c0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `contains.json::maxContains = 0 with minContains = 0 (2020-12)` — `g_2c34e074e1999a52/g_2c34e074e1999a52.go:19:10: invalid receiver type Root (pointer or interface type)`
- `content.json::validation of string-encoded content based on media type (2020-12)` — `g_6796b82f152935e1/g_6796b82f152935e1.go:19:10: invalid receiver type Root (pointer or interface type)`
- `content.json::validation of binary string-encoding (2020-12)` — `g_f4230cd14834ac2a/g_f4230cd14834ac2a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `content.json::validation of binary-encoded media type documents (2020-12)` — `g_3718a26c686f5de7/g_3718a26c686f5de7.go:19:10: invalid receiver type Root (pointer or interface type)`
- `content.json::validation of binary-encoded media type documents with schema (2020-12)` — `g_aeee99e9c0b8539f/g_aeee99e9c0b8539f.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::single dependency (2020-12)` — `g_57b910d25f9f54fe/g_57b910d25f9f54fe.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::single dependency (dependencies) (7)` — `g_874cf04c90d06c5e/g_874cf04c90d06c5e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::empty dependents (2020-12)` — `g_7a053aee7c272b0d/g_7a053aee7c272b0d.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::empty dependents (dependencies) (7)` — `g_4ea77a7f60620d9d/g_4ea77a7f60620d9d.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::multiple dependents required (2020-12)` — `g_52237d2c3bcfdf1f/g_52237d2c3bcfdf1f.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::multiple dependents required (dependencies) (7)` — `g_47bf2223b6542db0/g_47bf2223b6542db0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::dependencies with escaped characters (2020-12)` — `g_c00333b91f99d285/g_c00333b91f99d285.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentRequired.json::dependencies with escaped characters (dependencies) (7)` — `g_fc5246199dc52ef8/g_fc5246199dc52ef8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::single dependency (2020-12)` — `g_b856017dd3fa3f11/g_b856017dd3fa3f11.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::single dependency (dependencies) (7)` — `g_25bb3ff96815596c/g_25bb3ff96815596c.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::boolean subschemas (2020-12)` — `g_88f7d812ffd36fae/g_88f7d812ffd36fae.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::boolean subschemas (dependencies) (7)` — `g_f92ea6c4a1bf6bce/g_f92ea6c4a1bf6bce.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::dependents with escaped characters (2020-12)` — `g_4a341f7af07f21a7/g_4a341f7af07f21a7.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dependentSchemas.json::dependents with escaped characters (dependencies) (7)` — `g_84600951351a259d/g_84600951351a259d.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (2020) (2020-12)` — `g_e92c8ffc741f45e4/g_e92c8ffc741f45e4.go:19:10: invalid receiver type Root (pointer or interface type)`
- … and 133 more

#### 🤪 `mixed` (8)

5 distinct first-line errors; most common:

- **2×** `accepted '[false, false] from items array is not valid' though valid: false`
- **2×** `accepted 'when if matches and it has unevaluated items' though valid: false`
- **2×** `accepted 'wrong types' though valid: false`
- **1×** `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
- **1×** `valid items: json: cannot unmarshal number into .1 of type string`

Per group:

- `id-legacy.json::$ref prevents a sibling $id from changing the base uri (7)` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `items.json::a schema given for prefixItems (2020-12)` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `items.json::an array of schemas for items (2019-09)` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `items.json::items validation adjusts the starting index for additionalItems (2019-09)` — `valid items: json: cannot unmarshal number into .1 of type string`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `unevaluatedItems.json::unevaluatedItems with if/then/else (prefixItems) (2020-12)` — `accepted 'when if matches and it has unevaluated items' though valid: false`
  - cases: when if matches and it has unevaluated items (accepted), when if doesn't match and it has no unevaluated items (rejected)
- `unevaluatedItems.json::unevaluatedItems with if/then/else (2019-09)` — `accepted 'when if matches and it has unevaluated items' though valid: false`
  - cases: when if matches and it has unevaluated items (accepted), when if doesn't match and it has no unevaluated items (rejected)
- `uniqueItems.json::uniqueItems with an array of items (prefixItems) (2020-12)` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `uniqueItems.json::uniqueItems with an array of items (2019-09)` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)

#### 🚧 `false_reject` (13)

7 distinct first-line errors; most common:

- **4×** `with additional items: json: cannot unmarshal number into .1 of type string`
- **2×** `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
- **2×** `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`
- **2×** `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
- **1×** `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_ba91446bfcf3ecbb.Root`

Per group:

- `enum.json::heterogeneous enum validation (2020-12)` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `items.json::additional items are allowed by default (2020-12)` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `items.json::additional items are allowed by default (2019) (2019-09)` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `recursiveRef.json::$ref with $recursiveAnchor (2019-09)` — `extra items allowed for inner arrays: Union does not contain number`
  - cases: extra items allowed for inner arrays (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_ba91446bfcf3ecbb.Root`
  - cases: a float with zero fractional part is an integer (rejected)
- `unevaluatedItems.json::unevaluatedItems with items and prefixItems (2020-12)` — `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`
  - cases: unevaluatedItems doesn't apply (rejected)
- `unevaluatedItems.json::unevaluatedItems with items and additionalItems (2019-09)` — `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`
  - cases: unevaluatedItems doesn't apply (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested prefixItems and items (2020-12)` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested items and additionalItems (2019-09)` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems (prefixItems) (2020-12)` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems (2019-09)` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `uniqueItems.json::uniqueItems=false with prefixItems (2020-12)` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `uniqueItems.json::uniqueItems=false with an array of items (2019-09)` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)

#### 🙈 `false_accept` (130)

74 distinct first-line errors; most common:

- **14×** `accepted 'with unevaluated items' though valid: false`
- **11×** `accepted 'with unevaluated properties' though valid: false`
- **5×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **4×** `accepted 'always fails' though valid: false`
- **3×** `accepted 'integer does not match as a property value' though valid: false`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `additionalProperties.json::additionalProperties with schema (2020-12)` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `additionalProperties.json::dependentSchemas with additionalProperties (2020-12)` — `accepted 'additionalProperties doesn't consider dependentSchemas' though valid: false`
  - cases: additionalProperties doesn't consider dependentSchemas (accepted), additionalProperties can't see bar (accepted), additionalProperties can't see bar even when foo2 is present (accepted)
- `allOf.json::allOf (2020-12)` — `accepted 'mismatch second' though valid: false`
  - cases: mismatch second (accepted), mismatch first (accepted)
- `allOf.json::allOf with base schema (2020-12)` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch first allOf (accepted), mismatch second allOf (accepted), mismatch both (accepted)
- `anyOf.json::anyOf with base schema (2020-12)` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `const.json::const validation (2020-12)` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted)
- `const.json::const with -2.0 matches integer and float types (2020-12)` — `accepted 'integer 2 is invalid' though valid: false`
  - cases: integer 2 is invalid (accepted), float 2.0 is invalid (accepted), float -2.00001 is invalid (accepted)
- `const.json::float and integers are equal up to 64-bit representation limits (2020-12)` — `accepted 'integer minus one is invalid' though valid: false`
  - cases: integer minus one is invalid (accepted), float minus one is invalid (accepted)
- `const.json::nul characters in strings (2020-12)` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `const.json::characters with the same visual representation but different codepoint (2020-12)` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `const.json::characters with the same visual representation, but different number of codepoints (2020-12)` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `contains.json::items + contains (2020-12)` — `accepted 'matches items, does not match contains' though valid: false`
  - cases: matches items, does not match contains (accepted), does not match items, matches contains (accepted), matches neither items nor contains (accepted)
- `default.json::the default keyword does not do anything if the property is missing (2020-12)` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `dependentSchemas.json::dependent subschema incompatible with root (2020-12)` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `dependentSchemas.json::dependent subschema incompatible with root (dependencies) (7)` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020) (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (2020) (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- `dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `accepted 'An array of strings is invalid' though valid: false`
  - cases: An array of strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `accepted 'The recursive part is not valid against the root' though valid: false`
  - cases: The recursive part is not valid against the root (accepted)
- `dynamicRef.json::$dynamicRef points to a boolean schema (2020-12)` — `accepted 'follow $dynamicRef to a false schema' though valid: false`
  - cases: follow $dynamicRef to a false schema (accepted)
- `enum.json::simple enum validation (2020-12)` — `accepted 'something else is invalid' though valid: false`
  - cases: something else is invalid (accepted)
- `enum.json::enums in properties (2020-12)` — `accepted 'wrong foo value' though valid: false`
  - cases: wrong foo value (accepted), wrong bar value (accepted), missing required property is invalid (accepted), missing all properties is invalid (accepted)
- `enum.json::enum with escaped characters (2020-12)` — `accepted 'another string is invalid' though valid: false`
  - cases: another string is invalid (accepted)
- `enum.json::nul characters in strings (2020-12)` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `enum.json::characters with the same visual representation but different codepoint (2020-12)` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `enum.json::characters with the same visual representation, but different number of codepoints (2020-12)` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `items.json::items and subitems (2020-12)` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong sub-item (accepted)
- `items.json::items and subitems (2019) (2019-09)` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong sub-item (accepted)
- `items.json::items and subitems (7) (7)` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong sub-item (accepted)
- `items.json::prefixItems with no additional items permitted (2020-12)` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `items.json::array of items with no additional items permitted (2019-09)` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `items.json::items does not look in applicators, valid case (2020-12)` — `accepted 'prefixItems in allOf does not constrain items, invalid case' though valid: false`
  - cases: prefixItems in allOf does not constrain items, invalid case (accepted)
- `items.json::additionalItems does not look in applicators, invalid case (2019-09)` — `accepted 'items defined in allOf are not examined' though valid: false`
  - cases: items defined in allOf are not examined (accepted)
- `items.json::prefixItems validation adjusts the starting index for items (2020-12)` — `accepted 'wrong type of second item' though valid: false`
  - cases: wrong type of second item (accepted)
- `items.json::prefixItems with heterogeneous array (2020-12)` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `items.json::additionalItems with heterogeneous array (2019-09)` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `not.json::forbidden property (2020-12)` — `accepted 'property present' though valid: false`
  - cases: property present (accepted)
- `oneOf.json::oneOf with base schema (2020-12)` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `oneOf.json::oneOf complex types (2020-12)` — `accepted 'both oneOf valid (complex)' though valid: false`
  - cases: both oneOf valid (complex) (accepted)
- `oneOf.json::oneOf with required (2020-12)` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `oneOf.json::oneOf with missing optional property (2020-12)` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted), neither oneOf valid (accepted)
- `pattern.json::pattern with Unicode property escape requires unicode mode (2020-12)` — `accepted 'Digits do not match' though valid: false`
  - cases: Digits do not match (accepted)
- `pattern.json::pattern with Unicode property escape (2020-12)` — `accepted 'an invalid case where the string contains digits' though valid: false`
  - cases: an invalid case where the string contains digits (accepted)
- `patternProperties.json::patternProperties with Unicode property escape (2020-12)` — `accepted 'Unicode letter property name matches but value type is incorrect' though valid: false`
  - cases: Unicode letter property name matches but value type is incorrect (accepted)
- … and 80 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 | 17 | 1 | 0 | 0 | 59 | 13 | 0 | 98 | 8.2% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 42 | 77 | 54.5% | 1006 | 4.2% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/bignum.json | 3 | 2 | 1 | 0 | 0 | 8 | 0 | 0 | 14 | 🚧✅✅🦀🦀🦀🦀🦀🦀🙈🙈✅🦀🦀 |
| optional/content.json | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 0 | 0 | 5 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format-regex.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/non-schemas.json | 3 | 0 | 0 | 0 | 0 | 2 | 8 | 0 | 13 | ❌❌❌🦀🦀❌❌❌✅❌✅✅❌ |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **8** | **17** | **1** | **0** | **0** | **59** | **13** | **0** | **98** |  |

### Failures

#### ❌ `generation_error` (13)

7 distinct first-line errors; most common:

- **6×** `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- **2×** `SchemaKeyNotInObject: Key unknown~1keyword not in schema object at Root#unknown~1keyword`
- **1×** `SchemaFetchError: Could not fetch schema bar, referred to from Root#/properties/bar-item`
- **1×** `SchemaFetchError: Could not fetch schema http://localhost:1234/draft2019-09/dependentRequired.json, referred to from Root#/allOf/1`
- **1×** `SchemaFetchError: Could not fetch schema http://localhost:1234/draft2019-09/ignore-prefixItems.json, referred to from Root#`

Per group:

- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft2019-09/dependentRequired.json, referred to from Root#/allOf/1`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft2020-12/prefixItems.json, referred to from Root#`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft7/ignore-dependentRequired.json, referred to from Root#/allOf/1`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft2019-09/ignore-prefixItems.json, referred to from Root#`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `SchemaFetchError: Could not fetch schema bar, referred to from Root#/properties/bar-item`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/$defs) (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/definitions) (7)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::$id inside an enum is not a real identifier (id/definitions) (4)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($anchor) (2020-12)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($id) (7)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier (id) (4)` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `optional/non-schemas.json::reference of a root arbitrary keyword with encoded ref (2020-12)` — `SchemaKeyNotInObject: Key unknown~1keyword not in schema object at Root#unknown~1keyword`
- `optional/non-schemas.json::reference of an arbitrary keyword of a sub-schema with encoded ref (2020-12)` — `SchemaKeyNotInObject: Key unknown~1keyword not in schema object at Root#unknown~1keyword`

#### 🦀 `compile_fail` (59)

49 distinct first-line errors; most common:

- **7×** `g_3a11be8561b69801/g_3a11be8561b69801.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_1115b2c99c539742/g_1115b2c99c539742.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_4f60ad98e6697c1e/g_4f60ad98e6697c1e.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_72d68c138ce6a983/g_72d68c138ce6a983.go:19:10: invalid receiver type Root (pointer or interface type)`
- **2×** `g_8c76d82e50ef5dab/g_8c76d82e50ef5dab.go:19:10: invalid receiver type Root (pointer or interface type)`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `g_5d585201c0b14af8/g_5d585201c0b14af8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-date.json::validation of date strings (2020-12)` — `g_25adedebc677c818/g_25adedebc677c818.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-duration.json::validation of duration strings (2020-12)` — `g_3df3609f4207fdf7/g_3df3609f4207fdf7.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-email.json::validation of e-mail addresses (2020-12)` — `g_4f60ad98e6697c1e/g_4f60ad98e6697c1e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-email.json::validation of e-mail addresses against the RFC 5321 Mailbox rule (2020-12)` — `g_4f60ad98e6697c1e/g_4f60ad98e6697c1e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-hostname.json::validation of host names (2020-12)` — `g_8c76d82e50ef5dab/g_8c76d82e50ef5dab.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-hostname.json::validation of host names without IDNA rules (6)` — `g_5aecdcc0a2a7324c/g_5aecdcc0a2a7324c.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-hostname.json::validation of A-label (punycode) host names (2020-12)` — `g_8c76d82e50ef5dab/g_8c76d82e50ef5dab.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-idn-email.json::validation of an internationalized e-mail addresses (2020-12)` — `g_c575ba72afa0e40b/g_c575ba72afa0e40b.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-idn-hostname.json::validation of internationalized host names (2020-12)` — `g_1115b2c99c539742/g_1115b2c99c539742.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-idn-hostname.json::validation of separators in internationalized host names (2020-12)` — `g_1115b2c99c539742/g_1115b2c99c539742.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-ipv4.json::validation of IP addresses (2020-12)` — `g_942becf6b579e5e6/g_942becf6b579e5e6.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `g_33c7956b174a5daa/g_33c7956b174a5daa.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-iri-reference.json::validation of IRI References (2020-12)` — `g_64e35fc18a341c34/g_64e35fc18a341c34.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-iri.json::validation of IRIs (2020-12)` — `g_32486e353c4617f0/g_32486e353c4617f0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-json-pointer.json::validation of JSON-pointers (JSON String Representation) (2020-12)` — `g_c498b41832ca344c/g_c498b41832ca344c.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-regex.json::validation of regular expressions (2020-12)` — `g_3a11be8561b69801/g_3a11be8561b69801.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-relative-json-pointer.json::validation of Relative JSON Pointers (RJP) (2020-12)` — `g_9a1a1a4e13ce8a87/g_9a1a1a4e13ce8a87.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-time.json::validation of time strings (2020-12)` — `g_72d68c138ce6a983/g_72d68c138ce6a983.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-uri-reference.json::validation of URI References (2020-12)` — `g_deee05d2b4d8aaa5/g_deee05d2b4d8aaa5.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `g_976ccba3e844d16a/g_976ccba3e844d16a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-uri.json::validation of URIs (2020-12)` — `g_076bb24c932b67d2/g_076bb24c932b67d2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-uuid.json::uuid format (2020-12)` — `g_6db80bf5a2a7ae46/g_6db80bf5a2a7ae46.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::maximum integer comparison (2020-12)` — `g_072054d6fc578add/g_072054d6fc578add.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `g_4c7880e25cc0b8a0/g_4c7880e25cc0b8a0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::float comparison with high precision (2020-12)` — `g_84209d3579d62829/g_84209d3579d62829.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::minimum integer comparison (2020-12)` — `g_d6a5a45b55176ab9/g_d6a5a45b55176ab9.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `g_4a06fdcca778b007/g_4a06fdcca778b007.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `g_d1284e9da3459557/g_d1284e9da3459557.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `g_1b40852edf43fbd0/g_1b40852edf43fbd0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/bignum.json::multipleOf with integers above 2^53 (2020-12)` — `g_32a7bde5f9138252/g_32a7bde5f9138252.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/content.json::validation of string-encoded content based on media type (7)` — `g_dbf36a37c4a5b155/g_dbf36a37c4a5b155.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/content.json::validation of binary string-encoding (7)` — `g_9232e8e90ff05ab3/g_9232e8e90ff05ab3.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/content.json::validation of binary-encoded media type documents (7)` — `g_935886b3500759c8/g_935886b3500759c8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::single dependency (2020-12)` — `g_bd3fe00efd5437df/g_bd3fe00efd5437df.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::empty dependents (2020-12)` — `g_50909df55c31dc66/g_50909df55c31dc66.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::multiple dependents required (2020-12)` — `g_58d703606d2a95d5/g_58d703606d2a95d5.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::dependencies with escaped characters (2020-12)` — `g_260ca40ad76f2ec0/g_260ca40ad76f2ec0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::single schema dependency (2020-12)` — `g_8dbc837cdb020172/g_8dbc837cdb020172.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::boolean subschemas (2020-12)` — `g_2e8b1db73883faf1/g_2e8b1db73883faf1.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/dependencies-compatibility.json::schema dependencies with escaped characters (2020-12)` — `g_fcdb302064e2f1b0/g_fcdb302064e2f1b0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `g_199e66ef19a071a0/g_199e66ef19a071a0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/ecmascript-regex.json::\w in patterns matches [A-Za-z0-9_], not unicode letters (2020-12)` — `g_0638645677c5f195/g_0638645677c5f195.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/ecmascript-regex.json::pattern with ASCII ranges (2020-12)` — `g_bc389f8b6b09c343/g_bc389f8b6b09c343.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/ecmascript-regex.json::\d in pattern matches [0-9], not unicode digits (2020-12)` — `g_8ee3c6dcd82b02c2/g_8ee3c6dcd82b02c2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/ecmascript-regex.json::pattern with non-ASCII digits (2020-12)` — `g_e95117e6b4523a38/g_e95117e6b4523a38.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `g_6223e79c2a23b5b7/g_6223e79c2a23b5b7.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true (2020-12)` — `g_19ced816249b7445/g_19ced816249b7445.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `g_3a11be8561b69801/g_3a11be8561b69801.go:19:10: invalid receiver type Root (pointer or interface type)`
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `g_3a11be8561b69801/g_3a11be8561b69801.go:19:10: invalid receiver type Root (pointer or interface type)`
- … and 9 more

#### 🚧 `false_reject` (1)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_ba91446bfcf3ecbb.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)

#### 🙈 `false_accept` (17)

12 distinct first-line errors; most common:

- **3×** `accepted 'does not match' though valid: false`
- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'literal unicode character in json string' though valid: false`
- **1×** `accepted 'ASCII 'a' does not match' though valid: false`

Per group:

- `optional/bignum.json::enum comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::const comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/ecmascript-regex.json::ECMA 262 regex $ does not match trailing newline (2020-12)` — `accepted 'matches in Python, but not in ECMA 262' though valid: false`
  - cases: matches in Python, but not in ECMA 262 (accepted)
- `optional/ecmascript-regex.json::ECMA 262 regex converts \t to horizontal tab (2020-12)` — `accepted 'does not match' though valid: false`
  - cases: does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter (2020-12)` — `accepted 'does not match' though valid: false`
  - cases: does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter (2020-12)` — `accepted 'does not match' though valid: false`
  - cases: does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \d matches ascii digits only (2020-12)` — `accepted 'NKO DIGIT ZERO does not match (unlike e.g. Python)' though valid: false`
  - cases: NKO DIGIT ZERO does not match (unlike e.g. Python) (accepted), NKO DIGIT ZERO (as \u escape) does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \D matches everything but ascii digits (2020-12)` — `accepted 'ASCII zero does not match' though valid: false`
  - cases: ASCII zero does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \w matches ascii letters only (2020-12)` — `accepted 'latin-1 e-acute does not match (unlike e.g. Python)' though valid: false`
  - cases: latin-1 e-acute does not match (unlike e.g. Python) (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \W matches everything but ascii letters (2020-12)` — `accepted 'ASCII 'a' does not match' though valid: false`
  - cases: ASCII 'a' does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \s matches whitespace (2020-12)` — `accepted 'Non-whitespace control does not match' though valid: false`
  - cases: Non-whitespace control does not match (accepted), Non-whitespace does not match (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \S matches everything but whitespace (2020-12)` — `accepted 'ASCII space does not match' though valid: false`
  - cases: ASCII space does not match (accepted), Character tabulation does not match (accepted), Line tabulation does not match (accepted), Form feed does not match (accepted), latin-1 non-breaking-space does not match (accepted), zero-width whitespace does not match (accepted), … and 3 more
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

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 21 | 34 | 61.8% | 34 | 61.8% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| **Total** | **0** | **5** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

#### 🙈 `false_accept` (5)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads inline (2020-12)` — `accepted 'circle and square together match both branches, invalid' though valid: false`
  - cases: circle and square together match both branches, invalid (accepted), neither variant key is present, invalid (accepted)
- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads by $ref (2020-12)` — `accepted 'circle and square together match both branches, invalid' though valid: false`
  - cases: circle and square together match both branches, invalid (accepted), neither variant key is present, invalid (accepted)
- `compound/externally-tagged-union.json::oneOf of objects with one required key each and additionalProperties false (2020-12)` — `accepted 'circle plus an unrelated key is invalid' though valid: false`
  - cases: circle plus an unrelated key is invalid (accepted), circle and square together is invalid (accepted), an empty object is invalid (accepted)
- `compound/externally-tagged-union.json::oneOf mixing a unit variant and a data variant (2020-12)` — `accepted 'the unit variant in map form with null is invalid' though valid: false`
  - cases: the unit variant in map form with null is invalid (accepted), the unit variant in map form with an empty object is invalid (accepted), the data variant with a null payload is invalid (accepted), the data variant with an extra key is invalid (accepted), the data variant's name as a bare string is invalid (accepted)
- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a non-member string is invalid' though valid: false`
  - cases: a non-member string is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9 | 0 | 0 | 0 | 9 | 6 | 0 | 25 | 4.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 9 | 22 | 40.9% | 90 | 10.0% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 1 | 4 | 0 | 0 | 0 | 2 | 5 | 0 | 12 | 🙈🙈🙈✅🦀❌❌❌❌❌🙈🦀 |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/additionalProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| proposals/propertyDependencies/propertyDependencies.json | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| unevaluatedProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **1** | **9** | **0** | **0** | **0** | **9** | **6** | **0** | **25** |  |

### Failures

#### ❌ `generation_error` (6)

6 distinct first-line errors; most common:

- **1×** `SchemaFetchError: Could not fetch schema bar, referred to from Root#/properties/bar-item`
- **1×** `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#`
- **1×** `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/0`
- **1×** `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/1`
- **1×** `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-dynamicref.json, referred to from Root#`

Per group:

- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `SchemaFetchError: Could not fetch schema tree.json, referred to from Root#`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/0`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `SchemaFetchError: Could not fetch schema extendible-dynamic-ref.json, referred to from Root#/allOf/1`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `SchemaFetchError: Could not fetch schema http://localhost:1234/detached-dynamicref.json, referred to from Root#`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `SchemaFetchError: Could not fetch schema bar, referred to from Root#/properties/bar-item`

#### 🦀 `compile_fail` (9)

8 distinct first-line errors; most common:

- **2×** `g_132a59baa35cf167/g_132a59baa35cf167.go:19:10: invalid receiver type Root (pointer or interface type)`
- **1×** `g_403f9e0dcf6797a2/g_403f9e0dcf6797a2.go:19:10: invalid receiver type Root (pointer or interface type)`
- **1×** `g_89a26f4b879c6e19/g_89a26f4b879c6e19.go:19:10: invalid receiver type Root (pointer or interface type)`
- **1×** `g_90cef3e3f7ccdd10/g_90cef3e3f7ccdd10.go:19:10: invalid receiver type Root (pointer or interface type)`
- **1×** `g_9ad388d59bec58ff/g_9ad388d59bec58ff.go:19:10: invalid receiver type Root (pointer or interface type)`

Per group:

- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `g_f75e19005b73833b/g_f75e19005b73833b.go:19:10: invalid receiver type Root (pointer or interface type)`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `g_e727756a6ba6108e/g_e727756a6ba6108e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `g_403f9e0dcf6797a2/g_403f9e0dcf6797a2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `format-time.json::leap seconds are not valid times (v1)` — `g_90cef3e3f7ccdd10/g_90cef3e3f7ccdd10.go:19:10: invalid receiver type Root (pointer or interface type)`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `g_d7587ef36ccc09a2/g_d7587ef36ccc09a2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `g_89a26f4b879c6e19/g_89a26f4b879c6e19.go:19:10: invalid receiver type Root (pointer or interface type)`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-objects (v1)` — `g_132a59baa35cf167/g_132a59baa35cf167.go:19:10: invalid receiver type Root (pointer or interface type)`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-string property values (v1)` — `g_132a59baa35cf167/g_132a59baa35cf167.go:19:10: invalid receiver type Root (pointer or interface type)`
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `g_9ad388d59bec58ff/g_9ad388d59bec58ff.go:19:10: invalid receiver type Root (pointer or interface type)`

#### 🙈 `false_accept` (9)

7 distinct first-line errors; most common:

- **3×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **1×** `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
- **1×** `accepted 'disallows bar if foo != foo1' though valid: false`
- **1×** `accepted 'string property fails' though valid: false`
- **1×** `accepted 'unevaluatedProperties doesn't consider propertyDependencies' though valid: false`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
  - cases: additionalProperties doesn't consider propertyDependencies properties (accepted), additionalProperties can't see buz even when foo2 is present (accepted), additionalProperties can't see buz (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `accepted 'disallows bar if foo != foo1' though valid: false`
  - cases: disallows bar if foo != foo1 (accepted), disallows bar if foo is absent (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `accepted 'unevaluatedProperties doesn't consider propertyDependencies' though valid: false`
  - cases: unevaluatedProperties doesn't consider propertyDependencies (accepted), unevaluatedProperties doesn't see buz when foo2 is absent (accepted)
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `accepted 'with unevaluated items' though valid: false`
  - cases: with unevaluated items (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)

