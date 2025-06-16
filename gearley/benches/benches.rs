#![feature(test)]

extern crate cfg;
extern crate gearley;
extern crate test;

use gearley::*;
#[cfg(feature = "memusage")]
use gearley::recognizer::memory_usage::MemoryUse;

use gearley_example_grammars::{ambiguous_math, precedenced_math};

#[bench]
fn bench_ambiguous_arithmetic(b: &mut test::Bencher) {
    let tokens = ambiguous_math::tokenize("2-0*3+1/2+88+1/2");
    let external = ambiguous_math::grammar();
    let cfg = DefaultGrammar::from_grammar(external);

    b.iter(|| {
        let evaluator = ambiguous_math::Evaluator;
        let mut rec: Recognizer<&'_ DefaultGrammar, Bocage> =
            Recognizer::with_forest(&cfg, Bocage::new(&cfg));
        assert!(rec.parse(&tokens));
        let finished_node = rec.finished_node().expect("exhausted");
        let results = rec.into_forest().evaluate(evaluator, finished_node);
        test::black_box(results);
    })
}

#[bench]
fn bench_evaluate_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_math::grammar();
    let cfg = DefaultGrammar::from_grammar(external);
    let tokens = precedenced_math::tokenize("1+(2*3-4)/(55)-(54)*55+62-13-((36))");
    let sum_tokens = test::black_box(tokens);

    b.iter(|| {
        let evaluator = precedenced_math::Evaluator;
        let bocage = Bocage::new(&cfg);
        let mut recognizer = Recognizer::with_forest (&cfg, bocage);
        recognizer.parse(&sum_tokens);
        let finished_node = recognizer.finished_node().expect("exhausted");
        let results = recognizer.into_forest().evaluate(evaluator, finished_node);
        test::black_box(results);
    })
}

#[bench]
fn bench_process_grammar_for_precedenced_arith(b: &mut test::Bencher) {
    let external = precedenced_math::grammar();

    b.iter(|| {
        test::black_box(DefaultGrammar::from_grammar(external.clone()));
    })
}

#[bench]
fn bench_clone_grammar(b: &mut test::Bencher) {
    let external = precedenced_math::grammar();

    b.iter(|| {
        test::black_box(external.clone());
    })
}

#[bench]
fn bench_recognize_precedenced_arith(b: &mut test::Bencher) {
    let grammar = precedenced_math::grammar();
    let cfg = DefaultGrammar::from_grammar(grammar);
    let tokens = precedenced_math::tokenize("1+(2*3-4)/(55)-(54)*55+62-13-((36))");
    let sum_tokens = test::black_box(tokens);

    b.iter(|| {
        let mut recognizer: Recognizer<&DefaultGrammar, NullForest> = Recognizer::with_forest(&cfg, NullForest);
        test::black_box(&recognizer.parse(&sum_tokens));
    })
}
