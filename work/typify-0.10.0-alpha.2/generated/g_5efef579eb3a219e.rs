// draft2019-09/properties.json::properties whose names are Javascript object property names
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub constructor: ::std::option::Option<f64>,
    #[serde(
        rename = "__proto__",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub proto: ::std::option::Option<f64>,
    #[serde(rename = "toString", skip_serializing_if = "::std::option::Option::is_none")]
    pub to_string: ::std::option::Option<RootToString>,
}
///`RootToString`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct RootToString {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub length: ::std::option::Option<::std::string::String>,
}
