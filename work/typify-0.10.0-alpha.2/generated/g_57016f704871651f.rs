// draft2020-12/type.json::multiple types can be specified in an array
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for Root {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for Root {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
