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

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<X>(v).map(|_| ()).map_err(|e| e.to_string())
}
