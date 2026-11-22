// Serves the case's own external schemas, keyed by retrieval URI, and
// nothing else.
//
// This file is embedded verbatim in every generated module and is also
// `include!`d by this crate's tests, so what it serves and refuses is
// exercised against the pinned `jsonschema` rather than only read. It must
// stay free of anything that isn't `std`, `serde_json` or `jsonschema`.
//
// `jsonschema` calls the retriever for every resource it does not already
// hold: a `$ref` target, and a `$schema` that names a custom meta-schema.
// Both are looked up in the same map. Nothing here touches the filesystem
// or the network.

struct ExternalSchemas(::std::collections::HashMap<::std::string::String, ::serde_json::Value>);

impl ExternalSchemas {
    /// Parse the case's `externalSchemas` object from the JSON text the
    /// harness sent.
    ///
    /// Each key is normalized the way the reference layer normalizes a URI
    /// before it calls the retriever, so a lookup matches whatever spelling
    /// the suite used.
    fn parse(text: &::std::primitive::str) -> ::std::result::Result<Self, ::std::string::String> {
        let map: ::serde_json::Map<::std::string::String, ::serde_json::Value> =
            ::serde_json::from_str(text).map_err(|e| ::std::format!("external schemas: {e}"))?;
        let mut out = ::std::collections::HashMap::with_capacity(map.len());
        for (key, schema) in map {
            let uri = ::jsonschema::Uri::parse(key.clone())
                .map_err(|(e, _)| ::std::format!("external schema {key:?}: {e}"))?
                .normalize();
            out.insert(uri.as_str().to_string(), schema);
        }
        ::std::result::Result::Ok(ExternalSchemas(out))
    }
}

impl ::jsonschema::Retrieve for ExternalSchemas {
    fn retrieve(
        &self,
        uri: &::jsonschema::Uri<::std::string::String>,
    ) -> ::std::result::Result<
        ::serde_json::Value,
        ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync>,
    > {
        let uri = uri.as_str();
        match self.0.get(uri) {
            ::std::option::Option::Some(schema) => ::std::result::Result::Ok(schema.clone()),
            ::std::option::Option::None => ::std::result::Result::Err(
                ::std::format!("refusing to retrieve {uri}: not one of this case's external schemas").into(),
            ),
        }
    }
}
