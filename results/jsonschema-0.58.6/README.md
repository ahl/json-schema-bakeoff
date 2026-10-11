# jsonschema 0.58.6

jsonschema is a Rust validator, not a code generator: it interprets the
schema at run time instead of compiling it into types, so it can enforce
every keyword, which a type generator structurally cannot. It is the
reference row: it shows the ceiling the suite allows, calibrates the harness
(a miss here is a harness bug or a policy), and prices the policies the
generators live with. It resolves `localhost:1234` remotes from
`suite/remotes/`, selects the dialect from the suite directory, and leaves
`format` at the library default (asserted through draft 7, annotation-only
from 2019-09 on).

_generated 2026-10-11T18:14:09Z_

**1408 of 1408 non-optional groups pass (100.0%)**; 301 optional groups reported separately

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

## Headline

### Groups

| Draft | ✅ | Total | Pass rate |
|---|---:|---:|---:|
| draft4 | 160 | 160 | 100.0% |
| draft6 | 233 | 233 | 100.0% |
| draft7 | 258 | 258 | 100.0% |
| draft2019-09 | 373 | 373 | 100.0% |
| draft2020-12 | 384 | 384 | 100.0% |
| **Total** | **1408** | **1408** | **100.0%** |

### Optional groups

| Draft | ✅ | 🙈 | Total | Pass rate |
|---|---:|---:|---:|---:|
| draft4 | 39 | 0 | 39 | 100.0% |
| draft6 | 44 | 0 | 44 | 100.0% |
| draft7 | 64 | 0 | 64 | 100.0% |
| draft2019-09 | 52 | 24 | 76 | 68.4% |
| draft2020-12 | 54 | 24 | 78 | 69.2% |
| **Total** | **253** | **48** | **301** | **84.1%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 618 | 618 | 100.0% | 618 | 100.0% |
| draft6 | 841 | 841 | 100.0% | 841 | 100.0% |
| draft7 | 929 | 929 | 100.0% | 929 | 100.0% |
| draft2019-09 | 1261 | 1261 | 100.0% | 1261 | 100.0% |
| draft2020-12 | 1301 | 1301 | 100.0% | 1301 | 100.0% |
| **Total** | **4950** | **4950** | **100.0%** | **4950** | **100.0%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 391 | 391 | 100.0% | 391 | 100.0% |
| draft6 | 520 | 520 | 100.0% | 520 | 100.0% |
| draft7 | 922 | 922 | 100.0% | 922 | 100.0% |
| draft2019-09 | 566 | 1043 | 54.3% | 1043 | 54.3% |
| draft2020-12 | 570 | 1047 | 54.4% | 1047 | 54.4% |
| **Total** | **2969** | **3923** | **75.7%** | **3923** | **75.7%** |

## Chart

### draft4

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| additionalItems.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 5 | 5 | ✅✅✅✅✅ |
| default.json | 3 | 3 | ✅✅✅ |
| definitions.json | 1 | 1 | ✅ |
| dependencies.json | 5 | 5 | ✅✅✅✅✅ |
| enum.json | 16 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| format.json | 6 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 1 | ✅ |
| items.json | 6 | 6 | ✅✅✅✅✅✅ |
| maxItems.json | 1 | 1 | ✅ |
| maxLength.json | 1 | 1 | ✅ |
| maxProperties.json | 2 | 2 | ✅✅ |
| maximum.json | 4 | 4 | ✅✅✅✅ |
| minItems.json | 1 | 1 | ✅ |
| minLength.json | 1 | 1 | ✅ |
| minProperties.json | 1 | 1 | ✅ |
| minimum.json | 4 | 4 | ✅✅✅✅ |
| multipleOf.json | 5 | 5 | ✅✅✅✅✅ |
| not.json | 6 | 6 | ✅✅✅✅✅✅ |
| oneOf.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| pattern.json | 2 | 2 | ✅✅ |
| patternProperties.json | 4 | 4 | ✅✅✅✅ |
| properties.json | 5 | 5 | ✅✅✅✅✅ |
| ref.json | 19 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| required.json | 4 | 4 | ✅✅✅✅ |
| type.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 6 | ✅✅✅✅✅✅ |
| **Total** | **160** | **160** |  |

