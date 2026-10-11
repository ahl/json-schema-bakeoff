// draft7/not.json::forbid everything with boolean schema true
// root type: Root

///`Root`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(deny_unknown_fields)]
pub enum Root {}
