use std::collections::{HashMap, HashSet};

use super::{lrnode::LRNode, lr0rule::LR0Rule, rule::LRRule, lr1graph::LR1Rule};

pub type LR0Graph<'a> = LRGraph<'a, LR0Rule>;
pub type LR1Graph<'a> = LRGraph<'a, LR1Rule>;

#[derive(Debug)]
pub struct LRGraph<'a, T> where T: LRRule {
    pub(crate) nodes: Vec<LRNode<'a, T>>,
    pub(crate) edges: Vec<HashMap<char, usize>>,
}

impl<'a, T> LRGraph<'a, T> where T: LRRule {
    pub(crate) fn new() -> Self {
        LRGraph {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn exist(&self, node: &LRNode<T>) -> (bool, usize) {
        for i in 0..(self.nodes.len()) {
            if self.nodes[i].eq(node) {
                return (true, i);
            }
        }

        (false, 0)
    }

    fn add_node(&mut self, node: LRNode<'a, T>) -> usize {
        let mut node = node;
        node.create_closure();
        self.nodes.push(node);
        self.edges.push(HashMap::new());

        let index = self.nodes.len() - 1;

        let steps = self.nodes[index].get_steps();
        let g = self.nodes[index].gramm;

        for (c, rules) in steps {
            let nnode = LRNode::new(HashSet::from_iter(rules.into_iter()), c, g);
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

    pub(crate) fn construct(&mut self, start_node: LRNode<'a, T>) {
        self.add_node(start_node);
    }
}
