# go-jsonschema v0.24.1

Go structs from JSON Schema, plus hand-written `UnmarshalJSON` methods that
re-check the schema's keywords while decoding. Its design center is a config
file with a known shape: a root that declares `type: object` becomes a tagged
struct, and `required`, `enum`, lengths, patterns and bounds are enforced in
the unmarshaller rather than left to the type system. The flip side is that a
root with no `type` keyword produces no root type at all, and the combinators
collapse into a merged type or `interface{}`.

_generated 2026-11-22T03:50:44Z_

**14 of 540 standard groups pass (2.6%)**

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
| 14 | 38 | 2 | 2 | 0 | 356 | 128 | 0 | 540 | 2.6% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 131 | 197 | 66.5% | 1664 | 7.9% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anchor-legacy.json | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 14 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🙈🦀🦀🦀🦀🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 1 | 20 | 0 | 21 | ❌❌❌❌🦀❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 4 | 4 | 0 | 8 | 🦀❌🦀❌🦀❌🦀❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 5 | 3 | 0 | 8 | 🦀❌🦀❌🦀❌🦀🦀 |
| dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 12 | 7 | 0 | 21 | 🙈🦀❌🦀❌🦀🦀🦀🦀🦀❌🦀🙈❌🦀🦀🦀❌❌❌🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 16 | 0 | 0 | 17 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
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
| id-legacy.json | 0 | 0 | 0 | 0 | 0 | 35 | 5 | 0 | 40 | 🦀❌❌❌❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀 |
| id.json | 0 | 0 | 0 | 0 | 0 | 21 | 4 | 0 | 25 | 🦀❌❌🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 1 | 11 | 0 | 12 | ❌❌❌❌❌❌🦀❌❌❌❌❌ |
| infinite-loop-detection-legacy.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 1 | 0 | 0 | 0 | 12 | 14 | 0 | 28 | 🦀❌❌🦀🦀❌❌🙈❌❌✅🦀❌❌🦀🦀🦀❌🦀❌❌🦀❌🦀❌🦀❌🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | 🦀❌🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 9 | 0 | 0 | 11 | 🦀🙈🦀🦀🦀🦀🦀🦀🙈🦀🦀 |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 4 | 🦀🦀✅✅ |
| patternProperties.json | 0 | 1 | 0 | 0 | 0 | 5 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🙈 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| recursiveRef.json | 0 | 0 | 0 | 0 | 0 | 7 | 3 | 0 | 10 | 🦀🦀🦀🦀🦀🦀🦀❌❌❌ |
| ref-legacy.json | 0 | 1 | 0 | 0 | 0 | 12 | 1 | 0 | 14 | ❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🙈🦀 |
| ref.json | 0 | 1 | 0 | 0 | 0 | 21 | 1 | 0 | 23 | 🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🙈🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| type.json | 1 | 8 | 1 | 2 | 0 | 0 | 0 | 0 | 12 | 🙈🚧🙈🙈🙈🙈🙈🙈✅🤪🤪🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 19 | 28 | 0 | 47 | ❌❌❌🦀❌❌🦀❌🦀🦀🦀❌🦀🦀🦀❌🦀❌🦀❌🦀❌🦀❌❌❌🦀🦀❌🦀❌🦀❌🦀❌🦀❌❌🦀❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 7 | 21 | 0 | 0 | 0 | 12 | 5 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🦀✅✅🙈🙈🙈🙈🙈🙈🙈❌🦀🦀❌❌❌❌🦀🦀🦀🦀 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 6 | 4 | 0 | 10 | 🦀🦀🦀❌🦀❌🦀❌🦀❌ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 4 | 🦀✅🦀✅ |
| **Total** | **14** | **38** | **2** | **2** | **0** | **356** | **128** | **0** | **540** |  |

### Failures

#### ❌ `generation_error` (128)

13 distinct first-line errors; most common:

