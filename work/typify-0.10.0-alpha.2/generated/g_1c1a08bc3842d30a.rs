// draft2019-09/allOf.json::allOf
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    pub bar: i64,
    pub foo: ::std::string::String,
}
