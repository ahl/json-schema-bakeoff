// draft2020-12/items.json::items with boolean schema (false)
// root type: :: std :: vec :: Vec < RootItem >

///`RootItem`
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
pub enum RootItem {}
