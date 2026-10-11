# typify 0.3.0

typify 0.3.0, from December 2024: the version most typify users were on
before the 0.4 through 0.8 releases and well before the rewrite. Same design
center; it's here to show how much the last two years moved.

_generated 2026-10-11T19:26:33Z_

**137 of 393 non-optional groups pass (34.9%)**; 83 optional groups reported separately

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

## Core

### Groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 59 | 16 | 14 | 6 | 0 | 3 | 19 | 43 | 160 | 36.9% |
| draft6 | 78 | 37 | 17 | 6 | 0 | 3 | 28 | 64 | 233 | 33.5% |
| **Total** | **137** | **53** | **31** | **12** | **0** | **6** | **47** | **107** | **393** | **34.9%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 22 | 0 | 5 | 0 | 0 | 0 | 2 | 10 | 39 | 56.4% |
| draft6 | 20 | 2 | 6 | 0 | 0 | 0 | 0 | 16 | 44 | 45.5% |
| **Total** | **42** | **2** | **11** | **0** | **0** | **0** | **2** | **26** | **83** | **50.6%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 303 | 391 | 77.5% | 618 | 49.0% |
| draft6 | 403 | 538 | 74.9% | 841 | 47.9% |
| **Total** | **706** | **929** | **76.0%** | **1459** | **48.4%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 81 | 93 | 87.1% | 391 | 20.7% |
| draft6 | 81 | 96 | 84.4% | 520 | 15.6% |
| **Total** | **162** | **189** | **85.7%** | **911** | **17.8%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 2 | 0 | 1 | 0 | 0 | 0 | 6 | 0 | 9 | ❌✅❌🚧❌❌❌❌✅ |
| additionalProperties.json | 6 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅✅✅✅ |
| allOf.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 9 | ✅✅🤯✅✅✅✅✅🤯 |
| anyOf.json | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 1 | 5 | 🤯🚧🙈🚧✅ |
| default.json | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | ❌✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 12 | 0 | 2 | 0 | 0 | 1 | 0 | 1 | 16 | ✅🤯✅✅🦀✅✅✅✅✅🚧✅🚧✅✅✅ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 6 | 🤯🤯🤯🤯🤯🤯 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 2 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 6 | 🚧❌❌✅✅❌ |
| maxItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| maxLength.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| maxProperties.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 4 | 🤯🤯❌❌ |
| minItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| minLength.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| minProperties.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 4 | 🤯❌❌🤯 |
| multipleOf.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 5 | 🤯🤯🤯✅✅ |
| not.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 6 | 🤯🤯🤯🤯🤯🤯 |
| oneOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 1 | 7 | 🤯🙈✅🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| patternProperties.json | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 4 | 🚧🙈🙈✅ |
| properties.json | 1 | 0 | 2 | 1 | 0 | 1 | 0 | 0 | 5 | 🚧🤪🦀✅🚧 |
| ref.json | 7 | 1 | 0 | 0 | 0 | 0 | 1 | 10 | 19 | 🤯🤯❌🤯✅✅🤯🤯✅✅🤯🤯🤯🤯✅🤯✅✅🙈 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 8 | 🤯🤯🤯🤯🤯🤯🤯🤯 |
| required.json | 1 | 0 | 2 | 0 | 0 | 1 | 0 | 0 | 4 | 🚧✅🦀🚧 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 1 | 1 | 0 | 0 | 0 | 0 | 4 | 0 | 6 | 🙈❌❌✅❌❌ |
| **Total** | **59** | **16** | **14** | **6** | **0** | **3** | **19** | **43** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 0 | 2 | 2 | 7 | 🚧✅✅🤯❌🤯❌ |
| optional/ecmascript-regex.json | 16 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🚧✅✅✅🚧🚧✅✅✅🚧 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **22** | **0** | **5** | **0** | **0** | **0** | **2** | **10** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 3 | 0 | 1 | 0 | 0 | 0 | 6 | 0 | 10 | ❌✅✅❌🚧❌❌❌❌✅ |
| additionalProperties.json | 6 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅✅✅✅ |
| allOf.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 12 | ✅✅🤯✅✅✅✅✅✅✅✅🤯 |
| anyOf.json | 1 | 1 | 3 | 0 | 0 | 0 | 0 | 3 | 8 | 🤯🚧🚧🤯🤯🙈🚧✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| default.json | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | ❌✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 11 | 0 | 2 | 0 | 0 | 1 | 0 | 1 | 15 | ✅🤯✅✅🦀✅✅✅✅✅🚧✅🚧✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 9 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 0 | 1 | 0 | 0 | 0 | 4 | 0 | 9 | 🚧❌✅✅❌❌✅✅❌ |
| maxItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| maxLength.json | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🚧❌ |
| maxProperties.json | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 3 | 🤪❌🙈 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| minItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| minLength.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| minProperties.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 2 | 🤪❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 🤯🤯 |
| multipleOf.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 5 | 🤯🤯🤯✅✅ |
| not.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 8 | 🤯🤯🤯🤯🤯✅✅🤯 |
| oneOf.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 5 | 11 | 🤯🙈🤯🤯🤯🤯✅🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| patternProperties.json | 1 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 5 | 🚧🙈🙈🙈✅ |
| properties.json | 1 | 1 | 2 | 1 | 0 | 1 | 0 | 0 | 6 | 🚧🤪🙈🦀✅🚧 |
| propertyNames.json | 3 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 6 | 🚧✅✅🤯🙈✅ |
| ref.json | 13 | 1 | 0 | 0 | 0 | 0 | 1 | 16 | 31 | 🤯🤯❌🤯✅✅🤯🤯✅✅✅✅🤯🤯🤯🤯🤯✅🤯🤯🤯✅✅✅✅🤯🤯🤯✅✅🙈 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 11 | 🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| required.json | 2 | 0 | 2 | 0 | 0 | 1 | 0 | 0 | 5 | 🚧✅✅🦀🚧 |
| type.json | 10 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 1 | 1 | 0 | 0 | 0 | 0 | 4 | 0 | 6 | 🙈❌❌✅❌❌ |
| **Total** | **78** | **37** | **17** | **6** | **0** | **3** | **28** | **64** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 7 | 🚧✅✅🤯🤯🤯🤯 |
| optional/ecmascript-regex.json | 16 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🚧✅✅✅🚧🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/id.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 1 | 3 | 🤯🙈🙈 |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| **Total** | **20** | **2** | **6** | **0** | **0** | **0** | **0** | **16** | **44** |  |

