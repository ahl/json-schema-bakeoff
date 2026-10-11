///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(::serde_json::Value),
    Variant1(::serde_json::Value),
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
