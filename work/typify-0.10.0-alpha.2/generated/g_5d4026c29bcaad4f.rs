// draft7/properties.json::properties, patternProperties, additionalProperties interaction
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bar: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub foo: ::std::vec::Vec<::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, i64>,
}
