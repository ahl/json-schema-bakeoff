// draft2020-12/items.json::items and subitems
// root type: :: std :: vec :: Vec < RootItem >

///`Item`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Item(pub ::std::vec::Vec<ItemItem>);
impl ::std::ops::Deref for Item {
    type Target = ::std::vec::Vec<ItemItem>;
    fn deref(&self) -> &::std::vec::Vec<ItemItem> {
        &self.0
    }
}
impl ::std::convert::From<Item> for ::std::vec::Vec<ItemItem> {
    fn from(value: Item) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ItemItem>> for Item {
    fn from(value: ::std::vec::Vec<ItemItem>) -> Self {
        Self(value)
    }
}
///`ItemItem`
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
pub enum ItemItem {}
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
///`SubItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SubItem {
    pub foo: ::serde_json::Value,
}
