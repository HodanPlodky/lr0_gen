use std::collections::{HashMap, HashSet};

use crate::grammar::Grammar;

use super::{
    lr0rule::LR0Rule,
    lr1graph::{LR1Node, LR1Rule},
    lrnode::LRNode,
};

#[derive(Debug)]
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

impl<'a> LALRNode<'a> {
    pub fn union_nodes(&self, other: &LALRNode<'a>) -> (LALRNode<'a>, bool) {
        let union_base = HashSet::from_iter(self.lr1node.base().union(other.lr1node.base()).copied());
        let conti = union_base.len() != self.lr1node.base().len();
        (LALRNode::new(union_base, self.from(), self.gramm()), conti)
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::Sym;

    use super::*;

    #[test]
    fn eq_test() {
        let mut g1 = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        g1.add_rule('S', "A").unwrap();
        g1.add_rule('A', "a").unwrap();
        let left = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Eps)]), 'S', &g1);
        let right = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Normal('a'))]), 'S', &g1);
        let different = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Normal('a')), LR1Rule::new(0, 1, Sym::Normal('a'))]), 'S', &g1);

        assert_eq!(left, right);
        assert_eq!(right, left);
        assert_ne!(different, right);
        assert_ne!(left, different);
    }

    #[test]
    fn union_test() {
        let mut g1 = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        g1.add_rule('S', "A").unwrap();
        g1.add_rule('A', "a").unwrap();
        let left = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Eps)]), 'S', &g1);
        let right = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Normal('a'))]), 'S', &g1);
        let corr_union = LALRNode::new(HashSet::from([LR1Rule::new(0, 0, Sym::Normal('a')), LR1Rule::new(0, 0, Sym::Eps)]), 'S', &g1);
        let (res, conti) = left.union_nodes(&right);
        assert!(conti);
        assert_eq!(res.base(), corr_union.base());
        assert_eq!(res.base().len(), 2);
    }
}
