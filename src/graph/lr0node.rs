use crate::graph::lr0rule::LR0Rule;

use super::lrnode::LRNode;

pub type LR0Node<'a> = LRNode<'a, LR0Rule>;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::grammar::Grammar;

    use super::*;

    fn test_closure(rules: Vec<LR0Rule>, closure: Vec<LR0Rule>, gramm: &Grammar) {
        let mut lr0node = LR0Node::new(HashSet::from_iter(rules.into_iter()), 'X', &gramm);

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

        let mut g = Grammar::new(
            HashSet::from(['S', 'E', 'T']),
            HashSet::from(['(', ')', 'a', '$', '+']),
        );
        g.add_rule('S', "E$").unwrap();
        g.add_rule('E', "E+T").unwrap();
        g.add_rule('E', "T").unwrap();
        g.add_rule('T', "a").unwrap();
        g.add_rule('T', "(E)").unwrap();

        test_closure(
            vec![LR0Rule::new(0, 0)],
            vec![
                LR0Rule::new(1, 0),
                LR0Rule::new(2, 0),
                LR0Rule::new(3, 0),
                LR0Rule::new(4, 0),
            ],
            &g,
        );
    }

    fn test_steps(base: Vec<LR0Rule>, syms: Vec<char>, gramm: &Grammar) {
        let mut lr0node = LR0Node::new(HashSet::from_iter(base.into_iter()), 'X', gramm);
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

        let mut g = Grammar::new(
            HashSet::from(['S', 'E', 'T']),
            HashSet::from(['(', ')', 'a', '$', '+']),
        );
        g.add_rule('S', "E$").unwrap();
        g.add_rule('E', "E+T").unwrap();
        g.add_rule('E', "T").unwrap();
        g.add_rule('T', "a").unwrap();
        g.add_rule('T', "(E)").unwrap();

        test_steps(vec![LR0Rule::new(0, 0)], vec!['E', 'a', '(', 'T'], &g);
        test_steps(vec![LR0Rule::new(1, 2)], vec!['a', '(', 'T'], &g);
    }
}
