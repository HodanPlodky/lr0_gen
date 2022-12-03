use std::{collections::HashMap, fmt::Display};

use crate::{
    grammar::{Grammar, Sym},
    graph::{lrgraph::LR0Graph, rule::LRRule},
    table::lrtable::{Action, Table},
};

pub(crate) struct SLR1Table<'a> {
    action: Vec<(char, HashMap<Sym, Action>)>,
    goto: Vec<HashMap<char, usize>>,
    syms: Vec<char>,
    gramm: &'a Grammar,
}

impl Display for SLR1Table<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SLR1Table")?;
        write!(f, "state\t|")?;
        for c in self.syms.iter().filter(|x| self.gramm.is_term(x)) {
            write!(f, "{}\t", c)?;
        }
        write!(f, "{}\t|", Sym::Eps)?;
        for c in &self.syms {
            write!(f, "{}\t", c)?;
        }
        writeln!(f, "")?;
        for i in 0..self.action.len() {
            let (c, _) = &self.action[i];
            write!(f, "{}{}\t|", c, i)?;
            let (_, a) = &self.action[i];
            for c in self.syms.iter().filter(|x| self.gramm.is_term(x)) {
                match a.get(&Sym::Normal(*c)) {
                    Some(a) => write!(f, "{}\t", a),
                    None => write!(f, " \t"),
                }?;
            }
            match a.get(&Sym::Eps) {
                Some(a) => write!(f, "{}\t|", a),
                None => write!(f, " \t|"),
            }?;
            for s in &self.syms {
                match self.goto[i].get(s) {
                    Some(g) => write!(f, "{}\t", g),
                    None => write!(f, " \t"),
                }?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl <'a> SLR1Table <'a> {
    pub(crate) fn new(graph: LR0Graph, gramm: &'a Grammar) -> Self {
        let mut syms: Vec<char> = vec![];
        gramm
            .terms
            .union(&gramm.non_terms)
            .for_each(|x| syms.push(*x));
        let action: Vec<(char, HashMap<Sym, Action>)> = graph
            .nodes
            .iter()
            .map(|x| {
                let mut res =
                    HashMap::from_iter(syms.iter().map(|x| (Sym::Normal(*x), Action::Empty)));
                res.insert(Sym::Eps, Action::Empty);
                for r in x.all_rules() {
                    match r.get_sym(gramm) {
                        Some(s) => {
                            if gramm.is_non_term(&s) {
                                continue;
                            }
                            let tmp = res.get(&Sym::Normal(s)).unwrap().update(Action::Shift);
                            res.insert(Sym::Normal(s), tmp);
                        }
                        None => {
                            for f in gramm.follow(r.get_left(gramm).unwrap()) {
                                let tmp = res.get(&f).unwrap().update(Action::Reduction(r.rule));
                                res.insert(f.clone(), tmp);
                            }
                        }
                    }
                }
                (x.from, res)
            })
            .collect();

        Self {
            action,
            goto: graph.edges,
            syms,
            gramm,
        }
    }
}

impl Table for SLR1Table<'_> {
    fn get_action(&self, state: usize, sym: Sym) -> Option<Action> {
        let (_, a) = self.action.get(state)?;
        a.get(&sym).copied()
    }

    fn get_goto(&self, state: usize, chr: char) -> Option<usize> {
        self.goto.get(state)?.get(&chr).copied()
    }

    fn get_char(&self, state: usize) -> Option<char> {
        let (c, _) = self.action.get(state)?;
        Some(*c)
    }
}
