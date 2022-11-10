use std::fmt::Display;

use crate::{
    grammar::{Grammar, Rule},
    lr0table::{Action, LR0Table},
};

#[derive(Debug)]
pub(crate) struct StackAutomata<'a> {
    stack: Vec<usize>,
    table: LR0Table,
    input: Vec<char>,
    result: Vec<usize>,
    place: usize,
    gramm: &'a Grammar,
}

impl Display for StackAutomata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stack : ",)?;
        for s in self.stack.iter() {
            write!(f, "{} ", s)?;
        }
        writeln!(f, "")?;

        write!(f, "input : ",)?;
        for s in self.input.iter().skip(self.place) {
            write!(f, "{} ", s)?;
        }
        writeln!(f, "")?;

        write!(f, "result : ",)?;
        for s in self.result.iter() {
            write!(f, "{} ", s)?;
        }
        writeln!(f, "")?;

        Ok(())
    }
}

impl<'a> StackAutomata<'a> {
    pub(crate) fn new(table: LR0Table, input: &'a str, gramm: &'a Grammar) -> Self {
        Self {
            stack: vec![0],
            table,
            input: input.chars().collect(),
            result: vec![],
            place: 0,
            gramm,
        }
    }

    fn compare_stack(&mut self, rule: &Rule) -> Option<char> {
        if rule.right.len() > self.stack.len() {
            return None;
        }
        for c in rule.right.iter().rev() {
            let state = self.stack.pop()?;
            let tmp = self.table.get_char(state)?;
            if tmp != *c {
                return None;
            }
        }
        Some(rule.left)
    }

    pub(crate) fn run(&mut self) -> Option<()> {
        loop {
            let a = self.step()?;
            if a == Action::Accept {
                break;
            }
        }
        Some(())
    }

    pub(crate) fn step(&mut self) -> Option<Action> {
        let top_stack = self.top()?;
        let action = self.table.get_action(top_stack)?;
        match action {
            Action::Shift => {
                let c = self.next_char()?;
                self.stack.push(self.get_goto(c)?);
                Some(Action::Shift)
            }
            Action::Accept => {
                let x = 0;
                let rule = self.gramm.rules.get(x)?;
                let c = self.compare_stack(rule)?;
                self.result.push(x);
                Some(Action::Accept)
            }
            Action::Reduction(x) => {
                let x = x.clone();
                let rule = self.gramm.rules.get(x)?;
                let c = self.compare_stack(rule)?;
                self.stack.push(self.get_goto(c)?);
                self.result.push(x);
                Some(Action::Reduction(x))
            }
            Action::Error | Action::Empty => return None,
        }
    }

    pub(crate) fn top(&self) -> Option<usize> {
        self.stack.last().copied()
    }

    pub(crate) fn next_char(&mut self) -> Option<char> {
        if self.place < self.input.len() {
            self.place += 1;
            self.input.get(self.place - 1).copied()
        } else {
            None
        }
    }

    pub(crate) fn get_goto(&self, c: char) -> Option<usize> {
        let state = self.top()?;
        self.table.get_goto(state, c)
    }
}
