mod lr0rule;
mod lr0node;
mod grammar;
mod lr0graph;

use std::collections::HashSet;

use crate::{grammar::Grammar, lr0graph::LR0Graph, lr0node::LR0Node};




fn main() -> Result<(), &'static str> {
    let mut g = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
    g.add_rule('S', "aA")?;
    g.add_rule_vec('A', vec!['a'])?;
    println!("{:?}", g);
    println!("{}", g);

    let mut graph = LR0Graph::new();
    graph.construct(LR0Node::default(&g));

    println!("{:?}", graph);

    Ok(())
}


