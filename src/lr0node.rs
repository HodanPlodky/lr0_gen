use std::collections::{HashMap, HashSet};

use crate::{lr0rule::LR0Rule, Grammar};

#[derive(Debug)]
pub struct LR0Node<'a> {
    base: HashSet<LR0Rule>,
    closure: HashSet<LR0Rule>,
    pub(crate) gramm: &'a Grammar,
}

impl PartialEq for LR0Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Eq for LR0Node<'_> {}

impl<'a> LR0Node<'a> {
    pub(crate) fn new(base: HashSet<LR0Rule>, gramm: &'a Grammar) -> Self {
        Self {
            base,
            closure: HashSet::new(),
            gramm,
        }
    }

    pub(crate) fn default(g: &'a Grammar) -> Self {
        Self::new(HashSet::from([LR0Rule::new(0, 0)]), g)
    }

    pub(crate) fn create_closure(&mut self) {
        let mut flag = true;
        let mut acc = self.base.clone();
        while flag {
            let mut tmp: HashSet<LR0Rule> = HashSet::new();
            acc.iter().for_each(|b| {
                if let Some(x) = b.get_sym(self.gramm) {
                    if self.gramm.is_non_term(&x) {
                        self.gramm.rule_for_sym(x).iter().for_each(|r| {
                            tmp.insert(LR0Rule::new(*r, 0));
                        });
                    }
                }
            });
            acc = tmp;
            let l = self.closure.len();
            for i in acc.iter() {
                self.closure.insert(i.clone());
            }
            flag = self.closure.len() != l;
        }
    }

    pub(crate) fn get_steps(&self) -> HashMap<char, Vec<LR0Rule>> {
        let mut res: HashMap<char, Vec<LR0Rule>> = HashMap::new();
        for rule in self.base.union(&self.closure) {
            if let Some(c) = rule.get_sym(&self.gramm) {
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

    pub(crate) fn all_rules(&self) -> HashSet<&LR0Rule> {
        self.base
            .union(&self.closure)
            .collect::<HashSet<&LR0Rule>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_closure(rules: Vec<LR0Rule>, closure: Vec<LR0Rule>, gramm: &Grammar) {
        let mut lr0node = LR0Node::new(HashSet::from_iter(rules.into_iter()), &gramm);

        lr0node.create_closure();
        println!("{:?}", lr0node.closure);
        for r in closure {
            assert!(lr0node.closure.contains(&r));
        }
    }

    #[test]
    fn test_create_closure() {
        let mut g1 = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        g1.add_rule('S', "A").unwrap();
        g1.add_rule('A', "a").unwrap();

        test_closure(vec![LR0Rule::new(0, 0)], vec![LR0Rule::new(1, 0)], &g1);

        let mut g2 = Grammar::new(HashSet::from(['S', 'A', 'A']), HashSet::from(['a']));
        g2.add_rule('S', "A").unwrap();
        g2.add_rule('A', "a").unwrap();
        g2.add_rule('A', "aA").unwrap();

        test_closure(
            vec![LR0Rule::new(0, 0)],
            vec![LR0Rule::new(1, 0), LR0Rule::new(2, 0)],
            &g2,
        );

        let mut g3 = Grammar::new(HashSet::from(['S', 'A', 'B']), HashSet::from(['a']));
        g3.add_rule('S', "A").unwrap();
        g3.add_rule('A', "a").unwrap();
        g3.add_rule('A', "aA").unwrap();
        g3.add_rule('A', "BA").unwrap();
        g3.add_rule('B', "a").unwrap();

        test_closure(
            vec![LR0Rule::new(0, 0)],
            vec![
                LR0Rule::new(1, 0),
                LR0Rule::new(2, 0),
                LR0Rule::new(3, 0),
                LR0Rule::new(4, 0),
            ],
            &g3,
        );

        let mut g4 = Grammar::new(HashSet::from(['S', 'A', 'B', 'C']), HashSet::from(['a']));
        g4.add_rule('S', "A").unwrap();
        g4.add_rule('A', "a").unwrap();
        g4.add_rule('A', "aA").unwrap();
        g4.add_rule('A', "BA").unwrap();
        g4.add_rule('B', "a").unwrap();
        g4.add_rule('C', "a").unwrap();

        test_closure(
            vec![LR0Rule::new(0, 0)],
            vec![
                LR0Rule::new(1, 0),
                LR0Rule::new(2, 0),
                LR0Rule::new(3, 0),
                LR0Rule::new(4, 0),
            ],
            &g4,
        );
    }

    fn test_steps(base: Vec<LR0Rule>, syms: Vec<char>, gramm: &Grammar) {
        let mut lr0node = LR0Node::new(HashSet::from_iter(base.into_iter()), gramm);
        lr0node.create_closure();
        let gen_syms = lr0node.get_steps();

        let hset: HashSet<char> = HashSet::from_iter(syms.into_iter());

        assert_eq!(gen_syms.len(), hset.len());
        //assert_eq!(HashSet::from_iter(syms.into_iter()), gen_syms);

        for c in hset {
            assert!(gen_syms.contains_key(&c));
        }
    }

    #[test]
    fn test_get_steps() {
        let mut g1 = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        g1.add_rule('S', "A").unwrap();
        g1.add_rule('A', "a").unwrap();

        test_steps(vec![LR0Rule::new(0, 0)], vec!['A', 'a'], &g1);

        let mut g2 = Grammar::new(HashSet::from(['S', 'A', 'A']), HashSet::from(['a']));
        g2.add_rule('S', "A").unwrap();
        g2.add_rule('A', "a").unwrap();
        g2.add_rule('A', "aA").unwrap();
        test_steps(vec![LR0Rule::new(0, 0)], vec!['A', 'a'], &g2);

        let mut g3 = Grammar::new(HashSet::from(['S', 'A', 'B']), HashSet::from(['a']));
        g3.add_rule('S', "A").unwrap();
        g3.add_rule('A', "a").unwrap();
        g3.add_rule('A', "aA").unwrap();
        g3.add_rule('A', "BA").unwrap();
        g3.add_rule('B', "a").unwrap();
        test_steps(vec![LR0Rule::new(0, 0)], vec!['A', 'a', 'B'], &g3);

        let mut g4 = Grammar::new(HashSet::from(['S', 'A', 'B', 'C']), HashSet::from(['a']));
        g4.add_rule('S', "A").unwrap();
        g4.add_rule('A', "a").unwrap();
        g4.add_rule('A', "aA").unwrap();
        g4.add_rule('A', "BA").unwrap();
        g4.add_rule('B', "a").unwrap();
        g4.add_rule('C', "a").unwrap();
        test_steps(vec![LR0Rule::new(0, 0)], vec!['A', 'a', 'B'], &g4);
    }
}