#### optional

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 1 | ✅ |
| optional/format/date-time.json | 1 | 1 | ✅ |
| optional/format/email.json | 1 | 1 | ✅ |
| optional/format/hostname.json | 1 | 1 | ✅ |
| optional/format/ipv4.json | 1 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 1 | ✅ |
| optional/format/unknown.json | 1 | 1 | ✅ |
| optional/format/uri.json | 1 | 1 | ✅ |
| optional/id.json | 1 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 2 | ✅✅ |
| optional/zeroTerminatedFloats.json | 1 | 1 | ✅ |
| **Total** | **39** | **39** |  |

### draft6

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| additionalItems.json | 10 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 2 | ✅✅ |
| const.json | 17 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 6 | 6 | ✅✅✅✅✅✅ |
| default.json | 3 | 3 | ✅✅✅ |
| definitions.json | 1 | 1 | ✅ |
| dependencies.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 1 | ✅ |
| format.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 1 | ✅ |
| items.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 2 | ✅✅ |
| maxLength.json | 2 | 2 | ✅✅ |
| maxProperties.json | 3 | 3 | ✅✅✅ |
| maximum.json | 2 | 2 | ✅✅ |
| minItems.json | 2 | 2 | ✅✅ |
| minLength.json | 2 | 2 | ✅✅ |
| minProperties.json | 2 | 2 | ✅✅ |
| minimum.json | 2 | 2 | ✅✅ |
| multipleOf.json | 5 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 2 | ✅✅ |
| patternProperties.json | 5 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 6 | ✅✅✅✅✅✅ |
| ref.json | 31 | 31 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 6 | ✅✅✅✅✅✅ |
| **Total** | **233** | **233** |  |

#### optional

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 1 | ✅ |
| optional/format/date-time.json | 1 | 1 | ✅ |
| optional/format/email.json | 1 | 1 | ✅ |
| optional/format/hostname.json | 1 | 1 | ✅ |
| optional/format/ipv4.json | 1 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 1 | ✅ |
| optional/format/unknown.json | 1 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 1 | ✅ |
| optional/format/uri.json | 1 | 1 | ✅ |
| optional/id.json | 3 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 2 | ✅✅ |
| optional/unknownKeyword.json | 1 | 1 | ✅ |
| **Total** | **44** | **44** |  |

### draft7

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| additionalItems.json | 10 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 2 | ✅✅ |
| const.json | 17 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| default.json | 3 | 3 | ✅✅✅ |
| definitions.json | 1 | 1 | ✅ |
| dependencies.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 1 | ✅ |
| format.json | 17 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 1 | ✅ |
| items.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 2 | ✅✅ |
| maxLength.json | 2 | 2 | ✅✅ |
| maxProperties.json | 3 | 3 | ✅✅✅ |
| maximum.json | 2 | 2 | ✅✅ |
| minItems.json | 2 | 2 | ✅✅ |
| minLength.json | 2 | 2 | ✅✅ |
| minProperties.json | 2 | 2 | ✅✅ |
| minimum.json | 2 | 2 | ✅✅ |
| multipleOf.json | 5 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 2 | ✅✅ |
| patternProperties.json | 5 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 6 | ✅✅✅✅✅✅ |
| ref.json | 35 | 35 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 6 | ✅✅✅✅✅✅ |
| **Total** | **258** | **258** |  |

