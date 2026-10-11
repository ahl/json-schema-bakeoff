// draft2020-12/required.json::required properties whose names are Javascript object property names
// root type: Root

///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Root {
    pub constructor: ::serde_json::Value,
    #[serde(rename = "__proto__")]
    pub proto: ::serde_json::Value,
    #[serde(rename = "toString")]
    pub to_string: ::serde_json::Value,
}
