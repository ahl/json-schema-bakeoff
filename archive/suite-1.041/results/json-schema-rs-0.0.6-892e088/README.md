# json-schema-rs 0.0.6+892e088

A young Rust generator aimed at hand-written config schemas, pinned to an
unreleased main: codegen rebuilt around an intermediate graph that
handles any root shape, recursion, `allOf` as a conjunction, and `const`.
Unions are `#[serde(untagged)]`, so `oneOf` exclusivity isn't checked, and
constraints come out as attributes for its own validator, which serde ignores.
The published 0.0.6 row (results kept, hidden) refused any root that wasn't
an object. The codegen measured here was written by Claude Code: the repo
is run from a CLAUDE.md and a contribution skill, and every commit of the
type-graph rewrite (about 38k lines, five days in October 2026) carries a
Claude session tag.

_generated 2026-10-31T16:46:40Z_

**619 of 1408 non-optional groups pass (44.0%)**; 301 optional groups reported separately

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
| draft4 | 77 | 48 | 15 | 12 | 0 | 0 | 8 | 0 | 160 | 48.1% |
| draft6 | 115 | 71 | 19 | 12 | 0 | 1 | 15 | 0 | 233 | 49.4% |
| draft7 | 123 | 80 | 27 | 12 | 0 | 1 | 15 | 0 | 258 | 47.7% |
| draft2019-09 | 151 | 161 | 29 | 12 | 0 | 1 | 19 | 0 | 373 | 40.5% |
| draft2020-12 | 153 | 164 | 30 | 12 | 0 | 1 | 24 | 0 | 384 | 39.8% |
| **Total** | **619** | **524** | **120** | **60** | **0** | **4** | **81** | **0** | **1408** | **44.0%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 7 | 19 | 7 | 6 | 0 | 0 | 0 | 0 | 39 | 17.9% |
| draft6 | 7 | 20 | 8 | 9 | 0 | 0 | 0 | 0 | 44 | 15.9% |
| draft7 | 8 | 30 | 8 | 17 | 0 | 0 | 1 | 0 | 64 | 12.5% |
| draft2019-09 | 15 | 31 | 8 | 20 | 0 | 0 | 2 | 0 | 76 | 19.7% |
| draft2020-12 | 15 | 34 | 8 | 20 | 0 | 0 | 1 | 0 | 78 | 19.2% |
| **Total** | **52** | **134** | **39** | **72** | **0** | **0** | **4** | **0** | **301** | **17.3%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 414 | 601 | 68.9% | 618 | 67.0% |
| draft6 | 543 | 790 | 68.7% | 841 | 64.6% |
| draft7 | 572 | 878 | 65.1% | 929 | 61.6% |
| draft2019-09 | 762 | 1202 | 63.4% | 1261 | 60.4% |
| draft2020-12 | 781 | 1229 | 63.5% | 1301 | 60.0% |
| **Total** | **3072** | **4700** | **65.4%** | **4950** | **62.1%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 131 | 391 | 33.5% | 391 | 33.5% |
| draft6 | 188 | 520 | 36.2% | 520 | 36.2% |
| draft7 | 318 | 920 | 34.6% | 922 | 34.5% |
| draft2019-09 | 377 | 1040 | 36.2% | 1043 | 36.1% |
| draft2020-12 | 380 | 1046 | 36.3% | 1047 | 36.3% |
| **Total** | **1394** | **3917** | **35.6%** | **3923** | **35.5%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 4 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧🚧✅✅✅🙈✅ |
| allOf.json | 7 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅🙈✅✅✅✅✅🙈 |
| anyOf.json | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 15 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| format.json | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 6 | 🚧🚧🚧🚧🚧🚧 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 3 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 6 | 🚧🙈🙈✅✅✅ |
| maxItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| maxLength.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maximum.json | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 4 | 🤪🙈🤪🙈 |
| minItems.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| minLength.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minimum.json | 0 | 1 | 0 | 3 | 0 | 0 | 0 | 0 | 4 | 🤪🤪🙈🤪 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| oneOf.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈✅ |
| properties.json | 2 | 0 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧🤪✅✅🚧 |
| ref.json | 16 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 19 | 🚧✅🙈✅✅✅✅🙈✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 4 | 🚧✅🙈🤪 |
| type.json | 10 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅🚧 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **77** | **48** | **15** | **12** | **0** | **0** | **8** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 0 | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧🚧🚧🚧 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/hostname.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **7** | **19** | **7** | **6** | **0** | **0** | **0** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 4 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧🚧✅✅✅🙈✅ |
| allOf.json | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅🙈✅✅✅✅✅✅✅✅🙈 |
| anyOf.json | 6 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅✅✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🦀✅✅✅ |
| contains.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| exclusiveMinimum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| format.json | 0 | 0 | 9 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🚧🚧🚧🚧🚧🚧🚧🚧 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🙈✅🙈🙈🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🤪🤪 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 3 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈✅🙈✅🙈🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 2 | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 6 | 🚧🤪🙈✅✅🚧 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 25 | 3 | 1 | 0 | 0 | 0 | 2 | 0 | 31 | 🚧✅🙈✅✅✅✅🙈✅✅✅✅✅✅✅✅✅✅❌❌🙈✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🤪 |
| type.json | 9 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅✅✅✅✅🚧 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **115** | **71** | **19** | **12** | **0** | **1** | **15** | **0** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 0 | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧🚧🚧🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/hostname.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/id.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **7** | **20** | **8** | **9** | **0** | **0** | **0** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 4 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧🚧✅✅✅🙈✅ |
| allOf.json | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅🙈✅✅✅✅✅✅✅✅🙈 |
| anyOf.json | 6 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅✅✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🦀✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| exclusiveMinimum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| format.json | 0 | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 17 | 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🙈✅🙈🙈🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🤪🤪 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 3 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈✅🙈✅🙈🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 2 | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 6 | 🚧🤪🙈✅✅🚧 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 29 | 3 | 1 | 0 | 0 | 0 | 2 | 0 | 35 | 🚧✅🙈✅✅✅✅🙈✅✅✅✅✅✅✅✅✅✅❌❌✅🙈✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🤪 |
| type.json | 9 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅✅✅✅✅🚧 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **123** | **80** | **27** | **12** | **0** | **1** | **15** | **0** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧🚧🚧🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/idn-hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/id.json | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **8** | **30** | **8** | **17** | **0** | **0** | **1** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 6 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🚧✅✅✅🙈✅✅✅ |
| allOf.json | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅🙈✅✅✅✅✅✅✅✅🙈 |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 6 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅✅✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🦀✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| enum.json | 15 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| exclusiveMinimum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| format.json | 0 | 0 | 19 | 0 | 0 | 0 | 0 | 0 | 19 | 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🙈✅🙈🙈🙈✅✅✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🤪🤪 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 3 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈✅🙈✅🙈🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 2 | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 6 | 🚧🤪🙈✅✅🚧 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🙈🙈🙈🙈 |
| ref.json | 26 | 8 | 1 | 0 | 0 | 0 | 2 | 0 | 37 | 🚧✅🙈✅✅🙈🙈✅✅✅✅✅✅🙈✅❌❌✅🙈🙈✅🙈✅✅✅✅✅✅✅✅✅✅✅✅✅✅🙈 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🤪 |
| type.json | 9 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅✅✅✅✅🚧 |
| unevaluatedItems.json | 7 | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 26 | ✅🙈🙈✅🙈✅🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| vocabulary.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| **Total** | **151** | **161** | **29** | **12** | **0** | **1** | **19** | **0** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/ecmascript-regex.json | 0 | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧🚧🚧🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/duration.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/idn-hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uuid.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **15** | **31** | **8** | **20** | **0** | **0** | **2** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 6 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 9 | 🚧🚧✅✅✅🙈✅✅✅ |
| allOf.json | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅🙈✅✅✅✅✅✅✅✅🙈 |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 6 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅✅✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🦀✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| dynamicRef.json | 5 | 11 | 0 | 0 | 0 | 0 | 5 | 0 | 21 | 🙈🙈✅🙈🙈🙈✅✅✅🙈✅🙈🙈❌❌❌❌❌🙈🙈🙈 |
| enum.json | 15 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅🚧✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| exclusiveMinimum.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| format.json | 0 | 0 | 19 | 0 | 0 | 0 | 0 | 0 | 19 | 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 3 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 10 | 🚧✅🙈🙈✅🙈🙈🚧🙈✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minLength.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🤪🤪 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 3 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈✅🙈✅🙈🙈🙈🙈✅ |
| pattern.json | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 3 | 🤪✅🙈 |
| patternProperties.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| prefixItems.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| properties.json | 2 | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 6 | 🚧🤪🙈✅✅🚧 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 26 | 7 | 1 | 0 | 0 | 0 | 2 | 0 | 36 | 🚧✅🙈✅✅🙈🙈✅✅✅✅✅✅🙈✅❌❌✅🙈🙈✅🙈✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🤪 |
| type.json | 9 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅✅✅✅✅🚧 |
| unevaluatedItems.json | 8 | 21 | 0 | 0 | 0 | 0 | 0 | 0 | 29 | ✅🙈🙈✅🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| vocabulary.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| **Total** | **153** | **164** | **30** | **12** | **0** | **1** | **24** | **0** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 0 | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧🚧🚧🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/duration.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/idn-hostname.json | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 🤪🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/unknown.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/format/uuid.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🤪 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **15** | **34** | **8** | **20** | **0** | **0** | **1** | **0** | **78** |  |

