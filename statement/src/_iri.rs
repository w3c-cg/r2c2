use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

/// Wrapper around a [`Cow<str>`] guaranteeing that the underlying text satisfies [RFC3987].
///
/// [RFC3987]: https://datatracker.ietf.org/doc/rfc3987/
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Iri<'a>(Cow<'a, str>);

impl<'a> Iri<'a> {
    /// Return a new [`Iri`] if the argument is a valid IRI, otherwise None.
    pub fn new(txt: impl Into<Cow<'a, str>>) -> Option<Self> {
        let inner = txt.into();
        IRI_REGEX.is_match(&inner).then_some(Iri(inner))
    }

    /// Return a new [`Iri`], assuming the argument is a valid IRI.
    pub fn new_unchecked(txt: impl Into<Cow<'a, str>>) -> Self {
        Iri(txt.into())
    }

    /// Return the inner [`Cow<str>`](Cow).
    pub fn unwrap(self) -> Cow<'a, str> {
        self.0
    }

    /// Apply a function to the inner txt, assuming the result of the function is still a valid IRI.
    pub fn unchecked_map(self, mut f: impl FnMut(Cow<'a, str>) -> Cow<'a, str>) -> Self {
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

pub(crate) static IRI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(IRI_REGEX_SRC).unwrap());

/// Match an absolute IRI reference.
pub static IRI_REGEX_SRC: &str = r"(?x)^
        #scheme
       ( # CAPTURE scheme
        [A-Za-z] [-A-Za-z0-9+.]*
       )
        :
        #ihier_part
        (?: #iauthority + ipath_abempty
          //
         ( # CAPTURE iauthority
          (?: # iuserinfo
            (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:]
          |
            %[0-9a-fA-F]{2}
          )*
          @
          )?
          # ihost
          (?: # ip_literal
             \[
            (?: # ipv6address
              (?:
                (?:[0-9a-fA-F]{1,4}:){6}
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                ::
                (?:[0-9a-fA-F]{1,4}:){5}
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:[0-9a-fA-F]{1,4})?
                ::
                (?:[0-9a-fA-F]{1,4}:){4}
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,1}:[0-9a-fA-F]{1,4})?
                ::
                (?:[0-9a-fA-F]{1,4}:){3}
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,2}:[0-9a-fA-F]{1,4})?
                ::
                (?:[0-9a-fA-F]{1,4}:){2}
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,3}:[0-9a-fA-F]{1,4})?
                ::
                [0-9a-fA-F]{1,4}:
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,4}:[0-9a-fA-F]{1,4})?
                ::
                (?:[0-9a-fA-F]{1,4}:[0-9a-fA-F]{1,4}|(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))(?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3})
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,5}:[0-9a-fA-F]{1,4})?
                ::
                [0-9a-fA-F]{1,4}
              |
                (?:(?:[0-9a-fA-F]{1,4}:){0,6}:[0-9a-fA-F]{1,4})?
                ::
              )
            | # ipvfuture
              v[0-9a-fA-F]+ \. [-A-Za-z0-9._~!$&'()*+,;=:]+
            )
             \]
          | # ipv4address
            (?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5])) (?:\.(?:[0-9]|(?:[1-9][0-9])|(?:1[0-9]{2})|(?:2[0-4][0-9])|(?:25[0-5]))){3}
          | # ireg_name
              (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=]
              | %[0-9a-fA-F]{2}
              )*
          )
          (?:
            :
            [0-9]* # port
          )?
         )
          #ipath_abempty
         ( # CAPTURE ipath_abempty
          (?:
            /
            (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@]
            | %[0-9a-fA-F]{2}
            )*
          )*
         )
        | #ipath_absolute
         ( # CAPTURE ipath_absolute
          /
          (?:
            (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@]
            | %[0-9a-fA-F]{2}
            )*
            (?:
              /
              (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@]
              | %[0-9a-fA-F]{2}
              )*
            )*
          )?
         )
        | #ipath_rootless
         ( # CAPTURE ipath_rootless
          (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@]
          | %[0-9a-fA-F]{2}
          )+
          (?:
            /
            (?: [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@]
            | %[0-9a-fA-F]{2}
            )*
          )*
         )
        )? # optional because of ipath_empty
        (?: # ?iquery
          \?
         ( # CAPTURE iquery
          (?:
            [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@'\u{E000}-\u{F8FF}\u{F0000}-\u{FFFFD}\u{100000}-\u{10FFFD}/?]
            | %[0-9a-fA-F]{2}
          )*
         )
        )?
        (?: # #ifragment
          \#
         ( # CAPTURE ifragment
          (?:
            [-A-Za-z0-9._~\u{A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}\u{10000}-\u{1FFFD}\u{20000}-\u{2FFFD}\u{30000}-\u{3FFFD}\u{40000}-\u{4FFFD}\u{50000}-\u{5FFFD}\u{60000}-\u{6FFFD}\u{70000}-\u{7FFFD}\u{80000}-\u{8FFFD}\u{90000}-\u{9FFFD}\u{A0000}-\u{AFFFD}\u{B0000}-\u{BFFFD}\u{C0000}-\u{CFFFD}\u{D0000}-\u{DFFFD}\u{E1000}-\u{EFFFD}!$&'()*+,;=:@/?]
            | %[0-9a-fA-F]{2}
          )*
         )
        )?
$";

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

    #[test]
    fn regex() {
        for txt in POSITIVE_IRIS {
            assert!(IRI_REGEX.is_match(txt));
        }
        for txt in NEGATIVE_IRIS {
            assert!(!IRI_REGEX.is_match(txt));
        }
    }

    /// An array of valid IRIs
    pub const POSITIVE_IRIS: &[&str] = &[
        "http:",
        "http://example.org",
        "http://127.0.0.1",
        "http://[::]",
        "http://%0D",
        "http://example.org/",
        "http://éxample.org/",
        "http://user:pw@example.org:1234/",
        "http://example.org/foo/bar/baz",
        "http://example.org/foo/bar/",
        "http://example.org/foo/bar/bàz",
        "http://example.org/foo/.././/bar",
        "http://example.org/!$&'()*+,=:@/foo%0D",
        "http://example.org/?abc",
        "http://example.org/?!$&'()*+,=:@/?\u{E000}",
        "http://example.org/#def",
        "http://example.org/?abc#def",
        "tag:abc/def",
        "tag:",
        "http://example.org/#Andr%C3%A9",
        "http://example.org/?Andr%C3%A9",
    ];

    /// An array of invalid IRIs.
    pub const NEGATIVE_IRIS: &[&str] = &[
        // valid IRI references that are not IRIs (relative)
        "foo",
        "..",
        "//example.org",
        "?",
        "#",
        "?#",
        "?Andr%C3%A9#Andr%C3%A9",
        // invalid IRI references
        "http://[/",
        "http://a/[",
        "http://a/]",
        "http://a/|",
        "http://a/ ",
        "http://a/\u{E000}",
        "[",
        "]",
        "|",
        " ",
        "\u{E000}",
    ];
}
