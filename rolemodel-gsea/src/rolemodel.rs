pub mod genelist;
pub mod saveable;
pub mod activeable;

use crate::{Activeable, Part, Whole};



pub trait Rolemodel {

    fn burn_in(&self) -> usize ;
    fn nsamples(&self) -> usize;
    fn thinning(&self) -> usize;

    fn draw_samples(&mut self);

    fn calc_posterior_llikelihood(&mut self) -> f64;
    fn posterior_llikelihood(&self) -> f64;

    fn calc_prior_llikelihood(&mut self) -> f64;
    fn prior_llikelihood(&self) -> f64;

    fn calc_data_llikelihood(&mut self) -> f64;
    fn data_llikelihood(&self) -> f64;
    
}