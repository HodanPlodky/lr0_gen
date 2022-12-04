use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use super::{
    lr0node::LR0Node,
    lr0rule::LR0Rule,
    lr1graph::{LR1Node, LR1Rule},
    lrnode::LRNode,
    rule::LRRule,
};

pub trait LRGraph<'a, T, R>
where
    T: LRNode<'a, R>,
    R: LRRule,
{
    // data
    fn nodes(&self) -> &Vec<T>;
    fn edges(&self) -> &Vec<HashMap<char, usize>>;
    // behavior
    fn new() -> Self;
    fn exist(&self, node: &T) -> (bool, usize);
    fn add_node(&mut self, node: T) -> usize;
    fn construct(&mut self, start_node: T) {
        self.add_node(start_node);
    }
}

pub trait LRFollowGraph<'a, T>: LRGraph<'a, T, LR1Rule>
where
    T: LRNode<'a, LR1Rule>,
{
}

pub type LR0Graph<'a> = LRGraphStruct<'a, LR0Node<'a>, LR0Rule>;
pub type LR1Graph<'a> = LRGraphStruct<'a, LR1Node<'a>, LR1Rule>;

impl<'a> LRFollowGraph<'a, LR1Node<'a>> for LR1Graph<'a> {}

#[derive(Debug)]
pub struct LRGraphStruct<'a, T, R>
where
    R: LRRule,
    T: LRNode<'a, R>,
{
    nodes: Vec<T>,
    edges: Vec<HashMap<char, usize>>,
    phantom: PhantomData<&'a R>,
}

impl<'a, T, R> LRGraphStruct<'a, T, R>
where
    T: LRNode<'a, R>,
    R: LRRule,
{
    pub fn insert_node(&mut self, index: usize, node : T) {
        self.nodes[index] = node;
    }

    pub fn insert_edge(&mut self, index: usize, c : char, i : usize) {
        self.edges[index].insert(c, i);
    }

    pub fn push_node(&mut self, node : T) {
        self.nodes.push(node);
    }

    pub fn push_edge(&mut self, item : HashMap<char, usize>) {
        self.edges.push(item);
    } 
}

impl<'a, T, R> LRGraph<'a, T, R> for LRGraphStruct<'a, T, R>
where
    T: LRNode<'a, R>,
    R: LRRule,
{
    fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            phantom: PhantomData,
        }
    }

    fn exist(&self, node: &T) -> (bool, usize) {
        for i in 0..(self.nodes.len()) {
            if self.nodes[i].eq(node) {
                return (true, i);
            }
        }

        (false, 0)
    }

    fn add_node(&mut self, node: T) -> usize {
        let mut node = node;
        node.create_closure();
        self.nodes.push(node);
        self.edges.push(HashMap::new());

        let index = self.nodes.len() - 1;

        let steps = self.nodes[index].get_steps();
        let g = self.nodes[index].gramm();

        for (c, rules) in steps {
            let nnode = T::new(HashSet::from_iter(rules.into_iter()), c, g);
            let (e, i) = self.exist(&nnode);
            if e {
                self.edges[index].insert(c, i);
                continue;
            }
            let i = self.add_node(nnode);
            self.edges[index].insert(c, i);
        }
        index
    }

    fn nodes(&self) -> &Vec<T> {
        &self.nodes
    }

    fn edges(&self) -> &Vec<HashMap<char, usize>> {
        &self.edges
    }
}
