# ajv 8.20.0

The JSON Schema validator for JavaScript, and for much of the ecosystem the
reference implementation: it compiles a schema into purpose-built JS and runs
that at the boundary. No types come out; the schema stays the artifact. It's
the reference row for the Node path, resolving `localhost:1234` remotes from
`suite/remotes/` and shipping no formats, so `format` is ignored everywhere.

_generated 2026-10-11T18:47:47Z_

**1348 of 1408 non-optional groups pass (95.7%)**; 301 optional groups reported separately

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

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 156 | 1 | 2 | 1 | 0 | 0 | 0 | 160 | 97.5% |
| draft6 | 229 | 1 | 2 | 1 | 0 | 0 | 0 | 233 | 98.3% |
| draft7 | 254 | 1 | 2 | 1 | 0 | 0 | 0 | 258 | 98.4% |
| draft2019-09 | 361 | 1 | 4 | 3 | 0 | 1 | 3 | 373 | 96.8% |
| draft2020-12 | 348 | 6 | 17 | 2 | 4 | 4 | 3 | 384 | 90.6% |
| **Total** | **1348** | **10** | **27** | **8** | **4** | **5** | **6** | **1408** | **95.7%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | ❌ | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|
| draft4 | 31 | 7 | 1 | 0 | 39 | 79.5% |
| draft6 | 33 | 9 | 1 | 1 | 44 | 75.0% |
| draft7 | 36 | 26 | 1 | 1 | 64 | 56.2% |
| draft2019-09 | 48 | 25 | 2 | 1 | 76 | 63.2% |
| draft2020-12 | 48 | 24 | 3 | 3 | 78 | 61.5% |
| **Total** | **196** | **91** | **8** | **6** | **301** | **65.1%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 610 | 618 | 98.7% | 618 | 98.7% |
| draft6 | 833 | 841 | 99.0% | 841 | 99.0% |
| draft7 | 921 | 929 | 99.1% | 929 | 99.1% |
| draft2019-09 | 1233 | 1247 | 98.9% | 1261 | 97.8% |
| draft2020-12 | 1239 | 1281 | 96.7% | 1301 | 95.2% |
| **Total** | **4836** | **4916** | **98.4%** | **4950** | **97.7%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 218 | 391 | 55.8% | 391 | 55.8% |
| draft6 | 294 | 517 | 56.9% | 520 | 56.5% |
| draft7 | 475 | 919 | 51.7% | 922 | 51.5% |
| draft2019-09 | 560 | 1040 | 53.8% | 1043 | 53.7% |
| draft2020-12 | 560 | 1040 | 53.8% | 1047 | 53.5% |
| **Total** | **2107** | **3907** | **53.9%** | **3923** | **53.7%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| additionalItems.json | 9 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 9 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| enum.json | 16 | 0 | 0 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| format.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| maxItems.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| maxLength.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| maxProperties.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| maximum.json | 4 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| minItems.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| minLength.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| minProperties.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| minimum.json | 4 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| oneOf.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 4 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| properties.json | 4 | 0 | 1 | 0 | 5 | ✅✅✅✅🚧 |
| ref.json | 17 | 0 | 1 | 1 | 19 | ✅✅✅✅✅🚧🤪✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 8 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| required.json | 3 | 1 | 0 | 0 | 4 | ✅✅✅🙈 |
| type.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **156** | **1** | **2** | **1** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | Total | Chart |
|---|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 0 | 1 | 0 | 1 | 🙈 |
| optional/id.json | 1 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 2 | ✅✅ |
| optional/zeroTerminatedFloats.json | 0 | 1 | 0 | 1 | 🙈 |
| **Total** | **31** | **7** | **1** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 9 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 5 | 0 | 1 | 0 | 6 | ✅✅✅✅✅🚧 |
| propertyNames.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 29 | 0 | 1 | 1 | 31 | ✅✅✅✅✅🚧🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 4 | 1 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| type.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **229** | **1** | **2** | **1** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 1 | ❌ |
| **Total** | **33** | **9** | **1** | **1** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 17 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 5 | 0 | 1 | 0 | 6 | ✅✅✅✅✅🚧 |
| propertyNames.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 33 | 0 | 1 | 1 | 35 | ✅✅✅✅✅🚧🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 4 | 1 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| type.json | 11 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **254** | **1** | **2** | **1** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/content.json | 0 | 3 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 3 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 1 | ❌ |
| **Total** | **36** | **26** | **1** | **1** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 9 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependentRequired.json | 4 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| enum.json | 15 | 0 | 0 | 0 | 1 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅❌✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minContains.json | 8 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 5 | 0 | 1 | 0 | 0 | 0 | 6 | ✅✅✅✅✅🚧 |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| recursiveRef.json | 8 | 0 | 0 | 1 | 0 | 0 | 9 | ✅✅✅✅✅🤪✅✅✅ |
| ref.json | 34 | 0 | 0 | 0 | 0 | 3 | 37 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤯🤯✅✅✅✅✅✅✅✅✅✅✅🤯✅✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 4 | 1 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 24 | 0 | 1 | 1 | 0 | 0 | 26 | ✅✅✅✅✅✅✅✅✅🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅🚧✅ |
| unevaluatedProperties.json | 42 | 0 | 1 | 1 | 0 | 0 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🚧✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 1 | 0 | 1 | 0 | 0 | 0 | 2 | 🚧✅ |
| **Total** | **361** | **1** | **4** | **3** | **1** | **3** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 0 | 1 | 1 | 0 | 2 | 🙈🚧 |
| optional/dependencies-compatibility.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 1 | ❌ |
| **Total** | **48** | **25** | **2** | **1** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependentRequired.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dynamicRef.json | 2 | 2 | 12 | 0 | 2 | 3 | 0 | 21 | 🚧🚧✅🚧🚧🚧🚧🚧🚧❌❌🙈❌✅🚧🚧🚧💀🙈🚧💀 |
| enum.json | 15 | 0 | 0 | 0 | 0 | 1 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅❌✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minContains.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| patternProperties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| prefixItems.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| properties.json | 5 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅🚧 |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 33 | 0 | 0 | 0 | 0 | 0 | 3 | 36 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤯🤯✅✅✅✅✅✅✅✅✅✅✅🤯✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 22 | 3 | 2 | 1 | 1 | 0 | 0 | 29 | ✅✅✅✅✅✅✅✅🤪✅✅✅✅✅✅✅✅✅💀✅✅🙈🙈🙈🚧✅✅🚧✅ |
| unevaluatedProperties.json | 41 | 0 | 1 | 1 | 1 | 0 | 0 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤪✅✅✅✅✅💀✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🚧✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🚧✅ |
| **Total** | **348** | **6** | **17** | **2** | **4** | **4** | **3** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/dependencies-compatibility.json | 7 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/dynamicRef.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 1 | ❌ |
| **Total** | **48** | **24** | **3** | **3** | **78** |  |

