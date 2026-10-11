///`Bool`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Bool(pub ::serde_json::Value);
impl ::std::ops::Deref for Bool {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Bool> for ::serde_json::Value {
    fn from(value: Bool) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for Bool {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Bool>(v).map(|_| ()).map_err(|e| e.to_string())
}
