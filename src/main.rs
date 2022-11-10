mod grammar;
mod lr0graph;
mod lr0node;
mod lr0rule;
mod lr0table;

use std::collections::HashSet;

use crate::{grammar::Grammar, lr0graph::LR0Graph, lr0node::LR0Node, lr0table::LR0Table};

fn main() -> Result<(), &'static str> {
    //let mut g = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
    //g.add_rule('S', "aA")?;
    //g.add_rule_vec('A', vec!['a'])?;

    //let mut graph = LR0Graph::new();
    //graph.construct(LR0Node::default(&g));

    //let lr0t = LR0Table::new(graph, &g);
    //println!("{}", lr0t);
    
    let mut g = Grammar::new(HashSet::from(['S', 'E', 'T']), HashSet::from(['(', ')', 'a', '$', '+']));
    g.add_rule('S', "E$")?;
    g.add_rule('E', "E+T")?;
    g.add_rule('E', "T")?;
    g.add_rule('T', "a")?;
    g.add_rule('T', "(E)")?;

    let mut graph = LR0Graph::new();
    graph.construct(LR0Node::default(&g));

    let lr0t = LR0Table::new(graph, &g);
    println!("{}", lr0t);

    Ok(())
}
