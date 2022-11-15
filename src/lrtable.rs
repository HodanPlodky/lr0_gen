use std::fmt::Display;

use crate::grammar::Sym;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Action {
    Shift,
    Accept,
    Reduction(usize),
    Error,
    Empty,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Shift => write!(f, "S"),
            Action::Accept => write!(f, "A"),
            Action::Reduction(x) => write!(f, "R{}", x),
            Action::Error => write!(f, "E"),
            Action::Empty => write!(f, ""),
        }
    }
}

impl Action {
    pub(crate) fn update(&self, new: Action) -> Action {
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

pub trait Table {
    fn get_action(&self, state : usize, sym : Sym) -> Option<Action>;
    fn get_goto(&self, state : usize, chr : char) -> Option<usize>;
    fn get_char(&self, state : usize) -> Option<char>;
}
