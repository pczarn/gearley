#[macro_use]
extern crate log;
extern crate cfg;
extern crate env_logger;
extern crate gearley;

mod helpers;

use cfg::earley::Grammar;
use cfg::Symbol;

use gearley::forest::Bocage;
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;
use gearley::policy::PerformancePolicy16;

use helpers::{Parse, SimpleEvaluator};

#[test]
fn test_math() {
    let _ = env_logger::try_init();
    let mut external = Grammar::new();
    let (start, expr, ident, lparen, rparen, num) = external.sym();
    external
        .rule(start)
            .rhs([expr])
        .rule(expr)
            .rhs([ident, lparen, expr, rparen])
            .rhs([expr, expr])
            .rhs([num])
            .rhs([ident])
            .rhs([lparen, expr, rparen]);
    external.set_start(start);
    let cfg = InternalGrammar::<PerformancePolicy16>::from_grammar(&external);
    let mut evaluator = SimpleEvaluator::new(
        |sym: Symbol| {
            if sym == num {
                "num".to_string()
            } else if sym == ident {
                "f".to_string()
            } else if sym == lparen {
                "(".to_string()
            } else if sym == rparen {
                ")".to_string()
            } else {
                unreachable!()
            }
        },
        |rule: u32, args: &[&String]| {
            match rule {
                0 => {
                    args[0].clone()
                }
                1 => {
                    format!("{}({})", args[0], args[2])
                }
                2 => {
                    format!("{} times {}", args[0], args[1])
                }
                3 | 4 => {
                    args[0].clone()
                }
                5 => {
                    format!("({})", args[1])
                }
                _ => unreachable!()
            }
        },
        |_, _: &mut Vec<String>| unreachable!(),
    );
    let bocage = Bocage::new(&cfg);
    let mut rec = Recognizer::new(&cfg, bocage);
    let ident = ident.usize() as u32;
    let lparen = lparen.usize() as u32;
    let rparen = rparen.usize() as u32;
    let num = num.usize() as u32;
    let finished = rec.parse(&[ident, lparen, num, rparen, ident, lparen, num, rparen, ident, lparen, num, rparen]);
    assert!(finished);
    let mut traversal = rec.forest.traverse();
    let results = evaluator.traverse(&mut traversal, rec.finished_node().unwrap());
    // assert_eq!(results, vec![
    //     "f times (num) times f times (num)".to_string(),
    //     "f times (num) times f(num)".to_string(),
    //     "f times (num) times f times (num)".to_string(),
    //     "f times (num) times f times (num)".to_string(),
    //     "f times (num) times f times (num)".to_string(),
    //     "f(num) times f times (num)".to_string(),
    //     "f times (num) times f times (num)".to_string(),
    //     "f(num) times f times (num)".to_string(),
    //     "f times (num) times f(num)".to_string(),
    //     "f(num) times f(num)".to_string(),
    // ]);
}
