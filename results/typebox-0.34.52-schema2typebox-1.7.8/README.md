# typebox 0.34.52 + schema2typebox 1.7.8

TypeBox is a TypeScript type builder: one `Type.Object({...})` definition is
both a JSON Schema fragment and a static type, and `TypeCompiler` checks
instances against it. Its design center is the TypeScript end of the wire,
so its checker refuses schemas it didn't build. Going from JSON Schema to
TypeBox needs a separate generator; this row uses schema2typebox, the one
TypeBox's readme points at. Most of what it can't express becomes
`Type.Unknown()`.

_generated 2026-10-11T19:24:11Z_

**227 of 1408 non-optional groups pass (16.1%)**; 301 optional groups reported separately

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
| draft4 | 37 | 5 | 2 | 1 | 0 | 6 | 108 | 1 | 160 | 23.1% |
| draft6 | 46 | 5 | 4 | 2 | 0 | 20 | 155 | 1 | 233 | 19.7% |
| draft7 | 46 | 5 | 4 | 2 | 0 | 20 | 180 | 1 | 258 | 17.8% |
| draft2019-09 | 51 | 28 | 4 | 2 | 0 | 20 | 266 | 2 | 373 | 13.7% |
| draft2020-12 | 47 | 6 | 5 | 2 | 0 | 20 | 302 | 2 | 384 | 12.2% |
| **Total** | **227** | **49** | **19** | **9** | **0** | **86** | **1011** | **7** | **1408** | **16.1%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 14 | 6 | 0 | 0 | 0 | 0 | 19 | 0 | 39 | 35.9% |
| draft6 | 14 | 5 | 0 | 0 | 0 | 0 | 25 | 0 | 44 | 31.8% |
| draft7 | 14 | 5 | 0 | 0 | 0 | 0 | 45 | 0 | 64 | 21.9% |
| draft2019-09 | 14 | 5 | 0 | 0 | 0 | 0 | 57 | 0 | 76 | 18.4% |
| draft2020-12 | 14 | 5 | 0 | 0 | 0 | 0 | 59 | 0 | 78 | 17.9% |
| **Total** | **70** | **26** | **0** | **0** | **0** | **0** | **205** | **0** | **301** | **23.3%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 146 | 164 | 89.0% | 618 | 23.6% |
| draft6 | 184 | 206 | 89.3% | 841 | 21.9% |
| draft7 | 184 | 206 | 89.3% | 929 | 19.8% |
| draft2019-09 | 222 | 276 | 80.4% | 1261 | 17.6% |
| draft2020-12 | 193 | 221 | 87.3% | 1301 | 14.8% |
| **Total** | **929** | **1073** | **86.6%** | **4950** | **18.8%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 54 | 64 | 84.4% | 391 | 13.8% |
| draft6 | 54 | 63 | 85.7% | 520 | 10.4% |
| draft7 | 54 | 63 | 85.7% | 922 | 5.9% |
| draft2019-09 | 54 | 63 | 85.7% | 1043 | 5.2% |
| draft2020-12 | 54 | 63 | 85.7% | 1047 | 5.2% |
| **Total** | **270** | **316** | **85.4%** | **3923** | **6.9%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 9 | ❌❌❌✅✅✅✅✅❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅✅ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| enum.json | 10 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 16 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅✅ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 0 | 0 | 0 | 0 | 1 | 4 | 0 | 6 | ❌❌🦀✅❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅✅✅❌✅✅ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ❌❌❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| ref.json | 4 | 0 | 1 | 0 | 0 | 0 | 13 | 1 | 19 | 🤯❌❌❌✅❌❌❌❌❌❌❌❌❌🚧❌✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| type.json | 6 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 11 | 🙈✅✅🙈✅✅✅🤪✅🙈🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| **Total** | **37** | **5** | **2** | **1** | **0** | **6** | **108** | **1** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ✅✅✅❌❌❌❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/zeroTerminatedFloats.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **14** | **6** | **0** | **0** | **0** | **0** | **19** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 3 | 4 | 0 | 12 | ❌❌❌🦀🦀🦀✅✅✅✅✅❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 8 | ❌❌🦀🦀🦀❌✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 10 | 0 | 3 | 0 | 0 | 0 | 4 | 0 | 17 | ✅❌🚧❌✅✅🚧🚧❌❌✅✅✅✅✅✅✅ |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| enum.json | 9 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 15 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 0 | 0 | 0 | 0 | 1 | 7 | 0 | 9 | ❌❌❌❌❌🦀✅❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 8 | ✅✅✅❌✅🦀🦀✅ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 4 | 4 | 0 | 11 | ❌❌🦀🦀🦀🦀❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| ref.json | 4 | 0 | 0 | 1 | 0 | 2 | 23 | 1 | 31 | 🤯❌❌❌✅❌❌❌❌❌🦀🦀❌❌❌❌❌🤪❌❌❌❌❌❌❌❌❌❌✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 6 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 11 | 🙈✅✅🙈✅✅✅🤪✅🙈🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| **Total** | **46** | **5** | **4** | **2** | **0** | **20** | **155** | **1** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ✅✅✅❌❌❌❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **14** | **5** | **0** | **0** | **0** | **0** | **25** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 3 | 4 | 0 | 12 | ❌❌❌🦀🦀🦀✅✅✅✅✅❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 8 | ❌❌🦀🦀🦀❌✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 10 | 0 | 3 | 0 | 0 | 0 | 4 | 0 | 17 | ✅❌🚧❌✅✅🚧🚧❌❌✅✅✅✅✅✅✅ |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| enum.json | 9 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 15 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 17 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 0 | 0 | 0 | 0 | 1 | 7 | 0 | 9 | ❌❌❌❌❌🦀✅❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 8 | ✅✅✅❌✅🦀🦀✅ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 4 | 4 | 0 | 11 | ❌❌🦀🦀🦀🦀❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| ref.json | 4 | 0 | 0 | 1 | 0 | 2 | 27 | 1 | 35 | 🤯❌❌❌✅❌❌❌❌❌🦀🦀❌❌❌❌❌🤪❌❌❌❌❌❌❌❌❌❌❌❌❌❌✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 6 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 11 | 🙈✅✅🙈✅✅✅🤪✅🙈🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| **Total** | **46** | **5** | **4** | **2** | **0** | **20** | **180** | **1** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ✅✅✅❌❌❌❌ |
| optional/content.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **14** | **5** | **0** | **0** | **0** | **0** | **45** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 10 | ❌❌❌❌❌❌❌❌❌❌ |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 3 | 4 | 0 | 12 | ❌❌❌🦀🦀🦀✅✅✅✅✅❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 8 | ❌❌🦀🦀🦀❌✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 10 | 0 | 3 | 0 | 0 | 0 | 4 | 0 | 17 | ✅❌🚧❌✅✅🚧🚧❌❌✅✅✅✅✅✅✅ |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| enum.json | 10 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 16 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 19 | 0 | 19 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 0 | 0 | 0 | 0 | 1 | 7 | 0 | 9 | ❌❌❌❌❌🦀✅❌❌ |
| maxContains.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minContains.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 9 | ✅✅✅❌✅🦀🦀✅❌ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 4 | 4 | 0 | 11 | ❌❌🦀🦀🦀🦀❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| recursiveRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| ref.json | 1 | 0 | 0 | 1 | 0 | 0 | 34 | 1 | 37 | 🤯❌❌❌❌❌❌❌❌❌❌❌❌❌🤪❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌✅❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 6 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 11 | 🙈✅✅🙈✅✅✅🤪✅🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 25 | 0 | 26 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌🦀❌❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 6 | 23 | 0 | 0 | 0 | 1 | 13 | 1 | 44 | ✅🙈🙈🙈🙈🦀✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌✅✅🙈🙈🙈🙈🙈🙈🙈🤯❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **51** | **28** | **4** | **2** | **0** | **20** | **266** | **2** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ✅✅✅❌❌❌❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **14** | **5** | **0** | **0** | **0** | **0** | **57** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 9 | ❌❌❌❌❌❌❌❌❌ |
| allOf.json | 5 | 0 | 0 | 0 | 0 | 3 | 4 | 0 | 12 | ❌❌❌🦀🦀🦀✅✅✅✅✅❌ |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 3 | 0 | 8 | ❌❌🦀🦀🦀❌✅✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 10 | 0 | 3 | 0 | 0 | 0 | 4 | 0 | 17 | ✅❌🚧❌✅✅🚧🚧❌❌✅✅✅✅✅✅✅ |
| contains.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| content.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| default.json | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌✅ |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 0 | 21 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| enum.json | 10 | 0 | 0 | 0 | 0 | 5 | 1 | 0 | 16 | ✅🦀✅✅❌✅🦀✅🦀✅🦀✅🦀✅✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 0 | 0 | 0 | 0 | 0 | 0 | 19 | 0 | 19 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 12 | ❌❌❌❌❌❌❌❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| items.json | 1 | 1 | 0 | 0 | 0 | 0 | 8 | 0 | 10 | ❌❌❌🙈✅❌❌❌❌❌ |
| maxContains.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minContains.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| multipleOf.json | 1 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 5 | ❌❌❌✅🚧 |
| not.json | 5 | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 9 | ✅✅✅❌✅🦀🦀✅❌ |
| oneOf.json | 2 | 1 | 0 | 0 | 0 | 4 | 4 | 0 | 11 | ❌❌🦀🦀🦀🦀❌✅🙈❌✅ |
| pattern.json | 0 | 0 | 1 | 0 | 0 | 0 | 2 | 0 | 3 | ❌❌🚧 |
| patternProperties.json | 1 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 6 | ❌❌❌❌❌✅ |
| prefixItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| properties.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| ref.json | 1 | 0 | 0 | 1 | 0 | 0 | 33 | 1 | 36 | 🤯❌❌❌❌❌❌❌❌❌❌❌❌❌🤪❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| type.json | 6 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 11 | 🙈✅✅🙈✅✅✅🤪✅🙈🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 28 | 0 | 29 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌🦀❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| unevaluatedProperties.json | 1 | 0 | 0 | 0 | 0 | 2 | 40 | 1 | 44 | ❌❌❌❌❌🦀✅❌❌❌❌❌❌❌❌❌❌❌🦀❌❌❌❌❌❌❌❌❌❌❌❌❌❌🤯❌❌❌❌❌❌❌❌❌❌ |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **47** | **6** | **5** | **2** | **0** | **20** | **302** | **2** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 7 | ✅✅✅❌❌❌❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅❌❌❌❌❌🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 5 | ❌❌❌❌❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **14** | **5** | **0** | **0** | **0** | **0** | **59** | **0** | **78** |  |

