mod _language_tag;
use std::borrow::Cow;

pub use _language_tag::*;

use crate::Iri;

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

/// A utility type representing an RDF [literal].
///
/// [literal]: https://www.w3.org/TR/rdf12-concepts/#dfn-literal
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Literal<'a> {
    /// A literal with a specified datatype.
    Typed(Cow<'a, str>, Iri<'a>),
    /// A [language tagged string](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tagged-string),
    /// or a [directional language tagged string](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tagged-string),
    /// depending on the presence of a [`BaseDir`] in the third component.
    LanguageString(Cow<'a, str>, LangTag<'a>, Option<BaseDir>),
}

impl Literal<'_> {
    /// Borrow this [`Literal`] as another [`Literal`].
    pub fn borrowed(&self) -> Literal<'_> {
        match self {
            Literal::Typed(lex, iri) => Literal::Typed(Cow::from(lex.as_ref()), iri.borrowed()),
            Literal::LanguageString(lex, lang_tag, base_dir) => {
                Literal::LanguageString(Cow::from(lex.as_ref()), lang_tag.borrowed(), *base_dir)
            }
        }
    }

    /// [lexical form](https://www.w3.org/TR/rdf12-concepts/#dfn-lexical-form) of this literal
    pub fn lexical_form(&self) -> Cow<'_, str> {
        let ref_cow = match self {
            Literal::Typed(lex, ..) => lex,
            Literal::LanguageString(lex, ..) => lex,
        };
        Cow::from(ref_cow.as_ref())
    }

    /// [datatype IRI](https://www.w3.org/TR/rdf12-concepts/#dfn-datatype-iri) of this literal
    pub fn datatype_iri(&self) -> Iri<'_> {
        match self {
            Literal::Typed(_, iri) => iri.borrowed(),
            Literal::LanguageString(_, _, None) => Iri::new_unchecked(RDF_LANG_STRING),
            Literal::LanguageString(_, _, Some(_)) => Iri::new_unchecked(RDF_DIR_LANG_STRING),
        }
    }

    /// [language tag](https://www.w3.org/TR/rdf12-concepts/#dfn-language-tag) of this literal, if any
    pub fn language_tag(&self) -> Option<LangTag<'_>> {
        if let Literal::LanguageString(_, tag, _) = self {
            Some(tag.borrowed())
        } else {
            None
        }
    }

    /// [base direction](https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction) of this literal, if any
    pub fn base_direction(&self) -> Option<BaseDir> {
        if let Literal::LanguageString(_, _, Some(dir)) = self {
            Some(*dir)
        } else {
            None
        }
    }
}

static RDF_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
static RDF_DIR_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString";
