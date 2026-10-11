//! Dependency bootstrap, shared by generate.mjs, build.mjs and probe.mjs.
//!
//! Nothing here may import an npm package: this module runs *before*
//! `node_modules` is known to exist. `npm ci` output goes to stderr so
//! stdout stays clean for the harness protocol.
//!
//! This is a copy of the same helpers in `generators/quicktype/deps.mjs`.
//! Generators don't import across directories: a generator's source hash
//! covers only its own files, so a shared module would let an edit in one
//! integration silently invalidate another's stored results.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, writeSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export const HERE = dirname(fileURLToPath(import.meta.url));

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
 * hundred load diagnostics in it clears that easily, and the harness would
 * see truncated JSON. So loop, and treat `EAGAIN` as "the reader hasn't
 * caught up yet".
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
  if (
    existsSync(join(HERE, "node_modules", "json-schema-to-zod", "package.json")) &&
    existsSync(join(HERE, "node_modules", "zod", "package.json"))
  ) {
    return;
  }
  process.stderr.write("json-schema-to-zod: node_modules missing, running npm ci\n");
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
