// draft7/ref.json::ref overrides any sibling keywords
// root type: Root

///`Reffed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Reffed(pub ::std::vec::Vec<::serde_json::Value>);
impl ::std::ops::Deref for Reffed {
    type Target = ::std::vec::Vec<::serde_json::Value>;
    fn deref(&self) -> &::std::vec::Vec<::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Reffed> for ::std::vec::Vec<::serde_json::Value> {
    fn from(value: Reffed) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for Reffed {
    fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
        Self(value)
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub foo: ::std::vec::Vec<::serde_json::Value>,
}