## Failures

### 🤯 `generation_panic` (6)

1 distinct first-line errors; most common:

- **6×** `RangeError: Maximum call stack size exceeded`

Per group:

- `draft2019-09/ref.json::refs with relative uris and defs` — `RangeError: Maximum call stack size exceeded`
- `draft2019-09/ref.json::relative refs with absolute uris and defs` — `RangeError: Maximum call stack size exceeded`
- `draft2019-09/ref.json::URN ref with nested pointer ref` — `RangeError: Maximum call stack size exceeded`
- `draft2020-12/ref.json::refs with relative uris and defs` — `RangeError: Maximum call stack size exceeded`
- `draft2020-12/ref.json::relative refs with absolute uris and defs` — `RangeError: Maximum call stack size exceeded`
- `draft2020-12/ref.json::URN ref with nested pointer ref` — `RangeError: Maximum call stack size exceeded`

### ❌ `generation_error` (11)

7 distinct first-line errors; most common:

- **3×** `Error: "$dynamicRef" only supports hash fragment reference`
- **2×** `Error: enum must have non-empty array`
- **2×** `Error: reference "https://localhost:1234/unknownKeyword/my_identifier.json" resolves to more than one schema`
- **1×** `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-false.jso…`
- **1×** `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-true.json`

Per group:

