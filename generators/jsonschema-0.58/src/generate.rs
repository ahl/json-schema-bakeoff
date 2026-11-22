//! Turn one schema group into a Rust module that validates with the
//! `jsonschema` crate.
//!
//! There is no code generation here in the usual sense: the module embeds
//! the group's schema and its external schemas verbatim, and
//! `__conformance_check` compiles the schema with `jsonschema` (once,
//! lazily) and validates the instance. "Deserialize succeeded" means the
//! validator found no error.
//!
//! Why this row exists: `jsonschema` passes essentially the whole official
//! suite, so wherever the bakeoff doesn't show ✅ for it, the cause is a
//! harness policy or a harness bug rather than the tool. See README.md.

use std::time::Duration;

/// Identity string: `jsonschema <pinned crate version>` (from build.rs).
pub const VERSION: &str = concat!("jsonschema ", env!("JSONSCHEMA_CRATE_VERSION"));

/// The pinned version on its own, for `batch-deps.toml`.
pub const CRATE_VERSION: &str = env!("JSONSCHEMA_CRATE_VERSION");

/// The `jsonschema::Retrieve` implementation every module embeds, kept in
/// its own file so this crate's tests can `include!` the same text the
/// generated modules get.
const RETRIEVE: &str = include_str!("retrieve.inc.rs");

