// draft2020-12/additionalProperties.json::dependentSchemas with additionalProperties
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo2: ::std::option::Option<::serde_json::Value>,
}
