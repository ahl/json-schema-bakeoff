///`Root`
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Root(::std::vec::Vec<::serde_json::Value>);
impl ::std::ops::Deref for Root {
    type Target = ::std::vec::Vec<::serde_json::Value>;
    fn deref(&self) -> &::std::vec::Vec<::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Root> for ::std::vec::Vec<::serde_json::Value> {
    fn from(value: Root) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<::std::vec::Vec<::serde_json::Value>> for Root {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::vec::Vec<::serde_json::Value>,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            vec![
                ::serde_json::from_str:: < ::serde_json::Value > ("0")
                .expect("invalid default provided")
            ],
        ]
            .contains(&value)
        {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for Root {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(
                <::std::vec::Vec<::serde_json::Value>>::deserialize(deserializer)?,
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

pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<Root>(v).map(|_| ()).map_err(|e| e.to_string())
}
