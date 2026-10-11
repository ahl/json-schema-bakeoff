//! TypeBox generator for the bakeoff harness.
//!
//!   version              → "typebox <version> + schema2typebox <version>"
//!   generate <out_dir>   → stdin: JSON array of {key, draft, schema};
//!                          writes <out_dir>/<key>.ts;
//!                          stdout: JSON array of {key, status, message?}
//!
//! One long-lived process handles every group.
//!
//! ## Why schema2typebox is in the pipeline
//!
//! TypeBox is a *type builder*: `Type.Object({...})` produces an object
//! that is both a JSON Schema fragment and a TypeScript type, and
//! `TypeCompiler` / `Value` then check instances against it. What TypeBox
//! has no way to do is consume a JSON Schema it did not build. Its
//! checkers dispatch on the `Symbol.for('TypeBox.Kind')` property that the
//! `Type.*` constructors attach, so a plain schema object is rejected
//! outright:
//!
//!   Value.Check({ type: 'string' }, 'x')            → throws "Unknown type"
//!   TypeCompiler.Compile({ type: 'string' })        → throws "Preflight
//!                                                      validation check
//!                                                      failed to guard for
//!                                                      the given schema"
//!   Value.Check(Type.Unsafe({ type: 'string' }), 'x') → throws "Unknown type"
//!                                                      (Kind is 'Unsafe',
//!                                                      which nothing
//!                                                      implements)
//!
//! So the "wrap the raw schema" route does not exist, and neither does a
//! Json Schema front end in `@sinclair/typebox-codegen` 0.11.1: that
//! package maps *TypeScript source* to TypeBox (`TypeScriptToTypeBox`,
//! `TypeScriptToModel`) and then a TypeBox model out to zod, valibot,
//! io-ts, effect, arktype, yup and Json Schema. There is no
//! `JsonSchemaToTypeBox` and no `JsonSchemaToModel`; it is not installed
//! here. `@sinclair/typemap`, from the same author, is also a TypeScript
//! *syntax* front end, not a Json Schema one.
//!
//! The one Json Schema → TypeBox code generator is `schema2typebox`, which
//! TypeBox's own readme lists in its ecosystem table ("Creating TypeBox
//! code from Json Schemas"). It emits TypeScript source: `export const T =
//! Type.Object({...})` plus `export type T = Static<typeof T>`. That makes
//! this row "types generated from the schema, then checked by TypeBox's
//! compiler" — the same claim as typify's, in TypeScript — rather than
//! "schema handed to a validator", which is what the `ajv` row measures.
//!
//! ## What this glue adds to the tool's output
//!
//! The tool's text is kept verbatim and a block is appended: an alias
//! `Root` for whatever it exported, the `TypeCompiler.Compile` call, and
//! `__conformanceCheck`. Nothing is removed.
//!
//! schema2typebox names its export after the root schema's `title`
//! (pascal-cased), falling back to `T`; there is no option for it. The
//! contract asks for a root named `Root` "where the tool lets the caller
//! choose", and this one doesn't — injecting a `title` would be editing
//! the schema under measurement — so the emitted name is detected from the
//! `export const` line and aliased.
//!
//! Compiling at module scope is deliberate: `TypeCompiler.Compile` is
//! where TypeBox rejects a type it cannot build a checker for, and the
//! build step's load probe turns that into `compile_fail` rather than a
//! rejection of every case.
//!
//! ## Options
//!
//! None. `schema2typebox({ input })` takes no options beyond the schema
//! text, and TypeBox's `TypeSystemPolicy` is left at its default, which is
//! the Json Schema assertion policy rather than the TypeScript one. The
//! `FormatRegistry` is left empty — see the README for what that costs.
//!
//! ## Dialect
//!
//! Ignored. TypeBox targets Json Schema draft 7 and has no dialect
//! setting; schema2typebox has none either and never reads `$schema`. The
//! `draft` field arrives on stdin and is dropped, so no result here
//! depends on which suite directory a group came from.
//!
//! ## References
//!
//! schema2typebox dereferences `$ref` with `@apidevtools/json-schema-ref-parser`
//! before it generates anything. Local pointers are inlined (with a bug:
//! a root `$ref` is rewritten to `"#"` and then fails to parse); anything
//! else is retrieved, over HTTP or off the filesystem, and
//! `blockRefRetrieval()` refuses both. See deps.mjs.

