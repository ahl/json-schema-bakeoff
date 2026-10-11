// Embedded verbatim into every generated module by generate.mjs, below the
// four constants the generator writes out (AJV_MODULES, REMOTES_DIR, DRAFT,
// SCHEMA_TEXT). Kept in its own file so it is readable, reviewable and part
// of the generator's source hash rather than a string in a template.
//
// CommonJS on purpose: the generated modules live under target/, where
// there is no package.json of our own to declare ESM, and `require` of an
// absolute path is how a module reaches the packages installed next to the
// generator.

const fs = require("fs");
const path = require("path");

/** `require` a package installed next to the generator, by absolute path. */
function dep(spec) {
  return require(path.join(AJV_MODULES, spec));
}

/**
 * Every file under `suite/remotes/`, keyed by the URL the suite references
 * it as: `http://localhost:1234/<path relative to remotes/>`.
 *
 * This is the whole of the remote-reference policy. ajv does not fetch
 * anything unless it is handed a `loadSchema`, and it is not: these 79
 * files are the only remotes that exist, they come off the local disk, and
 * any other URI is a missing reference.
 */
function loadRemotes() {
  const out = [];
  const walk = (dir, base) => {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    entries.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
    for (const entry of entries) {
      const rel = base ? `${base}/${entry.name}` : entry.name;
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) walk(full, rel);
      else if (entry.name.endsWith(".json")) {
        out.push({ url: `http://localhost:1234/${rel}`, schema: JSON.parse(fs.readFileSync(full, "utf8")) });
      }
    }
  };
  walk(REMOTES_DIR, "");
  return out;
}

/**
 * The remotes, read once per process rather than once per module: a draft
 * is a thousand-odd modules in one node process and they all want the same
 * 79 files.
 */
function remotes() {
  return (globalThis.__bakeoffAjvRemotes ??= loadRemotes());
}

/**
 * A fresh Ajv for the suite directory this group came from.
 *
 * The dialect is the class, so the suite directory picks it and a schema's
 * own `$schema` cannot change it — which is the policy this row wants (see
 * README.md). Options:
 *
 * - `strict: false`, because the suite is full of keywords, `$id` shapes
 *   and formats that ajv's strict mode rejects outright; strict mode is a
 *   linting layer over the spec, and this row is measuring the spec.
 * - `allErrors: false`, the default: one error is all the harness reports.
 * - `logger: false`, because stdout is the harness protocol and ajv's
 *   default logger is `console`.
 *
 * `validateFormats` is left at its default (`true`). It is moot: the `ajv`
 * package ships no format implementations, so with `strict: false` every
 * format in the suite is an unknown one and is ignored. Installing
 * `ajv-formats` would change that, and deliberately isn't.
 */
function newAjv(draft) {
  const opts = { strict: false, allErrors: false, logger: false };
  switch (draft) {
    case "draft4":
      // ajv-draft-04 is the draft-04 dialect: `id`, the boolean form of
      // `exclusiveMaximum`/`exclusiveMinimum`, no `const`/`contains`.
      return new (dep("ajv-draft-04").default)(opts);
    case "draft6": {
      // ajv has no draft-06 dialect; draft-07 is the closest it ships, and
      // the two differ only in keywords draft 6 doesn't test. The draft-06
      // meta-schema has to be added by hand so a `$ref` to it resolves.
      const ajv = new (dep("ajv").default)(opts);
      ajv.addMetaSchema(dep("ajv/dist/refs/json-schema-draft-06.json"));
      return ajv;
    }
    case "draft7":
      return new (dep("ajv").default)(opts);
    case "draft2019-09":
      return new (dep("ajv/dist/2019").default)(opts);
    case "draft2020-12":
      return new (dep("ajv/dist/2020").default)(opts);
    default:
      throw new Error(`unknown draft ${draft}`);
  }
}

let AJV = null;
let VALIDATE = null;
let COMPILE_ERROR = null;
let COMPILED = false;

function compile() {
  const ajv = newAjv(DRAFT);
  for (const { url, schema } of remotes()) {
    // A remote that declares `$vocabulary` is a meta-schema — there are
    // six, and the suite points some group's `$schema` at every one of
    // them — and ajv will only accept a `$schema` it was given as one.
    //
    // Both calls pass `_validateSchema = false`. The remotes are fixtures,
    // not the subject, and every remote is registered on every instance,
    // so a 2020-12 remote would otherwise be checked against draft 4's
    // meta-schema and rejected for saying `$defs`.
    if (schema !== null && typeof schema === "object" && schema.$vocabulary !== undefined) {
      ajv.addMetaSchema(schema, url, false);
    } else {
      ajv.addSchema(schema, url, false, false);
    }
  }
  // JSON.parse of the text the harness sent, never a re-serialized copy:
  // see the SCHEMA_TEXT comment in the generated preamble.
  const validate = ajv.compile(JSON.parse(SCHEMA_TEXT));
  AJV = ajv;
  VALIDATE = validate;
}

/**
 * Compile the schema, once. Throws whatever ajv threw, every time it is
 * called, so a schema ajv rejects reads the same way on the first case and
 * the hundredth.
 *
 * The generate step calls this to decide a group's status, so a schema ajv
 * refuses is a `generation_error` and never reaches the runner. This is
 * still here as the guard for the case where it somehow does.
 */
function __conformanceCompile() {
  if (!COMPILED) {
    COMPILED = true;
    try {
      compile();
    } catch (e) {
      COMPILE_ERROR = e instanceof Error ? e : new Error(String(e));
    }
  }
  if (COMPILE_ERROR !== null) throw COMPILE_ERROR;
}

/**
 * Validate one instance. Returns normally when the instance is valid and
 * throws with ajv's own `errorsText` when it isn't — the shape the runner
 * turns into accept/reject.
 *
 * `instance` is a parsed JavaScript value, which is the only thing ajv
 * validates. Where that loses digits (an integer past 2^53) it loses them
 * the way every JavaScript JSON parser does; see README.md on bignums.
 */
function __conformanceCheck(instance) {
  __conformanceCompile();
  if (VALIDATE(instance)) return;
  throw new Error(AJV.errorsText(VALIDATE.errors));
}

module.exports = { __conformanceCheck, __conformanceCompile, DRAFT, SCHEMA_TEXT };
