// draft2019-09/unevaluatedProperties.json::Evaluated properties collection needs to consider instance location
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
}
