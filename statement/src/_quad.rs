use crate::{GraphName, Object, Predicate, Subject};

/// A trait for RDF [quads].
///
/// [quads]: https://www.w3.org/TR/rdf12-concepts/#dfn-quad
pub trait Quad {
    /// The type of [RDF terms] appearing in the [subjects] position,
    /// as returned by [`Quad::subject`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [subjects]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
    type Subject<'x>: Subject
    where
        Self: 'x;
    /// The type of [RDF terms] appearing in the [predicates] position,
    /// as returned by [`Quad::predicate`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [predicates]: https://www.w3.org/TR/rdf12-concepts/#dfn-predicate
    type Predicate<'x>: Predicate
    where
        Self: 'x;
    /// The type of [RDF terms] appearing in the [objects] position,
    /// as returned by [`Quad::object`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [objects]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
    type Object<'x>: Object
    where
        Self: 'x;
    /// The type of [RDF terms] used as [graph name],
    /// as returned by [`Quad::graph_name`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [graph name]: https://www.w3.org/TR/rdf12-concepts/#dfn-graph-name
    type GraphName<'x>: GraphName
    where
        Self: 'x;

    /// The [subject] of this quad
    ///
    /// [subject]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
    fn subject(&self) -> Self::Subject<'_>;
    /// The [predicate] of this quad
    ///
    /// [predicate]: https://www.w3.org/TR/rdf12-concepts/#dfn-predicate
    fn predicate(&self) -> Self::Predicate<'_>;
    /// The [object] of this quad
    ///
    /// [object]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
    fn object(&self) -> Self::Object<'_>;
    /// The [graph name] of this quad, if any.
    /// [RDF triples] belonging to the [default graph] of an [RDF dataset]
    /// have no [graph name].
    ///
    /// [graph name]: https://www.w3.org/TR/rdf12-concepts/#dfn-graph-name
    /// [RDF triples]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
    /// [default graph]: https://www.w3.org/TR/rdf12-concepts/#dfn-default-graph
    /// [RDF dataset]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
    fn graph_name(&self) -> Option<Self::GraphName<'_>>;

    /// Whether this quad is [ground](https://https://www.w3.org/TR/rdf12-concepts/#dfn-ground).
    ///
    /// NB: RDF Concepts does not actually defined the notion of "ground quad",
    /// but this is a natural extension: all terms, including the graph name if present, must be ground.
    fn ground(&self) -> bool {
        self.subject().ground()
            && self.object().ground()
            && self.graph_name().map(|n| n.ground()).unwrap_or(true)
    }
}

/// Any reference to a [`Quad`] also trivially implements [`Quad`]
/// (as all methods of [`Quad`] apply to `&self` anyway).
impl<T: Quad> Quad for &'_ T {
    type Subject<'x>
        = T::Subject<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = T::Predicate<'x>
    where
        Self: 'x;

    type Object<'x>
        = T::Object<'x>
    where
        Self: 'x;

    type GraphName<'x>
        = T::GraphName<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        (*self).subject()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        (*self).predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        (*self).object()
    }

    fn graph_name(&self) -> Option<Self::GraphName<'_>> {
        (*self).graph_name()
    }
}
