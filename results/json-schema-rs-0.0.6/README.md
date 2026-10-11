# Conformance summary — json-schema-rs-0.0.6 (json-schema-rs 0.0.6)

_generated 2026-10-11T17:48:16Z_

**16 of 1408 non-optional groups pass (1.1%)**; 301 optional groups reported separately

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

| Draft | ✅ | 🙈 | 🚧 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 2 | 2 | 10 | 3 | 143 | 0 | 160 | 1.2% |
| draft6 | 2 | 2 | 9 | 3 | 217 | 0 | 233 | 0.9% |
| draft7 | 2 | 2 | 9 | 3 | 242 | 0 | 258 | 0.8% |
| draft2019-09 | 5 | 7 | 18 | 3 | 339 | 1 | 373 | 1.3% |
| draft2020-12 | 5 | 4 | 15 | 3 | 357 | 0 | 384 | 1.3% |
| **Total** | **16** | **17** | **61** | **15** | **1298** | **1** | **1408** | **1.1%** |

### Optional groups

| Draft | ❌ | Total | Pass rate |
|---|---:|---:|---:|
| draft4 | 39 | 39 | 0.0% |
| draft6 | 44 | 44 | 0.0% |
| draft7 | 64 | 64 | 0.0% |
| draft2019-09 | 76 | 76 | 0.0% |
| draft2020-12 | 78 | 78 | 0.0% |
| **Total** | **301** | **301** | **0.0%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 28 | 50 | 56.0% | 618 | 4.5% |
| draft6 | 26 | 46 | 56.5% | 841 | 3.1% |
| draft7 | 26 | 46 | 56.5% | 929 | 2.8% |
| draft2019-09 | 50 | 95 | 52.6% | 1261 | 4.0% |
| draft2020-12 | 42 | 72 | 58.3% | 1301 | 3.2% |
| **Total** | **172** | **309** | **55.7%** | **4950** | **3.5%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 0 | 0 | - | 391 | 0.0% |
| draft6 | 0 | 0 | - | 520 | 0.0% |
| draft7 | 0 | 0 | - | 922 | 0.0% |
| draft2019-09 | 0 | 0 | - | 1043 | 0.0% |
| draft2020-12 | 0 | 0 | - | 1047 | 0.0% |
| **Total** | **0** | **0** | **-** | **3923** | **0.0%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 9 | 9 | ❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 1 | 0 | 0 | 6 | 7 | ❌❌❌❌❌🙈❌ |
| allOf.json | 1 | 1 | 0 | 0 | 7 | 9 | ✅🙈❌❌❌❌❌❌❌ |
| anyOf.json | 0 | 0 | 4 | 1 | 0 | 5 | 🚧🚧🚧🚧🦀 |
| default.json | 0 | 0 | 0 | 1 | 2 | 3 | ❌❌🦀 |
| definitions.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| enum.json | 1 | 0 | 0 | 0 | 15 | 16 | ❌❌❌✅❌❌❌❌❌❌❌❌❌❌❌❌ |
| format.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| items.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| minLength.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| minimum.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| multipleOf.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| not.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| oneOf.json | 0 | 0 | 6 | 1 | 0 | 7 | 🚧🚧🚧🚧🚧🚧🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 19 | 19 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 8 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| type.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| **Total** | **2** | **2** | **10** | **3** | **143** | **160** |  |

#### optional

