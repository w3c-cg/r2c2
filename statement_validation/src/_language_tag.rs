use std::{borrow::Cow, sync::LazyLock};

use r2c2_statement::LangTag;
use regex::Regex;

/// Extension trait for [`LangTag`] providing validation methods.
pub trait LangTagValidation<'a> {
    /// Return a new [`LangTag`] if the argument is a valid IRI, otherwise None.
    #[allow(clippy::new_ret_no_self)]
    fn new(txt: impl Into<Cow<'a, str>>) -> Option<LangTag<'a>>;

    /// In debug mode, panic if this [`LangTag`] is not valid.
    /// In release mode, does nothing.
    ///
    /// Can be useful after a [`new_unchecked`](LangTag::new_unchecked)
    fn debug_assert_is_valid(&self);
}

impl<'a> LangTagValidation<'a> for LangTag<'a> {
    fn new(txt: impl Into<Cow<'a, str>>) -> Option<Self> {
        let inner = txt.into();
        TAG_REGEX
            .is_match(&inner)
            .then_some(LangTag::new_unchecked(inner))
    }

    #[inline]
    fn debug_assert_is_valid(&self) {
        debug_assert!(TAG_REGEX.is_match(self.as_ref()))
    }
}

pub(crate) static TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(TAG_REGEX_SRC).unwrap());

/// Match a valid BCP47 language tag
pub static TAG_REGEX_SRC: &str = r"(?xi-u)^
(
  (?:
    (?: #language
      (?:
        [A-Z]{2,3}
        (?: #extlang
          (?:
            -[A-Z]{3}
          ){0,3}
        )
      )
    |
      [A-Z]{4,8}
    )
    (?: #script
      -[A-Z]{4}
    )?
    (?: #region
      -
      (?:
        [A-Z]{2}
      |
        [0-9]{3}
      )
    )?
    (?: #variant
      -
      (?:
        [A-Z0-9]{5,8}
      |
        [0-9][A-Z0-9]{3}
      )
    )*
    (?: #extension
      -[0-9A-WY-Z]
      (?:
        -[A-Z0-9]{2,8}
      )+
    )*
    (?: #privateUse
      -X
      (?:
        -[A-Z0-9]{1,8}
      )+
    )?
  )
|
  (?: #privateUse
    X
    (?:
      -[A-Z0-9]{1,8}
    )+
  )
|
  (?: #grandfathered
    en-GB-oed|i-ami|i-bnn|i-default|i-enochian|i-hak|i-klingon|i-lux|i-mingo|i-navajo|i-pwn|i-tao|i-tay|i-tsu|sgn-BE-FR|sgn-BE-NL|sgn-CH-DE
    # NB regular grandfathered tags are not included,
    # as they will be matched by the normal case
  )
)$";

#[cfg(test)]
mod test {
    use std::iter::once;

    use super::*;

    #[test]
    fn regex_valid() {
        for mut tag in valid_tags() {
            assert!(TAG_REGEX.is_match(&tag), "{tag}");
            tag.make_ascii_uppercase();
            assert!(TAG_REGEX.is_match(&tag), "{tag}");
        }
        for mut txt in private_uses(3) {
            let tag = &txt[1..];
            assert!(TAG_REGEX.is_match(tag), "{tag}");
            txt.make_ascii_uppercase();
            let tag = &txt[1..];
            assert!(TAG_REGEX.is_match(tag), "{tag}");
        }
        for tag in GRANDFATHERED_TAGS {
            assert!(TAG_REGEX.is_match(tag), "{tag}");
            assert!(TAG_REGEX.is_match(&tag.to_ascii_uppercase()), "{tag}");
            assert!(TAG_REGEX.is_match(&tag.to_ascii_lowercase()), "{tag}");
        }
    }

    #[test]
    fn regex_invalid() {
        for tag in valid_tags() {
            for invalid_suffix in ["a@", "abcdefghi"] {
                let txt = format!("{tag}-{invalid_suffix}");
                assert!(!TAG_REGEX.is_match(&txt), "{txt}");
            }
        }
        for txt in INVALID_TAGS {
            assert!(!TAG_REGEX.is_match(txt), "{txt}");
        }
    }

    // below are utility functions used to generate valid (and invalid) tags for testing

