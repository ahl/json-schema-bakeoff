# quicktype 26.0.0 rust

Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

_generated 2026-11-22T02:30:55Z_

**193 of 540 standard groups pass (35.7%)**

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
| 193 | 218 | 17 | 7 | 0 | 0 | 92 | 13 | 540 | 35.7% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 1007 | 1418 | 71.0% | 1664 | 60.5% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 6 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 12 | ✅🙈🙈✅❌❌✅✅✅✅🙈🙈 |
| anchor-legacy.json | 3 | 0 | 0 | 0 | 0 | 0 | 9 | 2 | 14 | ✅❌✅❌✅❌🤯🤯❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 1 | 9 | ❌❌❌❌❌🤯❌❌❌ |
| anyOf.json | 3 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈✅❌❌✅✅🙈 |
| boolean_schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ✅❌ |
| const.json | 7 | 3 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈✅✅✅ |
| contains.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈🙈🙈 |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 4 | 12 | 0 | 0 | 0 | 0 | 5 | 0 | 21 | 🙈🙈🙈✅🙈❌❌❌❌❌🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈 |
| enum.json | 9 | 1 | 2 | 0 | 0 | 0 | 5 | 0 | 17 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅❌✅✅ |
| format-date-time.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-date.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-duration.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-email.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-idn-email.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-idn-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-ipv4.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-ipv6.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-iri-reference.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-iri.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-json-pointer.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-relative-json-pointer.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-time.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format-uri-reference.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-uri-template.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-uri.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-uuid.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| id-legacy.json | 16 | 0 | 0 | 1 | 0 | 0 | 17 | 6 | 40 | ❌❌❌❌❌🤪❌✅❌✅❌✅❌🤯🤯🤯✅✅✅✅✅✅✅✅🤯🤯✅❌✅✅✅✅❌🤯❌❌❌❌❌❌ |
| id.json | 11 | 1 | 0 | 0 | 0 | 0 | 9 | 4 | 25 | ❌❌❌✅✅✅🙈❌✅🤯🤯🤯✅✅✅✅🤯✅✅✅❌❌❌❌❌ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 10 | 10 | 2 | 3 | 0 | 0 | 3 | 0 | 28 | ✅🤪🤪✅❌❌❌🙈🙈🙈✅✅✅✅✅✅🙈🙈✅🚧🚧🙈🙈🙈🤪🙈🙈✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| multipleOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 0 | 8 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | 🙈🙈🙈❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| patternProperties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈🙈🙈🙈🙈🙈🚧 |
| ref-legacy.json | 6 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 14 | ✅❌✅✅✅❌✅❌❌✅❌❌❌❌ |
| ref.json | 10 | 3 | 0 | 0 | 0 | 0 | 10 | 0 | 23 | 🙈✅✅❌✅🙈✅✅❌✅❌❌❌🙈✅✅✅✅❌❌❌❌❌ |
| required.json | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 🤪✅✅🙈🙈 |
| type.json | 9 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 12 | ✅🚧✅✅✅✅✅🙈✅✅✅🚧 |
| unevaluatedItems.json | 7 | 32 | 8 | 0 | 0 | 0 | 0 | 0 | 47 | ✅🙈🙈✅🙈🙈🚧🚧✅🙈🙈🙈🙈🙈🚧🚧🚧🚧🙈🙈🙈🙈✅✅🚧🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈 |
| unevaluatedProperties.json | 9 | 36 | 0 | 0 | 0 | 0 | 0 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 3 | 3 | 2 | 2 | 0 | 0 | 0 | 0 | 10 | 🙈✅🤪🤪🙈🙈🚧🚧✅✅ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 4 | ❌✅❌✅ |
| **Total** | **193** | **218** | **17** | **7** | **0** | **0** | **92** | **13** | **540** |  |

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

#### 🤪 `mixed` (7)

5 distinct first-line errors; most common:

