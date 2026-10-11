// draft2019-09/unevaluatedProperties.json::unevaluatedProperties with $recursiveRef
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub branches: ::std::option::Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub node: ::serde_json::Value,
}
///`Tree`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Tree {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub branches: ::std::option::Option<::serde_json::Value>,
    pub node: ::serde_json::Value,
}
