// draft2020-12/unevaluatedProperties.json::unevaluatedProperties with adjacent non-bool additionalProperties
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
