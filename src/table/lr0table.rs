use std::{collections::HashMap, fmt::Display};

use crate::{grammar::{Grammar, Sym}, graph::{lrgraph::LR0Graph, rule::LRRule, lrnode::LRNode}, table::lrtable::{Action, Table}};

#[derive(Debug)]
pub(crate) struct LR0Table {
    action: Vec<(char, Action)>,
    goto: Vec<HashMap<char, usize>>,
    syms : Vec<char>,
}

impl Display for LR0Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LR0Table")?;
        write!(f, "state\taction\t")?;
        for c in &self.syms {
            write!(f, "{}\t", c)?;
        }
        writeln!(f, "")?;
        for i in 0..self.action.len() {
            let (c, tmp) = &self.action[i];
            write!(f, "{}{}\t{}\t", c, i, tmp)?;
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

impl LR0Table {
    pub(crate) fn new(graph: LR0Graph, g: &Grammar) -> Self {
        let action: Vec<(char, Action)> = graph
            .nodes
            .iter()
            .map(|x| {
                let mut res = Action::Empty;
                for r in x.all_rules() {
                    match r.get_sym(g) {
                        Some(_) => res = res.update(Action::Shift),
                        None => res = res.update(Action::Reduction(r.rule)),
                    }
                }
                (x.from(), res)
            })
            .collect();
        let mut syms : Vec<char> = vec![];
        g.terms.union(&g.non_terms).for_each(|x| {syms.push(*x)});

        Self {
            action,
            goto: graph.edges,
            syms,
        }
    }
}
impl Table for LR0Table {
    fn get_action(&self, state : usize, _ : Sym) -> Option<Action> {
        let (_, a) = self.action.get(state).copied()?;
        Some(a)
    }

    fn get_char(&self, state : usize) -> Option<char> {
        let (c, _) = self.action.get(state)?;
        Some(*c)
    }

    fn get_goto(&self, state : usize, c : char) -> Option<usize> {
        let goto_line = self.goto.get(state)?;
        goto_line.get(&c).copied()
    }
}
