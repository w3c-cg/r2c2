//! I define traits and utility types for describing RDF statements
//! ([triples] and [quads]) as well as their constituent [RDF terms].
//!
//! [triples]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-triple
//! [quads]: https://www.w3.org/TR/rdf12-concepts/#dfn-quad
//! [RDF terms]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
//! [RDF datasets]: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
//!
//! # Features
//! * `poc_impl`: include implementation of the traits defined in this crate
//!   for existing RDF implementations.
//!
//!   As the name implies, this is only a proof of concept implementation.
//!   It is expected that such RDF implementations will eventually implements the traits themselves.
#![deny(missing_docs)]

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

#[cfg(test)]
mod test;
