//! Proof-of-concept implementation of this crate's traits for [`rdf_types`].
//!
//! Only present with the `poc_impl` feature.
//!
//! [`rdf_types`] is an implementation of RDF 1.1,
//! which makes it a strict subset of RDF 1.2.
//! Therefore, while [`rdf_types`] types can implement R2C2 traits,
//! conversion from R2C2 is performed using TryFrom.
use crate::*;
use rdf_types as rt;

// rdf_types::LexicalTriple as Triple

impl Triple for rt::LexicalTriple {
    type Subject<'x>
        = rt::LexicalSubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = &'x rt::Iri
    where
        Self: 'x;

    type Object<'x>
        = rt::LexicalObjectRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        self.subject().as_lexical_subject_ref()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        self.predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        self.object().as_lexical_object_ref()
    }
}

/// This function would typically be implemented as a method of rdf_types::Triple in the crate itself.
pub fn try_from_r2c2_triple<T: Triple>(triple: T) -> Result<rt::LexicalTriple, &'static str> {
    Ok(rt::Triple(
        triple.subject().as_subject_proxy().into(),
        triple.predicate().as_iri().into(),
        triple.object().as_object_proxy().try_into()?,
    ))
}

// rdf_types::LexicalTripleRef as Triple

impl Triple for rt::LexicalTripleRef<'_> {
    type Subject<'x>
        = rt::LexicalSubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = &'x rt::Iri
    where
        Self: 'x;

    type Object<'x>
        = rt::LexicalObjectRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        *self.subject()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        *self.predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        *self.object()
    }
}

// rdf_types::LexicalQuad as Quad

impl Quad for rt::LexicalQuad {
    type Subject<'x>
        = rt::LexicalSubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = &'x rt::Iri
    where
        Self: 'x;

    type Object<'x>
        = rt::LexicalObjectRef<'x>
    where
        Self: 'x;

    type GraphName<'x>
        = rt::LexicalGraphLabelRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        self.subject().as_lexical_subject_ref()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        self.predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        self.object().as_lexical_object_ref()
    }

    fn graph_name(&self) -> Option<Self::GraphName<'_>> {
        self.graph().map(|gn| gn.as_graph_label_ref())
    }
}

/// This function would typically be implemented as a method of rdf_types::Quad in the crate itself.
pub fn try_from_r2c2_quad<T: Quad>(quad: T) -> Result<rt::LexicalQuad, &'static str> {
    Ok(rt::Quad(
        quad.subject().as_subject_proxy().into(),
        quad.predicate().as_iri().into(),
        quad.object().as_object_proxy().try_into()?,
        quad.graph_name().map(|gn| gn.as_graph_name_proxy().into()),
    ))
}

// rdf_types::LexicalQuadRef as Quad

impl Quad for rt::LexicalQuadRef<'_> {
    type Subject<'x>
        = rt::LexicalSubjectRef<'x>
    where
        Self: 'x;

    type Predicate<'x>
        = &'x rt::Iri
    where
        Self: 'x;

    type Object<'x>
        = rt::LexicalObjectRef<'x>
    where
        Self: 'x;

    type GraphName<'x>
        = rt::LexicalGraphLabelRef<'x>
    where
        Self: 'x;

    fn subject(&self) -> Self::Subject<'_> {
        *self.subject()
    }

    fn predicate(&self) -> Self::Predicate<'_> {
        *self.predicate()
    }

    fn object(&self) -> Self::Object<'_> {
        *self.object()
    }

    fn graph_name(&self) -> Option<Self::GraphName<'_>> {
        self.graph().copied()
    }
}

// rdf_types::Subject as Subject

impl Subject for rt::Subject {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        match self {
            rt::Id::Blank(bid) => SubjectProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Id::Iri(iri) => SubjectProxy::Iri(Iri::new_unchecked(iri.as_str())),
        }
    }
}

impl<'a> From<SubjectProxy<'a>> for rt::Subject {
    fn from(value: SubjectProxy<'a>) -> Self {
        match value {
            SubjectProxy::Iri(iri) => rt::Subject::Iri(rt::IriBuf::from(iri)),
            SubjectProxy::BlankNode(bnid) => rt::Subject::Blank(safe_bnode(bnid)),
        }
    }
}

// rdf::types::LexicalSubjectRef as Subject

impl Subject for rt::LexicalSubjectRef<'_> {
    fn as_subject_proxy(&self) -> SubjectProxy<'_> {
        match self {
            rt::Id::Blank(bid) => SubjectProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Id::Iri(iri) => SubjectProxy::Iri(Iri::new_unchecked(iri.as_str())),
        }
    }
}

// rdf_types::IriBuf as Predicate

impl Predicate for rt::IriBuf {
    fn as_iri(&self) -> Iri<'_> {
        Iri::new_unchecked(self.as_str())
    }
}

