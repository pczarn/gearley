extern crate cfg;
extern crate gearley;

mod grammars;

use gearley::forest::NullForest;
use gearley::grammar::Grammar;
use gearley::recognizer::Recognizer;

use grammars::*;

#[test]
fn test_recognize_nested() {
    let mut external = Grammar::new();
    let (start, nested, terminal) = external.sym();
    external.rule(start).rhs([nested, terminal])
            .rule(nested).rhs([terminal, terminal]);
    external.set_start(start);
    let cfg = external.to_internal_grammar();
    let bocage = NullForest;
    let mut rec = Recognizer::new(&cfg, &bocage);
    rec.parse(&[terminal.usize() as u32; 3]);
}

#[test]
fn test_recognize_reset() {
    let mut external = Grammar::new();
    let (start, nested, terminal) = external.sym();
    external.rule(start).rhs([nested, terminal])
            .rule(nested).rhs([terminal, terminal]);
    external.set_start(start);
    let cfg = external.to_internal_grammar();
    let bocage = NullForest;
    let mut rec = Recognizer::new(&cfg, &bocage);
    for _ in 0..100 {
        let is_finished = rec.parse(&[terminal.usize() as u32; 3]);
        assert!(is_finished);
        rec.reset();
    }
}
