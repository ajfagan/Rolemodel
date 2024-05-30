//! Rolemodel
//! 
//! A library for conducting multiset Gene Set Enrichment Calculations on gene expression data.

use std::cell::Ref;

/// A trait for running GSEA calculations on gene expression data.
pub trait GeneSet {
    type PartNode;
    type WholeNode;

    fn iter_parts(&self) -> impl Iterator<Item = Self::PartNode>;
    fn iter_wholes(&self) -> impl Iterator<Item = Self::WholeNode>;
}

/// A trait for storing the Part nodes of a GSEA calculation.
pub trait Part {
    type Data;
    type WholeNode;

    fn borrow_data(&self) -> Option<&Self::Data>;
    fn data(&self) -> Option<Ref<Self::Data>>;
    fn wholes(&self) -> impl Iterator<Item = Self::WholeNode>;
}

/// A trait for storing the Whole nodes of a GSEA calculation.
pub trait Whole {
    type Data;
    type PartNode;

    fn borrow_data(&self) -> Option<&Self::Data>;
    fn data(&self) -> Option<Ref<Self::Data>>;
    fn parts(&self) -> impl Iterator<Item = Self::PartNode>;
}

mod gene_ontology;
pub use gene_ontology::{genes::Gene, terms::Term, GeneOntology};
mod rolemodel;
mod unsafe_rolemodel;