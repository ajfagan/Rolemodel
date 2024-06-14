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
pub struct GeneOntology<Id, Td, Gd> 
where
    Gd: Copy,
    Td: Copy,
{
    genes: Vec<Gene<Id, Gd>>,
    terms: Vec<Term<Id, Td>>,
}
impl<Id, Td, Gd> GeneOntology<Id, Td, Gd> 
where
    Id: Clone,
    Td: std::fmt::Debug + Copy,
    Gd: std::fmt::Debug + Copy,
{
    pub fn new(
        genes: Vec<Gene<Id, Gd>>,
        terms: Vec<Term<Id, Td>>,
    ) -> Self {
        Self {
            genes,
            terms
        }
    }

    pub fn genes(&self) -> &Vec<Gene<Id, Gd>> {
        &self.genes
    }
    pub fn terms(&self) -> &Vec<Term<Id, Td>> {
        &self.terms
    }

    pub fn from_incidence(
        genes: Vec<(Id, Gd)>, 
        terms: Vec<(Id, Td)>, 
        adj: Vec<(usize, usize)>
    ) -> Self {


        let mut gene_ontology = Self::new(
            genes.iter().map(|(id, data)| Gene::new(id.clone(), *data, vec![])).collect(), 
            terms.iter().map(|(id, data)| Term::new(id.clone(), *data, vec![])).collect(), 
        );


        adj.iter()
            .for_each(|(term_idx, gene_idx)| {
                gene_ontology.wholes_mut()[*term_idx].add_gene(*gene_idx);
                gene_ontology.parts_mut()[*gene_idx].add_term(*term_idx);
                
            });


        gene_ontology
    }

}