## Failures

### 🤯 `generation_panic` (7)

2 distinct first-line errors; most common:

- **6×** `TypeError: Converting circular structure to JSON`
- **1×** `RangeError: Maximum call stack size exceeded`

Per group:

- `draft4/ref.json::root pointer ref` — `TypeError: Converting circular structure to JSON`
- `draft6/ref.json::root pointer ref` — `TypeError: Converting circular structure to JSON`
- `draft7/ref.json::root pointer ref` — `TypeError: Converting circular structure to JSON`
- `draft2019-09/ref.json::root pointer ref` — `TypeError: Converting circular structure to JSON`
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties + single cyclic ref` — `RangeError: Maximum call stack size exceeded`
- `draft2020-12/ref.json::root pointer ref` — `TypeError: Converting circular structure to JSON`
- `draft2020-12/unevaluatedProperties.json::unevaluatedProperties + single cyclic ref` — `TypeError: Converting circular structure to JSON`

### ❌ `generation_error` (1216)

578 distinct first-line errors; most common:

- **16×** `SyntaxError: Argument expression expected. (6:28)`
- **15×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"type":"integer"}},"required":["bar"]}`
- **15×** `ResolverError: Error reading "<repo>/generators/typebox/folderInteger.json": the bakeoff forbids filesystem access`
- **11×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$schema":"https://json-schema.org/draft/2019-09/schema","contains":{"const":1},"…`
- **11×** `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"$schema":"https://json-schema.org/draft/2020-12/schema","contains":{"const":1},"…`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":{"type":"integer"},"items":[{}],"$id":"T"}`
- `draft4/additionalItems.json::when items is schema, additionalItems does nothing` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":false,"items":{},"$id":"T"}`
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":false,"items":[{},{},{}],"$id":"T"}`
- `draft4/additionalItems.json::additionalItems as false without items` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":false,"$id":"T"}`
- `draft4/additionalItems.json::additionalItems are allowed by default` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":[{"type":"integer"}],"$id":"T"}`
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":[{"type":"integer"},{"type":"string"}]}`
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":{"type":"integer"},"items":[{"type":"string"}],"$id":"T"}`
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":false,"items":[{}],"$id":"T"}`
- `draft4/additionalItems.json::additionalItems with null instance elements` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalItems":{"type":"null"},"$id":"T"}`
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"patternProperties":{"^v":{}},"properties":{"bar":{}…`
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":false,"patternProperties":{"^á":{}},"$id":"T"}`
- `draft4/additionalProperties.json::additionalProperties with schema` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"boolean"},"properties":{"bar":{},"foo":{}},"$id":…`
- `draft4/additionalProperties.json::additionalProperties can exist by itself` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"boolean"},"$id":"T"}`
- `draft4/additionalProperties.json::additionalProperties are allowed by default` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{},"foo":{}},"$id":"T"}`
- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{}}}`
- `draft4/additionalProperties.json::additionalProperties with null valued instance properties` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"additionalProperties":{"type":"null"},"$id":"T"}`
- `draft4/allOf.json::allOf` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"type":"integer"}},"required":["bar"]}`
- `draft4/allOf.json::allOf with base schema` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{"type":"string"}},"required":["foo"]}`
- `draft4/allOf.json::allOf simple types` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maximum":30}`
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"multipleOf":3}`
- `draft4/anyOf.json::anyOf` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"minimum":2}`
- `draft4/anyOf.json::anyOf with base schema` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxLength":2}`
- `draft4/anyOf.json::anyOf complex types` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"type":"integer"}},"required":["bar"]}`
- `draft4/default.json::invalid type for default` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{"default":[],"type":"integer"}},"$id":"T"}`
- `draft4/default.json::invalid string value for default` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"bar":{"default":"bad","minLength":4,"type":"string"}},"$id":"T"}`
- `draft4/definitions.json::validate definition against metaschema` — `ResolverError: Error downloading http://json-schema.org/draft-04/schema`
- `draft4/dependencies.json::dependencies` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":["foo"]},"$id":"T"}`
- `draft4/dependencies.json::multiple dependencies` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"quux":["foo","bar"]},"$id":"T"}`
- `draft4/dependencies.json::multiple dependencies subschema` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"bar":{"properties":{"bar":{"type":"integer"},"foo":{"type":"inte…`
- `draft4/dependencies.json::dependencies with escaped characters` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"foo\tbar":{"minProperties":4},"foo\nbar":["foo\rbar"],"foo\"bar"…`
- `draft4/dependencies.json::dependent subschema incompatible with root` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"dependencies":{"foo":{"additionalProperties":false,"properties":{"bar":{}}}},"pr…`
- `draft4/enum.json::enum with escaped characters` — `SyntaxError: Unterminated string literal. (6:48)`
- `draft4/format.json::email format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"email","$id":"T"}`
- `draft4/format.json::ipv4 format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"ipv4","$id":"T"}`
- `draft4/format.json::ipv6 format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"ipv6","$id":"T"}`
- `draft4/format.json::hostname format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"hostname","$id":"T"}`
- `draft4/format.json::date-time format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"date-time","$id":"T"}`
- `draft4/format.json::uri format` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"format":"uri","$id":"T"}`
- `draft4/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"properties":{"foo":{"type":"integer"}}}`
- `draft4/items.json::a schema given for items` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":{"type":"integer"},"$id":"T"}`
- `draft4/items.json::an array of schemas for items` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":[{"type":"integer"},{"type":"string"}],"$id":"T"}`
- `draft4/items.json::items with null instance elements` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":{"type":"null"},"$id":"T"}`
- `draft4/items.json::array-form items with null instance elements` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"items":[{"type":"null"}],"$id":"T"}`
- `draft4/maxItems.json::maxItems validation` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxItems":2,"$id":"T"}`
- `draft4/maxLength.json::maxLength validation` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxLength":2,"$id":"T"}`
- `draft4/maxProperties.json::maxProperties validation` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxProperties":2,"$id":"T"}`
- `draft4/maxProperties.json::maxProperties = 0 means the object is empty` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maxProperties":0,"$id":"T"}`
- `draft4/maximum.json::maximum validation` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maximum":3,"$id":"T"}`
- `draft4/maximum.json::maximum validation with unsigned integer` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"maximum":300,"$id":"T"}`
- `draft4/maximum.json::maximum validation (explicit false exclusivity)` — `Error: Unsupported schema. Did not match any type of the parsers. Schema was: {"exclusiveMaximum":false,"maximum":3,"$id":"T"}`
- … and 1166 more

### 🦀 `compile_fail` (86)

86 distinct first-line errors; most common:

- **1×** `g_0238786a92eb9a3c.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_040df7d07bd4d47c.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_0980e8612cd968c1.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_0a5a00d5149b213e.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- **1×** `g_10934e93eec83f99.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`

