# ajv 8.20.0

The JSON Schema validator for JavaScript, and for much of the ecosystem the
reference implementation: it compiles a schema into purpose-built JS and runs
that at the boundary. No types come out; the schema stays the artifact. It's
the reference row for the Node path, resolving `localhost:1234` remotes from
the case's own `externalSchemas` and shipping no formats, so `format` is
ignored everywhere.

_generated 2026-11-22T02:30:50Z_

**498 of 540 standard groups pass (92.2%)**

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
| 498 | 6 | 20 | 5 | 4 | 0 | 4 | 3 | 540 | 92.2% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 1593 | 1644 | 96.9% | 1664 | 95.7% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor-legacy.json | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 21 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| dependentRequired.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| dependentSchemas.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| dynamicRef.json | 2 | 2 | 12 | 0 | 2 | 0 | 3 | 0 | 21 | 🚧🚧🚧🚧❌✅🚧🚧🚧💀🚧💀🚧✅🚧🚧🚧❌❌🙈🙈 |
| enum.json | 16 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅❌✅✅ |
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
| id-legacy.json | 38 | 0 | 0 | 2 | 0 | 0 | 0 | 0 | 40 | ✅✅✅✅✅🤪🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| id.json | 22 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 25 | ✅✅✅🤯🤯✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤯✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection-legacy.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 28 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 28 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| multipleOf.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| not.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| patternProperties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| properties.json | 5 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅🚧 |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| recursiveRef.json | 9 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅🤪✅✅✅✅ |
| ref-legacy.json | 13 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 14 | ✅✅✅🚧✅✅✅✅✅✅✅✅✅✅ |
| ref.json | 23 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 23 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅🙈 |
| type.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 39 | 3 | 3 | 1 | 1 | 0 | 0 | 0 | 47 | ✅✅✅✅✅✅✅✅✅✅✅✅✅🤪✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅💀✅✅✅✅✅🙈🙈🙈🚧✅✅🚧🚧✅✅ |
| unevaluatedProperties.json | 42 | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 45 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🤪✅✅✅✅✅💀✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅🚧✅✅✅✅ |
| uniqueItems.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| vocabulary.json | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 🚧✅🚧✅ |
| **Total** | **498** | **6** | **20** | **5** | **4** | **0** | **4** | **3** | **540** |  |

### Failures

#### 🤯 `generation_panic` (3)

- `id.json::refs with relative uris and defs (2020-12)` — `RangeError: Maximum call stack size exceeded`
- `id.json::relative refs with absolute uris and defs (2020-12)` — `RangeError: Maximum call stack size exceeded`
- `id.json::URN ref with nested pointer ref (2020-12)` — `RangeError: Maximum call stack size exceeded`

#### ❌ `generation_error` (4)

- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (2020) (2020-12)` — `Error: "$dynamicRef" only supports hash fragment reference`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema with a matching $dynamicAnchor resolves to the first $dynamicAnchor in the dynamic scope (2020-12)` — `Error: "$dynamicRef" only supports hash fragment reference`
- `dynamicRef.json::A $dynamicRef that initially resolves to a schema without a matching $dynamicAnchor behaves like a normal $ref to $anchor (2020-12)` — `Error: "$dynamicRef" only supports hash fragment reference`
- `enum.json::empty enum (2020-12)` — `Error: enum must have non-empty array`

#### 💀 `runtime_panic` (4)

- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (2020) (2020-12)` — `number is valid: Maximum call stack size exceeded`
  - cases: number is valid (panicked), non-number is invalid (panicked)
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (2020) (2020-12)` — `data is sufficient for schema at second#/$defs/length: Maximum call stack size exceeded`
  - cases: data is sufficient for schema at second#/$defs/length (panicked), data is not sufficient for schema at second#/$defs/length (panicked)
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (2020-12) (2020-12)` — `with no unevaluated items: Maximum call stack size exceeded`
  - cases: with no unevaluated items (panicked), with unevaluated items (panicked)
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (2020) (2020-12)` — `with no unevaluated properties: Maximum call stack size exceeded`
  - cases: with no unevaluated properties (panicked), with unevaluated properties (panicked)

#### 🤪 `mixed` (5)

