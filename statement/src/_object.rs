use std::borrow::Cow;

use crate::{Iri, Literal, Triple};

/// A trait for [RDF terms] allowed in the [object] position of an [RDF triple].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [object]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
/// [RDF triple]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
pub trait Object {
    /// The type representing [triple terms] for this implementation of [`Object`]
    ///
    /// [triple term]: https://www.w3.org/TR/rdf12-concepts/#dfn-triple-term
    type Triple<'x>: Triple
    where
        Self: 'x;

    /// Return a [`ObjectProxy`] representing this object.
    fn as_object_proxy(&self) -> ObjectProxy<'_, Self::Triple<'_>>;

    /// Return the [kind](ObjectKind) of this object.
    ///
    /// # Implementers
    /// A default implementation is provided for this method, based on [`Object::as_object_proxy`].
    /// It may be useful to override it, especially for types where the inner values of [`ObjectProxy`]
    /// are allocated as owned [`Cow<str>`](std::borrow::Cow) rather than borrowed.
    fn object_kind(&self) -> ObjectKind {
        match self.as_object_proxy() {
            ObjectProxy::Iri(_) => ObjectKind::Iri,
            ObjectProxy::BlankNode(_) => ObjectKind::BlankNode,
            ObjectProxy::Literal(_) => ObjectKind::Literal,
            ObjectProxy::Triple(_) => ObjectKind::Triple,
        }
    }

    /// Return true if this object is an IRI.
    fn is_iri(&self) -> bool {
        self.object_kind() == ObjectKind::Iri
    }

    /// Return true if this object is a blank node.
    fn is_blank_node(&self) -> bool {
        self.object_kind() == ObjectKind::BlankNode
    }

    /// Return true if this object is a literal
    fn is_literal(&self) -> bool {
        self.object_kind() == ObjectKind::Literal
    }

    /// Return true if this object is a triple term
    fn is_triple(&self) -> bool {
        self.object_kind() == ObjectKind::Triple
    }

    /// If this object is an IRI, return it as an [`Iri`], otherwise `None`.
    fn as_iri(&self) -> Option<Iri<'_>> {
        match self.as_object_proxy() {
            ObjectProxy::Iri(iri) => Some(iri),
            _ => None,
        }
    }

    /// If this object is a blank node, return its internal identifier, otherwise `None`.
    fn as_blank_node(&self) -> Option<Cow<'_, str>> {
        match self.as_object_proxy() {
            ObjectProxy::BlankNode(bnid) => Some(bnid),
            _ => None,
        }
    }

    /// If this object is a literal, return it as an [`Literal`], otherwise `None`.
    fn as_literal(&self) -> Option<Literal<'_>> {
        match self.as_object_proxy() {
            ObjectProxy::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    /// If this object is a triple term, return it as an [`Self::Triple`](Object::Triple), otherwise `None`.
    fn as_triple(&self) -> Option<Self::Triple<'_>> {
        match self.as_object_proxy() {
            ObjectProxy::Triple(tr) => Some(tr),
            _ => None,
        }
    }

    /// Whether this object is [ground](https://www.w3.org/TR/rdf12-concepts/#dfn-ground).
    fn ground(&self) -> bool {
        match self.object_kind() {
            ObjectKind::Iri | ObjectKind::Literal => true,
            ObjectKind::BlankNode => false,
            ObjectKind::Triple => self.as_triple().unwrap().ground(),
        }
    }
}

/// An enum conveying the inner information of a value implementing [`Object`].
/// The return type of [`Object::as_object_proxy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ObjectProxy<'a, T: Triple + 'a> {
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
    /// A [literal](https://www.w3.org/TR/rdf12-concepts/#dfn-literal)
    Literal(Literal<'a>),
    /// A [triple term](https://www.w3.org/TR/rdf12-concepts/#dfn-triple-term)
    Triple(T),
}

/// An enum representing the different kinds of [RDF terms] that can be [object].
/// The return type of [`Object::object_kind`].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [object]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectKind {
    /// An [IRI](https://www.w3.org/TR/rdf12-concepts/#section-IRIs)
    Iri,
    /// A [blank node](https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node)
    BlankNode,
    /// A [literal](https://www.w3.org/TR/rdf12-concepts/#dfn-literal)
    Literal,
    /// A [triple term](https://www.w3.org/TR/rdf12-concepts/#dfn-triple-term)
    Triple,
}

/// Any reference to a [`Object`] also trivially implements [`Object`]
/// (as all methods of [`Object`] apply to `&self` anyway).
impl<T: Object> Object for &'_ T {
    type Triple<'x>
        = T::Triple<'x>
    where
        Self: 'x;

    fn as_object_proxy(&self) -> ObjectProxy<'_, Self::Triple<'_>> {
        (*self).as_object_proxy()
    }

    fn object_kind(&self) -> ObjectKind {
        (*self).object_kind()
    }

    fn ground(&self) -> bool {
        (*self).ground()
    }
}

/// [`ObjectProxy`] implements the trait [`Object`].
/// This has not particular interest for [`ObjectProxy`]s obtained from another [`Object`]-implementing type,
/// via the [`Object::as_object_proxy`] method.
///
/// It can be useful, on the other hand, to provide a straightforward implementation of [`Object`]
/// (e.g. for testing or prototyping).
impl<T: Triple> Object for ObjectProxy<'_, T> {
    type Triple<'x>
        = &'x T
    where
        Self: 'x;

    fn as_object_proxy(&self) -> ObjectProxy<'_, &T> {
        match self {
            ObjectProxy::Iri(iri) => ObjectProxy::Iri(iri.borrowed()),
            ObjectProxy::BlankNode(cow) => ObjectProxy::BlankNode(Cow::from(cow.as_ref())),
            ObjectProxy::Literal(literal) => ObjectProxy::Literal(literal.borrowed()),
            ObjectProxy::Triple(triple) => ObjectProxy::Triple(triple),
        }
    }
}
