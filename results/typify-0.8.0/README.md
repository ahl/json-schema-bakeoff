# typify 0.8.0

The last typify release before the typespace rewrite. Same design center as
0.10; it's here so the rewrite has something to be measured against.

_generated 2026-11-22T01:57:36Z_

**133 of 540 standard groups pass (24.6%)**

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
| 133 | 99 | 31 | 8 | 0 | 2 | 62 | 205 | 540 | 24.6% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 664 | 925 | 71.8% | 1664 | 39.9% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 8 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧✅✅✅✅✅✅✅✅ |
| allOf.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 12 | ✅✅🤯✅✅✅✅✅✅✅✅🤯 |
| anchor-legacy.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 14 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 9 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| anyOf.json | 1 | 1 | 3 | 0 | 0 | 0 | 0 | 3 | 8 | 🤯🚧🚧🤯🤯🙈🚧✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 2 | 0 | 0 | 0 | 0 | 0 | 19 | 0 | 21 | ❌❌❌❌❌❌❌✅❌❌❌❌❌❌❌❌✅❌❌❌❌ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | ❌✅🙈 |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 18 | 21 | 🙈🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🙈🤯🤯🤯🤯🤯🤯🤯🙈 |
| enum.json | 13 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 17 | ✅🤯✅✅🤯✅✅✅✅✅🚧✅🚧✅✅✅✅ |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| id-legacy.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 28 | 40 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯✅✅✅✅✅✅✅✅🤯🤯🤯🤯✅✅✅✅🤯🤯🤯🤯🤯🤯🤯🤯 |
| id.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | 25 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯✅✅✅✅🤯🤯✅✅🤯🤯🤯🤯🤯 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 12 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 2 | 6 | 0 | 0 | 0 | 10 | 1 | 28 | 🚧🙈❌✅✅🙈❌🚧❌❌✅✅✅❌✅✅🚧❌🚧✅❌🤯❌🚧❌🚧❌✅ |
| maxItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| maxLength.json | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🚧❌ |
| maxProperties.json | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 3 | 🤪❌🙈 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 3 | 5 | 🤯🤯❌❌🤯 |
| minItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| minLength.json | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🚧❌ |
| minProperties.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 3 | 5 | 🤯❌❌🤯🤯 |
| multipleOf.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 6 | 🤯🤯🤯🤯✅✅ |
| not.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 9 | 🤯🤯🤯✅✅✅✅🤯🤯 |
| oneOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 5 | 11 | 🤯🙈🤯🤯🤯🤯✅🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 🚧✅🚧🚧 |
| patternProperties.json | 1 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 6 | 🚧🙈🙈🙈✅🚧 |
| properties.json | 1 | 1 | 2 | 1 | 0 | 1 | 0 | 0 | 6 | 🚧🤪🙈🦀✅🚧 |
| propertyNames.json | 3 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 6 | 🚧✅✅🤯🙈✅ |
| recursiveRef.json | 0 | 3 | 1 | 3 | 0 | 0 | 1 | 2 | 10 | 🙈🤪🚧🤪🤪🙈🙈🤯🤯❌ |
| ref-legacy.json | 6 | 1 | 0 | 0 | 0 | 0 | 1 | 6 | 14 | ❌🤯✅✅✅✅✅✅🤯🙈🤯🤯🤯🤯 |
| ref.json | 6 | 4 | 0 | 0 | 0 | 0 | 0 | 13 | 23 | 🤯🤯🙈🤯✅🙈✅✅✅✅✅🤯🤯🙈🤯🤯🤯🙈🤯🤯🤯🤯🤯 |
| required.json | 2 | 0 | 2 | 0 | 0 | 1 | 0 | 0 | 5 | 🚧✅✅🦀🚧 |
| type.json | 11 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅🚧✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 8 | 13 | 3 | 0 | 0 | 0 | 16 | 7 | 47 | ✅🙈🙈✅🙈❌✅❌✅🙈🙈❌🙈🚧✅❌✅❌🚧❌🙈❌🤯❌🤯🤯🙈🙈❌🙈❌🤯❌🙈❌🙈❌❌🚧🤯❌✅✅🤯🤯🙈❌ |
| unevaluatedProperties.json | 10 | 28 | 0 | 0 | 0 | 0 | 0 | 7 | 45 | ✅🙈🙈🙈✅✅✅🙈🤯✅✅🙈🙈🙈🤯🤯🤯🙈🙈🙈🙈🤯🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🤯🙈🙈✅✅🙈🤯🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 2 | 2 | 0 | 0 | 0 | 4 | 0 | 10 | 🙈✅🙈❌🚧❌✅❌🚧❌ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 4 | 🤯✅🤯✅ |
| **Total** | **133** | **99** | **31** | **8** | **0** | **2** | **62** | **205** | **540** |  |

