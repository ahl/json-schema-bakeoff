// draft2019-09/unevaluatedProperties.json::unevaluatedProperties with oneOf
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0 {
        bar: ::serde_json::Value,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        foo: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        baz: ::serde_json::Value,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        foo: ::std::option::Option<::std::string::String>,
    },
}