- **2×** `accepted '[false, false] from items array is not valid' though valid: false`
- **2×** `accepted 'wrong types' though valid: false`
- **1×** `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
- **1×** `accepted 'non-present required property is invalid' though valid: false`
- **1×** `valid items: data did not match any variant of untagged enum Root`

Per group:

- `id-legacy.json::$ref prevents a sibling $id from changing the base uri (7)` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `items.json::a schema given for prefixItems (2020-12)` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `items.json::an array of schemas for items (2019-09)` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `items.json::items validation adjusts the starting index for additionalItems (2019-09)` — `valid items: data did not match any variant of untagged enum Root`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `required.json::required validation (2020-12)` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `uniqueItems.json::uniqueItems with an array of items (prefixItems) (2020-12)` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `uniqueItems.json::uniqueItems with an array of items (2019-09)` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)

#### 🚧 `false_reject` (17)

9 distinct first-line errors; most common:

- **4×** `with additional items: data did not match any variant of untagged enum Root`
- **2×** `null is valid: data did not match any variant of untagged enum Root`
- **2×** `only the first item is validated: data did not match any variant of untagged enum Root`
- **2×** `unevaluatedItems doesn't apply: data did not match any variant of untagged enum Root`
- **2×** `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`

Per group:

- `enum.json::heterogeneous enum validation (2020-12)` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `enum.json::heterogeneous enum-with-null validation (2020-12)` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `items.json::additional items are allowed by default (2020-12)` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `items.json::additional items are allowed by default (2019) (2019-09)` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `recursiveRef.json::$ref with $recursiveAnchor (2019-09)` — `extra items allowed for inner arrays: data did not match any variant of untagged enum RootElement`
  - cases: extra items allowed for inner arrays (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `type.json::type: array, object or null (2020-12)` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `unevaluatedItems.json::unevaluatedItems with items and prefixItems (2020-12)` — `unevaluatedItems doesn't apply: data did not match any variant of untagged enum Root`
  - cases: unevaluatedItems doesn't apply (rejected)
- `unevaluatedItems.json::unevaluatedItems with items and additionalItems (2019-09)` — `unevaluatedItems doesn't apply: data did not match any variant of untagged enum Root`
  - cases: unevaluatedItems doesn't apply (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested prefixItems and items (2020-12)` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested items and additionalItems (2019-09)` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems (prefixItems) (2020-12)` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems (2019-09)` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with if/then/else (prefixItems) (2020-12)` — `when if matches and it has no unevaluated items: data did not match any variant of untagged enum Root`
  - cases: when if matches and it has no unevaluated items (rejected), when if doesn't match and it has no unevaluated items (rejected)
- `unevaluatedItems.json::unevaluatedItems with if/then/else (2019-09)` — `when if matches and it has no unevaluated items: data did not match any variant of untagged enum Root`
  - cases: when if matches and it has no unevaluated items (rejected), when if doesn't match and it has no unevaluated items (rejected)
- `uniqueItems.json::uniqueItems=false with prefixItems (2020-12)` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `uniqueItems.json::uniqueItems=false with an array of items (2019-09)` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)

#### 🙈 `false_accept` (218)

128 distinct first-line errors; most common:

- **14×** `accepted 'with unevaluated items' though valid: false`
- **11×** `accepted 'with unevaluated properties' though valid: false`
- **6×** `accepted 'too long is invalid' though valid: false`
- **6×** `accepted 'too short is invalid' though valid: false`
- **5×** `accepted 'An array containing non-strings is invalid' though valid: false`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `additionalProperties.json::additionalProperties with schema (2020-12)` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `additionalProperties.json::dependentSchemas with additionalProperties (2020-12)` — `accepted 'additionalProperties doesn't consider dependentSchemas' though valid: false`
  - cases: additionalProperties doesn't consider dependentSchemas (accepted), additionalProperties can't see bar (accepted), additionalProperties can't see bar even when foo2 is present (accepted)
- `allOf.json::allOf with base schema (2020-12)` — `accepted 'mismatch second allOf' though valid: false`
  - cases: mismatch second allOf (accepted)
- `allOf.json::allOf simple types (2020-12)` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `allOf.json::nested allOf, to check validation semantics (2020-12)` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `anyOf.json::anyOf (2020-12)` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `anyOf.json::anyOf with base schema (2020-12)` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `anyOf.json::nested anyOf, to check validation semantics (2020-12)` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `const.json::const validation (2020-12)` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted)
- `const.json::const with -2.0 matches integer and float types (2020-12)` — `accepted 'integer 2 is invalid' though valid: false`
  - cases: integer 2 is invalid (accepted), float 2.0 is invalid (accepted), float -2.00001 is invalid (accepted)
