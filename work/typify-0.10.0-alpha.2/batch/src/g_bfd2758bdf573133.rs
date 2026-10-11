///`One`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum One {
    Variant0(Two),
    Variant1 { b: ::serde_json::Value },
    Variant2 { xx: ::serde_json::Value },
    Variant3 { all: ::serde_json::Value },
}
impl ::std::convert::From<Two> for One {
    fn from(value: Two) -> Self {
        Self::Variant0(value)
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(One),
    Variant1 { a: ::serde_json::Value },
}
impl ::std::convert::From<One> for Root {
    fn from(value: One) -> Self {
        Self::Variant0(value)
    }
}
///`Two`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum Two {
    #[serde(rename = "c")]
    C(::serde_json::Value),
    #[serde(rename = "d")]
    D(::serde_json::Value),
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