/// The `jsonschema::Draft` variant for a harness dialect name.
///
/// The harness hands every group the dialect it runs under, and that is
/// taken as the authority on the schema's dialect -- see `module()` for why
/// it outranks the schema's own `$schema`. `v1` has no variant in the
/// pinned crate.
pub fn draft_variant(dialect: &str) -> Option<&'static str> {
    Some(match dialect {
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
    /// A dialect this generator has no `jsonschema::Draft` for.
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
/// `dialect` is the dialect the harness runs the group under (`draft7`,
/// `draft2020-12`, ...); it selects the `jsonschema::Draft` the module
/// compiles with.
///
/// `external_json` is the case's `externalSchemas` object as JSON text,
/// taken off the wire the same way, or `None` when the case has none. It is
/// embedded in the module and served by the retriever, and nothing else is.
///
/// Nothing in here can realistically panic, but it runs under the same
/// big-stack thread and `catch_unwind` as the real code generators do, so a
/// surprise lands in `generation_panic` instead of killing the run.
pub fn generate(schema_json: &str, external_json: Option<&str>, dialect: &str, timeout: Duration) -> GenOutcome {
    let Some(draft) = draft_variant(dialect) else {
        return GenOutcome::Error(format!("unsupported dialect {dialect:?}: no jsonschema::Draft for it"));
    };
    let schema_json = schema_json.to_string();
    let external_json = external_json.unwrap_or("{}").to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("generate".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let outcome = match std::panic::catch_unwind(|| module(&schema_json, &external_json, draft)) {
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
fn module(schema_json: &str, external_json: &str, draft: &str) -> String {
    let hashes = raw_string_hashes(schema_json);
    let external_hashes = raw_string_hashes(external_json);
    format!(
        r##"// {version}: validate against the group's schema.
//
// The schema is embedded verbatim. The draft is the dialect the harness
// runs the group under, passed to `with_draft` below; in this crate that
// outranks the schema's own `$schema`. See the generator's README.

static SCHEMA_JSON: &::std::primitive::str = r{h}"{schema}"{h};

/// The case's external schemas, retrieval URI to schema, as the text the
/// harness sent. The retriever serves these and refuses everything else.
static EXTERNAL_SCHEMAS_JSON: &::std::primitive::str = r{eh}"{external}"{eh};

/// Compiled once per process, on first use. A schema `jsonschema` refuses
/// to compile is kept as the error so every case in the group reports it.
static VALIDATOR: ::std::sync::LazyLock<
    ::std::result::Result<::jsonschema::Validator, ::std::string::String>,
> = ::std::sync::LazyLock::new(build);

{retrieve}
fn build() -> ::std::result::Result<::jsonschema::Validator, ::std::string::String> {{
    let schema: ::serde_json::Value =
        ::serde_json::from_str(SCHEMA_JSON).map_err(|e| e.to_string())?;
    let external = ExternalSchemas::parse(EXTERNAL_SCHEMAS_JSON)?;
    // Formats are left at the crate's default, which is the spec's: an
    // assertion in draft 4/6/7, an annotation from 2019-09 on.
    ::jsonschema::options()
        .with_draft(::jsonschema::Draft::{draft})
        // Formats stay at the library default (asserted through draft 7,
        // annotation-only after). Asserting them everywhere would pass the
        // optional format files for 2019-09 and 2020-12 but fail 19 required
        // 2020-12 groups whose cases say an invalid string "is only an
        // annotation by default" and mark it valid. Required conformance wins.
        .with_retriever(external)
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
        retrieve = RETRIEVE,
        h = hashes,
        schema = schema_json,
        eh = external_hashes,
        external = external_json,
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
/// the pinned crate instead of only pasted into generated modules.
#[cfg(test)]
mod retrieve {
    include!("retrieve.inc.rs");

    /// The fixture: two external schemas as a case would supply them, one a
    /// plain schema and one a custom meta-schema.
    pub const EXTERNAL: &str = r#"{
        "http://localhost:1234/integer.json": {"type": "integer"},
        "http://localhost:1234/nested/string.json": {"type": "string"},
        "http://localhost:1234/draft2020-12/metaschema-no-validation.json": {
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://localhost:1234/draft2020-12/metaschema-no-validation.json",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/applicator": true,
                "https://json-schema.org/draft/2020-12/vocab/core": true
            },
            "$dynamicAnchor": "meta",
            "allOf": [
                {"$ref": "https://json-schema.org/draft/2020-12/meta/applicator"},
                {"$ref": "https://json-schema.org/draft/2020-12/meta/core"}
            ]
        }
    }"#;

    fn external() -> ExternalSchemas {
        ExternalSchemas::parse(EXTERNAL).expect("parse fixture")
    }

    /// Whether `text` parses as a case's external schemas.
    pub fn parse(text: &str) -> Result<(), String> {
        ExternalSchemas::parse(text).map(|_| ())
    }

    /// What the retriever for external schemas `text` serves for `uri`.
    pub fn retrieve_in(text: &str, uri: &str) -> Result<serde_json::Value, String> {
        use jsonschema::Retrieve;
        ExternalSchemas::parse(text)
            .expect("parse external schemas")
            .retrieve(&jsonschema::Uri::parse(uri.to_string()).expect("parse uri").normalize())
            .map_err(|e| e.to_string())
    }

    /// What a generated module's `build()` would get back for `uri`.
    pub fn retrieve(uri: &str) -> Result<serde_json::Value, String> {
        retrieve_in(EXTERNAL, uri)
    }

    /// A validator built the way a generated module builds one.
    pub fn build(draft: jsonschema::Draft, schema: serde_json::Value) -> Result<jsonschema::Validator, String> {
        jsonschema::options()
            .with_draft(draft)
            .with_retriever(external())
            .build(&schema)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: Duration = Duration::from_secs(10);

    /// The retriever serves the case's external schemas and nothing else.
    /// The refusals matter: the validator rows are the only ones that
    /// resolve remote references at all, and a URI the case did not supply
    /// must never turn into a read or a fetch.
    #[test]
    fn retriever_serves_only_the_case_schemas() {
        assert_eq!(
            super::retrieve::retrieve("http://localhost:1234/integer.json").unwrap(),
            serde_json::json!({"type": "integer"})
        );
        assert_eq!(
            super::retrieve::retrieve("http://localhost:1234/nested/string.json").unwrap(),
            serde_json::json!({"type": "string"})
        );

        // Another scheme, host or port is refused rather than fetched;
        // `https://localhost:1234/...` appears in the suite on purpose. So
        // is a suite remote this case did not supply, and anything that
        // looks like a path on disk.
        for uri in [
            "https://localhost:1234/integer.json",
            "http://localhost:5678/integer.json",
            "http://example.com/integer.json",
            "json-schema:///integer.json",
            "http://localhost:1234/name-defs.json",
            "http://localhost:1234/",
            "http://localhost:1234//etc/hosts",
            "file:///etc/hosts",
        ] {
            let e = super::retrieve::retrieve(uri).unwrap_err();
            assert!(e.contains("refusing to retrieve"), "{uri}: {e}");
            assert!(e.contains(uri), "{uri}: {e}");
        }

        // A case with no external schemas serves nothing.
        assert!(super::retrieve::retrieve_in("{}", "http://localhost:1234/integer.json").is_err());

        // Something other than an object is an error, not an empty map.
        assert!(super::retrieve::parse("[]").is_err());
    }

    /// Keys are normalized like the URIs the reference layer hands the
    /// retriever, so a lookup matches whichever way the suite spelled one.
    #[test]
    fn keys_are_normalized() {
        let text = r#"{"HTTP://LOCALHOST:1234/a/../integer.json": true}"#;
        assert_eq!(
            super::retrieve::retrieve_in(text, "http://localhost:1234/integer.json").unwrap(),
            serde_json::json!(true)
        );
    }

    /// End to end against the pinned crate: a `$ref` to an external schema
    /// resolves through the retriever, so does a `$schema` naming a custom
    /// meta-schema, and a `$ref` the case did not supply fails the compile
    /// with the retriever's message.
    #[test]
    fn retriever_backs_ref_and_schema() {
        use jsonschema::Draft;
        let v = super::retrieve::build(Draft::Draft202012, serde_json::json!({"$ref": "http://localhost:1234/integer.json"})).unwrap();
        assert!(v.is_valid(&serde_json::json!(1)));
        assert!(!v.is_valid(&serde_json::json!("a")));

        // The custom meta-schema leaves out the validation vocabulary, so
        // `minimum` is not a keyword here; the suite's
        // `vocabulary.json` case relies on this.
        let meta = serde_json::json!({
            "$schema": "http://localhost:1234/draft2020-12/metaschema-no-validation.json",
            "properties": {"numberProperty": {"minimum": 10}}
        });
        let v = super::retrieve::build(Draft::Draft202012, meta).unwrap();
        assert!(v.is_valid(&serde_json::json!({"numberProperty": 1})));

        let e = super::retrieve::build(Draft::Draft202012, serde_json::json!({"$ref": "http://localhost:1234/name-defs.json"}))
            .unwrap_err();
        assert!(e.contains("refusing to retrieve"), "{e}");
    }

    /// `..` never reaches the retriever: the referencing layer normalizes
    /// the reference first, and percent-decodes `%2e` on the way. The
    /// retriever normalizes its keys the same way, so its lookups depend on
    /// this.
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
        let external = r###"{"http://localhost:1234/a.json":{"maximum":18446744073709551615,"const":"\"##"}}"###;
        let GenOutcome::Code(code) = generate(schema, Some(external), "draft7", T) else {
            panic!("expected code");
        };
        assert!(code.contains(schema), "schema text not embedded as-is");
        assert!(code.contains(&format!("r###\"{external}\"###")), "external schemas not embedded as-is");
        assert!(code.contains("__conformance_check"));
        assert!(code.contains("Draft::Draft7"));

        // No external schemas is an empty map, not a missing constant.
        let GenOutcome::Code(code) = generate(schema, None, "draft7", T) else {
            panic!("expected code");
        };
        assert!(code.contains(r##"EXTERNAL_SCHEMAS_JSON: &::std::primitive::str = r#"{}"#;"##));
    }

    #[test]
    fn every_released_dialect_maps_and_nothing_else_does() {
        for (dir, variant) in [
            ("draft4", "Draft4"),
            ("draft6", "Draft6"),
            ("draft7", "Draft7"),
            ("draft2019-09", "Draft201909"),
            ("draft2020-12", "Draft202012"),
        ] {
            assert_eq!(draft_variant(dir), Some(variant));
            let GenOutcome::Code(code) = generate("true", None, dir, T) else {
                panic!("{dir}: expected code");
            };
            assert!(code.contains(&format!("Draft::{variant}")), "{dir}");
        }
        // An unknown dialect, `v1` among them, is an error, not a module that
        // won't compile.
        for dialect in ["v1", "draft3"] {
            let GenOutcome::Error(m) = generate("true", None, dialect, T) else {
                panic!("{dialect}: expected an error");
            };
            assert!(m.contains("unsupported dialect") && m.contains(dialect), "{m}");
        }
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
        // so that the harness's dialect decides.
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
