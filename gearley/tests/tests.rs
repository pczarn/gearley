#[macro_use]
extern crate log;
extern crate cfg;
extern crate gearley;
extern crate gearley_example_grammars;

// use gearley::{}

use gearley_example_grammars::{ambiguous_math, precedenced_math};
use gearley::{Bocage, DefaultGrammar, Recognizer, RecognizerParseExt};

#[test]
fn test_precedenced_arith() {
    let external = precedenced_math::grammar();
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, Bocage::new(&cfg));
    let tokens = precedenced_math::tokenize("1+(2*3-4)/(55)-(54)*55+62-13-((36))");
    assert!(rec.parse(&tokens).unwrap());
}

#[test]
fn test_ambiguous_arithmetic() {
    let tokens = ambiguous_math::tokenize("2-0*3+1");
    let external = ambiguous_math::grammar();
    let cfg = DefaultGrammar::from_grammar(external);
    let evaluate = ambiguous_math::Evaluator;
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::with_forest(&cfg, bocage);
    assert!(rec.parse(&tokens).unwrap());
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
