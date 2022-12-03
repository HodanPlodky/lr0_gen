use std::{collections::{HashMap, HashSet}, marker::PhantomData};

use super::{lr0rule::LR0Rule, lr1graph::{LR1Rule, LR1Node}, lrnode::LRNode, rule::LRRule, lr0node::LR0Node};

pub type LR0Graph<'a> = LRGraph<'a, LR0Node<'a>, LR0Rule>;
pub type LR1Graph<'a> = LRGraph<'a, LR1Node<'a>, LR1Rule>;

#[derive(Debug)]
pub struct LRGraph<'a, T, R>
where
    R: LRRule,
    T: LRNode<'a, R>,
{
    pub(crate) nodes: Vec<T>,
    pub(crate) edges: Vec<HashMap<char, usize>>,
    phantom : PhantomData<&'a R>,
}

impl<'a, T, R> LRGraph<'a, T, R>
where
    T: LRNode<'a, R>,
    R: LRRule,
{
    pub(crate) fn new() -> Self {
        LRGraph {
            nodes: vec![],
            edges: vec![],
            phantom : PhantomData,
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

    pub(crate) fn construct(&mut self, start_node: T) {
        self.add_node(start_node);
    }
}
