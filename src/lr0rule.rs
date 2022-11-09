use crate::Grammar;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LR0Rule {
    rule: usize,
    place: usize,
}

impl LR0Rule {
    pub fn new(rule: usize, place: usize) -> Self {
        Self { rule, place }
    }

    pub(crate) fn get_sym(&self, gramm: &Grammar) -> Option<char> {
        if gramm.rules.len() <= self.rule {
            return None;
        }

        if gramm.rules[self.rule].right.len() <= self.place {
            None
        } else {
            Some(gramm.rules[self.rule].right[self.place])
        }
    }

    pub(crate) fn next_rule(&self) -> Self {
        LR0Rule::new(self.rule, self.place + 1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn add_rules(g: &mut Grammar, rules: Vec<(char, &str)>) -> Result<(), &'static str> {
        for r in rules {
            g.add_rule(r.0, r.1)?;
        }
        Ok(())
    }

    fn dummy_gramm() -> Grammar {
        let mut g = Grammar::new(HashSet::from(['S', 'A']), HashSet::from(['a']));
        add_rules(&mut g, vec![('S', "aA"), ('A', "a")]).unwrap();
        g
    }

    #[test]
    fn test_get_sym() {
        let g = dummy_gramm();

        assert_eq!(LR0Rule::new(0, 0).get_sym(&g), Some('a'));
        assert_eq!(LR0Rule::new(4, 0).get_sym(&g), None);
        assert_eq!(LR0Rule::new(0, 1).get_sym(&g), Some('A'));
        assert_eq!(LR0Rule::new(1, 0).get_sym(&g), Some('a'));
        assert_eq!(LR0Rule::new(1, 1).get_sym(&g), None);
    }
}
