use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Sym {
    Normal(char),
    Eps,
}

impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sym::Normal(x) => write!(f, "{}", x),
            Sym::Eps => write!(f, "eps"),
        }
    }
}

#[derive(Debug)]
pub struct Grammar {
    pub(crate) non_terms: HashSet<char>,
    pub(crate) terms: HashSet<char>,

    pub(crate) rules: Vec<Rule>,

    follow: HashMap<char, HashSet<Sym>>,
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display")
    }
}

impl Grammar {
    pub(crate) fn new(non_terms: HashSet<char>, terms: HashSet<char>) -> Self {
        Self {
            non_terms,
            terms,
            rules: vec![],
            follow: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, left: char, right: &str) -> Result<(), &'static str> {
        if !self.non_terms.contains(&left) {
            return Err("left must be non terminal");
        }

        if right
            .chars()
            .any(|x| !self.is_term(&x) && !self.is_non_term(&x))
        {
            return Err("right must contain only terminals and non terminals");
        }

        let rule = Rule::new(left, right.chars().collect());

        self.rules.push(rule);
        Ok(())
    }

    pub(crate) fn add_rule_vec(
        &mut self,
        left: char,
        right: Vec<char>,
    ) -> Result<(), &'static str> {
        if !self.non_terms.contains(&left) {
            return Err("left must be non terminal");
        }

        if right
            .iter()
            .any(|x| !self.non_terms.contains(&x) && !self.terms.contains(&x))
        {
            return Err("right must contain only terminals and non terminals");
        }

        let rule = Rule::new(left, right);

        self.rules.push(rule);
        Ok(())
    }

    pub(crate) fn is_term(&self, sym: &char) -> bool {
        self.terms.contains(sym)
    }

    pub(crate) fn is_non_term(&self, sym: &char) -> bool {
        self.non_terms.contains(sym)
    }

    pub(crate) fn rule_for_sym(&self, sym: char) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        for i in 0..(self.rules.len()) {
            if self.rules[i].left == sym {
                res.push(i);
            }
        }
        res
    }

    pub(crate) fn first(&self, rule: usize) -> HashSet<Sym> {
        if self.rules.len() <= rule {
            return HashSet::new();
        }
        let right = &self.rules[rule].right;
        if right.len() == 0 {
            return HashSet::new();
        }
        if self.is_term(right.first().unwrap()) {
            return HashSet::from([Sym::Normal(right.first().copied().unwrap())]);
        }
        let left = self.rules[rule].left;
        let tmp = right.first().unwrap();
        if left == *tmp {
            let rules = self.rule_for_sym(left);
            let rules: Vec<usize> = rules
                .into_iter()
                .filter(|x| {
                    if *x == rule {
                        return false;
                    }
                    match self.rules[*x].right.first() {
                        Some(c) => self.is_term(c),
                        None => false,
                    }
                })
                .collect();
            let other_first = rules.iter().map(|x| self.first(*x));
            return other_first.re
        }
        let rules = self.rule_for_sym(*tmp);
        let mut res = HashSet::new();
        for r in rules {
            let tmp = self.first(r);
            res = HashSet::from_iter(res.union(&tmp).map(|x| *x));
        }
        res
    }

    fn first_from(&self, chrs: &[char]) -> HashSet<Sym> {
        if chrs.len() == 0 {
            return HashSet::from([Sym::Eps]);
        }
        if self.is_term(&chrs[0]) {
            return HashSet::from([Sym::Normal(chrs[0])]);
        }
        let tmp = chrs[0];
        let rules = self.rule_for_sym(tmp);
        let mut res = HashSet::new();
        for r in rules {
            let tmp = self.first(r);
            res = HashSet::from_iter(res.union(&tmp).map(|x| *x));
        }
        res
    }

    fn create_follow_for(&mut self, non_term: char) -> HashSet<Sym> {
        if let Some(x) = self.follow.get(&non_term) {
            return x.clone();
        }
        let mut res = HashSet::new();
        if self.rules.len() > 0 {
            if self.rules.get(0).unwrap().left == non_term {
                res.insert(Sym::Eps);
            }
        }
        let tmpr = self.rules.clone();
        for r in &tmpr {
            for i in 0..r.right.len() {
                if r.right[i] == non_term {
                    if i >= r.right.len() - 1 {
                        res =
                            HashSet::from_iter(res.union(&self.create_follow_for(r.left)).copied());
                    } else {
                        let tmp = &r.right[i..r.right.len()];
                        res = HashSet::from_iter(res.union(&self.first_from(tmp)).copied());
                    }
                }
            }
        }
        self.follow.insert(non_term, res);
        self.follow.get(&non_term).unwrap().clone()
    }

    pub(crate) fn follow(&self, non_term: char) -> &HashSet<Sym> {
        self.follow.get(&non_term).unwrap()
    }

    pub(crate) fn create_follow(&mut self) {
        let tmp = self.non_terms.clone();
        tmp.iter().for_each(|n| {
            self.create_follow_for(n.clone());
        });
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Rule {
    pub(crate) left: char,
    pub(crate) right: Vec<char>,
}

impl Rule {
    fn new(left: char, right: Vec<char>) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_gramm() -> Grammar {
        let mut g = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        add_rules(&mut g, vec![('S', "aA"), ('A', "a")]).unwrap();
        g
    }

    fn add_rules(g: &mut Grammar, rules: Vec<(char, &str)>) -> Result<(), &'static str> {
        for r in rules {
            g.add_rule(r.0, r.1)?;
        }
        Ok(())
    }

    #[test]
    fn basic() {
        let mut g = dummy_gramm();
        let tmp = add_rules(&mut g, vec![('S', "aA"), ('A', "a")]);
        assert!(tmp.is_ok());
        let tmp = add_rules(&mut g, vec![('S', "aA"), ('A', "g")]);
        assert!(tmp.is_err());
    }
}
