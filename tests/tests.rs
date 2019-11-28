#[macro_use]
extern crate log;
extern crate cfg;
extern crate gearley;

#[macro_use]
mod grammars;
mod helpers;

use gearley::grammar::InternalGrammar;
use gearley::forest::{Bocage, NullForest};
use gearley::recognizer::Recognizer;

use grammars::*;
use helpers::{SimpleEvaluator, Parse};

const SUM_TOKENS: &'static [u32] = precedenced_arith!(
    '1' '+' '(' '2' '*' '3' '-' '4' ')' '/'
    '(' '5' '5' ')' '-' '(' '5' '4' ')' '*'
    '5' '5' '+' '6' '2' '-' '1' '3' '-' '('
    '(' '3' '6' ')' ')'
);

#[test]
fn test_precedenced_arith() {
    let external = precedenced_arith::grammar();
    let cfg = InternalGrammar::from_grammar(&external);
    let mut rec = Recognizer::new(&cfg, NullForest);
    assert!(rec.parse(SUM_TOKENS));
}

#[test]
fn test_ambiguous_arithmetic() {
    let tokens = ambiguous_arith!('2' '-' '0' '*' '3' '+' '1');
    let external = ambiguous_arith::grammar();
    let cfg = InternalGrammar::from_grammar(&external);
    let mut evaluator = SimpleEvaluator::new(
        ambiguous_arith::leaf,
        ambiguous_arith::rule,
        |_, _: &mut Vec<i32>| unreachable!()
    );
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::new(&cfg, bocage);
    assert!(rec.parse(tokens));
    let mut traverse = rec.forest.traverse();
    let results = evaluator.traverse(&mut traverse, rec.finished_node().unwrap());

    // The result is currently ordered by rule ID:
    assert_eq!(results, vec![2, 1, 3, 7, 8]);

    // A result ordered by structure would be: [2, 1, 8, 3, 7]
    // where

    // 2  =  2 - (0 * (3 + 1))
    // 1  =  2 - ((0 * 3) + 1)
    // 8  =  (2 - 0) * (3 + 1)
    // 3  =  (2 - (0 * 3)) + 1
    // 7  =  ((2 - 0) * 3) + 1
}
