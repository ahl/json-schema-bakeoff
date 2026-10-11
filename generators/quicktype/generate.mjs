//! quicktype generator for the bakeoff harness.
//!
//!   version <target>              → "quicktype <version> <target>"
//!   generate <target> <out_dir>   → stdin: JSON array of {key, schema};
//!                                   writes <out_dir>/<key>.{rs,go,ts};
//!                                   stdout: JSON array of {key, status, message?}
//!
//! One long-lived process handles every group. Each schema becomes a
//! `JSONSchemaInput` with a single source named `Root`, so the top-level
//! type is `Root` in every language.
//!
//! References are resolved against a store that holds nothing: anything
//! with an address (the suite's `http://localhost:1234/...` remotes) comes
//! back undefined, which quicktype reports as `SchemaFetchError`. That is
//! the policy — remotes are recorded, never fetched.

import { ensureDeps, depVersion, quietStdout, writeAll } from "./deps.mjs";

const TARGETS = { rust: "rs", go: "go", typescript: "ts" };

const [, , cmd, target, outDir] = process.argv;
if (!(target in TARGETS)) {
  process.stderr.write(`usage: generate.mjs (version|generate) <${Object.keys(TARGETS).join("|")}> [out_dir]\n`);
  process.exit(2);
}

ensureDeps();

if (cmd === "version") {
  // writeAll, not process.stdout.write: stdout is a pipe here and
  // `process.exit` would truncate the asynchronous write.
  writeAll(1, `quicktype ${depVersion("quicktype-core")} ${target}\n`);
  process.exit(0);
}
if (cmd !== "generate" || !outDir) {
  process.stderr.write("usage: generate.mjs generate <target> <out_dir>\n");
  process.exit(2);
}

// From here on stdout is the status array and nothing else; quicktype's
// own console output (and anything a dependency prints) goes to stderr.
quietStdout();

const { mkdirSync, writeFileSync } = await import("node:fs");
const { join } = await import("node:path");
const { quicktype, InputData, JSONSchemaInput, JSONSchemaStore, QuickTypeError } = await import("quicktype-core");

/** How long one schema gets before we call it a hang. */
const TIMEOUT_MS = 30_000;

/**
 * A store with no schemas in it. `fetch` returning undefined is how
 * quicktype learns an address is unresolvable; throwing here instead would
 * escape as an opaque error and lose the `SchemaFetchError` label.
 */
class EmptyStore extends JSONSchemaStore {
  async fetch(_address) {
    return undefined;
  }
}

/**
 * Renderer options, per target.
 *
 * - rust: `visibility: public` so the module's types are reachable from
 *   the batch crate's dispatch table; `leading-comments: false` drops the
 *   `extern crate` example block, which is invalid inside a module even
 *   commented (it is also just noise). Density and integer-type stay at
 *   their defaults (`normal`, `conservative`), the latter keeping the i64
 *   range rather than narrowing everything to i32.
 * - go: `package` is the module key, because each group becomes its own
 *   Go package directory. `just-types` stays off so quicktype emits
 *   `UnmarshalRoot`, which is the entry point we call.
 * - typescript: `just-types` off and `runtime-typecheck` on (both
 *   defaults, set explicitly because they are the whole point) so the
 *   output carries the `Convert` class that actually validates at runtime;
 *   with `just-types` on, TypeScript interfaces check nothing at all.
 */
function rendererOptions(target, key) {
  switch (target) {
    case "rust":
      return { visibility: "public", "leading-comments": "false" };
    case "go":
      return { package: key, "just-types": "false", "multi-file-output": "false" };
    case "typescript":
      return { "just-types": "false", "runtime-typecheck": "true" };
  }
}

/**
 * The per-module entry point the build step calls. Appended to whatever
 * quicktype rendered; keeping it in the module (rather than in the
 * runner) means a module whose root type came out unusable fails to
 * compile on its own and is attributed to its own key.
 */