- `id-legacy.json::$ref prevents a sibling $id from changing the base uri (7)` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `id-legacy.json::$ref prevents a sibling id from changing the base uri (4)` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial target schema resource (2019-09)` — `leaf node matches: recursion uses the inner schema: data must be boolean, data/foo must be integer, data/foo/bar must be boolean, data/foo/bar must be object, d…`
  - cases: leaf node matches: recursion uses the inner schema (rejected), leaf node does not match: recursion uses the inner schema (accepted)
- `unevaluatedItems.json::unevaluatedItems with nested items (2020-12)` — `with no additional items: data/true must be boolean`
  - cases: with no additional items (rejected), with invalid additional item (accepted)
- `unevaluatedProperties.json::unevaluatedProperties with if/then/else, then not defined (2020-12)` — `when if is true and has no unevaluated properties: data must NOT have unevaluated properties`
  - cases: when if is true and has no unevaluated properties (rejected), when if is false and has unevaluated properties (accepted)

#### 🚧 `false_reject` (20)

11 distinct first-line errors; most common:

- **4×** `An array of strings is valid: data/0 must be array`
- **3×** `Any array is valid: data/0 must be array`
- **3×** `correct extended schema: data/elements/0 must have required property 'elements'`
- **2×** `no validation: invalid number, but it still validates: data/numberProperty must be >= 10`
- **2×** `valid in case if is evaluated: data must NOT have more than 0 items`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (2020) (2020-12)` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (2020) (2020-12)` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (2020) (2020-12)` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (2020) (2020-12)` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (2020) (2020-12)` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (2020) (2020-12)` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (2020) (2020-12)` — `correct extended schema: data/elements/0 must have required property 'elements'`
  - cases: correct extended schema (rejected)
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (2020) (2020-12)` — `integer property passes: data/bar-item/content must be object`
  - cases: integer property passes (rejected)
- `dynamicRef.json::A $dynamicRef to an $anchor in the same schema resource behaves like a normal $ref to an $anchor (2020-12)` — `An array of strings is valid: data/0 must be array`
  - cases: An array of strings is valid (rejected)
- `dynamicRef.json::A $dynamicRef without anchor in fragment behaves identical to $ref (2020-12)` — `An array of numbers is valid: data/0 must be array`
  - cases: An array of numbers is valid (rejected)
- `dynamicRef.json::A $dynamicRef without a matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `dynamicRef.json::A $dynamicRef with a non-matching $dynamicAnchor in the same schema resource behaves like a normal $ref to $anchor (2020-12)` — `Any array is valid: data/0 must be array`
  - cases: Any array is valid (rejected)
- `properties.json::properties whose names are Javascript object property names (2020-12)` — `none of the properties mentioned: data/constructor must be number`
  - cases: none of the properties mentioned (rejected)
- `ref-legacy.json::ref overrides any sibling keywords (7)` — `ref valid, maxItems ignored: data/foo must NOT have more than 2 items`
  - cases: ref valid, maxItems ignored (rejected)
- `unevaluatedItems.json::unevaluatedItems with minContains = 0 (2020-12)` — `all items evaluated by contains: data must NOT have more than 0 items`
  - cases: all items evaluated by contains (rejected)
- `unevaluatedItems.json::unevaluatedItems can see annotations from if without then and else (prefixItems) (2020-12)` — `valid in case if is evaluated: data must NOT have more than 0 items`
  - cases: valid in case if is evaluated (rejected)
- `unevaluatedItems.json::unevaluatedItems can see annotations from if without then and else (2019-09)` — `valid in case if is evaluated: data must NOT have more than 0 items`
  - cases: valid in case if is evaluated (rejected)
- `unevaluatedProperties.json::unevaluatedProperties can see annotations from if without then and else (2020-12)` — `valid in case if is evaluated: data must NOT have unevaluated properties`
  - cases: valid in case if is evaluated (rejected)
- `vocabulary.json::schema that uses custom metaschema with no validation vocabulary (recursive anchor) (2019-09)` — `no validation: invalid number, but it still validates: data/numberProperty must be >= 10`
  - cases: no validation: invalid number, but it still validates (rejected)
- `vocabulary.json::schema that uses custom metaschema with no validation vocabulary (2020-12)` — `no validation: invalid number, but it still validates: data/numberProperty must be >= 10`
  - cases: no validation: invalid number, but it still validates (rejected)

#### 🙈 `false_accept` (6)

6 distinct first-line errors; most common:

- **1×** `accepted '7 not evaluated, fails unevaluatedItems' though valid: false`
- **1×** `accepted 'contains passes, second item is not evaluated' though valid: false`
- **1×** `accepted 'follow $dynamicRef to a false schema' though valid: false`
- **1×** `accepted 'none of the properties mentioned' though valid: false`
- **1×** `accepted 'number list with string values' though valid: false`

Per group:

- `dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (2020) (2020-12)` — `accepted 'number list with string values' though valid: false`
  - cases: number list with string values (accepted), string list with number values (accepted)
- `dynamicRef.json::$dynamicRef points to a boolean schema (2020-12)` — `accepted 'follow $dynamicRef to a false schema' though valid: false`
  - cases: follow $dynamicRef to a false schema (accepted)
- `required.json::required properties whose names are Javascript object property names (2020-12)` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `unevaluatedItems.json::unevaluatedItems depends on adjacent contains (2020-12)` — `accepted 'contains passes, second item is not evaluated' though valid: false`
  - cases: contains passes, second item is not evaluated (accepted)
