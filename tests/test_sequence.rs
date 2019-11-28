#[macro_use]
extern crate log;
extern crate env_logger;
extern crate cfg;
extern crate gearley;

mod helpers;

use cfg::Symbol;
use cfg::sequence::Separator::Trailing;
use cfg::earley::Grammar;

use gearley::forest::Bocage;
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;

use helpers::{SimpleEvaluator, Parse};

#[test]
fn test_sequence() {
    let _ = env_logger::try_init();
    let (plus, minus) = (1, 2);
    let tokens = &[plus, minus, plus, minus, plus, minus];
    let mut external = Grammar::new();
    let (start, plus, minus) = external.sym();
    external.sequence(start).separator(Trailing(minus)).inclusive(3, Some(3)).rhs(plus);
    external.set_start(start);

    let cfg = InternalGrammar::from_grammar(&external);
    let mut evaluator = SimpleEvaluator::new(
        |sym: Symbol| {
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
        |_, _: &mut Vec<i32>| unreachable!()
    );
    let bocage = Bocage::new(&cfg);
    let mut recognizer = Recognizer::new(&cfg, bocage);
    assert!(recognizer.parse(tokens));

    let mut traversal = recognizer.forest.traverse();

    let results = evaluator.traverse(&mut traversal, recognizer.finished_node().unwrap());

    assert_eq!(results, vec![6]);
}
