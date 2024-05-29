use crate::{
    gene_ontology::genes::Gene, rolemodel::saveable::Saveable, Activeable, Part, Whole, Node,
};

use std::{
    cell::{
        Ref,
        RefCell
    },
    rc::Rc,
    ops::Deref,
};

#[derive(Debug)]
pub struct Term<Td, Gd> {
    data: Td,
    genes: Vec<Rc<RefCell<Gene<Td, Gd>>>>,
}

impl<Td, Gd> Node for Term<Td, Gd> {
    type Data = Td;
    type NeighborType = Rc<RefCell<Gene<Td, Gd>>>;

    fn data(&self) -> &Self::Data {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }
    fn ref_data(&self) -> Ref<Self::Data> {
        panic!("Never call ref_data on items not in a RefCell - call data instead");
    }
    fn iter_neighbors(&self) -> impl Iterator<Item = Self::NeighborType> {
        self.genes.clone().into_iter()
    }
}

impl<Td, Gd> Node for Rc<RefCell<Term<Td, Gd>>> {
    type Data = Td;
    type NeighborType = Rc<RefCell<Gene<Td, Gd>>>;

    fn data(&self) -> &Self::Data {
        panic!("Cannot borrow data from inside a RefCell - use ref_data instead")
    }
    fn data_mut(&mut self) -> &mut Self::Data {
        panic!("Cannot borrow data from inside a RefCell - use ref_data_mut instead")
    }
    fn ref_data(&self) -> Ref<Self::Data> {
        Ref::map(self.borrow(), |whole| &whole.data)
    }
    
    fn iter_neighbors(&self) -> impl Iterator<Item = Self::NeighborType> {
        self.borrow().genes.clone().into_iter()
    }
}

impl<Td, Gd> Term<Td, Gd> {
    pub fn new(data: Td, genes: Vec<Rc<RefCell<Gene<Td, Gd>>>>) -> Self {
        Self {
            data,
            genes,
        }
    }
    pub fn add_gene(&mut self, gene: Rc<RefCell<Gene<Td, Gd>>>) {
        self.genes.push(gene);
    }
}

impl<Td, Gd> Whole for Term<Td, Gd> { }

impl<Td, Gd> Saveable for Term<Td, Gd> 
where 
    Td: Saveable,
{
    type Output = Td::Output;

    fn save(&mut self) {
        self.data.save();
    }
    fn restore(&mut self) {
        self.data.restore();
    }
    fn current(&self) -> Self::Output {
        self.data.current()
    }
    fn saved(&self) -> Self::Output {
        self.data.saved()
    }
}

impl<Td, Gd> Whole for Rc<RefCell<Term<Td, Gd>>> { }

impl<Td, Gd> Saveable for Rc<RefCell<Term<Td, Gd>>> 
where 
    Td: Saveable,
{
    type Output = Td::Output;

    fn save(&mut self) {
        (self.borrow_mut().data.save());
    }
    fn restore(&mut self) {
        self.borrow_mut().data.restore();
    }
    fn current(&self) -> Self::Output {
        self.data().current()
    }
    fn saved(&self) -> Self::Output {
        self.data().saved()
    }
}
