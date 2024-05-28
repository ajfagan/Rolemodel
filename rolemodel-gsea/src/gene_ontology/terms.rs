use crate::{
    gene_ontology::genes::Gene, rolemodel::saveable::Saveable, Whole,
};

use std::{
    cell::{
        Ref,
        RefCell
    },
    rc::Rc,
};

pub struct Term<Td, Gd> {
    data: Td,
    genes: Vec<Rc<RefCell<Gene<Td, Gd>>>>,
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

impl<Td, Gd> Whole for Term<Td, Gd> {
    type Data = Td;
    type PartNode = Rc<RefCell<Gene<Td, Gd>>>;

    fn borrow_data(&self) -> Option<&Self::Data> {
        Some(&self.data)
    }
    fn data(&self) -> Option<Ref<Self::Data>> {
        None
    }
    fn parts(&self) -> impl Iterator<Item = Self::PartNode> {
        self.genes.iter().cloned()
    }
}

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

impl<Td, Gd> Whole for Rc<RefCell<Term<Td, Gd>>> {
    type Data = Td;
    type PartNode = Rc<RefCell<Gene<Td, Gd>>>;

    fn borrow_data(&self) -> Option<&Self::Data> {
        None
    }
    fn data(&self) -> Option<Ref<Self::Data>> {
        Some(Ref::map(self.borrow(), |whole| &whole.data))
    }
    fn parts(&self) -> impl Iterator<Item = Self::PartNode> {
        self.borrow().genes.clone().into_iter()
    }
}

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
        self.data().unwrap().current()
    }
    fn saved(&self) -> Self::Output {
        self.data().unwrap().saved()
    }
}
