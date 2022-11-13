use std::{collections::HashMap, fmt::Display};

use crate::{grammar::{Grammar, Sym}, lr0graph::LR0Graph, lr0table::Action};

pub(crate) struct SLR1Table {
    action: Vec<HashMap<Sym, Action>>,
    goto: Vec<HashMap<char, usize>>,
    syms: Vec<char>,
}

impl Display for SLR1Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display SLR")
    }
}

impl SLR1Table {
    pub(crate) fn new(graph: LR0Graph, gramm: &Grammar) -> Self {
        let mut syms : Vec<char> = vec![]; 
        gramm.terms.union(&gramm.non_terms).for_each(|x| {syms.push(*x)});
        let action: Vec<HashMap<Sym, Action>> = graph
            .nodes
            .iter()
            .map(|x| {
                let mut res = HashMap::from_iter(syms.iter().map(|x| (Sym::Normal(*x), Action::Empty)));
                res.insert(Sym::Eps, Action::Empty);
                for r in x.all_rules() {
                    match r.get_sym(gramm) {
                        Some(s) => {
                            let tmp = res.get(&Sym::Normal(s)).unwrap().update(Action::Shift);
                            res.insert(Sym::Normal(s), tmp);
                        } 
                        None => {
                            for f in gramm.follow(r.get_left(gramm).unwrap()) {
                                let tmp = res.get(&f).unwrap().update(Action::Reduction(r.rule));
                                res.insert(f, tmp);
                            } 
                        } 
                    }
                }
                res
            })
            .collect();

        Self {
            action,
            goto: graph.edges,
            syms,
        }
    }
}
