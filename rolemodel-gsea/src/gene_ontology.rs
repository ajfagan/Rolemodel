pub mod genes;
pub mod terms;

use crate::{
    gene_ontology::{
        genes::Gene,
        terms::Term,
    }, rolemodel::{
        genelist::GeneListRolemodel,
        saveable::Saveable,
        RmPart, RmWhole,
    }, GeneSet
};
use rv::data::Booleable;

use std::{
    cell::RefCell,
    rc::Rc,
};

pub struct GeneOntology<Td, Gd> {
    genes: Vec<Rc<RefCell<genes::Gene<Td, Gd>>>>,
    terms: Vec<Rc<RefCell<terms::Term<Td, Gd>>>>,
}
impl<Td, Gd> GeneOntology<Td, Gd> {
    pub fn new(
        genes: Vec<Rc<RefCell<genes::Gene<Td, Gd>>>>,
        terms: Vec<Rc<RefCell<terms::Term<Td, Gd>>>>,
    ) -> Self {
        Self {
            genes,
            terms
        }
    }

    pub fn genes(&self) -> &Vec<Rc<RefCell<genes::Gene<Td, Gd>>>> {
        &self.genes
    }
    pub fn terms(&self) -> &Vec<Rc<RefCell<terms::Term<Td, Gd>>>> {
        &self.terms
    }
    pub fn mut_terms(&mut self) -> &mut Vec<Rc<RefCell<terms::Term<Td, Gd>>>> {
        &mut self.terms
    }

    pub fn from_incidence(gene_data: Vec<Gd>, term_data: Vec<Td>, adj: Vec<(usize, usize)>) -> Self {
        let gene_ontology = Self::new(
            gene_data.into_iter().map(|d| Rc::new(RefCell::new(Gene::new(d, vec![])))).collect(),
            term_data.into_iter().map(|d| Rc::new(RefCell::new(Term::new(d, vec![])))).collect(),
        );

        adj.iter()
            .for_each(|(term_idx, gene_idx)| {
                let new_term = gene_ontology.terms[*term_idx].clone();
                gene_ontology.genes[*gene_idx].borrow_mut().add_term(new_term);

                let new_gene = gene_ontology.genes[*gene_idx].clone();
                gene_ontology.terms[*term_idx].borrow_mut().add_gene(new_gene);
            });

        gene_ontology
    }
}

impl<Td, Gd> GeneSet for GeneOntology<Td, Gd> {
    type PartNode = Rc<RefCell<genes::Gene<Td, Gd>>>;
    type WholeNode = Rc<RefCell<terms::Term<Td, Gd>>>;

    fn iter_parts(&self) -> impl Iterator<Item = Self::PartNode> {
        self.genes.iter().cloned()
    }
    fn iter_wholes(&self) -> impl Iterator<Item = Self::WholeNode> {
        self.terms.iter().cloned()
    }

}


impl<Td, Gd> Saveable for GeneOntology<Td, Gd>
where 
    Gd: Booleable + Saveable,
    Td: Booleable + Saveable,
{
    type Output = ();

    fn current(&self) -> Self::Output {
        ()
    }
    fn saved(&self) -> Self::Output {
        ()
    }
    fn restore(&mut self) {
        self.iter_parts().for_each(|gene| gene.borrow_mut().restore());
        self.iter_wholes().for_each(|term| term.borrow_mut().restore());
    }
    fn save(&mut self) {
        self.iter_parts().for_each(|gene| gene.borrow_mut().save());
        self.iter_wholes().for_each(|term| term.borrow_mut().save());
    }
}

struct GOGeneListRolemodel<Td, Gd> 
where 
    Gd: Booleable + Saveable,
    Td: Booleable + Saveable,
{
    gene_ontology: GeneOntology<Td, Gd>,

    burn_in: usize,
    nsamples: usize,
    thinning: usize,

    set_activity_probability: f64,
    true_active_gene_hit_rate: f64,
    false_inactive_gene_hit_rate: f64,
    illegal_set_penalty: f64,
}

impl<Td, Gd> GeneSet for GOGeneListRolemodel<Td, Gd> 
where 
    Gd: Booleable + Saveable,
    Td: Booleable + Saveable,
{
    type PartNode = Rc<RefCell<genes::Gene<Td, Gd>>>;
    type WholeNode = Rc<RefCell<terms::Term<Td, Gd>>>;

    fn iter_parts(&self) -> impl Iterator<Item = Self::PartNode> {
        self.gene_ontology.genes.iter().cloned()
    }
    fn iter_wholes(&self) -> impl Iterator<Item = Self::WholeNode> {
        self.gene_ontology.terms.iter().cloned()
    }
}

impl<Td, Gd> GeneListRolemodel for GOGeneListRolemodel<Td, Gd> 
where 
    Self::PartNode: RmPart<Data = Gd, WholeNode = Rc<RefCell<Term<Td, Gd>>>>,
    Self::WholeNode: RmWhole<Data = Td, PartNode = Rc<RefCell<Gene<Td, Gd>>>>,
    Gd: Saveable + Booleable,
    Td: Saveable + Booleable,
{
    fn set_activity_probability(&self) -> f64 {
        self.set_activity_probability
    }

    fn true_active_gene_hit_rate(&self) -> f64 {
        self.true_active_gene_hit_rate
    }

    fn false_inactive_gene_hit_rate(&self) -> f64 {
        self.false_inactive_gene_hit_rate
    }

    fn illegal_set_penalty(&self) -> f64 {
        self.illegal_set_penalty
    }

    fn burn_in(&self) -> usize {
        self.burn_in
    }
    fn nsamples(&self) -> usize {
        self.nsamples
    }
    fn thinning(&self) -> usize {
        self.thinning
    }
}
