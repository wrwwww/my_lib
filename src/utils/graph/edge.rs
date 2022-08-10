use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use super::node::Node;

#[derive(Debug)]
pub(crate) struct Edge {
    pub to: Rc<RefCell<Node>>,
    pub from: Rc<RefCell<Node>>,
    pub weight: i32,
}
impl Edge {
    pub fn new(to: Rc<RefCell<Node>>, from: Rc<RefCell<Node>>, weight: i32) -> Self {
        Self { to, from, weight }
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from || self.from == other.to || self.to == other.to
    }
}
impl Eq for Edge {}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

