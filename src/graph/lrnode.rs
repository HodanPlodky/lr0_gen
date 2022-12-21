use std::collections::{HashMap, HashSet};

use crate::grammar::Grammar;

use super::rule::LRRule;

pub trait LRNode<'a, T> : PartialEq where T: LRRule + Clone {
    // data
    fn from(&self) -> char;
    fn base(&self) -> &HashSet<T>;
    fn closure(&self) -> &HashSet<T>;
    fn gramm(&self) -> &'a Grammar;

    // behavior
    fn new(base: HashSet<T>, from: char, gramm: &'a Grammar) -> Self;
    fn default(g: &'a Grammar) -> Self;
    fn create_closure(&mut self);
    fn get_steps(&self) -> HashMap<char, Vec<T>>;

    fn all_rules(&self) -> HashSet<&T> {
        self.base().union(&self.closure()).collect::<HashSet<&T>>()
    }
}

#[derive(Debug)]
pub struct LRNodeStruct<'a, T>
where
    T: LRRule,
{
    from: char,
    base: HashSet<T>,
    closure: HashSet<T>,
    g: &'a Grammar,
}

impl<T> PartialEq for LRNodeStruct<'_, T>
where
    T: LRRule,
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl<'a, T> LRNode<'a, T> for LRNodeStruct<'a, T>
where
    T: LRRule,
{
    fn new(base: HashSet<T>, from: char, gramm: &'a Grammar) -> Self {
        Self {
            from,
            base,
            closure: HashSet::new(),
            g : gramm,
        }
    }
    fn default(g: &'a Grammar) -> Self {
        Self::new(HashSet::from([T::default()]), g.rules[0].left, g)
    }

    fn create_closure(&mut self) {
        let mut flag = true;
        let mut acc = self.base.clone();
        while flag {
            let mut tmp: HashSet<T> = HashSet::new();
            acc.iter().for_each(|b| {
                let c = b.create_closure(self.gramm());
                tmp = HashSet::from_iter(tmp.union(&c).copied());
            });
            acc = tmp;
            let l = self.closure.len();
            for i in acc.iter() {
                self.closure.insert(i.clone());
            }
            flag = self.closure.len() != l;
        }
    }

    fn get_steps(&self) -> HashMap<char, Vec<T>> {
        let mut res: HashMap<char, Vec<T>> = HashMap::new();
        for rule in self.base.union(&self.closure) {
            if let Some(c) = rule.get_sym(&self.gramm()) {
                let tmp = rule.next_rule();
                match res.get_mut(&c) {
                    Some(v) => v.push(tmp),
                    None => {
                        res.insert(c, vec![tmp]);
                    }
                }
            }
        }
        res
    }

    fn from(&self) -> char {
        self.from
    }

    fn base(&self) -> &HashSet<T> {
        &self.base
    }

    fn closure(&self) -> &HashSet<T> {
        &self.closure
    }

    fn gramm(&self) -> &'a Grammar {
        self.g
    }
}