- `unevaluatedItems.json::unevaluatedItems depends on multiple nested contains (2020-12)` — `accepted '7 not evaluated, fails unevaluatedItems' though valid: false`
  - cases: 7 not evaluated, fails unevaluatedItems (accepted)
- `unevaluatedItems.json::unevaluatedItems and contains interact to control item dependency relationship (2020-12)` — `accepted 'only b's are invalid' though valid: false`
  - cases: only b's are invalid (accepted), only c's are invalid (accepted), only b's and c's are invalid (accepted), only a's and c's are invalid (accepted)

## Optional

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 47 | 37 | 10 | 0 | 0 | 0 | 4 | 0 | 98 | 48.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 477 | 996 | 47.9% | 1006 | 47.4% |

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
| optional/bignum.json | 3 | 5 | 6 | 0 | 0 | 0 | 0 | 0 | 14 | ✅✅✅🙈🚧🚧🙈🚧🚧🙈🙈🚧🚧🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🚧🚧 |
| optional/dependencies-compatibility.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/dynamicRef.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/format-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/non-schemas.json | 11 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 13 | ✅✅✅❌❌✅✅✅✅✅✅✅✅ |
| optional/zero-terminated-floats.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **47** | **37** | **10** | **0** | **0** | **0** | **4** | **0** | **98** |  |

### Failures

#### ❌ `generation_error` (4)

- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: false (2020-12)` — `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-false.jso…`
- `optional/format-assertion.json::schema that uses custom metaschema with format-assertion: true (2020-12)` — `Error: can't resolve reference https://json-schema.org/draft/2020-12/meta/format-assertion from id http://localhost:1234/draft2020-12/format-assertion-true.json`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier ($defs) (2020-12)` — `Error: reference "https://localhost:1234/unknownKeyword/my_identifier.json" resolves to more than one schema`
- `optional/non-schemas.json::$id inside an unknown keyword is not a real identifier (7)` — `Error: reference "https://localhost:1234/unknownKeyword/my_identifier.json" resolves to more than one schema`

#### 🚧 `false_reject` (10)

7 distinct first-line errors; most common:

- **2×** `above the exclusive minimum is valid: data must be > -9.727837981879871e+26`
- **2×** `below the exclusive maximum is valid: data must be < 9.727837981879871e+26`
- **2×** `valid if optional overflow handling is implemented: data must be multiple of 0.5`
- **1×** `distinct high-precision decimals are unique: data must NOT have duplicate items (items ## 0 and 1 are identical)`
- **1×** `first item not a string is valid: data/0 must be string`

Per group:

- `optional/bignum.json::float comparison with high precision (boolean) (4)` — `below the exclusive maximum is valid: data must be < 9.727837981879871e+26`
  - cases: below the exclusive maximum is valid (rejected)
- `optional/bignum.json::float comparison with high precision (2020-12)` — `below the exclusive maximum is valid: data must be < 9.727837981879871e+26`
  - cases: below the exclusive maximum is valid (rejected)
- `optional/bignum.json::float comparison with high precision on negative numbers (boolean) (4)` — `above the exclusive minimum is valid: data must be > -9.727837981879871e+26`
  - cases: above the exclusive minimum is valid (rejected)
- `optional/bignum.json::float comparison with high precision on negative numbers (2020-12)` — `above the exclusive minimum is valid: data must be > -9.727837981879871e+26`
  - cases: above the exclusive minimum is valid (rejected)
