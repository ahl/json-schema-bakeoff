// draft2020-12/ref.json::nested refs
// root type: C

///`A`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct A(pub i64);
impl ::std::ops::Deref for A {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<A> for i64 {
    fn from(value: A) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for A {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for A {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
///`B`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct B(pub A);
impl ::std::ops::Deref for B {
    type Target = A;
    fn deref(&self) -> &A {
        &self.0
    }
}
impl ::std::convert::From<B> for A {
    fn from(value: B) -> Self {
        value.0
    }
}
impl ::std::convert::From<A> for B {
    fn from(value: A) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for B {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for B {
    type Err = <A as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
///`C`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct C(pub B);
impl ::std::ops::Deref for C {
    type Target = B;
    fn deref(&self) -> &B {
        &self.0
    }
}
impl ::std::convert::From<C> for B {
    fn from(value: C) -> Self {
        value.0
    }
}
impl ::std::convert::From<B> for C {
    fn from(value: B) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for C {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for C {
    type Err = <B as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
