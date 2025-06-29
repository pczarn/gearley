#[macro_use]
extern crate log;
extern crate cfg;
extern crate env_logger;
extern crate gearley;

use cfg::Cfg;

use gearley::{DefaultGrammar, NullForest, Recognizer, RecognizerParseExt};

#[test]
fn test_recognize_nested() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, nested, terminal] = external.sym();
    external
        .rule(start)
        .rhs([nested, terminal])
        .rule(nested)
        .rhs([terminal, terminal]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, NullForest);
    let finished = rec.parse(&[terminal; 3]).unwrap();
    assert!(finished);
}

#[test]
fn test_recognize_reset() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, nested, terminal] = external.sym();
    external
        .rule(start)
        .rhs([nested, terminal])
        .rule(nested)
        .rhs([terminal, terminal]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, NullForest);
    for _ in 0..1000 {
        let finished = rec.parse(&[terminal; 3]).unwrap();
        assert!(finished);
        rec.reset();
    }
}