- `optional/bignum.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)
- `optional/bignum.json::uniqueItems comparison with high precision (2020-12)` — `distinct high-precision decimals are unique: data must NOT have duplicate items (items ## 0 and 1 are identical)`
  - cases: distinct high-precision decimals are unique (rejected), arrays containing distinct decimals are unique (rejected), objects containing distinct decimals are unique (rejected)
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (7) (2020-12)` — `missing bar is valid: data must have property bar when property foo is present`
  - cases: missing bar is valid (rejected)
- `optional/cross-draft.json::refs to historic drafts are processed as historic drafts (2019) (2020-12)` — `first item not a string is valid: data/0 must be string`
  - cases: first item not a string is valid (rejected)
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (2020) (2020-12)` — `integer property passes: data/bar-item/content must be object`
  - cases: integer property passes (rejected)
- `optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled (2020-12)` — `valid if optional overflow handling is implemented: data must be multiple of 0.5`
  - cases: valid if optional overflow handling is implemented (rejected)

#### 🙈 `false_accept` (37)

36 distinct first-line errors; most common:

- **2×** `accepted 'a slightly smaller decimal is invalid' though valid: false`
- **1×** `accepted '1e20 is not multiple of 3' though valid: false`
- **1×** `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
- **1×** `accepted 'a float is not an integer even without fractional part' though valid: false`
- **1×** `accepted 'a invalid date string with 32 days in January' though valid: false`

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
- `optional/bignum.json::minimum integer comparison (2020-12)` — `accepted 'just below the minimum is invalid' though valid: false`
  - cases: just below the minimum is invalid (accepted)
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
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2019) (7)` — `accepted 'missing bar is invalid' though valid: false`
  - cases: missing bar is invalid (accepted)
- `optional/cross-draft.json::refs to future drafts are processed as future drafts (2020) (2019-09)` — `accepted 'first item not a string is invalid' though valid: false`
  - cases: first item not a string is invalid (accepted)
- `optional/format-regex.json::\a is not an ECMA 262 control escape (2020-12)` — `accepted 'when used as a pattern' though valid: false`
  - cases: when used as a pattern (accepted)
- `optional/format-regex.json::Python-specific regular expression syntax is not valid ECMA 262 (2020-12)` — `accepted 'Python named group (?P<name>...) is not ECMA 262' though valid: false`
  - cases: Python named group (?P<name>...) is not ECMA 262 (accepted), Python named backreference (?P=name) is not ECMA 262 (accepted), inline comment group (?#...) is not ECMA 262 (accepted)
- `optional/format-regex.json::global inline flag groups are not valid ECMA 262 (2020-12)` — `accepted 'a single global inline flag (?i)' though valid: false`
  - cases: a single global inline flag (?i) (accepted), multiple global inline flags (?ims) (accepted)
- `optional/format-time.json::validation of time strings (2020-12)` — `accepted 'invalid leap second, Zulu (wrong hour)' though valid: false`
  - cases: invalid leap second, Zulu (wrong hour) (accepted), invalid leap second, Zulu (wrong minute) (accepted), invalid leap second, zero time-offset (wrong hour) (accepted), invalid leap second, zero time-offset (wrong minute) (accepted), invalid leap second, positive time-offset (wrong hour) (accepted), invalid leap second, positive time-offset (wrong minute) (accepted), … and 2 more
- `optional/zero-terminated-floats.json::some languages do not distinguish between different types of numeric value (4)` — `accepted 'a float is not an integer even without fractional part' though valid: false`
  - cases: a float is not an integer even without fractional part (accepted)

## Compound

| ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 100.0% |

### Per-case match rate

| Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---:|---:|---:|---:|---:|
| 34 | 34 | 100.0% | 34 | 100.0% |

### Chart

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| compound/externally-tagged-union.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| **Total** | **5** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **5** |  |

### Failures

_none: every group passed._

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

- **25×** `Error: unsupported dialect v1: ajv has no class for it`

Per group:

- `dynamicRef.json::A $dynamicRef to a $dynamicAnchor in the same schema resource behaves like a normal $ref to an $anchor (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::A $dynamicRef resolves to the first $dynamicAnchor still in scope that is encountered when the schema is evaluated (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::A $dynamicRef with intermediate scopes that don't include a matching $dynamicAnchor does not affect dynamic scope resolution (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::An $anchor with the same name as a $dynamicAnchor is not used for dynamic scope resolution (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::after leaving a dynamic scope, it is not used by a $dynamicRef (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::strict-tree schema, guards against misspelled properties (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::tests for implementation dynamic anchor and reference link (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $defs first (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::$ref and $dynamicAnchor are independent of order - $ref first (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::$ref to $dynamicRef finds detached $dynamicAnchor (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::$dynamicRef skips over intermediate resources - direct reference (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `dynamicRef.json::$dynamicRef avoids the root of each schema, but scopes are still registered (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `format-date-time.json::validation of date-time strings with RFC 9557 suffixes (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `format-time.json::leap seconds are not valid times (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `optional/dynamicRef.json::$dynamicRef skips over intermediate resources - pointer reference across resource boundary (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/additionalProperties.json::propertyDependencies with additionalProperties (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/dynamicRef.json::multiple dynamic paths to the $dynamicRef keyword (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/dynamicRef.json::$dynamicAnchor inside propertyDependencies (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-objects (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/propertyDependencies.json::propertyDependencies doesn't act on non-string property values (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/propertyDependencies.json::multiple options selects the right one (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/unevaluatedProperties.json::unevaluatedProperties can see inside propertyDependencies (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `proposals/propertyDependencies/unevaluatedProperties.json::propertyDependencies with unevaluatedProperties (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `unevaluatedItems.json::unevaluatedItems with $dynamicRef (v1)` — `Error: unsupported dialect v1: ajv has no class for it`
- `unevaluatedProperties.json::unevaluatedProperties with $dynamicRef (v1)` — `Error: unsupported dialect v1: ajv has no class for it`

