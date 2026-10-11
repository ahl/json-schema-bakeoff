///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<RootSubtype0>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<RootSubtype1>,
}
///`RootSubtype0`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RootSubtype0 {
    pub bar: i64,
}
///`RootSubtype1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RootSubtype1 {
    pub foo: ::std::string::String,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
