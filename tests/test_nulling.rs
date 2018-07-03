extern crate cfg;
extern crate gearley;

mod grammars;

use cfg::Symbol;
use gearley::forest::{Bocage, Traversal};
use gearley::forest::depth_first::{NullOrder, ArrayEvaluator, ValueArray, ActionClosureEvaluator};
use gearley::grammar::Grammar;
use gearley::recognizer::Recognizer;
use gearley::util::slice_builder::SliceBuilder;

use grammars::*;

#[test]
fn test_trivial_grammar() {
    let mut external = Grammar::new();
    let start = external.sym();
    external.rule(start).rhs([]);
    external.set_start(start);
    let cfg = external.to_internal_grammar();
    let values = ValueArray::new();
    let mut evaluator = ArrayEvaluator::new(
        &values,
        ActionClosureEvaluator::new(
            |_: Symbol, _: Option<&()>| unreachable!(),
            |_: u32, _: &[&bool]| unreachable!(),
            |sym, builder: &mut SliceBuilder<bool>| {
                builder.reserve(1);
                if sym == start {
                    builder.push(true);
                } else {
                    builder.push(false);
                }
            }
        )
    );
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::new(&cfg, &bocage);
    rec.parse(&[]);
    let mut traversal = Traversal::new(&bocage, NullOrder::new());
    let results = evaluator.traverse(
        &mut traversal,
        rec.finished_node(),
    );
    assert_eq!(results, &[true]);
}

#[test]
fn test_grammar_with_nulling_intermediate() {
    let mut external = Grammar::new();
    let (start, a, b, c, d, foo) = external.sym();
    external.rule(start).rhs([a, b, c, d, foo])
            .rule(a).rhs([])
            .rule(b).rhs([])
            .rule(c).rhs([])
            .rule(d).rhs([]);
    external.set_start(start);
    let cfg = external.to_internal_grammar();
    let values = ValueArray::new();
    let mut evaluator = ArrayEvaluator::new(
        &values,
        ActionClosureEvaluator::new(
            |sym: Symbol, _: Option<&()>| {
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
            |sym, builder: &mut SliceBuilder<i32>| {
                builder.reserve(1);
                if sym == a {
                    builder.push(1);
                } else {
                    builder.push(2);
                }
            }
        )
    );
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::new(&cfg, &bocage);
    rec.parse(&[foo.usize() as u32]);
    let mut traversal = Traversal::new(&bocage, NullOrder::new());
    let results = evaluator.traverse(
        &mut traversal,
        rec.finished_node(),
    );
    assert_eq!(results, &[10]);
}