Per group:

- `draft4/enum.json::heterogeneous enum validation` — `g_f9bd00e5a57fe9ee.ts(16,16): error TS2345: Argument of type 'undefined[]' is not assignable to parameter of type 'TSchema'.`
- `draft4/enum.json::enum with [false] does not match [0]` — `g_7254ff1844c8c82e.ts(12,41): error TS2345: Argument of type 'TLiteral<false>[]' is not assignable to parameter of type 'TSchema'.`
- `draft4/enum.json::enum with [true] does not match [1]` — `g_a39b18e9d325c6f4.ts(12,41): error TS2345: Argument of type 'TLiteral<true>[]' is not assignable to parameter of type 'TSchema'.`
- `draft4/enum.json::enum with [0] does not match [false]` — `g_63cdfef5ef6eec35.ts(12,41): error TS2345: Argument of type 'TLiteral<0>[]' is not assignable to parameter of type 'TSchema'.`
- `draft4/enum.json::enum with [1] does not match [true]` — `g_1e8126dde3e21d24.ts(12,41): error TS2345: Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'.`
- `draft4/items.json::items and subitems` — `g_fe888af542a6ecec.ts(14,27): error TS2345: Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'.`
- `draft6/allOf.json::allOf with boolean schemas, all true` — `g_0980e8612cd968c1.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/allOf.json::allOf with boolean schemas, some false` — `g_0a5a00d5149b213e.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/allOf.json::allOf with boolean schemas, all false` — `g_31c6b4ee8e8134b0.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/anyOf.json::anyOf with boolean schemas, all true` — `g_9a18ea4a9a4fec7d.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/anyOf.json::anyOf with boolean schemas, some true` — `g_bc9e86c999d17649.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/anyOf.json::anyOf with boolean schemas, all false` — `g_bc0c8cad1d253bb2.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/enum.json::heterogeneous enum validation` — `g_294dc5a72f05e2c1.ts(16,16): error TS2345: Argument of type 'undefined[]' is not assignable to parameter of type 'TSchema'.`
- `draft6/enum.json::enum with [false] does not match [0]` — `g_1863a723be41abfc.ts(12,41): error TS2345: Argument of type 'TLiteral<false>[]' is not assignable to parameter of type 'TSchema'.`
- `draft6/enum.json::enum with [true] does not match [1]` — `g_21e2f38fb4adc6c8.ts(12,41): error TS2345: Argument of type 'TLiteral<true>[]' is not assignable to parameter of type 'TSchema'.`
- `draft6/enum.json::enum with [0] does not match [false]` — `g_a92de4dcbb4abd66.ts(12,41): error TS2345: Argument of type 'TLiteral<0>[]' is not assignable to parameter of type 'TSchema'.`
- `draft6/enum.json::enum with [1] does not match [true]` — `g_af8d8c3f74375665.ts(12,41): error TS2345: Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'.`
- `draft6/items.json::items and subitems` — `g_2fdbef2316e58bcb.ts(14,27): error TS2345: Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'.`
- `draft6/not.json::forbid everything with boolean schema true` — `g_a7b6d510524f474f.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `draft6/not.json::allow everything with boolean schema false` — `g_48bc8fd667f0ef20.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `draft6/oneOf.json::oneOf with boolean schemas, all true` — `g_feee66de8c8d006f.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/oneOf.json::oneOf with boolean schemas, one true` — `g_a62d6569180b31cf.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/oneOf.json::oneOf with boolean schemas, more than one true` — `g_c4c39ee69d78fa38.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/oneOf.json::oneOf with boolean schemas, all false` — `g_c38e79ea9ab1d601.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/ref.json::$ref to boolean schema true` — `g_040df7d07bd4d47c.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft6/ref.json::$ref to boolean schema false` — `g_729deea7c9c8bcfe.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/allOf.json::allOf with boolean schemas, all true` — `g_4d2b842651cec73b.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/allOf.json::allOf with boolean schemas, some false` — `g_e1cf9e9e5fcd75bb.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/allOf.json::allOf with boolean schemas, all false` — `g_5e129a4c07ad62e1.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/anyOf.json::anyOf with boolean schemas, all true` — `g_70c90d523e72b5f1.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/anyOf.json::anyOf with boolean schemas, some true` — `g_df556ba85bca4347.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/anyOf.json::anyOf with boolean schemas, all false` — `g_10934e93eec83f99.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/enum.json::heterogeneous enum validation` — `g_421353549796d147.ts(16,16): error TS2345: Argument of type 'undefined[]' is not assignable to parameter of type 'TSchema'.`
- `draft7/enum.json::enum with [false] does not match [0]` — `g_d8fa9d6bff8d32fb.ts(12,41): error TS2345: Argument of type 'TLiteral<false>[]' is not assignable to parameter of type 'TSchema'.`
- `draft7/enum.json::enum with [true] does not match [1]` — `g_771e137b471ef1e6.ts(12,41): error TS2345: Argument of type 'TLiteral<true>[]' is not assignable to parameter of type 'TSchema'.`
- `draft7/enum.json::enum with [0] does not match [false]` — `g_fe88feb5ba59e55e.ts(12,41): error TS2345: Argument of type 'TLiteral<0>[]' is not assignable to parameter of type 'TSchema'.`
- `draft7/enum.json::enum with [1] does not match [true]` — `g_3afbab8288bfbbb5.ts(12,41): error TS2345: Argument of type 'TLiteral<1>[]' is not assignable to parameter of type 'TSchema'.`
- `draft7/items.json::items and subitems` — `g_37ae667508124070.ts(14,27): error TS2345: Argument of type 'TUnknown' is not assignable to parameter of type 'TSchema[]'.`
- `draft7/not.json::forbid everything with boolean schema true` — `g_8379ef4a305dfe10.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `draft7/not.json::allow everything with boolean schema false` — `g_476181bf908bd652.ts(12,27): error TS2345: Argument of type 'boolean' is not assignable to parameter of type 'TSchema'.`
- `draft7/oneOf.json::oneOf with boolean schemas, all true` — `g_a7e01e5518714daa.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/oneOf.json::oneOf with boolean schemas, one true` — `g_4ee6ca05ba1d56bd.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/oneOf.json::oneOf with boolean schemas, more than one true` — `g_4473e78b88472e4d.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/oneOf.json::oneOf with boolean schemas, all false` — `g_741d21abcd0274f1.ts(41,25): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/ref.json::$ref to boolean schema true` — `g_b6af86cd07fe92db.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft7/ref.json::$ref to boolean schema false` — `g_e588d8724c6c5c5f.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft2019-09/allOf.json::allOf with boolean schemas, all true` — `g_0238786a92eb9a3c.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft2019-09/allOf.json::allOf with boolean schemas, some false` — `g_de5e7c39420ce1a1.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft2019-09/allOf.json::allOf with boolean schemas, all false` — `g_72935531ff8d4879.ts(12,34): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- `draft2019-09/anyOf.json::anyOf with boolean schemas, all true` — `g_6ee759c10bca55eb.ts(12,30): error TS2322: Type 'boolean' is not assignable to type 'TSchema'.`
- … and 36 more

