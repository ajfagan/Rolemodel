pub mod genelist;
pub mod saveable;

use crate::{Part, Whole};


pub trait RmPart: Part {
    fn is_active(&self) -> bool;
}
pub trait RmWhole: Whole {
    fn is_illegal(&self) -> bool;
}



pub trait Rolemodel {

    fn burn_in(&self) -> usize ;
    fn nsamples(&self) -> usize;
    fn thinning(&self) -> usize;

    fn draw_samples(&mut self);

    fn calc_posterior_llikelihood(&mut self) -> f64;
    fn posterior_llikelihood(&self) -> f64;

    fn calc_prior_llikelihood(&mut self) -> f64;
    fn prior_llikelihood(&self) -> f64;

    fn calc_data_llikelihood(&self) -> f64;
    fn data_llikelihood(&self) -> f64;
}