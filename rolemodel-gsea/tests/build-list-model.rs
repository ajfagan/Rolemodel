

#[cfg(test)]
mod tests {

    use std::{
        rc::Rc,
        cell::RefCell,
    };

    use rolemodel_gsea::{
        Gene, GeneOntology, GeneSet, Term, Whole, Part,
    };


    #[test]
    fn basic_gene_ont() {
        let mut gene_ontology = GeneOntology::<bool, bool>::new(
            vec![Rc::new(RefCell::new(Gene::new(true, vec![])))],
            vec![]);
        let gene = gene_ontology.genes()[0].clone();
        gene_ontology.mut_terms().push(Rc::new(RefCell::new(Term::new(false, vec![gene]))));

        assert_eq!(
            *gene_ontology.iter_wholes().next().unwrap().borrow().parts().next().unwrap().data().unwrap(), 
            true
        );
    }

    #[test]
    fn large_gene_ont() {
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
}