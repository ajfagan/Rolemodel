

#[cfg(test)]
mod tests {

    use rolemodel_gsea::{
        Activeable, GOGeneListRolemodel, Gene, GeneOntology, GeneSet, Node, Rolemodel, Saveable, SaveableData, Term, Whole
    };
    use rv::data::Booleable;


    #[test]
    fn basic_gene_ont() {
        let gene_ontology = GeneOntology::<usize, bool, bool>::new(
            vec![Gene::new(0, true, vec![0])],
            vec![Term::new(0, false, vec![0])]);

        assert_eq!(
            gene_ontology.wholes()[0].iter_parts(gene_ontology.parts()).next().unwrap().data().get(), 
            true
        );
    }

    #[test]
    fn larger_gene_ont() {
        let gene_ontology = GeneOntology::<usize, bool, bool>::from_incidence(
            vec![
                (0,true), 
                (1,false), 
                (2,true), 
                (3,true), 
                (4,true), 
                (5,false), 
                (6,false), 
                (7,false), 
                (8,false), 
                (9,true)
            ], 
            vec![
                (0,false), 
                (1,true), 
                (2,false), 
                (3,false)
            ],
            vec![
                (0,0), (0,1), (0,4),
                (1,1), (1,5), (1,8), (1,9),
                (2,1),
                (3,0), (3, 6),
            ],
        );

        let result: Vec<usize> = gene_ontology.iter_wholes().map(|whole| 
            whole.iter_parts(gene_ontology.genes()).fold(0, |curr, _| curr + 1)
        ).collect();

        assert_eq!(result, vec![3, 4, 1, 2]);
    }

    #[test]
    fn read_gene_ont() {
        let gene_data_file = Some("./tests/data/atovaquone-rnaseq/results.csv".to_string());
        let term_data_file = None;
        let adj_file = "./tests/data/atovaquone-rnaseq/adjacency.csv".to_string();

        let gene_ontology = GeneOntology::<String, f64, f64>::read_scalar_data(
            gene_data_file, 
            Some("gene".into()), Some("padj".into()), 
            term_data_file, 
            None, None, 
            adj_file, 
            Some("symbol".into()), Some("go_id".into())
        );

        assert_eq!(
            gene_ontology.terms()[0].data().get(),
            0.0
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
    
    #[derive(Debug, Default, Copy, Clone)]
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
            ::<String, TermData, GeneData>
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
            |_| TermData { activity: SaveableData::new(false, false) },
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