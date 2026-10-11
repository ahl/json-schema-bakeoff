///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(f64),
    Variant1(::serde_json::Value),
}
impl ::std::convert::From<f64> for Root {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::serde_json::Value> for Root {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant1(value)
    }
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
