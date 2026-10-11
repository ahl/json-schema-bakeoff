// draft2020-12/unevaluatedProperties.json::Evaluated properties collection needs to consider instance location with additionalProperties
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub foo: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
