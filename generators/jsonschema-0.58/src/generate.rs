//! Turn one schema group into a Rust module that validates with the
//! `jsonschema` crate.
//!
//! There is no code generation here in the usual sense: the module embeds
//! the group's schema verbatim and `__conformance_check` compiles it with
//! `jsonschema` (once, lazily) and validates the instance. "Deserialize
//! succeeded" means the validator found no error.
//!
//! Why this row exists: `jsonschema` passes essentially the whole official
//! suite, so wherever the bakeoff doesn't show ✅ for it, the cause is a
//! harness policy or a harness bug rather than the tool. See README.md.

use std::time::Duration;

/// Identity string: `jsonschema <pinned crate version>` (from build.rs).
pub const VERSION: &str = concat!("jsonschema ", env!("JSONSCHEMA_CRATE_VERSION"));

/// The pinned version on its own, for `batch-deps.toml`.
pub const CRATE_VERSION: &str = env!("JSONSCHEMA_CRATE_VERSION");

/// URI prefix the suite uses for its remote schemas.
pub const REMOTE_PREFIX: &str = "http://localhost:1234/";

/// The `jsonschema::Retrieve` implementation every module embeds, kept in
/// its own file so this crate's tests can `include!` the same text the
/// generated modules get. It uses the `REMOTES_DIR` and `REMOTE_PREFIX`
/// consts its includer defines.
const RETRIEVE: &str = include_str!("retrieve.inc.rs");

/// The `jsonschema::Draft` variant for a suite directory name.
///
/// The harness hands every group the directory it came from, and that is
/// taken as the authority on the schema's dialect -- see `module()` for why
/// it outranks the schema's own `$schema`.
pub fn draft_variant(suite_draft: &str) -> Option<&'static str> {
    Some(match suite_draft {
        "draft4" => "Draft4",
        "draft6" => "Draft6",
        "draft7" => "Draft7",
        "draft2019-09" => "Draft201909",
        "draft2020-12" => "Draft202012",
        _ => return None,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenOutcome {
    /// A complete module body (items only), including `__conformance_check`.
    Code(String),
    /// A suite draft name this generator has no `jsonschema::Draft` for.
    Error(String),
    Panic(String),
}

/// Build the module for one schema.
///
/// `schema_json` is the group's schema as JSON text, taken straight off the
/// wire (`serde_json::value::RawValue`) so the bytes the harness sent are
/// the bytes the validator sees -- no round trip through `f64` to blunt
/// `multipleOf: 0.0001` or a bignum bound.
///
/// `suite_draft` is the suite directory the group came from (`draft7`,
/// `draft2020-12`, ...); it selects the `jsonschema::Draft` the module
/// compiles with.
///
/// `remotes_dir` is an absolute path to `suite/remotes`; it is baked into
/// the module, because the batch crate runs with its own cwd and shouldn't
/// have to be told where the repo is.
///
/// Nothing in here can realistically panic, but it runs under the same
/// big-stack thread and `catch_unwind` as the real code generators do, so a
/// surprise lands in `generation_panic` instead of killing the run.
pub fn generate(schema_json: &str, suite_draft: &str, remotes_dir: &str, timeout: Duration) -> GenOutcome {
    let Some(draft) = draft_variant(suite_draft) else {
        return GenOutcome::Error(format!("no jsonschema::Draft for suite draft {suite_draft:?}"));
    };
    let schema_json = schema_json.to_string();
    let remotes_dir = remotes_dir.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("generate".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let outcome = match std::panic::catch_unwind(|| module(&schema_json, draft, &remotes_dir)) {
                Ok(code) => GenOutcome::Code(code),
                Err(payload) => GenOutcome::Panic(panic_message(&payload)),
            };
            let _ = tx.send(outcome);
        })
        .expect("spawn generation thread");
    match rx.recv_timeout(timeout) {
        Ok(outcome) => outcome,
        Err(_) => GenOutcome::Panic(format!("generation did not finish within {timeout:?}")),
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "non-string panic payload".to_string())
}

/// The module source for one schema.
fn module(schema_json: &str, draft: &str, remotes_dir: &str) -> String {
    let hashes = raw_string_hashes(schema_json);
    format!(
        r##"// jsonschema {version}: validate against the group's schema.
//
// The schema is embedded verbatim. The draft is the suite directory the
// group came from, passed to `with_draft` below; in this crate that
// outranks the schema's own `$schema`. See the generator's README.

/// Absolute path to the suite's `remotes/` directory, baked in at generate
/// time: the batch crate runs from a scratch directory of its own.
const REMOTES_DIR: &::std::primitive::str = {remotes_dir:?};

/// The one URI prefix we serve. Anything else is an error, so a run can
/// never reach the network.
const REMOTE_PREFIX: &::std::primitive::str = {remote_prefix:?};

static SCHEMA_JSON: &::std::primitive::str = r{h}"{schema}"{h};

/// Compiled once per process, on first use. A schema `jsonschema` refuses
/// to compile is kept as the error so every case in the group reports it.
static VALIDATOR: ::std::sync::LazyLock<
    ::std::result::Result<::jsonschema::Validator, ::std::string::String>,
> = ::std::sync::LazyLock::new(build);

{retrieve}
fn build() -> ::std::result::Result<::jsonschema::Validator, ::std::string::String> {{
    let schema: ::serde_json::Value =
        ::serde_json::from_str(SCHEMA_JSON).map_err(|e| e.to_string())?;
    // Formats are left at the crate's default, which is the spec's: an
    // assertion in draft 4/6/7, an annotation from 2019-09 on.
    ::jsonschema::options()
        .with_draft(::jsonschema::Draft::{draft})
        .with_retriever(SuiteRemotes)
        .build(&schema)
        .map_err(|e| e.to_string())
}}

pub fn __conformance_check(
    v: ::serde_json::Value,
) -> ::std::result::Result<(), ::std::string::String> {{
    match &*VALIDATOR {{
        ::std::result::Result::Ok(validator) => validator.validate(&v).map_err(|e| e.to_string()),
        ::std::result::Result::Err(e) => {{
            ::std::result::Result::Err(::std::format!("schema failed to compile: {{e}}"))
        }}
    }}
}}
"##,
        version = VERSION,
        draft = draft,
        remotes_dir = remotes_dir,
        remote_prefix = REMOTE_PREFIX,
        retrieve = RETRIEVE,
        h = hashes,
        schema = schema_json,
    )
}

/// The `#`s needed to wrap `s` in a raw string literal: one more than the
/// longest run of `#` that follows a `"` in the text.
fn raw_string_hashes(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut needed = 1;
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'"' {
            let run = bytes[i + 1..].iter().take_while(|c| **c == b'#').count();
            needed = needed.max(run + 1);
        }
    }
    "#".repeat(needed)
}

