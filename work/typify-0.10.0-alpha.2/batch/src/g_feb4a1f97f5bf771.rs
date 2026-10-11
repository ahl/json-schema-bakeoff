///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    pub constructor: ::serde_json::Value,
    #[serde(rename = "__proto__")]
    pub proto: ::serde_json::Value,
    #[serde(rename = "toString")]
    pub to_string: ::serde_json::Value,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
