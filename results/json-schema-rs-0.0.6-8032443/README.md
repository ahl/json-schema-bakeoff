# json-schema-rs 0.0.6+8032443

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

_generated 2026-11-22T03:39:29Z_

**304 of 540 standard groups pass (56.3%)**

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
| 304 | 180 | 42 | 5 | 0 | 1 | 8 | 0 | 540 | 56.3% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 1175 | 1638 | 71.7% | 1664 | 70.6% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 6 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 9 | 🤪🙈✅✅✅🙈✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor-legacy.json | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🦀✅✅✅ |
| contains.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈🙈🙈 |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈✅🙈✅✅🙈✅🙈🙈 |
| enum.json | 16 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| format-date-time.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-date.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-duration.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-email.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-hostname.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-idn-email.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-idn-hostname.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-ipv4.json | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧🚧 |
| format-ipv6.json | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧🚧 |
| format-iri-reference.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-iri.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-json-pointer.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-regex.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-relative-json-pointer.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-time.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| format-uri-reference.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-uri-template.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-uri.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| format-uuid.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| id-legacy.json | 35 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 40 | ❌✅✅✅✅✅✅❌❌❌❌✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| id.json | 22 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 25 | ❌✅✅❌❌✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 12 | 15 | 1 | 0 | 0 | 0 | 0 | 0 | 28 | 🚧🙈🙈✅🙈🙈🙈🙈🙈🙈✅✅✅✅✅✅🙈🙈✅✅✅✅🙈🙈🙈🙈🙈✅ |
| maxItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxLength.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 2 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🚧✅🚧✅🚧 |
| minItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minLength.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 1 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 5 | 🚧🚧✅🚧🚧 |
| multipleOf.json | 5 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 6 | 🚧✅✅✅✅✅ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 9 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅🙈✅🙈✅✅✅✅✅✅ |
| pattern.json | 3 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 4 | 🚧✅✅✅ |
| patternProperties.json | 1 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | 🤪🙈🙈🙈✅🙈 |
| properties.json | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 6 | 🚧🙈🙈✅✅🚧 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅🚧✅✅✅✅🙈🙈🙈 |
| ref-legacy.json | 13 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 🙈✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| ref.json | 19 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 23 | 🚧✅🙈✅✅🙈✅✅✅✅✅✅✅🙈✅✅✅✅✅✅✅✅✅ |
| required.json | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🤪 |
| type.json | 10 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 12 | ✅🚧✅✅✅✅✅✅✅✅✅🚧 |
| unevaluatedItems.json | 13 | 34 | 0 | 0 | 0 | 0 | 0 | 0 | 47 | ✅🙈🙈✅🙈🙈✅✅✅🙈🙈🙈🙈🙈✅✅✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈✅✅🙈🙈🙈🙈 |
| unevaluatedProperties.json | 15 | 30 | 0 | 0 | 0 | 0 | 0 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈✅✅✅🙈✅🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 3 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅🙈🙈🙈🙈✅✅🙈🙈 |
| vocabulary.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈✅ |
| **Total** | **304** | **180** | **42** | **5** | **0** | **1** | **8** | **0** | **540** |  |

### Failures

#### ❌ `generation_error` (8)

3 distinct first-line errors; most common:

- **4×** `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- **2×** `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`
- **2×** `schema at index 0: could not resolve $ref "folderInteger.json": $ref "folderInteger.json" is not in this schema's registry (the schema is not part of it)`

Per group:

- `id-legacy.json::base URI change (4)` — `schema at index 0: could not resolve $ref "folderInteger.json": $ref "folderInteger.json" is not in this schema's registry (the schema is not part of it)`
- `id-legacy.json::refs with relative uris and defs (7)` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `id-legacy.json::refs with relative uris and defs (id) (4)` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `id-legacy.json::relative refs with absolute uris and defs (7)` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `id-legacy.json::relative refs with absolute uris and defs (id) (4)` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `id.json::base URI change (2020-12)` — `schema at index 0: could not resolve $ref "folderInteger.json": $ref "folderInteger.json" is not in this schema's registry (the schema is not part of it)`
- `id.json::refs with relative uris and defs (2020-12)` — `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`
- `id.json::relative refs with absolute uris and defs (2020-12)` — `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`