- `const.json::float and integers are equal up to 64-bit representation limits (2020-12)` — `accepted 'integer minus one is invalid' though valid: false`
  - cases: integer minus one is invalid (accepted), float minus one is invalid (accepted)
- `contains.json::contains keyword validation (2020-12)` — `accepted 'array without items matching schema is invalid' though valid: false`
  - cases: array without items matching schema is invalid (accepted), empty array is invalid (accepted)
- `contains.json::contains keyword with const keyword (2020-12)` — `accepted 'array without item 5 is invalid' though valid: false`
  - cases: array without item 5 is invalid (accepted)
- `contains.json::contains keyword with boolean schema true (2020-12)` — `accepted 'empty array is invalid' though valid: false`
  - cases: empty array is invalid (accepted)
- `contains.json::contains keyword with boolean schema false (2020-12)` — `accepted 'any non-empty array is invalid' though valid: false`
  - cases: any non-empty array is invalid (accepted), empty array is invalid (accepted)
- `contains.json::items + contains (2020-12)` — `accepted 'matches items, does not match contains' though valid: false`
  - cases: matches items, does not match contains (accepted), does not match items, matches contains (accepted), matches neither items nor contains (accepted)
- `contains.json::contains with false if subschema (2020-12)` — `accepted 'empty array is invalid' though valid: false`
  - cases: empty array is invalid (accepted)
- `contains.json::minContains=1 with contains (2020-12)` — `accepted 'empty data' though valid: false`
  - cases: empty data (accepted), no elements match (accepted)
- `contains.json::minContains=2 with contains (2020-12)` — `accepted 'empty data' though valid: false`
  - cases: empty data (accepted), all elements match, invalid minContains (accepted), some elements match, invalid minContains (accepted)
- `contains.json::minContains=2 with contains with a decimal value (2020-12)` — `accepted 'one element matches, invalid minContains' though valid: false`
  - cases: one element matches, invalid minContains (accepted)
- `contains.json::maxContains = minContains (2020-12)` — `accepted 'empty data' though valid: false`
  - cases: empty data (accepted), all elements match, invalid minContains (accepted), all elements match, invalid maxContains (accepted)
- `contains.json::maxContains < minContains (2020-12)` — `accepted 'empty data' though valid: false`
  - cases: empty data (accepted), invalid minContains (accepted), invalid maxContains (accepted), invalid maxContains and minContains (accepted)
- `contains.json::minContains = 0 with maxContains (2020-12)` — `accepted 'too many' though valid: false`
  - cases: too many (accepted)
- `contains.json::maxContains with contains (2020-12)` — `accepted 'empty data' though valid: false`
  - cases: empty data (accepted), all elements match, invalid maxContains (accepted), some elements match, invalid maxContains (accepted)
- `contains.json::maxContains with contains, value with a decimal (2020-12)` — `accepted 'too many elements match, invalid maxContains' though valid: false`
  - cases: too many elements match, invalid maxContains (accepted)
- `contains.json::minContains < maxContains (2020-12)` — `accepted 'actual < minContains < maxContains' though valid: false`
  - cases: actual < minContains < maxContains (accepted), minContains < maxContains < actual (accepted)
- `contains.json::maxContains = 0 with minContains = 0 (2020-12)` — `accepted 'one matching item' though valid: false`
  - cases: one matching item (accepted)
- `default.json::the default keyword does not do anything if the property is missing (2020-12)` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `dependentRequired.json::single dependency (2020-12)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `dependentRequired.json::single dependency (dependencies) (7)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `dependentRequired.json::multiple dependents required (2020-12)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `dependentRequired.json::multiple dependents required (dependencies) (7)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `dependentRequired.json::dependencies with escaped characters (2020-12)` — `accepted 'CRLF missing dependent' though valid: false`
  - cases: CRLF missing dependent (accepted), quoted quotes missing dependent (accepted)
