use std::borrow::Cow;

use crate::Iri;

/// A trait for [RDF terms] allowed as a [graph name] in an [RDF dataset].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [graph name]: https://www.w3.org/TR/rdf12-concepts/#dfn-graph-name
/// [RDF dataset]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
pub trait GraphName {
    /// Return a [`GraphNameProxy`] representing this graph name.
    ///
    /// [RDF term]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_>;

    /// Return the [kind](GraphNameKind) of this graph name.
    ///
    /// # Implementers
    /// A default implementation is provided for this method, based on [`GraphName::as_graph_name_proxy`].
    /// It may be useful to override it, especially for types where the inner values of [`GraphNameProxy`]
    /// are allocated as owned [`Cow<str>`](std::borrow::Cow) rather than borrowed.
    fn graph_name_kind(&self) -> GraphNameKind {
        match self.as_graph_name_proxy() {
            GraphNameProxy::Iri(_) => GraphNameKind::Iri,
            GraphNameProxy::BlankNode(_) => GraphNameKind::BlankNode,
        }
    }

    /// Return true if this graph name is an IRI.
    fn is_iri(&self) -> bool {
        self.graph_name_kind() == GraphNameKind::Iri
    }

    /// Return true if this graph name is a blank node.
    fn is_blank_node(&self) -> bool {
        self.graph_name_kind() == GraphNameKind::BlankNode
    }

    /// If this graph name is an IRI, return it as b_ an [`Iri`], otherwise `None`.
    fn as_iri(&self) -> Option<Iri<'_>> {
        match self.as_graph_name_proxy() {
            GraphNameProxy::Iri(iri) => Some(iri),
            _ => None,
        }
    }

    /// If this graph name is a blank node, return its internal identifier, otherwise `None`.
    fn as_blank_node(&self) -> Option<Cow<'_, str>> {
        match self.as_graph_name_proxy() {
            GraphNameProxy::BlankNode(bnid) => Some(bnid),
            _ => None,
        }
    }

    /// Whether this graph_name is [ground](https://www.w3.org/TR/rdf12-concepts/#dfn-ground).
    fn ground(&self) -> bool {
        match self.graph_name_kind() {
            GraphNameKind::Iri => true,
            GraphNameKind::BlankNode => false,
        }
    }
}

/// An enum conveying the inner information of a value implementing [`GraphName`].
/// The return type of [`GraphName::as_graph_name_proxy`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GraphNameProxy<'a> {
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

/// An enum representing the different kinds of [RDF terms] that can be [graph name].
/// The return type of [`GraphName::graph_name_kind`].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [graph name]: https://www.w3.org/TR/rdf12-concepts/#dfn-graph_name
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GraphNameKind {
    /// An [IRI](https://www.w3.org/TR/rdf12-concepts/#section-IRIs)
    Iri,
    /// A [blank node](https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node)
    BlankNode,
}

/// Any reference to a [`GraphName`] also trivially implements [`GraphName`]
/// (as all methods of [`GraphName`] apply to `&self` anyway).
impl<T: GraphName> GraphName for &'_ T {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        (*self).as_graph_name_proxy()
    }

    fn graph_name_kind(&self) -> GraphNameKind {
        (*self).graph_name_kind()
    }

    fn ground(&self) -> bool {
        (*self).ground()
    }
}

/// [`GraphNameProxy`] implements the trait [`GraphName`].
/// This has not particular interest for [`GraphNameProxy`]s obtained from another [`GraphName`]-implementing type,
/// via the [`GraphName::as_graph_name_proxy`] method.
///
/// It can be useful, on the other hand, to provide a straightforward implementation of [`GraphName`]
/// (e.g. for testing or prototyping).
impl GraphName for GraphNameProxy<'_> {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        match self {
            GraphNameProxy::Iri(iri) => GraphNameProxy::Iri(iri.borrowed()),
            GraphNameProxy::BlankNode(cow) => GraphNameProxy::BlankNode(Cow::from(cow.as_ref())),
        }
    }
}
