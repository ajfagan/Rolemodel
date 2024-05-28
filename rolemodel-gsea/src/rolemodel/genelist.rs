use rv::data::Booleable;
use crate::{
    GeneSet, Part, Whole
};

use super::{
    saveable::Saveable, 
    Rolemodel,
    RmPart, RmWhole,
};

pub trait GeneListEnrichment: GeneSet
where 
    <Self as GeneSet>::PartNode: Part,
    <Self as GeneSet>::WholeNode: Whole,
    <Self::PartNode as Part>::Data: Booleable,
    <Self::WholeNode as Whole>::Data: Booleable,
{
}


pub trait GeneListRolemodel: GeneSet
where 
    <Self as GeneSet>::PartNode: RmPart,
    <<Self as GeneSet>::PartNode as Part>::Data: Booleable,
    <<Self as GeneSet>::PartNode as Part>::WholeNode: Whole,
    <<<Self as GeneSet>::PartNode as Part>::WholeNode as Whole>::Data: Booleable,
    <Self as GeneSet>::WholeNode: RmWhole,
    <<Self as GeneSet>::WholeNode as Whole>::Data: Booleable,
{
    fn burn_in(&self) -> usize;
    fn nsamples(&self) -> usize;
    fn thinning(&self) -> usize;

    fn set_activity_probability(&self) -> f64;
    fn true_active_gene_hit_rate(&self) -> f64;
    fn false_inactive_gene_hit_rate(&self) -> f64;
    fn illegal_set_penalty(&self) -> f64;

    fn llikelihood_wholes(&self) -> f64 {
        let mut n_illegal = 0.0;
        self.iter_wholes()
            .map(|whole| {
                if whole.is_illegal() {n_illegal += 1.0}
                match whole.data().unwrap().try_into_bool().expect("Input data should be binary 0/1") {
                    true => {
                        self.set_activity_probability().ln()
                    },
                    false => {
                        (1.0 - self.set_activity_probability()).ln()
                    },
                }
            })
            .sum::<f64>() - self.illegal_set_penalty() * n_illegal
    }
    fn llikelihood_parts(&self) -> f64 {
        self.iter_parts()
            .map(|part| {
                let activity = part.is_active();
                match (activity, part.data().unwrap().try_into_bool().expect("Input data should be binary 0/1")) {
                    (true, true) => self.true_active_gene_hit_rate().ln(),
                    (true, false) => (1.0 - self.true_active_gene_hit_rate()).ln(),
                    (false, true) => self.false_inactive_gene_hit_rate().ln(),
                    (false, false) => (1.0 - self.false_inactive_gene_hit_rate()).ln()
                }
            })
            .sum()
    }
}


impl<G> Rolemodel for G
where 
    G: GeneListRolemodel + Saveable,
    <G as GeneSet>::PartNode: RmPart,
    <<G as GeneSet>::PartNode as Part>::Data: Booleable,
    <<G as GeneSet>::PartNode as Part>::WholeNode: Whole,
    <<<G as GeneSet>::PartNode as Part>::WholeNode as Whole>::Data: Booleable,
    <G as GeneSet>::WholeNode: RmWhole,
    <<G as GeneSet>::WholeNode as Whole>::Data: Booleable,
{


    fn draw_samples(&mut self) {
        (0..self.nsamples()).for_each(|_| {
            self.save();

            let _ = self.calc_posterior_llikelihood();
            todo!()
        }); 
    }

    fn calc_posterior_llikelihood(&mut self) -> f64 {
        todo!()
    }
    fn posterior_llikelihood(&self) -> f64 {
        todo!()
    }

    fn calc_prior_llikelihood(&mut self) -> f64 {
        todo!()
    }
    fn prior_llikelihood(&self) -> f64 {
        todo!()
    }

    fn calc_data_llikelihood(&self) -> f64 {
        todo!()
    }
    fn data_llikelihood(&self) -> f64 {
        todo!()
    }
    
    fn burn_in(&self) -> usize  {
        self.burn_in()
    }
    
    fn nsamples(&self) -> usize {
        self.nsamples()
    }
    
    fn thinning(&self) -> usize {
        self.thinning()
    }
}