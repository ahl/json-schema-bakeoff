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

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