    fn valid_tags() -> impl Iterator<Item = String> {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for language in languages() {
                for script in once("").chain(scripts()) {
                    for region in once("").chain(regions()) {
                        for variant in once("".to_string()).chain(variants(1)) {
                            for extension in once("".to_string()).chain(extensions(1)) {
                                for private_use in once("".to_string()).chain(private_uses(1)) {
                                    let tag = format!(
                                        "{language}{script}{region}{variant}{extension}{private_use}"
                                    );
                                    tx.send(tag).unwrap();
                                }
                            }
                        }
                    }
                }
            }
            for variant in variants(2) {
                let tag = format!("en{variant}");
                tx.send(tag).unwrap();
            }
            for extension in extensions(2) {
                let tag = format!("en{extension}");
                tx.send(tag).unwrap();
            }
            for private_use in private_uses(2) {
                let tag = format!("en{private_use}");
                tx.send(tag).unwrap();
            }
        });
        rx.into_iter()
    }

    fn languages() -> impl Iterator<Item = String> {
        ["en", "eng"]
            .into_iter()
            .flat_map(|language| langexts().map(move |exts| format!("{language}{exts}")))
            .chain(["dial", "diale", "dialec", "dialect", "dialects"].map(Into::into))
    }

    fn langexts() -> impl Iterator<Item = &'static str> {
        ["", "-ext", "-ext-ext", "-ext-ext-ext"].into_iter()
    }

    fn scripts() -> impl Iterator<Item = &'static str> {
        ["-latn"].into_iter()
    }

    fn regions() -> impl Iterator<Item = &'static str> {
        ["-uk", "-826"].into_iter()
    }
    fn variants(max: u8) -> impl Iterator<Item = String> {
        debug_assert!(max >= 1);
        (1..=max).flat_map(variant_parts)
    }

    fn variant_parts(n: u8) -> Box<dyn Iterator<Item = String>> {
        match n {
            0 => Box::new(once("".to_string())),
            n => Box::new(variant_parts(n - 1).flat_map(|prefix| {
                ["varia", "variaa", "variant", "variants", "0var"]
                    .map(move |suffix| format!("{prefix}-{suffix}"))
            })),
        }
    }

    fn extensions(max: u8) -> impl Iterator<Item = String> {
        debug_assert!(max >= 1);
        (1..=max).flat_map(move |i| extension_parts(i, max))
    }

    fn extension_parts(n: u8, max: u8) -> Box<dyn Iterator<Item = String>> {
        match n {
            0 => Box::new(once("".to_string())),
            n => Box::new(extension_parts(n - 1, max).flat_map(move |prefix| {
                (1..=max)
                    .flat_map(extension_part_parts)
                    .map(move |suffix| format!("{prefix}-{suffix}"))
            })),
        }
    }

    fn extension_part_parts(n: u8) -> Box<dyn Iterator<Item = String>> {
        match n {
            0 => Box::new(["a", "1"].into_iter().map(ToString::to_string)),
            n => Box::new(extension_part_parts(n - 1).flat_map(|prefix| {
                [
                    "ab", "abc", "abcd", "abcde", "abcdefg", "abcdefgh", "12", "123", "1234",
                    "12345", "1234567", "12345678", "1b", "1b3", "1b3d", "1b3d5", "1b3d5f7",
                    "1b3d5f7h",
                ]
                .map(|suffix| format!("{prefix}-{suffix}"))
            })),
        }
    }

    fn private_uses(max: u8) -> impl Iterator<Item = String> {
        debug_assert!(max >= 1);
        (1..=max).flat_map(private_use_parts)
    }

    fn private_use_parts(n: u8) -> Box<dyn Iterator<Item = String>> {
        match n {
            0 => Box::new(once("-x".to_string())),
            n => Box::new(private_use_parts(n - 1).flat_map(|prefix| {
                [
                    "a", "ab", "abc", "abcd", "abcde", "abcdefg", "abcdefgh", "1", "12", "123",
                    "1234", "12345", "1234567", "12345678", "1b", "1b3", "1b3d", "1b3d5",
                    "1b3d5f7", "1b3d5f7h",
                ]
                .map(|suffix| format!("{prefix}-{suffix}"))
            })),
        }
    }

    /// An array of valid TAGs
    pub const GRANDFATHERED_TAGS: &[&str] = &[
        // irregular grandfathered
        "en-GB-oed",
        "i-ami",
        "i-bnn",
        "i-default",
        "i-enochian",
        "i-hak",
        "i-klingon",
        "i-lux",
        "i-mingo",
        "i-navajo",
        "i-pwn",
        "i-tao",
        "i-tay",
        "i-tsu",
        "sgn-BE-FR",
        "sgn-BE-NL",
        "sgn-CH-DE",
        // regular grandfathered
        "art-lojban",
        "cel-gaulish",
        "no-bok",
        "no-nyn",
        "zh-guoyu",
        "zh-hakka",
        "zh-min",
        "zh-min-nan",
        "zh-xiang",
    ];

    /// An array of valid TAGs
    pub const INVALID_TAGS: &[&str] = &[
        "12",        // invalid characters
        "a@",        // invalid characters
        "a",         // too short
        "abcdefghi", // too long
        // wrong ordering
        "ab-ab-abc",
        "ab-ab-abcd",
        "ab-123-abc",
        "ab-123-abcd",
        "ab-abcd-abc",
        "ab-1bcd-ab",
        "ab-1bcd-abc",
        "ab-1bcd-123",
        "ab-1bcd-abcd",
        "ab-abcde-ab",
        "ab-abcde-abc",
        "ab-abcde-123",
        "ab-abcde-abcd",
        "ab-a-b",
        "abcd-abc",
    ];
}