import { ensureDeps, depVersion, quietStdout, writeAll, blockRefRetrieval } from "./deps.mjs";

const [, , cmd, outDir] = process.argv;

ensureDeps();

if (cmd === "version") {
  // writeAll, not process.stdout.write: stdout is a pipe here and
  // `process.exit` would truncate the asynchronous write.
  writeAll(1, `typebox ${depVersion("@sinclair/typebox")} + schema2typebox ${depVersion("schema2typebox")}\n`);
  process.exit(0);
}
if (cmd !== "generate" || !outDir) {
  process.stderr.write("usage: generate.mjs (version | generate <out_dir>)\n");
  process.exit(2);
}

// From here on stdout is the status array and nothing else.
quietStdout();
// Before schema2typebox (and so json-schema-ref-parser) is loaded.
blockRefRetrieval();

const { mkdirSync, writeFileSync } = await import("node:fs");
const { join } = await import("node:path");
const { schema2typebox } = await import("schema2typebox");

/** How long one schema gets before we call it a hang. */
const TIMEOUT_MS = 30_000;

/** A number kept as the text the harness sent, not as a float64. */
class RawNumber {
  constructor(text) {
    this.text = text;
  }
}

/**
 * Parse the harness's input without rounding any number.
 *
 * The harness serializes schemas with serde_json's `arbitrary_precision`,
 * so stdin carries the suite's digits exactly. schema2typebox takes the
 * schema as *text*, so the digits only survive if this process never turns
 * them into a float64 and back; `rawStringify` puts the original text back
 * where the number was.
 */
function parseRaw(text) {
  return JSON.parse(text, function (_key, value, context) {
    if (typeof value === "number" && context && typeof context.source === "string") {
      return new RawNumber(context.source);
    }
    return value;
  });
}

/** JSON.stringify, but numbers come back out exactly as they went in. */
function rawStringify(value) {
  if (value instanceof RawNumber) return value.text;
  if (value === null) return "null";
  if (Array.isArray(value)) return `[${value.map(rawStringify).join(",")}]`;
  if (typeof value === "object") {
    const parts = [];
    for (const key of Object.keys(value)) {
      parts.push(`${JSON.stringify(key)}:${rawStringify(value[key])}`);
    }
    return `{${parts.join(",")}}`;
  }
  return JSON.stringify(value);
}

/**
 * The first `export const <name> =` in the tool's output.
 *
 * schema2typebox emits exactly one, last. First-match anyway: the one way
 * a second could appear is the unescaped string interpolation in
 * `parseConst` (`Type.Literal("${schema.const}")`), and a `const` whose
 * text contains `export const` must not be able to retarget `Root`.
 */
function exportedName(code) {
  const m = code.match(/^export const ([A-Za-z_$][\w$]*) =/m);
  return m === null ? undefined : m[1];
}

/**
 * The module we emit for one group: the tool's output, then the entry
 * point the runner calls.
 *
 * `Root` is an alias rather than a rename, so the tool's own text stays
 * byte-for-byte what it produced. When the schema's `title` already made
 * the export `Root`, there is nothing to alias.
 *
 * The import sits at the bottom next to the code that needs it; import
 * declarations are hoisted, so position is cosmetic.
 */
function module_(code, name) {
  const alias = name === "Root" ? "" : `export const Root = ${name};\n\n`;
  return `${code}
// ---------------------------------------------------------------------------
// Appended by the bakeoff harness. Everything above is schema2typebox's.
// ---------------------------------------------------------------------------
import { TypeCompiler as __TypeCompiler } from "@sinclair/typebox/compiler";

${alias}// Compiling here rather than per case: a type TypeBox can't build a
// checker for is a build failure, not a rejected value.
const __checker = __TypeCompiler.Compile(Root);

export function __conformanceCheck(json: string): void {
  const value: unknown = JSON.parse(json);
  if (__checker.Check(value)) return;
  const first: any = __checker.Errors(value).First();
  if (first === undefined) throw new Error("the value does not match the type");
  throw new Error(\`\${first.message} at \${first.path === "" ? "<root>" : first.path}\`);
}
`;
}

