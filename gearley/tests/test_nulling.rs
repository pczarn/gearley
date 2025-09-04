use cfg::Cfg;
use cfg::Symbol;

use gearley_forest::Evaluate;
use simple_bocage::Bocage;

use gearley::{DefaultGrammar, Grammar, Recognizer, RecognizerParseExt};

struct NullingEval(Symbol);

impl Evaluate for NullingEval {
    type Elem = bool;

    fn leaf(&self, _terminal: Symbol, _values: u32) -> Self::Elem {
        false
    }

    fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>) {
        results.push(symbol == self.0);
    }

    fn product<'a>(&self, _action: u32, _args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
        false
    }
}

#[test]
fn test_trivial_grammar() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start] = external.sym();
    external.rule(start).rhs([]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::with_forest(&cfg, bocage);
    assert!(rec.parse(&[]).unwrap());
    let finished_node = rec.finished_node().unwrap();
    let results = rec.into_forest().evaluate(NullingEval(start), finished_node);
    assert_eq!(results, &[true]);
}

#[test]
fn test_grammar_with_nulling_intermediate() {
        struct NullingIntermediateEval {
            a: Symbol,
            foo: Symbol,
        }

        impl Evaluate for NullingIntermediateEval {
            type Elem = i32;

            fn leaf(&self, terminal: Symbol, _values: u32) -> Self::Elem {
                println!("LEAF {:?}", terminal);
                if terminal == self.foo {
                    3
                } else {
                    panic!("terminal {:?} is not {:?}", terminal, self.foo)
                }
            }
        
            fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>) {
                println!("NULLING {:?}", symbol);
                results.push(if symbol == self.a { 10 } else { 100 });
            }
        
            fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
                print!("PRODUCT {:?}", action);
                if action == 0 {
                    args.cloned().fold(0, |a, e| a + e)
                } else {
                    panic!("action {}", action)
                }
            }
        }

        let _ = env_logger::try_init();
        let mut external = Cfg::new();
        let [start, a, b, c, d, foo] = external.sym();
        external
            .rule(start)
            .rhs([a, b, c, d, foo])
            .rule(a)
            .rhs([])
            .rule(b)
            .rhs([])
            .rule(c)
            .rhs([])
            .rule(d)
            .rhs([]);
        external.set_roots([start]);
        let cfg = DefaultGrammar::from_grammar(external);
        let bocage = Bocage::new(&cfg);
        let mut rec = Recognizer::with_forest(&cfg, bocage);
        assert!(rec.parse(&[foo]).unwrap());
        let finished_node = rec.finished_node().expect("exhausted");
        let results = rec.into_forest().evaluate(NullingIntermediateEval { a: cfg.to_internal(a).unwrap(), foo: cfg.to_internal(foo).unwrap() }, finished_node);
        assert_eq!(results, &[313]);
}

#[test]
fn test_grammar_with_nulling_intermediate_compact() {
        struct NullingIntermediateEval {
            a: Symbol,
            foo: Symbol,
        }

        impl Evaluate for NullingIntermediateEval {
            type Elem = i32;

            fn leaf(&self, terminal: Symbol, _values: u32) -> Self::Elem {
                println!("LEAF {:?}", terminal);
                if terminal == self.foo {
                    3
                } else {
                    panic!("terminal {:?} is not {:?}", terminal, self.foo)
                }
            }
        
            fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>) {
                println!("NULLING {:?}", symbol);
                results.push(if symbol == self.a { 10 } else { 100 });
            }
        
            fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
                print!("PRODUCT {:?}", action);
                if action == 0 {
                    args.cloned().fold(0, |a, e| a + e)
                } else {
                    panic!("action {}", action)
                }
            }
        }

        let _ = env_logger::try_init();
        let mut external = Cfg::new();
        let [start, a, b, c, d, foo] = external.sym();
        external
            .rule(start)
            .rhs([a, b, c, d, foo])
            .rule(a)
            .rhs([])
            .rule(b)
            .rhs([])
            .rule(c)
            .rhs([])
            .rule(d)
            .rhs([]);
        external.set_roots([start]);
        let cfg = DefaultGrammar::from_grammar(external);
        let bocage = compact_bocage::Bocage::new(&cfg);
        let mut rec = Recognizer::with_forest(&cfg, bocage);
        assert!(rec.parse(&[foo]).unwrap());
        let finished_node = rec.finished_node().expect("exhausted");
        let results = rec.into_forest().evaluate(NullingIntermediateEval { a: cfg.to_internal(a).unwrap(), foo: cfg.to_internal(foo).unwrap() }, finished_node);
        assert_eq!(results, &[313]);
}