function checkEntryPoint(target) {
  switch (target) {
    case "rust":
      return [
        "",
        "pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {",
        "    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())",
        "}",
        "",
      ].join("\n");
    case "go":
      return [
        "",
        "func ConformanceCheck(data []byte) error {",
        "\t_, err := UnmarshalRoot(data)",
        "\treturn err",
        "}",
        "",
      ].join("\n");
    case "typescript":
      return [
        "",
        "export function __conformanceCheck(json: string): void {",
        "    Convert.toRoot(json);",
        "}",
        "",
      ].join("\n");
  }
}

/**
 * Fix up rendered source that can't stand alone as a module.
 *
 * quicktype's Rust output opens with `use serde::{Serialize,
 * Deserialize};`, which is fine inside a non-root module in edition 2018+.
 * `extern crate` lines are not, so drop them if a future version emits
 * them uncommented.
 */
function patchSource(target, code) {
  if (target !== "rust") return code;
  return code
    .split("\n")
    .filter((line) => !/^\s*extern\s+crate\b/.test(line))
    .join("\n");
}

async function generateOne(target, key, schema) {
  const input = new JSONSchemaInput(new EmptyStore());
  // A single source named `Root` makes `Root` the top-level type name.
  await input.addSource({ name: "Root", schema: JSON.stringify(schema) });
  const inputData = new InputData();
  inputData.addInput(input);
  const result = await quicktype({
    inputData,
    lang: target,
    rendererOptions: rendererOptions(target, key),
  });
  return result.lines.join("\n");
}

/**
 * Buckets an exception.
 *
 * quicktype signals "I can't represent this" by throwing a
 * `QuickTypeError` with a named message, so those are generation *errors*.
 * The one exception is `InternalError`, which is what `assert()` and
 * `panic()` raise from inside the type graph: that's a crash wearing an
 * error's clothes. Anything that isn't a `QuickTypeError` at all
 * (`RangeError: Maximum call stack size exceeded` on a self-referential
 * root, say) is a crash too.
 */
function classify(e) {
  const message = e instanceof Error ? `${e.name}: ${e.message}` : String(e);
  if (e instanceof QuickTypeError && e.messageName !== "InternalError") {
    return { status: "error", message: `${e.messageName}: ${e.userMessage ?? e.message}` };
  }
  return { status: "panic", message };
}

function timeout(ms) {
  let timer;
  const promise = new Promise((_resolve, reject) => {
    timer = setTimeout(() => reject(new Error(`timed out after ${ms}ms`)), ms);
  });
  return { promise, cancel: () => clearTimeout(timer) };
}

async function main() {
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  const groups = JSON.parse(Buffer.concat(chunks).toString("utf8"));

  mkdirSync(outDir, { recursive: true });
  const ext = TARGETS[target];
  const statuses = [];
  for (const { key, schema } of groups) {
    const guard = timeout(TIMEOUT_MS);
    // The loser of a `Promise.race` keeps running; settle the generation
    // into a value so a late rejection can't become an unhandled one.
    const work = generateOne(target, key, schema).then(
      (code) => ({ code }),
      (error) => ({ error }),
    );
    try {
      const done = await Promise.race([work, guard.promise]);
      if ("error" in done) throw done.error;
      writeFileSync(join(outDir, `${key}.${ext}`), patchSource(target, done.code) + checkEntryPoint(target));
      statuses.push({ key, status: "ok" });
    } catch (e) {
      statuses.push({ key, ...classify(e) });
    } finally {
      guard.cancel();
    }
  }
  // stdout may be a pipe, where `write` is asynchronous and a process that
  // exits first truncates it.
  writeAll(1, JSON.stringify(statuses) + "\n");
}

await main();
// The status array is out, so we're done — but a generation we gave up on
// may still hold a timer or a socket, and node won't exit while it does.
// Leave on our own terms rather than hanging the harness.
process.exit(0);
