// draft7/allOf.json::allOf with base schema
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    pub bar: i64,
    pub baz: (),
    pub foo: ::std::string::String,
}
