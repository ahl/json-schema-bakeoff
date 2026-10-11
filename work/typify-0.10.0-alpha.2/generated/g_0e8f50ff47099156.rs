// draft2020-12/ref.json::property named $ref that is not a reference
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(rename = "$ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<::std::string::String>,
}
