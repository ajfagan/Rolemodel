

#[cfg(test)]
mod tests {

    use std::{
        rc::Rc,
        cell::RefCell,
        ops::Deref,
    };

    use rolemodel_gsea::{
        Activeable, GOGeneListRolemodel, Gene, GeneOntology, GeneSet, Node, Part, Rolemodel, Saveable, SaveableData, Term, Whole
    };
    use rv::data::Booleable;


    #[test]
    fn basic_gene_ont() {
        let mut gene_ontology = GeneOntology::<bool, bool>::new(
            vec![Rc::new(RefCell::new(Gene::new(true, vec![])))],
            vec![]);
        let gene = gene_ontology.genes()[0].clone();
        gene_ontology.mut_terms().push(Rc::new(RefCell::new(Term::new(false, vec![gene]))));

        assert_eq!(
            *gene_ontology.iter_wholes().next().unwrap().borrow().parts().next().unwrap().ref_data().deref(), 
            true
        );
    }

    #[test]
    fn larger_gene_ont() {
        let gene_ontology = GeneOntology::<bool, bool>::from_incidence(
            vec![true, false, true, true, true, false, false, false, false, true], 
            vec![false, true, false, false],
            vec![
                (0,0), (0,1), (0,4),
                (1,1), (1,5), (1,8), (1,9),
                (2,1),
                (3,0), (3, 6),
            ],
        );

        let result: Vec<usize> = gene_ontology.iter_wholes().map(|whole| whole.parts().size_hint().0).collect();

        assert_eq!(result, vec![3, 4, 1, 2]);
    }

    #[test]
    fn read_gene_ont() {
        let gene_data_file = Some("./tests/data/atovaquone-rnaseq/results.csv".to_string());
        let term_data_file = None;
        let adj_file = "./tests/data/atovaquone-rnaseq/adjacency.csv".to_string();

        let gene_ontology = GeneOntology::<f64, f64>::read_scalar_data(
            gene_data_file, 
            Some("gene".into()), Some("padj".into()), 
            term_data_file, 
            None, None, 
            adj_file, 
            Some("symbol".into()), Some("go_id".into())
        );

        assert_eq!(
            gene_ontology.terms()[0].ref_data().deref(),
            &0.0
        );
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct GeneData {
        data: bool,
        activity: SaveableData<bool>,
    }
    impl Booleable for GeneData {
        fn try_into_bool(self) -> Option<bool> {
            Some(self.data)
        }
        fn from_bool(b: bool) -> Self {
            Self {
                data: b,
                activity: SaveableData::new(b, b)
            }
        }
    }
    impl Saveable for GeneData {
        type Output = bool;
    
        fn save(&mut self) {
            self.activity.save()
        }
    
        fn restore(&mut self) {
            self.activity.restore()
        }
    
        fn current(&self) -> Self::Output {
            self.activity.current()
        }
    
        fn saved(&self) -> Self::Output {
            self.activity.saved()
        }
    }
    impl Activeable for GeneData {
        fn is_active(&self) -> bool {
            self.current()
        }
    
        fn set_activity(&mut self, b: bool) {
            self.activity.set_activity(b);
        }
    
        fn is_legal(&self) -> bool {
            true
        }
    }
    
    #[derive(Debug, Default)]
    struct TermData {
        activity: SaveableData<bool>,
    }
    impl Saveable for TermData {
        type Output = bool;

        fn saved(&self) -> Self::Output {
            self.activity.saved()
        }
        fn current(&self) -> Self::Output {
            self.activity.current()
        }
        fn save(&mut self) {
            self.activity.save()
        }
        fn restore(&mut self) {
            self.activity.restore()
        }
    }
    impl Activeable for TermData {
        fn is_active(&self) -> bool {
            self.current()
        }
    
        fn set_activity(&mut self, b: bool) {
            self.activity.set_activity(b);
        }
    
        fn is_legal(&self) -> bool {
            true
        }
    }

    #[test]
    fn build_gene_list_model() {
        let gene_data_file = Some("./tests/data/atovaquone-rnaseq/results.csv".to_string());
        let term_data_file = None;
        let adj_file = "./tests/data/atovaquone-rnaseq/adjacency.csv".to_string();

        let gene_ontology = GeneOntology
            ::<TermData, GeneData>
            ::read_apply_scalar_data::<f64, _, bool, _>
        (
            gene_data_file, 
            Some("gene".into()), Some("padj".into()), 
            term_data_file, 
            None, None, 
            adj_file, 
            Some("symbol".into()), Some("go_id".into()),
            |x: f64| GeneData {
                data: x > 0.01,
                activity: SaveableData::new(false, false)
            }, 
            |x| TermData { activity: SaveableData::new(false, false) },
        );

        let mut rolemodel = GOGeneListRolemodel::new(
            gene_ontology,
            5,
            10,
            10,

            0.2,
            0.1,
            0.05,
            2.0,
        );

        rolemodel.draw_samples()

    }
}