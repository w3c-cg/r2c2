use std::borrow::Cow;

/// Wrapper around a [`Cow<str>`] signaling that it complies with [RFC3987],
/// i.e. it is a valid IRI.
///
/// ## Contract
/// * Consumers of [`Iri`]s can safely assume that the underlying text is a valid IRI.
/// * Producers of [`Iri`]s are responsible for ensuring that constraint.
///
/// [RFC3987]: https://datatracker.ietf.org/doc/rfc3987/
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Iri<'a>(Cow<'a, str>);

impl<'a> Iri<'a> {
    /// Return a new [`Iri`], assuming the argument is a valid IRI.
    ///
    /// ## Precondition
    /// It is the responsibility of the caller to ensure that `txt` is a valid IRI
    pub fn new_unchecked(txt: impl Into<Cow<'a, str>>) -> Self {
        Iri(txt.into())
    }

    /// Return the inner [`Cow<str>`](Cow).
    pub fn unwrap(self) -> Cow<'a, str> {
        self.0
    }

    /// Apply a function to the inner text, assuming the result is still a valid IRI.
    ///
    /// ## Precondition
    /// It is the responsibility of the caller to ensure that `f`
    /// produces a valid IRI when its argument is a valid IRI.
    pub fn map_unchecked(self, mut f: impl FnMut(Cow<'a, str>) -> Cow<'a, str>) -> Self {
        Self(f(self.0))
    }

    /// Borrow this [`Iri`] as another [`Iri`].
    pub fn borrowed(&self) -> Iri<'_> {
        Iri::new_unchecked(self.as_ref())
    }
}

impl std::borrow::Borrow<str> for Iri<'_> {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::AsRef<str> for Iri<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::ops::Deref for Iri<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::cmp::PartialEq<&str> for Iri<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_ref() == *other
    }
}

impl std::cmp::PartialEq<Iri<'_>> for &str {
    fn eq(&self, other: &Iri) -> bool {
        *self == other.0.as_ref()
    }
}

impl std::cmp::PartialOrd<&str> for Iri<'_> {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        Some(self.0.as_ref().cmp(other))
    }
}

impl std::cmp::PartialOrd<Iri<'_>> for &str {
    fn partial_cmp(&self, other: &Iri<'_>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other.0.as_ref()))
    }
}

impl std::fmt::Display for Iri<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_str() {
        let ex = "http://example.org/foo/bar";
        let iri1 = Iri::new_unchecked(ex.to_string());
        assert!(iri1.starts_with("http:"));
        assert_eq!(iri1, ex);
        assert_eq!(ex, iri1);
        assert!("http:" < iri1 && iri1 < "i");
    }

    #[test]
    fn borrowed() {
        let ex = "http://example.org/foo/bar";
        let iri1 = Iri::new_unchecked(ex.to_string());
        let iri2 = iri1.borrowed();
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn display() {
        let ex = "http://example.org/foo/bar";
        let iri1 = Iri::new_unchecked(ex.to_string());
        assert_eq!(iri1.to_string(), format!("<{ex}>"));
    }
}
