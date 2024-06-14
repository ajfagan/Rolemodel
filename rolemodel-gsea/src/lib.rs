//! Rolemodel
//! 
//! A library for conducting multiset Gene Set Enrichment Calculations on gene expression data.

use std::cell::Cell;

/// A trait for running GSEA calculations on gene expression data.
pub trait GeneSet {
    type PartNode;
    type WholeNode;

    fn parts(&self) -> &Vec<Self::PartNode>;
    fn wholes(&self) -> &Vec<Self::WholeNode>;
    fn parts_mut(&mut self) -> &mut Vec<Self::PartNode>;
    fn wholes_mut(&mut self) -> &mut Vec<Self::WholeNode>;
    fn iter_parts(&self) -> impl Iterator<Item = &Self::PartNode>;
    fn iter_wholes(&self) -> impl Iterator<Item = &Self::WholeNode>;
    fn iter_parts_mut(&mut self) -> impl Iterator<Item = &mut Self::PartNode>;
    fn iter_wholes_mut(&mut self) -> impl Iterator<Item = &mut Self::WholeNode>;
}

struct NodeIterator<'a, T: 'a, Neighbors> 
where
    Neighbors: std::ops::Index<usize, Output = T>,
{
    indices: std::slice::Iter<'a, usize>,
    all_nodes: &'a Neighbors,
}
impl<'a, T: 'a, Neighbors> NodeIterator<'a, T, Neighbors> 
where 
    Neighbors: std::ops::Index<usize, Output = T>,
{
    fn new(indices: &'a [usize], all_nodes: &'a Neighbors) -> Self {
        Self {
            indices: indices.iter(),
            all_nodes,
        }
    }
}

impl<'a, T: 'a, Neighbors> Iterator for NodeIterator<'a, T, Neighbors> 
where
    Neighbors: std::ops::Index<usize, Output = T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indices.next() {
            Some(idx) => Some(&self.all_nodes[*idx]),
            None => None,
        }
    }
}

pub trait Node {
    type Id;
    type Data: Copy;

    fn id(&self) -> &Self::Id;

    fn data(&self) -> &Cell<Self::Data>;
    fn data_mut(&mut self) -> &mut Cell<Self::Data>;

    fn idx_neighbors(&self) -> &Vec<usize>;
    fn iter_neighbors<'a, T: 'a, Neighbors>(
        &'a self, 
        all_nodes: &'a Neighbors
    ) -> impl Iterator<Item = &'a T> 
    where
        Neighbors: std::ops::Index<usize, Output = T>,
    {
        NodeIterator::new(
            self.idx_neighbors(),
            all_nodes,
        )
    }
}

impl<N: Node> Activeable for N
where 
    <N as Node>::Data: Activeable,
{
    fn is_active(&self) -> bool { self.data().get().is_active() }
    fn set_activity(&mut self, b: bool) { self.data().get().set_activity(b)}
    
    fn is_legal(&self) -> bool {
        todo!()
    }
}

/// A trait for storing the Part nodes of a GSEA calculation.
pub trait Part: Node 
{

    // fn data(&self) -> &Self::Data;
    // fn data_mut(&mut self) -> &mut Self::Data;
    // fn ref_data(&self) -> Ref<Self::Data>;
    // fn wholes<T>(&self) -> Vec<T>;
    fn iter_wholes<'a, T: 'a + Whole>(&'a self, all_nodes: &'a Vec<T>) -> impl Iterator<Item = &'a T> {
        self.iter_neighbors::<T, Vec<T>>(all_nodes)
    }
}
/// A trait for storing the Whole nodes of a GSEA calculation.
pub trait Whole: Node
{
    // fn parts<T>(&self) -> Vec<T> {
    //     self.iter_neighbors().collect()
    // }
    fn iter_parts<'a, T: 'a>(&'a self, all_nodes: &'a Vec<T>) -> impl Iterator<Item = &'a T> {
        self.iter_neighbors::<T, Vec<T>>(all_nodes)
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
// pub use gene_ontology::{genes::Gene, terms::Term, GeneOntology};
// mod rolemodel;
// mod unsafe_rolemodel;
