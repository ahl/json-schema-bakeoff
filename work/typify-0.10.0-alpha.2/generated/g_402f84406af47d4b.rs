// draft2020-12/required.json::required default validation
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<::serde_json::Value>,
}
