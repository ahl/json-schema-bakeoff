// draft2019-09/recursiveRef.json::$recursiveRef with no $recursiveAnchor in the outer schema resource
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Boolean(bool),
    Object(::std::collections::HashMap<::std::string::String, RootObjectValue>),
}
impl ::std::convert::From<bool> for Root {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, RootObjectValue>,
> for Root {
    fn from(
        value: ::std::collections::HashMap<::std::string::String, RootObjectValue>,
    ) -> Self {
        Self::Object(value)
    }
}
///`RootObjectValue`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RootObjectValue {
    Integer(i64),
    Object(::serde_json::Map<::std::string::String, ::serde_json::Value>),
}
impl ::std::convert::From<i64> for RootObjectValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for RootObjectValue {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self::Object(value)
    }
}