impl<Id, Td, Gd> GeneOntology<Id, Td, Gd>
where 
    Id: Clone + Eq + std::hash::Hash + std::fmt::Debug,
    for<'a> Td: Default + std::fmt::Debug + Copy,
    for<'a> Gd: Default + std::fmt::Debug + Copy,
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
        for<'a> Id: serde::de::Deserialize<'a>,
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
        for<'a> Id: serde::Deserialize<'a>,
        for<'a> Dg: serde::de::Deserialize<'a> + Default,
        Fg: Fn(Dg) -> Gd,
        for<'a> Dt: serde::de::Deserialize<'a> + Default,
        Ft: Fn(Dt) -> Td,
    {
        let adj: Vec<(Id, Id)> = Self::parse_adj_file(
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
                let mut gene_map = HashMap::<Id, usize>::new();
                let gene_names = adj.iter().map(|(_, gene_name)| gene_name.clone()).unique();
                gene_names.clone().enumerate()
                    .for_each(|(idx, name)| {
                        gene_map.insert(name, idx);
                    });

                (
                    gene_names.map(|name| (name, f_gene(Dg::default()))).collect::<Vec<(Id, Gd)>>(),
                    gene_map
                )
            },
        };
        let (terms, term_map) = match term_data_file {
            Some(term_data_file) => {
                Self::parse_scalar_term_data(
                    term_data_file, 
                    term_name_header, term_data_header,
                    f_term,
                )
            },
            None => {
                let mut term_map = HashMap::new();
                let term_names = adj.iter().map(|(term_name, _)| term_name.clone()).unique();
                term_names.clone().enumerate()
                    .for_each(|(idx, name)| {
                        term_map.insert(name, idx);
                    });

                (
                    term_names.map(|name| (name, f_term(Dt::default()))).collect::<Vec<(Id, Td)>>(),
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
    ) -> (Vec<(Id, Gd)>, HashMap<Id, usize>) 
    where 
        for<'a> Id: serde::de::Deserialize<'a>,
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
                let (name, data): (Id, D) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (Id, Dg)",
                            name_header, data_header, gene_data_file
                        )
                    );
                if let Some(idx) = gene_map.get(&name) {
                    println!(
                        "Warning: gene {:?} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    gene_map.insert(name.clone(), genes.len());
                    genes.push((name, f(data)));
                }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().enumerate().for_each(|(entry_idx, result)| {
                // let record: (String, Gd) = result
                let (name, data): (Id, D) = result
                    .unwrap_or_else(|_| 
                        panic!(
                            "Could not coerce data in file {} to (String, Dg)",
                            gene_data_file
                        )
                    );
                if let Some(idx) = gene_map.get(&name) {
                    println!(
                        "Warning: gene {:?} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    gene_map.insert(name.clone(), genes.len());
                    genes.push((name, f(data)));
                }
            })
        }
        
        (genes, gene_map)
    }
    fn parse_scalar_term_data<D, F>(
        term_data_file: String, 
        term_name_header: Option<String>, term_data_header: Option<String>,
        f: F,
    ) -> (Vec<(Id, Td)>, HashMap<Id, usize>) 
    where
        for<'a> Id: serde::de::Deserialize<'a>,
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
                let (name, data): (Id, D) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (String, D)",
                            name_header, data_header, term_data_file
                        )
                    );
                if let Some(idx) = term_map.get(&name) {
                    println!(
                        "Warning: term {:?} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    term_map.insert(name.clone(), terms.len());
                    terms.push((name, f(data)));
                }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().enumerate().for_each(|(entry_idx, result)| {
                // let record: (String, Gd) = result
                let (name, data): (Id, D) = result
                    .unwrap_or_else(|_| 
                        panic!(
                            "Could not coerce data in file {} to (String, Td)",
                            term_data_file
                        )
                    );
                if let Some(idx) = term_map.get(&name) {
                    println!(
                        "Warning: term {:?} appears in data multiple times (index {} and {}). Skipping repeated entry.", 
                        name, 
                        idx, 
                        entry_idx
                    )
                } else {
                    term_map.insert(name.clone(), terms.len());
                    terms.push((name, f(data)));
                }
            })
        }
        
        (terms, term_map)
    }

    fn parse_adj_file(
        adj_file: String, 
        adj_term_header: Option<String>,
        adj_gene_header: Option<String>
    ) -> Vec<(Id, Id)> 
    where 
        for<'a> Id: serde::de::Deserialize<'a>,
    {
        assert_eq!(
            adj_term_header.is_none(),
            adj_gene_header.is_none(),
            "Headers must be given for either both terms and genes in the adjacency data, or for neither",
        );

        // let mut adj = vec![];
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
                let (term_name, gene_name): (Id, Id) = result.unwrap()
                    .deserialize(Some(&headers))
                    .unwrap_or_else(|_|
                        panic!(
                            "Could not coerce data in columns {}, {} of file {} to (String, String)",
                            term_header, gene_header, adj_file
                        )
                    );
                // if adj_set.contains(&(term_name.clone(), gene_name.clone())) {
                //     println!(
                //         "Warning: annotation ({:?} - {:?}) appears in adjacency data multiple times. Skipping repeated entry.", 
                //         term_name, 
                //         gene_name
                //     )
                // } else {
                    // adj.push((term_name.clone(), gene_name.clone()));
                adj_set.insert((term_name, gene_name));
                // }
            })
        } else {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            rdr.deserialize().for_each(|result| {
                let (term_name, gene_name): (Id, Id) = result.unwrap();
                // if adj.contains(&(term_name.clone(), gene_name.clone())) {
                //     println!(
                //         "Warning: annotation ({} - {}) appears in adjacency data multiple times. Skipping repeated entry.", 
                //         term_name, 
                //         gene_name
                //     )
                // } else {
                adj_set.insert((term_name, gene_name));
                // }
            });
        }
        
        adj_set.into_iter().collect()
    }
}

impl<Id, Td, Gd> GeneSet for GeneOntology<Id, Td, Gd> 
where 
    Td: Copy,
    Gd: Copy,
{
    type PartNode = Gene<Id, Gd>;
    type WholeNode = Term<Id, Td>;

    fn iter_parts(&self) -> impl Iterator<Item = &Self::PartNode> {
        self.genes.iter()
    }
    fn iter_wholes(&self) -> impl Iterator<Item = &Self::WholeNode> {
        self.terms.iter()
    }
    
    fn iter_parts_mut(&mut self) -> impl Iterator<Item = &mut Self::PartNode> {
        self.genes.iter_mut()
    }
    
    fn iter_wholes_mut(&mut self) -> impl Iterator<Item = &mut Self::WholeNode> {
        self.terms.iter_mut()
    }
    
    fn parts(&self) -> &Vec<Self::PartNode> {
        &self.genes
    }
    
    fn wholes(&self) -> &Vec<Self::WholeNode> {
        &self.terms
    }
    
    fn parts_mut(&mut self) -> &mut Vec<Self::PartNode> {
        &mut self.genes
    }
    
    fn wholes_mut(&mut self) -> &mut Vec<Self::WholeNode> {
        &mut self.terms
    }

}


