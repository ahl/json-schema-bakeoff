///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum Root {
    #[serde(rename = "bar")]
    Bar(i64),
    #[serde(rename = "foo")]
    Foo(::std::string::String),
}
impl ::std::convert::From<i64> for Root {
    fn from(value: i64) -> Self {
        Self::Bar(value)
    }
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
