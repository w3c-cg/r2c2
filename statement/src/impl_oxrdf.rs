//! Proof-of-concept implementation of this crate's traits for [`oxrdf`].
//!
//! Only present with the `poc_impl` feature.
//!
//! This module is developed as if [`oxrdf`] implemented RDF 1.2 completely and strictly,
//! which is not entirely true:
//! - [`oxrdf`] does not support base direction in literals, so it is not complete;
//! - [`oxrdf`] with the [`rdf-star`] feature allows triple terms in the subject position, so it is not strict.
//!
//! This is handled by panic'ing when those situations are encountered.
//!
//! A more future proof way of dealing with this would be:
//! - for incomplete implementations, the conversions *from* R2C2 should use
//!   [`TryFrom`] rather than [`From`] (see for example [`crate::impl_rdf_types`])
//! - for generalized implementations, R2C2 would need to be augmented,
//!   possibly with a GeneralizedTerm trait that would allow fallible conversions to strict term categories.
use crate::*;
use oxrdf as ox;

// oxrdf::Triple as Triple

impl Triple for ox::Triple {
    type Subject<'x>
        = &'x ox::Subject
    where
        Self: 'x;

    type Predicate<'x>
        = &'x ox::NamedNode
    where
        Self: 'x;

    type Object<'x>
        = &'x ox::Term
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        &self.subject
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        &self.predicate
    }

    fn object(&self) -> Self::Object<'_> {
        &self.object
    }
}

/// This function would typically be implemented as a method of oxrdf::Triple in the crate itself.
pub fn from_r2c2_triple<T: Triple>(triple: T) -> ox::Triple {
    ox::Triple::new(
        triple.subject().as_subject_proxy(),
        triple.predicate().as_iri(),
        triple.object().as_object_proxy(),
    )
}

// oxrdf::TripleRef as Triple

impl Triple for ox::TripleRef<'_> {
    type Subject<'x>
        = ox::SubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = ox::NamedNodeRef<'x>
    where
        Self: 'x;

    type Object<'x>
        = ox::TermRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        self.subject
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        self.predicate
    }

    fn object(&self) -> Self::Object<'_> {
        self.object
    }
}

// oxrdf::Quad as Quad

impl Quad for ox::Quad {
    type Subject<'x>
        = &'x ox::Subject
    where
        Self: 'x;

    type Predicate<'x>
        = &'x ox::NamedNode
    where
        Self: 'x;

    type Object<'x>
        = &'x ox::Term
    where
        Self: 'x;

    type GraphName<'x>
        = ox::NamedOrBlankNodeRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        &self.subject
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        &self.predicate
    }

    fn object(&self) -> Self::Object<'_> {
        &self.object
    }

    fn graph_name(&self) -> Option<Self::GraphName<'_>> {
        match self.graph_name.as_ref() {
            ox::GraphNameRef::NamedNode(named_node) => {
                Some(ox::NamedOrBlankNodeRef::NamedNode(named_node))
            }
            ox::GraphNameRef::BlankNode(blank_node) => {
                Some(ox::NamedOrBlankNodeRef::BlankNode(blank_node))
            }
            ox::GraphNameRef::DefaultGraph => None,
        }
    }
}

/// This function would typically be implemented as a method of oxrdf::Quad in the crate itself.
pub fn from_r2c2_quad<T: Quad>(quad: T) -> ox::Quad {
    ox::Quad::new(
        quad.subject().as_subject_proxy(),
        quad.predicate().as_iri(),
        quad.object().as_object_proxy(),
        match quad.graph_name() {
            None => ox::GraphName::DefaultGraph,
            Some(gn) => gn.as_graph_name_proxy().into(),
        },
    )
}

// oxrdf::QuadRef as Quad

impl Quad for ox::QuadRef<'_> {
    type Subject<'x>
        = ox::SubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = ox::NamedNodeRef<'x>
    where
        Self: 'x;

    type Object<'x>
        = ox::TermRef<'x>
    where
        Self: 'x;

    type GraphName<'x>
        = ox::NamedOrBlankNodeRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        self.subject
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        self.predicate
    }

    fn object(&self) -> Self::Object<'_> {
        self.object
    }

    fn graph_name(&self) -> Option<Self::GraphName<'_>> {
        match self.graph_name {
            ox::GraphNameRef::NamedNode(named_node) => {
                Some(ox::NamedOrBlankNodeRef::NamedNode(named_node))
            }
            ox::GraphNameRef::BlankNode(blank_node) => {
                Some(ox::NamedOrBlankNodeRef::BlankNode(blank_node))
            }
            ox::GraphNameRef::DefaultGraph => None,
        }
    }
}