## Failures

### ❌ `generation_error` (85)

64 distinct first-line errors; most common:

- **4×** `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`
- **4×** `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- **4×** `the document is the boolean schema 'false', not a schema object`
- **4×** `the document is the boolean schema 'true', not a schema object`
- **3×** `schema at index 0: $ref "folderInteger.json" at /definitions/baz/definitions/bar/items ("http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json…`

Per group:

- `draft4/refRemote.json::remote ref` — `schema at index 0: $ref "http://localhost:1234/integer.json" at root ("http://localhost:1234/integer.json") does not resolve: could not retrieve "http://localho…`
- `draft4/refRemote.json::fragment within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft4/subSchemas.json#/definitions/integer" at root ("http://localhost:1234/draft4/subSchemas.json#/definitions/…`
- `draft4/refRemote.json::ref within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft4/subSchemas.json#/definitions/refToInteger" at root ("http://localhost:1234/draft4/subSchemas.json#/definit…`
- `draft4/refRemote.json::base URI change` — `schema at index 0: $ref "folderInteger.json" at /items/items ("http://localhost:1234/baseUriChange/folderInteger.json") does not resolve: could not retrieve "ht…`
- `draft4/refRemote.json::base URI change - change folder` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/items ("http://localhost:1234/baseUriChangeFolder/folderInteger.json") does not resolve: could …`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/definitions/bar/items ("http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json…`
- `draft4/refRemote.json::root ref in remote ref` — `schema at index 0: $ref "draft4/name.json#/definitions/orNull" at /properties/name ("http://localhost:1234/draft4/name.json#/definitions/orNull") does not resol…`
- `draft4/refRemote.json::Location-independent identifier in remote ref` — `schema at index 0: $ref "http://localhost:1234/draft4/locationIndependentIdentifier.json#/definitions/refToInteger" at root ("http://localhost:1234/draft4/locat…`
- `draft6/boolean_schema.json::boolean schema 'true'` — `the document is the boolean schema 'true', not a schema object`
- `draft6/boolean_schema.json::boolean schema 'false'` — `the document is the boolean schema 'false', not a schema object`
- `draft6/ref.json::refs with relative uris and defs` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `draft6/ref.json::relative refs with absolute uris and defs` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `draft6/refRemote.json::remote ref` — `schema at index 0: $ref "http://localhost:1234/integer.json" at root ("http://localhost:1234/integer.json") does not resolve: could not retrieve "http://localho…`
- `draft6/refRemote.json::fragment within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft6/subSchemas.json#/definitions/integer" at root ("http://localhost:1234/draft6/subSchemas.json#/definitions/…`
- `draft6/refRemote.json::ref within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft6/subSchemas.json#/definitions/refToInteger" at root ("http://localhost:1234/draft6/subSchemas.json#/definit…`
- `draft6/refRemote.json::base URI change` — `schema at index 0: $ref "folderInteger.json" at /items/items ("http://localhost:1234/baseUriChange/folderInteger.json") does not resolve: could not retrieve "ht…`
- `draft6/refRemote.json::base URI change - change folder` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/items ("http://localhost:1234/baseUriChangeFolder/folderInteger.json") does not resolve: could …`
- `draft6/refRemote.json::base URI change - change folder in subschema` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/definitions/bar/items ("http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json…`
- `draft6/refRemote.json::root ref in remote ref` — `schema at index 0: $ref "draft6/name.json#/definitions/orNull" at /properties/name ("http://localhost:1234/draft6/name.json#/definitions/orNull") does not resol…`
- `draft6/refRemote.json::remote ref with ref to definitions` — `schema at index 0: $ref "draft6/ref-and-definitions.json" at /allOf/0 ("http://localhost:1234/draft6/ref-and-definitions.json") does not resolve: could not retr…`
- `draft6/refRemote.json::Location-independent identifier in remote ref` — `schema at index 0: $ref "http://localhost:1234/draft6/locationIndependentIdentifier.json#/definitions/refToInteger" at root ("http://localhost:1234/draft6/locat…`
- `draft6/refRemote.json::retrieved nested refs resolve relative to their URI not $id` — `schema at index 0: $ref "nested/foo-ref-string.json" at /properties/name ("http://localhost:1234/nested/foo-ref-string.json") does not resolve: could not retrie…`
- `draft6/refRemote.json::$ref to $ref finds location-independent $id` — `schema at index 0: $ref "http://localhost:1234/draft6/detached-ref.json#/definitions/foo" at root ("http://localhost:1234/draft6/detached-ref.json#/definitions/…`
- `draft7/boolean_schema.json::boolean schema 'true'` — `the document is the boolean schema 'true', not a schema object`
- `draft7/boolean_schema.json::boolean schema 'false'` — `the document is the boolean schema 'false', not a schema object`
- `draft7/ref.json::refs with relative uris and defs` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `draft7/ref.json::relative refs with absolute uris and defs` — `schema at index 0: could not resolve $ref "#/definitions/inner": $ref "#/definitions/inner" is not in this schema's registry (the schema is not part of it)`
- `draft7/refRemote.json::remote ref` — `schema at index 0: $ref "http://localhost:1234/integer.json" at root ("http://localhost:1234/integer.json") does not resolve: could not retrieve "http://localho…`
- `draft7/refRemote.json::fragment within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft7/subSchemas.json#/definitions/integer" at root ("http://localhost:1234/draft7/subSchemas.json#/definitions/…`
- `draft7/refRemote.json::ref within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft7/subSchemas.json#/definitions/refToInteger" at root ("http://localhost:1234/draft7/subSchemas.json#/definit…`
- `draft7/refRemote.json::base URI change` — `schema at index 0: $ref "folderInteger.json" at /items/items ("http://localhost:1234/baseUriChange/folderInteger.json") does not resolve: could not retrieve "ht…`
- `draft7/refRemote.json::base URI change - change folder` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/items ("http://localhost:1234/baseUriChangeFolder/folderInteger.json") does not resolve: could …`
- `draft7/refRemote.json::base URI change - change folder in subschema` — `schema at index 0: $ref "folderInteger.json" at /definitions/baz/definitions/bar/items ("http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json…`
- `draft7/refRemote.json::root ref in remote ref` — `schema at index 0: $ref "draft7/name.json#/definitions/orNull" at /properties/name ("http://localhost:1234/draft7/name.json#/definitions/orNull") does not resol…`
- `draft7/refRemote.json::remote ref with ref to definitions` — `schema at index 0: $ref "draft7/ref-and-definitions.json" at /allOf/0 ("http://localhost:1234/draft7/ref-and-definitions.json") does not resolve: could not retr…`
- `draft7/refRemote.json::Location-independent identifier in remote ref` — `schema at index 0: $ref "http://localhost:1234/draft7/locationIndependentIdentifier.json#/definitions/refToInteger" at root ("http://localhost:1234/draft7/locat…`
- `draft7/refRemote.json::retrieved nested refs resolve relative to their URI not $id` — `schema at index 0: $ref "nested/foo-ref-string.json" at /properties/name ("http://localhost:1234/nested/foo-ref-string.json") does not resolve: could not retrie…`
- `draft7/refRemote.json::$ref to $ref finds location-independent $id` — `schema at index 0: $ref "http://localhost:1234/draft7/detached-ref.json#/definitions/foo" at root ("http://localhost:1234/draft7/detached-ref.json#/definitions/…`
- `draft2019-09/boolean_schema.json::boolean schema 'true'` — `the document is the boolean schema 'true', not a schema object`
- `draft2019-09/boolean_schema.json::boolean schema 'false'` — `the document is the boolean schema 'false', not a schema object`
- `draft2019-09/ref.json::refs with relative uris and defs` — `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`
- `draft2019-09/ref.json::relative refs with absolute uris and defs` — `schema at index 0: could not resolve $ref "#/$defs/inner": $ref "#/$defs/inner" is not in this schema's registry (the schema is not part of it)`
- `draft2019-09/refRemote.json::remote ref` — `schema at index 0: $ref "http://localhost:1234/draft2019-09/integer.json" at root ("http://localhost:1234/draft2019-09/integer.json") does not resolve: could no…`
- `draft2019-09/refRemote.json::fragment within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft2019-09/subSchemas.json#/$defs/integer" at root ("http://localhost:1234/draft2019-09/subSchemas.json#/$defs/…`
- `draft2019-09/refRemote.json::anchor within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft2019-09/locationIndependentIdentifier.json#foo" at root ("http://localhost:1234/draft2019-09/locationIndepen…`
- `draft2019-09/refRemote.json::ref within remote ref` — `schema at index 0: $ref "http://localhost:1234/draft2019-09/subSchemas.json#/$defs/refToInteger" at root ("http://localhost:1234/draft2019-09/subSchemas.json#/$…`
- `draft2019-09/refRemote.json::base URI change` — `schema at index 0: $ref "folderInteger.json" at /items/items ("http://localhost:1234/draft2019-09/baseUriChange/folderInteger.json") does not resolve: could not…`
- `draft2019-09/refRemote.json::base URI change - change folder` — `schema at index 0: $ref "folderInteger.json" at /$defs/baz/items ("http://localhost:1234/draft2019-09/baseUriChangeFolder/folderInteger.json") does not resolve:…`
- `draft2019-09/refRemote.json::base URI change - change folder in subschema` — `schema at index 0: $ref "folderInteger.json" at /$defs/baz/$defs/bar/items ("http://localhost:1234/draft2019-09/baseUriChangeFolderInSubschema/folderInteger.jso…`
- `draft2019-09/refRemote.json::root ref in remote ref` — `schema at index 0: $ref "name-defs.json#/$defs/orNull" at /properties/name ("http://localhost:1234/draft2019-09/name-defs.json#/$defs/orNull") does not resolve:…`
- … and 35 more

### 🦀 `compile_fail` (4)

- `draft6/const.json::float and integers are equal up to 64-bit representation limits` — `error: literal out of range for 'i32'`
- `draft7/const.json::float and integers are equal up to 64-bit representation limits` — `error: literal out of range for 'i32'`
- `draft2019-09/const.json::float and integers are equal up to 64-bit representation limits` — `error: literal out of range for 'i32'`
- `draft2020-12/const.json::float and integers are equal up to 64-bit representation limits` — `error: literal out of range for 'i32'`

### 🤪 `mixed` (132)

11 distinct first-line errors; most common:

- **70×** `all string formats ignore integers: invalid type: integer '12', expected a string`
- **10×** `accepted 'too long is invalid' though valid: false`
- **10×** `accepted 'too short is invalid' though valid: false`
- **8×** `accepted 'boundary point is invalid' though valid: false`
- **6×** `accepted 'above the maximum is invalid' though valid: false`

Per group:

- `draft4/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft4/maxLength.json::maxLength validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-strings (rejected)
- `draft4/maximum.json::maximum validation` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted), ignores non-numbers (rejected)
- `draft4/maximum.json::maximum validation (explicit false exclusivity)` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted), ignores non-numbers (rejected)
- `draft4/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft4/minLength.json::minLength validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft4/minimum.json::minimum validation` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft4/minimum.json::minimum validation (explicit false exclusivity)` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft4/minimum.json::minimum validation with signed integer` — `accepted 'float below the minimum is invalid' though valid: false`
  - cases: float below the minimum is invalid (accepted), int below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft4/pattern.json::pattern validation` — `accepted 'a non-matching pattern is invalid' though valid: false`
  - cases: a non-matching pattern is invalid (accepted), ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), … and 1 more
- `draft4/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `draft4/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft6/exclusiveMaximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), above the exclusiveMaximum is invalid (accepted), ignores non-numbers (rejected)
- `draft6/exclusiveMinimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), below the exclusiveMinimum is invalid (accepted), ignores non-numbers (rejected)
- `draft6/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft6/maxLength.json::maxLength validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-strings (rejected)
- `draft6/maximum.json::maximum validation` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted), ignores non-numbers (rejected)
- `draft6/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft6/minLength.json::minLength validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft6/minimum.json::minimum validation` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft6/minimum.json::minimum validation with signed integer` — `accepted 'float below the minimum is invalid' though valid: false`
  - cases: float below the minimum is invalid (accepted), int below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft6/pattern.json::pattern validation` — `accepted 'a non-matching pattern is invalid' though valid: false`
  - cases: a non-matching pattern is invalid (accepted), ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), … and 1 more