impl<'a> From<Iri<'a>> for rt::IriBuf {
    fn from(value: Iri<'a>) -> Self {
        unsafe {
            // SAFETY: we know that value is a valid IRI
            rt::IriBuf::new_unchecked(value.unwrap().into_owned())
        }
    }
}

// rdf_types::Iri as Predicate

impl Predicate for &rt::Iri {
    fn as_iri(&self) -> Iri<'_> {
        Iri::new_unchecked(self.as_str())
    }
}

// rdf_types::Object as Object

impl Object for rt::Object {
    type Triple<'x>
        = NeverTriple
    where
        Self: 'x;

    fn as_object_proxy(&self) -> ObjectProxy<'_, Self::Triple<'_>> {
        match self {
            rt::Term::Id(rt::Id::Blank(bid)) => ObjectProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Term::Id(rt::Id::Iri(iri)) => ObjectProxy::Iri(Iri::new_unchecked(iri.as_str())),
            rt::Term::Literal(lit) => ObjectProxy::Literal(match &lit.type_ {
                rt::LiteralType::Any(iri) => {
                    Literal::Typed(lit.as_str().into(), Iri::new_unchecked(iri.as_str()))
                }
                rt::LiteralType::LangString(lang_tag_buf) => Literal::LanguageString(
                    lit.as_str().into(),
                    LangTag::new_unchecked(lang_tag_buf.as_str()),
                    None,
                ),
            }),
        }
    }
}

impl<'a, T: Triple> TryFrom<ObjectProxy<'a, T>> for rt::Object {
    type Error = &'static str;

    fn try_from(value: ObjectProxy<'a, T>) -> Result<Self, Self::Error> {
        Ok(match value {
            ObjectProxy::Iri(iri) => rt::Object::Id(rt::Id::Iri(rt::IriBuf::from(iri))),
            ObjectProxy::BlankNode(bnid) => rt::Object::Id(rt::Id::Blank(safe_bnode(bnid))),
            ObjectProxy::Literal(literal) => rt::Object::Literal(match literal {
                Literal::Typed(lex, iri) => rt::Literal::new(
                    lex.into_owned(),
                    rt::LiteralType::Any(unsafe {
                        // SAFETY: iri is known to be a valid IRI
                        rt::IriBuf::new_unchecked(iri.unwrap().into_owned())
                    }),
                ),
                Literal::LanguageString(lex, lang_tag, None) => rt::Literal::new(
                    lex.into_owned(),
                    rt::LiteralType::LangString(unsafe {
                        // SAFETY: lang_tag is known to be a valid language tag
                        langtag::LangTagBuf::new_unchecked(lang_tag.unwrap().into_owned())
                    }),
                ),
                Literal::LanguageString(_, _, Some(_)) => {
                    Err("directional language strings are not supported by the crate rdf_types")?
                }
            }),
            ObjectProxy::Triple(_) => Err("triple-terms are not supported by the crate rdf_types")?,
        })
    }
}

// rdf_types::LexicalObjectRef as Object

impl Object for rt::LexicalObjectRef<'_> {
    type Triple<'x>
        = NeverTriple
    where
        Self: 'x;

    fn as_object_proxy(&self) -> ObjectProxy<'_, Self::Triple<'_>> {
        match self {
            rt::Term::Id(rt::Id::Blank(bid)) => ObjectProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Term::Id(rt::Id::Iri(iri)) => ObjectProxy::Iri(Iri::new_unchecked(iri.as_str())),
            rt::Term::Literal(lit) => ObjectProxy::Literal(match &lit.type_ {
                rt::LiteralType::Any(iri) => {
                    Literal::Typed(lit.as_str().into(), Iri::new_unchecked(iri.as_str()))
                }
                rt::LiteralType::LangString(lang_tag_buf) => Literal::LanguageString(
                    lit.as_str().into(),
                    LangTag::new_unchecked(lang_tag_buf.as_str()),
                    None,
                ),
            }),
        }
    }
}

// rdf_types::GraphLabel as GraphName

impl GraphName for rt::GraphLabel {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        match self {
            rt::Id::Blank(bid) => GraphNameProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Id::Iri(iri) => GraphNameProxy::Iri(Iri::new_unchecked(iri.as_str())),
        }
    }
}

impl<'a> From<GraphNameProxy<'a>> for rt::GraphLabel {
    fn from(value: GraphNameProxy<'a>) -> Self {
        match value {
            GraphNameProxy::Iri(iri) => rt::GraphLabel::Iri(rt::IriBuf::from(iri)),
            GraphNameProxy::BlankNode(bnid) => rt::GraphLabel::Blank(safe_bnode(bnid)),
        }
    }
}

// rdf::types::LexicalGraphLabelRef as GraphName

