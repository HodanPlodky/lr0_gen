mod grammar;
mod graph;
mod stackautomata;
mod table;

use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, Read},
};

use crate::{
    grammar::Grammar,
    graph::{lr0node::LR0Node, lrgraph::LR0Graph},
    graph::{lr1graph::LR1Node, lrgraph::LR1Graph, lrnode::LRNode},
    stackautomata::StackAutomata,
    table::lr0table::LR0Table,
    table::slr1table::SLR1Table,
    table::{lr1table::LR1Table, lrtable::Table},
};

fn load_lines(path: String) -> std::io::Result<Vec<String>> {
    let mut f = File::open(path)?;
    let mut cont = String::new();
    f.read_to_string(&mut cont)?;
    let res: Vec<String> = cont.split('\n').map(|x| String::from(x)).collect();
    Ok(res)
}

fn get_chars(line: &String) -> Result<Vec<char>, &'static str> {
    let chars = line.split(' ').collect::<Vec<&str>>();

    if chars.iter().any(|x| x.len() != 1) {
        return Err("Non terminals must be one character");
    }
    Ok(chars
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>()[0])
        .collect())
}

fn load() -> Result<Grammar, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("No file inserted");
    }

    let path = String::from(args.get(1).unwrap().as_str());
    let lines = match load_lines(path) {
        Ok(l) => Ok(l),
        Err(_) => Err("Error while reading file"),
    }?;

    if lines.len() < 2 {
        return Err("Wrong format : atleast non-terminals and terminals needed");
    }

    let non_terms = HashSet::from_iter(get_chars(&lines[0])?.into_iter());
    let terms = HashSet::from_iter(get_chars(&lines[1])?.into_iter());

    let mut res = Grammar::new(non_terms, terms);
    for line in &lines[2..] {
        let sides: Vec<&str> = line.split("->").collect();
        if sides.len() == 1 && sides[0] == "" {
            continue;
        }
        if sides.len() != 2 {
            return Err("Wrong format of rule");
        }
        if sides[0].len() != 1 {
            return Err("Wrong format of rule");
        }
        res.add_rule(sides[0].chars().collect::<Vec<char>>()[0], sides[1])?;
    }
    res.create_first();
    res.create_follow();
    Ok(res)
}

fn get_input<F>(pred: F) -> Option<String>
where
    F: Fn(&String) -> bool,
{
    let stdin = io::stdin();
    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Err(_) => return None,
            _ => (),
        };
        buf.remove(buf.len() - 1);
        if pred(&buf) {
            return Some(buf);
        }
    }
}

fn main() -> Result<(), &'static str> {
    let g = load()?;

    println!("1. LR0\n2. SLR(1)\n3. LR(1)");

    let ttype = get_input(|x: &String| match x.parse::<i32>() {
        Ok(x) => x == 1 || x == 2 || x == 3,
        Err(_) => false,
    });

    let ttype = match ttype {
        Some(x) => Ok(x),
        None => Err("Bad input"),
    }?;

    let lrtab: Box<dyn Table> = match ttype.as_str() {
        "1" => {
            let mut graph = LR0Graph::new();
            graph.construct(LR0Node::default(&g));
            Box::new(LR0Table::new(graph, &g))
        }
        "2" => {
            let mut graph = LR0Graph::new();
            graph.construct(LR0Node::default(&g));
            Box::new(SLR1Table::new(graph, &g))
        }
        "3" => {
            let mut graph = LR1Graph::new();
            graph.construct(LR1Node::default(&g));
            Box::new(LR1Table::new(graph, &g))
        }
        _ => unreachable!(),
    };
    println!("{}", lrtab);

    loop {
        println!("Write string :");

        let input = match get_input(|_| true) {
            Some(s) => Ok(s),
            None => Err("Bad input"),
        }?;
        if input == "quit" {
            break;
        }

        let mut autom = StackAutomata::new(lrtab.as_ref(), input.as_str(), &g);
        autom.run();
        println!("{}", autom);
    }
    Ok(())
}
