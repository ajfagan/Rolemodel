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

pub trait Node {
    type Data;
    type NeighborType: Node;

    fn data(&self) -> &Self::Data;
    fn data_mut(&mut self) -> &mut Self::Data;
    fn ref_data(&self) -> Ref<Self::Data>;
    fn iter_neighbors(&self) -> impl Iterator<Item = Self::NeighborType>;
}
impl<N: Node> Activeable for N
where 
    <N as Node>::Data: Activeable,
    <<N as Node>::NeighborType as Node>::Data: Activeable,
{
    fn is_active(&self) -> bool { self.data().is_active() }
    fn set_activity(&mut self, b: bool) { self.data_mut().set_activity(b)}
    
    fn is_legal(&self) -> bool {
        todo!()
    }
}
// impl<N: Node> Saveable for N
// where
//     <N as Node>::Data: Saveable,
//     <<N as Node>::NeighborType as Node>::Data: Saveable,
// {
//     type Output = <Self as Node>::Data;

//     fn save(&mut self) {
//         self.data_mut().save()
//     }

//     fn restore(&mut self) {
//         self.data_mut().restore()
//     }

//     fn current(&self) -> Self::Output {
//         todo!()
//     }

//     fn saved(&self) -> Self::Output {
//         todo!()
//     }
// }


/// A trait for storing the Part nodes of a GSEA calculation.
pub trait Part: Node 
{

    // fn data(&self) -> &Self::Data;
    // fn data_mut(&mut self) -> &mut Self::Data;
    // fn ref_data(&self) -> Ref<Self::Data>;
    fn wholes(&self) -> impl Iterator<Item = <Self as Node>::NeighborType> {
        self.iter_neighbors()
    }
}
/// A trait for storing the Whole nodes of a GSEA calculation.
pub trait Whole: Node
{
    fn parts(&self) -> impl Iterator<Item = <Self as Node>::NeighborType> {
        self.iter_neighbors()
    }

    // fn is_active(&self) -> bool { self.data().is_active() }
    // fn is_legal(&self) -> bool { self.data().is_legal() }
    // fn switch_activity(&mut self, b: bool) { self.data_mut().set_activity(b)}
}

mod gene_ontology;
pub use gene_ontology::{
    genes::Gene, 
    terms::Term, 
    GeneOntology,
    GOGeneListRolemodel,
};
mod rolemodel;
pub use rolemodel::saveable::Saveable;
pub use rolemodel::{
    Rolemodel,
    saveable::SaveableData,
    genelist::GeneListRolemodel,
    activeable::Activeable,
};
pub use gene_ontology::{genes::Gene, terms::Term, GeneOntology};
mod rolemodel;
mod unsafe_rolemodel;
