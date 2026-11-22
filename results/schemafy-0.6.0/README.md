# schemafy 0.6.0

The original Rust JSON Schema → serde generator, usually used through the
`schemafy!` macro. Its design center is a hand-written draft-4 schema you own:
walk `properties` and `definitions`, name a struct or alias for each, and
get out of the way. It models structure only, so validation keywords vanish
and anything that isn't a plain object graph panics. The last release
(0.6.0) is from October 2021.

_generated 2026-11-22T02:41:27Z_

**126 of 540 standard groups pass (23.3%)**

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
| 126 | 205 | 3 | 3 | 0 | 43 | 53 | 107 | 540 | 23.3% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 778 | 1149 | 67.7% | 1664 | 46.8% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 6 | 0 | 1 | 0 | 0 | 0 | 0 | 9 | 🤪🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🙈🙈❌❌❌✅✅🙈🙈🙈🙈 |
| anchor-legacy.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 9 | 14 | 🤯🦀🤯🦀🤯🦀🦀🦀🤯🤯🤯🤯🤯🤯 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 8 | 9 | 🤯🤯🤯🤯🤯🦀🤯🤯🤯 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 0 | 3 | 0 | 8 | 🙈🙈❌❌❌🙈✅🙈 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈🙈🙈 |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 0 | 5 | 0 | 0 | 0 | 2 | 0 | 14 | 21 | 🙈🤯🤯🤯🙈🤯🤯🤯🤯🤯🦀🤯🙈🦀🤯🤯🤯🤯🤯🙈🙈 |
| enum.json | 3 | 2 | 0 | 0 | 0 | 1 | 0 | 11 | 17 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅🙈✅✅ |
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
| id-legacy.json | 12 | 3 | 0 | 0 | 0 | 11 | 0 | 14 | 40 | 🙈🦀🦀🦀🦀🤯🦀🤯🤯🤯🤯🤯🦀🦀🦀🦀✅✅✅✅✅✅✅✅✅✅🤯🦀🙈✅🙈✅🤯🦀🤯🤯🤯🤯🤯🤯 |
| id.json | 0 | 1 | 0 | 0 | 0 | 11 | 0 | 13 | 25 | 🙈🦀🦀🤯🤯🤯🤯🤯🤯🦀🦀🦀🦀🦀🦀🦀🦀🤯🤯🤯🤯🦀🤯🤯🤯 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 9 | 12 | 0 | 0 | 0 | 1 | 6 | 0 | 28 | 🙈🙈🙈❌❌🙈❌❌🦀🙈✅✅✅✅🙈✅❌🙈✅✅✅🙈🙈🙈🙈❌🙈✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈🙈❌ |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈🙈❌ |
| multipleOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈🙈🙈🙈❌❌✅❌ |
| oneOf.json | 0 | 7 | 0 | 0 | 0 | 0 | 4 | 0 | 11 | 🙈🙈❌❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈🙈❌✅🙈 |
| properties.json | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 0 | 6 | 🚧🙈❌🦀✅🤪 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 5 | 10 | 🙈🤯🤯🤯🤯🙈🙈🙈🙈🤯 |
| ref-legacy.json | 3 | 1 | 0 | 0 | 0 | 3 | 2 | 5 | 14 | 🙈🦀✅✅✅🤯❌❌🦀🤯🤯🤯🦀🤯 |
| ref.json | 1 | 1 | 0 | 1 | 0 | 6 | 0 | 14 | 23 | 🤪🦀🙈🦀🤯🦀✅🦀🤯🤯🤯🤯🦀🤯🤯🤯🤯🤯🤯🤯🤯🦀🤯 |
| required.json | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🙈 |
| type.json | 7 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅🚧✅✅✅✅✅🙈✅🙈🙈🙈 |
| unevaluatedItems.json | 8 | 22 | 0 | 0 | 0 | 0 | 12 | 5 | 47 | ✅🙈🙈✅🙈🙈❌✅🙈🙈🙈❌🙈❌❌✅✅✅🙈❌🙈❌🙈❌🙈🙈❌🤯🤯🤯🤯🤯❌🙈❌🙈❌🙈🙈🙈🙈✅✅🙈🙈🙈❌ |
| unevaluatedProperties.json | 6 | 23 | 0 | 0 | 0 | 0 | 7 | 9 | 45 | ✅🙈🙈🙈🙈✅✅🙈🤯🤯🤯🙈🙈🙈🙈🙈🙈🙈❌🤯🤯🤯🤯❌❌🤯✅🤯🙈🙈🙈🙈❌❌🙈❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 3 | 5 | 0 | 0 | 0 | 0 | 2 | 0 | 10 | 🙈✅🙈🙈❌🙈✅✅❌🙈 |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 4 | ❌✅❌✅ |
| **Total** | **126** | **205** | **3** | **3** | **0** | **43** | **53** | **107** | **540** |  |

