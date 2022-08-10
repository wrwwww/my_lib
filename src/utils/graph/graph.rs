use crate::utils::PreQueue;

use super::edge::Edge;
use super::node::Node;
use std::{cell::RefCell, collections::HashSet, rc::Rc};
/// 图结构
/// 一个点有多条边
/// 多个点有很多重&复的边
/// 需要使用Rc 和 Refcell
#[derive(Debug)]
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
    edges: Vec<Rc<RefCell<Edge>>>,
}
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn push(&mut self, from: i32, to: i32, weight: i32) {
        let mut from1 = Rc::new(RefCell::new(Node::new(from)));
        let mut to1 = Rc::new(RefCell::new(Node::new(to)));
        let mut flag1 = true;
        let mut flag2 = true;
        for i in 0..self.nodes.len() {
            if (*self.nodes[i]).borrow().id == from {
                from1 = Rc::clone(&self.nodes[i]);
                flag1 = false;
            } else if (*self.nodes[i]).borrow().id == to {
                to1 = Rc::clone(&self.nodes[i]);
                flag2 = false;
            }
        }
        if flag1 {
            self.nodes.push(Rc::clone(&from1));
        }
        if flag2 {
            self.nodes.push(Rc::clone(&to1));
        }
        let edge = Rc::new(RefCell::new(Edge::new(
            Rc::clone(&from1),
            Rc::clone(&to1),
            weight,
        )));
        (*from1).borrow_mut().push(Rc::clone(&edge));
        (*to1).borrow_mut().push(Rc::clone(&edge));
        self.edges.push(Rc::clone(&edge));
    }
    pub fn min_struct_tree(&mut self) {
        let mut queue = PreQueue::new();
        // 将所有边进入优先队列
        for i in 0..self.edges.len() {
            queue.push(Rc::clone(&self.edges[i]));
        }
        // 每次从优先队列中取出
    }
    pub fn prim(&mut self) {
        unsafe {
            let mut list = HashSet::new();
            let mut ans = HashSet::new();
            list.insert(self.nodes[0].as_ptr());
            // 问题两个点 应该添加那个点才不会重复添加点?
            // 问题 怎么操作使得边不会选择已经选择的边?
            while ans.len() < self.nodes.len() - 1 {
                // 遍历点的集合
                // 找到这些点的集合中最小的边
                // 找到后添加到答案
                let mut min = i32::MAX;
                let mut min_edge = (*self.nodes[0].as_ptr()).edges[0].as_ptr();

                for i in list.iter() {
                    let i = *i;
                    for j in 0..(*i).edges.len() {
                        // let s=&((*i).edges[j]).as_ptr();
                        // 没有添加的边 且权值比较低
                        // 并且这条边的点最好不在set内
                        let val = (*(*i).edges[j]).borrow_mut().weight;
                        // let s=(*(*i).edges[j].as_ptr()).to.as_ptr();
                        
                        if val < min && !ans.contains(&((*i).edges[j]).as_ptr())&&(!list.contains(&(*(*i).edges[j].as_ptr()).from.as_ptr())||!list.contains(&(*(*i).edges[j].as_ptr()).to.as_ptr())) {
                            min_edge = (*i).edges[j].as_ptr();
                            min = val;
                        }
                    }
                }
                list.insert((*min_edge).from.as_ptr());
                list.insert((*min_edge).to.as_ptr());
                ans.insert(min_edge);
            }
            println!("点集的长度:{}", list.len());
            println!("边集的长度:{}", ans.len());
            for i in list {
                print!("{}\t", (*i).id);
            }
            println!("");
            for i in ans {
                print!("地点一:{}\t", (*(*i).from.as_ptr()).id);
                print!("地点二{}\t", (*(*i).to.as_ptr()).id);
                print!("权值:{}\n", (*i).weight);
            }
        }
    }
}

#[test]
fn test1() {
    let mut arr = Graph::new();
    arr.push(1, 2, 6);
    arr.push(1, 4, 5);
    arr.push(1, 3, 1);
    arr.push(2, 3, 5);
    arr.push(2, 5, 3);
    arr.push(3, 4, 5);
    arr.push(3, 5, 6);
    arr.push(3, 6, 4);
    arr.push(4, 6, 2);
    arr.push(5, 6, 6);
    // println!("{:?}",arr);
    println!("点集个数:{}", arr.nodes.len());
    println!("边的个数:{}", arr.edges.len());
    let mut queue = PreQueue::new();
    for i in 0..arr.edges.len() {
        queue.push(Rc::clone(&arr.edges[i]));
    }
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", (*queue.pop().unwrap()).borrow().weight);
    println!("{:?}", queue.pop());
}
#[test]
fn test() {
    let mut arr = Graph::new();
    arr.push(1, 2, 6);
    arr.push(1, 4, 5);
    arr.push(1, 3, 1);
    arr.push(2, 3, 5);
    arr.push(2, 5, 3);
    arr.push(3, 4, 5);
    arr.push(3, 5, 6);
    arr.push(3, 6, 4);
    arr.push(4, 6, 2);
    arr.push(5, 6, 6);
    println!("点集个数:{}", arr.nodes.len());
    println!("边的个数:{}", arr.edges.len());
    arr.prim();
}
