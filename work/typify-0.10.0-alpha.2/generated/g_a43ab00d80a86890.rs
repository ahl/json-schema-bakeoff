// draft7/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop
// root type: Root

///`Int`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Int(pub i64);
impl ::std::ops::Deref for Int {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Int> for i64 {
    fn from(value: Int) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for Int {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for Int {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for Int {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<Int>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, Int>,
}
