// draft2020-12/properties.json::properties with null valued instance properties
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
pub struct Root {
    #[serde(default)]
    pub foo: (),
}
