// draft2020-12/unevaluatedProperties.json::property is evaluated in an uncle schema to unevaluatedProperties
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foo: ::std::option::Option<RootFoo>,
}
///`RootFoo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct RootFoo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub faz: ::std::option::Option<::std::string::String>,
}
