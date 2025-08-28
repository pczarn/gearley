use cfg::sequence::CfgSequenceExt;
use cfg::Cfg;
use cfg::sequence::Separator::Trailing;
use cfg::Symbol;

use gearley::{Bocage, DefaultGrammar, Grammar, Recognizer, RecognizerParseExt};
use gearley_forest::Evaluate;

struct Eval;

impl Evaluate for Eval {
    type Elem = i32;

    fn leaf(&self, terminal: Symbol, _values: u32) -> Self::Elem {
        match terminal.usize() {
            2 => 1,
            3 => -1,
            _ => unreachable!(),
        }
    }

    fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a {
        println!("ACTION {}", action);
        args.count() as i32
    }

    fn nulling<'r>(&self, _symbol: Symbol, _results: &'r mut Vec<Self::Elem>) {
        unreachable!()
    }
}

#[test]
fn test_sequence() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, plus, minus] = external.sym();
    external.sequence(start)
        .separator(Trailing(minus))
        .inclusive(3, Some(3))
        .rhs(plus);
    external.set_roots([start]);

    let cfg = DefaultGrammar::from_grammar(external);
    let tokens = &[plus, minus, plus, minus, plus, minus];
    let bocage = Bocage::new(&cfg);
    assert_eq!(cfg.to_internal(plus).map(|s| s.usize()), Some(2));
    assert_eq!(cfg.to_internal(minus).map(|s| s.usize()), Some(3));
    let mut recognizer = Recognizer::with_forest(&cfg, bocage);
    assert!(recognizer.parse(tokens).unwrap());

    let finished_node = recognizer.finished_node().expect("exhausted");

    let results = recognizer.into_forest().evaluate(Eval, finished_node);

    assert_eq!(results, vec![6]);
}
