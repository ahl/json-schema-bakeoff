// draft2019-09/ref.json::property named $ref, containing an actual $ref
// root type: Root

///`IsString`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct IsString(pub ::std::string::String);
impl ::std::ops::Deref for IsString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IsString> for ::std::string::String {
    fn from(value: IsString) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for IsString {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for IsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for IsString {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(rename = "$ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<IsString>,
}
