
# RDF Rust Common Crates (R2C2) Community Group

The mission of the RDF Rust Common Crates (<abbr>R2C2</abbr>)
Community Group is to develop a **common API** for working with [RDF] in
[Rust], published as a set of [library crates].
The goal is to improve the interoperability of the RDF ecosystem in Rust.

## `r2c2_statement`

This crate provides traits representing the core notions of [RDF]
(terms and statements), that other crates can implement and/or consume.

## `r2c2_statement_validation`

This crate provides functions checking the validity of data,
with respect to the integrity constrains of the traits in `r2c2_statement`
(e.g. checking that a string is a valid IRI or a valid language tag).

It is separate from `r2c2_statement`,
because some implementations may have their own validation code already.


[library crates]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[RDF]: https://www.w3.org/TR/rdf-primer/
[Rust]: https://www.rust-lang.org/

* [Community Group Homepage](https://www.w3.org/groups/cg/r2c2)
* [Community Group Charter](Charter.md)
* [Community Group Code of Conduct](CODE_OF_CONDUCT.md)
