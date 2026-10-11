# JSON Schema codegen bakeoff

Runs the [JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
(vendored at `suite/`) through code generators and keeps score: for every
schema group, did the generator produce a type, did it compile, and does
deserializing each test case agree with the suite's `valid` flag?

The scoreboard is [RESULTS.md](RESULTS.md). Each row links to a page with
every failing group and its error. RESULTS.md and the per-tool pages are
generated: `cargo run -- run` rewrites them after storing results, and
`cargo run -- render` rebuilds them from stored results alone.

```sh
git submodule update --init
cargo run -- generators                       # what's under generators/
cargo run -- run                              # every generator, every draft
cargo run -- run --generator typify-0.10 --draft draft7
cargo run -- run --generator quicktype --target go
cargo run -- run --generator typify-0.10 --filter 'ref.json::relative pointer'
cargo run -- render                           # pages from stored data only
cargo run -- diff results/typify-0.10.0-alpha.2/results.json results/typify-0.10.0-beta.7/results.json
cargo run -- key
```

There are two actions. `run` executes the suite and stores data under
`results/<generator version>/results.json`; `render` turns stored data into
pages. A filtered run prints per-case detail with the path to the generated
code and writes under `target/` so it never touches stored results.
Everything scratch lives in `target/`; delete it freely.

[DESIGN.md](DESIGN.md) has the generator contract, the layout, the buckets,
and the caching rules. To add a generator, create `generators/<name>/` with
a `generator.toml` and the three commands it names.

## Generators

One directory each under `generators/`; each has a README with the version,
the options chosen, and the quirks found. The scoreboard's "The tools"
section has a paragraph on each tool's design center.

- `typify-0.10`, `typify-0.8`: [typify](https://crates.io/crates/typify), Rust.
  typify 0.3.0 has results but no generator; it was measured once with a copy
  of `typify-0.8` pinned to `=0.3.0` (plus a redirect of the tool's stray
  stdout) and the glue was then removed.
- `quicktype`: [quicktype](https://quicktype.io) via `quicktype-core`, targets
  `rust`, `go`, `typescript`.
- `schemafy-0.6`: [schemafy_lib](https://crates.io/crates/schemafy_lib), Rust.
  Generation runs in a supervised worker because the tool overflows the stack
  on some schemas.
- `json-schema-rs-0.0`: [json-schema-rs](https://crates.io/crates/json-schema-rs),
  Rust. Hidden from the scoreboard tables (its results are too poor to be
  informative); still measured and paged.
- `json-schema-to-zod`: [json-schema-to-zod](https://www.npmjs.com/package/json-schema-to-zod)
  with zod 4, TypeScript. Node runs the emitted code directly; the build step
  is a load probe.
- `typebox`: [TypeBox](https://github.com/sinclairzx81/typebox) via
  schema2typebox, TypeScript, compiled with tsc.
- `go-jsonschema`: [go-jsonschema](https://github.com/omissis/go-jsonschema)
  as a library, Go.
- `ogen`: [ogen](https://ogen.dev), Go. Each schema is wrapped as a component
  of an OpenAPI 3.1 document; the check calls the generated decoder and
  `Validate()`.
- `jsonschema-0.58`, `ajv`: validators, not generators (marked 𝓥 on the
  scoreboard). They calibrate the Rust and Node paths and resolve the suite's
  `localhost:1234` remotes from `suite/remotes/`. jsonschema leaves `format` at
  the library default (asserted through draft 7; 2020-12's required tests
  say it must be annotation-only); ajv ships no formats and ignores it.

## License

MIT; see [LICENSE](LICENSE). The test suite under `suite/` is a submodule
of [json-schema-org/JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite),
under its own MIT license.