- `draft6/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `draft6/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft7/exclusiveMaximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), above the exclusiveMaximum is invalid (accepted), ignores non-numbers (rejected)
- `draft7/exclusiveMinimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), below the exclusiveMinimum is invalid (accepted), ignores non-numbers (rejected)
- `draft7/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft7/maxLength.json::maxLength validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-strings (rejected)
- `draft7/maximum.json::maximum validation` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted), ignores non-numbers (rejected)
- `draft7/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft7/minLength.json::minLength validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft7/minimum.json::minimum validation` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft7/minimum.json::minimum validation with signed integer` — `accepted 'float below the minimum is invalid' though valid: false`
  - cases: float below the minimum is invalid (accepted), int below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft7/pattern.json::pattern validation` — `accepted 'a non-matching pattern is invalid' though valid: false`
  - cases: a non-matching pattern is invalid (accepted), ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), … and 1 more
- `draft7/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `draft7/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft2019-09/exclusiveMaximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), above the exclusiveMaximum is invalid (accepted), ignores non-numbers (rejected)
- `draft2019-09/exclusiveMinimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), below the exclusiveMinimum is invalid (accepted), ignores non-numbers (rejected)
- `draft2019-09/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft2019-09/maxLength.json::maxLength validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-strings (rejected)
- `draft2019-09/maximum.json::maximum validation` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted), ignores non-numbers (rejected)
- `draft2019-09/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft2019-09/minLength.json::minLength validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-strings (rejected), one grapheme is not long enough (accepted)
- `draft2019-09/minimum.json::minimum validation` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft2019-09/minimum.json::minimum validation with signed integer` — `accepted 'float below the minimum is invalid' though valid: false`
  - cases: float below the minimum is invalid (accepted), int below the minimum is invalid (accepted), ignores non-numbers (rejected)