impl GraphName for rt::LexicalGraphLabelRef<'_> {
    fn as_graph_name_proxy(&self) -> GraphNameProxy<'_> {
        match self {
            rt::Id::Blank(bid) => GraphNameProxy::BlankNode(bid.as_str()[2..].into()),
            rt::Id::Iri(iri) => GraphNameProxy::Iri(Iri::new_unchecked(iri.as_str())),
        }
    }
}

// utility functions

/// This function converts an R2C2 bnode label into an rdf_types Blank Node,
/// ensuring that bnode labels that are not valid SPARQL bnodeIds are correctly handled
fn safe_bnode(bnid: std::borrow::Cow<str>) -> rt::BlankIdBuf {
    rt::BlankIdBuf::new(format!("_:{bnid}")).unwrap_or_else(|err| {
        use std::hash::{DefaultHasher, Hash, Hasher};
        let mut s = DefaultHasher::new();
        err.0.hash(&mut s);
        rt::BlankIdBuf::from_u64(s.finish())
    })
}

#[cfg(test)]
mod test_round_trip {
    use rdf_types::{FromBlankId, FromIri};

    use super::*;

    #[test]
    fn subject_iri() -> TestResult {
        let s1 = rt::Subject::Iri(rt::IriBuf::new("https://example.org/ns/alice".into())?);
        let s2: rt::Subject = s1.as_subject_proxy().into();
        assert_eq!(s1, s2);
        let s2: rt::Subject = s1.as_lexical_subject_ref().as_subject_proxy().into();
        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn subject_bnode() -> TestResult {
        let s1 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let s2: rt::Subject = s1.as_subject_proxy().into();
        assert_eq!(s1, s2);
        let s2: rt::Subject = s1.as_lexical_subject_ref().as_subject_proxy().into();
        assert_eq!(s1, s2);
        Ok(())
    }

    #[test]
    fn predicate() -> TestResult {
        let p1 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let p2: rt::IriBuf = Predicate::as_iri(&p1).into();
        assert_eq!(p1, p2);
        let p2: rt::IriBuf = Predicate::as_iri(&p1.as_iri()).into();
        assert_eq!(p1, p2);
        Ok(())
    }

    #[test]
    fn object_iri() -> TestResult {
        let o1 = rt::Object::from_iri(rt::IriBuf::new("https://example.org/ns/alice".into())?);
        let o2: rt::Object = o1.as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        let o2: rt::Object = o1.as_lexical_object_ref().as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_bnode() -> TestResult {
        let o1 = rt::Object::from_blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let o2: rt::Object = o1.as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        let o2: rt::Object = o1.as_lexical_object_ref().as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_typed_literal() -> TestResult {
        let o1 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let o2: rt::Object = o1.as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        let o2: rt::Object = o1.as_lexical_object_ref().as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn object_language_tagged_literal() -> TestResult {
        let o1 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::LangString(langtag::LangTagBuf::new("en-Latn-UK".into())?),
        });
        let o2: rt::Object = o1.as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        let o2: rt::Object = o1.as_lexical_object_ref().as_object_proxy().try_into()?;
        assert_eq!(o1, o2);
        Ok(())
    }

    #[test]
    fn triple() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let t1 = rt::Triple(s0, p0, o0);
        let t2 = try_from_r2c2_triple(&t1)?;
        assert_eq!(t1, t2);
        let t2 = try_from_r2c2_triple(t1.as_lexical_triple_ref())?;
        assert_eq!(t1, t2);
        Ok(())
    }

    #[test]
    fn quad_default_graph() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let q1 = rt::Quad(s0, p0, o0, None);
        let q2 = try_from_r2c2_quad(&q1)?;
        assert_eq!(q1, q2);
        let q2 = try_from_r2c2_quad(q1.as_lexical_quad_ref())?;
        assert_eq!(q1, q2);
        Ok(())
    }

