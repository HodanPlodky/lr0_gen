use std::collections::{HashMap, HashSet};

use crate::graph::lr0node::LR0Node;

#[derive(Debug)]
pub(crate) struct LRGraph<'a> {
    pub(crate) nodes: Vec<LR0Node<'a>>,
    pub(crate) edges: Vec<HashMap<char, usize>>,
}

impl<'a> LRGraph<'a> {
    pub(crate) fn new() -> Self {
        LRGraph {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn exist(&self, node: &LR0Node) -> (bool, usize) {
        for i in 0..(self.nodes.len()) {
            if self.nodes[i].eq(node) {
                return (true, i);
            }
        }

        (false, 0)
    }

    fn add_node(&mut self, node: LR0Node<'a>) -> usize {
        let mut node = node;
        node.create_closure();
        self.nodes.push(node);
        self.edges.push(HashMap::new());

        let index = self.nodes.len() - 1;

        let steps = self.nodes[index].get_steps();
        let g = self.nodes[index].gramm;

        for (c, rules) in steps {
            let nnode = LR0Node::new(HashSet::from_iter(rules.into_iter()), c, g);
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

    pub(crate) fn construct(&mut self, start_node: LR0Node<'a>) {
        self.add_node(start_node);
    }
}
