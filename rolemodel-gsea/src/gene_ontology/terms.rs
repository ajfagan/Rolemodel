use crate::{
    rolemodel::saveable::Saveable, Whole, Node,
};

use std::cell::Cell;

#[derive(Debug)]
pub struct Term<Id, Td> 
where
    Td: Copy
{
    id: Id,
    data: Cell<Td>,
    genes: Vec<usize>,
}

impl<Id, Td> Node for Term<Id, Td> 
where 
    Td: Copy
{
    type Id = Id;
    type Data = Td;

    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn data(&self) -> &Cell<Self::Data> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Cell<Self::Data> {
        &mut self.data
    }
    
    fn idx_neighbors(&self) -> &Vec<usize> {
        &self.genes
    }
}

impl<Id, Td> Term<Id, Td> 
where 
    Td: Copy
{
    pub fn new(id: Id, data: Td, genes: Vec<usize>) -> Self {
        Self {
            id,
            data: Cell::new(data),
            genes,
        }
    }
    pub fn add_gene(&mut self, gene: usize) {
        self.genes.push(gene);
    }
    pub fn genes(&self) -> &Vec<usize> {
        &self.genes
    }
}

impl<Id, Td> Whole for Term<Id, Td> 
where
    Td: Copy
{ }

impl<Id, Td> Saveable for Term<Id, Td> 
where 
    Td: Saveable + Copy,
{
    type Output = Td::Output;

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