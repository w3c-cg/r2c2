use std::{borrow::Cow, sync::LazyLock};

use r2c2_statement::Iri;
use regex::Regex;

/// Extension trait for [`Iri`] providing validation methods.
pub trait IriValidation<'a> {
    /// Return a new [`Iri`] if the argument is a valid IRI, otherwise None.
    #[allow(clippy::new_ret_no_self)]
    fn new(txt: impl Into<Cow<'a, str>>) -> Option<Iri<'a>>;

    /// In debug mode, panic if this [`Iri`] is not valid.
    /// In release mode, does nothing.
    ///
    /// Can be useful after a [`new_unchecked`](Iri::new_unchecked)
    fn debug_assert_is_valid(&self);
}

impl<'a> IriValidation<'a> for Iri<'a> {
    fn new(txt: impl Into<Cow<'a, str>>) -> Option<Self> {
        let inner = txt.into();
        IRI_REGEX
            .is_match(&inner)
            .then_some(Iri::new_unchecked(inner))
    }

    #[inline]
    fn debug_assert_is_valid(&self) {
        debug_assert!(IRI_REGEX.is_match(self.as_ref()))
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