// oxrdf::Subject as Subject

impl Subject for ox::Subject {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        match self {
            ox::Subject::NamedNode(named_node) => SubjectProxy::Iri(named_node.as_iri()),
            ox::Subject::BlankNode(blank_node) => {
                SubjectProxy::BlankNode(blank_node.as_str().into())
            }
            ox::Subject::Triple(_) => {
                panic!()
                // This only exists because we enabled the `rdf-star` feature, in order to emulate RDF 1.2's triple terms.
                // It is assumed that OxRdf will eventually implement (strict) RDF 1.2, and that this panic!() will disappear.
                //
                // In the future we may have traits for types that *extend* RDF,
                // with methods of the form `try_as_subject_proxy`, etc...
            }
        }
    }
}

impl<'a> From<SubjectProxy<'a>> for ox::Subject {
    fn from(value: SubjectProxy<'a>) -> Self {
        match value {
            SubjectProxy::Iri(iri) => ox::NamedNode::from(iri).into(),
            SubjectProxy::BlankNode(bnid) => safe_bnode(bnid).into(),
        }
    }
}

// oxrdf::SubjectRef as Subject

impl Subject for ox::SubjectRef<'_> {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        match self {
            ox::SubjectRef::NamedNode(named_node) => SubjectProxy::Iri(named_node.as_iri()),
            ox::SubjectRef::BlankNode(blank_node) => {
                SubjectProxy::BlankNode(blank_node.as_str().into())
            }
            ox::SubjectRef::Triple(_) => {
                panic!()
                // This only exists because we enabled the `rdf-star` feature, in order to emulate RDF 1.2's triple terms.
                // It is assumed that OxRdf will eventually implement (strict) RDF 1.2, and that this panic!() will disappear.
                //
                // In the future we may have traits for types that *extend* RDF,
                // with methods of the form `try_as_subject_proxy`, etc...
            }
        }
    }
}

// oxrdf::NamedNode as Predicate

impl Predicate for ox::NamedNode {
    fn as_iri(&self) -> Iri<'_> {
        Iri::new_unchecked(self.as_str())
    }
}

impl<'a> From<Iri<'a>> for ox::NamedNode {
    fn from(value: Iri<'a>) -> Self {
        ox::NamedNode::new_unchecked(value.unwrap().into_owned())
    }
}

// oxrdf::NamedNodeRef as Predicate

impl Predicate for ox::NamedNodeRef<'_> {
    fn as_iri(&self) -> Iri<'_> {
        Iri::new_unchecked(self.as_str())
    }
}

// oxrdf::Term as Object

impl Object for ox::Term {
    type Triple<'x>
        = &'x ox::Triple
    where
        Self: 'x;

    fn as_object_proxy(&'_ self) -> ObjectProxy<'_, &'_ ox::Triple> {
        match self {
            ox::Term::NamedNode(named_node) => ObjectProxy::Iri(named_node.as_iri()),
            ox::Term::BlankNode(blank_node) => ObjectProxy::BlankNode(blank_node.as_str().into()),
            ox::Term::Literal(literal) => ObjectProxy::Literal(match literal.as_ref().destruct() {
                (lex, None, None) => Literal::Typed(lex.into(), Iri::new_unchecked(XSD_STRING)),
                (lex, _, Some(tag)) => {
                    Literal::LanguageString(lex.into(), LangTag::new_unchecked(tag), None)
                }
                (lex, Some(dt), _) => Literal::Typed(lex.into(), Iri::new_unchecked(dt.as_str())),
            }),
            ox::Term::Triple(triple) => ObjectProxy::Triple(triple),
        }
    }
}