/**
 * Buckets an exception from schema2typebox.
 *
 * The tool throws plain `Error`s for the things it knows it can't do —
 * `Unsupported schema. Did not match any type of the parsers` for a schema
 * with no `type` and no combinator it recognizes — and relays
 * json-schema-ref-parser's `ResolverError` for a `$ref` it can't reach.
 * Those are `error`: the tool returned an answer, and the answer was no.
 * Two ways it comes apart instead:
 *
 * - `TypeError: Converting circular structure to JSON` — the cycle that
 *   dereferencing a recursive `$ref` produces, reached through the
 *   `JSON.stringify` in the tool's own error message. 6 of the 7 🤯
 *   groups.
 * - `RangeError`, stack exhausted: the same cycle reached instead by the
 *   code generator's recursive descent, which has no visited set. 1 group.
 * - the timeout below, recognized by identity (`TIMED_OUT`) rather than by
 *   its message, so that a tool error that happens to mention a timeout
 *   can't be mistaken for one.
 *
 * `SyntaxError` is prettier failing to parse what the tool just wrote
 * (`Type.Literal({` for an object `const`), i.e. the tool emitted code
 * that isn't TypeScript. That is the tool returning broken output, not
 * crashing, so it stays an `error` — and it is reported as such rather
 * than written out, because the bad text never leaves the tool.
 */
function classify(e) {
  if (e === TIMED_OUT) {
    return { status: "panic", message: `timed out after ${TIMEOUT_MS}ms` };
  }
  const message = e instanceof Error ? `${e.name}: ${e.message}` : String(e);
  const cyclic = e instanceof TypeError && /circular structure/.test(e.message);
  if (e instanceof RangeError || cyclic) {
    return { status: "panic", message };
  }
  return { status: "error", message };
}

/** Rejection value of the timeout race, compared by identity. */
const TIMED_OUT = Symbol("timed out");

function timeout(ms) {
  let timer;
  const promise = new Promise((_resolve, reject) => {
    timer = setTimeout(() => reject(TIMED_OUT), ms);
  });
  return { promise, cancel: () => clearTimeout(timer) };
}

async function main() {
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  const groups = parseRaw(Buffer.concat(chunks).toString("utf8"));

  mkdirSync(outDir, { recursive: true });
  const statuses = [];
  for (const group of groups) {
    // Only numbers come back wrapped; `key` and `draft` are strings.
    const key = group.key;
    const guard = timeout(TIMEOUT_MS);
    // The tool is only nominally async — it awaits the dereference, then
    // generates synchronously — so a runaway recursion blocks the event
    // loop and this timer can't fire. Race anyway: it costs nothing and it
    // covers the asynchronous half.
    const work = schema2typebox({ input: rawStringify(group.schema) }).then(
      (code) => ({ code }),
      (error) => ({ error }),
    );
    let code;
    try {
      const done = await Promise.race([work, guard.promise]);
      if ("error" in done) throw done.error;
      if (typeof done.code !== "string" || done.code.trim() === "") {
        throw new Error(`the tool returned ${typeof done.code}, not source`);
      }
      code = done.code;
    } catch (e) {
      statuses.push({ key, ...classify(e) });
      continue;
    } finally {
      guard.cancel();
    }
    const name = exportedName(code);
    if (name === undefined) {
      statuses.push({ key, status: "error", message: "schema2typebox emitted no `export const`" });
      continue;
    }
    // Writing the module is ours, not the tool's: a failure here is a
    // broken run, not a schema schema2typebox couldn't handle.
    writeFileSync(join(outDir, `${key}.ts`), module_(code, name));
    statuses.push({ key, status: "ok" });
  }
  // stdout may be a pipe, where `write` is asynchronous and a process that
  // exits first truncates it.
  writeAll(1, JSON.stringify(statuses) + "\n");
}

await main();
// The status array is out, so we're done — but a generation we gave up on
// may still hold a timer, and node won't exit while it does.
process.exit(0);
