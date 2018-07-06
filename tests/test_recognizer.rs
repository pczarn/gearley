#[macro_use]
extern crate log;
extern crate env_logger;
extern crate cfg;
extern crate gearley;

mod helpers;

use cfg::earley::Grammar;

use gearley::forest::NullForest;
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;

use helpers::Parse;

#[test]
fn test_recognize_nested() {
    let _ = env_logger::try_init();
    let mut external = Grammar::new();
    let (start, nested, terminal) = external.sym();
    external.rule(start).rhs([nested, terminal])
            .rule(nested).rhs([terminal, terminal]);
    external.set_start(start);
    let cfg = InternalGrammar::from_grammar(&external);
    let mut rec = Recognizer::new(&cfg, NullForest);
    let finished = rec.parse(&[terminal.usize() as u32; 3]);
    assert!(finished);
}

#[test]
fn test_recognize_reset() {
    let _ = env_logger::try_init();
    let mut external = Grammar::new();
    let (start, nested, terminal) = external.sym();
    external.rule(start).rhs([nested, terminal])
            .rule(nested).rhs([terminal, terminal]);
    external.set_start(start);
    let cfg = InternalGrammar::from_grammar(&external);
    let mut rec = Recognizer::new(&cfg, NullForest);
    for _ in 0..100 {
        let finished = rec.parse(&[terminal.usize() as u32; 3]);
        assert!(finished);
        rec.reset();
    }
}
