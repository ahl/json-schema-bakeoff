///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub constructor: ::std::option::Option<f64>,
    #[serde(
        rename = "__proto__",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub proto: ::std::option::Option<f64>,
    #[serde(rename = "toString", skip_serializing_if = "::std::option::Option::is_none")]
    pub to_string: ::std::option::Option<RootToString>,
}
///`RootToString`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct RootToString {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub length: ::std::option::Option<::std::string::String>,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
