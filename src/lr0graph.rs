use std::collections::HashMap;

use crate::lr0node::LR0Node;

pub(crate) struct LR0Graph<'a> {
    nodes: Vec<LR0Node<'a>>,
    edges: Vec<HashMap<char, usize>>,
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

    pub(crate) fn add_node(&mut self, node: LR0Node<'a>) -> bool {
        let mut node = node;
        let (e, _) = self.exist(&node);
        if e {
            return false;
        }

        node.create_closure();
        self.nodes.push(node);

        return true;
    }
}