### Failures

#### 🤯 `generation_panic` (107)

68 distinct first-line errors; most common:

- **6×** `Expected definition: '#/$defs/bar' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **5×** `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **4×** `Expected definition: '#/$defs/myobject' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **3×** `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **3×** `Expected definition: 'http://localhost:1234/different-id-ref-string.json' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`

Per group:

- `anchor-legacy.json::Location-independent identifier (7)` — `Expected definition: '#foo' #foo at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `Expected definition: 'http://localhost:1234/nested.json#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `Expected definition: 'https://example.com/schema-with-anchor#foo' https: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::anchor within remote ref (7)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#/definitions/refToInteger' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#/definitions/refToInteger' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `Expected definition: 'http://localhost:1234/detached-ref.json#/definitions/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `Expected definition: 'http://localhost:1234/detached-ref.json#/definitions/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::Location-independent identifier (2020-12)` — `Expected definition: '#foo' #foo at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `Expected definition: 'http://localhost:1234/bar#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `Expected definition: 'http://localhost:1234/nested.json#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::same $anchor with different base uri (2020-12)` — `Expected definition: 'child1#my_anchor' child1#my_anchor at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `Expected definition: 'https://example.com/schema-with-anchor#foo' https: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::anchor within remote ref (2020-12)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `Expected definition: 'http://localhost:1234/locationIndependentIdentifier.json#/$defs/refToInteger' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `Expected definition: 'http://localhost:1234/detached-ref.json#/$defs/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (2020) (2020-12)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `Expected definition: 'intermediate-scope' intermediate-scope at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (2020) (2020-12)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (2020) (2020-12)` — `Expected definition: 'tree.json' tree.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `Expected definition: 'http://localhost:1234/detached-dynamicref.json#/$defs/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (2020) (2020-12)` — `Expected definition: 'first#/$defs/stuff' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `Expected definition: 'extended' extended at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor (2020-12)` — `Expected definition: 'extended' extended at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `enum.json::simple enum validation (2020-12)` — `Expected string for enum got '1' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::heterogeneous enum validation (2020-12)` — `Expected string for enum got '6' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::heterogeneous enum-with-null validation (2020-12)` — `Expected string for enum got '6' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with false does not match 0 (2020-12)` — `Expected string for enum got 'false' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with [false] does not match [0] (2020-12)` — `Expected string for enum got '[false]' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with true does not match 1 (2020-12)` — `Expected string for enum got 'true' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with [true] does not match [1] (2020-12)` — `Expected string for enum got '[true]' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with 0 does not match false (2020-12)` — `Expected string for enum got '0' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with [0] does not match [false] (2020-12)` — `Expected string for enum got '[0]' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with 1 does not match true (2020-12)` — `Expected string for enum got '1' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `enum.json::enum with [1] does not match [true] (2020-12)` — `Expected string for enum got '[1]' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `id-legacy.json::$ref prevents a sibling $id from changing the base uri (7)` — `Expected definition: 'foo.json' foo.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::refs with relative uris and defs (7)` — `Expected definition: 'schema-relative-uri-defs2.json' schema-relative-uri-defs2.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::refs with relative uris and defs (id) (4)` — `Expected definition: 'schema-relative-uri-defs2.json' schema-relative-uri-defs2.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::relative refs with absolute uris and defs (7)` — `Expected definition: 'schema-refs-absolute-uris-defs2.json' schema-refs-absolute-uris-defs2.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::relative refs with absolute uris and defs (id) (4)` — `Expected definition: 'schema-refs-absolute-uris-defs2.json' schema-refs-absolute-uris-defs2.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::$id must be resolved against nearest parent, not just immediate parent (7)` — `Expected definition: 'http://example.com/b/d.json' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::ref with absolute-path-reference (7)` — `Expected definition: '/absref/foobar.json'  at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `id-legacy.json::URN ref with nested pointer ref (7)` — `Expected definition: '#/definitions/bar' bar at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- … and 57 more

#### ❌ `generation_error` (53)

7 distinct first-line errors; most common:

- **29×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- **10×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- **6×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- **3×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- **3×** `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`

Per group:

