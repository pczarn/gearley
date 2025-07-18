#![cfg(feature = "test-serde")]

use cfg::Cfg;
use gearley::{DefaultGrammar, Grammar};

use miniserde::json;

#[test]
fn test_serde() {
    let mut cfg = Cfg::new();
    let [test, a, b, c] = cfg.sym();
    cfg.rule(test).rhs([a, b, c]);
    cfg.set_roots([test]);
    let original = DefaultGrammar::from_grammar(cfg);
    let json = json::to_string(&original);
    println!("{}", json);
    match json::from_str::<DefaultGrammar>(&json) {
        Ok(x) => {
            assert_eq!(x.externalized_start_sym(), test);
            assert_eq!(x.num_rules(), original.num_rules());
        }
        Err(err) => {
            // if let Err(err) = serde_json::from_str::<DefaultGrammar>(&json) {
            //     panic!("{:?}", err);
            // }
            panic!()
        }
    }
}