| File | ❌ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/ecmascript-regex.json | 20 | 20 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/float-overflow.json | 1 | 1 | ❌ |
| optional/format/date-time.json | 1 | 1 | ❌ |
| optional/format/email.json | 1 | 1 | ❌ |
| optional/format/hostname.json | 1 | 1 | ❌ |
| optional/format/ipv4.json | 1 | 1 | ❌ |
| optional/format/ipv6.json | 1 | 1 | ❌ |
| optional/format/unknown.json | 1 | 1 | ❌ |
| optional/format/uri.json | 1 | 1 | ❌ |
| optional/id.json | 1 | 1 | ❌ |
| optional/non-bmp-regex.json | 2 | 2 | ❌❌ |
| optional/zeroTerminatedFloats.json | 1 | 1 | ❌ |
| **Total** | **39** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 10 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 1 | 0 | 0 | 6 | 7 | ❌❌❌❌❌🙈❌ |
| allOf.json | 1 | 1 | 0 | 0 | 10 | 12 | ✅🙈❌❌❌❌❌❌❌❌❌❌ |
| anyOf.json | 0 | 0 | 4 | 1 | 3 | 8 | 🚧🚧❌❌❌🚧🚧🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 17 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| contains.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| default.json | 0 | 0 | 0 | 1 | 2 | 3 | ❌❌🦀 |
| definitions.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 7 | 7 | ❌❌❌❌❌❌❌ |
| enum.json | 1 | 0 | 0 | 0 | 14 | 15 | ❌❌❌✅❌❌❌❌❌❌❌❌❌❌❌ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 9 | 9 | ❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| items.json | 0 | 0 | 0 | 0 | 9 | 9 | ❌❌❌❌❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| multipleOf.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| not.json | 0 | 0 | 0 | 0 | 8 | 8 | ❌❌❌❌❌❌❌❌ |
| oneOf.json | 0 | 0 | 5 | 1 | 5 | 11 | 🚧🚧❌❌❌❌🚧🚧🚧❌🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 31 | 31 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| type.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| **Total** | **2** | **2** | **9** | **3** | **217** | **233** |  |

#### optional

| File | ❌ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/ecmascript-regex.json | 20 | 20 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/float-overflow.json | 1 | 1 | ❌ |
| optional/format/date-time.json | 1 | 1 | ❌ |
| optional/format/email.json | 1 | 1 | ❌ |
| optional/format/hostname.json | 1 | 1 | ❌ |
| optional/format/ipv4.json | 1 | 1 | ❌ |
| optional/format/ipv6.json | 1 | 1 | ❌ |
| optional/format/json-pointer.json | 1 | 1 | ❌ |
| optional/format/unknown.json | 1 | 1 | ❌ |
| optional/format/uri-reference.json | 1 | 1 | ❌ |
| optional/format/uri-template.json | 1 | 1 | ❌ |
| optional/format/uri.json | 1 | 1 | ❌ |
| optional/id.json | 3 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 2 | 2 | ❌❌ |
| optional/unknownKeyword.json | 1 | 1 | ❌ |
| **Total** | **44** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 10 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 1 | 0 | 0 | 6 | 7 | ❌❌❌❌❌🙈❌ |
| allOf.json | 1 | 1 | 0 | 0 | 10 | 12 | ✅🙈❌❌❌❌❌❌❌❌❌❌ |
| anyOf.json | 0 | 0 | 4 | 1 | 3 | 8 | 🚧🚧❌❌❌🚧🚧🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 17 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| contains.json | 0 | 0 | 0 | 0 | 7 | 7 | ❌❌❌❌❌❌❌ |
| default.json | 0 | 0 | 0 | 1 | 2 | 3 | ❌❌🦀 |
| definitions.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 7 | 7 | ❌❌❌❌❌❌❌ |
| enum.json | 1 | 0 | 0 | 0 | 14 | 15 | ❌❌❌✅❌❌❌❌❌❌❌❌❌❌❌ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 17 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| items.json | 0 | 0 | 0 | 0 | 9 | 9 | ❌❌❌❌❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| multipleOf.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| not.json | 0 | 0 | 0 | 0 | 8 | 8 | ❌❌❌❌❌❌❌❌ |
| oneOf.json | 0 | 0 | 5 | 1 | 5 | 11 | 🚧🚧❌❌❌❌🚧🚧🚧❌🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 35 | 35 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| type.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| **Total** | **2** | **2** | **9** | **3** | **242** | **258** |  |

#### optional

