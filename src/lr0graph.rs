use std::collections::{HashMap, HashSet};

use crate::lr0node::LR0Node;

#[derive(Debug)]
pub(crate) struct LR0Graph<'a> {
    pub(crate) nodes: Vec<LR0Node<'a>>,
    pub(crate) edges: Vec<HashMap<char, usize>>,
}

impl<'a> LR0Graph<'a> {
    pub(crate) fn new() -> Self {
        LR0Graph {
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
            let (e, _) = self.exist(&nnode);
            if e {
                continue;
            }
            let tmp = self.add_node(nnode);
            self.edges[index].insert(c, tmp);
        }
        index
    }

    pub(crate) fn construct(&mut self, start_node: LR0Node<'a>) {
        self.add_node(start_node);
    }
}