#### optional

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| optional/content.json | 3 | 3 | ✅✅✅ |
| optional/cross-draft.json | 1 | 1 | ✅ |
| optional/ecmascript-regex.json | 20 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 1 | ✅ |
| optional/format/date-time.json | 1 | 1 | ✅ |
| optional/format/date.json | 1 | 1 | ✅ |
| optional/format/ecmascript-regex.json | 6 | 6 | ✅✅✅✅✅✅ |
| optional/format/email.json | 1 | 1 | ✅ |
| optional/format/hostname.json | 2 | 2 | ✅✅ |
| optional/format/idn-email.json | 1 | 1 | ✅ |
| optional/format/idn-hostname.json | 2 | 2 | ✅✅ |
| optional/format/ipv4.json | 1 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 1 | ✅ |
| optional/format/iri-reference.json | 1 | 1 | ✅ |
| optional/format/iri.json | 1 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 1 | ✅ |
| optional/format/regex.json | 1 | 1 | ✅ |
| optional/format/relative-json-pointer.json | 1 | 1 | ✅ |
| optional/format/time.json | 1 | 1 | ✅ |
| optional/format/unknown.json | 1 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 1 | ✅ |
| optional/format/uri.json | 1 | 1 | ✅ |
| optional/id.json | 3 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 2 | ✅✅ |
| optional/unknownKeyword.json | 1 | 1 | ✅ |
| **Total** | **64** | **64** |  |

### draft2019-09

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| additionalItems.json | 10 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 2 | ✅✅ |
| const.json | 17 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 4 | ✅✅✅✅ |
| default.json | 3 | 3 | ✅✅✅ |
| defs.json | 1 | 1 | ✅ |
| dependentRequired.json | 4 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 4 | ✅✅✅✅ |
| enum.json | 16 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 1 | ✅ |
| format.json | 19 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 1 | ✅ |
| items.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 2 | ✅✅ |
| maxLength.json | 2 | 2 | ✅✅ |
| maxProperties.json | 3 | 3 | ✅✅✅ |
| maximum.json | 2 | 2 | ✅✅ |
| minContains.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 2 | ✅✅ |
| minLength.json | 2 | 2 | ✅✅ |
| minProperties.json | 2 | 2 | ✅✅ |
| minimum.json | 2 | 2 | ✅✅ |
| multipleOf.json | 5 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 2 | ✅✅ |
| patternProperties.json | 5 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 6 | ✅✅✅✅✅✅ |
| recursiveRef.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| ref.json | 37 | 37 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 26 | 26 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedProperties.json | 44 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 2 | 2 | ✅✅ |
| **Total** | **373** | **373** |  |

#### optional

| File | ✅ | 🙈 | Total | Chart |
|---|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 2 | 0 | 2 | ✅✅ |
| optional/dependencies-compatibility.json | 7 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 1 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 1 | 🙈 |
| optional/id.json | 1 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 1 | ✅ |
| **Total** | **52** | **24** | **76** |  |

### draft2020-12

| File | ✅ | Total | Chart |
|---|---:|---:|---|
| additionalProperties.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 2 | ✅✅ |
| const.json | 17 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 4 | ✅✅✅✅ |
| default.json | 3 | 3 | ✅✅✅ |
| defs.json | 1 | 1 | ✅ |
| dependentRequired.json | 4 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 4 | ✅✅✅✅ |
| dynamicRef.json | 21 | 21 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| enum.json | 16 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 1 | ✅ |
| format.json | 19 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 1 | ✅ |
| items.json | 10 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 2 | ✅✅ |
| maxLength.json | 2 | 2 | ✅✅ |
| maxProperties.json | 3 | 3 | ✅✅✅ |
| maximum.json | 2 | 2 | ✅✅ |
| minContains.json | 8 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 2 | ✅✅ |
| minLength.json | 2 | 2 | ✅✅ |
| minProperties.json | 2 | 2 | ✅✅ |
| minimum.json | 2 | 2 | ✅✅ |
| multipleOf.json | 5 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 3 | 3 | ✅✅✅ |
| patternProperties.json | 6 | 6 | ✅✅✅✅✅✅ |
| prefixItems.json | 4 | 4 | ✅✅✅✅ |
| properties.json | 6 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 6 | ✅✅✅✅✅✅ |
| ref.json | 36 | 36 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 29 | 29 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedProperties.json | 44 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 2 | 2 | ✅✅ |
| **Total** | **384** | **384** |  |

#### optional

