# ogen v1.24.0

An OpenAPI v3 generator for Go: its design center is an API document, and
it emits a type per schema with a code-generated JSON codec and validator on
top of `go-faster/jx`. Each group here is wrapped as an OpenAPI 3.1 document
with the schema as a component (3.1 schema objects are JSON Schema 2020-12,
so nothing is translated), and the check calls both `Decode` and
`Validate`.

_generated 2026-11-22T02:41:40Z_

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
| 101 | 220 | 2 | 0 | 0 | 31 | 186 | 0 | 540 | 18.7% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 718 | 1066 | 67.4% | 1664 | 43.1% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 3 | 0 | 12 | 🙈🙈🙈❌❌❌✅✅✅✅✅🦀 |
| anchor-legacy.json | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 14 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 8 | ❌❌❌❌❌❌❌✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈🙈🙈 |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 0 | 5 | 0 | 0 | 0 | 0 | 16 | 0 | 21 | 🙈❌❌❌🙈❌❌❌❌❌❌❌🙈❌❌❌❌❌❌🙈🙈 |
| enum.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-email.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-idn-email.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-idn-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-iri-reference.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-iri.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-json-pointer.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-relative-json-pointer.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format-uri-reference.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-uri-template.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| id-legacy.json | 0 | 10 | 0 | 0 | 0 | 2 | 28 | 0 | 40 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌🦀🙈🦀🙈❌❌❌❌❌❌❌❌ |
| id.json | 0 | 4 | 0 | 0 | 0 | 2 | 19 | 0 | 25 | ❌❌❌❌❌❌❌❌❌❌❌❌🙈🙈🙈🙈❌❌🦀🦀❌❌❌❌❌ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection-legacy.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 10 | 11 | 0 | 0 | 0 | 0 | 7 | 0 | 28 | 🙈🙈🙈❌❌🙈❌❌🙈✅✅✅✅✅🙈✅❌🙈✅✅✅🙈❌🙈🙈❌🙈✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈🙈❌ |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈🙈❌ |
| multipleOf.json | 0 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 6 | 🙈🙈🙈🙈❌❌ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌✅ |
| pattern.json | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅✅✅ |
| patternProperties.json | 2 | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈🙈❌✅✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈❌🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 0 | 0 | 0 | 0 | 1 | 0 | 10 | 🙈🙈✅🙈🙈🙈🙈🙈🙈❌ |
| ref-legacy.json | 0 | 7 | 0 | 0 | 0 | 0 | 7 | 0 | 14 | 🙈❌🙈🙈🙈🙈❌❌🙈🙈❌❌❌❌ |
| ref.json | 0 | 8 | 0 | 0 | 0 | 6 | 9 | 0 | 23 | 🙈🙈🙈❌🦀🙈🙈🙈🙈❌❌❌🙈🦀🦀🦀🦀🦀❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 8 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🚧✅✅✅✅✅✅✅❌❌❌ |
| unevaluatedItems.json | 8 | 20 | 0 | 0 | 0 | 2 | 17 | 0 | 47 | ✅🙈🙈✅🙈🙈❌✅🙈🙈🙈❌🙈❌❌✅✅✅❌❌❌❌🙈🙈🙈🙈❌🦀❌🦀❌❌❌🙈❌❌❌🙈🙈🙈🙈✅✅🙈🙈🙈❌ |
| unevaluatedProperties.json | 9 | 20 | 0 | 0 | 0 | 2 | 14 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈❌✅✅❌❌🙈🙈🙈🙈🙈❌🦀🦀❌❌❌❌✅✅🙈🙈🙈🙈❌❌❌❌❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 3 | 1 | 0 | 0 | 0 | 0 | 6 | 0 | 10 | ❌✅❌❌❌❌✅✅❌🙈 |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 4 | ❌✅❌✅ |
| **Total** | **101** | **220** | **2** | **0** | **0** | **31** | **186** | **0** | **540** |  |

### Failures

#### ❌ `generation_error` (186)

101 distinct first-line errors; most common:

