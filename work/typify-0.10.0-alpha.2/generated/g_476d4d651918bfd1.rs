// draft7/required.json::required validation
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::serde_json::Value>,
    pub foo: ::serde_json::Value,
}
