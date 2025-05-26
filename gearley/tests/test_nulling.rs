#[macro_use]
extern crate log;
extern crate cfg;
extern crate env_logger;
extern crate gearley;

mod helpers;

use cfg::Cfg;
use cfg::{Symbol, Symbolic};

use gearley_forest::Evaluate;
use simple_bocage::Bocage;

use gearley::{DefaultGrammar, Recognizer, RecognizerParseExt};

struct NullingEval(Symbol);

impl Evaluate<Symbol> for NullingEval {
    type Elem = bool;

    fn leaf(&self, terminal: Symbol, values: u32) -> Self::Elem {
        unreachable!()
    }

    fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>) {
        results.push(symbol == self.0);
    }

    fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
        unreachable!()
    }
}

macro_rules! test_trivial_grammar {
    ($Bocage:ident, $SimpleEvaluator:ident) => {
        let _ = env_logger::try_init();
        let mut external = Cfg::new();
        let [start] = external.sym();
        external.rule(start).rhs([]);
        external.set_roots([start]);
        let cfg = DefaultGrammar::from_grammar(external);
        let bocage = $Bocage::new(&cfg);
        let mut rec = Recognizer::with_forest(&cfg, bocage);
        assert!(rec.parse(&[]));
        let finished_node = rec.finished_node().unwrap();
        let results = rec.into_forest().evaluate(NullingEval(start), finished_node);
        assert_eq!(results, &[true]);
    };
}

#[test]
fn test_trivial_grammar() {
    test_trivial_grammar!(Bocage, SimpleEvaluator);
}

// #[test]
// fn test_trivial_grammar_compact() {
//     test_trivial_grammar!(CompactBocage, SimpleCompactEvaluator);
// }

macro_rules! test_grammar_with_nulling_intermediate {
    ($Bocage:ident, $SimpleEvaluator:ident) => {
        struct NullingIntermediateEval {
            a: Symbol,
            foo: Symbol,
        }

        impl Evaluate<Symbol> for NullingIntermediateEval {
            type Elem = i32;

            fn leaf(&self, terminal: Symbol, values: u32) -> Self::Elem {
                if terminal == self.foo {
                    3
                } else {
                    unreachable!()
                }
            }
        
            fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>) {
                results.push(if symbol == self.a { 1 } else { 2 });
            }
        
            fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
                if action == 0 {
                    args.cloned().fold(0, |a, e| a + e)
                } else {
                    unreachable!()
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
        let bocage = $Bocage::new(&cfg);
        let mut rec = Recognizer::with_forest(&cfg, bocage);
        assert!(rec.parse(&[foo]));
        let finished_node = rec.finished_node().expect("exhausted");
        let results = rec.into_forest().evaluate(NullingIntermediateEval { a, foo }, finished_node);
        assert_eq!(results, &[10]);
    };
}

#[test]
fn test_grammar_with_nulling_intermediate() {
    test_grammar_with_nulling_intermediate!(Bocage, SimpleEvaluator);
}

// #[test]
// fn test_grammar_with_nulling_intermediate_compact() {
//     test_grammar_with_nulling_intermediate!(CompactBocage, SimpleCompactEvaluator);
// }
