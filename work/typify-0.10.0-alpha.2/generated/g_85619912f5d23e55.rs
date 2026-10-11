// draft2019-09/unevaluatedProperties.json::unevaluatedProperties with boolean schemas
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::std::string::String>,
}