- `dependentRequired.json::dependencies with escaped characters (dependencies) (7)` — `accepted 'CRLF missing dependent' though valid: false`
  - cases: CRLF missing dependent (accepted), quoted quotes missing dependent (accepted)
- `dependentSchemas.json::single dependency (2020-12)` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `dependentSchemas.json::single dependency (dependencies) (7)` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `dependentSchemas.json::boolean subschemas (2020-12)` — `accepted 'object with property having schema false is invalid' though valid: false`
  - cases: object with property having schema false is invalid (accepted), object with both properties is invalid (accepted)
- `dependentSchemas.json::boolean subschemas (dependencies) (7)` — `accepted 'object with property having schema false is invalid' though valid: false`
  - cases: object with property having schema false is invalid (accepted), object with both properties is invalid (accepted)
- `dependentSchemas.json::dependents with escaped characters (2020-12)` — `accepted 'quoted quote' though valid: false`
  - cases: quoted quote (accepted), quoted tab invalid under dependent schema (accepted), quoted quote invalid under dependent schema (accepted)
- `dependentSchemas.json::dependents with escaped characters (dependencies) (7)` — `accepted 'invalid object 1' though valid: false`
  - cases: invalid object 1 (accepted), invalid object 2 (accepted), invalid object 3 (accepted), invalid object 4 (accepted)
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
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (2020) (2020-12)` — `accepted 'string matches /$defs/thingy, but the $dynamicRef does not stop here' though valid: false`
  - cases: string matches /$defs/thingy, but the $dynamicRef does not stop here (accepted), first_scope is not in dynamic scope for the $dynamicRef (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- … and 168 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 13 | 71 | 1 | 0 | 0 | 0 | 13 | 0 | 98 | 13.3% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 395 | 973 | 40.6% | 1006 | 39.3% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| format-date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-date.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-duration.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-email.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| format-hostname.json | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅🙈 |
| format-idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| format-ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uuid.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/bignum.json | 3 | 10 | 1 | 0 | 0 | 0 | 0 | 0 | 14 | 🚧✅✅🙈🙈🙈🙈🙈🙈🙈🙈✅🙈🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/non-schemas.json | 3 | 2 | 0 | 0 | 0 | 0 | 8 | 0 | 13 | ❌❌❌🙈🙈❌❌❌✅❌✅✅❌ |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **13** | **71** | **1** | **0** | **0** | **0** | **13** | **0** | **98** |  |

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

#### 🚧 `false_reject` (1)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)

#### 🙈 `false_accept` (71)

56 distinct first-line errors; most common:

- **4×** `accepted 'ascii non-digits' though valid: false`
- **4×** `accepted 'literal unicode character in json string' though valid: false`
- **3×** `accepted 'does not match' though valid: false`
- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'comparison works for high numbers' though valid: false`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid day in date-time string (accepted), an invalid offset in date-time string (accepted), an invalid closing Z after time-zone offset (accepted), … and 24 more
- `format-date.json::validation of date strings (2020-12)` — `accepted 'a invalid date string with 32 days in January' though valid: false`
  - cases: a invalid date string with 32 days in January (accepted), a invalid date string with 30 days in February (leap) (accepted), a invalid date string with 32 days in March (accepted), a invalid date string with 31 days in April (accepted), a invalid date string with 32 days in May (accepted), a invalid date string with 31 days in June (accepted), … and 52 more
- `format-duration.json::validation of duration strings (2020-12)` — `accepted 'an invalid duration string' though valid: false`
  - cases: an invalid duration string (accepted), must start with P (accepted), no elements present (accepted), no time elements present (accepted), no date or time elements present (accepted), elements out of order (accepted), … and 19 more
- `format-email.json::validation of e-mail addresses (2020-12)` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), two email addresses is not valid (accepted), full "From" header is invalid (accepted), … and 26 more
- `format-email.json::validation of e-mail addresses against the RFC 5321 Mailbox rule (2020-12)` — `accepted 'a parenthesis in an unquoted local part is not valid' though valid: false`
  - cases: a parenthesis in an unquoted local part is not valid (accepted), a parenthesis in the domain is not valid (accepted), bracketed content that is not an address is not valid (accepted), empty brackets after the @ are not valid (accepted), an IPv4-address-literal with three octets is not valid (accepted), an IPv4-address-literal with five octets is not valid (accepted), … and 6 more
