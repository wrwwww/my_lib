use super::edge::Edge;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub(crate) struct Node {
    pub edges: Vec<Rc<RefCell<Edge>>>,
    pub id: i32,
}
impl Node {
    pub fn new(id: i32) -> Self {
        Self {
            edges: Vec::new(),
            id,
        }
    }
    pub fn push(&mut self, ele: Rc<RefCell<Edge>>) {
        self.edges.push(ele)
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}
