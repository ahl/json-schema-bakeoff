// draft7/default.json::the default keyword does not do anything if the property is missing
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    #[serde(default = "defaults::root_alpha")]
    pub alpha: f64,
}
impl ::std::default::Default for Root {
    fn default() -> Self {
        Self {
            alpha: defaults::root_alpha(),
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn root_alpha() -> f64 {
        5_f64
    }
}
