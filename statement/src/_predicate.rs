use crate::Iri;

/// A trait for [RDF terms] allowed in the [predicate] position of an [RDF triple].
///
/// [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
/// [predicate]: https://www.w3.org/TR/rdf12-concepts/#dfn-predicate
/// [RDF triple]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
pub trait Predicate {
    /// Return the [`Iri`] of this predicate.
    fn as_iri(&self) -> Iri<'_>;
}

/// Any reference to a [`Predicate`] also trivially implements [`Predicate`]
/// (as all methods of [`Predicate`] apply to `&self` anyway).
impl<T: Predicate> Predicate for &'_ T {
    fn as_iri(&self) -> Iri<'_> {
        (*self).as_iri()
    }
}

/// [`Iri`] implements the trait [`Predicate`].
/// This has not particular interest for [`Iri`]s obtained from another [`Predicate`]-implementing type,
/// via the [`Predicate::as_iri`] method.
///
/// It can be useful, on the other hand, to provide a straightforward implementation of [`Predicate`]
/// (e.g. for testing or prototyping).
impl Predicate for Iri<'_> {
    fn as_iri(&self) -> Iri<'_> {
        self.borrowed()
    }
}
