// draft6/optional/id.json::non-schema object containing a plain-name $id property
// root type: Root

///`ConstNotAnchor`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ConstNotAnchor(pub ::serde_json::Value);
impl ::std::ops::Deref for ConstNotAnchor {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<ConstNotAnchor> for ::serde_json::Value {
    fn from(value: ConstNotAnchor) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for ConstNotAnchor {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Root`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Root {
    Variant0(::serde_json::Value),
    Variant1(RootVariant1),
}
impl ::std::convert::From<::serde_json::Value> for Root {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<RootVariant1> for Root {
    fn from(value: RootVariant1) -> Self {
        Self::Variant1(value)
    }
}
///`RootVariant1`
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RootVariant1(::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for RootVariant1 {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RootVariant1>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: RootVariant1) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<
    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
> for RootVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            [
                (
                    "$id".to_string(),
                    ::serde_json::from_str::<
                        ::serde_json::Value,
                    >("\"#not_a_real_anchor\"")
                        .expect("invalid default provided"),
                ),
            ]
                .into_iter()
                .collect(),
        ]
            .contains(&value)
        {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RootVariant1 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(
                <::serde_json::Map<
                    ::std::string::String,
                    ::serde_json::Value,
                >>::deserialize(deserializer)?,
            )
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
