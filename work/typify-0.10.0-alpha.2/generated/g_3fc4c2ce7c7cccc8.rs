// draft2019-09/additionalProperties.json::additionalProperties with schema
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, bool>,
}
