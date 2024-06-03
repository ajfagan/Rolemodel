pub mod genes;
pub mod terms;

use crate::{
    gene_ontology::{
        genes::Gene,
        terms::Term,
    }, rolemodel::{
        genelist::GeneListRolemodel,
        saveable::Saveable,
    }, Activeable, GeneSet, Node, Part, Whole
};
use rv::data::Booleable;

use std::{
    cell::RefCell,
    rc::Rc,
    ops::Deref,
};
use hashbrown::{HashMap, HashSet};

use itertools::Itertools;

// impl<Td, Gd> RmPart for Rc<RefCell<genes::Gene<Td, Gd>>> 
// where 
//     Td: Booleable
// {
//     fn is_active(&self) -> bool {
//         self.wholes().any(|whole| whole.ref_data().deref().into_bool())
//     }
// }

// impl<Td, Gd> RmWhole for Rc<RefCell<terms::Term<Td, Gd>>>
// where 
//     Td: Booleable
// {
//     fn is_illegal(&self) -> bool {
//         let is_active = self.ref_data().deref().into_bool();
//         match is_active {
//             true => self.parts().all(|part| part.is_active()),
//             false => self.parts().any(|part| !part.is_active()),
//         }
//     }
// }

#[derive(Debug)]
pub struct GeneOntology<Td, Gd> {
    genes: Vec<Rc<RefCell<genes::Gene<Td, Gd>>>>,
    terms: Vec<Rc<RefCell<terms::Term<Td, Gd>>>>,
}
impl<Td, Gd> GeneOntology<Td, Gd> 
where
    Td: std::fmt::Debug,
    Gd: std::fmt::Debug,
{
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


impl<Td, Gd> GeneOntology<Td, Gd>
where 
    for<'a> Td: Default + std::fmt::Debug,
    for<'a> Gd: Default + std::fmt::Debug,
{
    pub fn read_scalar_data(
        gene_data_file: Option<String>, 
        gene_name_header: Option<String>, gene_data_header: Option<String>,
        term_data_file: Option<String>, 
        term_name_header: Option<String>, term_data_header: Option<String>,
        adj_file: String, 
        adj_gene_header: Option<String>, adj_term_header: Option<String>,
    ) -> Self 
    where 
        for<'a> Gd: serde::de::Deserialize<'a>,
        for<'a> Td: serde::de::Deserialize<'a>,
    {
        Self::read_apply_scalar_data(
            gene_data_file, gene_name_header, gene_data_header,
            term_data_file, term_name_header, term_data_header,
            adj_file, adj_gene_header, adj_term_header,
            |x: Gd| x, |x: Td| x,
        )
    }
    pub fn read_apply_scalar_data<Dg, Fg, Dt, Ft>(
        gene_data_file: Option<String>, 
        gene_name_header: Option<String>, gene_data_header: Option<String>,
        term_data_file: Option<String>, 
        term_name_header: Option<String>, term_data_header: Option<String>,
        adj_file: String, 
        adj_gene_header: Option<String>, adj_term_header: Option<String>,
        f_gene: Fg, f_term: Ft,
    ) -> Self 
    where 
        for<'a> Dg: serde::de::Deserialize<'a> + Default,
        Fg: Fn(Dg) -> Gd,
        for<'a> Dt: serde::de::Deserialize<'a> + Default,
        Ft: Fn(Dt) -> Td,
    {
        let adj = Self::parse_adj_file(
            adj_file, 
            adj_gene_header, adj_term_header
        );
        let (genes, gene_map) = match gene_data_file {
            Some(gene_data_file) => Self::parse_scalar_gene_data(
                gene_data_file, 
                gene_name_header, gene_data_header,
                f_gene,
            ),
            None => {
                let mut gene_map = HashMap::new();
                let gene_names = adj.iter().map(|(_, gene_name)| gene_name.clone()).unique();
                gene_names.clone().enumerate()
                    .for_each(|(idx, name)| {
                        gene_map.insert(name, idx);
                    });

                (
                    gene_names.map(|_| f_gene(Dg::default())).collect::<Vec<Gd>>(),
                    gene_map
                )
            },
        };
        let (terms, term_map) = match term_data_file {
            Some(term_data_file) => Self::parse_scalar_term_data(
                term_data_file, 
                term_name_header, term_data_header,
                f_term,
            ),
            None => {
                let mut term_map = HashMap::new();
                let term_names = adj.iter().map(|(term_name, _)| term_name.clone()).unique();
                term_names.clone().enumerate()
                    .for_each(|(idx, name)| {
                        term_map.insert(name, idx);
                    });

                (
                    term_names.map(|_| f_term(Dt::default())).collect::<Vec<Td>>(),
                    term_map
                )
            },
        };
        let adj = adj.iter()
            .map(|(term, gene)| {
                (term_map[term], gene_map[gene])
            })
            .collect::<Vec<(usize, usize)>>();

        Self::from_incidence(genes, terms, adj)
    }

    fn parse_scalar_gene_data<D, F> (
        gene_data_file: String, 
        gene_name_header: Option<String>, gene_data_header: Option<String>,
        f: F,
    ) -> (Vec<Gd>, HashMap<String, usize>) 
    where 
        for<'a> D: serde::de::Deserialize<'a>,
        F: Fn(D) -> Gd
    {

        assert_eq!(
            gene_name_header.is_none(),
            gene_data_header.is_none(),
            "Headers must be given for either both terms and genes in the adjacency data, or for neither",
        );

        let mut genes = vec![];
        let mut gene_map = HashMap::new();

        let file = std::fs::File::open(gene_data_file.clone()).
            unwrap_or_else(|_| 
                panic!("File {} not found", gene_data_file)
            );

        if let (Some(name_header), Some(data_header)) = (gene_name_header, gene_data_header) {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(file);

            assert!(
                rdr.headers().unwrap().iter().any(|header| header == name_header),
                "File expected to have column {} for gene names, found {:?}",
                name_header, rdr.headers().unwrap()
            );
            assert!(
                rdr.headers().unwrap().iter().any(|header| header == data_header),
                "File expected to have column {} for gene data, found {:?}",
                name_header, rdr.headers().unwrap()
            );
            let headers = csv::StringRecord::from(vec![name_header.clone(), data_header.clone()]);
            rdr.records().enumerate().for_each(|(entry_idx, result)| {
                let (name, data) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (String, Dg)",
                            name_header, data_header, gene_data_file
                        )
                    );
                if let Some(idx) = gene_map.get(&name) {
                    println!(
                        "Warning: gene {} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    gene_map.insert(name, genes.len());
                    genes.push(f(data));
                }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().enumerate().for_each(|(entry_idx, result)| {
                // let record: (String, Gd) = result
                let (name, data): (String, D) = result
                    .unwrap_or_else(|_| 
                        panic!(
                            "Could not coerce data in file {} to (String, Dg)",
                            gene_data_file
                        )
                    );
                if let Some(idx) = gene_map.get(&name) {
                    println!(
                        "Warning: gene {} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    gene_map.insert(name, genes.len());
                    genes.push(f(data));
                }
            })
        }
        
        (genes, gene_map)
    }
    fn parse_scalar_term_data<D, F>(
        term_data_file: String, 
        term_name_header: Option<String>, term_data_header: Option<String>,
        f: F,
    ) -> (Vec<Td>, HashMap<String, usize>) 
    where
        for<'a> D: serde::de::Deserialize<'a>,
        F: Fn(D) -> Td,
    {

        assert_eq!(
            term_name_header.is_none(),
            term_data_header.is_none(),
            "Headers must be given for either both terms and genes in the adjacency data, or for neither",
        );

        let mut terms = vec![];
        let mut term_map = HashMap::new();

        let file = std::fs::File::open(term_data_file.clone()).
            unwrap_or_else(|_| 
                panic!("File {} not found", term_data_file)
            );
            
        if let (Some(name_header), Some(data_header)) = (term_name_header, term_data_header) {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(file);

            assert!(
                rdr.headers().unwrap().iter().any(|header| header == name_header),
                "File expected to have column {} for term names, found {:?}",
                name_header, rdr.headers().unwrap()
            );
            assert!(
                rdr.headers().unwrap().iter().any(|header| header == data_header),
                "File expected to have column {} for term data, found {:?}",
                name_header, rdr.headers().unwrap()
            );
            let headers = csv::StringRecord::from(vec![name_header.clone(), data_header.clone()]);
            rdr.records().enumerate().for_each(|(entry_idx, result)| {
                let (name, data) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (String, D)",
                            name_header, data_header, term_data_file
                        )
                    );
                if let Some(idx) = term_map.get(&name) {
                    println!(
                        "Warning: term {} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    term_map.insert(name, terms.len());
                    terms.push(f(data));
                }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().enumerate().for_each(|(entry_idx, result)| {
                // let record: (String, Gd) = result
                let (name, data): (String, D) = result
                    .unwrap_or_else(|_| 
                        panic!(
                            "Could not coerce data in file {} to (String, Td)",
                            term_data_file
                        )
                    );
                if let Some(idx) = term_map.get(&name) {
                    println!(
                        "Warning: term {} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    term_map.insert(name, terms.len());
                    terms.push(f(data));
                }
            })
        }
        
        (terms, term_map)
    }

    fn parse_adj_file(
        adj_file: String, 
        adj_term_header: Option<String>,
        adj_gene_header: Option<String>
    ) -> Vec<(String, String)> {
        assert_eq!(
            adj_term_header.is_none(),
            adj_gene_header.is_none(),
            "Headers must be given for either both terms and genes in the adjacency data, or for neither",
        );

        let mut adj = vec![];
        let mut adj_set = HashSet::new();

        let file = std::fs::File::open(adj_file.clone()).
            unwrap_or_else(|_| 
                panic!("File {} not found", adj_file)
            );
            
        if let (Some(term_header), Some(gene_header)) = (adj_term_header, adj_gene_header) {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(file);

            assert!(
                rdr.headers().unwrap().iter().any(|header| header == term_header),
                "File expected to have column {} for term names, found {:?}",
                term_header, rdr.headers().unwrap()
            );
            assert!(
                rdr.headers().unwrap().iter().any(|header| header == gene_header),
                "File expected to have column {} for gene names, found {:?}",
                gene_header, rdr.headers().unwrap()
            );

            let headers = csv::StringRecord::from(vec![term_header.clone(), gene_header.clone()]);
            rdr.records().for_each(|result| {
                let (term_name, gene_name): (String, String) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (String, String)",
                            term_header, gene_header, adj_file
                        )
                    );
                if adj_set.contains(&(term_name.clone(), gene_name.clone())) {
                    println!(
                        "Warning: annotation ({} - {}) appears in adjacency data multiple times. Skipping repeated entry.", 
                        term_name, 
                        gene_name
                    )
                } else {
                    adj.push((term_name.clone(), gene_name.clone()));
                    adj_set.insert((term_name, gene_name));
                }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().for_each(|result| {
                let (term_name, gene_name): (String, String) = result.unwrap();
                // if adj.contains(&(term_name.clone(), gene_name.clone())) {
                //     println!(
                //         "Warning: annotation ({} - {}) appears in adjacency data multiple times. Skipping repeated entry.", 
                //         term_name, 
                //         gene_name
                //     )
                // } else {
                    adj.push((term_name, gene_name));
                // }
            });
        }
        
        adj
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
    Gd: Booleable + Saveable + Activeable,
    Td: Activeable + Saveable,
{
    type Output = ();

    fn current(&self) -> Self::Output { }
    fn saved(&self) -> Self::Output {}
    fn restore(&mut self) {
        self.iter_parts().for_each(|gene| gene.borrow_mut().restore());
        self.iter_wholes().for_each(|term| term.borrow_mut().restore());
    }
    fn save(&mut self) {
        self.iter_parts().for_each(|gene| gene.borrow_mut().save());
        self.iter_wholes().for_each(|term| term.borrow_mut().save());
    }
}

pub struct GOGeneListRolemodel<Td, Gd> 
where 
    Gd: Booleable + Saveable + Activeable,
    Td: Activeable + Saveable,
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

impl<Td, Gd> Saveable for GOGeneListRolemodel<Td, Gd>
where 
    Gd: Saveable + Booleable + Activeable,
    Td: Activeable + Saveable,
{
    type Output = ();
    fn current(&self) -> Self::Output { }
    fn saved(&self) -> Self::Output { }
    fn restore(&mut self) {
        self.iter_wholes().for_each(|mut whole| whole.restore());
        self.iter_parts().for_each(|mut part| part.restore());
    }
    fn save(&mut self) {
        self.iter_wholes().for_each(|mut whole| whole.save());
        self.iter_parts().for_each(|mut part| part.save());
        
    }
}

impl<Td, Gd> GOGeneListRolemodel<Td, Gd>
where 
    Gd: Booleable + Saveable + Activeable,
    Td: Activeable + Saveable,
{
    pub fn new(
        gene_ontology: GeneOntology<Td, Gd>,
    
        burn_in: usize,
        nsamples: usize,
        thinning: usize,
    
        set_activity_probability: f64,
        true_active_gene_hit_rate: f64,
        false_inactive_gene_hit_rate: f64,
        illegal_set_penalty: f64,
    ) -> Self {
        Self {
            gene_ontology,
            burn_in,
            nsamples,
            thinning,
            set_activity_probability,
            true_active_gene_hit_rate,
            false_inactive_gene_hit_rate,
            illegal_set_penalty,
        }
    }
}

impl<Td, Gd> GeneSet for GOGeneListRolemodel<Td, Gd> 
where 
    Gd: Booleable + Saveable + Activeable,
    Td: Activeable + Saveable,
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
    Self::PartNode: Node<Data = Gd, NeighborType = Rc<RefCell<Term<Td, Gd>>>> + Part,
    Self::WholeNode: Node<Data = Td, NeighborType = Rc<RefCell<Gene<Td, Gd>>>> + Whole,
    Gd: Saveable + Booleable + Activeable,
    Td: Saveable + Activeable,
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
    
    fn posterior_llikelihood(&self) -> &crate::SaveableData<f64> {
        todo!()
    }
}
