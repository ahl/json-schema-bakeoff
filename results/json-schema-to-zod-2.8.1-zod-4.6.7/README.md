# json-schema-to-zod 2.8.1 + zod 4.6.7

Turns a JSON Schema into a zod schema expression, so you get a static
TypeScript type and a runtime parser from one artifact. Its dispatcher keys
on `type`: a typed schema picks up bounds, lengths, patterns, enums and
`const`, while a schema without `type` becomes `z.any()` whatever else it
says. It has no `$ref` resolver. Unmaintained since March 2026; 2.8.1 is the
last release, and it runs on zod 4 here.

_generated 2026-11-22T02:41:31Z_

**173 of 540 standard groups pass (32.0%)**

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
| 173 | 339 | 25 | 3 | 0 | 0 | 0 | 0 | 540 | 32.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 1137 | 1664 | 68.3% | 1664 | 68.3% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 8 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅✅✅✅✅✅✅✅🙈 |
| anchor-legacy.json | 0 | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| anchor.json | 0 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| anyOf.json | 5 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅🙈✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 11 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧🚧✅✅✅🚧🚧🚧🚧✅✅✅✅✅✅✅ |
| contains.json | 5 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈🙈🙈 |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| dependentRequired.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅🙈🙈🙈🙈 |
| dependentSchemas.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| dynamicRef.json | 4 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈✅🙈🙈 |
| enum.json | 12 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅✅✅ |
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
| id-legacy.json | 0 | 40 | 0 | 0 | 0 | 0 | 0 | 0 | 40 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| id.json | 0 | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 25 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| if-then-else.json | 5 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅✅🙈🚧🙈🙈 |
| infinite-loop-detection-legacy.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 10 | 16 | 0 | 2 | 0 | 0 | 0 | 0 | 28 | 🙈🙈🙈✅🙈🙈🙈🙈🤪🤪✅✅✅✅🙈✅🙈🙈✅✅✅🙈🙈🙈🙈🙈🙈✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| multipleOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| not.json | 7 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅🙈✅✅✅✅🚧 |
| oneOf.json | 6 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🤪🚧✅✅✅✅🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🚧🚧 |
| patternProperties.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| properties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈🙈🙈🙈🙈🙈🙈 |
| ref-legacy.json | 1 | 12 | 1 | 0 | 0 | 0 | 0 | 0 | 14 | 🙈🙈🙈🙈🙈🚧✅🙈🙈🙈🙈🙈🙈🙈 |
| ref.json | 1 | 21 | 1 | 0 | 0 | 0 | 0 | 0 | 23 | 🙈🙈🙈🙈🙈🙈🙈🙈🚧✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 10 | 35 | 2 | 0 | 0 | 0 | 0 | 0 | 47 | ✅🙈🙈✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈✅✅✅✅🙈🙈🚧🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈 |
| unevaluatedProperties.json | 9 | 34 | 2 | 0 | 0 | 0 | 0 | 0 | 45 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 3 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅🙈🙈🙈🙈✅✅🙈🙈 |
| vocabulary.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈✅ |
| **Total** | **173** | **339** | **25** | **3** | **0** | **0** | **0** | **0** | **540** |  |

### Failures

#### 🤪 `mixed` (3)

- `items.json::items and subitems (2019) (2019-09)` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `items.json::items and subitems (7) (7)` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `oneOf.json::oneOf (2020-12)` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)

#### 🚧 `false_reject` (25)

21 distinct first-line errors; most common:

- **2×** `[false] is valid: ZodError: <root>: Invalid input: expected false`
- **2×** `[true] is valid: ZodError: <root>: Invalid input: expected true`
- **2×** `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
- **2×** `with no unevaluated items: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
- **1×** `ASCII letters match: ZodError: <root>: Invalid string: must match pattern /^\p{Letter}+$/`

Per group:

