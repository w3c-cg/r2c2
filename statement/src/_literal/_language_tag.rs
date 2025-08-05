use std::borrow::Cow;

/// Wrapper around a [`Cow<str>`] guaranteeing that the underlying text satisfies [BCP47].
///
/// NB: This type checks that the structure of the tag complies with the grammar,
/// but does *not* check that each component is a valid code
/// (i.e. ISO 639 for 2-3 characters language tag, or ISO 15924 for the script)
///
/// [BCP47]: https://datatracker.ietf.org/doc/bcp47/
#[derive(Clone, Debug, Eq, Ord)]
pub struct LangTag<'a>(Cow<'a, str>);

impl<'a> LangTag<'a> {
    /// Return a new [`LangTag`], assuming the argument is a valid language tag.
    pub fn new_unchecked(txt: impl Into<Cow<'a, str>>) -> Self {
        LangTag(txt.into())
    }

    /// Return the inner [`Cow<str>`](Cow).
    pub fn unwrap(self) -> Cow<'a, str> {
        self.0
    }

    /// Apply a function to the inner txt, assuming the result of the function is still a valid language tag.
    pub fn unchecked_map(self, mut f: impl FnMut(Cow<'a, str>) -> Cow<'a, str>) -> Self {
        Self(f(self.0))
    }

    /// Borrow this [`LangTag`] as another [`LangTag`].
    pub fn borrowed(&self) -> LangTag<'_> {
        LangTag::new_unchecked(self.0.as_ref())
    }
}

impl std::borrow::Borrow<str> for LangTag<'_> {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::AsRef<str> for LangTag<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::ops::Deref for LangTag<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::hash::Hash for LangTag<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().to_ascii_lowercase().hash(state)
    }
}

impl std::cmp::PartialEq for LangTag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
    }
}

impl std::cmp::PartialEq<&str> for LangTag<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other)
    }
}

impl std::cmp::PartialEq<LangTag<'_>> for &str {
    fn eq(&self, other: &LangTag) -> bool {
        self.eq_ignore_ascii_case(other.0.as_ref())
    }
}

impl std::cmp::PartialOrd for LangTag<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.0
                .to_ascii_lowercase()
                .cmp(&other.0.to_ascii_lowercase()),
        )
    }
}

impl std::cmp::PartialOrd<&str> for LangTag<'_> {
    fn partial_cmp(&self, other: &&'_ str) -> Option<std::cmp::Ordering> {
        Some(self.0.to_ascii_lowercase().cmp(&other.to_ascii_lowercase()))
    }
}

impl std::cmp::PartialOrd<LangTag<'_>> for &str {
    fn partial_cmp(&self, other: &LangTag<'_>) -> Option<std::cmp::Ordering> {
        Some(self.to_ascii_lowercase().cmp(&other.0.to_ascii_lowercase()))
    }
}

impl std::fmt::Display for LangTag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_str() {
        let ex = "en-GB";
        let tag1 = LangTag::new_unchecked(ex.to_string());
        assert!(tag1.starts_with("en"));
        assert_eq!(tag1, ex);
        assert_eq!(ex, tag1);
        assert!(("en"..="en-GB").contains(&tag1));
    }

    #[test]
    fn borrowed() {
        let ex = "en-GB";
        let tag1 = LangTag::new_unchecked(ex.to_string());
        let tag2 = tag1.borrowed();
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn display() {
        let ex = "en-GB";
        let tag1 = LangTag::new_unchecked(ex.to_string());
        assert_eq!(tag1.to_string(), ex);
    }

    #[test]
    fn case_insensitive() {
        let tag1 = LangTag::new_unchecked("en-GB");
        let tag2 = LangTag::new_unchecked("en-gb");
        assert_eq!(tag1, tag2);
        assert_eq!(tag1, "en-gb");
        assert!(tag1 <= tag2 && tag2 <= tag1);
        assert!("EN" < tag1 && tag1 < "EN-ZZ");
    }
}