- `draft2019-09/enum.json::empty enum` — `Error: enum must have non-empty array`
- `draft2020-12/dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope` — `Error: "$dynamicRef" only supports hash fragment reference`
- `draft2020-12/dynamicRef.json::A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor` — `Error: "$dynamicRef" only supports hash fragment reference`
- `draft2020-12/dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef` — `Error: "$dynamicRef" only supports hash fragment reference`
- `draft2020-12/enum.json::empty enum` — `Error: enum must have non-empty array`
- `draft6/optional/unknownKeyword.json::$id inside an unknown keyword is not a real identifier` — `Error: reference "https://localhost:1234/unknownKeyword/my_identifier.json" resolves to more than one schema`
- `draft7/optional/unknownKeyword.json::$id inside an unknown keyword is not a real identifier` — `Error: reference "https://localhost:1234/unknownKeyword/my_identifier.json" resolves to more than one schema`
- `draft2019-09/optional/unknownKeyword.json::$id inside an unknown keyword is not a real identifier` — `Error: reference "https://localhost:1234/draft2019-09/unknownKeyword/my_identifier.json" resolves to more than one schema`
- `draft2020-12/optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false` — `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-false.jso…`
- `draft2020-12/optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true` — `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-true.json`
- `draft2020-12/optional/unknownKeyword.json::$id inside an unknown keyword is not a real identifier` — `Error: reference "https://localhost:1234/draft2020-12/unknownKeyword/my_identifier.json" resolves to more than one schema`

### 💀 `runtime_panic` (4)

- `draft2020-12/dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor` — `number is valid: Maximum call stack size exceeded`
  - cases: number is valid (panicked), non-number is invalid (panicked)
- `draft2020-12/dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered` — `data is sufficient for schema at second#/$defs/length: Maximum call stack size exceeded`
  - cases: data is sufficient for schema at second#/$defs/length (panicked), data is not sufficient for schema at second#/$defs/length (panicked)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with $dynamicRef` — `with no unevaluated items: Maximum call stack size exceeded`
  - cases: with no unevaluated items (panicked), with unevaluated items (panicked)
- `draft2020-12/unevaluatedProperties.json::unevaluatedProperties with $dynamicRef` — `with no unevaluated properties: Maximum call stack size exceeded`
  - cases: with no unevaluated properties (panicked), with unevaluated properties (panicked)

### 🤪 `mixed` (8)

4 distinct first-line errors; most common:

