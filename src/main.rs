mod lr0rule;
mod lr0node;
mod grammar;
mod lr0graph;

use std::{collections::HashSet, fmt::Display};

use crate::grammar::Grammar;




fn main() -> Result<(), &'static str> {
    let mut g = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
    g.add_rule('S', "aA")?;
    g.add_rule_vec('A', vec!['a'])?;
    println!("{:?}", g);
    println!("{}", g);
    Ok(())
}