| File | ❌ | Total | Chart |
|---|---:|---:|---|
| optional/bignum.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/content.json | 3 | 3 | ❌❌❌ |
| optional/cross-draft.json | 1 | 1 | ❌ |
| optional/ecmascript-regex.json | 20 | 20 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/float-overflow.json | 1 | 1 | ❌ |
| optional/format/date-time.json | 1 | 1 | ❌ |
| optional/format/date.json | 1 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 6 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 1 | 1 | ❌ |
| optional/format/hostname.json | 2 | 2 | ❌❌ |
| optional/format/idn-email.json | 1 | 1 | ❌ |
| optional/format/idn-hostname.json | 2 | 2 | ❌❌ |
| optional/format/ipv4.json | 1 | 1 | ❌ |
| optional/format/ipv6.json | 1 | 1 | ❌ |
| optional/format/iri-reference.json | 1 | 1 | ❌ |
| optional/format/iri.json | 1 | 1 | ❌ |
| optional/format/json-pointer.json | 1 | 1 | ❌ |
| optional/format/regex.json | 1 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 1 | 1 | ❌ |
| optional/format/time.json | 1 | 1 | ❌ |
| optional/format/unknown.json | 1 | 1 | ❌ |
| optional/format/uri-reference.json | 1 | 1 | ❌ |
| optional/format/uri-template.json | 1 | 1 | ❌ |
| optional/format/uri.json | 1 | 1 | ❌ |
| optional/id.json | 3 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 2 | 2 | ❌❌ |
| optional/unknownKeyword.json | 1 | 1 | ❌ |
| **Total** | **64** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 1 | 0 | 0 | 8 | 0 | 9 | ❌❌❌❌❌🙈❌❌❌ |
| allOf.json | 1 | 1 | 0 | 0 | 10 | 0 | 12 | ✅🙈❌❌❌❌❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 0 | 0 | 4 | 1 | 3 | 0 | 8 | 🚧🚧❌❌❌🚧🚧🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 17 | 0 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| contains.json | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| default.json | 0 | 0 | 0 | 1 | 2 | 0 | 3 | ❌❌🦀 |
| defs.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| enum.json | 1 | 0 | 0 | 0 | 15 | 0 | 16 | ❌❌❌✅❌❌❌❌❌❌❌❌❌❌❌❌ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 19 | 0 | 19 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| maxContains.json | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minContains.json | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| multipleOf.json | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| not.json | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| oneOf.json | 0 | 0 | 5 | 1 | 5 | 0 | 11 | 🚧🚧❌❌❌❌🚧🚧🚧❌🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| recursiveRef.json | 0 | 0 | 6 | 0 | 3 | 0 | 9 | ❌🚧🚧🚧🚧🚧🚧❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 37 | 0 | 37 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 26 | 0 | 26 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 3 | 5 | 3 | 0 | 32 | 1 | 44 | ❌❌❌🙈❌✅✅🙈❌❌❌🚧🚧🙈❌❌❌🙈❌❌❌❌❌❌❌✅❌🙈❌❌🚧❌❌🤯❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| vocabulary.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| **Total** | **5** | **7** | **18** | **3** | **339** | **1** | **373** |  |

#### optional

| File | ❌ | Total | Chart |
|---|---:|---:|---|
| optional/anchor.json | 1 | 1 | ❌ |
| optional/bignum.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/cross-draft.json | 2 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/ecmascript-regex.json | 20 | 20 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/float-overflow.json | 1 | 1 | ❌ |
| optional/format/date-time.json | 1 | 1 | ❌ |
| optional/format/date.json | 1 | 1 | ❌ |
| optional/format/duration.json | 1 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 6 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 1 | 1 | ❌ |
| optional/format/hostname.json | 2 | 2 | ❌❌ |
| optional/format/idn-email.json | 1 | 1 | ❌ |
| optional/format/idn-hostname.json | 2 | 2 | ❌❌ |
| optional/format/ipv4.json | 1 | 1 | ❌ |
| optional/format/ipv6.json | 1 | 1 | ❌ |
| optional/format/iri-reference.json | 1 | 1 | ❌ |
| optional/format/iri.json | 1 | 1 | ❌ |
| optional/format/json-pointer.json | 1 | 1 | ❌ |
| optional/format/regex.json | 1 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 1 | 1 | ❌ |
| optional/format/time.json | 1 | 1 | ❌ |
| optional/format/unknown.json | 1 | 1 | ❌ |
| optional/format/uri-reference.json | 1 | 1 | ❌ |
| optional/format/uri-template.json | 1 | 1 | ❌ |
| optional/format/uri.json | 1 | 1 | ❌ |
| optional/format/uuid.json | 1 | 1 | ❌ |
| optional/id.json | 1 | 1 | ❌ |
| optional/no-schema.json | 1 | 1 | ❌ |
| optional/non-bmp-regex.json | 2 | 2 | ❌❌ |
| optional/refOfUnknownKeyword.json | 5 | 5 | ❌❌❌❌❌ |
| optional/unknownKeyword.json | 1 | 1 | ❌ |
| **Total** | **76** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 0 | 1 | 0 | 0 | 8 | 9 | ❌❌❌❌❌🙈❌❌❌ |
| allOf.json | 1 | 1 | 0 | 0 | 10 | 12 | ✅🙈❌❌❌❌❌❌❌❌❌❌ |
| anchor.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| anyOf.json | 0 | 0 | 4 | 1 | 3 | 8 | 🚧🚧❌❌❌🚧🚧🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 17 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| contains.json | 0 | 0 | 0 | 0 | 7 | 7 | ❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| default.json | 0 | 0 | 0 | 1 | 2 | 3 | ❌❌🦀 |
| defs.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| dynamicRef.json | 0 | 0 | 0 | 0 | 21 | 21 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| enum.json | 1 | 0 | 0 | 0 | 15 | 16 | ❌❌❌✅❌❌❌❌❌❌❌❌❌❌❌❌ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 19 | 19 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| items.json | 0 | 0 | 0 | 0 | 10 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| maxContains.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minContains.json | 0 | 0 | 0 | 0 | 8 | 8 | ❌❌❌❌❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| multipleOf.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| not.json | 0 | 0 | 0 | 0 | 9 | 9 | ❌❌❌❌❌❌❌❌❌ |
| oneOf.json | 0 | 0 | 5 | 1 | 5 | 11 | 🚧🚧❌❌❌❌🚧🚧🚧❌🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 3 | 3 | ❌❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| prefixItems.json | 0 | 0 | 0 | 0 | 4 | 4 | ❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 36 | 36 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 15 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 5 | 5 | ❌❌❌❌❌ |
| type.json | 0 | 0 | 0 | 0 | 11 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| unevaluatedItems.json | 0 | 0 | 3 | 0 | 26 | 29 | ❌❌❌❌❌❌❌❌❌❌❌🚧🚧❌❌❌❌❌❌❌🚧❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 3 | 2 | 3 | 0 | 36 | 44 | ❌❌❌❌❌✅✅🙈❌❌❌🚧🚧❌❌❌❌❌❌❌❌❌❌❌❌✅❌🙈❌❌🚧❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 6 | 6 | ❌❌❌❌❌❌ |
| vocabulary.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| **Total** | **5** | **4** | **15** | **3** | **357** | **384** |  |

