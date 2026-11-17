//! I define traits types for describing RDF statements
//! ([triples] and [quads]) as well as their constituent [RDF terms]:
//! [`Triple`], [`Quad`], [`Subject`], [`Predicate`], [`Object`], and [`GraphName`].
//!
//! I also define utility types that are used by those traits, including:
//! * [`AnyStr`]: a wrapper for usual "string" types
//! * [`Iri`], [`LangTag`]: wrappers around [`AnyStr`]s constraining their content;
//! * [`BaseDir`]: a simple enum for representing [base direction]
//! * [`SubjectKind`], [`ObjectKind`], [`GraphNameKind`]: simple enums representing the kinds of [RDF terms]: [IRI], [blank node], [literal] and [triple term]
//! * [`SubjectProxy`], [`ObjectProxy`], [`GraphNameProxy`], [`Literal`]: complex enums conveying the inner information of the types respectively implementing [`Subject`], [`Object`], [`GraphName`], or representing RDF [literal]s.
//!
//!
//! [triples]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
//! [quads]: https://www.w3.org/TR/rdf12-concepts/#dfn-quad
//! [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
//! [RDF datasets]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
//! [base direction]: https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction
//! [IRI]: https://www.w3.org/TR/rdf12-concepts/#dfn-iri
//! [blank node]: https://www.w3.org/TR/rdf12-concepts/#dfn-blank-node
//! [literal]: https://www.w3.org/TR/rdf12-concepts/#dfn-literal
//! [triple term]: https://www.w3.org/TR/rdf12-concepts/#dfn-triple-term
//!
//! # Features
//! * `poc_impl`: include implementation of the traits defined in this crate
//!   for existing RDF implementations.
//!
//!   As the name implies, this is only a proof of concept implementation.
//!   It is expected that such RDF implementations will eventually implements the traits themselves.
#![deny(missing_docs)]

mod _any_str;
pub use _any_str::*;
mod _iri;
pub use _iri::*;
mod _literal;
pub use _literal::*;

mod _subject;
pub use _subject::*;
mod _predicate;
pub use _predicate::*;
mod _graph_name;
pub use _graph_name::*;
mod _object;
pub use _object::*;

mod _triple;
pub use _triple::*;
mod _quad;
pub use _quad::*;

#[cfg(feature = "poc_impl")]
pub mod impl_oxrdf;
#[cfg(feature = "poc_impl")]
pub mod impl_rdf_types;
