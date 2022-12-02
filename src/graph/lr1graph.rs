use std::collections::HashSet;

use crate::{
    grammar::{Grammar, Sym},
    graph::lr0rule::LR0Rule,
};

use super::{lrnode::LRNode, rule::LRRule};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LR1Rule {
    lr0: LR0Rule,
    pub follow: Sym,
}

impl LR1Rule {
    pub fn new(rule: usize, place: usize, follow: Sym) -> Self {
        Self {
            lr0: LR0Rule::new(rule, place),
            follow,
        }
    }

    fn get_rest<'a>(&self, gramm: &'a Grammar) -> Option<&'a [char]> {
        if gramm.rules.len() <= self.lr0.rule {
            return None;
        }

        if gramm.rules[self.lr0.rule].right.len() <= self.lr0.place {
            None
        } else {
            Some(&gramm.rules[self.lr0.rule].right[self.lr0.place..])
        }
    }

    fn get_new_follow(&self, gramm: &Grammar) -> HashSet<Sym> {
        if let Some(rest) = self.get_rest(&gramm) {
            match self.follow {
                Sym::Normal(x) => {
                    let mut tmp = Vec::from(rest);
                    tmp.push(x);
                    gramm.first_from(&tmp[0..])
                }
                Sym::Eps => gramm.first_from(rest),
            }
        } else {
            HashSet::from([self.follow])
        }
    }

    pub fn rule(&self) -> usize {
        self.lr0.rule
    }
}

impl LRRule for LR1Rule {
    fn default() -> Self {
        LR1Rule::new(0, 0, Sym::Eps)
    }

    fn get_sym(&self, g: &Grammar) -> Option<char> {
        self.lr0.get_sym(g)
    }

    fn get_left(&self, g: &Grammar) -> Option<char> {
        self.lr0.get_left(g)
    }

    fn next_rule(&self) -> LR1Rule {
        LR1Rule::new(self.lr0.rule, self.lr0.place + 1, self.follow.clone())
    }

    fn create_closure(&self, gramm: &Grammar) -> HashSet<Self> {
        let mut res: HashSet<Self> = HashSet::new();
        if let Some(x) = self.get_sym(gramm) {
            let nfollow = self.get_new_follow(gramm);
            if gramm.is_non_term(&x) {
                gramm.rule_for_sym(x).iter().for_each(|r| {
                    for nf in &nfollow {
                        res.insert(Self::new(*r, 0, *nf));
                    }
                });
            }
        }
        res
    }
}

pub type LR1Node<'a> = LRNode<'a, LR1Rule>;
