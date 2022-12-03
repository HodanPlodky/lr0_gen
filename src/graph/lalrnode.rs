use std::collections::{HashMap, HashSet};

use crate::grammar::Grammar;

use super::{
    lr0rule::LR0Rule,
    lr1graph::{LR1Node, LR1Rule}, lrnode::LRNode,
};

pub struct LALRNode<'a> {
    lr1node: LR1Node<'a>,
}

impl PartialEq for LALRNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        let lr0_self: HashSet<LR0Rule> =
            HashSet::from_iter(self.lr1node.base().iter().map(|x| x.lr0));
        let lr0_other: HashSet<LR0Rule> =
            HashSet::from_iter(other.lr1node.base().iter().map(|x| x.lr0));

        lr0_other == lr0_self
    }
}

impl<'a> LRNode<'a, LR1Rule> for LALRNode<'a> {
    fn new(base: HashSet<LR1Rule>, from: char, gramm: &'a Grammar) -> Self {
        Self {
            lr1node: LR1Node::new(base, from, gramm),
        }
    }

    fn default(g: &'a Grammar) -> Self {
        Self {
            lr1node: LR1Node::default(g),
        }
    }

    fn create_closure(&mut self) {
        self.lr1node.create_closure()
    }

    fn get_steps(&self) -> HashMap<char, Vec<LR1Rule>> {
        self.lr1node.get_steps()
    }

    fn all_rules(&self) -> HashSet<&LR1Rule> {
        self.lr1node.all_rules()
    }

    fn from(&self) -> char {
        self.lr1node.from()
    }

    fn base(&self) -> &HashSet<LR1Rule> {
        self.lr1node.base()
    }

    fn closure(&self) -> &HashSet<LR1Rule> {
        self.lr1node.closure()
    }

    fn gramm(&self) -> &'a Grammar {
        self.lr1node.gramm()
    }
}
