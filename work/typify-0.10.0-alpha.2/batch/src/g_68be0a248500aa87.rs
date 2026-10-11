///`Bar`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Bar(pub ::serde_json::Value);
impl ::std::ops::Deref for Bar {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Bar> for ::serde_json::Value {
    fn from(value: Bar) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for Bar {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Bar>(v).map(|_| ()).map_err(|e| e.to_string())
}