#### 🦀 `compile_fail` (1)

- `const.json::float and integers are equal up to 64-bit representation limits (2020-12)` — `error: literal out of range for 'i32'`

#### 🤪 `mixed` (5)

- `additionalProperties.json::additionalProperties being false does not allow other properties (2020-12)` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `maxItems.json::maxItems validation (2020-12)` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `minItems.json::minItems validation (2020-12)` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `patternProperties.json::patternProperties validates properties matching a regex (2020-12)` — `accepted 'a single invalid match is invalid' though valid: false`
  - cases: a single invalid match is invalid (accepted), multiple invalid matches is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `required.json::required properties whose names are Javascript object property names (2020-12)` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)

#### 🚧 `false_reject` (42)

18 distinct first-line errors; most common:

- **17×** `all string formats ignore integers: invalid type: integer '12', expected a string`
- **7×** `ignores non-numbers: invalid type: string "x", expected f64`
- **3×** `ignores arrays: invalid type: sequence, expected struct Root`
- **1×** `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
- **1×** `all string formats ignore integers: invalid type: integer '12', expected IPv4 address`

Per group:

- `enum.json::heterogeneous enum-with-null validation (2020-12)` — `null is valid: value null not in enum (allowed: 6)`
  - cases: null is valid (rejected)
- `format-date-time.json::validation of non-string date-time values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-date.json::validation of non-string date values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-duration.json::validation of non-string duration values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-email.json::validation of non-string email values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-hostname.json::validation of non-string hostname values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-idn-email.json::validation of non-string idn-email values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-idn-hostname.json::validation of non-string idn-hostname values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-ipv4.json::validation of non-string IP addresses (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected IPv4 address`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-ipv4.json::ipv4 format annotation (2020-12)` — `invalid ipv4 string is only an annotation by default: invalid IPv4 address syntax`
  - cases: invalid ipv4 string is only an annotation by default (rejected)
- `format-ipv6.json::validation of non-string ipv6 values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected IPv6 address`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-ipv6.json::ipv6 format (2020-12)` — `invalid ipv6 string is only an annotation by default: invalid IPv6 address syntax`
  - cases: invalid ipv6 string is only an annotation by default (rejected)
- `format-iri-reference.json::validation of non-string iri-reference values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-iri.json::validation of non-string iri values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-json-pointer.json::validation of non-string json-pointer values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-regex.json::validation of non-string regex values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-relative-json-pointer.json::validation of non-string relative-json-pointer values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-time.json::validation of non-string time values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-unknown.json::unknown format (2020-12)` — `unknown formats ignore integers: invalid type: integer '12', expected a string`
  - cases: unknown formats ignore integers (rejected), unknown formats ignore floats (rejected), unknown formats ignore objects (rejected), unknown formats ignore arrays (rejected), unknown formats ignore booleans (rejected), unknown formats ignore nulls (rejected)
- `format-uri-reference.json::validation of non-string uri-reference values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-uri-template.json::validation of non-string uri-template values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-uri.json::validation of non-string uri values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `format-uuid.json::validation of non-string uuid values (2020-12)` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `items.json::a schema given for items (2020-12)` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `maxLength.json::maxLength validation (2020-12)` — `ignores non-strings: invalid type: integer '100', expected a string`
  - cases: ignores non-strings (rejected)
- `maximum.json::maximum validation (2020-12)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `maximum.json::maximum validation (explicit false exclusivity) (4)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `maximum.json::exclusiveMaximum validation (2020-12)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `minLength.json::minLength validation (2020-12)` — `ignores non-strings: invalid type: integer '1', expected a string`
  - cases: ignores non-strings (rejected)
- `minimum.json::minimum validation (2020-12)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `minimum.json::minimum validation (explicit false exclusivity) (4)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `minimum.json::minimum validation with signed integer (2020-12)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `minimum.json::exclusiveMinimum validation (2020-12)` — `ignores non-numbers: invalid type: string "x", expected f64`
  - cases: ignores non-numbers (rejected)
- `multipleOf.json::multipleOf int (2020-12)` — `ignores non-numbers: invalid type: string "foo", expected f64`
  - cases: ignores non-numbers (rejected)
- `pattern.json::pattern validation (2020-12)` — `ignores booleans: invalid type: boolean 'true', expected a string`
  - cases: ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), ignores null (rejected)
