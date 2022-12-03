use super::{lalrnode::LALRNode, lrgraph::LRGraph, rule::LRRule, lr1graph::LR1Rule};

// Because of different creating of graph
// LALR must be distinct from others
pub struct LALRGraph<'a> {
    lr1_graph : LRGraph<'a, LALRNode<'a>, LR1Rule>,
}

impl <'a> LALRGraph<'a> {
}