#### optional

| File | ❌ | Total | Chart |
|---|---:|---:|---|
| optional/anchor.json | 1 | 1 | ❌ |
| optional/bignum.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/cross-draft.json | 1 | 1 | ❌ |
| optional/dependencies-compatibility.json | 7 | 7 | ❌❌❌❌❌❌❌ |
| optional/dynamicRef.json | 1 | 1 | ❌ |
| optional/ecmascript-regex.json | 20 | 20 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| optional/float-overflow.json | 1 | 1 | ❌ |
| optional/format-assertion.json | 2 | 2 | ❌❌ |
| optional/format/date-time.json | 1 | 1 | ❌ |
| optional/format/date.json | 1 | 1 | ❌ |
| optional/format/duration.json | 1 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 6 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 1 | 1 | ❌ |
| optional/format/hostname.json | 2 | 2 | ❌❌ |
| optional/format/idn-email.json | 1 | 1 | ❌ |
| optional/format/idn-hostname.json | 2 | 2 | ❌❌ |
| optional/format/ipv4.json | 1 | 1 | ❌ |
| optional/format/ipv6.json | 1 | 1 | ❌ |
| optional/format/iri-reference.json | 1 | 1 | ❌ |
| optional/format/iri.json | 1 | 1 | ❌ |
| optional/format/json-pointer.json | 1 | 1 | ❌ |
| optional/format/regex.json | 1 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 1 | 1 | ❌ |
| optional/format/time.json | 1 | 1 | ❌ |
| optional/format/unknown.json | 1 | 1 | ❌ |
| optional/format/uri-reference.json | 1 | 1 | ❌ |
| optional/format/uri-template.json | 1 | 1 | ❌ |
| optional/format/uri.json | 1 | 1 | ❌ |
| optional/format/uuid.json | 1 | 1 | ❌ |
| optional/id.json | 1 | 1 | ❌ |
| optional/no-schema.json | 1 | 1 | ❌ |
| optional/non-bmp-regex.json | 2 | 2 | ❌❌ |
| optional/refOfUnknownKeyword.json | 5 | 5 | ❌❌❌❌❌ |
| optional/unknownKeyword.json | 1 | 1 | ❌ |
| **Total** | **78** | **78** |  |

## Failures

### 🤯 `generation_panic` (1)

- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties + single cyclic ref` — `generation did not finish within 60s`

### ❌ `generation_error` (1599)

90 distinct first-line errors; most common:

- **1175×** `schema at index 0: root schema must have type "object" and non-empty properties`
- **98×** `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- **70×** `invalid JSON Schema: invalid type: boolean 'true', expected struct JsonSchemaHelper`
- **66×** `invalid JSON Schema: invalid type: map, expected a string`
- **36×** `invalid JSON Schema: invalid type: boolean 'false', expected struct JsonSchemaHelper`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::when items is schema, additionalItems does nothing` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::additionalItems as false without items` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalItems.json::additionalItems are allowed by default` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `invalid JSON Schema: invalid type: map, expected a string`
- `draft4/additionalItems.json::additionalItems with null instance elements` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::additionalProperties with schema` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::additionalProperties can exist by itself` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::additionalProperties are allowed by default` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/additionalProperties.json::additionalProperties with null valued instance properties` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/allOf.json::allOf simple types` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::allOf with one empty schema` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::allOf with two empty schemas` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::allOf with the first empty schema` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::allOf with the last empty schema` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::nested allOf, to check validation semantics` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `schema at index 0: allOf subschema at index 0 is not object-like (need type "object" or non-empty properties)`
- `draft4/default.json::invalid type for default` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/default.json::invalid string value for default` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/definitions.json::validate definition against metaschema` — `schema at index 0: could not resolve $ref "http://json-schema.org/draft-04/schema#": UnsupportedRef { ref_str: "http://json-schema.org/draft-04/schema#" }`
- `draft4/dependencies.json::dependencies` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/dependencies.json::multiple dependencies` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/dependencies.json::multiple dependencies subschema` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/dependencies.json::dependencies with escaped characters` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/dependencies.json::dependent subschema incompatible with root` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::simple enum validation` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::heterogeneous enum validation` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::heterogeneous enum-with-null validation` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with escaped characters` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with false does not match 0` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with [false] does not match [0]` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with true does not match 1` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with [true] does not match [1]` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with 0 does not match false` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with [0] does not match [false]` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with 1 does not match true` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::enum with [1] does not match [true]` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::nul characters in strings` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::characters with the same visual representation but different codepoint` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/enum.json::characters with the same visual representation, but different number of codepoints` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/format.json::email format` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/format.json::ipv4 format` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/format.json::ipv6 format` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/format.json::hostname format` — `schema at index 0: root schema must have type "object" and non-empty properties`
- `draft4/format.json::date-time format` — `schema at index 0: root schema must have type "object" and non-empty properties`
- … and 1549 more

### 🦀 `compile_fail` (15)

3 distinct first-line errors; most common:

