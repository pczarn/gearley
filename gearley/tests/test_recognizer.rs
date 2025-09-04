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

#[test]
fn test_recognize_gensym() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, a, b, c, d, e] = external.sym();
    external
        .rule(start)
        .rhs([a, b, c, d])
        .rule(c)
        .rhs([])
        .rhs([e])
        .rule(b)
        .rhs([e]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, NullForest);
    for _ in 0..1000 {
        let finished = rec.parse(&[a, e, d]).unwrap();
        assert!(finished);
        rec.reset();
    }
}
