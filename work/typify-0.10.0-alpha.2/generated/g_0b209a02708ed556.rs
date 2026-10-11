// draft2019-09/ref.json::naive replacement of $ref with its destination is not correct
// root type: Root

///`AString`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct AString(pub ::std::string::String);
impl ::std::ops::Deref for AString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AString> for ::std::string::String {
    fn from(value: AString) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AString {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for AString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for AString {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`Root`
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Root(::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for Root {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Root>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: Root) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<
    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
> for Root {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            [
                (
                    "$ref".to_string(),
                    ::serde_json::from_str::<::serde_json::Value>("\"#/$defs/a_string\"")
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
impl<'de> ::serde::Deserialize<'de> for Root {
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
