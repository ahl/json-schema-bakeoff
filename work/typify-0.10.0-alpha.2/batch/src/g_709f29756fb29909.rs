///`A`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct A(pub ::serde_json::Value);
impl ::std::ops::Deref for A {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<A> for ::serde_json::Value {
    fn from(value: A) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for A {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prop1: ::std::option::Option<::std::string::String>,
}

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
