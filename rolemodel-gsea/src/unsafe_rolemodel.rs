
#[allow(dead_code)]

#[derive(Clone)]
struct IdxVec {
    vec: Vec<usize>,
}
impl IdxVec {
    fn pop(&mut self) -> Option<usize> {
        self.vec.pop()
    }
    fn get(&self, idx: usize) -> Option<&usize> {
        self.vec.get(idx)
    }
}


#[allow(dead_code)]
struct IdxVecIter<'a, D> {
    vec: &'a IdxVec,
    target: &'a Vec<D>,
    curr_idx: usize,
}
impl<'a, D> Iterator for IdxVecIter<'a, D> {
    type Item = &'a D;

    fn next(&mut self) -> Option<&'a D> {
        if let Some(&next_idx) = self.vec.get(self.curr_idx) {
            if next_idx < self.target.len() {
                self.curr_idx += 1;
                return self.target.get(next_idx)
            } else {
                panic!("Attempted to access vector of length {} at index {}", self.target.len(), next_idx)
            }
        } else {
            return None
        }
    }
}

#[allow(dead_code)]
impl<'a, D> IdxVecIter<'a, D> {
    pub fn new(vec: &'a IdxVec, target: &'a Vec<D>) -> Self {
        Self {
            vec,
            target,
            curr_idx: 0,
        }
    }
}

#[allow(dead_code)]
struct IdxVecIterMut<'a, D> {
    vec: &'a IdxVec,
    target: &'a mut Vec<D>,
    curr_idx: usize,
}
impl<'a, D: 'a> Iterator for IdxVecIterMut<'a, D> {
    type Item = &'a mut D;

    fn next(&mut self) -> Option<&'a mut D> {
        if let Some(&next_idx) = self.vec.get(self.curr_idx) {
            if next_idx < self.target.len() {
                self.curr_idx += 1;
                // if let Some(t) = self.target.get_mut(next_idx) {
                //     return Some(t);
                // } else {
                //     panic!()
                // }
                let ptr = self.target.as_mut_ptr();
                Some(unsafe {
                    &mut *ptr.add(next_idx)
                })
            } else {
                panic!("Attempted to access vector of length {} at index {}", self.target.len(), next_idx)
            }
        } else {
            return None
        }
    }
}

#[allow(dead_code)]
impl<'a, D> IdxVecIterMut<'a, D> {
    pub fn new(vec: &'a IdxVec, target: &'a mut Vec<D>) -> Self {
        Self {
            vec,
            target,
            curr_idx: 0,
        }
    }
}

#[allow(dead_code)]
impl IdxVec {
    fn iter_over<'a, D>(&'a self, target: &'a mut Vec<D>) -> IdxVecIter<'a, D> {
        IdxVecIter::new(&self, target)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct Node<D> {
    data: D,
    neighbors: IdxVec,
}

#[allow(dead_code)]
impl<D> Node<D> {
    fn iter_neighbors<'a, Dother>(&'a self, neighbors: &'a Vec<Node<Dother>>) -> impl Iterator<Item = &'a Node<Dother>> {

        IdxVecIter::new(&self.neighbors, neighbors)
    }
    fn iter_neighbors_mut<'a, Dother>(&'a self, neighbors: &'a mut Vec<Node<Dother>>) -> impl Iterator<Item = &'a mut Node<Dother>> {

        IdxVecIterMut::new(&self.neighbors, neighbors)
    }
}


#[allow(dead_code)]
struct GeneOntGraph<Td, Gd> {
    terms: Vec<Node<Td>>,
    genes: Vec<Node<Gd>>,
}

#[allow(dead_code)]
impl<Td, Gd> GeneOntGraph<Td, Gd> 
where 
    Td: Clone,
    Gd: Clone,
{
    pub fn iter_terms(&self) -> impl Iterator<Item = &Node<Td>> {
        self.terms.iter()
    }
    pub fn iter_terms_mut(&mut self) -> impl Iterator<Item = &mut Node<Td>> {
        self.terms.iter_mut()
    }
    pub fn iter_term_neighbors<'a>(&'a self, term: &'a Node<Td>) -> impl Iterator<Item = &'a Node<Gd>> {
        term.iter_neighbors(&self.genes)
    }
    pub fn iter_term_neighbors_mut<'a>(&'a mut self, term: &'a Node<Td>) -> impl Iterator<Item = &'a mut Node<Gd>> {
        term.iter_neighbors_mut(&mut self.genes)
    }

    pub fn iter_genes(&self) -> impl Iterator<Item = &Node<Gd>> {
        self.genes.iter()
    }
    pub fn iter_genes_mut(&mut self) -> impl Iterator<Item = &mut Node<Gd>> {
        self.genes.iter_mut()
    }
    pub fn iter_gene_neighbors<'a>(&'a self, gene: &'a Node<Gd>) -> impl Iterator<Item = &'a Node<Td>> {
        gene.iter_neighbors(&self.terms)
    }
    pub fn iter_gene_neighbors_mut<'a>(&'a mut self, gene: &'a Node<Gd>) -> impl Iterator<Item = &'a mut Node<Td>> {
        gene.iter_neighbors_mut(&mut self.terms)
    }
}
