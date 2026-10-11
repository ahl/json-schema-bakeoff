///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bar: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub foo: ::std::vec::Vec<::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, i64>,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
