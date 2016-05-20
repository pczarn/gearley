extern crate cfg;
extern crate gearley;

#[macro_use]
mod grammars;

use gearley::forest::{Bocage, Traversal, NullForest};
use gearley::forest::depth_first::{NullOrder, ArrayEvaluator, ValueArray, ActionClosureEvaluator};
use gearley::recognizer::Recognizer;
use gearley::util::slice_builder::SliceBuilder;

use grammars::*;

const SUM_TOKENS: &'static [u32] = precedenced_arith!(
    '1' '+' '(' '2' '*' '3' '-' '4' ')' '/'
    '(' '5' '5' ')' '-' '(' '5' '4' ')' '*'
    '5' '5' '+' '6' '2' '-' '1' '3' '-' '('
    '(' '3' '6' ')' ')'
);

#[test]
fn test_precedenced_arith() {
    let external = precedenced_arith::grammar();
    let cfg = external.into_internal_grammar();
    let null_forest = NullForest;
    let mut rec = Recognizer::new(&cfg, &null_forest);
    rec.parse(SUM_TOKENS);
}

#[test]
fn test_ambiguous_arithmetic() {
    let tokens = ambiguous_arith!('2' '-' '0' '*' '3' '+' '1');
    let external = ambiguous_arith::grammar();
    let cfg = external.into_internal_grammar();
    let values = ValueArray::new();
    let mut evaluator = ArrayEvaluator::new(
        &values,
        ActionClosureEvaluator::new(
            ambiguous_arith::leaf,
            ambiguous_arith::rule,
            |_, _: &mut SliceBuilder<i32>| unreachable!()
        )
    );
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::new(&cfg, &bocage);
    rec.parse(tokens);
    let mut traversal = Traversal::new(&bocage, NullOrder::new());
    let results = evaluator.traverse(
        &mut traversal,
        rec.finished_node(),
    );

    // A result ordered by structure would be: [2, 1, 8, 3, 7]
    // The result is currently ordered by rule ID:
    assert_eq!(results, &[3, 7, 2, 1, 8]);

    // 3  =  (2 - (0 * 3)) + 1
    // 7  =  ((2 - 0) * 3) + 1
    // 2  =  2 - (0 * (3 + 1))
    // 1  =  2 - ((0 * 3) + 1)
    // 8  =  (2 - 0) * (3 + 1)
}
