#[macro_use]
extern crate log;
extern crate cfg;
extern crate gearley;

#[macro_use]
mod grammars;
mod helpers;

// use gearley::{}

use cfg::Symbol;
use ambiguous_arith::AmbiguousArithEvaluator;
use gearley::{Bocage, DefaultGrammar, Recognizer, RecognizerParseExt};
use grammars::*;

const SUM_TOKENS: &'static [Symbol] = precedenced_arith!(
    '1' '+' '(' '2' '*' '3' '-' '4' ')' '/'
    '(' '5' '5' ')' '-' '(' '5' '4' ')' '*'
    '5' '5' '+' '6' '2' '-' '1' '3' '-' '('
    '(' '3' '6' ')' ')'
);

#[test]
fn test_precedenced_arith() {
    let external = precedenced_arith::grammar();
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, Bocage::new(&cfg));
    assert!(rec.parse(SUM_TOKENS));
}

#[test]
fn test_ambiguous_arithmetic() {
    let tokens = ambiguous_arith!('2' '-' '0' '*' '3' '+' '1');
    let external = ambiguous_arith::grammar();
    let cfg = DefaultGrammar::from_grammar(external);
    let mut evaluate = AmbiguousArithEvaluator;
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::with_forest(&cfg, bocage);
    assert!(rec.parse(tokens));
    let finished_node = rec.finished_node().expect("exhausted");
    let mut forest = rec.into_forest();
    let results = forest.evaluate(evaluate, finished_node);
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
