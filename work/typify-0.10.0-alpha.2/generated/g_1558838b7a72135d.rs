// draft2020-12/dynamicRef.json::$dynamicRef points to a boolean schema
// root type: Root

///`False`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(deny_unknown_fields)]
pub enum False {}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(rename = "false", skip_serializing_if = "::std::option::Option::is_none")]
    pub false_: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "true", skip_serializing_if = "::std::option::Option::is_none")]
    pub true_: ::std::option::Option<::serde_json::Value>,
}
///`True`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct True(pub ::serde_json::Value);
impl ::std::ops::Deref for True {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<True> for ::serde_json::Value {
    fn from(value: True) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for True {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
