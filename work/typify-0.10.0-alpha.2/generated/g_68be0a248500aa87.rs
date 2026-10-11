// draft2020-12/unevaluatedItems.json::unevaluatedItems before $ref
// root type: Bar

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
