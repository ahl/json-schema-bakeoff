// draft2020-12/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false
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
