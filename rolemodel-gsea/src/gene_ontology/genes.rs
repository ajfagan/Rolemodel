use crate::{
    gene_ontology::terms::Term,
    Part,
    rolemodel::saveable::Saveable,
};

use std::{
    cell::{
        Ref, RefCell,
    },
    rc::Rc,
};

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

impl<Td, Gd> Part for Gene<Td, Gd> {
    type Data = Gd;
    type WholeNode = Rc<RefCell<Term<Td, Gd>>>;

    fn borrow_data(&self) -> Option<&Self::Data> {
        Some(&self.data)
    }
    fn data(&self) -> Option<Ref<Self::Data>> {
        None
    }
    fn wholes(&self) -> impl Iterator<Item = Self::WholeNode> {
        self.terms.iter().cloned()
    }
}

impl<Td, Gd> Saveable for Gene<Td, Gd> 
where 
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

impl<Td, Gd> Part for Rc<RefCell<Gene<Td, Gd>>> {
    type Data = Gd;
    type WholeNode = Rc<RefCell<Term<Td, Gd>>>;

    fn borrow_data(&self) -> Option<&Self::Data> {
        None
    }
    fn data(&self) -> Option<Ref<Self::Data>> {
        Some(Ref::map(self.borrow(), |whole| &whole.data))
    }
    fn wholes(&self) -> impl Iterator<Item = Self::WholeNode> {
        self.borrow().terms.clone().into_iter()
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
        self.data().unwrap().current()
    }
    fn saved(&self) -> Self::Output {
        self.data().unwrap().saved()
    }
}