- `const.json::const with object (2020-12)` — `same object is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same object is valid (rejected), same object with different property order is valid (rejected)
- `const.json::const with array (2020-12)` — `same array is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same array is valid (rejected)
- `const.json::const with [false] does not match [0] (2020-12)` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `const.json::const with [true] does not match [1] (2020-12)` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `const.json::const with {"a": false} does not match {"a": 0} (2020-12)` — `{"a": false} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": false} is valid (rejected)
- `const.json::const with {"a": true} does not match {"a": 1} (2020-12)` — `{"a": true} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": true} is valid (rejected)
- `enum.json::heterogeneous enum validation (2020-12)` — `one of the enum is valid: ZodError: <root>: Invalid input`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `enum.json::enum with [false] does not match [0] (2020-12)` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `enum.json::enum with [true] does not match [1] (2020-12)` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `enum.json::enum with [0] does not match [false] (2020-12)` — `[0] is valid: ZodError: <root>: Invalid input: expected 0`
  - cases: [0] is valid (rejected), [0.0] is valid (rejected)
- `enum.json::enum with [1] does not match [true] (2020-12)` — `[1] is valid: ZodError: <root>: Invalid input: expected 1`
  - cases: [1] is valid (rejected), [1.0] is valid (rejected)
- `if-then-else.json::if appears at the end when serialized (keyword processing sequence) (2020-12)` — `other redirects to else and passes: Cannot read properties of undefined (reading 'forEach')`
  - cases: other redirects to else and passes (rejected)
- `not.json::collect annotations inside a 'not', even if collection is disabled (2020-12)` — `unevaluated property: ZodError: <root>: Invalid input: Should NOT be valid against schema`
  - cases: unevaluated property (rejected)
