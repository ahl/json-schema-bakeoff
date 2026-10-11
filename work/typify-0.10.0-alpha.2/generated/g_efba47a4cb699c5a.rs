// draft2020-12/oneOf.json::oneOf with missing optional property
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0 {
        bar: ::serde_json::Value,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        baz: ::std::option::Option<::serde_json::Value>,
    },
    Variant1 { foo: ::serde_json::Value },
}
