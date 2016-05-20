#![feature(test)]

extern crate test;
extern crate cfg;
extern crate gearley;

#[macro_use]
#[path = "../tests/grammars/mod.rs"]
mod grammars;

use gearley::forest::{Bocage, Traversal, NullForest};
use gearley::forest::depth_first::{NullOrder, ArrayEvaluator, ArrayStore, ActionClosureEvaluator};
use gearley::recognizer::Recognizer;

use grammars::*;

const SUM_TOKENS: &'static [u32] = precedenced_arith!(
    '1' '+' '(' '2' '*' '3' '-' '4' ')' '/'
    '(' '5' '5' ')' '-' '(' '5' '4' ')' '*'
    '5' '5' '+' '6' '2' '-' '1' '3' '-' '('
    '(' '3' '6' ')' ')'
);

#[bench]
fn bench_ambiguous_arithmetic(b: &mut test::Bencher) {
    let tokens = ambiguous_arith!('2' '-' '0' '*' '3' '+' '1' '/' '2' '+' '8' '8' '+' '1' '/' '2');
    let external = ambiguous_arith::grammar();
    let cfg = external.into_internal_grammar();

    b.iter(|| {
        let arena = ArrayStore::new();
        let closures = ActionClosureEvaluator::new(
            ambiguous_arith::leaf,
            ambiguous_arith::rule,
            |_, _: &mut _| unreachable!()
        );
        let mut evaluator = ArrayEvaluator::new(&arena, closures);
        let bocage = Bocage::new(&cfg);
        let mut rec = Recognizer::new(&cfg, &bocage);
        rec.parse(tokens);
        let mut traversal = Traversal::new(&bocage, NullOrder::new());
        let results = evaluator.traverse(
            &mut traversal,
            rec.finished_node(),
        );
        test::black_box(results);
    })
}

#[bench]
fn bench_evaluate_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_arith::grammar();
    let cfg = external.into_internal_grammar();
    let sum_tokens = test::black_box(SUM_TOKENS);

    b.iter(|| {
        let arena = ArrayStore::new();
        let mut evaluator = ArrayEvaluator::new(
            &arena,
            ActionClosureEvaluator::new(
                precedenced_arith::leaf,
                precedenced_arith::rule,
                |_, _: &mut _| unreachable!()
            )
        );
        let bocage = Bocage::new(&cfg);
        let mut recognizer = Recognizer::new(&cfg, &bocage);
        recognizer.parse(sum_tokens);
        let mut traversal = Traversal::new(&bocage, NullOrder::new());
        let results = evaluator.traverse(
            &mut traversal,
            recognizer.finished_node(),
        );
        test::black_box(results);
    })
}

#[bench]
fn bench_process_grammar_for_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_arith::grammar();

    b.iter(|| {
        test::black_box(&external.into_internal_grammar());
    })
}

#[bench]
fn bench_recognize_precedenced_arith(b: &mut test::Bencher) {
    let grammar = precedenced_arith::grammar().into_internal_grammar();
    let sum_tokens = test::black_box(SUM_TOKENS);

    b.iter(|| {
        let null_forest = NullForest;
        let mut recce = Recognizer::new(&grammar, &null_forest);
        test::black_box(&recce.parse(sum_tokens));
    })
}

// #[bench]
// fn bench_eval_sum(b: &mut test::Bencher) {
//     let external = precedenced_arith::grammar();
//     let cfg = external.into_internal_grammar();
//     let sum_tokens = test::black_box(SUM_TOKENS);

//     b.iter(|| {
//         let arena = ArrayStore::new();
//         let evaluator = ArrayEvaluator::new(
//             &arena,
//             ClosureActionEvaluator::new(
//                 precedenced_arith::leaf,
//                 precedenced_arith::rule,
//                 |_, _: &mut _| unreachable!()
//             )
//         );
//         let bocage = Bocage::new(&cfg);
//         let mut rec = Recognizer::new(&cfg, &bocage);
//         rec.parse(sum_tokens);
//         let mut traversal = Traversal::new(&bocage, NullOrder::new());
//         let results = traversal.traverse(
//             rec.finished_node(),
//             evaluator,
//         );
//         test::black_box(results);
//     })
// }