- `properties.json::object properties validation (2020-12)` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `properties.json::properties whose names are Javascript object property names (2020-12)` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `recursiveRef.json::$recursiveRef with nesting (2019-09)` — `integer now matches as a property value: no variant of the anyOf union matched`
  - cases: integer now matches as a property value (rejected), two levels, properties match with $recursiveRef (rejected)
- `ref.json::root pointer ref (2020-12)` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected)
- `required.json::required validation (2020-12)` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `type.json::integer type matches integers (6) (2020-12)` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `type.json::type: array, object or null (2020-12)` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)

#### 🙈 `false_accept` (180)

100 distinct first-line errors; most common:

- **16×** `accepted 'with unevaluated items' though valid: false`
- **10×** `accepted 'with unevaluated properties' though valid: false`
- **5×** `accepted 'empty data' though valid: false`
- **4×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **4×** `accepted '[false, false] from items array is not valid' though valid: false`

Per group:

- `additionalProperties.json::non-ASCII pattern with additionalProperties (2020-12)` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `additionalProperties.json::additionalProperties does not look in applicators (2020-12)` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `contains.json::contains keyword validation (2020-12)` — `accepted 'array without items matching schema is invalid' though valid: false`
  - cases: array without items matching schema is invalid (accepted), empty array is invalid (accepted)
- `contains.json::contains keyword with const keyword (2020-12)` — `accepted 'array without item 5 is invalid' though valid: false`
  - cases: array without item 5 is invalid (accepted)
- `contains.json::contains keyword with boolean schema true (2020-12)` — `accepted 'empty array is invalid' though valid: false`
  - cases: empty array is invalid (accepted)
- `contains.json::contains keyword with boolean schema false (2020-12)` — `accepted 'any non-empty array is invalid' though valid: false`
  - cases: any non-empty array is invalid (accepted), empty array is invalid (accepted)
- `contains.json::items + contains (2020-12)` — `accepted 'matches items, does not match contains' though valid: false`
  - cases: matches items, does not match contains (accepted)
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
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (2020) (2020-12)` — `accepted 'instance with misspelled field' though valid: false`
  - cases: instance with misspelled field (accepted)
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `accepted 'incorrect extended schema' though valid: false`
  - cases: incorrect extended schema (accepted)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `accepted 'incorrect extended schema' though valid: false`
  - cases: incorrect extended schema (accepted)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `accepted 'incorrect extended schema' though valid: false`
  - cases: incorrect extended schema (accepted)
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (2020) (2020-12)` — `accepted 'data is not sufficient for schema at second#/$defs/length' though valid: false`
  - cases: data is not sufficient for schema at second#/$defs/length (accepted)
- `dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `accepted 'An array of strings is invalid' though valid: false`
  - cases: An array of strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `accepted 'The recursive part is not valid against the root' though valid: false`
  - cases: The recursive part is not valid against the root (accepted)
- `dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (2020) (2020-12)` — `accepted 'number list with string values' though valid: false`
  - cases: number list with string values (accepted), string list with number values (accepted)
- `dynamicRef.json::$dynamicRef points to a boolean schema (2020-12)` — `accepted 'follow $dynamicRef to a false schema' though valid: false`
  - cases: follow $dynamicRef to a false schema (accepted)
- `if-then-else.json::if and then without else (2020-12)` — `accepted 'invalid through then' though valid: false`
  - cases: invalid through then (accepted)
- `if-then-else.json::if and else without then (2020-12)` — `accepted 'invalid through else' though valid: false`
  - cases: invalid through else (accepted)
- … and 130 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 46 | 46 | 6 | 0 | 0 | 0 | 0 | 0 | 98 | 46.9% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 523 | 1006 | 52.0% | 1006 | 52.0% |

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
| format-ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format-ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
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
| optional/bignum.json | 4 | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 14 | 🚧✅✅🚧🚧🚧🙈🚧🚧🙈🙈✅🙈✅ |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅🙈 |
| optional/non-schemas.json | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 13 | ✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/zero-terminated-floats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **46** | **46** | **6** | **0** | **0** | **0** | **0** | **0** | **98** |  |

### Failures

#### 🚧 `false_reject` (6)

4 distinct first-line errors; most common:

- **2×** `above the exclusive minimum is valid: value -9.727837981879871e+26 is not above exclusiveMinimum -9.727837981879871e+26`
- **2×** `below the exclusive maximum is valid: value 9.727837981879871e+26 is not below exclusiveMaximum 9.727837981879871e+26`
- **1×** `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
- **1×** `comparison works for high numbers: value 1.8446744073709552e+19 is above maximum 18446744073709551615`