## Failures

### 🤯 `generation_panic` (133)

39 distinct first-line errors; most common:

- **59×** `not yet implemented: invalid (or unexpected) schema:`
- **14×** `not yet implemented: unhandled not schema Object(`
- **4×** `external references are not supported: folderInteger.json at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- **4×** `internal error: entered unreachable code at <registry>/typify-impl-0.3.0/src/enums.rs:86`
- **2×** `$ref # is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`

Per group:

- `draft4/allOf.json::allOf simple types` — `not implemented: this is fairly fussy and I don't want to do it at <registry>/typify-impl-0.3.0/src/merge.rs:695`
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `assertion failed: any_of.is_none() || one_of.is_none() at <registry>/typify-impl-0.3.0/src/merge.rs:315`
- `draft4/anyOf.json::anyOf` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/definitions.json::validate definition against metaschema` — `external references are not supported: http://json-schema.org/draft-04/schema# at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/enum.json::heterogeneous enum validation` — `multiple implied types for an un-typed enum {Boolean, Object, Array, Number, String} [Number(6), String("foo"), Array [], Bool(true), Object {"foo": Number(12)}…`
- `draft4/format.json::email format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/format.json::ipv4 format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/format.json::ipv6 format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/format.json::hostname format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/format.json::date-time format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/format.json::uri format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/maximum.json::maximum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/maximum.json::maximum validation with unsigned integer` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/minimum.json::minimum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/minimum.json::minimum validation with signed integer` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/multipleOf.json::by int` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/multipleOf.json::by number` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/multipleOf.json::by small number` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/not.json::not` — `not yet implemented: unhandled not schema Object(`
- `draft4/not.json::not multiple types` — `not yet implemented: unhandled not schema Object(`
- `draft4/not.json::not more complex schema` — `not yet implemented: unhandled not schema Object(`
- `draft4/not.json::forbidden property` — `not yet implemented: unhandled not schema Object(`
- `draft4/not.json::forbid everything with empty schema` — `not yet implemented: unhandled not schema Object(`
- `draft4/not.json::double negation` — `not yet implemented: unhandled not schema Object(`
- `draft4/oneOf.json::oneOf` — `not yet implemented: invalid (or unexpected) schema:`
- `draft4/ref.json::root pointer ref` — `$ref # is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`
- `draft4/ref.json::relative pointer ref to object` — `$ref #/properties/foo is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`
- `draft4/ref.json::escaped pointer ref` — `$ref #/definitions/percent%25field is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`
- `draft4/ref.json::$ref prevents a sibling id from changing the base uri` — `external references are not supported: foo.json at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/ref.json::remote ref, containing refs itself` — `external references are not supported: http://json-schema.org/draft-04/schema# at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/ref.json::Recursive references between schemas` — `external references are not supported: tree at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/ref.json::refs with quote` — `$ref #/definitions/foo%22bar is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`
- `draft4/ref.json::Location-independent identifier` — `expected a '/' in $ref: #foo at <registry>/typify-impl-0.3.0/src/util.rs:538`
- `draft4/ref.json::Location-independent identifier with base URI change in subschema` — `external references are not supported: http://localhost:1234/nested.json#foo at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/ref.json::id must be resolved against nearest parent, not just immediate parent` — `not yet implemented: unhandled not schema Object(`
- `draft4/refRemote.json::remote ref` — `external references are not supported: http://localhost:1234/integer.json at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/refRemote.json::fragment within remote ref` — `external references are not supported: http://localhost:1234/draft4/subSchemas.json#/definitions/integer at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/refRemote.json::ref within remote ref` — `external references are not supported: http://localhost:1234/draft4/subSchemas.json#/definitions/refToInteger at <registry>/typify-impl-0.3.0/src/convert.rs:127…`
- `draft4/refRemote.json::base URI change` — `external references are not supported: folderInteger.json at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/refRemote.json::base URI change - change folder` — `external references are not supported: folderInteger.json at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `$ref #/definitions/baz/definitions/bar is missing at <registry>/typify-impl-0.3.0/src/convert.rs:1285`
- `draft4/refRemote.json::root ref in remote ref` — `external references are not supported: draft4/name.json#/definitions/orNull at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft4/refRemote.json::Location-independent identifier in remote ref` — `external references are not supported: http://localhost:1234/draft4/locationIndependentIdentifier.json#/definitions/refToInteger at <registry>/typify-impl-0.3.0…`
- `draft6/allOf.json::allOf simple types` — `not implemented: this is fairly fussy and I don't want to do it at <registry>/typify-impl-0.3.0/src/merge.rs:695`
- `draft6/allOf.json::allOf combined with anyOf, oneOf` — `assertion failed: any_of.is_none() || one_of.is_none() at <registry>/typify-impl-0.3.0/src/merge.rs:315`
- `draft6/anyOf.json::anyOf` — `not yet implemented: invalid (or unexpected) schema:`
- `draft6/anyOf.json::anyOf with boolean schemas, some true` — `internal error: entered unreachable code at <registry>/typify-impl-0.3.0/src/enums.rs:86`
- `draft6/anyOf.json::anyOf with boolean schemas, all false` — `not yet implemented at <registry>/typify-impl-0.3.0/src/enums.rs:90`
- `draft6/definitions.json::validate definition against metaschema` — `external references are not supported: http://json-schema.org/draft-06/schema# at <registry>/typify-impl-0.3.0/src/convert.rs:1279`
- `draft6/enum.json::heterogeneous enum validation` — `multiple implied types for an un-typed enum {Boolean, Object, Array, Number, String} [Number(6), String("foo"), Array [], Bool(true), Object {"foo": Number(12)}…`
- … and 83 more

### ❌ `generation_error` (49)

7 distinct first-line errors; most common:

- **33×** `invalid schema for Root: unhandled array validation ArrayValidation {`
- **4×** `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- **4×** `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- **2×** `invalid schema for item: unhandled array validation ArrayValidation {`
- **2×** `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'false', expected f64`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/additionalItems.json::additionalItems are allowed by default` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/default.json::invalid type for default` — `value does not conform to the given schema`
- `draft4/items.json::an array of schemas for items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/items.json::items and subitems` — `invalid schema for item: unhandled array validation ArrayValidation {`
- `draft4/items.json::array-form items with null instance elements` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/maximum.json::maximum validation (explicit false exclusivity)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'false', expected f64`
- `draft4/maximum.json::exclusiveMaximum validation` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `draft4/minimum.json::minimum validation (explicit false exclusivity)` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'false', expected f64`
- `draft4/minimum.json::exclusiveMinimum validation` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `draft4/ref.json::relative pointer ref to array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/uniqueItems.json::uniqueItems=false with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::additionalItems as schema` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::array of items with no additionalItems permitted` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::additionalItems are allowed by default` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::additionalItems does not look in applicators, invalid case` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::items validation adjusts the starting index for additionalItems` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/additionalItems.json::additionalItems with heterogeneous array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::contains keyword validation` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::contains keyword with const keyword` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::contains keyword with boolean schema true` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::contains keyword with boolean schema false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::items + contains` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/contains.json::contains with null instance elements` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/default.json::invalid type for default` — `value does not conform to the given schema`
- `draft6/items.json::an array of schemas for items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/items.json::items with boolean schemas` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/items.json::items and subitems` — `invalid schema for item: unhandled array validation ArrayValidation {`
- `draft6/items.json::array-form items with null instance elements` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/maxItems.json::maxItems validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft6/maxLength.json::maxLength validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft6/maxProperties.json::maxProperties validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft6/minItems.json::minItems validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `draft6/minLength.json::minLength validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft6/minProperties.json::minProperties validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `draft6/ref.json::relative pointer ref to array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/uniqueItems.json::uniqueItems with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/uniqueItems.json::uniqueItems=false with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft6/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft4/optional/bignum.json::float comparison with high precision` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`
- `draft4/optional/bignum.json::float comparison with high precision on negative numbers` — `schema did not parse as schemars::schema::RootSchema: invalid type: boolean 'true', expected f64`

### 🦀 `compile_fail` (6)

2 distinct first-line errors; most common:

- **4×** `error[E0124]: field 'foo_bar' is already declared`
- **2×** `error[E0428]: the name 'FooBar' is defined multiple times`

Per group:

- `draft4/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft4/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft4/required.json::required with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft6/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft6/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft6/required.json::required with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`

### 🤪 `mixed` (12)

4 distinct first-line errors; most common:

- **4×** `accepted 'too long is invalid' though valid: false`
- **4×** `accepted 'too short is invalid' though valid: false`
- **2×** `accepted 'property invalidates property' though valid: false`
- **2×** `ignores non-strings: invalid type: integer '1', expected a string`

Per group:

- `draft4/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft4/maxProperties.json::maxProperties validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft4/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft4/minLength.json::minLength validation` — `ignores non-strings: invalid type: integer '1', expected a string`
  - cases: ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft4/minProperties.json::minProperties validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `draft4/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `draft6/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft6/maxProperties.json::maxProperties validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft6/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft6/minLength.json::minLength validation` — `ignores non-strings: invalid type: integer '1', expected a string`
  - cases: ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft6/minProperties.json::minProperties validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `draft6/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)

### 🚧 `false_reject` (42)

18 distinct first-line errors; most common:

- **4×** `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
- **4×** `ascii digits: doesn't match pattern "^\p{digit}+$"`
- **4×** `ignores non-arrays: invalid type: map, expected a sequence`
- **4×** `ignores other non-objects: invalid type: integer '12', expected struct Root`
- **3×** `ignores arrays: invalid type: sequence, expected a map`

Per group:

- `draft4/additionalItems.json::additionalItems as false without items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected)
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft4/anyOf.json::anyOf with base schema` — `one anyOf valid: invalid type: string "foobar", expected struct Root`
  - cases: one anyOf valid (rejected)
- `draft4/anyOf.json::anyOf with one empty schema` — `string is valid: invalid type: string "foo", expected struct Root`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft4/enum.json::enum with [0] does not match [false]` — `[0.0] is valid: invalid value`
  - cases: [0.0] is valid (rejected)
- `draft4/enum.json::enum with [1] does not match [true]` — `[1.0] is valid: invalid value`
  - cases: [1.0] is valid (rejected)
- `draft4/items.json::a schema given for items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `draft4/maxLength.json::maxLength validation` — `ignores non-strings: invalid type: integer '100', expected a string`
  - cases: ignores non-strings (rejected), two graphemes is long enough (rejected)
- `draft4/pattern.json::pattern validation` — `ignores booleans: invalid type: boolean 'true', expected a string`
  - cases: ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), ignores null (rejected)
- `draft4/patternProperties.json::patternProperties validates properties matching a regex` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft4/properties.json::object properties validation` — `ignores other non-objects: invalid type: integer '12', expected struct Root`
  - cases: ignores other non-objects (rejected)
- `draft4/properties.json::properties whose names are Javascript object property names` — `ignores other non-objects: invalid type: integer '12', expected struct Root`
  - cases: ignores other non-objects (rejected)
- `draft4/required.json::required validation` — `ignores arrays: invalid length 1, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft4/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft6/additionalItems.json::additionalItems as false without items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected)
- `draft6/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft6/anyOf.json::anyOf with base schema` — `one anyOf valid: invalid type: string "foobar", expected struct Root`
  - cases: one anyOf valid (rejected)
- `draft6/anyOf.json::anyOf with boolean schemas, all true` — `any value is valid: invalid type: string "foo", expected struct Root`
  - cases: any value is valid (rejected)
- `draft6/anyOf.json::anyOf with one empty schema` — `string is valid: invalid type: string "foo", expected struct Root`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft6/enum.json::enum with [0] does not match [false]` — `[0.0] is valid: invalid value`
  - cases: [0.0] is valid (rejected)
- `draft6/enum.json::enum with [1] does not match [true]` — `[1.0] is valid: invalid value`
  - cases: [1.0] is valid (rejected)
- `draft6/items.json::a schema given for items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `draft6/maxLength.json::maxLength validation` — `ignores non-strings: invalid type: integer '100', expected a string`
  - cases: ignores non-strings (rejected), two graphemes is long enough (rejected)
- `draft6/pattern.json::pattern validation` — `ignores booleans: invalid type: boolean 'true', expected a string`
  - cases: ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), ignores null (rejected)
- `draft6/patternProperties.json::patternProperties validates properties matching a regex` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft6/properties.json::object properties validation` — `ignores other non-objects: invalid type: integer '12', expected struct Root`
  - cases: ignores other non-objects (rejected)
- `draft6/properties.json::properties whose names are Javascript object property names` — `ignores other non-objects: invalid type: integer '12', expected struct Root`
  - cases: ignores other non-objects (rejected)
- `draft6/propertyNames.json::propertyNames validation` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `draft6/required.json::required validation` — `ignores arrays: invalid length 1, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft6/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft4/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft4/optional/ecmascript-regex.json::patterns always use unicode semantics with pattern` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `draft4/optional/ecmascript-regex.json::pattern with non-ASCII digits` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `draft4/optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `draft4/optional/ecmascript-regex.json::patternProperties with non-ASCII digits` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `draft6/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/ecmascript-regex.json::patterns always use unicode semantics with pattern` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `draft6/optional/ecmascript-regex.json::pattern with non-ASCII digits` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `draft6/optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties` — `ascii character in json string: doesn't match pattern "\p{Letter}cole"`
  - cases: ascii character in json string (rejected), literal unicode character in json string (rejected), unicode character in hex format in string (rejected)
- `draft6/optional/ecmascript-regex.json::patternProperties with non-ASCII digits` — `ascii digits: doesn't match pattern "^\p{digit}+$"`
  - cases: ascii digits (rejected), non-ascii digits (BENGALI DIGIT FOUR, BENGALI DIGIT TWO) (rejected)
- `draft6/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`
  - cases: valid if optional overflow handling is implemented (rejected)

### 🙈 `false_accept` (55)

36 distinct first-line errors; most common:

- **4×** `accepted 'both oneOf valid' though valid: false`
- **4×** `accepted 'both valid - invalid' though valid: false`
- **4×** `accepted 'missing dependency' though valid: false`
- **2×** `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
- **2×** `accepted 'an invalid due to one is invalid' though valid: false`

Per group:

- `draft4/anyOf.json::anyOf complex types` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `draft4/default.json::the default keyword does not do anything if the property is missing` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `draft4/dependencies.json::dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `draft4/dependencies.json::multiple dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `draft4/dependencies.json::multiple dependencies subschema` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `draft4/dependencies.json::dependencies with escaped characters` — `accepted 'invalid object 1' though valid: false`
  - cases: invalid object 1 (accepted), invalid object 2 (accepted), invalid object 3 (accepted), invalid object 4 (accepted)
- `draft4/dependencies.json::dependent subschema incompatible with root` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `draft4/maxProperties.json::maxProperties = 0 means the object is empty` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `draft4/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/oneOf.json::oneOf with empty schema` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with required` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with missing optional property` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/patternProperties.json::multiple simultaneous patternProperties are validated` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `draft4/patternProperties.json::regexes are not anchored by default and are case sensitive` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `draft4/ref.json::empty tokens in $ref json-pointer` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)
- `draft4/uniqueItems.json::uniqueItems validation` — `accepted 'non-unique array of integers is invalid' though valid: false`
  - cases: non-unique array of integers is invalid (accepted), non-unique array of more than two integers is invalid (accepted), numbers are unique if mathematically unequal (accepted), non-unique array of strings is invalid (accepted), non-unique array of objects is invalid (accepted), property order of array of objects is ignored (accepted), … and 5 more
- `draft6/anyOf.json::anyOf complex types` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `draft6/const.json::const validation` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted), another type is invalid (accepted)
- `draft6/const.json::const with object` — `accepted 'another object is invalid' though valid: false`
  - cases: another object is invalid (accepted), another type is invalid (accepted)
- `draft6/const.json::const with array` — `accepted 'another array item is invalid' though valid: false`
  - cases: another array item is invalid (accepted), array with additional items is invalid (accepted)
- `draft6/const.json::const with null` — `accepted 'not null is invalid' though valid: false`
  - cases: not null is invalid (accepted)
- `draft6/const.json::const with false does not match 0` — `accepted 'integer zero is invalid' though valid: false`
  - cases: integer zero is invalid (accepted), float zero is invalid (accepted)
- `draft6/const.json::const with true does not match 1` — `accepted 'integer one is invalid' though valid: false`
  - cases: integer one is invalid (accepted), float one is invalid (accepted)
- `draft6/const.json::const with [false] does not match [0]` — `accepted '[0] is invalid' though valid: false`
  - cases: [0] is invalid (accepted), [0.0] is invalid (accepted)
- `draft6/const.json::const with [true] does not match [1]` — `accepted '[1] is invalid' though valid: false`
  - cases: [1] is invalid (accepted), [1.0] is invalid (accepted)
- `draft6/const.json::const with {"a": false} does not match {"a": 0}` — `accepted '{"a": 0} is invalid' though valid: false`
  - cases: {"a": 0} is invalid (accepted), {"a": 0.0} is invalid (accepted)
- `draft6/const.json::const with {"a": true} does not match {"a": 1}` — `accepted '{"a": 1} is invalid' though valid: false`
  - cases: {"a": 1} is invalid (accepted), {"a": 1.0} is invalid (accepted)
- `draft6/const.json::const with 0 does not match other zero-like types` — `accepted 'false is invalid' though valid: false`
  - cases: false is invalid (accepted), empty object is invalid (accepted), empty array is invalid (accepted), empty string is invalid (accepted)
- `draft6/const.json::const with 1 does not match true` — `accepted 'true is invalid' though valid: false`
  - cases: true is invalid (accepted)
- `draft6/const.json::const with -2.0 matches integer and float types` — `accepted 'integer 2 is invalid' though valid: false`
  - cases: integer 2 is invalid (accepted), float 2.0 is invalid (accepted), float -2.00001 is invalid (accepted)
- `draft6/const.json::float and integers are equal up to 64-bit representation limits` — `accepted 'integer minus one is invalid' though valid: false`
  - cases: integer minus one is invalid (accepted), float minus one is invalid (accepted)
- `draft6/const.json::nul characters in strings` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `draft6/const.json::characters with the same visual representation but different codepoint` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `draft6/const.json::characters with the same visual representation, but different number of codepoints` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `draft6/default.json::the default keyword does not do anything if the property is missing` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `draft6/dependencies.json::dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `draft6/dependencies.json::multiple dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `draft6/dependencies.json::multiple dependencies subschema` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `draft6/dependencies.json::dependencies with boolean subschemas` — `accepted 'object with property having schema false is invalid' though valid: false`
  - cases: object with property having schema false is invalid (accepted), object with both properties is invalid (accepted)
- `draft6/dependencies.json::dependencies with escaped characters` — `accepted 'invalid object 1' though valid: false`
  - cases: invalid object 1 (accepted), invalid object 2 (accepted), invalid object 3 (accepted), invalid object 4 (accepted)
- `draft6/dependencies.json::dependent subschema incompatible with root` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `draft6/maxProperties.json::maxProperties = 0 means the object is empty` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `draft6/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft6/oneOf.json::oneOf with empty schema` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft6/oneOf.json::oneOf with required` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft6/oneOf.json::oneOf with missing optional property` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft6/patternProperties.json::multiple simultaneous patternProperties are validated` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `draft6/patternProperties.json::regexes are not anchored by default and are case sensitive` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `draft6/patternProperties.json::patternProperties with boolean schemas` — `accepted 'object with property matching schema false is invalid' though valid: false`
  - cases: object with property matching schema false is invalid (accepted), object with both properties is invalid (accepted), object with a property matching both true and false is invalid (accepted)
- `draft6/properties.json::properties with boolean schema` — `accepted 'only 'false' property present is invalid' though valid: false`
  - cases: only 'false' property present is invalid (accepted), both properties present is invalid (accepted)
- … and 5 more