- `format-hostname.json::validation of host names (2020-12)` — `accepted 'empty string' though valid: false`
  - cases: empty string (accepted), single dot (accepted), leading dot (accepted), trailing dot (accepted), IDN label separator (accepted), starts with hyphen (accepted), … and 8 more
- `format-hostname.json::validation of A-label (punycode) host names (2020-12)` — `accepted 'invalid Punycode' though valid: false`
  - cases: invalid Punycode (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), Begins with a Spacing Combining Mark (accepted), Begins with a Nonspacing Mark (accepted), Begins with an Enclosing Mark (accepted), Exceptions that are DISALLOWED, right-to-left chars (accepted), … and 17 more
- `format-idn-email.json::validation of an internationalized e-mail addresses (2020-12)` — `accepted 'an invalid e-mail/idn e-mail address' though valid: false`
  - cases: an invalid e-mail/idn e-mail address (accepted), a fullwidth commercial at is not a local-part separator (accepted), a domain label starting with a hyphen is not valid (accepted), a domain label ending with a hyphen is not valid (accepted), a domain starting with a dot is not valid (accepted), a domain ending with a dot is not valid (accepted), … and 2 more
- `format-idn-hostname.json::validation of internationalized host names (2020-12)` — `accepted 'illegal first char U+302E Hangul single dot tone mark' though valid: false`
  - cases: illegal first char U+302E Hangul single dot tone mark (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), a single label of 64 characters is too long (accepted), invalid label, correct Punycode (accepted), invalid Punycode (accepted), U-label contains "--" in the 3rd and 4th position (accepted), … and 40 more
- `format-idn-hostname.json::validation of separators in internationalized host names (2020-12)` — `accepted 'single dot' though valid: false`
  - cases: single dot (accepted), single ideographic full stop (accepted), single fullwidth full stop (accepted), single halfwidth ideographic full stop (accepted), leading dot (accepted), leading ideographic full stop (accepted), … and 6 more
- `format-ipv4.json::validation of IP addresses (2020-12)` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `accepted 'an IPv6 address with out-of-range values' though valid: false`
  - cases: an IPv6 address with out-of-range values (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `format-iri-reference.json::validation of IRI References (2020-12)` — `accepted 'an invalid IRI Reference' though valid: false`
  - cases: an invalid IRI Reference (accepted), an invalid IRI fragment (accepted), an embedded IPv4 with a leading zero is invalid (accepted), a trailing newline after a valid IRI Reference is invalid (accepted), invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), … and 4 more
- `format-iri.json::validation of IRIs (2020-12)` — `accepted 'invalid percent-encoding with non-hex digits' though valid: false`
  - cases: invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), lone percent sign is invalid (accepted), an invalid IRI based on IPv6 (accepted), an invalid relative IRI Reference (accepted), an invalid IRI (accepted), … and 9 more
- `format-json-pointer.json::validation of JSON-pointers (JSON String Representation) (2020-12)` — `accepted 'not a valid JSON-pointer (~ not escaped)' though valid: false`
  - cases: not a valid JSON-pointer (~ not escaped) (accepted), not a valid JSON-pointer (URI Fragment Identifier) #1 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #2 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #3 (accepted), not a valid JSON-pointer (some escaped, but not all) #1 (accepted), not a valid JSON-pointer (some escaped, but not all) #2 (accepted), … and 6 more
- `format-regex.json::validation of regular expressions (2020-12)` — `accepted 'a regular expression with unclosed parens is invalid' though valid: false`
  - cases: a regular expression with unclosed parens is invalid (accepted), a quantifier with nothing to repeat is invalid (accepted), a character class range in reverse order is invalid (accepted), a trailing backslash is invalid (accepted), an unmatched closing parenthesis is invalid (accepted), an unterminated character class is invalid (accepted)
- `format-relative-json-pointer.json::validation of Relative JSON Pointers (RJP) (2020-12)` — `accepted 'an invalid RJP that is a valid JSON Pointer' though valid: false`
  - cases: an invalid RJP that is a valid JSON Pointer (accepted), a json-pointer part that does not start with a slash is invalid (accepted), negative prefix (accepted), explicit positive prefix (accepted), non-ASCII digit in the prefix is not allowed (accepted), ## is not a valid json-pointer (accepted), … and 8 more
