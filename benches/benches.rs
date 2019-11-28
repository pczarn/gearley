#![feature(test)]

extern crate test;
extern crate cfg;
extern crate gearley;

macro_rules! trace(($($tt:tt)*) => ());

#[macro_use]
#[path = "../tests/grammars/mod.rs"]
mod grammars;
#[path = "../tests/helpers/mod.rs"]
mod helpers;

use gearley::grammar::InternalGrammar;
use gearley::forest::{Bocage, NullForest};
use gearley::recognizer::Recognizer;
use gearley::memory_use::MemoryUse;

use grammars::*;
use helpers::{SimpleEvaluator, Parse};

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
    let cfg = InternalGrammar::from_grammar(&external);

    b.iter(|| {
        let mut evaluator = SimpleEvaluator::new(
            ambiguous_arith::leaf,
            ambiguous_arith::rule,
            |_, _: &mut _| unreachable!()
        );
        let mut rec: Recognizer<Bocage<&'_ InternalGrammar>> = Recognizer::new_with_hint(&cfg, tokens.len());
        assert!(rec.parse(tokens));
        let mut traversal = rec.forest.traverse();
        let results = evaluator.traverse(&mut traversal, rec.finished_node().unwrap());
        test::black_box(results);
    })
}

#[bench]
fn bench_evaluate_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_arith::grammar();
    let cfg = InternalGrammar::from_grammar(&external);
    let sum_tokens = test::black_box(SUM_TOKENS);

    b.iter(|| {
        let mut evaluator = SimpleEvaluator::new(
            precedenced_arith::leaf,
            precedenced_arith::rule,
            |_, _: &mut _| unreachable!(),
        );
        let bocage = Bocage::new(&cfg);
        let mut recognizer = Recognizer::new(&cfg, bocage);
        recognizer.parse(sum_tokens);
        let mut traversal = recognizer.forest.traverse();
        let results = evaluator.traverse(&mut traversal, recognizer.finished_node().unwrap());
        test::black_box(results);
    })
}

#[bench]
fn bench_process_grammar_for_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_arith::grammar();

    b.iter(|| {
        test::black_box(InternalGrammar::from_grammar(&external));
    })
}

#[bench]
fn bench_recognize_precedenced_arith(b: &mut test::Bencher) {
    let grammar = precedenced_arith::grammar();
    let cfg = InternalGrammar::from_grammar(&grammar);
    let sum_tokens = test::black_box(SUM_TOKENS);

    b.iter(|| {
        let mut recognizer = Recognizer::new(&cfg, NullForest);
        test::black_box(&recognizer.parse(sum_tokens));
    })
}