- `draft2019-09/pattern.json::pattern validation` — `accepted 'a non-matching pattern is invalid' though valid: false`
  - cases: a non-matching pattern is invalid (accepted), ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), … and 1 more
- `draft2019-09/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)
- `draft2019-09/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft2020-12/exclusiveMaximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), above the exclusiveMaximum is invalid (accepted), ignores non-numbers (rejected)
- `draft2020-12/exclusiveMinimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted), below the exclusiveMinimum is invalid (accepted), ignores non-numbers (rejected)
- … and 82 more

### 🚧 `false_reject` (159)

16 distinct first-line errors; most common:

- **70×** `all string formats ignore integers: invalid type: integer '12', expected a string`
- **15×** `ignores arrays: invalid type: sequence, expected struct Root`
- **10×** `ascii character in json string: unknown field 'l'ecole', there are no fields`
- **10×** `ascii digits: unknown field '42', there are no fields`
- **5×** `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`

Per group:

- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `matching the pattern is valid: unknown field 'ármányos', there are no fields`
  - cases: matching the pattern is valid (rejected)
- `draft4/enum.json::heterogeneous enum-with-null validation` — `null is valid: null is not a Root value`
  - cases: null is valid (rejected)
- `draft4/format.json::email format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/format.json::ipv4 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/format.json::ipv6 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/format.json::hostname format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/format.json::date-time format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/format.json::uri format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft4/items.json::a schema given for items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `draft4/properties.json::object properties validation` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft4/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft4/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected)
- `draft4/required.json::required validation` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft4/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft6/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft6/additionalProperties.json::non-ASCII pattern with additionalProperties` — `matching the pattern is valid: unknown field 'ármányos', there are no fields`
  - cases: matching the pattern is valid (rejected)