/// The embedded retriever, compiled here too so it can be tested against
/// the pinned crate instead of only pasted into generated modules. The two
/// consts stand in for the ones `module()` writes above the include; the
/// suite's own `remotes/` is the fixture.
#[cfg(test)]
mod retrieve {
    pub const REMOTES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../suite/remotes");
    pub const REMOTE_PREFIX: &str = super::REMOTE_PREFIX;
    include!("retrieve.inc.rs");

    /// What a generated module's `build()` would get back for `uri`.
    pub fn retrieve(uri: &str) -> Result<serde_json::Value, String> {
        use jsonschema::Retrieve;
        SuiteRemotes
            .retrieve(&jsonschema::Uri::parse(uri.to_string()).expect("parse uri"))
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: Duration = Duration::from_secs(10);

    /// The retriever serves the suite's remotes and nothing else. The
    /// refusals matter: this row is the only one that resolves remote
    /// references at all, and it must not be a way to read the filesystem
    /// or reach the network.
    #[test]
    fn retriever_serves_only_the_suite_remotes() {
        assert_eq!(
            super::retrieve::retrieve("http://localhost:1234/integer.json").unwrap(),
            serde_json::json!({"type": "integer"})
        );
        assert_eq!(
            super::retrieve::retrieve("http://localhost:1234/nested/string.json").unwrap(),
            serde_json::json!({"type": "string"})
        );

        // Another scheme, host or port is refused rather than fetched;
        // `https://localhost:1234/...` appears in the suite on purpose.
        for uri in [
            "https://localhost:1234/integer.json",
            "http://localhost:5678/integer.json",
            "http://example.com/integer.json",
            "json-schema:///integer.json",
        ] {
            let e = super::retrieve::retrieve(uri).unwrap_err();
            assert!(e.contains("refusing to retrieve"), "{uri}: {e}");
        }

        // An empty path, an absolute one (`join` would drop REMOTES_DIR and
        // read the file named by the URI), or a parent segment that somehow
        // survived normalization: all refused before any read.
        for uri in ["http://localhost:1234/", "http://localhost:1234//etc/hosts", "http://localhost:1234/sub/../x.json"] {
            let e = super::retrieve::retrieve(uri).unwrap_err();
            assert!(e.contains("not a plain path"), "{uri}: {e}");
        }
        // An interior `//` is not an escape: `Path` ignores the empty
        // segment, so it just names `nested/string.json`.
        assert_eq!(
            super::retrieve::retrieve("http://localhost:1234/nested//string.json").unwrap(),
            serde_json::json!({"type": "string"})
        );

        // A remote that isn't there is an error naming the file, which the
        // harness surfaces as a per-case reject.
        let e = super::retrieve::retrieve("http://localhost:1234/nope.json").unwrap_err();
        assert!(e.contains("nope.json"), "{e}");
        assert!(e.contains("No such file"), "{e}");
    }

    /// `..` never reaches the retriever: the referencing layer normalizes
    /// the reference first, and percent-decodes `%2e` on the way. Pinned
    /// because the guard above is otherwise the only thing standing between
    /// a `$ref` and the rest of the disk.
    #[test]
    fn dot_segments_are_normalized_before_retrieval() {
        for uri in ["http://localhost:1234/../../etc/hosts", "http://localhost:1234/%2e%2e/%2e%2e/etc/hosts"] {
            let parsed = jsonschema::Uri::parse(uri.to_string()).expect("parse").normalize();
            assert_eq!(parsed.as_str(), "http://localhost:1234/etc/hosts", "{uri}");
        }
    }

    #[test]
    fn hashes_clear_any_quote_hash_run() {
        assert_eq!(raw_string_hashes(r#"{"a": 1}"#), "#");
        assert_eq!(raw_string_hashes(r###"{"pattern": "\"##"}"###), "###");
        // A trailing quote is fine with one hash: `""#` doesn't terminate.
        assert_eq!(raw_string_hashes(r#""x""#), "#");
    }

    #[test]
    fn module_embeds_the_schema_verbatim() {
        let schema = r###"{"multipleOf":1e-8,"const":"has a \"quote\"#"}"###;
        let GenOutcome::Code(code) = generate(schema, "draft7", "/tmp/remotes", T) else {
            panic!("expected code");
        };
        assert!(code.contains(schema), "schema text not embedded as-is");
        assert!(code.contains("__conformance_check"));
        assert!(code.contains(r#""/tmp/remotes""#));
        assert!(code.contains("Draft::Draft7"));
    }

    #[test]
    fn every_suite_draft_maps_and_nothing_else_does() {
        for (dir, variant) in [
            ("draft4", "Draft4"),
            ("draft6", "Draft6"),
            ("draft7", "Draft7"),
            ("draft2019-09", "Draft201909"),
            ("draft2020-12", "Draft202012"),
        ] {
            assert_eq!(draft_variant(dir), Some(variant));
            let GenOutcome::Code(code) = generate("true", dir, "/tmp/remotes", T) else {
                panic!("{dir}: expected code");
            };
            assert!(code.contains(&format!("Draft::{variant}")), "{dir}");
        }
        // An unknown draft is an error, not a module that won't compile.
        assert!(matches!(generate("true", "draft3", "/tmp/remotes", T), GenOutcome::Error(_)));
    }

    /// The generated module's assumptions about the pinned API, checked
    /// against the pinned crate: formats are annotations by default from
    /// 2019-09 on, a bad schema is an error rather than a panic, and the
    /// draft comes from `$schema` when the schema carries one.
    #[test]
    fn pinned_api_behaves_as_the_template_assumes() {
        let v = jsonschema::options().build(&serde_json::json!({"format": "ipv4"})).unwrap();
        assert!(v.is_valid(&serde_json::json!("nope")), "2020-12 formats should be annotations");
        assert_eq!(v.draft(), jsonschema::Draft::Draft202012, "default draft");

        let seven = serde_json::json!({"$schema": "http://json-schema.org/draft-07/schema#", "format": "ipv4"});
        let v = jsonschema::options().build(&seven).unwrap();
        assert_eq!(v.draft(), jsonschema::Draft::Draft7, "$schema should pick the draft");
        assert!(!v.is_valid(&serde_json::json!("nope")), "draft 7 formats are assertions");

        // `with_draft` is what the modules use, and in this crate it is an
        // override rather than a default: it wins over the schema's own
        // `$schema`, in both directions. The generated modules rely on that,
        // so that the suite directory decides the dialect.
        let v = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft7)
            .build(&serde_json::json!({"format": "ipv4"}))
            .unwrap();
        assert!(!v.is_valid(&serde_json::json!("nope")));
        let v = jsonschema::options().with_draft(jsonschema::Draft::Draft202012).build(&seven).unwrap();
        assert_eq!(v.draft(), jsonschema::Draft::Draft202012, "with_draft outranks $schema");
        let twenty = serde_json::json!({"$schema": "https://json-schema.org/draft/2020-12/schema", "format": "ipv4"});
        let v = jsonschema::options().with_draft(jsonschema::Draft::Draft7).build(&twenty).unwrap();
        assert_eq!(v.draft(), jsonschema::Draft::Draft7, "and in the other direction");
        // Not just the reported draft: the keywords really are draft 7's, so
        // `format` asserts here even though the schema says 2020-12.
        assert!(!v.is_valid(&serde_json::json!("nope")), "with_draft decides how keywords behave");

        // Draft 4's boolean `exclusiveMaximum` is not valid 2020-12, and the
        // validator says so instead of panicking.
        assert!(jsonschema::options()
            .build(&serde_json::json!({"maximum": 3, "exclusiveMaximum": true}))
            .is_err());

        // Only the active draft's meta-schema is in the registry, which is
        // why `definitions.json::validate definition against metaschema`
        // needs the right draft to resolve its `$ref` at all. The retriever
        // never sees these URIs, so they were never a remote we declined.
        let meta = serde_json::json!({"$ref": "http://json-schema.org/draft-07/schema#"});
        assert!(jsonschema::options().with_draft(jsonschema::Draft::Draft202012).build(&meta).is_err());
        assert!(jsonschema::options().with_draft(jsonschema::Draft::Draft7).build(&meta).is_ok());
    }
}
