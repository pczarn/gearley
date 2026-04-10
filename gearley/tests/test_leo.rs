use cfg::{Cfg, Symbol};
use gearley::utils::RecognizerParseExt;
use gearley::{Bocage, DefaultGrammar, Forest, Recognizer};
use gearley_forest::Evaluate;

struct Eval;

impl Evaluate for Eval {
    type Elem = i32;

    fn leaf(&self, terminal: Symbol, _values: u32) -> Self::Elem {
        match terminal.usize() {
            2 => 1,
            3 => -1,
            4 => 0,
            _ => unreachable!(),
        }
    }

    fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem
    where
        Self::Elem: 'a,
    {
        args.count() as i32
    }

    fn nulling<'r>(&self, _symbol: Symbol, _results: &'r mut Vec<Self::Elem>) {
        unreachable!()
    }
}

#[test]
fn test_measure_leo() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr] = external.sym();
    external
        .rule(start)
        .rhs([rr])
        .rule(rr)
        .rhs([rep, rr])
        .rhs([rep]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, InstrBocage::new());
    assert!(rec.parse(&[rep, rep, rep, rep]).unwrap());
    assert_eq!(
        rec.into_forest(),
        InstrBocage {
            num_sums: 4,
            num_leaves: 5,
            num_products: 1,
            num_summands: 4,
            num_leo_products: 3,
        }
    );
}

#[test]
fn test_eval_ambiguous_leo() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr, a] = external.sym();
    external
        .rule(start)
        .rhs([rr])
        .rule(rr)
        .rhs([rep, rr])
        .rhs([rep])
        .rule(rep)
        .rhs([a])
        .rhs([a, a]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, Bocage::new(&cfg));
    assert!(rec.parse(&[a, a, a, a]).unwrap());
    let finished_node = rec.finished_node().unwrap();
    let result = rec.into_forest().evaluate(Eval, finished_node);
    assert_eq!(result.len(), 1);
}
#[test]
fn test_eval_leo() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr] = external.sym();
    external
        .rule(start)
        .rhs([rr])
        .rule(rr)
        .rhs([rep, rr])
        .rhs([rep]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, Bocage::new(&cfg));
    assert!(rec.parse(&[rep, rep, rep, rep]).unwrap());
    let finished_node = rec.finished_node().unwrap();
    let result = rec.into_forest().evaluate(Eval, finished_node);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 1);
}

#[derive(Default, Debug, Eq, PartialEq)]
struct InstrBocage {
    num_sums: usize,
    num_leaves: usize,
    num_products: usize,
    num_summands: usize,
    num_leo_products: usize,
}

impl InstrBocage {
    fn new() -> Self {
        Default::default()
    }
}

impl Forest for InstrBocage {
    type NodeRef = ();
    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 1;

    fn begin_sum(&mut self) {
        //
    }

    fn leaf(&mut self, token: cfg::Symbol, pos: u32, value: u32) -> Self::NodeRef {
        self.num_leaves += 1;
    }

    fn nulling(&mut self, token: cfg::Symbol) -> Self::NodeRef {
        //
    }

    fn product(
        &mut self,
        left_factor: Self::NodeRef,
        right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
        self.num_products += 1;
    }

    fn push_summand(&mut self, item: gearley::Item<Self::NodeRef>) {
        self.num_summands += 1;
    }

    fn sum(&mut self, lhs_sym: cfg::Symbol, origin: u32) -> Self::NodeRef {
        self.num_sums += 1;
    }

    fn leo_product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef) -> Self::NodeRef {
        self.num_leo_products += 1;
    }
}