### 🤪 `mixed` (9)

2 distinct first-line errors; most common:

- **5×** `a string is valid: the value does not match the type`
- **4×** `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`

Per group:

- `draft4/type.json::multiple types can be specified in an array` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)
- `draft6/ref.json::naive replacement of $ref with its destination is not correct` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `draft6/type.json::multiple types can be specified in an array` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)
- `draft7/ref.json::naive replacement of $ref with its destination is not correct` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `draft7/type.json::multiple types can be specified in an array` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)
- `draft2019-09/ref.json::naive replacement of $ref with its destination is not correct` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `draft2019-09/type.json::multiple types can be specified in an array` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)
- `draft2020-12/ref.json::naive replacement of $ref with its destination is not correct` — `accepted 'do not evaluate the $ref inside the enum, definition exact match' though valid: false`
  - cases: do not evaluate the $ref inside the enum, definition exact match (accepted), match the enum exactly (rejected)
- `draft2020-12/type.json::multiple types can be specified in an array` — `a string is valid: the value does not match the type`
  - cases: a string is valid (rejected), a float is invalid (accepted)

### 🚧 `false_reject` (19)

6 distinct first-line errors; most common:

- **5×** `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
- **4×** `[false] is valid: Expected false at <root>`
- **4×** `[true] is valid: Expected true at <root>`
- **4×** `same array is valid: Expected object at <root>`
- **1×** `ASCII letters match: Expected string to match '^\p{Letter}+$' at <root>`

Per group:

- `draft4/multipleOf.json::small multiple of large integer` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `draft4/ref.json::naive replacement of $ref with its destination is not correct` — `match the enum exactly: Expected required property at /type`
  - cases: match the enum exactly (rejected)
- `draft6/const.json::const with array` — `same array is valid: Expected object at <root>`
  - cases: same array is valid (rejected)
- `draft6/const.json::const with [false] does not match [0]` — `[false] is valid: Expected false at <root>`
  - cases: [false] is valid (rejected)
- `draft6/const.json::const with [true] does not match [1]` — `[true] is valid: Expected true at <root>`
  - cases: [true] is valid (rejected)
- `draft6/multipleOf.json::small multiple of large integer` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `draft7/const.json::const with array` — `same array is valid: Expected object at <root>`
  - cases: same array is valid (rejected)
- `draft7/const.json::const with [false] does not match [0]` — `[false] is valid: Expected false at <root>`
  - cases: [false] is valid (rejected)
- `draft7/const.json::const with [true] does not match [1]` — `[true] is valid: Expected true at <root>`
  - cases: [true] is valid (rejected)
- `draft7/multipleOf.json::small multiple of large integer` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `draft2019-09/const.json::const with array` — `same array is valid: Expected object at <root>`
  - cases: same array is valid (rejected)
- `draft2019-09/const.json::const with [false] does not match [0]` — `[false] is valid: Expected false at <root>`
  - cases: [false] is valid (rejected)
- `draft2019-09/const.json::const with [true] does not match [1]` — `[true] is valid: Expected true at <root>`
  - cases: [true] is valid (rejected)
- `draft2019-09/multipleOf.json::small multiple of large integer` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `draft2020-12/const.json::const with array` — `same array is valid: Expected object at <root>`
  - cases: same array is valid (rejected)
- `draft2020-12/const.json::const with [false] does not match [0]` — `[false] is valid: Expected false at <root>`
  - cases: [false] is valid (rejected)
- `draft2020-12/const.json::const with [true] does not match [1]` — `[true] is valid: Expected true at <root>`
  - cases: [true] is valid (rejected)
- `draft2020-12/multipleOf.json::small multiple of large integer` — `any integer is a multiple of 1e-8: Expected number to be a multiple of 1e-8 at <root>`
  - cases: any integer is a multiple of 1e-8 (rejected)
- `draft2020-12/pattern.json::pattern with Unicode property escape requires unicode mode` — `ASCII letters match: Expected string to match '^\p{Letter}+$' at <root>`
  - cases: ASCII letters match (rejected), Non-ASCII letters match (rejected)

### 🙈 `false_accept` (75)

18 distinct first-line errors; most common:

- **10×** `accepted 'ascii non-digits' though valid: false`
- **10×** `accepted 'literal unicode character in json string' though valid: false`
- **10×** `accepted 'number is invalid' though valid: false`
- **9×** `accepted 'with unevaluated properties' though valid: false`
- **5×** `accepted 'a float is not an integer' though valid: false`

Per group:

- `draft4/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft4/type.json::integer type matches integers` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `draft4/type.json::object type matches objects` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `draft4/type.json::type: array or object` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `draft4/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft6/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft6/type.json::integer type matches integers` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `draft6/type.json::object type matches objects` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `draft6/type.json::type: array or object` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `draft6/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft7/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft7/type.json::integer type matches integers` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `draft7/type.json::object type matches objects` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `draft7/type.json::type: array or object` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `draft7/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft2019-09/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft2019-09/type.json::integer type matches integers` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `draft2019-09/type.json::object type matches objects` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `draft2019-09/type.json::type: array or object` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `draft2019-09/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties schema` — `accepted 'with invalid unevaluated properties' though valid: false`
  - cases: with invalid unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties false` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with adjacent properties` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with adjacent patternProperties` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with nested properties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with nested patternProperties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with anyOf` — `accepted 'when one matches and has unevaluated properties' though valid: false`
  - cases: when one matches and has unevaluated properties (accepted), when two match and has unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with oneOf` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with not` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with if/then/else` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with if/then/else, then not defined` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with if/then/else, else not defined` — `accepted 'when if is true and has unevaluated properties' though valid: false`
  - cases: when if is true and has unevaluated properties (accepted), when if is false and has no unevaluated properties (accepted), when if is false and has unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with dependentSchemas` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with boolean schemas` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with $ref` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties before $ref` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties outside` — `accepted 'with no nested unevaluated properties' though valid: false`
  - cases: with no nested unevaluated properties (accepted), with nested unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::nested unevaluatedProperties, outer true, inner false, properties inside` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::cousin unevaluatedProperties, true and false, true with properties` — `accepted 'with no nested unevaluated properties' though valid: false`
  - cases: with no nested unevaluated properties (accepted), with nested unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::cousin unevaluatedProperties, true and false, false with properties` — `accepted 'with nested unevaluated properties' though valid: false`
  - cases: with nested unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::property is evaluated in an uncle schema to unevaluatedProperties` — `accepted 'uncle keyword evaluation is not significant' though valid: false`
  - cases: uncle keyword evaluation is not significant (accepted)
- `draft2019-09/unevaluatedProperties.json::in-place applicator siblings, allOf has unevaluated` — `accepted 'base case: both properties present' though valid: false`
  - cases: base case: both properties present (accepted), in place applicator siblings, foo is missing (accepted)
- `draft2019-09/unevaluatedProperties.json::in-place applicator siblings, anyOf has unevaluated` — `accepted 'base case: both properties present' though valid: false`
  - cases: base case: both properties present (accepted), in place applicator siblings, bar is missing (accepted)
- `draft2020-12/items.json::items and subitems` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted)
- `draft2020-12/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft2020-12/type.json::integer type matches integers` — `accepted 'a float is not an integer' though valid: false`
  - cases: a float is not an integer (accepted)
- `draft2020-12/type.json::object type matches objects` — `accepted 'an integer is not an object' though valid: false`
  - cases: an integer is not an object (accepted), a float is not an object (accepted), a string is not an object (accepted), an array is not an object (accepted), a boolean is not an object (accepted), null is not an object (accepted)
- `draft2020-12/type.json::type: array or object` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), null is invalid (accepted)
- `draft2020-12/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft4/optional/ecmascript-regex.json::patterns always use unicode semantics with patternProperties` — `accepted 'unicode matching is case-sensitive' though valid: false`
  - cases: unicode matching is case-sensitive (accepted)
- … and 25 more