- `oneOf.json::oneOf with base schema (2020-12)` — `one oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: one oneOf valid (rejected)
- `oneOf.json::oneOf complex types (2020-12)` — `first oneOf valid (complex): ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `oneOf.json::oneOf with required (2020-12)` — `first valid - valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `oneOf.json::oneOf with missing optional property (2020-12)` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `pattern.json::pattern with Unicode property escape requires unicode mode (2020-12)` — `ASCII letters match: ZodError: <root>: Invalid string: must match pattern /^\p{Letter}+$/`
  - cases: ASCII letters match (rejected), Non-ASCII letters match (rejected)
- `pattern.json::pattern with Unicode property escape (2020-12)` — `a valid case with a Unicode letter: ZodError: <root>: Invalid string: must match pattern /^\p{Letter}+$/`
  - cases: a valid case with a Unicode letter (rejected), another valid case with multiple Unicode letters (rejected)
- `ref-legacy.json::naive replacement of $ref with its destination is not correct (7)` — `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: match the enum exactly (rejected)
- `ref.json::naive replacement of $ref with its destination is not correct (2020-12)` — `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: match the enum exactly (rejected)
- `unevaluatedItems.json::unevaluatedItems with oneOf (prefixItems) (2020-12)` — `with no unevaluated items: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: with no unevaluated items (rejected)
- `unevaluatedItems.json::unevaluatedItems with oneOf (2019-09)` — `with no unevaluated items: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: with no unevaluated items (rejected)
- `unevaluatedProperties.json::unevaluatedProperties + ref inside allOf / oneOf (2020-12)` — `a and x are valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: a and x are valid (rejected), a and y are valid (rejected), a and b and x are valid (rejected), a and b and y are valid (rejected)
- `unevaluatedProperties.json::dynamic evaluation inside nested refs (2020-12)` — `a is valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: a is valid (rejected), b is valid (rejected), c is valid (rejected), d is valid (rejected), xx is valid (rejected), xx + foox is valid (rejected), … and 2 more

#### 🙈 `false_accept` (339)

153 distinct first-line errors; most common:

- **21×** `accepted 'a non-string is invalid' though valid: false`
- **15×** `accepted 'non-number is invalid' though valid: false`
- **14×** `accepted 'with unevaluated items' though valid: false`
- **13×** `accepted 'mismatch' though valid: false`
- **11×** `accepted 'with unevaluated properties' though valid: false`

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
- `allOf.json::allOf combined with anyOf, oneOf (2020-12)` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `anchor-legacy.json::Location-independent identifier (7)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::Location-independent identifier (id) (4)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (7)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::Location-independent identifier with base URI change in subschema (id) (4)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::Reference an anchor with a non-relative URI (7)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::Reference an anchor with a non-relative URI (id) (4)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor-legacy.json::URN base URI with URN and anchor ref (7)` — `accepted 'a non-string is invalid' though valid: false`
  - cases: a non-string is invalid (accepted)
- `anchor-legacy.json::URN base URI with URN and anchor ref (id) (4)` — `accepted 'a non-string is invalid' though valid: false`
  - cases: a non-string is invalid (accepted)
- `anchor-legacy.json::anchor within remote ref (7)` — `accepted 'remote anchor invalid' though valid: false`
  - cases: remote anchor invalid (accepted)
- `anchor-legacy.json::anchor within remote ref (id) (4)` — `accepted 'remote anchor invalid' though valid: false`
  - cases: remote anchor invalid (accepted)
- `anchor-legacy.json::Location-independent identifier in remote ref (7)` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `anchor-legacy.json::Location-independent identifier in remote ref (id) (4)` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `anchor-legacy.json::$ref to $ref finds detached $anchor (7)` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `anchor-legacy.json::$ref to $ref finds detached $anchor (id) (4)` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `anchor.json::Location-independent identifier (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor.json::Location-independent identifier with absolute URI (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor.json::Location-independent identifier with base URI change in subschema (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor.json::same $anchor with different base uri (2020-12)` — `accepted '$ref does not resolve to /$defs/A/allOf/0' though valid: false`
  - cases: $ref does not resolve to /$defs/A/allOf/0 (accepted)
- `anchor.json::Reference an anchor with a non-relative URI (2020-12)` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `anchor.json::URN base URI with URN and anchor ref (2020-12)` — `accepted 'a non-string is invalid' though valid: false`
  - cases: a non-string is invalid (accepted)
- `anchor.json::anchor within remote ref (2020-12)` — `accepted 'remote anchor invalid' though valid: false`
  - cases: remote anchor invalid (accepted)
- `anchor.json::Location-independent identifier in remote ref (2020-12)` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `anchor.json::$ref to $ref finds detached $anchor (2020-12)` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `anyOf.json::anyOf (2020-12)` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `anyOf.json::anyOf with base schema (2020-12)` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), both anyOf invalid (accepted)
- `anyOf.json::anyOf complex types (2020-12)` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
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
- … and 289 more

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | 71 | 5 | 0 | 0 | 0 | 0 | 0 | 98 | 22.4% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 432 | 1006 | 42.9% | 1006 | 42.9% |

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
| optional/bignum.json | 2 | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 14 | 🚧✅✅🙈🙈🙈🙈🙈🙈🙈🙈🚧🙈🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/non-schemas.json | 0 | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 13 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| optional/zero-terminated-floats.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **22** | **71** | **5** | **0** | **0** | **0** | **0** | **0** | **98** |  |

### Failures

#### 🚧 `false_reject` (5)

- `optional/bignum.json::integer (2020-12)` — `a bignum is an integer: ZodError: <root>: Too big: expected int to be <=9007199254740991`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `optional/bignum.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `valid if optional overflow handling is implemented: ZodError: <root>: Invalid number: must be a multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties (2020-12)` — `ascii character in json string: ZodError: l'ecole: Invalid input: must match catchall schema`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `optional/ecmascript-regex.json::patternProperties with non-ASCII digits (2020-12)` — `ascii digits: ZodError: 42: Invalid input: must match catchall schema`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `valid if optional overflow handling is implemented: ZodError: <root>: Invalid number: must be a multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)

#### 🙈 `false_accept` (71)

58 distinct first-line errors; most common:

- **5×** `accepted 'mismatch' though valid: false`
- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **2×** `accepted 'ascii non-digits' though valid: false`
- **2×** `accepted 'comparison works for high numbers' though valid: false`
- **2×** `accepted 'comparison works for very negative numbers' though valid: false`

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
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `accepted 'format-assertion: false: invalid string' though valid: false`
  - cases: format-assertion: false: invalid string (accepted)
- … and 21 more

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
| compound/externally-tagged-union.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈✅✅✅ |
| **Total** | **4** | **1** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

#### 🙈 `false_accept` (1)

- `compound/externally-tagged-union.json::oneOf of objects with one required key each, payloads by $ref (2020-12)` — `accepted 'a non-number radius is invalid' though valid: false`
  - cases: a non-number radius is invalid (accepted)

## Upcoming

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 3 | 22 | 0 | 0 | 0 | 0 | 0 | 0 | 25 | 12.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 46 | 90 | 51.1% | 90 | 51.1% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| dynamicRef.json | 1 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🙈🙈 |
| format-date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/additionalProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| proposals/propertyDependencies/dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| proposals/propertyDependencies/propertyDependencies.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| proposals/propertyDependencies/unevaluatedProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| unevaluatedItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| unevaluatedProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **3** | **22** | **0** | **0** | **0** | **0** | **0** | **0** | **25** |  |

### Failures

#### 🙈 `false_accept` (22)

17 distinct first-line errors; most common:

- **3×** `accepted 'An array containing non-strings is invalid' though valid: false`
- **3×** `accepted 'incorrect parent schema' though valid: false`
- **2×** `accepted 'string property fails' though valid: false`
- **1×** `accepted 'a leap second is not a valid time' though valid: false`
- **1×** `accepted 'additionalProperties doesn't consider propertyDependencies properties' though valid: false`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `accepted 'An array containing non-strings is invalid' though valid: false`
  - cases: An array containing non-strings is invalid (accepted)
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `accepted 'string matches /$defs/thingy, but the $dynamicRef does not stop here' though valid: false`
  - cases: string matches /$defs/thingy, but the $dynamicRef does not stop here (accepted), first_scope is not in dynamic scope for the $dynamicRef (accepted)
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `accepted 'instance with misspelled field' though valid: false`
  - cases: instance with misspelled field (accepted)
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `accepted 'incorrect parent schema' though valid: false`
  - cases: incorrect parent schema (accepted), incorrect extended schema (accepted)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `accepted 'incorrect parent schema' though valid: false`
  - cases: incorrect parent schema (accepted), incorrect extended schema (accepted)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `accepted 'incorrect parent schema' though valid: false`
  - cases: incorrect parent schema (accepted), incorrect extended schema (accepted)
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `accepted 'data is not sufficient for schema at second#/$defs/length' though valid: false`
  - cases: data is not sufficient for schema at second#/$defs/length (accepted)
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `accepted 'an invalid date-time-ext with an empty suffix' though valid: false`
  - cases: an invalid date-time-ext with an empty suffix (accepted), an invalid date-time-ext with an unclosed time-zone suffix (accepted), an invalid date-time-ext with a 15 character time-zone-part (accepted), an invalid date-time-ext with a "." time-zone-part (accepted), an invalid date-time-ext with an uppercase suffix-key (accepted), an invalid date-time-ext with two time-zone suffixes (accepted), … and 2 more
- `format-time.json::leap seconds are not valid times (v1)` — `accepted 'a leap second is not a valid time' though valid: false`
  - cases: a leap second is not a valid time (accepted), a leap second with a positive time-offset is not a valid time (accepted), a leap second with a negative time-offset is not a valid time (accepted)
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `accepted 'string property fails' though valid: false`
  - cases: string property fails (accepted)
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

