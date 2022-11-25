
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
    first: Vec<HashSet<Sym>>,
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
            first: vec![],
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

    pub(crate) fn first(&self, rule: usize) -> Option<&HashSet<Sym>> {
        self.first.get(rule)
    }

    pub(crate) fn create_first(&mut self) {
        self.first.resize(self.rules.len(), HashSet::new());
        let mut flag = true;
        while flag {
            flag = false;
            for i in 0..self.rules.len() {
                let rule = &self.rules[i];
                let orig = self.first[i].len();
                self.first[i] = HashSet::from_iter(self.first[i].union(&self.first_from(&rule.right[0..])).copied());
                flag |= orig != self.first[i].len();
            }
        }
    }

    fn first_from(&self, syms: &[char]) -> HashSet<Sym> {
        let mut res: HashSet<Sym> = HashSet::new();
        if syms.len() == 0 {
            res.insert(Sym::Eps);
        } else if self.is_term(&syms[0]) {
            res.insert(Sym::Normal(syms[0]));
        } else if self.is_non_term(&syms[0]) {
            let rules = self.rule_for_sym(syms[0]);
            let mut to_add: HashSet<Sym> = HashSet::new();
            for r in rules {
                to_add = HashSet::from_iter(to_add.union(self.first(r).unwrap()).copied());
            }
            if to_add.contains(&Sym::Eps) {
                to_add.remove(&Sym::Eps);
                to_add = HashSet::from_iter(to_add.union(&self.first_from(&syms[1..])).copied());
            }
            res = HashSet::from_iter(res.union(&to_add).copied());
        }
        res
    }

    pub(crate) fn follow(&self, non_term: char) -> &HashSet<Sym> {
        self.follow.get(&non_term).unwrap()
    }

    fn add_follow_from(&mut self, rule: usize) -> bool {
        let mut added = false;
        let rule = &self.rules[rule];
        for i in 0..rule.right.len() {
            if self.is_non_term(&rule.right[i]) {
                let nterm = rule.right[i];

                let orig = self.follow.get(&nterm).unwrap();
                let orig_len = orig.len();
                let mut to_add = self.first_from(&rule.right[i + 1..]);
                if to_add.contains(&Sym::Eps) {
                    to_add.remove(&Sym::Eps);
                    to_add = HashSet::from_iter(to_add.union(self.follow(rule.left)).copied());
                }
                let new: HashSet<Sym> = HashSet::from_iter(orig.union(&to_add).copied());
                added = new.len() != orig_len;
                self.follow.insert(nterm, new);
            }
        }
        added
    }

    pub(crate) fn create_follow(&mut self) {
        for n in &self.non_terms {
            self.follow.insert(*n, HashSet::new());
        }
        self.follow
            .insert(self.rules[0].left, HashSet::from([Sym::Eps]));
        let mut flag = true;
        while flag {
            flag = false;
            for r in 0..self.rules.len() {
                flag |= self.add_follow_from(r);
            }
        }
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
