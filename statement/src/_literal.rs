mod _language_tag;

pub use _language_tag::*;

use crate::{AnyStr, Iri};

/// The different possible value for literals' [base direction].
///
/// [base direction]: https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseDir {
    #[default]
    /// The [base direction] `ltr` (left to right)
    ///
    /// [base direction]: https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction
    Ltr,
    /// The [base direction] `rtl` (right to left)
    ///
    /// [base direction]: https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction
    Rtl,
}

/// An enum conveying the inner information of a value representing a [literal].
/// The return type of [`Object::as_literal`](crate::Object::as_literal).
///
/// [literal]: https://www.w3.org/TR/rdf12-concepts/#dfn-literal
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LiteralProxy<'a> {
    /// A literal with a specified datatype.
    Typed(AnyStr<'a>, Iri<'a>),
    /// A [language tagged string](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tagged-string),
    /// or a [directional language tagged string](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tagged-string),
    /// depending on the presence of a [`BaseDir`] in the third component.
    LanguageString(AnyStr<'a>, LangTag<'a>, Option<BaseDir>),
}

impl LiteralProxy<'_> {
    /// Borrow this [`LiteralProxy`] as another [`LiteralProxy`].
    pub fn borrowed(&self) -> LiteralProxy<'_> {
        match self {
            LiteralProxy::Typed(lex, iri) => LiteralProxy::Typed(lex.borrowed(), iri.borrowed()),
            LiteralProxy::LanguageString(lex, lang_tag, base_dir) => {
                LiteralProxy::LanguageString(lex.borrowed(), lang_tag.borrowed(), *base_dir)
            }
        }
    }

    /// [lexical form](https://www.w3.org/TR/rdf12-concepts/#dfn-lexical-form) of this literal
    pub fn lexical_form(&self) -> AnyStr<'_> {
        match self {
            LiteralProxy::Typed(lex, ..) => lex,
            LiteralProxy::LanguageString(lex, ..) => lex,
        }
        .borrowed()
    }

    /// [datatype IRI](https://www.w3.org/TR/rdf12-concepts/#dfn-datatype-iri) of this literal
    pub fn datatype_iri(&self) -> Iri<'_> {
        match self {
            LiteralProxy::Typed(_, iri) => iri.borrowed(),
            LiteralProxy::LanguageString(_, _, None) => Iri::new_unchecked(RDF_LANG_STRING),
            LiteralProxy::LanguageString(_, _, Some(_)) => Iri::new_unchecked(RDF_DIR_LANG_STRING),
        }
    }

    /// [language tag](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tag) of this literal, if any
    pub fn language_tag(&self) -> Option<LangTag<'_>> {
        if let LiteralProxy::LanguageString(_, tag, _) = self {
            Some(tag.borrowed())
        } else {
            None
        }
    }

    /// [base direction](https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction) of this literal, if any
    pub fn base_direction(&self) -> Option<BaseDir> {
        if let LiteralProxy::LanguageString(_, _, Some(dir)) = self {
            Some(*dir)
        } else {
            None
        }
    }
}

static RDF_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
static RDF_DIR_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString";
