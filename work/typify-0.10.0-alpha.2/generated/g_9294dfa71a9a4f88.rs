// draft2020-12/ref.json::empty tokens in $ref json-pointer
// root type: X

///`X`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct X(pub ::serde_json::Value);
impl ::std::ops::Deref for X {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<X> for ::serde_json::Value {
    fn from(value: X) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for X {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
