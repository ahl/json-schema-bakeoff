//! `probe.mjs <batch_dir> <start>`: load each emitted module once, by
//! itself, and say whether it loads.
//!
//! This is the "does it compile" step for a generator whose output is
//! source code that node runs directly (see build.mjs). Reads
//! `<batch_dir>/keys.json`, imports `<batch_dir>/<key>.ts` for every key
//! from `<start>` onward, and prints one line per module, flushing after
//! each:
//!
//!   {"ready": <start>}
//!   {"i": <index>, "r": "ok"}
//!   {"i": <index>, "r": {"fail": "<message>"}}
//!
//! The `ready` line comes out before the first import, so a probe that
//! never gets that far is a setup problem (no `keys.json`, a broken node)
//! rather than a module's fault — the same distinction the harness's runner
//! loop draws. Indices are absolute so the parent's cursor can't drift, and
//! the flush-per-line is what lets the parent attribute a module that takes
//! the whole process down (stack exhaustion while zod builds the schema) to
//! the right key: whatever index never reported is the culprit.

import { quietStdout, writeAll } from "./deps.mjs";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [, , batchDir, startArg] = process.argv;
if (!batchDir || startArg === undefined) {
  process.stderr.write("usage: probe.mjs <batch_dir> <start>\n");
  process.exit(2);
}
const start = Number.parseInt(startArg, 10);
const keys = JSON.parse(readFileSync(join(batchDir, "keys.json"), "utf8"));

// A module we import could print; stdout is the protocol.
quietStdout();
writeAll(1, JSON.stringify({ ready: start }) + "\n");

for (let i = start; i < keys.length; i++) {
  const url = pathToFileURL(join(batchDir, `${keys[i]}.ts`));
  let r;
  try {
    const mod = await import(url.href);
    r = typeof mod.__conformanceCheck === "function" ? "ok" : { fail: "module exports no __conformanceCheck" };
  } catch (e) {
    r = { fail: describe(e) };
  }
  writeAll(1, JSON.stringify({ i, r }) + "\n");
}

/**
 * A load failure as a diagnostic.
 *
 * Two kinds land here and both belong in the same bucket, because both
 * mean the emitted source is not usable as code:
 *
 * - a syntax error, i.e. the tool wrote something node can't parse
 *   (TypeScript that isn't erasable, an expression it built wrong);
 * - a throw from the module body, i.e. zod refused to *build* the schema
 *   the tool asked for (`z.string().ip()` — removed in zod 4 —, a regex
 *   literal JS won't accept, a `z.literal` of something unsupported).
 *
 * Keep one line of location with the message. A throw has a stack whose
 * frames name files; a syntax error from node's type stripping has no
 * useful frame at all (every frame is inside `node:internal/modules`) and
 * puts the file and line in the text *above* the error instead, so look
 * there too.
 */
function describe(e) {
  if (!(e instanceof Error)) return String(e);
  const head = `${e.name}: ${e.message}`;
  const lines = (e.stack ?? "").split("\n");
  const frame = lines.find((l) => /^\s+at /.test(l) && l.includes("file://"));
  const preamble = lines.find((l) => /^file:\/\/\S+:\d+$/.test(l.trim()));
  const where = frame ?? (preamble ? `at ${preamble.trim()}` : lines.find((l) => /^\s+at /.test(l)));
  return where ? `${head}\n${where.trim()}` : head;
}
