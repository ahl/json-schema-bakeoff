// draft2020-12/unevaluatedItems.json::unevaluatedItems with oneOf
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(::serde_json::Value),
    Variant1(::serde_json::Value),
}