- **74×** `schema has no root`
- **39×** `error parsing from file <schema>: cannot load schema`
- **3×** `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- **2×** `cannot add struct field: could not generate type with scope 'ExtendedBar': could not follow $ref "bar" to file "bar": cannot load schema`
- **2×** `cannot add struct field: could not generate type with scope 'RootList': definition does not exist in schema: "baz/definitions/bar" (from ref "#/definitions/baz/…`

Per group:

- `boolean_schema.json::boolean schema 'true' (2020-12)` — `error parsing from file <schema>: cannot load schema`
- `boolean_schema.json::boolean schema 'false' (2020-12)` — `error parsing from file <schema>: cannot load schema`
- `contains.json::contains keyword validation (2020-12)` — `schema has no root`
- `contains.json::contains keyword with const keyword (2020-12)` — `schema has no root`
- `contains.json::contains keyword with boolean schema true (2020-12)` — `schema has no root`
- `contains.json::contains keyword with boolean schema false (2020-12)` — `schema has no root`
- `contains.json::contains with false if subschema (2020-12)` — `schema has no root`
- `contains.json::contains with null instance elements (2020-12)` — `schema has no root`
- `contains.json::minContains without contains is ignored (2020-12)` — `schema has no root`
- `contains.json::minContains=1 with contains (2020-12)` — `schema has no root`
- `contains.json::minContains=2 with contains (2020-12)` — `schema has no root`
- `contains.json::minContains=2 with contains with a decimal value (2020-12)` — `schema has no root`
- `contains.json::maxContains = minContains (2020-12)` — `schema has no root`
- `contains.json::maxContains < minContains (2020-12)` — `schema has no root`
- `contains.json::minContains = 0 (2020-12)` — `schema has no root`
- `contains.json::minContains = 0 with no maxContains (2019-09)` — `schema has no root`
- `contains.json::minContains = 0 with maxContains (2020-12)` — `schema has no root`
- `contains.json::maxContains without contains is ignored (2020-12)` — `schema has no root`
- `contains.json::maxContains with contains (2020-12)` — `schema has no root`
- `contains.json::maxContains with contains, value with a decimal (2020-12)` — `schema has no root`
- `contains.json::minContains < maxContains (2020-12)` — `schema has no root`
- `contains.json::maxContains = 0 with minContains = 0 (2020-12)` — `schema has no root`
- `content.json::validation of string-encoded content based on media type (2020-12)` — `schema has no root`
- `content.json::validation of binary string-encoding (2020-12)` — `schema has no root`
- `content.json::validation of binary-encoded media type documents (2020-12)` — `schema has no root`
- `content.json::validation of binary-encoded media type documents with schema (2020-12)` — `schema has no root`
- `dependentRequired.json::single dependency (dependencies) (7)` — `schema has no root`
- `dependentRequired.json::empty dependents (dependencies) (7)` — `schema has no root`
- `dependentRequired.json::multiple dependents required (dependencies) (7)` — `schema has no root`
- `dependentRequired.json::dependencies with escaped characters (dependencies) (7)` — `schema has no root`
- `dependentSchemas.json::single dependency (dependencies) (7)` — `schema has no root`
- `dependentSchemas.json::boolean subschemas (dependencies) (7)` — `schema has no root`
- `dependentSchemas.json::dependents with escaped characters (dependencies) (7)` — `schema has no root`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `could not follow $ref "list" to file "list": cannot load schema`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (2020) (2020-12)` — `schema has no root`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `could not follow $ref "item" to file "item": cannot load schema`
- `dynamicRef.json::A $ref to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `cannot generate referenced type: value must point to definition within file: '#items'`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `cannot add struct field: could not generate type with scope 'ExtendedBar': could not follow $ref "bar" to file "bar": cannot load schema`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor (2020-12)` — `cannot add struct field: could not generate type with scope 'ExtendedBar': could not follow $ref "bar" to file "bar": cannot load schema`
- `dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (2020) (2020-12)` — `schema has no root`
- `id-legacy.json::base URI change - change folder (7)` — `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- `id-legacy.json::base URI change - change folder (id) (4)` — `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- `id-legacy.json::base URI change - change folder in subschema (7)` — `cannot add struct field: could not generate type with scope 'RootList': definition does not exist in schema: "baz/definitions/bar" (from ref "#/definitions/baz/…`
- `id-legacy.json::base URI change - change folder in subschema (id) (4)` — `cannot add struct field: could not generate type with scope 'RootList': definition does not exist in schema: "baz/definitions/bar" (from ref "#/definitions/baz/…`
- `id-legacy.json::URN ref with nested pointer ref (id) (4)` — `cannot add struct field: could not generate type with scope 'FooBar': definition does not exist in schema: "bar" (from ref "#/definitions/bar")`
- `id.json::base URI change - change folder (2020-12)` — `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- `id.json::base URI change - change folder in subschema (2020-12)` — `cannot add struct field: could not generate type with scope 'RootList': could not follow $ref "baseUriChangeFolderInSubschema/#/$defs/bar" to file "baseUriChang…`
- `id.json::order of evaluation: $id and $ref on nested schema (2020-12)` — `could not follow $ref "./bar.json" to file "./bar.json": cannot load schema`
- `id.json::URN ref with nested pointer ref (2020-12)` — `definition does not exist in schema: "bar" (from ref "#/$defs/bar")`
- `if-then-else.json::ignore if without then or else (2020-12)` — `schema has no root`
- … and 78 more

#### 🦀 `compile_fail` (356)

335 distinct first-line errors; most common:

- **2×** `g_0a70441cdda6c460/g_0a70441cdda6c460.go:9:8: undefined: Root`
- **2×** `g_0af2878ecdd01217/g_0af2878ecdd01217.go:9:8: undefined: Root`
- **2×** `g_0bed1ad10d1984ae/g_0bed1ad10d1984ae.go:9:8: undefined: Root`
- **2×** `g_1c10cd3ae3e713e5/g_1c10cd3ae3e713e5.go:11:8: undefined: Root`
- **2×** `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`

Per group:

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `g_c5735ec66ee06485/g_c5735ec66ee06485.go:9:8: undefined: Root`
- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `g_1e597764768f0a93/g_1e597764768f0a93.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties with schema (2020-12)` — `g_cafa094c406951dc/g_cafa094c406951dc.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties can exist by itself (2020-12)` — `g_077d92850537efc9/g_077d92850537efc9.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties are allowed by default (2020-12)` — `g_7b59774687a919db/g_7b59774687a919db.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties does not look in applicators (2020-12)` — `g_ddb75c735f7c4b1a/g_ddb75c735f7c4b1a.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties with null valued instance properties (2020-12)` — `g_5e530a6ed87d7426/g_5e530a6ed87d7426.go:9:8: undefined: Root`
- `additionalProperties.json::additionalProperties with propertyNames (2020-12)` — `g_ba5c324f80636c8a/g_ba5c324f80636c8a.go:9:8: undefined: Root`
- `additionalProperties.json::dependentSchemas with additionalProperties (2020-12)` — `g_cab8c37f83bef044/g_cab8c37f83bef044.go:9:8: undefined: Root`
- `allOf.json::allOf (2020-12)` — `g_e71e4d78180d36dd/g_e71e4d78180d36dd.go:9:8: undefined: Root`
- `allOf.json::allOf with base schema (2020-12)` — `g_f192e579f6973902/g_f192e579f6973902.go:9:8: undefined: Root`
- `allOf.json::allOf simple types (2020-12)` — `g_0e7058a7b9e9f3be/g_0e7058a7b9e9f3be.go:9:8: undefined: Root`
- `allOf.json::allOf with boolean schemas, all true (2020-12)` — `g_af0f90eb0721f351/g_af0f90eb0721f351.go:9:8: undefined: Root`
- `allOf.json::allOf with boolean schemas, some false (2020-12)` — `g_ff3db7dbaae62c5f/g_ff3db7dbaae62c5f.go:9:8: undefined: Root`
- `allOf.json::allOf with boolean schemas, all false (2020-12)` — `g_7caaeb6b1a5582e9/g_7caaeb6b1a5582e9.go:9:8: undefined: Root`
- `allOf.json::allOf with one empty schema (2020-12)` — `g_3a559a5b72af5ac0/g_3a559a5b72af5ac0.go:9:8: undefined: Root`
- `allOf.json::allOf with two empty schemas (2020-12)` — `g_f3f699a80cb69ebc/g_f3f699a80cb69ebc.go:9:8: undefined: Root`
- `allOf.json::allOf with the first empty schema (2020-12)` — `g_7f4915c0478e96f9/g_7f4915c0478e96f9.go:9:8: undefined: Root`
- `allOf.json::allOf with the last empty schema (2020-12)` — `g_aebfda358f575c9d/g_aebfda358f575c9d.go:9:8: undefined: Root`
- `allOf.json::nested allOf, to check validation semantics (2020-12)` — `g_c881a006028cbeaf/g_c881a006028cbeaf.go:9:8: undefined: Root`
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `g_db999cafabfbd80f/g_db999cafabfbd80f.go:9:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier (7)` — `g_09fe2c8c486b950a/g_09fe2c8c486b950a.go:11:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `g_ff7ef39c75d69f03/g_ff7ef39c75d69f03.go:11:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `g_d21b5c151461641d/g_d21b5c151461641d.go:11:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `g_c0407cd72e2aaf96/g_c0407cd72e2aaf96.go:11:8: undefined: Root`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `g_967599391e2d4fd5/g_967599391e2d4fd5.go:11:8: undefined: Root`
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `g_7058b7e625cfb223/g_7058b7e625cfb223.go:11:8: undefined: Root`
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `g_f4069ff1f1b77e00/g_f4069ff1f1b77e00.go:11:8: undefined: Root`
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `g_7746a57eab417e9d/g_7746a57eab417e9d.go:11:8: undefined: Root`
- `anchor-legacy.json::anchor within remote ref (7)` — `g_efb550b24c725af2/g_efb550b24c725af2.go:9:8: undefined: Root`
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `g_a95573f8d0e55ccb/g_a95573f8d0e55ccb.go:9:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `g_f73731d033c94837/g_f73731d033c94837.go:9:8: undefined: Root`
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `g_75f21fe3eee5ecac/g_75f21fe3eee5ecac.go:9:8: undefined: Root`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `g_cf882989e3c68b4e/g_cf882989e3c68b4e.go:9:8: undefined: Root`
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `g_cbd27f1e5ae1b9f3/g_cbd27f1e5ae1b9f3.go:9:8: undefined: Root`
- `anchor.json::Location-independent identifier (2020-12)` — `g_246e7a90d3c082e1/g_246e7a90d3c082e1.go:11:8: undefined: Root`
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `g_0ed2dd949d3aa806/g_0ed2dd949d3aa806.go:11:8: undefined: Root`
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `g_322d163e6f7fc139/g_322d163e6f7fc139.go:11:8: undefined: Root`
- `anchor.json::same $anchor with different base uri (2020-12)` — `g_59c195833d9e63cd/g_59c195833d9e63cd.go:11:8: undefined: Root`
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `g_66b4079e630b727d/g_66b4079e630b727d.go:11:8: undefined: Root`
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `g_3e5bd15b44b6f4ec/g_3e5bd15b44b6f4ec.go:11:8: undefined: Root`
- `anchor.json::anchor within remote ref (2020-12)` — `g_c43e5a78a9143458/g_c43e5a78a9143458.go:9:8: undefined: Root`
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `g_a97e74e4d6831149/g_a97e74e4d6831149.go:9:8: undefined: Root`
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `g_2d9fa248ed6e3b18/g_2d9fa248ed6e3b18.go:9:8: undefined: Root`
- `anyOf.json::anyOf (2020-12)` — `g_0d0db68fabed6b6f/g_0d0db68fabed6b6f.go:9:8: undefined: Root`
- `anyOf.json::anyOf with boolean schemas, all true (2020-12)` — `g_0a1b60215f3d45be/g_0a1b60215f3d45be.go:9:8: undefined: Root`
- `anyOf.json::anyOf with boolean schemas, some true (2020-12)` — `g_1b01dd21a320fd3f/g_1b01dd21a320fd3f.go:9:8: undefined: Root`
- `anyOf.json::anyOf with boolean schemas, all false (2020-12)` — `g_f416c92cde251fb8/g_f416c92cde251fb8.go:9:8: undefined: Root`
- `anyOf.json::anyOf complex types (2020-12)` — `g_6b8608af14790a01/g_6b8608af14790a01.go:9:8: undefined: Root`
- `anyOf.json::anyOf with one empty schema (2020-12)` — `g_1bfa5511972291db/g_1bfa5511972291db.go:9:8: undefined: Root`
- … and 306 more

#### 🤪 `mixed` (2)

- `type.json::multiple types can be specified in an array (2020-12)` — `an integer is valid: json: cannot unmarshal number into Go value of type g_5d7ec011b3ba22be.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `type.json::type: array or object (2020-12)` — `array is valid: json: cannot unmarshal array into Go value of type g_4fd47997f5844cf7.Root`
  - cases: array is valid (rejected), null is invalid (accepted)

#### 🚧 `false_reject` (2)

- `default.json::the default keyword does not do anything if the property is missing (2020-12)` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_c7901d42c855e895.Root`
  - cases: a float with zero fractional part is an integer (rejected)

#### 🙈 `false_accept` (38)

24 distinct first-line errors; most common:

- **7×** `accepted 'with unevaluated properties' though valid: false`
- **3×** `accepted 'when if is true and has unevaluated properties' though valid: false`
- **2×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **2×** `accepted 'base case: both properties present' though valid: false`
- **2×** `accepted 'object is invalid' though valid: false`

Per group:

- `anyOf.json::anyOf with base schema (2020-12)` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020) (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `items.json::items and subitems (2020-12)` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted)
- `oneOf.json::oneOf with base schema (2020-12)` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `oneOf.json::oneOf with required (2020-12)` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `patternProperties.json::patternProperties with Unicode property escape (2020-12)` — `accepted 'Unicode letter property name matches but value type is incorrect' though valid: false`
  - cases: Unicode letter property name matches but value type is incorrect (accepted)
- `ref-legacy.json::root ref in remote ref (7)` — `accepted 'object is invalid' though valid: false`
  - cases: object is invalid (accepted)
- `ref.json::root ref in remote ref (2020-12)` — `accepted 'object is invalid' though valid: false`
  - cases: object is invalid (accepted)
- `type.json::integer type matches integers (2020-12)` — `accepted 'null is not an integer' though valid: false`
  - cases: null is not an integer (accepted)
- `type.json::number type matches numbers (2020-12)` — `accepted 'null is not a number' though valid: false`
  - cases: null is not a number (accepted)
- `type.json::string type matches strings (2020-12)` — `accepted 'null is not a string' though valid: false`
  - cases: null is not a string (accepted)
- `type.json::object type matches objects (2020-12)` — `accepted 'null is not an object' though valid: false`
  - cases: null is not an object (accepted)
- `type.json::array type matches arrays (2020-12)` — `accepted 'null is not an array' though valid: false`
  - cases: null is not an array (accepted)
- `type.json::boolean type matches booleans (2020-12)` — `accepted 'null is not a boolean' though valid: false`
  - cases: null is not a boolean (accepted)
- `type.json::null type matches only the null object (2020-12)` — `accepted 'an integer is not null' though valid: false`
  - cases: an integer is not null (accepted), a float is not null (accepted), zero is not null (accepted), a string is not null (accepted), an empty string is not null (accepted), an object is not null (accepted), … and 3 more
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
| 10 | 7 | 6 | 0 | 0 | 58 | 17 | 0 | 98 | 10.2% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 43 | 69 | 62.3% | 1006 | 4.3% |

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
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 11 | 0 | 0 | 14 | 🚧✅✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| optional/content.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/cross-draft.json | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🚧✅🚧 |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format-regex.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/non-schemas.json | 0 | 0 | 0 | 0 | 0 | 7 | 6 | 0 | 13 | ❌❌❌🦀🦀❌❌❌🦀🦀🦀🦀🦀 |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **10** | **7** | **6** | **0** | **0** | **58** | **17** | **0** | **98** |  |

### Failures

#### ❌ `generation_error` (17)

7 distinct first-line errors; most common:

- **10×** `schema has no root`
- **2×** `enum has non-primitive value map[$id:https://localhost:1234/id/my_identifier.json type:null]`
- **1×** `could not follow $ref "item" to file "item": cannot load schema`
- **1×** `enum has non-primitive value map[$anchor:my_anchor type:null]`
- **1×** `enum has non-primitive value map[$id:#my_anchor type:null]`

Per group:

- `optional/content.json::validation of string-encoded content based on media type (7)` — `schema has no root`
- `optional/content.json::validation of binary string-encoding (7)` — `schema has no root`
- `optional/content.json::validation of binary-encoded media type documents (7)` — `schema has no root`
- `optional/dependencies-compatibility.json::single dependency (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::empty dependents (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::multiple dependents required (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::dependencies with escaped characters (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::single schema dependency (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::boolean subschemas (2020-12)` — `schema has no root`
- `optional/dependencies-compatibility.json::schema dependencies with escaped characters (2020-12)` — `schema has no root`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `could not follow $ref "item" to file "item": cannot load schema`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/$defs) (2020-12)` — `enum has non-primitive value map[$id:https://localhost:1234/id/my_identifier.json type:null]`
- `optional/non-schemas.json::$id inside an enum is not a real identifier ($id/definitions) (7)` — `enum has non-primitive value map[$id:https://localhost:1234/id/my_identifier.json type:null]`
- `optional/non-schemas.json::$id inside an enum is not a real identifier (id/definitions) (4)` — `enum has non-primitive value map[id:https://localhost:1234/my_identifier.json type:null]`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($anchor) (2020-12)` — `enum has non-primitive value map[$anchor:my_anchor type:null]`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier ($id) (7)` — `enum has non-primitive value map[$id:#my_anchor type:null]`
- `optional/non-schemas.json::anchor inside an enum is not a real identifier (id) (4)` — `enum has non-primitive value map[id:#my_anchor type:null]`

#### 🦀 `compile_fail` (58)

47 distinct first-line errors; most common:

- **7×** `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- **2×** `g_0a70441cdda6c460/g_0a70441cdda6c460.go:9:8: undefined: Root`
- **2×** `g_6334eca8743723cb/g_6334eca8743723cb.go:9:8: undefined: Root`
- **2×** `g_e4f4208a13d9bd48/g_e4f4208a13d9bd48.go:9:8: undefined: Root`
- **2×** `g_ec3f9b746484dcc3/g_ec3f9b746484dcc3.go:21:25: cannot use plain (variable of float64 type Plain) as float64 value in argument to math.Mod`

Per group:

- `format-date-time.json::validation of date-time strings (2020-12)` — `g_0bed1ad10d1984ae/g_0bed1ad10d1984ae.go:9:8: undefined: Root`
- `format-date.json::validation of date strings (2020-12)` — `g_a706e604ade06cac/g_a706e604ade06cac.go:9:8: undefined: Root`
- `format-duration.json::validation of duration strings (2020-12)` — `g_eb650cec3fb08419/g_eb650cec3fb08419.go:9:8: undefined: Root`
- `format-email.json::validation of e-mail addresses (2020-12)` — `g_0a70441cdda6c460/g_0a70441cdda6c460.go:9:8: undefined: Root`
- `format-email.json::validation of e-mail addresses against the RFC 5321 Mailbox rule (2020-12)` — `g_0a70441cdda6c460/g_0a70441cdda6c460.go:9:8: undefined: Root`
- `format-hostname.json::validation of host names (2020-12)` — `g_f709aa440ddddac4/g_f709aa440ddddac4.go:9:8: undefined: Root`
- `format-hostname.json::validation of host names without IDNA rules (6)` — `g_875a447979f4c34a/g_875a447979f4c34a.go:9:8: undefined: Root`
- `format-hostname.json::validation of A-label (punycode) host names (2020-12)` — `g_f709aa440ddddac4/g_f709aa440ddddac4.go:9:8: undefined: Root`
- `format-idn-email.json::validation of an internationalized e-mail addresses (2020-12)` — `g_b9cf9d7278084f15/g_b9cf9d7278084f15.go:9:8: undefined: Root`
- `format-idn-hostname.json::validation of internationalized host names (2020-12)` — `g_6334eca8743723cb/g_6334eca8743723cb.go:9:8: undefined: Root`
- `format-idn-hostname.json::validation of separators in internationalized host names (2020-12)` — `g_6334eca8743723cb/g_6334eca8743723cb.go:9:8: undefined: Root`
- `format-ipv4.json::validation of IP addresses (2020-12)` — `g_af3296113624745d/g_af3296113624745d.go:9:8: undefined: Root`
- `format-ipv6.json::validation of IPv6 addresses (2020-12)` — `g_ab64fe1cee11b181/g_ab64fe1cee11b181.go:9:8: undefined: Root`
- `format-iri-reference.json::validation of IRI References (2020-12)` — `g_0af2878ecdd01217/g_0af2878ecdd01217.go:9:8: undefined: Root`
- `format-iri.json::validation of IRIs (2020-12)` — `g_fc1cac2fb059b641/g_fc1cac2fb059b641.go:9:8: undefined: Root`
- `format-json-pointer.json::validation of JSON-pointers (JSON String Representation) (2020-12)` — `g_6d59aba3c53588e3/g_6d59aba3c53588e3.go:9:8: undefined: Root`
- `format-regex.json::validation of regular expressions (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `format-relative-json-pointer.json::validation of Relative JSON Pointers (RJP) (2020-12)` — `g_f25242752638c41f/g_f25242752638c41f.go:9:8: undefined: Root`
- `format-time.json::validation of time strings (2020-12)` — `g_e4f4208a13d9bd48/g_e4f4208a13d9bd48.go:9:8: undefined: Root`
- `format-uri-reference.json::validation of URI References (2020-12)` — `g_65d0a5e4edcfd875/g_65d0a5e4edcfd875.go:9:8: undefined: Root`
- `format-uri-template.json::validation of URI-Templates (2020-12)` — `g_53ba86c19daa36bc/g_53ba86c19daa36bc.go:9:8: undefined: Root`
- `format-uri.json::validation of URIs (2020-12)` — `g_9ca67e3acafe2e69/g_9ca67e3acafe2e69.go:9:8: undefined: Root`
- `format-uuid.json::uuid format (2020-12)` — `g_7b68bf652e5704c8/g_7b68bf652e5704c8.go:9:8: undefined: Root`
- `optional/bignum.json::maximum integer comparison (2020-12)` — `g_09bd9c4763974ad9/g_09bd9c4763974ad9.go:9:8: undefined: Root`
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `g_e1bf1302f53e619d/g_e1bf1302f53e619d.go:9:8: undefined: Root`
- `optional/bignum.json::float comparison with high precision (2020-12)` — `g_603b032589ae2a69/g_603b032589ae2a69.go:9:8: undefined: Root`
- `optional/bignum.json::minimum integer comparison (2020-12)` — `g_b46e983d93588436/g_b46e983d93588436.go:9:8: undefined: Root`
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `g_3c8a22c935dec77e/g_3c8a22c935dec77e.go:9:8: undefined: Root`
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `g_e3abdd01ed373805/g_e3abdd01ed373805.go:9:8: undefined: Root`
- `optional/bignum.json::enum comparison with high precision (2020-12)` — `g_cf3e8506f422af02/g_cf3e8506f422af02.go:9:8: undefined: Root`
- `optional/bignum.json::const comparison with high precision (2020-12)` — `g_09aefa7b9f5ce96c/g_09aefa7b9f5ce96c.go:9:8: undefined: Root`
- `optional/bignum.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `g_ec3f9b746484dcc3/g_ec3f9b746484dcc3.go:21:25: cannot use plain (variable of float64 type Plain) as float64 value in argument to math.Mod`
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `g_9eab10a3d7d2ad08/g_9eab10a3d7d2ad08.go:9:8: undefined: Root`
- `optional/bignum.json::multipleOf with integers above 2^53 (2020-12)` — `g_6608680d06556832/g_6608680d06556832.go:9:8: undefined: Root`
- `optional/ecmascript-regex.json::patterns always use unicode semantics with pattern (2020-12)` — `g_6fb40d8c9a91145b/g_6fb40d8c9a91145b.go:9:8: undefined: Root`
- `optional/ecmascript-regex.json::\w in patterns matches [A-Za-z0-9_], not unicode letters (2020-12)` — `g_18805e3dd9a5ba30/g_18805e3dd9a5ba30.go:9:8: undefined: Root`
- `optional/ecmascript-regex.json::pattern with ASCII ranges (2020-12)` — `g_b37be1be33e6b1ff/g_b37be1be33e6b1ff.go:9:8: undefined: Root`
- `optional/ecmascript-regex.json::\d in pattern matches [0-9], not unicode digits (2020-12)` — `g_7d7e1436afa47c98/g_7d7e1436afa47c98.go:9:8: undefined: Root`
- `optional/ecmascript-regex.json::pattern with non-ASCII digits (2020-12)` — `g_5a18b4fff0e5234f/g_5a18b4fff0e5234f.go:9:8: undefined: Root`
- `optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `g_ec3f9b746484dcc3/g_ec3f9b746484dcc3.go:21:25: cannot use plain (variable of float64 type Plain) as float64 value in argument to math.Mod`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `g_739aab1f136613f1/g_739aab1f136613f1.go:9:8: undefined: Root`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true (2020-12)` — `g_82c45f7b7e8f211e/g_82c45f7b7e8f211e.go:9:8: undefined: Root`
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-regex.json::global inline flag groups are not valid ECMA 262 (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-regex.json::ECMA 262 named groups and backreferences are valid (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-regex.json::ECMA 262 lookbehind is valid, including variable width (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-regex.json::ECMA 262 character classes and escapes (2020-12)` — `g_1ce2e62ba49e2d6d/g_1ce2e62ba49e2d6d.go:9:8: undefined: Root`
- `optional/format-time.json::validation of time strings (2020-12)` — `g_e4f4208a13d9bd48/g_e4f4208a13d9bd48.go:9:8: undefined: Root`
- `optional/non-bmp-regex.json::Proper UTF-16 surrogate pair handling: pattern (2020-12)` — `g_17b5500880e0cbff/g_17b5500880e0cbff.go:9:8: undefined: Root`
- … and 8 more

#### 🚧 `false_reject` (6)

6 distinct first-line errors; most common:

- **1×** `Line tabulation matches: field  pattern match: must match ^\s$`
- **1×** `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_c7901d42c855e895.Root`
- **1×** `first item is a string is valid: json: cannot unmarshal array into Go value of type g_9b0b814a04e3e281.Root`
- **1×** `first item not a string is valid: json: cannot unmarshal array into Go value of type g_3ba614f45426e578.Root`
- **1×** `matches: field  pattern match: must match ^\cC$`

Per group:

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_c7901d42c855e895.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `first item is a string is valid: json: cannot unmarshal array into Go value of type g_9b0b814a04e3e281.Root`
  - cases: first item is a string is valid (rejected)
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `first item not a string is valid: json: cannot unmarshal array into Go value of type g_3ba614f45426e578.Root`
  - cases: first item not a string is valid (rejected)
- `optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter (2020-12)` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter (2020-12)` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `optional/ecmascript-regex.json::ECMA 262 \s matches whitespace (2020-12)` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)

#### 🙈 `false_accept` (7)

5 distinct first-line errors; most common:

- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'literal unicode character in json string' though valid: false`
- **1×** `accepted 'Line tabulation does not match' though valid: false`
- **1×** `accepted 'missing bar is invalid' though valid: false`
- **1×** `accepted 'unicode matching is case-sensitive' though valid: false`

Per group:

- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `accepted 'missing bar is invalid' though valid: false`
  - cases: missing bar is invalid (accepted)
- `optional/ecmascript-regex.json::ECMA 262 \S matches everything but whitespace (2020-12)` — `accepted 'Line tabulation does not match' though valid: false`
  - cases: Line tabulation does not match (accepted), latin-1 non-breaking-space does not match (accepted), zero-width whitespace does not match (accepted), paragraph separator does not match (line terminator) (accepted), EM SPACE does not match (Space_Separator) (accepted)
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
| 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 0 | 0 | - | 34 | 0.0% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| **Total** | **0** | **0** | **0** | **0** | **0** | **5** | **0** | **0** | **5** |  |

### Failures

#### 🦀 `compile_fail` (5)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads inline (2020-12)` — `g_94ffcf31d372f737/g_94ffcf31d372f737.go:9:8: undefined: Root`
- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads by $ref (2020-12)` — `g_843b6373e23b50f2/g_843b6373e23b50f2.go:19:8: undefined: Root`
- `compound/externally-tagged-union.json::oneOf of objects with one required key each and additionalProperties false (2020-12)` — `g_04a324efb2aea652/g_04a324efb2aea652.go:19:8: undefined: Root`
- `compound/externally-tagged-union.json::oneOf mixing a unit variant and a data variant (2020-12)` — `g_fc504fa40c3825f8/g_fc504fa40c3825f8.go:9:8: undefined: Root`
- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `g_916acbd036c65c97/g_916acbd036c65c97.go:9:8: undefined: Root`

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | 0 | 0 | 0 | 15 | 9 | 0 | 25 | 0.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 1 | 2 | 50.0% | 90 | 1.1% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 8 | 3 | 0 | 12 | 🙈🦀❌🦀❌🦀🦀🦀🦀🦀❌🦀 |
| format-date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| proposals/propertyDependencies/additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| proposals/propertyDependencies/propertyDependencies.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| unevaluatedProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| **Total** | **0** | **1** | **0** | **0** | **0** | **15** | **9** | **0** | **25** |  |

### Failures

#### ❌ `generation_error` (9)

3 distinct first-line errors; most common:

- **6×** `schema has no root`
- **2×** `could not follow $ref "item" to file "item": cannot load schema`
- **1×** `could not follow $ref "list" to file "list": cannot load schema`

Per group:

- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `could not follow $ref "list" to file "list": cannot load schema`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `schema has no root`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `could not follow $ref "item" to file "item": cannot load schema`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `could not follow $ref "item" to file "item": cannot load schema`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `schema has no root`
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `schema has no root`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-objects (v1)` — `schema has no root`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-string property values (v1)` — `schema has no root`
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `schema has no root`

#### 🦀 `compile_fail` (15)

15 distinct first-line errors; most common:

- **1×** `g_06ea4463440466cb/g_06ea4463440466cb.go:9:8: undefined: Root`
- **1×** `g_2aad3cb46c870a54/g_2aad3cb46c870a54.go:9:8: undefined: Root`
- **1×** `g_4b76b6fcdb66ec40/g_4b76b6fcdb66ec40.go:9:8: undefined: Root`
- **1×** `g_54235078ea3f79a0/g_54235078ea3f79a0.go:9:8: undefined: Root`
- **1×** `g_582cca5de59d76cf/g_582cca5de59d76cf.go:13:8: undefined: Root`

Per group:

- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `g_582cca5de59d76cf/g_582cca5de59d76cf.go:13:8: undefined: Root`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `g_ef4280dae372374b/g_ef4280dae372374b.go:13:8: undefined: Root`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `g_897068878fa9077d/g_897068878fa9077d.go:9:8: undefined: Root`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `g_8a0d428d6b2671fa/g_8a0d428d6b2671fa.go:35:8: undefined: Root`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `g_06ea4463440466cb/g_06ea4463440466cb.go:9:8: undefined: Root`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `g_f92f8c30ec6f3e4d/g_f92f8c30ec6f3e4d.go:9:8: undefined: Root`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `g_c7b96f9a8f4b3e99/g_c7b96f9a8f4b3e99.go:9:8: undefined: Root`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `g_d5a19f549ac9c849/g_d5a19f549ac9c849.go:15:8: undefined: Root`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `g_731ee363111decbc/g_731ee363111decbc.go:9:8: undefined: Root`
- `format-time.json::leap seconds are not valid times (v1)` — `g_2aad3cb46c870a54/g_2aad3cb46c870a54.go:9:8: undefined: Root`
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `g_54235078ea3f79a0/g_54235078ea3f79a0.go:9:8: undefined: Root`
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `g_6219d3ec5af5d3a8/g_6219d3ec5af5d3a8.go:9:8: undefined: Root`
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `g_4b76b6fcdb66ec40/g_4b76b6fcdb66ec40.go:9:8: undefined: Root`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `g_9c49639909472337/g_9c49639909472337.go:13:8: undefined: Root`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `g_b3c1a6353012b543/g_b3c1a6353012b543.go:19:8: undefined: Root`

#### 🙈 `false_accept` (1)

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)

