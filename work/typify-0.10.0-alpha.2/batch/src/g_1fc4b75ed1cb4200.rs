///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<i64>,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
