///`One`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct One {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub a: ::std::option::Option<::serde_json::Value>,
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0 {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        a: ::std::option::Option<::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        b: ::std::option::Option<::serde_json::Value>,
        x: ::serde_json::Value,
    },
    Variant1 {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        a: ::std::option::Option<::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        b: ::std::option::Option<::serde_json::Value>,
        y: ::serde_json::Value,
    },
}
///`Two`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Two {
    pub x: ::serde_json::Value,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
