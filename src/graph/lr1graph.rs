use std::collections::HashSet;

use crate::{grammar::Sym, graph::lr0rule::LR0Rule};

pub struct LR1Rule {
    lr0: LR0Rule,
    set: HashSet<Sym>,
}

impl LR1Rule {
    pub fn new(rule: usize, place: usize, set: HashSet<Sym>) -> Self {
        Self {
            lr0: LR0Rule::new(rule, place),
            set,
        }
    }
}
