use std::collections::HashMap;

use crate::{grammar::Grammar, lr0graph::LR0Graph};

#[derive(PartialEq, Eq, Debug)]
enum Action {
    Shift,
    Accept,
    Reduction(usize),
    Error,
    Empty,
}

impl Action {
    fn update(&self, new: Action) -> Action {
        match (self, new) {
            (Action::Shift | Action::Empty, Action::Shift) => Action::Shift,
            (Action::Empty, Action::Reduction(0)) => Action::Accept,
            (Action::Accept, Action::Reduction(0)) => Action::Accept,
            (Action::Empty, Action::Reduction(r)) => Action::Reduction(r),
            (Action::Reduction(r1), Action::Reduction(r2)) => {
                if *r1 == r2 {
                    Action::Reduction(*r1)
                } else {
                    Action::Error
                }
            }
            _ => Action::Error,
        }
    }
}

#[derive(Debug)]
pub(crate) struct LR0Table {
    action: Vec<Action>,
    goto: Vec<HashMap<char, usize>>,
}

impl LR0Table {
    pub(crate) fn new(graph: LR0Graph, g: &Grammar) -> Self {
        let action: Vec<Action> = graph
            .nodes
            .iter()
            .map(|x| {
                let mut res = Action::Empty;
                for r in x.all_rules() {
                    match r.get_sym(g) {
                        Some(_) => {
                            res = res.update(Action::Shift);
                        }
                        None => res = res.update(Action::Reduction(r.rule)),
                    }
                }
                res
            })
            .collect();
        Self {
            action,
            goto: graph.edges,
        }
    }
}