    #[test]
    fn quad_named_graph() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let g0 = rt::GraphLabel::Iri(rt::IriBuf::new("https://example.org/".into())?);
        let q1 = rt::Quad(s0, p0, o0, Some(g0));
        let q2 = try_from_r2c2_quad(&q1)?;
        assert_eq!(q1, q2);
        let q2 = try_from_r2c2_quad(q1.as_lexical_quad_ref())?;
        assert_eq!(q1, q2);
        Ok(())
    }

    // testing round trip rdf_types → oxrdf → rdf_types -> oxrdf

    #[test]
    fn subject_iri_via_oxrdf() -> TestResult {
        let s1 = rt::Subject::Iri(rt::IriBuf::new("https://example.org/ns/alice".into())?);
        let s2: oxrdf::Subject = s1.as_subject_proxy().into();
        let s3: rt::Subject = s2.as_subject_proxy().into();
        assert_eq!(s1, s3);
        let s4: oxrdf::Subject = s3.as_subject_proxy().into();
        assert_eq!(s2, s4);
        Ok(())
    }

    #[test]
    fn subject_bnode_via_oxrdf() -> TestResult {
        let s1 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let s2: oxrdf::Subject = s1.as_subject_proxy().into();
        let s3: rt::Subject = s2.as_subject_proxy().into();
        assert_eq!(s1, s3);
        let s4: oxrdf::Subject = s3.as_subject_proxy().into();
        assert_eq!(s2, s4);
        Ok(())
    }

    #[test]
    fn predicate_via_oxrdf() -> TestResult {
        let p1 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let p2: oxrdf::NamedNode = Predicate::as_iri(&p1).into();
        let p3: rt::IriBuf = p2.as_iri().into();
        assert_eq!(p1, p3);
        let p4: oxrdf::NamedNode = Predicate::as_iri(&p3).into();
        assert_eq!(p2, p4);
        Ok(())
    }

    #[test]
    fn object_iri_via_oxrdf() -> TestResult {
        let o1: rt::Object =
            rt::Object::from_iri(rt::IriBuf::new("https://example.org/ns/alice".into())?);
        let o2: oxrdf::Term = o1.as_object_proxy().into();
        let o3: rt::Object = o2.as_object_proxy().try_into()?;
        assert_eq!(o1, o3);
        let o4: oxrdf::Term = o3.as_object_proxy().into();
        assert_eq!(o2, o4);
        Ok(())
    }

    #[test]
    fn object_bnode_via_oxrdf() -> TestResult {
        let o1: rt::Object = rt::Object::from_blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let o2: oxrdf::Term = o1.as_object_proxy().into();
        let o3: rt::Object = o2.as_object_proxy().try_into()?;
        assert_eq!(o1, o3);
        let o4: oxrdf::Term = o3.as_object_proxy().into();
        assert_eq!(o2, o4);
        Ok(())
    }

    #[test]
    fn object_typed_literal_via_oxrdf() -> TestResult {
        let o1 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let o2: oxrdf::Term = o1.as_object_proxy().into();
        let o3: rt::Object = o2.as_object_proxy().try_into()?;
        assert_eq!(o1, o3);
        let o4: oxrdf::Term = o3.as_object_proxy().into();
        assert_eq!(o2, o4);
        Ok(())
    }

    #[test]
    fn object_language_tagged_literal_via_oxrdf() -> TestResult {
        let o1 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::LangString(langtag::LangTagBuf::new("en-Latn-UK".into())?),
        });
        let o2: oxrdf::Term = o1.as_object_proxy().into();
        let o3: rt::Object = o2.as_object_proxy().try_into()?;
        assert_eq!(o1, o3);
        let o4: oxrdf::Term = o3.as_object_proxy().into();
        assert_eq!(o2, o4);
        Ok(())
    }

    #[test]
    fn triple_via_oxrdf() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let t1 = rt::Triple(s0, p0, o0);
        let t2 = crate::impl_oxrdf::from_r2c2_triple(&t1);
        let t3 = try_from_r2c2_triple(&t2)?;
        assert_eq!(t1, t3);
        let t4 = crate::impl_oxrdf::from_r2c2_triple(&t3);
        assert_eq!(t2, t4);
        Ok(())
    }

    #[test]
    fn quad_default_graph_via_oxrdf() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let q1 = rt::Quad(s0, p0, o0, None);
        let q2 = crate::impl_oxrdf::from_r2c2_quad(&q1);
        let q3 = try_from_r2c2_quad(&q2)?;
        assert_eq!(q1, q3);
        let q4 = crate::impl_oxrdf::from_r2c2_quad(&q3);
        assert_eq!(q2, q4);
        Ok(())
    }

    #[test]
    fn quad_named_graph_via_oxrdf() -> TestResult {
        let s0 = rt::Subject::Blank(rt::BlankIdBuf::new("_:b1".into()).unwrap());
        let p0 = rt::IriBuf::new("https://example.org/ns/alice".into())?;
        let o0 = rt::Object::Literal(rt::Literal {
            value: "⛄".into(),
            type_: rt::LiteralType::Any(rt::IriBuf::new(XSD_STRING.into())?),
        });
        let g0 = rt::GraphLabel::Iri(rt::IriBuf::new("https://example.org/".into())?);
        let q1 = rt::Quad(s0, p0, o0, Some(g0));
        let q2 = crate::impl_oxrdf::from_r2c2_quad(&q1);
        let q3 = try_from_r2c2_quad(&q2)?;
        assert_eq!(q1, q3);
        let q4 = crate::impl_oxrdf::from_r2c2_quad(&q3);
        assert_eq!(q2, q4);
        Ok(())
    }

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    static XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
}
