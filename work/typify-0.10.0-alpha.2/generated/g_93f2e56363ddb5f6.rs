// draft2019-09/additionalProperties.json::additionalProperties does not look in applicators
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, bool>,
}
