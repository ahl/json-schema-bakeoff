// draft2019-09/unevaluatedProperties.json::in-place applicator siblings, allOf has unevaluated
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::serde_json::Value>,
}
