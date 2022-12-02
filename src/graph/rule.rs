use std::collections::HashSet;
use std::hash::Hash;

use crate::grammar::Grammar;

pub trait LRRule: Sized + Hash + Clone + Copy + Eq {
    fn default() -> Self;
    fn get_sym(&self, g: &Grammar) -> Option<char>;
    fn get_left(&self, g: &Grammar) -> Option<char>;
    fn next_rule(&self) -> Self;
    fn create_closure(&self, g: &Grammar) -> HashSet<Self>;
}

