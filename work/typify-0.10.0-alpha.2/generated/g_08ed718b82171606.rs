// draft2020-12/unevaluatedProperties.json::unevaluatedProperties with not
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    pub bar: ::serde_json::Value,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::std::string::String>,
}
