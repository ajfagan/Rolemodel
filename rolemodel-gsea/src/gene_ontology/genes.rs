use crate::{
    gene_ontology::terms::Term, 
    rolemodel::saveable::Saveable, 
    Activeable, 
    Node, Part
};

use std::{
    cell::{
        Ref, RefCell,
    },
    rc::Rc,
};

#[derive(Debug)]
pub struct Gene<Td, Gd> {
    data: Gd,
    terms: Vec<Rc<RefCell<Term<Td, Gd>>>>,
}

impl<Td, Gd> Gene<Td, Gd> {
    pub fn new(data: Gd, terms: Vec<Rc<RefCell<Term<Td, Gd>>>>) -> Self {
        Self {
            data,
            terms,
        }
    }
    pub fn terms(&self) -> &Vec<Rc<RefCell<Term<Td, Gd>>>> {&self.terms}
    pub fn add_term(&mut self, term: Rc<RefCell<Term<Td, Gd>>>) {self.terms.push(term)} 
}

impl<Td, Gd> Node for Gene<Td, Gd> {
    type Data = Gd;
    type NeighborType = Rc<RefCell<Term<Td, Gd>>>;

    fn data(&self) -> &Self::Data {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }
    fn ref_data(&self) -> Ref<Self::Data> {
        panic!("Never call ref_data on items not in a RefCell");
    }
    fn iter_neighbors(&self) -> impl Iterator<Item = Self::NeighborType> {
        self.terms.clone().into_iter()
    }
}
impl<Td, Gd> Node for Rc<RefCell<Gene<Td, Gd>>> {
    type Data = Gd;
    type NeighborType = Rc<RefCell<Term<Td, Gd>>>;

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
        self.borrow().terms.clone().into_iter()
    }
}

impl<Td, Gd> Part for Gene<Td, Gd> { }
impl<Td, Gd> Part for Rc<RefCell<Gene<Td, Gd>>> { }

impl<Td, Gd> Saveable for Gene<Td, Gd> 
where 
    Td: Saveable,
    Gd: Saveable,
{
    type Output = Gd::Output;

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

impl<Td, Gd> Saveable for Rc<RefCell<Gene<Td, Gd>>> 
where 
    Gd: Saveable,
{
    type Output = Gd::Output;

    fn save(&mut self) {
        self.borrow_mut().data.save();
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