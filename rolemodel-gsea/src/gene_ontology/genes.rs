use crate::{
    rolemodel::saveable::Saveable, 
    Node, Part
};

use std::cell::Cell;

#[derive(Debug)]
pub struct Gene<Id, Gd> 
where 
    Gd: Copy
{
    id: Id,
    data: Cell<Gd>,
    terms: Vec<usize>,
}

impl<Id, Gd> Gene<Id, Gd> 
where 
    Gd: Copy
{
    pub fn new(id: Id, data: Gd, terms: Vec<usize>) -> Self {
        Self {
            id,
            data: Cell::new(data),
            terms,
        }
    }
    pub fn terms(&self) -> &Vec<usize> {&self.terms}
    pub fn add_term(&mut self, term: usize) {self.terms.push(term)} 
}

impl<Id, Gd> Node for Gene<Id, Gd> 
where 
    Gd: Copy
{
    type Id = Id;
    type Data = Gd;

    fn data(&self) -> &Cell<Self::Data> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Cell<Self::Data> {
        &mut self.data
    }
    fn id(&self) -> &Id {&self.id}
    
    fn idx_neighbors(&self) -> &Vec<usize> {
        &self.terms
    }
    // fn iter_neighbors(&self) -> impl Iterator<Item = Self::NeighborType> {
    //     self.terms.clone().into_iter()
    // }
}

impl<Id, Gd> Part for Gene<Id, Gd> 
where
    Gd: Copy 
{ 

}

impl<Id, Gd> Saveable for Gene<Id, Gd> 
where 
    Gd: Saveable + Copy,
{
    type Output = Gd::Output;

    fn save(&mut self) {
        self.data.get_mut().save();
    }
    fn restore(&mut self) {
        self.data.get_mut().restore();
    }
    fn current(&self) -> Self::Output {
        self.data.get().current()
    }
    fn saved(&self) -> Self::Output {
        self.data.get().saved()
    }
}