- **3×** `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
- **2×** `when if is true and has no unevaluated properties: data must NOT have unevaluated properties`
- **2×** `with no additional items: data/true must be boolean`
- **1×** `leaf node matches: recursion uses the inner schema: data must be boolean, data/foo must be integer, data/foo/bar must be boolean, data/foo/bar must be object, d…`

Per group:

- `draft4/ref.json::$ref prevents a sibling id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft6/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft7/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft2019-09/recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial target schema resource` — `leaf node matches: recursion uses the inner schema: data must be boolean, data/foo must be integer, data/foo/bar must be boolean, data/foo/bar must be object, d…`
  - cases: leaf node matches: recursion uses the inner schema (rejected), leaf node does not match: recursion uses the inner schema (accepted)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with nested items` — `with no additional items: data/true must be boolean`
  - cases: with no additional items (rejected), with invalid additional item (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with if/then/else, then not defined` — `when if is true and has no unevaluated properties: data must NOT have unevaluated properties`
  - cases: when if is true and has no unevaluated properties (rejected), when if is false and has unevaluated properties (accepted)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with nested items` — `with no additional items: data/true must be boolean`
  - cases: with no additional items (rejected), with invalid additional item (accepted)
- `draft2020-12/unevaluatedProperties.json::unevaluatedProperties with if/then/else, then not defined` — `when if is true and has no unevaluated properties: data must NOT have unevaluated properties`
  - cases: when if is true and has no unevaluated properties (rejected), when if is false and has unevaluated properties (accepted)

### 🚧 `false_reject` (35)

14 distinct first-line errors; most common:

- **5×** `none of the properties mentioned: data/constructor must be number`
- **5×** `valid if optional overflow handling is implemented: data must be multiple of 0.5`
- **4×** `An array of strings is valid: data/0 must be array`
- **3×** `Any array is valid: data/0 must be array`
- **3×** `correct extended schema: data/elements/0 must have required property 'elements'`

Per group:

- `draft4/properties.json::properties whose names are Javascript object property names` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `draft4/ref.json::ref overrides any sibling keywords` — `ref valid, maxItems ignored: data/foo must NOT have more than 2 items`
  - cases: ref valid, maxItems ignored (rejected)
- `draft6/properties.json::properties whose names are Javascript object property names` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `draft6/ref.json::ref overrides any sibling keywords` — `ref valid, maxItems ignored: data/foo must NOT have more than 2 items`
  - cases: ref valid, maxItems ignored (rejected)
- `draft7/properties.json::properties whose names are Javascript object property names` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `draft7/ref.json::ref overrides any sibling keywords` — `ref valid, maxItems ignored: data/foo must NOT have more than 2 items`
  - cases: ref valid, maxItems ignored (rejected)
- `draft2019-09/properties.json::properties whose names are Javascript object property names` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems can see annotations from if without then and else` — `valid in case if is evaluated: data must NOT have more than 0 items`
  - cases: valid in case if is evaluated (rejected)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties can see annotations from if without then and else` — `valid in case if is evaluated: data must NOT have unevaluated properties`
  - cases: valid in case if is evaluated (rejected)
- `draft2019-09/vocabulary.json::schema that uses custom metaschema with with no validation vocabulary` — `no validation: invalid number, but it still validates: data/numberProperty must be >= 10`
  - cases: no validation: invalid number, but it still validates (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref` — `An array of numbers is valid: data/0 must be array`
  - cases: An array of numbers is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `draft2020-12/dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `draft2020-12/dynamicRef.json::A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `draft2020-12/dynamicRef.json::tests for implementation dynamic anchor and reference link` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `draft2020-12/dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `draft2020-12/dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `draft2020-12/dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference` — `integer property passes: data/bar-item/content must be object`
  - cases: integer property passes (rejected)
- `draft2020-12/properties.json::properties whose names are Javascript object property names` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with minContains = 0` — `all items evaluated by contains: data must NOT have more than 0 items`
  - cases: all items evaluated by contains (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems can see annotations from if without then and else` — `valid in case if is evaluated: data must NOT have more than 0 items`
  - cases: valid in case if is evaluated (rejected)
- `draft2020-12/unevaluatedProperties.json::unevaluatedProperties can see annotations from if without then and else` — `valid in case if is evaluated: data must NOT have unevaluated properties`
  - cases: valid in case if is evaluated (rejected)
- `draft2020-12/vocabulary.json::schema that uses custom metaschema with with no validation vocabulary` — `no validation: invalid number, but it still validates: data/numberProperty must be >= 10`
  - cases: no validation: invalid number, but it still validates (rejected)
- `draft4/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft6/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft7/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2019-09/optional/cross-draft.json::refs to historic drafts are processed as historic drafts` — `missing bar is valid: data must have property bar when property foo is present`
  - cases: missing bar is valid (rejected)
- `draft2019-09/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2020-12/optional/cross-draft.json::refs to historic drafts are processed as historic drafts` — `first item not a string is valid: data/0 must be string`
  - cases: first item not a string is valid (rejected)
- `draft2020-12/optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary` — `integer property passes: data/bar-item/content must be object`
  - cases: integer property passes (rejected)
- `draft2020-12/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)

### 🙈 `false_accept` (101)

37 distinct first-line errors; most common:

- **5×** `accepted 'a group with 5 hex digits is invalid' though valid: false`
- **5×** `accepted 'an IP address with too many components' though valid: false`
- **5×** `accepted 'an invalid date-time past leap second, UTC' though valid: false`
- **5×** `accepted 'an invalid e-mail address' though valid: false`
- **5×** `accepted 'an invalid protocol-relative URI Reference' though valid: false`

Per group:

- `draft4/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft6/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft7/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft2019-09/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft2020-12/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword` — `accepted 'number list with string values' though valid: false`
  - cases: number list with string values (accepted), string list with number values (accepted)
- `draft2020-12/dynamicRef.json::$dynamicRef points to a boolean schema` — `accepted 'follow $dynamicRef to a false schema' though valid: false`
  - cases: follow $dynamicRef to a false schema (accepted)
- `draft2020-12/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems depends on adjacent contains` — `accepted 'contains passes, second item is not evaluated' though valid: false`
  - cases: contains passes, second item is not evaluated (accepted)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems depends on multiple nested contains` — `accepted '7 not evaluated, fails unevaluatedItems' though valid: false`
  - cases: 7 not evaluated, fails unevaluatedItems (accepted)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems and contains interact to control item dependency relationship` — `accepted 'only b's are invalid' though valid: false`
  - cases: only b's are invalid (accepted), only c's are invalid (accepted), only b's and c's are invalid (accepted), only a's and c's are invalid (accepted)
- `draft4/optional/format/date-time.json::validation of date-time strings` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid hour in date-time string (accepted), an invalid minute in date-time string (accepted), an invalid offset minute in date-time string (accepted), … and 20 more
- `draft4/optional/format/email.json::validation of e-mail addresses` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), an invalid domain (accepted), an invalid IPv4-address-literal (accepted), … and 38 more
- `draft4/optional/format/hostname.json::validation of host names` — `accepted 'a host name starting with an illegal character' though valid: false`
  - cases: a host name starting with an illegal character (accepted), a host name containing illegal characters (accepted), a host name with a component too long (accepted), starts with hyphen (accepted), ends with hyphen (accepted), starts with underscore (accepted), … and 11 more
- `draft4/optional/format/ipv4.json::validation of IP addresses` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `draft4/optional/format/ipv6.json::validation of IPv6 addresses` — `accepted 'a group with 5 hex digits is invalid' though valid: false`
  - cases: a group with 5 hex digits is invalid (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `draft4/optional/format/uri.json::validation of URIs` — `accepted 'an invalid protocol-relative URI Reference' though valid: false`
  - cases: an invalid protocol-relative URI Reference (accepted), an invalid relative URI Reference (accepted), an invalid URI (accepted), an invalid URI though valid URI reference (accepted), an invalid URI with spaces (accepted), an invalid URI with spaces and missing scheme (accepted), … and 20 more
- `draft4/optional/zeroTerminatedFloats.json::some languages do not distinguish between different types of numeric value` — `accepted 'a float is not an integer even without fractional part' though valid: false`
  - cases: a float is not an integer even without fractional part (accepted)
- `draft6/optional/format/date-time.json::validation of date-time strings` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid hour in date-time string (accepted), an invalid minute in date-time string (accepted), an invalid offset minute in date-time string (accepted), … and 24 more
- `draft6/optional/format/email.json::validation of e-mail addresses` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), an invalid domain (accepted), an invalid IPv4-address-literal (accepted), … and 38 more
- `draft6/optional/format/hostname.json::validation of host names` — `accepted 'a host name starting with an illegal character' though valid: false`
  - cases: a host name starting with an illegal character (accepted), a host name containing illegal characters (accepted), a host name with a component too long (accepted), starts with hyphen (accepted), ends with hyphen (accepted), starts with underscore (accepted), … and 11 more
- `draft6/optional/format/ipv4.json::validation of IP addresses` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `draft6/optional/format/ipv6.json::validation of IPv6 addresses` — `accepted 'a group with 5 hex digits is invalid' though valid: false`
  - cases: a group with 5 hex digits is invalid (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `draft6/optional/format/json-pointer.json::validation of JSON-pointers (JSON String Representation)` — `accepted 'not a valid JSON-pointer (~ not escaped)' though valid: false`
  - cases: not a valid JSON-pointer (~ not escaped) (accepted), not a valid JSON-pointer (URI Fragment Identifier) #1 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #2 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #3 (accepted), not a valid JSON-pointer (some escaped, but not all) #1 (accepted), not a valid JSON-pointer (some escaped, but not all) #2 (accepted), … and 6 more
- `draft6/optional/format/uri-reference.json::validation of URI References` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- `draft6/optional/format/uri-template.json::format: uri-template` — `accepted 'an invalid uri-template' though valid: false`
  - cases: an invalid uri-template (accepted), an empty expression is invalid (accepted), an empty varspec inside the variable list is invalid (accepted), a zero prefix max-length is invalid (accepted), a five-digit prefix max-length is invalid (accepted), an unmatched closing brace is invalid (accepted), … and 15 more
- `draft6/optional/format/uri.json::validation of URIs` — `accepted 'an invalid protocol-relative URI Reference' though valid: false`
  - cases: an invalid protocol-relative URI Reference (accepted), an invalid relative URI Reference (accepted), an invalid URI (accepted), an invalid URI though valid URI reference (accepted), an invalid URI with spaces (accepted), an invalid URI with spaces and missing scheme (accepted), … and 20 more
- `draft7/optional/content.json::validation of string-encoded content based on media type` — `accepted 'an invalid JSON document' though valid: false`
  - cases: an invalid JSON document (accepted)
- `draft7/optional/content.json::validation of binary string-encoding` — `accepted 'an invalid base64 string (% is not a valid character)' though valid: false`
  - cases: an invalid base64 string (% is not a valid character) (accepted)
- `draft7/optional/content.json::validation of binary-encoded media type documents` — `accepted 'a validly-encoded invalid JSON document' though valid: false`
  - cases: a validly-encoded invalid JSON document (accepted), an invalid base64 string that is valid JSON (accepted)
- `draft7/optional/cross-draft.json::refs to future drafts are processed as future drafts` — `accepted 'missing bar is invalid' though valid: false`
  - cases: missing bar is invalid (accepted)
- `draft7/optional/format/date-time.json::validation of date-time strings` — `accepted 'an invalid date-time past leap second, UTC' though valid: false`
  - cases: an invalid date-time past leap second, UTC (accepted), an invalid date-time with leap second on a wrong minute, UTC (accepted), an invalid date-time with leap second on a wrong hour, UTC (accepted), an invalid hour in date-time string (accepted), an invalid minute in date-time string (accepted), an invalid offset minute in date-time string (accepted), … and 24 more
- `draft7/optional/format/date.json::validation of date strings` — `accepted 'a invalid date string with 32 days in January' though valid: false`
  - cases: a invalid date string with 32 days in January (accepted), a invalid date string with 30 days in February (leap) (accepted), a invalid date string with 32 days in March (accepted), a invalid date string with 31 days in April (accepted), a invalid date string with 32 days in May (accepted), a invalid date string with 31 days in June (accepted), … and 52 more
- `draft7/optional/format/ecmascript-regex.json::\a is not an ECMA 262 control escape` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `draft7/optional/format/ecmascript-regex.json::Python-specific regular expression syntax is not valid ECMA 262` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `draft7/optional/format/ecmascript-regex.json::global inline flag groups are not valid ECMA 262` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `draft7/optional/format/email.json::validation of e-mail addresses` — `accepted 'an invalid e-mail address' though valid: false`
  - cases: an invalid e-mail address (accepted), dot before local part is not valid (accepted), dot after local part is not valid (accepted), two subsequent dots inside local part are not valid (accepted), an invalid domain (accepted), an invalid IPv4-address-literal (accepted), … and 38 more
- `draft7/optional/format/hostname.json::validation of host names` — `accepted 'empty string' though valid: false`
  - cases: empty string (accepted), single dot (accepted), leading dot (accepted), trailing dot (accepted), IDN label separator (accepted), starts with hyphen (accepted), … and 6 more
- `draft7/optional/format/hostname.json::validation of A-label (punycode) host names` — `accepted 'invalid Punycode' though valid: false`
  - cases: invalid Punycode (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), Begins with a Spacing Combining Mark (accepted), Begins with a Nonspacing Mark (accepted), Begins with an Enclosing Mark (accepted), Exceptions that are DISALLOWED, right-to-left chars (accepted), … and 17 more
- `draft7/optional/format/idn-email.json::validation of an internationalized e-mail addresses` — `accepted 'an invalid e-mail/idn e-mail address' though valid: false`
  - cases: an invalid e-mail/idn e-mail address (accepted), a fullwidth commercial at is not a local-part separator (accepted), a domain label starting with a hyphen is not valid (accepted), a domain label ending with a hyphen is not valid (accepted), a domain starting with a dot is not valid (accepted), a domain ending with a dot is not valid (accepted), … and 2 more
- `draft7/optional/format/idn-hostname.json::validation of internationalized host names` — `accepted 'illegal first char U+302E Hangul single dot tone mark' though valid: false`
  - cases: illegal first char U+302E Hangul single dot tone mark (accepted), contains illegal char U+302E Hangul single dot tone mark (accepted), a single label of 64 characters is too long (accepted), invalid label, correct Punycode (accepted), invalid Punycode (accepted), U-label contains "--" in the 3rd and 4th position (accepted), … and 37 more
- `draft7/optional/format/idn-hostname.json::validation of separators in internationalized host names` — `accepted 'single dot' though valid: false`
  - cases: single dot (accepted), single ideographic full stop (accepted), single fullwidth full stop (accepted), single halfwidth ideographic full stop (accepted), leading dot (accepted), leading ideographic full stop (accepted), … and 6 more
- `draft7/optional/format/ipv4.json::validation of IP addresses` — `accepted 'an IP address with too many components' though valid: false`
  - cases: an IP address with too many components (accepted), an IP address with out-of-range values (accepted), an IP address without 4 components (accepted), a 2-part address resolving to a routable IP (inet_aton shorthand) (accepted), a 3-part address resolving to a routable IP (inet_aton shorthand) (accepted), an IP address as an integer (accepted), … and 24 more
- `draft7/optional/format/ipv6.json::validation of IPv6 addresses` — `accepted 'a group with 5 hex digits is invalid' though valid: false`
  - cases: a group with 5 hex digits is invalid (accepted), trailing 5 hex symbols is invalid (accepted), an IPv6 address with too many components (accepted), an IPv6 address containing illegal characters (accepted), missing leading octet is invalid (accepted), missing trailing octet is invalid (accepted), … and 22 more
- `draft7/optional/format/iri-reference.json::validation of IRI References` — `accepted 'an invalid IRI Reference' though valid: false`
  - cases: an invalid IRI Reference (accepted), an invalid IRI fragment (accepted), an embedded IPv4 with a leading zero is invalid (accepted), a trailing newline after a valid IRI Reference is invalid (accepted), invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), … and 1 more
- `draft7/optional/format/iri.json::validation of IRIs` — `accepted 'invalid percent-encoding with non-hex digits' though valid: false`
  - cases: invalid percent-encoding with non-hex digits (accepted), incomplete percent-encoding triplet (accepted), lone percent sign is invalid (accepted), an IPv6 address without enclosing brackets is invalid (accepted), an invalid relative IRI Reference (accepted), an invalid IRI (accepted), … and 3 more
- `draft7/optional/format/json-pointer.json::validation of JSON-pointers (JSON String Representation)` — `accepted 'not a valid JSON-pointer (~ not escaped)' though valid: false`
  - cases: not a valid JSON-pointer (~ not escaped) (accepted), not a valid JSON-pointer (URI Fragment Identifier) #1 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #2 (accepted), not a valid JSON-pointer (URI Fragment Identifier) #3 (accepted), not a valid JSON-pointer (some escaped, but not all) #1 (accepted), not a valid JSON-pointer (some escaped, but not all) #2 (accepted), … and 6 more
- `draft7/optional/format/regex.json::validation of regular expressions` — `accepted 'a regular expression with unclosed parens is invalid' though valid: false`
  - cases: a regular expression with unclosed parens is invalid (accepted), a quantifier with nothing to repeat is invalid (accepted), a character class range in reverse order is invalid (accepted), a trailing backslash is invalid (accepted), an unmatched closing parenthesis is invalid (accepted), an unterminated character class is invalid (accepted)
- `draft7/optional/format/relative-json-pointer.json::validation of Relative JSON Pointers (RJP)` — `accepted 'an invalid RJP that is a valid JSON Pointer' though valid: false`
  - cases: an invalid RJP that is a valid JSON Pointer (accepted), a json-pointer part that does not start with a slash is invalid (accepted), negative prefix (accepted), explicit positive prefix (accepted), non-ASCII digit in the prefix is not allowed (accepted), ## is not a valid json-pointer (accepted), … and 7 more
- `draft7/optional/format/time.json::validation of time strings` — `accepted 'invalid time string with extra leading zeros' though valid: false`
  - cases: invalid time string with extra leading zeros (accepted), invalid time string with no leading zero for single digit (accepted), hour, minute, second must be two digits (accepted), invalid leap second, Zulu (wrong hour) (accepted), invalid leap second, Zulu (wrong minute) (accepted), invalid leap second, zero time-offset (wrong hour) (accepted), … and 30 more
- `draft7/optional/format/uri-reference.json::validation of URI References` — `accepted 'an invalid URI Reference' though valid: false`
  - cases: an invalid URI Reference (accepted), an invalid URI fragment (accepted), unescaped non US-ASCII characters (accepted), invalid backslash character (accepted), an incomplete percent-encoding (accepted), incomplete percent-encoding triplet (accepted), … and 8 more
- … and 51 more