- `format-time.json::validation of time strings (2020-12)` — `accepted 'invalid time string with extra leading zeros' though valid: false`
  - cases: invalid time string with extra leading zeros (accepted), invalid time string with no leading zero for single digit (accepted), hour, minute, second must be two digits (accepted), hour, minute in time-offset must be two digits (accepted), an invalid time string with invalid hour (accepted), an invalid time string with invalid minute (accepted), … and 22 more
- `format-uri-reference.json::validation of URI References (2020-12)` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `accepted 'an invalid uri-template' though valid: false`
  - cases: an invalid uri-template (accepted), an empty expression is invalid (accepted), an empty varspec inside the variable list is invalid (accepted), a zero prefix max-length is invalid (accepted), a five-digit prefix max-length is invalid (accepted), an unmatched closing brace is invalid (accepted), … and 15 more
- `format-uri.json::validation of URIs (2020-12)` — `accepted 'an invalid protocol-relative URI Reference' though valid: false`
  - cases: an invalid protocol-relative URI Reference (accepted), an invalid relative URI Reference (accepted), an invalid URI (accepted), an invalid URI though valid URI reference (accepted), an invalid URI with spaces (accepted), an invalid URI with spaces and missing scheme (accepted), … and 21 more
- `format-uuid.json::uuid format (2020-12)` — `accepted 'wrong length' though valid: false`
  - cases: wrong length (accepted), missing section (accepted), bad characters (not hex) (accepted), no dashes (accepted), too few dashes (accepted), too many dashes (accepted), … and 8 more
- `optional/bignum.json::maximum integer comparison (2020-12)` — `accepted 'just above the maximum is invalid' though valid: false`
  - cases: just above the maximum is invalid (accepted)
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `accepted 'comparison works for high numbers' though valid: false`
  - cases: comparison works for high numbers (accepted)
- `optional/bignum.json::float comparison with high precision (2020-12)` — `accepted 'comparison works for high numbers' though valid: false`
  - cases: comparison works for high numbers (accepted)
- `optional/bignum.json::minimum integer comparison (2020-12)` — `accepted 'just below the minimum is invalid' though valid: false`
  - cases: just below the minimum is invalid (accepted)
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `accepted 'comparison works for very negative numbers' though valid: false`
  - cases: comparison works for very negative numbers (accepted)
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `accepted 'comparison works for very negative numbers' though valid: false`
  - cases: comparison works for very negative numbers (accepted)
- `optional/bignum.json::enum comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::const comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `accepted 'the same high-precision decimal is not unique' though valid: false`
  - cases: the same high-precision decimal is not unique (accepted), trailing zeroes do not make decimals unique (accepted), arrays containing the same decimal are not unique (accepted), objects containing the same decimal are not unique (accepted)
- `optional/bignum.json::multipleOf with integers above 2^53 (2020-12)` — `accepted '1e20 is not multiple of 3' though valid: false`
  - cases: 1e20 is not multiple of 3 (accepted)
- `optional/content.json::validation of string-encoded content based on media type (7)` — `accepted 'an invalid JSON document' though valid: false`
  - cases: an invalid JSON document (accepted)
- `optional/content.json::validation of binary string-encoding (7)` — `accepted 'an invalid base64 string (% is not a valid character)' though valid: false`
  - cases: an invalid base64 string (% is not a valid character) (accepted)
- `optional/content.json::validation of binary-encoded media type documents (7)` — `accepted 'a validly-encoded invalid JSON document' though valid: false`
  - cases: a validly-encoded invalid JSON document (accepted), an invalid base64 string that is valid JSON (accepted)
- `optional/dependencies-compatibility.json::single dependency (2020-12)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `optional/dependencies-compatibility.json::multiple dependents required (2020-12)` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `optional/dependencies-compatibility.json::dependencies with escaped characters (2020-12)` — `accepted 'CRLF missing dependent' though valid: false`
  - cases: CRLF missing dependent (accepted), quoted quotes missing dependent (accepted)