impl<Id, Td, Gd> Saveable for GeneOntology<Id, Td, Gd>
where 
    Gd: Booleable + Saveable + Activeable + Copy,
    Td: Activeable + Saveable + Copy,
{
    type Output = ();

    fn current(&self) -> Self::Output { }
    fn saved(&self) -> Self::Output {}
    fn restore(&mut self) {
        self.iter_parts_mut().for_each(|gene| gene.data_mut().get_mut().restore());
        self.iter_wholes_mut().for_each(|term| term.data_mut().get_mut().restore());
    }
    fn save(&mut self) {
        self.iter_parts_mut().for_each(|gene| gene.data_mut().get_mut().save());
        self.iter_wholes_mut().for_each(|term| term.data_mut().get_mut().save());
    }
}

pub struct GOGeneListRolemodel<Id, Td, Gd> 
where 
    Gd: Booleable + Saveable + Activeable + Copy,
    Td: Activeable + Saveable + Copy,
{
    gene_ontology: GeneOntology<Id, Td, Gd>,

    burn_in: usize,
    nsamples: usize,
    thinning: usize,

    set_activity_probability: f64,
    true_active_gene_hit_rate: f64,
    false_inactive_gene_hit_rate: f64,
    illegal_set_penalty: f64,
}

impl<Id, Td, Gd> Saveable for GOGeneListRolemodel<Id, Td, Gd>
where 
    Gd: Saveable + Booleable + Activeable + Copy,
    Td: Activeable + Saveable + Copy,
{
    type Output = ();
    fn current(&self) -> Self::Output { }
    fn saved(&self) -> Self::Output { }
    fn restore(&mut self) {
        self.gene_ontology.restore();
    }
    fn save(&mut self) {
        self.gene_ontology.save();
    }
}

impl<Id, Td, Gd> GOGeneListRolemodel<Id, Td, Gd>
where 
    Gd: Booleable + Saveable + Activeable + Copy,
    Td: Activeable + Saveable + Copy,
{
    pub fn new(
        gene_ontology: GeneOntology<Id, Td, Gd>,
    
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

impl<Id, Td, Gd> GeneSet for GOGeneListRolemodel<Id, Td, Gd> 
where 
    Gd: Booleable + Saveable + Activeable + Copy,
    Td: Activeable + Saveable + Copy,
{
    type PartNode = Gene<Id, Gd>;
    type WholeNode = Term<Id, Td>;

    fn iter_parts(&self) -> impl Iterator<Item = &Self::PartNode> {
        self.gene_ontology.genes.iter()
    }
    fn iter_wholes(&self) -> impl Iterator<Item = &Self::WholeNode> {
        self.gene_ontology.terms.iter()
    }
    
    fn iter_parts_mut(&mut self) -> impl Iterator<Item = &mut Self::PartNode> {
        self.gene_ontology.genes.iter_mut()
    }
    
    fn iter_wholes_mut(&mut self) -> impl Iterator<Item = &mut Self::WholeNode> {
        self.gene_ontology.terms.iter_mut()
    }
    
    fn parts(&self) -> &Vec<Self::PartNode> {
        self.gene_ontology.parts()
    }
    
    fn wholes(&self) -> &Vec<Self::WholeNode> {
        self.gene_ontology.wholes()
    }
    
    fn parts_mut(&mut self) -> &mut Vec<Self::PartNode> {
        &mut self.gene_ontology.genes
    }
    
    fn wholes_mut(&mut self) -> &mut Vec<Self::WholeNode> {
        &mut self.gene_ontology.terms
    }
}

impl<Id, Td, Gd> GeneListRolemodel for GOGeneListRolemodel<Id, Td, Gd> 
where 
    Self::PartNode: Node<Id = Id, Data = Gd> + Part,
    Self::WholeNode: Node<Id = Id, Data = Td> + Whole,
    Gd: Saveable + Booleable + Activeable + Copy,
    Td: Saveable + Activeable + Copy,
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
