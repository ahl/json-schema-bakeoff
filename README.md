# JSON Schema codegen bakeoff

Runs the [JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
(vendored at `suite/`) through code generators and keeps score: for every
schema group, did the generator produce a type, did it compile, and does
deserializing each test case agree with the suite's `valid` flag?

The scoreboard is [RESULTS.md](RESULTS.md). Each row links to a page with
every failing group and its error.

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

- `typify-0.10`: [typify](https://github.com/oxidecomputer/typify) 0.10 line,
  pinned in its `Cargo.toml`. The group's `definitions` are registered with
  `add_ref_types`, then the schema is added with the suggested name `Root`;
  `add_root_schema` is avoided because injecting a title makes typify panic
  on merged schemas. A schema that doesn't deserialize as a schemars 0.8
  `RootSchema` (e.g. draft 4's boolean `exclusiveMaximum`) is a
  `generation_error`. typify has no notion of draft; `$schema` is ignored.