- **22×** `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- **11×** `callback: json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- **9×** `unexpected type bool`
- **6×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- **5×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`

Per group:

- `allOf.json::allOf with boolean schemas, all true (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `allOf.json::allOf with boolean schemas, some false (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `allOf.json::allOf with boolean schemas, all false (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `anchor-legacy.json::Location-independent identifier (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "#foo": resolve: invalid pointer "foo": pointer…`
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "#foo": resolve: invalid pointer "foo"…`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "http://localhost:1234/nested.json#foo": get "h…`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "http://localhost:1234/nested.json#foo…`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "https://example.com/schema-with-anchor#foo": g…`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "https://example.com/schema-with-ancho…`
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "urn:uuid:deadbeef-1234-ff00-00ff-4321…`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "urn:uuid:deadbeef-1234-ff00-00ff-4321…`
- `anchor-legacy.json::anchor within remote ref (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/detached-ref.json#/definitions/…`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/detached-ref.json#/definitions/…`
- `anchor.json::Location-independent identifier (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "#foo": resolve: invalid pointer "foo": pointer must s…`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/bar#foo": get "http://localhost…`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/nested.json#foo": get "http://l…`
- `anchor.json::same $anchor with different base uri (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "child1#my_anchor": get "jsonschema://dummy/child1": e…`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "https://example.com/schema-with-anchor#foo": g…`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "foo": $ref: resolve "urn:uuid:deadbeef-1234-ff00-00ff-4321…`
- `anchor.json::anchor within remote ref (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/locationIndependentIdentifier.j…`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/detached-ref.json#/$defs/foo": …`
- `anyOf.json::anyOf (2020-12)` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `anyOf.json::anyOf with base schema (2020-12)` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `anyOf.json::anyOf complex types (2020-12)` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `anyOf.json::anyOf with one empty schema (2020-12)` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `boolean_schema.json::boolean schema 'true' (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `boolean_schema.json::boolean schema 'false' (2020-12)` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "intermediate-scope": get "jsonschema://dummy/intermed…`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "tree.json": get "jsonschema://dummy/tree.json": exter…`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "extendible-dynamic-ref.json": get "jsonschema://dummy…`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "extendible-dynamic-ref.json": get "jsonschema:…`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "extendible-dynamic-ref.json": get "jsonschema:…`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/detached-dynamicref.json#/$defs…`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "bar-item": $ref: resolve "item": get "jsonschema://dummy/i…`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "first#/$defs/stuff": get "jsonschema://dummy/first": …`
- `dynamicRef.json::A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": items: $ref: resolve "#items": resolve: invalid pointer "items": poi…`
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "extended": get "jsonschema://dummy/extended": externa…`
- … and 136 more

#### 🦀 `compile_fail` (31)

21 distinct first-line errors; most common:

- **2×** `g_0b026108eff52b4f/g_0b026108eff52b4f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- **2×** `g_4dd3df819e7eb4f1/g_4dd3df819e7eb4f1.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- **2×** `g_57098af8ea6b76df/g_57098af8ea6b76df.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- **2×** `g_64ac6148c0b66ff3/g_64ac6148c0b66ff3.go:31:15: cannot use v (variable of int64 type "time".Duration) as jx.Raw value in assignment`
- **2×** `g_751e2a631450f0d4/g_751e2a631450f0d4.go:62:8: undefined: Root`

Per group:

- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `g_a7bc76f46069b836/g_a7bc76f46069b836.go:16:8: undefined: Root`
- `format-date-time.json::validation of non-string date-time values (2020-12)` — `g_0b026108eff52b4f/g_0b026108eff52b4f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-date-time.json::date-time annotation (2020-12)` — `g_0b026108eff52b4f/g_0b026108eff52b4f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-date.json::validation of non-string date values (2020-12)` — `g_88b9c2f9d0ec847f/g_88b9c2f9d0ec847f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-date.json::date format annotation (2020-12)` — `g_88b9c2f9d0ec847f/g_88b9c2f9d0ec847f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-duration.json::validation of non-string duration values (2020-12)` — `g_64ac6148c0b66ff3/g_64ac6148c0b66ff3.go:31:15: cannot use v (variable of int64 type "time".Duration) as jx.Raw value in assignment`
- `format-duration.json::duration format annotation (2020-12)` — `g_64ac6148c0b66ff3/g_64ac6148c0b66ff3.go:31:15: cannot use v (variable of int64 type "time".Duration) as jx.Raw value in assignment`
- `format-ipv4.json::validation of non-string IP addresses (2020-12)` — `g_a390a6a874b37b87/g_a390a6a874b37b87.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-ipv4.json::ipv4 format annotation (2020-12)` — `g_a390a6a874b37b87/g_a390a6a874b37b87.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-ipv6.json::validation of non-string ipv6 values (2020-12)` — `g_c5bccd6173a0f8ba/g_c5bccd6173a0f8ba.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-ipv6.json::ipv6 format (2020-12)` — `g_c5bccd6173a0f8ba/g_c5bccd6173a0f8ba.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-time.json::validation of non-string time values (2020-12)` — `g_ec6f8732906c68ab/g_ec6f8732906c68ab.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-time.json::time format (2020-12)` — `g_ec6f8732906c68ab/g_ec6f8732906c68ab.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-uri.json::validation of non-string uri values (2020-12)` — `g_4dd3df819e7eb4f1/g_4dd3df819e7eb4f1.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `format-uri.json::uri format annotation (2020-12)` — `g_4dd3df819e7eb4f1/g_4dd3df819e7eb4f1.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `format-uuid.json::validation of non-string uuid values (2020-12)` — `g_57098af8ea6b76df/g_57098af8ea6b76df.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- `format-uuid.json::uuid format annotation (2020-12)` — `g_57098af8ea6b76df/g_57098af8ea6b76df.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- `id-legacy.json::$id with file URI still resolves pointers - *nix (7)` — `g_e4fafaf8d41a434b/g_e4fafaf8d41a434b.go:69:8: undefined: Root`
- `id-legacy.json::$id with file URI still resolves pointers - windows (7)` — `g_af990e863765fa73/g_af990e863765fa73.go:69:8: undefined: Root`
- `id.json::$id with file URI still resolves pointers - *nix (2020-12)` — `g_917206b598ca401a/g_917206b598ca401a.go:69:8: undefined: Root`
- `id.json::$id with file URI still resolves pointers - windows (2020-12)` — `g_0381fb7eb15b7547/g_0381fb7eb15b7547.go:69:8: undefined: Root`
- `ref.json::nested refs (2020-12)` — `g_3691cf520f18fe7a/g_3691cf520f18fe7a.go:60:8: undefined: Root`
- `ref.json::ref creates new scope when adjacent to keywords (2020-12)` — `g_d295a5390dba3f7d/g_d295a5390dba3f7d.go:62:8: undefined: Root`
- `ref.json::ref to if (2020-12)` — `g_6c0224fec4f4216a/g_6c0224fec4f4216a.go:60:8: undefined: Root`
- `ref.json::ref to then (2020-12)` — `g_4612667e7ba2171c/g_4612667e7ba2171c.go:60:8: undefined: Root`
- `ref.json::ref to else (2020-12)` — `g_5733ce677d191b67/g_5733ce677d191b67.go:60:8: undefined: Root`
- `ref.json::empty tokens in $ref json-pointer (2020-12)` — `g_f86ad1ddbf085ad9/g_f86ad1ddbf085ad9.go:69:8: undefined: Root`
- `unevaluatedItems.json::unevaluatedItems with $ref (prefixItems) (2020-12)` — `g_751e2a631450f0d4/g_751e2a631450f0d4.go:62:8: undefined: Root`
- `unevaluatedItems.json::unevaluatedItems before $ref (prefixItems) (2020-12)` — `g_751e2a631450f0d4/g_751e2a631450f0d4.go:62:8: undefined: Root`
- `unevaluatedProperties.json::unevaluatedProperties with $ref (2020-12)` — `g_8077669cdd981595/g_8077669cdd981595.go:62:8: undefined: Root`
- `unevaluatedProperties.json::unevaluatedProperties before $ref (2020-12)` — `g_8077669cdd981595/g_8077669cdd981595.go:62:8: undefined: Root`

#### 🚧 `false_reject` (2)

- `default.json::the default keyword does not do anything if the property is missing (2020-12)` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`
  - cases: a float with zero fractional part is an integer (rejected)

#### 🙈 `false_accept` (220)

137 distinct first-line errors; most common:

- **12×** `accepted 'a non-string is invalid' though valid: false`
- **6×** `accepted 'with unevaluated items' though valid: false`
- **5×** `accepted 'empty data' though valid: false`
- **5×** `accepted 'mismatch' though valid: false`
- **5×** `accepted 'with unevaluated properties' though valid: false`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
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
- `allOf.json::allOf (2020-12)` — `accepted 'mismatch second' though valid: false`
  - cases: mismatch second (accepted), mismatch first (accepted), wrong type (accepted)
- `allOf.json::allOf with base schema (2020-12)` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch first allOf (accepted), mismatch second allOf (accepted), mismatch both (accepted)
- `allOf.json::allOf simple types (2020-12)` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
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
- … and 170 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 25 | 44 | 1 | 0 | 0 | 12 | 16 | 0 | 98 | 25.5% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 305 | 598 | 51.0% | 1006 | 30.3% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-email.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| format-hostname.json | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅🙈 |
| format-idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| format-ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/bignum.json | 3 | 7 | 1 | 0 | 0 | 0 | 3 | 0 | 14 | 🚧✅✅🙈🙈❌🙈🙈❌🙈🙈✅❌🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/non-schemas.json | 0 | 4 | 0 | 0 | 0 | 1 | 8 | 0 | 13 | ❌❌❌❌❌❌❌❌🙈🙈🙈🦀🙈 |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **25** | **44** | **1** | **0** | **0** | **12** | **16** | **0** | **98** |  |

### Failures

#### ❌ `generation_error` (16)

11 distinct first-line errors; most common:

- **3×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "#my_anchor": resolve: invalid pointer "my_anch…`
- **2×** `callback: json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- **2×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/id/my_identifier.json":…`
- **2×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/unknownKeyword/my_ident…`
- **1×** `json: cannot unmarshal number into Go struct field .exclusiveMaximum of type bool`

Per group:

- `optional/bignum.json::float comparison with high precision (2020-12)` — `json: cannot unmarshal number into Go struct field .exclusiveMaximum of type bool`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `json: cannot unmarshal number into Go struct field .exclusiveMinimum of type bool`
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: empty uniqueItems not implemented`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `callback: json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/draft2020-12/prefixItems.json":…`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `callback: json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/draft2019-09/ignore-prefixItems…`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "bar-item": $ref: resolve "bar#/$defs/item": get "jsonschem…`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/$defs) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/id/my_identifier.json":…`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/definitions) (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/id/my_identifier.json":…`
- `optional/non-schemas.json::$id inside an enum is not a real identifier (id/definitions) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/my_identifier.json": ge…`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier ($defs) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/unknownKeyword/my_ident…`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "https://localhost:1234/unknownKeyword/my_ident…`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($anchor) (2020-12)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "#my_anchor": resolve: invalid pointer "my_anch…`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($id) (7)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "#my_anchor": resolve: invalid pointer "my_anch…`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier (id) (4)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": anyOf: $ref: resolve "#my_anchor": resolve: invalid pointer "my_anch…`

#### 🦀 `compile_fail` (12)

11 distinct first-line errors; most common:

- **2×** `g_ec6f8732906c68ab/g_ec6f8732906c68ab.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- **1×** `g_0b026108eff52b4f/g_0b026108eff52b4f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- **1×** `g_4dd3df819e7eb4f1/g_4dd3df819e7eb4f1.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- **1×** `g_57098af8ea6b76df/g_57098af8ea6b76df.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- **1×** `g_5a31db261562afd4/g_5a31db261562afd4.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `g_0b026108eff52b4f/g_0b026108eff52b4f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-date.json::validation of date strings (2020-12)` — `g_88b9c2f9d0ec847f/g_88b9c2f9d0ec847f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-duration.json::validation of duration strings (2020-12)` — `g_64ac6148c0b66ff3/g_64ac6148c0b66ff3.go:31:15: cannot use v (variable of int64 type "time".Duration) as jx.Raw value in assignment`
- `format-ipv4.json::validation of IP addresses (2020-12)` — `g_a390a6a874b37b87/g_a390a6a874b37b87.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `g_c5bccd6173a0f8ba/g_c5bccd6173a0f8ba.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `format-time.json::validation of time strings (2020-12)` — `g_ec6f8732906c68ab/g_ec6f8732906c68ab.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-uri.json::validation of URIs (2020-12)` — `g_4dd3df819e7eb4f1/g_4dd3df819e7eb4f1.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `format-uuid.json::uuid format (2020-12)` — `g_57098af8ea6b76df/g_57098af8ea6b76df.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `g_5a31db261562afd4/g_5a31db261562afd4.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true (2020-12)` — `g_7103ad40fd782529/g_7103ad40fd782529.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `optional/format-time.json::validation of time strings (2020-12)` — `g_ec6f8732906c68ab/g_ec6f8732906c68ab.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `optional/non-schemas.json::reference internals of known non-applicator (2020-12)` — `g_6afe637afaebf932/g_6afe637afaebf932.go:60:8: undefined: Root`

#### 🚧 `false_reject` (1)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)

#### 🙈 `false_accept` (44)

36 distinct first-line errors; most common:

- **4×** `accepted 'mismatch' though valid: false`
- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'doesn't match one' though valid: false`
- **2×** `accepted 'literal unicode character in json string' though valid: false`

Per group:

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
- `format-uri-reference.json::validation of URI References (2020-12)` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `accepted 'an invalid uri-template' though valid: false`
  - cases: an invalid uri-template (accepted), an empty expression is invalid (accepted), an empty varspec inside the variable list is invalid (accepted), a zero prefix max-length is invalid (accepted), a five-digit prefix max-length is invalid (accepted), an unmatched closing brace is invalid (accepted), … and 15 more
- `optional/bignum.json::maximum integer comparison (2020-12)` — `accepted 'just above the maximum is invalid' though valid: false`
  - cases: just above the maximum is invalid (accepted)
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `accepted 'comparison works for high numbers' though valid: false`
  - cases: comparison works for high numbers (accepted)
- `optional/bignum.json::minimum integer comparison (2020-12)` — `accepted 'just below the minimum is invalid' though valid: false`
  - cases: just below the minimum is invalid (accepted)
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `accepted 'comparison works for very negative numbers' though valid: false`
  - cases: comparison works for very negative numbers (accepted)
- `optional/bignum.json::enum comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
- `optional/bignum.json::const comparison with high precision (2020-12)` — `accepted 'a slightly smaller decimal is invalid' though valid: false`
  - cases: a slightly smaller decimal is invalid (accepted), a slightly larger decimal is invalid (accepted)
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
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `accepted 'unicode matching is case-sensitive' though valid: false`
  - cases: unicode matching is case-sensitive (accepted)
- `optional/ecmascript-regex.json::\w in patterns matches [A-Za-z0-9_], not unicode letters (2020-12)` — `accepted 'literal unicode character in json string' though valid: false`
  - cases: literal unicode character in json string (accepted), unicode character in hex format in string (accepted), unicode matching is case-sensitive (accepted)
- `optional/ecmascript-regex.json::pattern with ASCII ranges (2020-12)` — `accepted 'literal unicode character in json string' though valid: false`
  - cases: literal unicode character in json string (accepted), unicode character in hex format in string (accepted)
- `optional/ecmascript-regex.json::\d in pattern matches [0-9], not unicode digits (2020-12)` — `accepted 'ascii non-digits' though valid: false`
  - cases: ascii non-digits (accepted), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (accepted)
- `optional/ecmascript-regex.json::pattern with non-ASCII digits (2020-12)` — `accepted 'ascii non-digits' though valid: false`
  - cases: ascii non-digits (accepted)
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `optional/format-regex.json::global inline flag groups are not valid ECMA 262 (2020-12)` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `optional/non-bmp-regex.json::Proper UTF-16 surrogate pair handling: pattern (2020-12)` — `accepted 'doesn't match one' though valid: false`
  - cases: doesn't match one (accepted), doesn't match two (accepted), doesn't match one ASCII (accepted), doesn't match two ASCII (accepted)
- `optional/non-bmp-regex.json::Proper UTF-16 surrogate pair handling: patternProperties (2020-12)` — `accepted 'doesn't match one' though valid: false`
  - cases: doesn't match one (accepted), doesn't match two (accepted)
- `optional/non-schemas.json::reference of a root arbitrary keyword (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `optional/non-schemas.json::reference of a root arbitrary keyword with encoded ref (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `optional/non-schemas.json::reference of an arbitrary keyword of a sub-schema (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `optional/non-schemas.json::reference of an arbitrary keyword of a sub-schema with encoded ref (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 80.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 29 | 34 | 85.3% | 34 | 85.3% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| **Total** | **4** | **1** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

#### 🙈 `false_accept` (1)

- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a non-member string is invalid' though valid: false`
  - cases: a non-member string is invalid (accepted), a member in map form with null is invalid (accepted), a member in map form with an empty object is invalid (accepted), a non-member in map form with null is invalid (accepted), a member wrapped in an array is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 | 8 | 0 | 0 | 0 | 2 | 13 | 0 | 25 | 8.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 25 | 43 | 58.1% | 90 | 27.8% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 0 | 10 | 0 | 12 | 🙈❌❌❌🙈❌❌❌❌❌❌❌ |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/additionalProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| proposals/propertyDependencies/propertyDependencies.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **2** | **8** | **0** | **0** | **0** | **2** | **13** | **0** | **25** |  |

### Failures

#### ❌ `generation_error` (13)

10 distinct first-line errors; most common:

- **2×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "./baseSchema": get "jsonschema://dummy/baseSchema": e…`
- **2×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- **2×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "extendible-dynamic-ref.json": get "jsonschema:…`
- **1×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "extendible-dynamic-ref.json": get "jsonschema://dummy…`
- **1×** `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "first#/$defs/stuff": get "jsonschema://dummy/first": …`

Per group:

- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "intermediate-scope": get "jsonschema://dummy/intermed…`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "list": get "jsonschema://dummy/list": external refere…`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "tree.json": get "jsonschema://dummy/tree.json": exter…`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "extendible-dynamic-ref.json": get "jsonschema://dummy…`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "extendible-dynamic-ref.json": get "jsonschema:…`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "extendible-dynamic-ref.json": get "jsonschema:…`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/detached-dynamicref.json#/$defs…`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "bar-item": $ref: resolve "item": get "jsonschema://dummy/i…`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "first#/$defs/stuff": get "jsonschema://dummy/first": …`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "bar-item": $ref: resolve "bar#/$defs/item": get "jsonschem…`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "./baseSchema": get "jsonschema://dummy/baseSchema": e…`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "./baseSchema": get "jsonschema://dummy/baseSchema": e…`

#### 🦀 `compile_fail` (2)

- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `g_4cfc9e43cb78f43f/g_4cfc9e43cb78f43f.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `format-time.json::leap seconds are not valid times (v1)` — `g_1f9b7ecdc1697d47/g_1f9b7ecdc1697d47.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`

#### 🙈 `false_accept` (8)

8 distinct first-line errors; most common:

- **1×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **1×** `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`
- **1×** `accepted 'bar with more than 2 properties is invalid' though valid: false`
- **1×** `accepted 'disallows bar if foo != foo1' though valid: false`
- **1×** `accepted 'expected strings - additional property as not string is invalid' though valid: false`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `accepted 'string matches /$defs/thingy, but the $dynamicRef does not stop here' though valid: false`
  - cases: string matches /$defs/thingy, but the $dynamicRef does not stop here (accepted), first_scope is not in dynamic scope for the $dynamicRef (accepted)
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

