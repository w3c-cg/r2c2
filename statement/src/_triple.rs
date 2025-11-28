use crate::{Object, Predicate, Subject};

/// A trait for [RDF triples].
///
/// [RDF triples]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
pub trait Triple {
    /// The type of [RDF terms] appearing in the [subjects] position,
    /// as returned by [`Triple::subject`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [subjects]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
    type Subject<'x>: Subject
    where
        Self: 'x;
    /// The type of [RDF terms] appearing in the [predicates] position,
    /// as returned by [`Triple::predicate`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [predicates]: https://www.w3.org/TR/rdf12-concepts/#dfn-predicate
    type Predicate<'x>: Predicate
    where
        Self: 'x;
    /// The type of [RDF terms] appearing in the [objects] position,
    /// as returned by [`Triple::object`].
    ///
    /// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
    /// [objects]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
    type Object<'x>: Object
    where
        Self: 'x;

    /// The [subject] of this triple
    ///
    /// [subject]: https://www.w3.org/TR/rdf12-concepts/#dfn-subject
    fn subject(&self) -> Self::Subject<'_>;
    /// The [predicate] of this triple
    ///
    /// [predicate]: https://www.w3.org/TR/rdf12-concepts/#dfn-predicate
    fn predicate(&self) -> Self::Predicate<'_>;
    /// The [object] of this triple
    ///
    /// [object]: https://www.w3.org/TR/rdf12-concepts/#dfn-object
    fn object(&self) -> Self::Object<'_>;

    /// Whether this triple is [ground](https://www.w3.org/TR/rdf12-concepts/#dfn-ground).
    fn ground(&self) -> bool {
        self.subject().ground() && self.object().ground()
    }
}

/// Any reference to a [`Triple`] also trivially implements [`Triple`]
/// (as all methods of [`Triple`] apply to `&self` anyway).
impl<T: Triple> Triple for &'_ T {
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

    fn subject(&self) -> Self::Subject<'_> {
        (*self).subject()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        (*self).predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        (*self).object()
    }
}

/// Any boxed [`Triple`] also trivially implements [`Triple`]
/// (as all methods of [`Triple`] apply to `&self` anyway).
impl<T: Triple> Triple for Box<T> {
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

    fn subject(&self) -> Self::Subject<'_> {
        self.as_ref().subject()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        self.as_ref().predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        self.as_ref().object()
    }
}

/// A utility empty type for indicating that a given implementation does not support triple terms.
pub enum NeverTriple {}

impl Triple for NeverTriple {
    type Subject<'x>
        = crate::SubjectProxy<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = crate::Iri<'x>
    where
        Self: 'x;

    type Object<'x>
        = crate::ObjectProxy<'x, NeverTriple>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        unreachable!()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        unreachable!()
    }

    fn object(&self) -> Self::Object<'_> {
        unreachable!()
    }
}
