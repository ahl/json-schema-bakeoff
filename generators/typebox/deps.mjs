//! Dependency bootstrap, shared by generate.mjs, build.mjs and probe.mjs.
//!
//! Nothing here may import an npm package: this module runs *before*
//! `node_modules` is known to exist. `npm ci` output goes to stderr so
//! stdout stays clean for the harness protocol.
//!
//! This is a copy of the same helpers in `generators/json-schema-to-zod/deps.mjs`
//! and `generators/quicktype/deps.mjs`. Generators don't import across
//! directories: a generator's source hash covers only its own files, so a
//! shared module would let an edit in one integration silently invalidate
//! another's stored results.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, writeSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export const HERE = dirname(fileURLToPath(import.meta.url));

/** The packages this integration cannot run without. */
const REQUIRED = ["@sinclair/typebox", "schema2typebox", "typescript"];

/** stdout belongs to the harness protocol; send stray writes to stderr. */
export function quietStdout() {
  const real = process.stdout.write.bind(process.stdout);
  process.stdout.write = (chunk, enc, cb) => process.stderr.write(chunk, enc, cb);
  return real;
}

/**
 * Write every byte of `text` to `fd`.
 *
 * The protocol has to go out with `writeSync`, because `process.stdout`
 * is asynchronous on a macOS pipe and `process.exit` would truncate it.
 * But libuv puts fd 1 in non-blocking mode as soon as anything touches
 * `process.stdout` (which `quietStdout` does), and a single `writeSync` to
 * a non-blocking pipe stops at the pipe buffer — 64 KiB on macOS — and
 * reports the short count instead of failing. A build result with a few
 * hundred compiler diagnostics in it clears that easily, and the harness
 * would see truncated JSON. So loop, and treat `EAGAIN` as "the reader
 * hasn't caught up yet".
 */
export function writeAll(fd, text) {
  const buf = Buffer.isBuffer(text) ? text : Buffer.from(text, "utf8");
  let off = 0;
  while (off < buf.length) {
    try {
      off += writeSync(fd, buf, off, buf.length - off);
    } catch (e) {
      if (e.code !== "EAGAIN") throw e;
    }
  }
}

/** `npm ci` if the install is missing, so a fresh clone just works. */
export function ensureDeps() {
  if (REQUIRED.every((p) => existsSync(join(HERE, "node_modules", p, "package.json")))) {
    return;
  }
  process.stderr.write("typebox: node_modules missing, running npm ci\n");
  execFileSync("npm", ["ci", "--no-audit", "--no-fund"], {
    cwd: HERE,
    stdio: ["ignore", 2, 2],
  });
}

/** Version of an installed dependency, read from its own package.json. */
export function depVersion(name) {
  const path = join(HERE, "node_modules", name, "package.json");
  return JSON.parse(readFileSync(path, "utf8")).version;
}

/**
 * Refuse every `$ref` retrieval for the rest of this process — over the
 * network and off the filesystem alike.
 *
 * schema2typebox dereferences `$ref` with `@apidevtools/json-schema-ref-parser`
 * before it generates anything, and it offers no way to configure that: the
 * programmatic API takes only `{ input }` and calls
 * `$RefParser.dereference(schema)` with no options. Its default resolver
 * set is HTTP plus filesystem, and both have to go.
 *
 * - **HTTP.** Left alone the tool would fetch
 *   `http://localhost:1234/...` from the suite's `refRemote.json` *and*
 *   `http://json-schema.org/draft-07/schema` from the handful of groups
 *   that `$ref` a meta-schema. The resolver holds the `http` and `https`
 *   module objects and calls `protocol.get(...)` per request, so replacing
 *   those two functions before schema2typebox is imported is enough.
 * - **Filesystem.** The file resolver resolves a relative `$ref` against
 *   the process's cwd, which for a generator command is the generator
 *   directory — so `{"$ref": "package.json"}` would actually be read, an
 *   absolute `$ref` would read anywhere on disk, and the row's results
 *   would depend on what happens to sit next to this file. SKILL.md's
 *   trap list is explicit that a retriever must refuse anything outside
 *   `suite/remotes/`; a code generator resolves nothing at all, so the
 *   resolver's `read` is replaced too.
 *
 * Either way the tool sees a `ResolverError` and the message says why,
 * which is recorded like any other error it returns — the same shape as
 * quicktype's `SchemaFetchError` against an empty store.
 *
 * Both patches are load-bearing: if the resolver module can't be found,
 * that is a hard failure rather than a silently open sandbox.
 */
export function blockRefRetrieval() {
  const refuse = (what) => (arg) => {
    const where = typeof arg === "string" ? arg : (arg && (arg.url ?? arg.href ?? arg.host)) || "";
    throw new Error(`the bakeoff forbids ${what}${where ? ` (${where})` : ""}`);
  };
  for (const mod of ["node:http", "node:https"]) {
    // A `require`d module and an imported one are the same object, so
    // patching here is enough for the resolver's `protocol.get(...)`.
    const m = process.getBuiltinModule(mod);
    m.get = refuse(`network access, ${mod}.get`);
    m.request = refuse(`network access, ${mod}.request`);
  }
  const require_ = createRequire(join(HERE, "deps.mjs"));
  const file = require_("@apidevtools/json-schema-ref-parser/lib/resolvers/file");
  const { ResolverError } = require_("@apidevtools/json-schema-ref-parser/lib/util/errors");
  if (typeof file.read !== "function" || typeof ResolverError !== "function") {
    throw new Error("json-schema-ref-parser's file resolver isn't where it was; the sandbox would be open");
  }
  // A `ResolverError`, because `parse.js` relays one of those unchanged and
  // replaces anything else with a generic "Error reading file" that loses
  // the reason. The recorded message should say what happened.
  file.read = (f) =>
    Promise.reject(
      new ResolverError(new Error(`Error reading "${f && f.url}": the bakeoff forbids filesystem access`), f && f.url),
    );
}
