#[macro_use]
extern crate log;
extern crate cfg;
extern crate env_logger;
extern crate gearley;

mod helpers;

use cfg::earley::Grammar;
use cfg::Symbol;
use gearley::forest::{Bocage, CompactBocage};
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;

use helpers::{Parse, SimpleCompactEvaluator, SimpleEvaluator};

macro_rules! test_trivial_grammar {
    ($Bocage:ident, $SimpleEvaluator:ident) => {
        let _ = env_logger::try_init();
        let mut external = Grammar::new();
        let start = external.sym();
        external.rule(start).rhs([]);
        external.set_start(start);
        let cfg = InternalGrammar::from_grammar(&external);
        let mut evaluator = $SimpleEvaluator::new(
            |_: Symbol| unreachable!(),
            |_: u32, _: &[&bool]| unreachable!(),
            |sym, builder: &mut Vec<bool>| {
                builder.reserve(1);
                if sym == start {
                    builder.push(true);
                } else {
                    builder.push(false);
                }
            },
        );
        let bocage = $Bocage::new(&cfg);
        let mut rec = Recognizer::new(&cfg, bocage);
        assert!(rec.parse(&[]));
        let mut traversal = rec.forest.traverse();
        let results = evaluator.traverse(&mut traversal, rec.finished_node().unwrap());
        assert_eq!(results, &[true]);
    };
}

#[test]
fn test_trivial_grammar() {
    test_trivial_grammar!(Bocage, SimpleEvaluator);
}

#[test]
fn test_trivial_grammar_compact() {
    test_trivial_grammar!(CompactBocage, SimpleCompactEvaluator);
}

macro_rules! test_grammar_with_nulling_intermediate {
    ($Bocage:ident, $SimpleEvaluator:ident) => {
        let _ = env_logger::try_init();
        let mut external = Grammar::new();
        let (start, a, b, c, d, foo) = external.sym();
        external
            .rule(start)
            .rhs([a, b, c, d, foo])
            .rule(a)
            .rhs([])
            .rule(b)
            .rhs([])
            .rule(c)
            .rhs([])
            .rule(d)
            .rhs([]);
        external.set_start(start);
        let cfg = InternalGrammar::from_grammar(&external);
        let mut evaluator = $SimpleEvaluator::new(
            |sym: Symbol| {
                if sym == foo {
                    3
                } else {
                    unreachable!()
                }
            },
            |rule: u32, arg: &[&i32]| {
                if rule == 0 {
                    arg.iter().cloned().fold(0, |a, e| a + e)
                } else {
                    unreachable!()
                }
            },
            |sym, builder: &mut Vec<i32>| {
                builder.reserve(1);
                if sym == a {
                    builder.push(1);
                } else {
                    builder.push(2);
                }
            },
        );
        let bocage = $Bocage::new(&cfg);
        let mut rec = Recognizer::new(&cfg, bocage);
        assert!(rec.parse(&[foo.usize() as u32]));
        let mut traversal = rec.forest.traverse();
        let results = evaluator.traverse(&mut traversal, rec.finished_node().unwrap());
        assert_eq!(results, &[10]);
    };
}

#[test]
fn test_grammar_with_nulling_intermediate() {
    test_grammar_with_nulling_intermediate!(Bocage, SimpleEvaluator);
}

#[test]
fn test_grammar_with_nulling_intermediate_compact() {
    test_grammar_with_nulling_intermediate!(CompactBocage, SimpleCompactEvaluator);
}