| File | ✅ | 🙈 | Total | Chart |
|---|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 1 | 0 | 1 | ✅ |
| optional/dependencies-compatibility.json | 7 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/dynamicRef.json | 1 | 0 | 1 | ✅ |
| optional/ecmascript-regex.json | 20 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 1 | ✅ |
| optional/format-assertion.json | 2 | 0 | 2 | ✅✅ |
| optional/format/date-time.json | 0 | 1 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 1 | 🙈 |
| optional/id.json | 1 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 1 | ✅ |
| **Total** | **54** | **24** | **78** |  |

## Failures

### 🙈 `false_accept` (48)

24 distinct first-line errors; most common:

- **2×** `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
- **2×** `accepted 'a group with 5 hex digits is invalid' though valid: false`
- **2×** `accepted 'a invalid date string with 32 days in January' though valid: false`
- **2×** `accepted 'a regular expression with unclosed parens is invalid' though valid: false`
- **2×** `accepted 'a single global inline flag (?i)' though valid: false`

Per group:

- `draft2019-09/optional/format/date-time.json::validation of date-time strings` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid day in date-time string (accepted), an invalid offset in date-time string (accepted), an invalid closing Z after time-zone offset (accepted), … and 24 more
- `draft2019-09/optional/format/date.json::validation of date strings` — `accepted 'a invalid date string with 32 days in January' though valid: false`
  - cases: a invalid date string with 32 days in January (accepted), a invalid date string with 30 days in February (leap) (accepted), a invalid date string with 32 days in March (accepted), a invalid date string with 31 days in April (accepted), a invalid date string with 32 days in May (accepted), a invalid date string with 31 days in June (accepted), … and 52 more
- `draft2019-09/optional/format/duration.json::validation of duration strings` — `accepted 'an invalid duration string' though valid: false`
  - cases: an invalid duration string (accepted), must start with P (accepted), no elements present (accepted), no time elements present (accepted), no date or time elements present (accepted), elements out of order (accepted), … and 19 more
- `draft2019-09/optional/format/ecmascript-regex.json::\a is not an ECMA 262 control escape` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `draft2019-09/optional/format/ecmascript-regex.json::Python-specific regular expression syntax is not valid ECMA 262` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `draft2019-09/optional/format/ecmascript-regex.json::global inline flag groups are not valid ECMA 262` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `draft2019-09/optional/format/email.json::validation of e-mail addresses` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), an invalid domain (accepted), an invalid IPv4-address-literal (accepted), … and 38 more
- `draft2019-09/optional/format/hostname.json::validation of host names` — `accepted 'empty string' though valid: false`
  - cases: empty string (accepted), single dot (accepted), leading dot (accepted), trailing dot (accepted), IDN label separator (accepted), starts with hyphen (accepted), … and 6 more
- `draft2019-09/optional/format/hostname.json::validation of A-label (punycode) host names` — `accepted 'invalid Punycode' though valid: false`
  - cases: invalid Punycode (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), Begins with a Spacing Combining Mark (accepted), Begins with a Nonspacing Mark (accepted), Begins with an Enclosing Mark (accepted), Exceptions that are DISALLOWED, right-to-left chars (accepted), … and 17 more
- `draft2019-09/optional/format/idn-email.json::validation of an internationalized e-mail addresses` — `accepted 'an invalid e-mail/idn e-mail address' though valid: false`
  - cases: an invalid e-mail/idn e-mail address (accepted), a fullwidth commercial at is not a local-part separator (accepted), a domain label starting with a hyphen is not valid (accepted), a domain label ending with a hyphen is not valid (accepted), a domain starting with a dot is not valid (accepted), a domain ending with a dot is not valid (accepted), … and 2 more
- `draft2019-09/optional/format/idn-hostname.json::validation of internationalized host names` — `accepted 'illegal first char U+302E Hangul single dot tone mark' though valid: false`
  - cases: illegal first char U+302E Hangul single dot tone mark (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), a single label of 64 characters is too long (accepted), invalid label, correct Punycode (accepted), invalid Punycode (accepted), U-label contains "--" in the 3rd and 4th position (accepted), … and 37 more
- `draft2019-09/optional/format/idn-hostname.json::validation of separators in internationalized host names` — `accepted 'single dot' though valid: false`
  - cases: single dot (accepted), single ideographic full stop (accepted), single fullwidth full stop (accepted), single halfwidth ideographic full stop (accepted), leading dot (accepted), leading ideographic full stop (accepted), … and 6 more
- `draft2019-09/optional/format/ipv4.json::validation of IP addresses` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `draft2019-09/optional/format/ipv6.json::validation of IPv6 addresses` — `accepted 'a group with 5 hex digits is invalid' though valid: false`
  - cases: a group with 5 hex digits is invalid (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `draft2019-09/optional/format/iri-reference.json::validation of IRI References` — `accepted 'an invalid IRI Reference' though valid: false`
  - cases: an invalid IRI Reference (accepted), an invalid IRI fragment (accepted), an embedded IPv4 with a leading zero is invalid (accepted), a trailing newline after a valid IRI Reference is invalid (accepted), invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), … and 1 more
- `draft2019-09/optional/format/iri.json::validation of IRIs` — `accepted 'invalid percent-encoding with non-hex digits' though valid: false`
  - cases: invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), lone percent sign is invalid (accepted), an IPv6 address without enclosing brackets is invalid (accepted), an invalid relative IRI Reference (accepted), an invalid IRI (accepted), … and 3 more
- `draft2019-09/optional/format/json-pointer.json::validation of JSON-pointers (JSON String Representation)` — `accepted 'not a valid JSON-pointer (~ not escaped)' though valid: false`
  - cases: not a valid JSON-pointer (~ not escaped) (accepted), not a valid JSON-pointer (URI Fragment Identifier) #1 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #2 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #3 (accepted), not a valid JSON-pointer (some escaped, but not all) #1 (accepted), not a valid JSON-pointer (some escaped, but not all) #2 (accepted), … and 6 more
- `draft2019-09/optional/format/regex.json::validation of regular expressions` — `accepted 'a regular expression with unclosed parens is invalid' though valid: false`
  - cases: a regular expression with unclosed parens is invalid (accepted), a quantifier with nothing to repeat is invalid (accepted), a character class range in reverse order is invalid (accepted), a trailing backslash is invalid (accepted), an unmatched closing parenthesis is invalid (accepted), an unterminated character class is invalid (accepted)
- `draft2019-09/optional/format/relative-json-pointer.json::validation of Relative JSON Pointers (RJP)` — `accepted 'an invalid RJP that is a valid JSON Pointer' though valid: false`
  - cases: an invalid RJP that is a valid JSON Pointer (accepted), a json-pointer part that does not start with a slash is invalid (accepted), negative prefix (accepted), explicit positive prefix (accepted), non-ASCII digit in the prefix is not allowed (accepted), ## is not a valid json-pointer (accepted), … and 7 more
- `draft2019-09/optional/format/time.json::validation of time strings` — `accepted 'invalid time string with extra leading zeros' though valid: false`
  - cases: invalid time string with extra leading zeros (accepted), invalid time string with no leading zero for single digit (accepted), hour, minute, second must be two digits (accepted), invalid leap second, Zulu (wrong hour) (accepted), invalid leap second, Zulu (wrong minute) (accepted), invalid leap second, zero time-offset (wrong hour) (accepted), … and 30 more
- `draft2019-09/optional/format/uri-reference.json::validation of URI References` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- `draft2019-09/optional/format/uri-template.json::format: uri-template` — `accepted 'an invalid uri-template' though valid: false`
  - cases: an invalid uri-template (accepted), an empty expression is invalid (accepted), an empty varspec inside the variable list is invalid (accepted), a zero prefix max-length is invalid (accepted), a five-digit prefix max-length is invalid (accepted), an unmatched closing brace is invalid (accepted), … and 15 more
- `draft2019-09/optional/format/uri.json::validation of URIs` — `accepted 'an invalid protocol-relative URI Reference' though valid: false`
  - cases: an invalid protocol-relative URI Reference (accepted), an invalid relative URI Reference (accepted), an invalid URI (accepted), an invalid URI though valid URI reference (accepted), an invalid URI with spaces (accepted), an invalid URI with spaces and missing scheme (accepted), … and 20 more
- `draft2019-09/optional/format/uuid.json::uuid format` — `accepted 'wrong length' though valid: false`
  - cases: wrong length (accepted), missing section (accepted), bad characters (not hex) (accepted), no dashes (accepted), too few dashes (accepted), too many dashes (accepted), … and 8 more
- `draft2020-12/optional/format/date-time.json::validation of date-time strings` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid day in date-time string (accepted), an invalid offset in date-time string (accepted), an invalid closing Z after time-zone offset (accepted), … and 24 more
- `draft2020-12/optional/format/date.json::validation of date strings` — `accepted 'a invalid date string with 32 days in January' though valid: false`
  - cases: a invalid date string with 32 days in January (accepted), a invalid date string with 30 days in February (leap) (accepted), a invalid date string with 32 days in March (accepted), a invalid date string with 31 days in April (accepted), a invalid date string with 32 days in May (accepted), a invalid date string with 31 days in June (accepted), … and 52 more
- `draft2020-12/optional/format/duration.json::validation of duration strings` — `accepted 'an invalid duration string' though valid: false`
  - cases: an invalid duration string (accepted), must start with P (accepted), no elements present (accepted), no time elements present (accepted), no date or time elements present (accepted), elements out of order (accepted), … and 19 more
- `draft2020-12/optional/format/ecmascript-regex.json::\a is not an ECMA 262 control escape` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `draft2020-12/optional/format/ecmascript-regex.json::Python-specific regular expression syntax is not valid ECMA 262` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `draft2020-12/optional/format/ecmascript-regex.json::global inline flag groups are not valid ECMA 262` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `draft2020-12/optional/format/email.json::validation of e-mail addresses` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), an invalid domain (accepted), an invalid IPv4-address-literal (accepted), … and 38 more
- `draft2020-12/optional/format/hostname.json::validation of host names` — `accepted 'empty string' though valid: false`
  - cases: empty string (accepted), single dot (accepted), leading dot (accepted), trailing dot (accepted), IDN label separator (accepted), starts with hyphen (accepted), … and 6 more
- `draft2020-12/optional/format/hostname.json::validation of A-label (punycode) host names` — `accepted 'invalid Punycode' though valid: false`
  - cases: invalid Punycode (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), Begins with a Spacing Combining Mark (accepted), Begins with a Nonspacing Mark (accepted), Begins with an Enclosing Mark (accepted), Exceptions that are DISALLOWED, right-to-left chars (accepted), … and 17 more
- `draft2020-12/optional/format/idn-email.json::validation of an internationalized e-mail addresses` — `accepted 'an invalid e-mail/idn e-mail address' though valid: false`
  - cases: an invalid e-mail/idn e-mail address (accepted), a fullwidth commercial at is not a local-part separator (accepted), a domain label starting with a hyphen is not valid (accepted), a domain label ending with a hyphen is not valid (accepted), a domain starting with a dot is not valid (accepted), a domain ending with a dot is not valid (accepted), … and 2 more
- `draft2020-12/optional/format/idn-hostname.json::validation of internationalized host names` — `accepted 'illegal first char U+302E Hangul single dot tone mark' though valid: false`
  - cases: illegal first char U+302E Hangul single dot tone mark (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), a single label of 64 characters is too long (accepted), invalid label, correct Punycode (accepted), invalid Punycode (accepted), U-label contains "--" in the 3rd and 4th position (accepted), … and 37 more
- `draft2020-12/optional/format/idn-hostname.json::validation of separators in internationalized host names` — `accepted 'single dot' though valid: false`
  - cases: single dot (accepted), single ideographic full stop (accepted), single fullwidth full stop (accepted), single halfwidth ideographic full stop (accepted), leading dot (accepted), leading ideographic full stop (accepted), … and 6 more
- `draft2020-12/optional/format/ipv4.json::validation of IP addresses` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `draft2020-12/optional/format/ipv6.json::validation of IPv6 addresses` — `accepted 'a group with 5 hex digits is invalid' though valid: false`
  - cases: a group with 5 hex digits is invalid (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `draft2020-12/optional/format/iri-reference.json::validation of IRI References` — `accepted 'an invalid IRI Reference' though valid: false`
  - cases: an invalid IRI Reference (accepted), an invalid IRI fragment (accepted), an embedded IPv4 with a leading zero is invalid (accepted), a trailing newline after a valid IRI Reference is invalid (accepted), invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), … and 1 more
- `draft2020-12/optional/format/iri.json::validation of IRIs` — `accepted 'invalid percent-encoding with non-hex digits' though valid: false`
  - cases: invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), lone percent sign is invalid (accepted), an IPv6 address without enclosing brackets is invalid (accepted), an invalid relative IRI Reference (accepted), an invalid IRI (accepted), … and 3 more
- `draft2020-12/optional/format/json-pointer.json::validation of JSON-pointers (JSON String Representation)` — `accepted 'not a valid JSON-pointer (~ not escaped)' though valid: false`
  - cases: not a valid JSON-pointer (~ not escaped) (accepted), not a valid JSON-pointer (URI Fragment Identifier) #1 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #2 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #3 (accepted), not a valid JSON-pointer (some escaped, but not all) #1 (accepted), not a valid JSON-pointer (some escaped, but not all) #2 (accepted), … and 6 more
- `draft2020-12/optional/format/regex.json::validation of regular expressions` — `accepted 'a regular expression with unclosed parens is invalid' though valid: false`
  - cases: a regular expression with unclosed parens is invalid (accepted), a quantifier with nothing to repeat is invalid (accepted), a character class range in reverse order is invalid (accepted), a trailing backslash is invalid (accepted), an unmatched closing parenthesis is invalid (accepted), an unterminated character class is invalid (accepted)
- `draft2020-12/optional/format/relative-json-pointer.json::validation of Relative JSON Pointers (RJP)` — `accepted 'an invalid RJP that is a valid JSON Pointer' though valid: false`
  - cases: an invalid RJP that is a valid JSON Pointer (accepted), a json-pointer part that does not start with a slash is invalid (accepted), negative prefix (accepted), explicit positive prefix (accepted), non-ASCII digit in the prefix is not allowed (accepted), ## is not a valid json-pointer (accepted), … and 7 more
- `draft2020-12/optional/format/time.json::validation of time strings` — `accepted 'invalid time string with extra leading zeros' though valid: false`
  - cases: invalid time string with extra leading zeros (accepted), invalid time string with no leading zero for single digit (accepted), hour, minute, second must be two digits (accepted), invalid leap second, Zulu (wrong hour) (accepted), invalid leap second, Zulu (wrong minute) (accepted), invalid leap second, zero time-offset (wrong hour) (accepted), … and 30 more
- `draft2020-12/optional/format/uri-reference.json::validation of URI References` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- `draft2020-12/optional/format/uri-template.json::format: uri-template` — `accepted 'an invalid uri-template' though valid: false`
  - cases: an invalid uri-template (accepted), an empty expression is invalid (accepted), an empty varspec inside the variable list is invalid (accepted), a zero prefix max-length is invalid (accepted), a five-digit prefix max-length is invalid (accepted), an unmatched closing brace is invalid (accepted), … and 15 more
- `draft2020-12/optional/format/uri.json::validation of URIs` — `accepted 'an invalid protocol-relative URI Reference' though valid: false`
  - cases: an invalid protocol-relative URI Reference (accepted), an invalid relative URI Reference (accepted), an invalid URI (accepted), an invalid URI though valid URI reference (accepted), an invalid URI with spaces (accepted), an invalid URI with spaces and missing scheme (accepted), … and 20 more
- `draft2020-12/optional/format/uuid.json::uuid format` — `accepted 'wrong length' though valid: false`
  - cases: wrong length (accepted), missing section (accepted), bad characters (not hex) (accepted), no dashes (accepted), too few dashes (accepted), too many dashes (accepted), … and 8 more

