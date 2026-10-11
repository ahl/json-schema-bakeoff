// draft2020-12/oneOf.json::oneOf with required
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0 { bar: ::serde_json::Value, foo: ::serde_json::Value },
    Variant1 { baz: ::serde_json::Value, foo: ::serde_json::Value },
}
