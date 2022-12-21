use std::collections::{HashMap, HashSet};

use super::{
    lalrnode::LALRNode,
    lr1graph::LR1Rule,
    lrgraph::{LRFollowGraph, LRGraph, LRGraphStruct},
    lrnode::LRNode,
};

// Because of different creating of graph
// LALR must be distinct from others
pub struct LALRGraph<'a> {
    lr1_graph: LRGraphStruct<'a, LALRNode<'a>, LR1Rule>,
}

impl<'a> LALRGraph<'a> {
    fn update(&mut self, index: usize, new_node: &LALRNode<'a>) {
        let old_node = &self.lr1_graph.nodes()[index];
        let (mut union_node, conti) = old_node.union_nodes(new_node);

        if !conti {
            return;
        }

        union_node.create_closure();
        self.lr1_graph.insert_node(index, union_node);

        let steps = self.lr1_graph.nodes()[index].get_steps();
        let g = self.lr1_graph.nodes()[index].gramm();

        for (c, rules) in steps {
            let nnode = LALRNode::new(HashSet::from_iter(rules.into_iter()), c, g);
            let (e, i) = self.exist(&nnode);
            if e {
                self.lr1_graph.insert_edge(index, c, i);
                self.update(i, &nnode);
                continue;
            }
            let i = self.add_node(nnode);
            self.lr1_graph.insert_edge(index, c, i);
        }
    }
}

impl<'a> LRGraph<'a, LALRNode<'a>, LR1Rule> for LALRGraph<'a> {
    fn new() -> Self {
        Self {
            lr1_graph: LRGraphStruct::new(),
        }
    }

    fn exist(&self, node: &LALRNode<'a>) -> (bool, usize) {
        self.lr1_graph.exist(node)
    }

    fn add_node(&mut self, node: LALRNode<'a>) -> usize {
        let mut node = node;
        node.create_closure();
        self.lr1_graph.push_node(node);
        self.lr1_graph.push_edge(HashMap::new());

        let index = self.lr1_graph.nodes().len() - 1;
        let steps = self.lr1_graph.nodes()[index].get_steps();
        let g = self.lr1_graph.nodes()[index].gramm();

        for (c, rules) in steps {
            let nnode = LALRNode::new(HashSet::from_iter(rules.into_iter()), c, g);
            let (e, i) = self.exist(&nnode);
            if e {
                self.lr1_graph.insert_edge(index, c, i);
                self.update(i, &nnode);
                continue;
            }
            let i = self.add_node(nnode);
            self.lr1_graph.insert_edge(index, c, i);
        }
        index
    }

    fn nodes(&self) -> &Vec<LALRNode<'a>> {
        self.lr1_graph.nodes()
    }

    fn edges(&self) -> &Vec<HashMap<char, usize>> {
        self.lr1_graph.edges()
    }
}

impl<'a> LRFollowGraph<'a, LALRNode<'a>> for LALRGraph<'a> {}
