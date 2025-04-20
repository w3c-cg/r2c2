use std::borrow::Cow;

use crate::Iri;

/// A trait for [RDF terms] allowed in the [subject] position of an [RDF triple].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [subject]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
/// [RDF triple]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
pub trait Subject {
    /// Return a [`SubjectProxy`] representing this subject.
    fn as_subject_proxy(&self) -> SubjectProxy<'_>;

    /// Return the [kind](SubjectKind) of this subject.
    ///
    /// # Implementers
    /// A default implementation is provided for this method, based on [`Subject::as_subject_proxy`].
    /// It may be useful to override it, especially for types where the inner values of [`SubjectProxy`]
    /// are allocated as owned [`Cow<str>`](std::borrow::Cow) rather than borrowed.
    fn subject_kind(&self) -> SubjectKind {
        match self.as_subject_proxy() {
            SubjectProxy::Iri(_) => SubjectKind::Iri,
            SubjectProxy::BlankNode(_) => SubjectKind::BlankNode,
        }
    }

    /// Whether this subject is [ground](https://https://www.w3.org/TR/rdf12-concepts/#dfn-ground).
    fn ground(&self) -> bool {
        match self.subject_kind() {
            SubjectKind::Iri => true,
            SubjectKind::BlankNode => false,
        }
    }
}

/// An enum conveying the inner information of a value implementing [`Subject`].
/// The return type of [`Subject::as_subject_proxy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SubjectProxy<'a> {
    /// An [IRI](https://www.w3.org/TR/rdf12-concepts/#section-IRIs)
    Iri(Iri<'a>),
    /// A [blank node](https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node)
    ///
    /// The inner value is an internal [blank node identifier](https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node-identifier).
    /// This identifier is not part of RDF's abstract syntax, and only *locally* identifies the blank node.A
    ///
    /// Note that this API does not impose any constraint on blank node identifiers,
    /// but concrete syntax usually do, so serializer may alter these identifiers.
    BlankNode(Cow<'a, str>),
}

/// An enum representing the different kinds of [RDF terms] that can be [subject].
/// The return type of [`Subject::subject_kind`].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [subject]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SubjectKind {
    /// An [IRI](https://www.w3.org/TR/rdf12-concepts/#section-IRIs)
    Iri,
    /// A [blank node](https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node)
    BlankNode,
}

/// Any reference to a [`Subject`] also trivially implements [`Subject`]
/// (as all methods of [`Subject`] apply to `&self` anyway).
impl<T: Subject> Subject for &'_ T {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        (*self).as_subject_proxy()
    }

    fn subject_kind(&self) -> SubjectKind {
        (*self).subject_kind()
    }

    fn ground(&self) -> bool {
        (*self).ground()
    }
}

/// [`SubjectProxy`] implements the trait [`Subject`].
/// This has not particular interest for [`SubjectProxy`]s obtained from another [`Subject`]-implementing type,
/// via the [`Subject::as_subject_proxy`] method.
///
/// It can be useful, on the other hand, to provide a straightforward implementation of [`Subject`]
/// (e.g. for testing or prototyping).
impl Subject for SubjectProxy<'_> {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        match self {
            SubjectProxy::Iri(iri) => SubjectProxy::Iri(iri.borrowed()),
            SubjectProxy::BlankNode(cow) => SubjectProxy::BlankNode(Cow::from(cow.as_ref())),
        }
    }
}