Per group:

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `optional/bignum.json::maximum integer comparison (2020-12)` — `comparison works for high numbers: value 1.8446744073709552e+19 is above maximum 18446744073709551615`
  - cases: comparison works for high numbers (rejected)
- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `below the exclusive maximum is valid: value 9.727837981879871e+26 is not below exclusiveMaximum 9.727837981879871e+26`
  - cases: below the exclusive maximum is valid (rejected)
- `optional/bignum.json::float comparison with high precision (2020-12)` — `below the exclusive maximum is valid: value 9.727837981879871e+26 is not below exclusiveMaximum 9.727837981879871e+26`
  - cases: below the exclusive maximum is valid (rejected)
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `above the exclusive minimum is valid: value -9.727837981879871e+26 is not above exclusiveMinimum -9.727837981879871e+26`
  - cases: above the exclusive minimum is valid (rejected)
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `above the exclusive minimum is valid: value -9.727837981879871e+26 is not above exclusiveMinimum -9.727837981879871e+26`
  - cases: above the exclusive minimum is valid (rejected)

#### 🙈 `false_accept` (46)

42 distinct first-line errors; most common:

- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'literal unicode character in json string' though valid: false`
- **2×** `accepted 'missing dependency' though valid: false`
- **1×** `accepted 'CRLF missing dependent' though valid: false`

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
- `optional/bignum.json::minimum integer comparison (2020-12)` — `accepted 'just below the minimum is invalid' though valid: false`
  - cases: just below the minimum is invalid (accepted)
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
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `accepted 'missing bar is invalid' though valid: false`
  - cases: missing bar is invalid (accepted)
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `accepted 'first item not a string is invalid' though valid: false`
  - cases: first item not a string is invalid (accepted)
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
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
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
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `optional/format-regex.json::global inline flag groups are not valid ECMA 262 (2020-12)` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `optional/format-time.json::validation of time strings (2020-12)` — `accepted 'invalid leap second, Zulu (wrong hour)' though valid: false`
  - cases: invalid leap second, Zulu (wrong hour) (accepted), invalid leap second, Zulu (wrong minute) (accepted), invalid leap second, zero time-offset (wrong hour) (accepted), invalid leap second, zero time-offset (wrong minute) (accepted), invalid leap second, positive time-offset (wrong hour) (accepted), invalid leap second, positive time-offset (wrong minute) (accepted), … and 2 more
- `optional/non-bmp-regex.json::Proper UTF-16 surrogate pair handling: patternProperties (2020-12)` — `accepted 'doesn't match one' though valid: false`
  - cases: doesn't match one (accepted), doesn't match two (accepted)

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 80.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 33 | 34 | 97.1% | 34 | 97.1% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| **Total** | **4** | **1** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

#### 🙈 `false_accept` (1)

- `compound/externally-tagged-union.json::a string enum sent the map form of a unit variant (2020-12)` — `accepted 'a member in map form with null is invalid' though valid: false`
  - cases: a member in map form with null is invalid (accepted)

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

1 distinct first-line errors; most common:

- **25×** `no json-schema-rs SpecVersion for dialect "v1"`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `format-time.json::leap seconds are not valid times (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-objects (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-string property values (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `no json-schema-rs SpecVersion for dialect "v1"`

