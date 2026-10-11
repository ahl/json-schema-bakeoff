///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub foo: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
