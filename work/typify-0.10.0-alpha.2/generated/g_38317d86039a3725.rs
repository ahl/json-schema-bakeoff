// draft2020-12/ref.json::$id with file URI still resolves pointers - *nix
// root type: Foo

///`Foo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Foo(pub f64);
impl ::std::ops::Deref for Foo {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Foo> for f64 {
    fn from(value: Foo) -> Self {
        value.0
    }
}
impl ::std::convert::From<f64> for Foo {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for Foo {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