- `allOf.json::allOf with boolean schemas, all true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `allOf.json::allOf with boolean schemas, some false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `allOf.json::allOf with boolean schemas, all false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `boolean_schema.json::boolean schema 'true' (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `boolean_schema.json::boolean schema 'false' (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `items.json::items with boolean schema (true) (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- `items.json::items with boolean schema (false) (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `items.json::items as array with boolean schemas (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `items.json::items and subitems (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `items.json::prefixItems with no additional items permitted (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `items.json::prefixItems with heterogeneous array (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `maxItems.json::maxItems validation with a decimal (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `maxLength.json::maxLength validation with a decimal (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `maxProperties.json::maxProperties validation with a decimal (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `maximum.json::exclusiveMaximum validation (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '3.0', expected a boolean`
- `minimum.json::exclusiveMinimum validation (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '1.1', expected a boolean`
- `not.json::forbid everything with boolean schema true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `not.json::allow everything with boolean schema false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `not.json::collect annotations inside a 'not', even if collection is disabled (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `oneOf.json::oneOf with boolean schemas, all true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `oneOf.json::oneOf with boolean schemas, one true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `oneOf.json::oneOf with boolean schemas, more than one true (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `oneOf.json::oneOf with boolean schemas, all false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `patternProperties.json::patternProperties with boolean schemas (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `properties.json::properties with boolean schema (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `ref-legacy.json::$ref to boolean schema true (7)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `ref-legacy.json::$ref to boolean schema false (7)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with items and prefixItems (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- `unevaluatedItems.json::unevaluatedItems with nested tuple (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with nested items (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with nested prefixItems and items (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- `unevaluatedItems.json::unevaluatedItems with anyOf (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with oneOf (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with not (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with boolean schemas (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems with $recursiveRef (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::unevaluatedItems can't see inside cousins (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::item is evaluated in an uncle schema to unevaluatedItems (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedItems.json::Evaluated items collection needs to consider instance location (2019-09)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::unevaluatedProperties with boolean schemas (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::unevaluatedProperties can't see inside cousins (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::unevaluatedProperties can't see inside cousins (reverse order) (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::in-place applicator siblings, allOf has unevaluated (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::in-place applicator siblings, anyOf has unevaluated (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::unevaluatedProperties + ref inside allOf / oneOf (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `unevaluatedProperties.json::dynamic evaluation inside nested refs (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `uniqueItems.json::uniqueItems with prefixItems and items=false (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- … and 3 more

#### 🦀 `compile_fail` (43)

22 distinct first-line errors; most common:

- **6×** `error[E0425]: cannot find type 'Bar' in this scope`
- **4×** `error[E0425]: cannot find type 'Baz' in this scope`
- **4×** `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- **4×** `error[E0425]: cannot find type 'Foo' in this scope`
- **3×** `error[E0425]: cannot find type 'Something' in this scope`

Per group:

- `anchor-legacy.json::Location-independent identifier (id) (4)` — `error[E0425]: cannot find type 'Foo' in this scope`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `error[E0425]: cannot find type 'Foo' in this scope`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `error[E0425]: cannot find type 'Foo' in this scope`
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `error[E0425]: cannot find type 'Something' in this scope`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `error[E0425]: cannot find type 'Something' in this scope`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `error[E0425]: cannot find type 'Something' in this scope`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `error[E0425]: cannot find type 'Item' in this scope`
- `dynamicRef.json::A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `error[E0425]: cannot find type 'Items' in this scope`
- `enum.json::enum with escaped characters (2020-12)` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `id-legacy.json::base URI change - change folder (7)` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `id-legacy.json::base URI change - change folder (id) (4)` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `id-legacy.json::base URI change - change folder in subschema (7)` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `id-legacy.json::base URI change - change folder in subschema (id) (4)` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `id-legacy.json::$ref prevents a sibling id from changing the base uri (4)` — `error[E0425]: cannot find type 'FooJson' in this scope`
- `id-legacy.json::id must be resolved against nearest parent, not just immediate parent (4)` — `error[E0425]: cannot find type 'DJson' in this scope`
- `id-legacy.json::simple URN base URI with $ref via the URN (id) (4)` — `error[E0425]: cannot find type 'UrnUuidDeadbeef1234FfffFfff4321Feebdaed' in this scope`
- `id-legacy.json::simple tag URI base URI with $ref via the tag URI (id) (4)` — `error[E0425]: cannot find type 'Baz' in this scope`
- `id-legacy.json::simple base URI using an arbitrary URI scheme with $ref to that URI (id) (4)` — `error[E0425]: cannot find type 'Baz' in this scope`
- `id-legacy.json::ref with absolute-path-reference (id) (4)` — `error[E0425]: cannot find type 'FoobarJson' in this scope`
- `id-legacy.json::URN ref with nested pointer ref (id) (4)` — `error[E0425]: cannot find type 'UrnUuidDeadbeef4321FfffFfff1234Feebdaed' in this scope`
- `id.json::base URI change - change folder (2020-12)` — `error[E0107]: missing generics for enum 'std::option::Option'`
- `id.json::base URI change - change folder in subschema (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::simple URN base URI with $ref via the URN (2020-12)` — `error[E0425]: cannot find type 'UrnUuidDeadbeef1234FfffFfff4321Feebdaed' in this scope`
- `id.json::simple tag URI base URI with $ref via the tag URI (2020-12)` — `error[E0425]: cannot find type 'Baz' in this scope`
- `id.json::simple base URI using an arbitrary URI scheme with $ref to that URI (2020-12)` — `error[E0425]: cannot find type 'Baz' in this scope`
- `id.json::simple URN base URI with JSON pointer (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::URN base URI with NSS (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::URN base URI with r-component (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::URN base URI with q-component (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::URN base URI with URN and JSON pointer ref (2020-12)` — `error[E0425]: cannot find type 'Bar' in this scope`
- `id.json::retrieved nested refs resolve relative to their URI not $id (2020-12)` — `error[E0425]: cannot find type 'FooRefStringJson' in this scope`
- `infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop (2020-12)` — `error[E0425]: cannot find type 'Int' in this scope`
- `items.json::items and subitems (2019) (2019-09)` — `error[E0425]: cannot find type 'Item' in this scope`
- `properties.json::properties with escaped characters (2020-12)` — `error[E0124]: field 'foo_bar' is already declared`
- `ref-legacy.json::escaped pointer ref (7)` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `ref-legacy.json::refs with quote (7)` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `ref-legacy.json::root ref in remote ref (7)` — `error[E0425]: cannot find type 'OrNull' in this scope`
- `ref.json::relative pointer ref to object (2020-12)` — `error[E0425]: cannot find type 'Foo' in this scope`
- `ref.json::escaped pointer ref (2020-12)` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `ref.json::ref applies alongside sibling keywords (2020-12)` — `error[E0425]: cannot find type 'Reffed' in this scope`
- `ref.json::property named $ref, containing an actual $ref (2020-12)` — `error[E0425]: cannot find type 'IsString' in this scope`
- `ref.json::refs with quote (2020-12)` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `ref.json::root ref in remote ref (2020-12)` — `error[E0425]: cannot find type 'OrNull' in this scope`

#### 🤪 `mixed` (3)

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `properties.json::properties whose names are Javascript object property names (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `ref.json::root pointer ref (2020-12)` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)

#### 🚧 `false_reject` (3)

- `properties.json::object properties validation (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `required.json::required validation (2020-12)` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)

#### 🙈 `false_accept` (205)

139 distinct first-line errors; most common:

- **6×** `accepted 'too short is invalid' though valid: false`
- **6×** `accepted 'with unevaluated items' though valid: false`
- **6×** `accepted 'with unevaluated properties' though valid: false`
- **5×** `accepted 'empty data' though valid: false`
- **4×** `accepted 'missing dependency' though valid: false`

Per group:

- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `additionalProperties.json::additionalProperties with schema (2020-12)` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `additionalProperties.json::additionalProperties can exist by itself (2020-12)` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `additionalProperties.json::additionalProperties does not look in applicators (2020-12)` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `additionalProperties.json::additionalProperties with propertyNames (2020-12)` — `accepted 'Valid against propertyNames, but not additionalProperties' though valid: false`
  - cases: Valid against propertyNames, but not additionalProperties (accepted)
- `additionalProperties.json::dependentSchemas with additionalProperties (2020-12)` — `accepted 'additionalProperties doesn't consider dependentSchemas' though valid: false`
  - cases: additionalProperties doesn't consider dependentSchemas (accepted), additionalProperties can't see bar (accepted), additionalProperties can't see bar even when foo2 is present (accepted)
- `allOf.json::allOf with base schema (2020-12)` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted)
- `allOf.json::allOf simple types (2020-12)` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `allOf.json::allOf with the first empty schema (2020-12)` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `allOf.json::allOf with the last empty schema (2020-12)` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `allOf.json::nested allOf, to check validation semantics (2020-12)` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `anyOf.json::anyOf (2020-12)` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `anyOf.json::anyOf with base schema (2020-12)` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), both anyOf invalid (accepted)
- `anyOf.json::anyOf complex types (2020-12)` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `anyOf.json::nested anyOf, to check validation semantics (2020-12)` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
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
- … and 155 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 10 | 67 | 1 | 0 | 0 | 5 | 4 | 11 | 98 | 10.2% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 385 | 957 | 40.2% | 1006 | 38.3% |

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
| optional/bignum.json | 3 | 7 | 1 | 0 | 0 | 0 | 2 | 1 | 14 | 🚧✅✅🙈🙈❌🙈🙈❌🤯🙈✅🙈🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 4 | ❌🤯❌🤯 |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/non-schemas.json | 0 | 1 | 0 | 0 | 0 | 4 | 0 | 8 | 13 | 🤯🤯🤯🤯🙈🤯🤯🤯🦀🦀🦀🤯🦀 |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **10** | **67** | **1** | **0** | **0** | **5** | **4** | **11** | **98** |  |

### Failures

#### 🤯 `generation_panic` (11)

11 distinct first-line errors; most common:

- **1×** `Expected definition: '#/$defs/anchor_in_enum' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: '#/$defs/id_in_enum' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: '#/$defs/id_in_unknown0' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: '#/examples/0' examples at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: 'http://localhost:1234/draft2019-09/ignore-prefixItems.json' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`

Per group:

- `optional/bignum.json::enum comparison with high precision (2020-12)` — `Expected string for enum got '0.12345678912345678' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `Expected definition: 'http://localhost:1234/draft2020-12/prefixItems.json' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `Expected definition: 'http://localhost:1234/draft2019-09/ignore-prefixItems.json' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/$defs) (2020-12)` — `Expected definition: '#/$defs/id_in_enum' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/definitions) (7)` — `Expected string for enum got '{"$id":"https://localhost:1234/id/my_identifier.json","type":"null"}' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `optional/non-schemas.json::$id inside an enum is not a real identifier (id/definitions) (4)` — `Expected string for enum got '{"id":"https://localhost:1234/my_identifier.json","type":"null"}' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier ($defs) (2020-12)` — `Expected definition: '#/$defs/id_in_unknown0' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($anchor) (2020-12)` — `Expected definition: '#/$defs/anchor_in_enum' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($id) (7)` — `Expected string for enum got '{"$id":"#my_anchor","type":"null"}' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier (id) (4)` — `Expected string for enum got '{"id":"#my_anchor","type":"null"}' at <registry>/schemafy_lib-0.6.0/src/lib.rs:696`
- `optional/non-schemas.json::reference internals of known non-applicator (2020-12)` — `Expected definition: '#/examples/0' examples at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`

#### ❌ `generation_error` (4)

- `optional/bignum.json::float comparison with high precision (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '9.727837981879871e+26', expected a boolean`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '-9.727837981879871e+26', expected a boolean`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`

#### 🦀 `compile_fail` (5)

- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `error[E0425]: cannot find type 'Item' in this scope`
- `optional/non-schemas.json::reference of a root arbitrary keyword (2020-12)` — `error[E0425]: cannot find type 'UnknownKeyword' in this scope`
- `optional/non-schemas.json::reference of a root arbitrary keyword with encoded ref (2020-12)` — `error[E0425]: cannot find type 'Unknown1Keyword' in this scope`
- `optional/non-schemas.json::reference of an arbitrary keyword of a sub-schema (2020-12)` — `error[E0425]: cannot find type 'UnknownKeyword' in this scope`
- `optional/non-schemas.json::reference of an arbitrary keyword of a sub-schema with encoded ref (2020-12)` — `error[E0425]: cannot find type 'Unknown1Keyword' in this scope`

#### 🚧 `false_reject` (1)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)

#### 🙈 `false_accept` (67)

56 distinct first-line errors; most common:

- **4×** `accepted 'ascii non-digits' though valid: false`
- **4×** `accepted 'literal unicode character in json string' though valid: false`
- **3×** `accepted 'does not match' though valid: false`
- **2×** `accepted 'doesn't match one' though valid: false`
- **2×** `accepted 'missing dependency' though valid: false`

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
- `optional/bignum.json::minimum integer comparison (2020-12)` — `accepted 'just below the minimum is invalid' though valid: false`
  - cases: just below the minimum is invalid (accepted)
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `accepted 'comparison works for very negative numbers' though valid: false`
  - cases: comparison works for very negative numbers (accepted)
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
- `optional/ecmascript-regex.json::ECMA 262 \S matches everything but whitespace (2020-12)` — `accepted 'ASCII space does not match' though valid: false`
  - cases: ASCII space does not match (accepted), Character tabulation does not match (accepted), Line tabulation does not match (accepted), Form feed does not match (accepted), latin-1 non-breaking-space does not match (accepted), zero-width whitespace does not match (accepted), … and 3 more
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `accepted 'unicode matching is case-sensitive' though valid: false`
  - cases: unicode matching is case-sensitive (accepted)
- `optional/ecmascript-regex.json::\w in patterns matches [A-Za-z0-9_], not unicode letters (2020-12)` — `accepted 'literal unicode character in json string' though valid: false`
  - cases: literal unicode character in json string (accepted), unicode character in hex format in string (accepted), unicode matching is case-sensitive (accepted)
- … and 17 more

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 3 | 0 | 0 | 0 | 2 | 0 | 0 | 5 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 18 | 22 | 81.8% | 34 | 52.9% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 0 | 3 | 0 | 0 | 0 | 2 | 0 | 0 | 5 | 🙈🦀🦀🙈🙈 |
| **Total** | **0** | **3** | **0** | **0** | **0** | **2** | **0** | **0** | **5** |  |

### Failures

#### 🦀 `compile_fail` (2)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads by $ref (2020-12)` — `error[E0425]: cannot find type 'Circle' in this scope`
- `compound/externally-tagged-union.json::oneOf of objects with one required key each and additionalProperties false (2020-12)` — `error[E0425]: cannot find type 'Circle' in this scope`

#### 🙈 `false_accept` (3)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads inline (2020-12)` — `accepted 'circle and square together match both branches, invalid' though valid: false`
  - cases: circle and square together match both branches, invalid (accepted)
- `compound/externally-tagged-union.json::oneOf mixing a unit variant and a data variant (2020-12)` — `accepted 'the unit variant in map form with null is invalid' though valid: false`
  - cases: the unit variant in map form with null is invalid (accepted), the data variant with an extra key is invalid (accepted)
- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a member in map form with null is invalid' though valid: false`
  - cases: a member in map form with null is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 10 | 0 | 0 | 0 | 2 | 0 | 11 | 25 | 8.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 33 | 62 | 53.2% | 90 | 36.7% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 9 | 12 | 🙈🤯🤯🤯🙈🤯🤯🤯🤯🤯🦀🤯 |
| format-date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| proposals/propertyDependencies/additionalProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| proposals/propertyDependencies/propertyDependencies.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| **Total** | **2** | **10** | **0** | **0** | **0** | **2** | **0** | **11** | **25** |  |

### Failures

#### 🤯 `generation_panic` (11)

7 distinct first-line errors; most common:

- **3×** `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **2×** `Expected definition: './baseSchema' . at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **2×** `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: 'first#/$defs/stuff' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- **1×** `Expected definition: 'http://localhost:1234/detached-dynamicref.json#/$defs/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`

Per group:

- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `Expected definition: 'intermediate-scope' intermediate-scope at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `Expected definition: 'list' list at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `Expected definition: 'tree.json' tree.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `Expected definition: 'extendible-dynamic-ref.json' extendible-dynamic-ref.json at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `Expected definition: 'http://localhost:1234/detached-dynamicref.json#/$defs/foo' http: at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `Expected definition: 'first#/$defs/stuff' $defs at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `Expected definition: './baseSchema' . at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `Expected definition: './baseSchema' . at <registry>/schemafy_lib-0.6.0/src/lib.rs:410`

#### 🦀 `compile_fail` (2)

- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `error[E0425]: cannot find type 'Item' in this scope`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `error[E0425]: cannot find type 'Item' in this scope`

#### 🙈 `false_accept` (10)

10 distinct first-line errors; most common:

- **1×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **1×** `accepted 'a leap second is not a valid time' though valid: false`
- **1×** `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
- **1×** `accepted 'an invalid date-time-ext with an empty suffix' though valid: false`
- **1×** `accepted 'bar with more than 2 properties is invalid' though valid: false`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `accepted 'string matches /$defs/thingy, but the $dynamicRef does not stop here' though valid: false`
  - cases: string matches /$defs/thingy, but the $dynamicRef does not stop here (accepted), first_scope is not in dynamic scope for the $dynamicRef (accepted)
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