- `optional/dependencies-compatibility.json::single schema dependency (2020-12)` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `optional/dependencies-compatibility.json::boolean subschemas (2020-12)` — `accepted 'object with property having schema false is invalid' though valid: false`
  - cases: object with property having schema false is invalid (accepted), object with both properties is invalid (accepted)
- `optional/dependencies-compatibility.json::schema dependencies with escaped characters (2020-12)` — `accepted 'quoted quote' though valid: false`
  - cases: quoted quote (accepted), quoted tab invalid under dependent schema (accepted), quoted quote invalid under dependent schema (accepted)
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
- … and 21 more

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 24 | 34 | 70.6% | 34 | 70.6% |

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
  - cases: the unit variant in map form with null is invalid (accepted), the data variant with an extra key is invalid (accepted)
- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a member in map form with null is invalid' though valid: false`
  - cases: a member in map form with null is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | 16 | 0 | 0 | 0 | 0 | 6 | 0 | 25 | 12.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 40 | 75 | 53.3% | 90 | 44.4% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 1 | 6 | 0 | 0 | 0 | 0 | 5 | 0 | 12 | 🙈🙈🙈✅🙈❌❌❌❌❌🙈🙈 |
| format-date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/additionalProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| proposals/propertyDependencies/propertyDependencies.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| unevaluatedProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **3** | **16** | **0** | **0** | **0** | **0** | **6** | **0** | **25** |  |

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

#### 🙈 `false_accept` (16)

14 distinct first-line errors; most common:

- **3×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **1×** `accepted 'a leap second is not a valid time' though valid: false`
- **1×** `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
- **1×** `accepted 'an invalid date-time-ext with an empty suffix' though valid: false`
- **1×** `accepted 'bar with more than 2 properties is invalid' though valid: false`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `accepted 'string matches /$defs/thingy, but the $dynamicRef does not stop here' though valid: false`
  - cases: string matches /$defs/thingy, but the $dynamicRef does not stop here (accepted), first_scope is not in dynamic scope for the $dynamicRef (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `accepted 'data is not sufficient for schema at second#/$defs/length' though valid: false`
  - cases: data is not sufficient for schema at second#/$defs/length (accepted)
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `accepted 'an invalid date-time-ext with an empty suffix' though valid: false`
  - cases: an invalid date-time-ext with an empty suffix (accepted), an invalid date-time-ext with an unclosed time-zone suffix (accepted), an invalid date-time-ext with a 15 character time-zone-part (accepted), an invalid date-time-ext with a "." time-zone-part (accepted), an invalid date-time-ext with an uppercase suffix-key (accepted), an invalid date-time-ext with two time-zone suffixes (accepted), … and 2 more
- `format-time.json::leap seconds are not valid times (v1)` — `accepted 'a leap second is not a valid time' though valid: false`
  - cases: a leap second is not a valid time (accepted), a leap second with a positive time-offset is not a valid time (accepted), a leap second with a negative time-offset is not a valid time (accepted)
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
  - cases: additionalProperties doesn't consider propertyDependencies properties (accepted), additionalProperties can't see buz even when foo2 is present (accepted), additionalProperties can't see buz (accepted)
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `accepted 'number list with string values' though valid: false`
  - cases: number list with string values (accepted), string list with number values (accepted)
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `accepted 'expected strings - additional property as not string is invalid' though valid: false`
  - cases: expected strings - additional property as not string is invalid (accepted), expected integers - additional property as not integer is invalid (accepted)
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `accepted 'bar with more than 2 properties is invalid' though valid: false`
  - cases: bar with more than 2 properties is invalid (accepted), bar with fewer than 2 properties is invalid (accepted), baz with other properties is invalid (accepted), quux is disallowed (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `accepted 'disallows bar if foo != foo1' though valid: false`
  - cases: disallows bar if foo != foo1 (accepted), disallows bar if foo is absent (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `accepted 'unevaluatedProperties doesn't consider propertyDependencies' though valid: false`
  - cases: unevaluatedProperties doesn't consider propertyDependencies (accepted), unevaluatedProperties doesn't see buz when foo2 is absent (accepted)
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `accepted 'with unevaluated items' though valid: false`
  - cases: with unevaluated items (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)

