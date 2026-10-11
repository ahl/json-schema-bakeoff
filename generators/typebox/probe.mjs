//! `probe.mjs <batch_dir> <start>`: require each compiled module once, by
//! itself, and say whether it loads.
//!
//! `tsc` is the compiler, so this is not the "does it compile" step — it is
//! the step after it. TypeBox does much of its validation at *type
//! construction* time and the rest when `TypeCompiler.Compile` builds a
//! checker, and both of those run in a generated module's body:
//!
//! - `Type.Array([])` — what schema2typebox emits for an `enum` containing
//!   an empty array — throws from the constructor;
//! - `TypeCompiler.Compile` throws "Preflight validation check failed to
//!   guard for the given schema" for a type it can't build a checker for.
//!
//! A module that throws on load can't answer any case, so it belongs in
//! `compile_fail` with the tsc failures rather than rejecting every case of
//! its group. Probing it here, in a child process, one at a time, is also
//! how a module that takes the whole process down gets blamed on itself
//! rather than on its neighbours.
//!
//! Reads `<batch_dir>/keys.json`, requires `<batch_dir>/out/<key>.js` for
//! every key from `<start>` onward, and prints one line per module,
//! flushing after each:
//!
//!   {"ready": <start>}
//!   {"i": <index>, "r": "ok"}
//!   {"i": <index>, "r": {"fail": "<message>"}}
//!
//! The `ready` line comes out before the first require, so a probe that
//! never gets that far is a setup problem (no `keys.json`, a broken
//! install) rather than a module's fault. Indices are absolute so the
//! parent's cursor can't drift.

import { quietStdout, writeAll } from "./deps.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const [, , batchDir, startArg] = process.argv;
if (!batchDir || startArg === undefined) {
  process.stderr.write("usage: probe.mjs <batch_dir> <start>\n");
  process.exit(2);
}
const start = Number.parseInt(startArg, 10);
const keys = JSON.parse(readFileSync(join(batchDir, "keys.json"), "utf8"));
const outDir = join(batchDir, "out");
// tsc emits CommonJS, and `out/` carries a `"type": "commonjs"` marker.
const require_ = createRequire(join(outDir, "probe-anchor.cjs"));

// A module we load could print; stdout is the protocol.
quietStdout();
writeAll(1, JSON.stringify({ ready: start }) + "\n");

for (let i = start; i < keys.length; i++) {
  let r;
  try {
    const mod = require_(join(outDir, `${keys[i]}.js`));
    r = typeof mod.__conformanceCheck === "function" ? "ok" : { fail: "module exports no __conformanceCheck" };
  } catch (e) {
    r = { fail: describe(e) };
  }
  writeAll(1, JSON.stringify({ i, r }) + "\n");
}

/**
 * A load failure as a diagnostic: the message plus the first frame that
 * names a file, which for these is the line of the generated module that
 * threw.
 */
function describe(e) {
  if (!(e instanceof Error)) return String(e);
  const head = `${e.name}: ${e.message}`;
  const frame = (e.stack ?? "").split("\n").find((l) => /^\s+at /.test(l));
  return frame ? `${head}\n${frame.trim()}` : head;
}
