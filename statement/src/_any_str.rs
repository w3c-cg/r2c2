use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    sync::Arc,
};

/// Wrapper for usual "string" types.
#[derive(Clone, Debug, Eq)]
pub enum AnyStr<'a> {
    /// An immutable reference to [`str`]
    Ref(&'a str),
    /// A [`String`]
    String(String),
    /// An [`Arc`]`<`[`str`]`>`
    Arc(Arc<str>),
}

impl<'a> AnyStr<'a> {
    /// Borrow this [`AnyStr`] as an [`AnyStr::Ref`]
    pub fn borrowed(&self) -> AnyStr<'_> {
        AnyStr::Ref(self.as_ref())
    }

    /// Convert this [`AnyStr`] into a [`AnyStr`]`<'static>
    pub fn into_owned(self) -> AnyStr<'static> {
        match self {
            Self::Ref(inner) => AnyStr::String(inner.to_string()),
            Self::String(inner) => AnyStr::String(inner),
            Self::Arc(inner) => AnyStr::Arc(inner),
        }
    }
}

impl AsRef<str> for AnyStr<'_> {
    fn as_ref(&self) -> &str {
        match self {
            AnyStr::Ref(inner) => inner,
            AnyStr::String(inner) => inner,
            AnyStr::Arc(inner) => inner,
        }
    }
}

impl Borrow<str> for AnyStr<'_> {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl Display for AnyStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'a> From<&'a str> for AnyStr<'a> {
    fn from(value: &'a str) -> Self {
        Self::Ref(value)
    }
}

impl From<Arc<str>> for AnyStr<'_> {
    fn from(value: Arc<str>) -> Self {
        Self::Arc(value)
    }
}

impl From<Box<str>> for AnyStr<'_> {
    fn from(value: Box<str>) -> Self {
        Self::String(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for AnyStr<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(inner) => Self::Ref(inner),
            Cow::Owned(inner) => Self::String(inner),
        }
    }
}

impl From<String> for AnyStr<'_> {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl std::hash::Hash for AnyStr<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl Ord for AnyStr<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialEq for AnyStr<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl PartialEq<str> for AnyStr<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}

impl PartialOrd for AnyStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for AnyStr<'_> {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl std::ops::Deref for AnyStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