- **5×** `error[E0308]: mismatched types`
- **5×** `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- **5×** `error[E0428]: the name 'RootOneOf' is defined multiple times`

Per group:

- `draft4/anyOf.json::nested anyOf, to check validation semantics` — `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- `draft4/default.json::the default keyword does not do anything if the property is missing` — `error[E0308]: mismatched types`
- `draft4/oneOf.json::nested oneOf, to check validation semantics` — `error[E0428]: the name 'RootOneOf' is defined multiple times`
- `draft6/anyOf.json::nested anyOf, to check validation semantics` — `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- `draft6/default.json::the default keyword does not do anything if the property is missing` — `error[E0308]: mismatched types`
- `draft6/oneOf.json::nested oneOf, to check validation semantics` — `error[E0428]: the name 'RootOneOf' is defined multiple times`
- `draft7/anyOf.json::nested anyOf, to check validation semantics` — `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- `draft7/default.json::the default keyword does not do anything if the property is missing` — `error[E0308]: mismatched types`
- `draft7/oneOf.json::nested oneOf, to check validation semantics` — `error[E0428]: the name 'RootOneOf' is defined multiple times`
- `draft2019-09/anyOf.json::nested anyOf, to check validation semantics` — `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- `draft2019-09/default.json::the default keyword does not do anything if the property is missing` — `error[E0308]: mismatched types`
- `draft2019-09/oneOf.json::nested oneOf, to check validation semantics` — `error[E0428]: the name 'RootOneOf' is defined multiple times`
- `draft2020-12/anyOf.json::nested anyOf, to check validation semantics` — `error[E0428]: the name 'RootAnyOf' is defined multiple times`
- `draft2020-12/default.json::the default keyword does not do anything if the property is missing` — `error[E0308]: mismatched types`
- `draft2020-12/oneOf.json::nested oneOf, to check validation semantics` — `error[E0428]: the name 'RootOneOf' is defined multiple times`

### 🚧 `false_reject` (61)

19 distinct first-line errors; most common:

- **5×** `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
- **5×** `first anyOf valid: invalid type: integer '1', expected string or map`
- **5×** `first oneOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
- **5×** `first oneOf valid: invalid type: integer '1', expected string or map`
- **5×** `first valid - valid: invalid value: map, expected map with a single key`

Per group:

- `draft4/anyOf.json::anyOf` — `first anyOf valid: invalid type: integer '1', expected string or map`
  - cases: first anyOf valid (rejected), second anyOf valid (rejected), both anyOf valid (rejected)
- `draft4/anyOf.json::anyOf with base schema` — `one anyOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one anyOf valid (rejected)
- `draft4/anyOf.json::anyOf complex types` — `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first anyOf valid (complex) (rejected), second anyOf valid (complex) (rejected), both anyOf valid (complex) (rejected)
- `draft4/anyOf.json::anyOf with one empty schema` — `string is valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft4/oneOf.json::oneOf` — `first oneOf valid: invalid type: integer '1', expected string or map`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft4/oneOf.json::oneOf with base schema` — `one oneOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one oneOf valid (rejected)
- `draft4/oneOf.json::oneOf complex types` — `first oneOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft4/oneOf.json::oneOf with empty schema` — `one valid - valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: one valid - valid (rejected)
- `draft4/oneOf.json::oneOf with required` — `first valid - valid: invalid value: map, expected map with a single key`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft4/oneOf.json::oneOf with missing optional property` — `first oneOf valid: unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft6/anyOf.json::anyOf` — `first anyOf valid: invalid type: integer '1', expected string or map`
  - cases: first anyOf valid (rejected), second anyOf valid (rejected), both anyOf valid (rejected)
- `draft6/anyOf.json::anyOf with base schema` — `one anyOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one anyOf valid (rejected)
- `draft6/anyOf.json::anyOf complex types` — `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first anyOf valid (complex) (rejected), second anyOf valid (complex) (rejected), both anyOf valid (complex) (rejected)
- `draft6/anyOf.json::anyOf with one empty schema` — `string is valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft6/oneOf.json::oneOf` — `first oneOf valid: invalid type: integer '1', expected string or map`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft6/oneOf.json::oneOf with base schema` — `one oneOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one oneOf valid (rejected)
- `draft6/oneOf.json::oneOf complex types` — `first oneOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft6/oneOf.json::oneOf with empty schema` — `one valid - valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: one valid - valid (rejected)
- `draft6/oneOf.json::oneOf with required` — `first valid - valid: invalid value: map, expected map with a single key`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft7/anyOf.json::anyOf` — `first anyOf valid: invalid type: integer '1', expected string or map`
  - cases: first anyOf valid (rejected), second anyOf valid (rejected), both anyOf valid (rejected)
- `draft7/anyOf.json::anyOf with base schema` — `one anyOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one anyOf valid (rejected)
- `draft7/anyOf.json::anyOf complex types` — `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first anyOf valid (complex) (rejected), second anyOf valid (complex) (rejected), both anyOf valid (complex) (rejected)
- `draft7/anyOf.json::anyOf with one empty schema` — `string is valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft7/oneOf.json::oneOf` — `first oneOf valid: invalid type: integer '1', expected string or map`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft7/oneOf.json::oneOf with base schema` — `one oneOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one oneOf valid (rejected)
- `draft7/oneOf.json::oneOf complex types` — `first oneOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft7/oneOf.json::oneOf with empty schema` — `one valid - valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: one valid - valid (rejected)
- `draft7/oneOf.json::oneOf with required` — `first valid - valid: invalid value: map, expected map with a single key`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft2019-09/anyOf.json::anyOf` — `first anyOf valid: invalid type: integer '1', expected string or map`
  - cases: first anyOf valid (rejected), second anyOf valid (rejected), both anyOf valid (rejected)
- `draft2019-09/anyOf.json::anyOf with base schema` — `one anyOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one anyOf valid (rejected)
- `draft2019-09/anyOf.json::anyOf complex types` — `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first anyOf valid (complex) (rejected), second anyOf valid (complex) (rejected), both anyOf valid (complex) (rejected)
- `draft2019-09/anyOf.json::anyOf with one empty schema` — `string is valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: string is valid (rejected), number is valid (rejected)
- `draft2019-09/oneOf.json::oneOf` — `first oneOf valid: invalid type: integer '1', expected string or map`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft2019-09/oneOf.json::oneOf with base schema` — `one oneOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one oneOf valid (rejected)
- `draft2019-09/oneOf.json::oneOf complex types` — `first oneOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft2019-09/oneOf.json::oneOf with empty schema` — `one valid - valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: one valid - valid (rejected)
- `draft2019-09/oneOf.json::oneOf with required` — `first valid - valid: invalid value: map, expected map with a single key`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef without using nesting` — `integer matches at the outer level: invalid type: integer '1', expected string or map`
  - cases: integer matches at the outer level (rejected), single level match (rejected), two levels, properties match with inner definition (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with nesting` — `integer matches at the outer level: invalid type: integer '1', expected string or map`
  - cases: integer matches at the outer level (rejected), single level match (rejected), integer now matches as a property value (rejected), two levels, properties match with inner definition (rejected), two levels, properties match with $recursiveRef (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with $recursiveAnchor: false works like $ref` — `integer matches at the outer level: invalid type: integer '1', expected string or map`
  - cases: integer matches at the outer level (rejected), single level match (rejected), two levels, properties match with inner definition (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with no $recursiveAnchor works like $ref` — `integer matches at the outer level: invalid type: integer '1', expected string or map`
  - cases: integer matches at the outer level (rejected), single level match (rejected), two levels, properties match with inner definition (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial target schema resource` — `leaf node matches: recursion uses the inner schema: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: leaf node matches: recursion uses the inner schema (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with no $recursiveAnchor in the outer schema resource` — `leaf node matches: recursion only uses inner schema: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: leaf node matches: recursion only uses inner schema (rejected)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with anyOf` — `when one matches and has no unevaluated properties: invalid value: map, expected map with a single key`
  - cases: when one matches and has no unevaluated properties (rejected), when two match and has no unevaluated properties (rejected)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with oneOf` — `with no unevaluated properties: invalid value: map, expected map with a single key`
  - cases: with no unevaluated properties (rejected)
- `draft2019-09/unevaluatedProperties.json::property is evaluated in an uncle schema to unevaluatedProperties` — `no extra properties: unknown variant 'foo', expected 'Variant0'`
  - cases: no extra properties (rejected)
- `draft2020-12/anyOf.json::anyOf` — `first anyOf valid: invalid type: integer '1', expected string or map`
  - cases: first anyOf valid (rejected), second anyOf valid (rejected), both anyOf valid (rejected)
- `draft2020-12/anyOf.json::anyOf with base schema` — `one anyOf valid: unknown variant 'foobar', expected 'Variant0' or 'Variant1'`
  - cases: one anyOf valid (rejected)
- `draft2020-12/anyOf.json::anyOf complex types` — `first anyOf valid (complex): unknown variant 'bar', expected 'Variant0' or 'Variant1'`
  - cases: first anyOf valid (complex) (rejected), second anyOf valid (complex) (rejected), both anyOf valid (complex) (rejected)
- `draft2020-12/anyOf.json::anyOf with one empty schema` — `string is valid: unknown variant 'foo', expected 'Variant0' or 'Variant1'`
  - cases: string is valid (rejected), number is valid (rejected)
- … and 11 more

### 🙈 `false_accept` (17)

5 distinct first-line errors; most common:

- **5×** `accepted 'mismatch base schema' though valid: false`
- **5×** `accepted 'properties defined in allOf are not examined' though valid: false`
- **3×** `accepted 'with unevaluated properties' though valid: false`
- **2×** `accepted 'with additional properties' though valid: false`
- **2×** `accepted 'with nested unevaluated properties' though valid: false`

Per group:

- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft4/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch second allOf (accepted)
- `draft6/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft6/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch second allOf (accepted)
- `draft7/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft7/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch second allOf (accepted)
- `draft2019-09/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft2019-09/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch second allOf (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with adjacent properties` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with nested properties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with not` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with dependentSchemas` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties inside` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)
- `draft2020-12/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft2020-12/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch second allOf (accepted)
- `draft2020-12/unevaluatedProperties.json::unevaluatedProperties with nested properties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2020-12/unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties inside` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)

