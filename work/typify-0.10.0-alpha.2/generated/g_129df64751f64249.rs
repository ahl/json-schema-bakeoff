// draft2019-09/oneOf.json::oneOf with empty schema
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(f64),
    Variant1(::serde_json::Value),
}
impl ::std::convert::From<f64> for Root {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::serde_json::Value> for Root {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant1(value)
    }
}
