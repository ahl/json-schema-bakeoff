// Serves `http://localhost:1234/<path>` out of the suite's `remotes/<path>`.
//
// This file is embedded verbatim in every generated module and is also
// `include!`d by this crate's tests, so the guard below is exercised
// against the pinned `jsonschema` rather than only read. It depends on two
// consts its includer defines:
//
//   REMOTES_DIR    absolute path to `suite/remotes`
//   REMOTE_PREFIX  the one URI prefix we serve
//
// so it must stay free of anything else that isn't `std`, `serde_json` or
// `jsonschema`.

struct SuiteRemotes;

impl ::jsonschema::Retrieve for SuiteRemotes {
    fn retrieve(
        &self,
        uri: &::jsonschema::Uri<::std::string::String>,
    ) -> ::std::result::Result<
        ::serde_json::Value,
        ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync>,
    > {
        let uri = uri.as_str();
        let rest = uri.strip_prefix(REMOTE_PREFIX).ok_or_else(|| {
            ::std::format!("refusing to retrieve {uri}: only {REMOTE_PREFIX} is served (from the suite's remotes/)")
        })?;
        // The suite's remotes are all plain relative paths. `..` and `.`
        // segments are already gone by the time a reference reaches us (the
        // referencing layer normalizes, and it decodes `%2e` first), and an
        // absolute path would make `join` drop REMOTES_DIR entirely, so
        // insist on nothing but ordinary names: no root, no prefix, no
        // parent, no empty segment.
        let rel = ::std::path::Path::new(rest);
        let plain = !rest.is_empty()
            && rel
                .components()
                .all(|c| ::std::matches!(c, ::std::path::Component::Normal(_)));
        if !plain {
            return ::std::result::Result::Err(
                ::std::format!("{uri}: not a plain path under the suite's remotes/").into(),
            );
        }
        let path = ::std::path::Path::new(REMOTES_DIR).join(rel);
        let text = ::std::fs::read_to_string(&path)
            .map_err(|e| ::std::format!("{uri}: reading {}: {e}", path.display()))?;
        ::std::result::Result::Ok(::serde_json::from_str(&text)?)
    }
}