### Failures

#### 🤯 `generation_panic` (205)

65 distinct first-line errors; most common:

- **63×** `not yet implemented: invalid (or unexpected) schema:`
- **27×** `not implemented: if/then/else schemas are not supported: {`
- **6×** `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- **6×** `not yet implemented: unhandled not schema Object(`
- **5×** `external references are not supported: folderInteger.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`

Per group:

- `allOf.json::allOf simple types (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `assertion failed: any_of.is_none() || one_of.is_none() at <registry>/typify-impl-0.8.0/src/merge.rs:312`
- `anchor-legacy.json::Location-independent identifier (7)` — `expected a '/' in $ref: #foo at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `expected a '/' in $ref: #foo at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `external references are not supported: http://localhost:1234/nested.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `external references are not supported: http://localhost:1234/nested.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `external references are not supported: https://example.com/schema-with-anchor#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `external references are not supported: https://example.com/schema-with-anchor#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `external references are not supported: urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `external references are not supported: urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::anchor within remote ref (7)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#/definitions/refToInteger at <registry>/typify-impl-0.8.0/src/co…`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#/definitions/refToInteger at <registry>/typify-impl-0.8.0/src/co…`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `external references are not supported: http://localhost:1234/detached-ref.json#/definitions/foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `external references are not supported: http://localhost:1234/detached-ref.json#/definitions/foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::Location-independent identifier (2020-12)` — `expected a '/' in $ref: #foo at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `external references are not supported: http://localhost:1234/bar#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `external references are not supported: http://localhost:1234/nested.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::same $anchor with different base uri (2020-12)` — `external references are not supported: child1#my_anchor at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `external references are not supported: https://example.com/schema-with-anchor#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `external references are not supported: urn:uuid:deadbeef-1234-ff00-00ff-4321feebdaed#something at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::anchor within remote ref (2020-12)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `external references are not supported: http://localhost:1234/locationIndependentIdentifier.json#/$defs/refToInteger at <registry>/typify-impl-0.8.0/src/convert.…`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `external references are not supported: http://localhost:1234/detached-ref.json#/$defs/foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `anyOf.json::anyOf (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `internal error: entered unreachable code at <registry>/typify-impl-0.8.0/src/enums.rs:81`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `not yet implemented at <registry>/typify-impl-0.8.0/src/enums.rs:85`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (2020) (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (2020) (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (2020) (2020-12)` — `not implemented: if/then/else schemas are not supported: {`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (2020) (2020-12)` — `external references are not supported: tree.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `external references are not supported: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `expected a '/' in $ref: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `expected a '/' in $ref: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `external references are not supported: http://localhost:1234/detached-dynamicref.json#/$defs/foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `external references are not supported: item at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (2020) (2020-12)` — `external references are not supported: first#/$defs/stuff at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `expected a '/' in $ref: #items at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `external references are not supported: bar at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor (2020-12)` — `external references are not supported: bar at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (2020) (2020-12)` — `external references are not supported: genericList at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `enum.json::heterogeneous enum validation (2020-12)` — `multiple implied types for an un-typed enum {Boolean, Object, Array, Number, String} [Number(6), String("foo"), Array [], Bool(true), Object {"foo": Number(12)}…`
- `enum.json::enum with escaped characters (2020-12)` — `Failed to make unique variant names for [foo`
- `format-date-time.json::validation of non-string date-time values (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-date-time.json::date-time annotation (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- … and 155 more

#### ❌ `generation_error` (62)

12 distinct first-line errors; most common:

- **42×** `invalid schema for Root: unhandled array validation ArrayValidation {`
- **4×** `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- **2×** `invalid schema for Root_Variant0: unhandled array validation ArrayValidation {`
- **2×** `invalid schema for bar: unhandled array validation ArrayValidation {`
- **2×** `invalid schema for item: unhandled array validation ArrayValidation {`

Per group:

- `contains.json::contains keyword validation (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::contains keyword with const keyword (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::contains keyword with boolean schema true (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::contains keyword with boolean schema false (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::items + contains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::contains with false if subschema (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::contains with null instance elements (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains=1 with contains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains=2 with contains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains=2 with contains with a decimal value (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::maxContains = minContains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::maxContains < minContains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains = 0 (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains = 0 with no maxContains (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains = 0 with maxContains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::maxContains with contains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::maxContains with contains, value with a decimal (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::minContains < maxContains (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `contains.json::maxContains = 0 with minContains = 0 (2020-12)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `default.json::invalid type for default (2020-12)` — `value does not conform to the given schema`
- `items.json::an array of schemas for items (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::items as array with boolean schemas (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::items and subitems (2019) (2019-09)` — `invalid schema for item: unhandled array validation ArrayValidation {`
- `items.json::items and subitems (7) (7)` — `invalid schema for item: unhandled array validation ArrayValidation {`
- `items.json::array-form items with null instance elements (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::array of items with no additional items permitted (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::additional items are allowed by default (2019) (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::additionalItems does not look in applicators, invalid case (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::items validation adjusts the starting index for additionalItems (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `items.json::additionalItems with heterogeneous array (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `maxItems.json::maxItems validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `maxLength.json::maxLength validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `maxProperties.json::maxProperties validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `maximum.json::maximum validation (explicit false exclusivity) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'false', expected f64`
- `maximum.json::exclusiveMaximum validation (boolean form) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `minItems.json::minItems validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `minLength.json::minLength validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `minProperties.json::minProperties validation with a decimal (2020-12)` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `minimum.json::minimum validation (explicit false exclusivity) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'false', expected f64`
- `minimum.json::exclusiveMinimum validation (boolean form) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `recursiveRef.json::$ref with $recursiveAnchor (2019-09)` — `invalid schema for /schemas/unevaluated-items-are-allowed: unhandled array validation ArrayValidation {`
- `ref-legacy.json::relative pointer ref to array (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with tuple (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with items and additionalItems (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with nested tuple (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with nested items and additionalItems (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with anyOf (2019-09)` — `invalid schema for Root_Variant0: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with oneOf (2019-09)` — `invalid schema for Root_Variant0: unhandled array validation ArrayValidation {`
- `unevaluatedItems.json::unevaluatedItems with not (2019-09)` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- … and 12 more

#### 🦀 `compile_fail` (2)

- `properties.json::properties with escaped characters (2020-12)` — `error[E0124]: field 'foo_bar' is already declared`
- `required.json::required with escaped characters (2020-12)` — `error[E0124]: field 'foo_bar' is already declared`

#### 🤪 `mixed` (8)

4 distinct first-line errors; most common:

- **3×** `integer matches at the outer level: invalid type: integer '1', expected struct Root`
- **2×** `accepted 'too long is invalid' though valid: false`
- **2×** `accepted 'too short is invalid' though valid: false`
- **1×** `accepted 'property invalidates property' though valid: false`

Per group:

- `maxItems.json::maxItems validation (2020-12)` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `maxProperties.json::maxProperties validation (2020-12)` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `minItems.json::minItems validation (2020-12)` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `minProperties.json::minProperties validation (2020-12)` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `properties.json::properties, patternProperties, additionalProperties interaction (2020-12)` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `recursiveRef.json::$recursiveRef without using nesting (2019-09)` — `integer matches at the outer level: invalid type: integer '1', expected struct Root`
  - cases: integer matches at the outer level (rejected), integer does not match as a property value (accepted), two levels, no match (accepted)
- `recursiveRef.json::$recursiveRef with $recursiveAnchor: false works like $ref (2019-09)` — `integer matches at the outer level: invalid type: integer '1', expected struct Root`
  - cases: integer matches at the outer level (rejected), integer does not match as a property value (accepted), two levels, integer does not match as a property value (accepted)
- `recursiveRef.json::$recursiveRef with no $recursiveAnchor works like $ref (2019-09)` — `integer matches at the outer level: invalid type: integer '1', expected struct Root`
  - cases: integer matches at the outer level (rejected), integer does not match as a property value (accepted), two levels, integer does not match as a property value (accepted)

#### 🚧 `false_reject` (31)

26 distinct first-line errors; most common:

- **2×** `[false, true] from items array is valid: invalid type: boolean 'false', expected string or map`
- **2×** `ignores arrays: invalid length 0, expected struct Root with 2 elements`
- **2×** `ignores arrays: invalid length 0, expected struct Root with 3 elements`
- **2×** `ignores arrays: invalid type: sequence, expected a map`
- **2×** `ignores non-arrays: invalid type: map, expected a sequence`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `anyOf.json::anyOf with base schema (2020-12)` — `one anyOf valid: invalid type: string "foobar", expected struct Root`
  - cases: one anyOf valid (rejected)
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `any value is valid: invalid type: string "foo", expected struct Root`
  - cases: any value is valid (rejected)
- `anyOf.json::anyOf with one empty schema (2020-12)` — `string is valid: invalid type: string "foo", expected struct Root`
  - cases: string is valid (rejected), number is valid (rejected)
- `enum.json::enum with [0] does not match [false] (2020-12)` — `[0.0] is valid: invalid value`
  - cases: [0.0] is valid (rejected)
- `enum.json::enum with [1] does not match [true] (2020-12)` — `[1.0] is valid: invalid value`
  - cases: [1.0] is valid (rejected)
- `items.json::a schema given for items (2020-12)` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `items.json::items and subitems (2020-12)` — `valid items: invalid type: sequence, expected string or map`
  - cases: valid items (rejected), fewer items is valid (rejected)
- `items.json::prefixItems with no additional items permitted (2020-12)` — `fewer number of items present (1): invalid type: integer '1', expected string or map`
  - cases: fewer number of items present (1) (rejected), fewer number of items present (2) (rejected), equal number of items present (rejected)
- `items.json::additionalItems as false without items (2019-09)` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected)
- `items.json::prefixItems validation adjusts the starting index for items (2020-12)` — `valid items: invalid type: string "x", expected i64`
  - cases: valid items (rejected)
- `items.json::prefixItems with heterogeneous array (2020-12)` — `valid instance: invalid type: null, expected string or map`
  - cases: valid instance (rejected)
- `maxLength.json::maxLength validation (2020-12)` — `ignores non-strings: invalid type: integer '100', expected a string`
  - cases: ignores non-strings (rejected)
- `minLength.json::minLength validation (2020-12)` — `ignores non-strings: invalid type: integer '1', expected a string`
  - cases: ignores non-strings (rejected)
- `pattern.json::pattern validation (2020-12)` — `ignores booleans: invalid type: boolean 'true', expected a string`
  - cases: ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), ignores null (rejected)
- `pattern.json::pattern with Unicode property escape requires unicode mode (2020-12)` — `ASCII letters match: doesn't match pattern "^\p{Letter}+$"`
  - cases: ASCII letters match (rejected), Non-ASCII letters match (rejected)
- `pattern.json::pattern with Unicode property escape (2020-12)` — `a valid case with a Unicode letter: doesn't match pattern "^\p{Letter}+$"`
  - cases: a valid case with a Unicode letter (rejected), another valid case with multiple Unicode letters (rejected)
- `patternProperties.json::patternProperties validates properties matching a regex (2020-12)` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `patternProperties.json::patternProperties with Unicode property escape (2020-12)` — `Unicode letter property name matches: doesn't match pattern "^\p{Letter}+$"`
  - cases: Unicode letter property name matches (rejected), Non-letter property name does not match pattern (rejected)
- `properties.json::object properties validation (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `properties.json::properties whose names are Javascript object property names (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `propertyNames.json::propertyNames validation (2020-12)` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `recursiveRef.json::$recursiveRef with nesting (2019-09)` — `integer matches at the outer level: invalid type: integer '1', expected struct Root`
  - cases: integer matches at the outer level (rejected)
- `required.json::required validation (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `required.json::required properties whose names are Javascript object property names (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `unevaluatedItems.json::unevaluatedItems with nested items (2020-12)` — `with only (valid) additional items: invalid type: sequence, expected struct Root`
  - cases: with only (valid) additional items (rejected), with no additional items (rejected)
- `unevaluatedItems.json::unevaluatedItems with anyOf (prefixItems) (2020-12)` — `when one schema matches and has no unevaluated items: invalid type: sequence, expected struct Root`
  - cases: when one schema matches and has no unevaluated items (rejected), when two schemas match and has no unevaluated items (rejected)
- `unevaluatedItems.json::unevaluatedItems depends on multiple nested contains (2020-12)` — `5 not evaluated, passes unevaluatedItems: invalid type: sequence, expected string or map`
  - cases: 5 not evaluated, passes unevaluatedItems (rejected)
- `uniqueItems.json::uniqueItems with prefixItems and items=false (2020-12)` — `[false, true] from items array is valid: invalid type: boolean 'false', expected string or map`
  - cases: [false, true] from items array is valid (rejected), [true, false] from items array is valid (rejected)
- `uniqueItems.json::uniqueItems=false with prefixItems and items=false (2020-12)` — `[false, true] from items array is valid: invalid type: boolean 'false', expected string or map`
  - cases: [false, true] from items array is valid (rejected), [true, false] from items array is valid (rejected), [false, false] from items array is valid (rejected), [true, true] from items array is valid (rejected)

#### 🙈 `false_accept` (99)

65 distinct first-line errors; most common:

- **9×** `accepted 'with unevaluated properties' though valid: false`
- **7×** `accepted 'with unevaluated items' though valid: false`
- **4×** `accepted 'missing dependency' though valid: false`
- **3×** `accepted 'always fails' though valid: false`
- **3×** `accepted 'with an unevaluated property that exists at another location' though valid: false`

Per group:

- `anyOf.json::anyOf complex types (2020-12)` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `const.json::const validation (2020-12)` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted), another type is invalid (accepted)
- `const.json::const with object (2020-12)` — `accepted 'another object is invalid' though valid: false`
  - cases: another object is invalid (accepted), another type is invalid (accepted)
- `const.json::const with array (2020-12)` — `accepted 'another array item is invalid' though valid: false`
  - cases: another array item is invalid (accepted), array with additional items is invalid (accepted)
- `const.json::const with null (2020-12)` — `accepted 'not null is invalid' though valid: false`
  - cases: not null is invalid (accepted)
- `const.json::const with false does not match 0 (2020-12)` — `accepted 'integer zero is invalid' though valid: false`
  - cases: integer zero is invalid (accepted), float zero is invalid (accepted)
- `const.json::const with true does not match 1 (2020-12)` — `accepted 'integer one is invalid' though valid: false`
  - cases: integer one is invalid (accepted), float one is invalid (accepted)
- `const.json::const with [false] does not match [0] (2020-12)` — `accepted '[0] is invalid' though valid: false`
  - cases: [0] is invalid (accepted), [0.0] is invalid (accepted)
- `const.json::const with [true] does not match [1] (2020-12)` — `accepted '[1] is invalid' though valid: false`
  - cases: [1] is invalid (accepted), [1.0] is invalid (accepted)
- `const.json::const with {"a": false} does not match {"a": 0} (2020-12)` — `accepted '{"a": 0} is invalid' though valid: false`
  - cases: {"a": 0} is invalid (accepted), {"a": 0.0} is invalid (accepted)
- `const.json::const with {"a": true} does not match {"a": 1} (2020-12)` — `accepted '{"a": 1} is invalid' though valid: false`
  - cases: {"a": 1} is invalid (accepted), {"a": 1.0} is invalid (accepted)
- `const.json::const with 0 does not match other zero-like types (2020-12)` — `accepted 'false is invalid' though valid: false`
  - cases: false is invalid (accepted), empty object is invalid (accepted), empty array is invalid (accepted), empty string is invalid (accepted)
- `const.json::const with 1 does not match true (2020-12)` — `accepted 'true is invalid' though valid: false`
  - cases: true is invalid (accepted)
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
- `dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::$dynamicRef points to a boolean schema (2020-12)` — `accepted 'follow $dynamicRef to a false schema' though valid: false`
  - cases: follow $dynamicRef to a false schema (accepted)
- `items.json::a schema given for prefixItems (2020-12)` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted)
- `items.json::prefixItems with boolean schemas (2020-12)` — `accepted 'array with two items is invalid' though valid: false`
  - cases: array with two items is invalid (accepted)
- `maxProperties.json::maxProperties = 0 means the object is empty (2020-12)` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `oneOf.json::oneOf with base schema (2020-12)` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `oneOf.json::oneOf with empty schema (2020-12)` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `oneOf.json::oneOf with required (2020-12)` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `oneOf.json::oneOf with missing optional property (2020-12)` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `patternProperties.json::multiple simultaneous patternProperties are validated (2020-12)` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `patternProperties.json::regexes are not anchored by default and are case sensitive (2020-12)` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `patternProperties.json::patternProperties with boolean schemas (2020-12)` — `accepted 'object with property matching schema false is invalid' though valid: false`
  - cases: object with property matching schema false is invalid (accepted), object with both properties is invalid (accepted), object with a property matching both true and false is invalid (accepted)
- `properties.json::properties with boolean schema (2020-12)` — `accepted 'only 'false' property present is invalid' though valid: false`
  - cases: only 'false' property present is invalid (accepted), both properties present is invalid (accepted)
- `propertyNames.json::propertyNames with const (2020-12)` — `accepted 'object with any other property is invalid' though valid: false`
  - cases: object with any other property is invalid (accepted)
- `recursiveRef.json::$recursiveRef without $recursiveAnchor works like $ref (2019-09)` — `accepted 'recursive mismatch' though valid: false`
  - cases: recursive mismatch (accepted)
- `recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial target schema resource (2019-09)` — `accepted 'leaf node does not match: recursion uses the inner schema' though valid: false`
  - cases: leaf node does not match: recursion uses the inner schema (accepted)
- … and 49 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 24 | 12 | 5 | 0 | 0 | 0 | 2 | 55 | 98 | 24.5% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 115 | 153 | 75.2% | 1006 | 11.4% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 🤯🤯🤯 |
| format-idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/bignum.json | 3 | 3 | 1 | 0 | 0 | 0 | 2 | 5 | 14 | 🚧✅✅🤯❌🤯🤯❌🤯🙈🙈✅🙈🤯 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 4 | 🤯🤯🤯🤯 |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/ecmascript-regex.json | 16 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🚧✅✅✅🚧🚧✅✅✅🚧 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| optional/format-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 6 | 🤯🤯🤯🤯🤯🤯 |
| optional/format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/non-schemas.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 13 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **24** | **12** | **5** | **0** | **0** | **0** | **2** | **55** | **98** |  |

### Failures

#### 🤯 `generation_panic` (55)

13 distinct first-line errors; most common:

- **37×** `not yet implemented: invalid (or unexpected) schema:`
- **3×** `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/util.rs:588`
- **3×** `expected a '/' in $ref: #my_anchor at <registry>/typify-impl-0.8.0/src/util.rs:565`
- **2×** `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/convert.rs:394`
- **2×** `external references are not supported: https://localhost:1234/unknownKeyword/my_identifier.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-date.json::validation of date strings (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-duration.json::validation of duration strings (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-email.json::validation of e-mail addresses (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-email.json::validation of e-mail addresses against the RFC 5321 Mailbox rule (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-hostname.json::validation of host names (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-hostname.json::validation of host names without IDNA rules (6)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-hostname.json::validation of A-label (punycode) host names (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-idn-email.json::validation of an internationalized e-mail addresses (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-idn-hostname.json::validation of internationalized host names (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-idn-hostname.json::validation of separators in internationalized host names (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-ipv4.json::validation of IP addresses (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-iri-reference.json::validation of IRI References (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-iri.json::validation of IRIs (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-json-pointer.json::validation of JSON-pointers (JSON String Representation) (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-regex.json::validation of regular expressions (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-relative-json-pointer.json::validation of Relative JSON Pointers (RJP) (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-time.json::validation of time strings (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-uri-reference.json::validation of URI References (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-uri.json::validation of URIs (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-uuid.json::uuid format (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/bignum.json::maximum integer comparison (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/bignum.json::float comparison with high precision (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/bignum.json::minimum integer comparison (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/bignum.json::multipleOf with integers above 2^53 (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `unresolved reference: http://localhost:1234/draft2019-09/dependentRequired.json at <registry>/typify-impl-0.8.0/src/merge.rs:125`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/convert.rs:394`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `unresolved reference: http://localhost:1234/draft7/ignore-dependentRequired.json at <registry>/typify-impl-0.8.0/src/merge.rs:125`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/convert.rs:394`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `external references are not supported: item at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::global inline flag groups are not valid ECMA 262 (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::ECMA 262 named groups and backreferences are valid (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::ECMA 262 lookbehind is valid, including variable width (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-regex.json::ECMA 262 character classes and escapes (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/format-time.json::validation of time strings (2020-12)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/$defs) (2020-12)` — `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/util.rs:588`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/definitions) (7)` — `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/util.rs:588`
- `optional/non-schemas.json::$id inside an enum is not a real identifier (id/definitions) (4)` — `called 'Option::unwrap()' on a 'None' value at <registry>/typify-impl-0.8.0/src/util.rs:588`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier ($defs) (2020-12)` — `external references are not supported: https://localhost:1234/unknownKeyword/my_identifier.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier (7)` — `external references are not supported: https://localhost:1234/unknownKeyword/my_identifier.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($anchor) (2020-12)` — `expected a '/' in $ref: #my_anchor at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($id) (7)` — `expected a '/' in $ref: #my_anchor at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier (id) (4)` — `expected a '/' in $ref: #my_anchor at <registry>/typify-impl-0.8.0/src/util.rs:565`
- … and 5 more

#### ❌ `generation_error` (2)

- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`

#### 🚧 `false_reject` (5)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `optional/ecmascript-regex.json::pattern with non-ASCII digits (2020-12)` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties (2020-12)` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `optional/ecmascript-regex.json::patternProperties with non-ASCII digits (2020-12)` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)

#### 🙈 `false_accept` (12)

10 distinct first-line errors; most common:

- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'missing dependency' though valid: false`
- **1×** `accepted 'CRLF missing dependent' though valid: false`
- **1×** `accepted 'a validly-encoded invalid JSON document' though valid: false`
- **1×** `accepted 'an invalid JSON document' though valid: false`

Per group:

- `optional/bignum.json::enum comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::const comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `accepted 'the same high-precision decimal is not unique' though valid: false`
  - cases: the same high-precision decimal is not unique (accepted), trailing zeroes do not make decimals unique (accepted), arrays containing the same decimal are not unique (accepted), objects containing the same decimal are not unique (accepted)
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

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 5 | 20.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 28 | 34 | 82.4% | 34 | 82.4% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 1 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 5 | 🚧🚧✅🙈🙈 |
| **Total** | **1** | **2** | **2** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

#### 🚧 `false_reject` (2)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads inline (2020-12)` — `circle plus an unrelated key is valid: invalid value: map, expected map with a single key`
  - cases: circle plus an unrelated key is valid (rejected), an unrelated key before the variant key is valid (rejected)
- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads by $ref (2020-12)` — `circle plus an unrelated key is valid: invalid value: map, expected map with a single key`
  - cases: circle plus an unrelated key is valid (rejected), an unrelated key before the variant key is valid (rejected)

#### 🙈 `false_accept` (2)

- `compound/externally-tagged-union.json::oneOf mixing a unit variant and a data variant (2020-12)` — `accepted 'the unit variant in map form with null is invalid' though valid: false`
  - cases: the unit variant in map form with null is invalid (accepted)
- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a member in map form with null is invalid' though valid: false`
  - cases: a member in map form with null is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | 5 | 0 | 0 | 0 | 0 | 0 | 17 | 25 | 12.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 25 | 36 | 69.4% | 90 | 27.8% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 11 | 12 | 🙈🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| proposals/propertyDependencies/additionalProperties.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| proposals/propertyDependencies/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 2 | 🤯🙈 |
| proposals/propertyDependencies/propertyDependencies.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| **Total** | **3** | **5** | **0** | **0** | **0** | **0** | **0** | **17** | **25** |  |

### Failures

#### 🤯 `generation_panic` (17)

11 distinct first-line errors; most common:

- **3×** `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- **2×** `expected a '/' in $ref: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/util.rs:565`
- **2×** `external references are not supported: ./baseSchema at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- **2×** `external references are not supported: item at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- **2×** `not yet implemented: invalid (or unexpected) schema:`

Per group:

- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `external references are not supported: list at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `external references are not supported: inner_scope at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `external references are not supported: tree.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `external references are not supported: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `expected a '/' in $ref: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `expected a '/' in $ref: extendible-dynamic-ref.json at <registry>/typify-impl-0.8.0/src/util.rs:565`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `external references are not supported: http://localhost:1234/detached-dynamicref.json#/$defs/foo at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `external references are not supported: item at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `external references are not supported: first#/$defs/stuff at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `not yet implemented: invalid (or unexpected) schema:`
- `format-time.json::leap seconds are not valid times (v1)` — `not yet implemented: invalid (or unexpected) schema:`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `external references are not supported: item at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `external references are not supported: genericList at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `external references are not supported: ./baseSchema at <registry>/typify-impl-0.8.0/src/convert.rs:1357`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `external references are not supported: ./baseSchema at <registry>/typify-impl-0.8.0/src/convert.rs:1357`

#### 🙈 `false_accept` (5)

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `accepted 'expected strings - additional property as not string is invalid' though valid: false`
  - cases: expected strings - additional property as not string is invalid (accepted), expected integers - additional property as not integer is invalid (accepted)
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `accepted 'bar with more than 2 properties is invalid' though valid: false`
  - cases: bar with more than 2 properties is invalid (accepted), bar with fewer than 2 properties is invalid (accepted), baz with other properties is invalid (accepted), quux is disallowed (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `accepted 'disallows bar if foo != foo1' though valid: false`
  - cases: disallows bar if foo != foo1 (accepted), disallows bar if foo is absent (accepted)
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `accepted 'unevaluatedProperties doesn't consider propertyDependencies' though valid: false`
  - cases: unevaluatedProperties doesn't consider propertyDependencies (accepted), unevaluatedProperties doesn't see buz when foo2 is absent (accepted)

