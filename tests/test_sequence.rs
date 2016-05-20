extern crate cfg;
extern crate gearley;

#[macro_use]
mod grammars;

use cfg::Symbol;
use cfg::sequence::Separator::Trailing;

use gearley::forest::depth_first::*;
use gearley::grammar::Grammar;
use gearley::recognizer::Recognizer;
use gearley::util::slice_builder::SliceBuilder;

use grammars::*;

#[test]
fn test_sequence() {
    let (plus, minus) = (1, 2);
    let tokens = &[plus, minus, plus, minus, plus, minus];
    let mut external = Grammar::new();
    let (start, plus, minus) = external.sym();
    external.sequence(start).separator(Trailing(minus)).inclusive(3, Some(3)).rhs(plus);
    external.set_start(start);

    let cfg = external.into_internal_grammar();
    let values = ValueArray::new();
    let mut evaluator = ArrayEvaluator::new(
        &values,
        ActionClosureEvaluator::new(
            |sym: Symbol, _: Option<&()>| {
                match sym.usize() {
                    1 => 1,
                    2 => -1,
                    _ => unreachable!()
                }
            },
            |rule: u32, args: &[&i32]| {
                if rule == 0 {
                    args.len() as i32
                } else {
                    unreachable!()
                }
            },
            |_, _: &mut SliceBuilder<i32>| unreachable!()
        )
    );
    let bocage: Bocage<(), i32> = Bocage::new(&cfg);
    let mut recognizer = Recognizer::new(&cfg, &bocage);
    recognizer.parse(tokens);

    let mut traversal = Traversal::new(&bocage, NullOrder::new());

    let mut tree = evaluator.traverse(&mut traversal, recognizer.finished_node()).iter();

    assert_eq!(tree.next(), Some(&6));
    assert_eq!(tree.next(), None);
}