- `draft6/enum.json::heterogeneous enum-with-null validation` — `null is valid: null is not a Root value`
  - cases: null is valid (rejected)
- `draft6/format.json::email format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::ipv4 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::ipv6 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::hostname format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::date-time format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::json-pointer format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::uri format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::uri-reference format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/format.json::uri-template format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft6/items.json::a schema given for items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `draft6/properties.json::object properties validation` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft6/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft6/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected)
- `draft6/required.json::required validation` — `ignores arrays: invalid type: sequence, expected struct Root`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft6/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft7/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft7/additionalProperties.json::non-ASCII pattern with additionalProperties` — `matching the pattern is valid: unknown field 'ármányos', there are no fields`
  - cases: matching the pattern is valid (rejected)
- `draft7/enum.json::heterogeneous enum-with-null validation` — `null is valid: null is not a Root value`
  - cases: null is valid (rejected)
- `draft7/format.json::email format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::idn-email format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::regex format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::ipv4 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::ipv6 format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::idn-hostname format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::hostname format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::date format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::date-time format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::time format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::json-pointer format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::relative-json-pointer format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- `draft7/format.json::iri format` — `all string formats ignore integers: invalid type: integer '12', expected a string`
  - cases: all string formats ignore integers (rejected), all string formats ignore floats (rejected), all string formats ignore objects (rejected), all string formats ignore arrays (rejected), all string formats ignore booleans (rejected), all string formats ignore nulls (rejected)
- … and 109 more

### 🙈 `false_accept` (658)

150 distinct first-line errors; most common:

- **20×** `accepted 'with unevaluated properties' though valid: false`
- **18×** `accepted 'with unevaluated items' though valid: false`
- **17×** `accepted 'too long is invalid' though valid: false`
- **17×** `accepted 'too short is invalid' though valid: false`
- **15×** `accepted 'both oneOf valid' though valid: false`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `accepted 'items defined in allOf are not examined' though valid: false`
  - cases: items defined in allOf are not examined (accepted)
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `accepted 'wrong type of second item' though valid: false`
  - cases: wrong type of second item (accepted)
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft4/allOf.json::allOf simple types` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `draft4/anyOf.json::anyOf` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `draft4/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
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
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted)
- `draft4/items.json::items and subitems` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted)
- `draft4/maxProperties.json::maxProperties validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted)
- `draft4/maxProperties.json::maxProperties = 0 means the object is empty` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `draft4/maximum.json::maximum validation with unsigned integer` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted)
- `draft4/maximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted)
- `draft4/minProperties.json::minProperties validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted)
- `draft4/minimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted)
- `draft4/multipleOf.json::by int` — `accepted 'int by int fail' though valid: false`
  - cases: int by int fail (accepted)
- `draft4/multipleOf.json::by number` — `accepted '35 is not multiple of 1.5' though valid: false`
  - cases: 35 is not multiple of 1.5 (accepted)
- `draft4/multipleOf.json::by small number` — `accepted '0.00751 is not multiple of 0.0001' though valid: false`
  - cases: 0.00751 is not multiple of 0.0001 (accepted)
- `draft4/not.json::not` — `accepted 'disallowed' though valid: false`
  - cases: disallowed (accepted)
- `draft4/not.json::not multiple types` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted), other mismatch (accepted)
- `draft4/not.json::not more complex schema` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `draft4/not.json::forbidden property` — `accepted 'property present' though valid: false`
  - cases: property present (accepted)
- `draft4/not.json::forbid everything with empty schema` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), boolean true is invalid (accepted), boolean false is invalid (accepted), null is invalid (accepted), object is invalid (accepted), … and 3 more
- `draft4/oneOf.json::oneOf` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted), neither oneOf valid (accepted)
- `draft4/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/oneOf.json::oneOf complex types` — `accepted 'both oneOf valid (complex)' though valid: false`
  - cases: both oneOf valid (complex) (accepted)
- `draft4/oneOf.json::oneOf with empty schema` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with missing optional property` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/patternProperties.json::patternProperties validates properties matching a regex` — `accepted 'a single invalid match is invalid' though valid: false`
  - cases: a single invalid match is invalid (accepted), multiple invalid matches is invalid (accepted)
- `draft4/patternProperties.json::multiple simultaneous patternProperties are validated` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `draft4/patternProperties.json::regexes are not anchored by default and are case sensitive` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `draft4/ref.json::relative pointer ref to array` — `accepted 'mismatch array' though valid: false`
  - cases: mismatch array (accepted)
- `draft4/ref.json::remote ref, containing refs itself` — `accepted 'remote ref invalid' though valid: false`
  - cases: remote ref invalid (accepted)
- `draft4/required.json::required with escaped characters` — `accepted 'object with some properties missing is invalid' though valid: false`
  - cases: object with some properties missing is invalid (accepted)
- `draft4/uniqueItems.json::uniqueItems validation` — `accepted 'non-unique array of integers is invalid' though valid: false`
  - cases: non-unique array of integers is invalid (accepted), non-unique array of more than two integers is invalid (accepted), numbers are unique if mathematically unequal (accepted), non-unique array of strings is invalid (accepted), non-unique array of objects is invalid (accepted), property order of array of objects is ignored (accepted), … and 5 more
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), non-unique array extended from [false, true] is not valid (accepted), non-unique array extended from [true, false] is not valid (accepted)
- `draft4/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), extra items are invalid even if unique (accepted)
- `draft4/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `accepted 'extra items are invalid even if unique' though valid: false`
  - cases: extra items are invalid even if unique (accepted)
- `draft6/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft6/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- … and 608 more

