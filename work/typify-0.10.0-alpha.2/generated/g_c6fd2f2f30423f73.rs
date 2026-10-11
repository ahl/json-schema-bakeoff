// draft7/oneOf.json::oneOf complex types
// root type: Root

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