impl<'a, T: Triple> From<ObjectProxy<'a, T>> for ox::Term {
    fn from(value: ObjectProxy<'a, T>) -> Self {
        match value {
            ObjectProxy::Iri(iri) => ox::NamedNode::from(iri).into(),
            ObjectProxy::BlankNode(bnid) => safe_bnode(bnid).into(),
            ObjectProxy::Literal(literal) => match literal {
                Literal::Typed(lex, iri) => {
                    ox::Literal::new_typed_literal(lex.into_owned(), iri).into()
                }
                Literal::LanguageString(lex, lang_tag, base_dir) => {
                    if base_dir.is_some() {
                        panic!()
                        // Assuming here that oxrdf will eventually support base direction,
                        // this panic!() will go away.
                        //
                        // For a type that is *not* expected to implement all of RDF 1.2,
                        // they should implement TryFrom instead.
                    }
                    ox::Literal::new_language_tagged_literal_unchecked(
                        lex.into_owned(),
                        lang_tag.unwrap().into_owned(),
                    )
                    .into()
                }
            },
            ObjectProxy::Triple(triple) => ox::Term::Triple(Box::new(from_r2c2_triple(triple))),
        }
    }
}

// oxrdf::TermRef as Object

impl Object for ox::TermRef<'_> {
    type Triple<'x>
        = &'x ox::Triple
    where
        Self: 'x;

    fn as_object_proxy(&'_ self) -> ObjectProxy<'_, &'_ ox::Triple> {
        match self {
            ox::TermRef::NamedNode(named_node) => ObjectProxy::Iri(named_node.as_iri()),
            ox::TermRef::BlankNode(blank_node) => {
                ObjectProxy::BlankNode(blank_node.as_str().into())
            }
            ox::TermRef::Literal(literal) => ObjectProxy::Literal(match literal.destruct() {
                (lex, None, None) => Literal::Typed(lex.into(), Iri::new_unchecked(XSD_STRING)),
                (lex, _, Some(tag)) => {
                    Literal::LanguageString(lex.into(), LangTag::new_unchecked(tag), None)
                }
                (lex, Some(dt), _) => Literal::Typed(lex.into(), Iri::new_unchecked(dt.as_str())),
            }),
            ox::TermRef::Triple(triple) => ObjectProxy::Triple(triple),
        }
    }
}

// oxrdf::Subject as GraphName
//
// NB: ox::GraphName can not implement GraphName,
// because it has a variant 'DefaultGraph' which does not correspond to any term kind recognized by r2c2
//
// Note however that, conversely, GraphNameProxy can be converted to an ox::GraphName

impl GraphName for ox::Subject {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        match self {
            ox::Subject::NamedNode(named_node) => GraphNameProxy::Iri(named_node.as_iri()),
            ox::Subject::BlankNode(blank_node) => {
                GraphNameProxy::BlankNode(blank_node.as_str().into())
            }
            ox::Subject::Triple(_) => {
                panic!()
                // This only exists because we enabled the `rdf-star` feature, in order to emulate RDF 1.2's triple terms.
                // It is assumed that OxRdf will eventually implement (strict) RDF 1.2, and that this panic!() will disappear.
                //
                // In the future we may have traits for types that *extend* RDF,
                // with methods of the form `try_as_subject_proxy`, etc...
            }
        }
    }
}

impl<'a> From<GraphNameProxy<'a>> for ox::GraphName {
    fn from(value: GraphNameProxy<'a>) -> Self {
        match value {
            GraphNameProxy::Iri(iri) => ox::NamedNode::from(iri).into(),
            GraphNameProxy::BlankNode(bnid) => safe_bnode(bnid).into(),
        }
    }
}

// oxrdf::NamedOrBlankNodeRef as GraphName
//
// NB: ox::GraphNameRef can not implement GraphName,
// because it has a variant 'DefaultGraph' which does not correspond to any term kind recognized by r2c2

impl GraphName for ox::NamedOrBlankNodeRef<'_> {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        match self {
            ox::NamedOrBlankNodeRef::NamedNode(named_node) => {
                GraphNameProxy::Iri(named_node.as_iri())
            }
            ox::NamedOrBlankNodeRef::BlankNode(blank_node) => {
                GraphNameProxy::BlankNode(blank_node.as_str().into())
            }
        }
    }
}

// utility functions and constants

/// This function converts an R2C2 bnode label into an OxRDF Blank Node,
/// ensuring that bnode labels that are not valid SPARQL bnodeIds are correctly handled
fn safe_bnode(bnid: std::borrow::Cow<str>) -> ox::BlankNode {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut s = DefaultHasher::new();
    bnid.hash(&mut s);
    let h = s.finish();

    ox::BlankNode::new(bnid.into_owned())
        .unwrap_or_else(|_| ox::BlankNode::new_from_unique_id(h as u128))
}

static XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

#[cfg(test)]
mod test_round_trip {
    use super::*;

    #[test]
    fn subject_iri() -> TestResult {
        let s1: ox::Subject = ox::NamedNode::new("https://example.org/ns/alice")?.into();
        let s2: ox::Subject = s1.as_subject_proxy().into();
        assert_eq!(s1, s2);
        let s2: ox::Subject = s1.as_ref().as_subject_proxy().into();
        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn subject_bnode() -> TestResult {
        let s1: ox::Subject = ox::BlankNode::default().into();
        let s2: ox::Subject = s1.as_subject_proxy().into();
        assert_eq!(s1, s2);
        let s2: ox::Subject = s1.as_ref().as_subject_proxy().into();
        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn predicate() -> TestResult {
        let p1 = ox::NamedNode::new("https://example.org/ns/alice")?;
        let p2: ox::NamedNode = p1.as_iri().into();
        assert_eq!(p1, p2);
        let p2: ox::NamedNode = p1.as_ref().as_iri().into();
        assert_eq!(p1, p2);
        Ok(())
    }

    #[test]
    fn object_iri() -> TestResult {
        let o1: ox::Term = ox::NamedNode::new("https://example.org/ns/alice")?.into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_bnode() -> TestResult {
        let o1: ox::Term = ox::BlankNode::default().into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_simple_literal() -> TestResult {
        let o1: ox::Term = ox::Literal::new_simple_literal("⛄").into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_typed_literal() -> TestResult {
        let o1: ox::Term =
            ox::Literal::new_typed_literal("42", ox::NamedNode::new(XSD_INTEGER)?).into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_language_tagged_literal() -> TestResult {
        let o1: ox::Term = ox::Literal::new_language_tagged_literal("chat", "en-Latn-UK")?.into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_triple_term() -> TestResult {
        let subject = ox::BlankNode::default().into();
        let predicate = ox::NamedNode::new("https://example.org/ns/p")?;
        let object = ox::Literal::new_simple_literal("⛄").into();
        let o1: ox::Term = ox::Triple {
            subject,
            predicate,
            object,
        }
        .into();
        let o2: ox::Term = o1.as_object_proxy().into();
        assert_eq!(o1, o2);
        let o2: ox::Term = o1.as_ref().as_object_proxy().into();
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn triple() -> TestResult {
        let subject = ox::BlankNode::default().into();
        let predicate = ox::NamedNode::new("https://example.org/ns/p")?;
        let object = ox::Literal::new_simple_literal("⛄").into();
        let t1 = ox::Triple {
            subject,
            predicate,
            object,
        };
        let t2 = from_r2c2_triple(&t1);
        assert_eq!(t1, t2);
        let t2 = from_r2c2_triple(t1.as_ref());
        assert_eq!(t1, t2);
        Ok(())
    }

    #[test]
    fn quad_default_graph() -> TestResult {
        let subject = ox::BlankNode::default().into();
        let predicate = ox::NamedNode::new("https://example.org/ns/p")?;
        let object = ox::Literal::new_simple_literal("⛄").into();
        let graph_name = ox::GraphName::DefaultGraph;
        let q1 = ox::Quad {
            subject,
            predicate,
            object,
            graph_name,
        };
        let q2 = from_r2c2_quad(&q1);
        assert_eq!(q1, q2);
        let q2 = from_r2c2_quad(q1.as_ref());
        assert_eq!(q1, q2);
        Ok(())
    }

    #[test]
    fn quad_named_graph() -> TestResult {
        let subject = ox::BlankNode::default().into();
        let predicate = ox::NamedNode::new("https://example.org/ns/p")?;
        let object = ox::Literal::new_simple_literal("⛄").into();
        let graph_name = ox::NamedNode::new("https://example.org/")?.into();
        let q1 = ox::Quad {
            subject,
            predicate,
            object,
            graph_name,
        };
        let q2 = from_r2c2_quad(&q1);
        assert_eq!(q1, q2);
        let q2 = from_r2c2_quad(q1.as_ref());
        assert_eq!(q1, q2);
        Ok(())
    }

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    static XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
}
