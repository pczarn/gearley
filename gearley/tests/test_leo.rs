use cfg::{Cfg, Symbol};
use gearley::utils::RecognizerParseExt;
use gearley::{DefaultGrammar, Forest, PerfHint, Recognizer};

struct NoLeoPerfHint;

impl PerfHint for NoLeoPerfHint {
    const LEO: bool = false;
    const LOOKAHEAD: bool = true;
    type Symbol = Symbol;
}

#[test]
fn test_right_rec() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr, maybe] = external.sym();
    external
        .rule(start)
        .rhs([rr, rep])
        .rule(rr)
        .rhs([rep, maybe])
        .rhs([rep])
        .rule(maybe)
        .rhs([])
        .rhs([rr]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, InstrBocage::new());
    assert!(rec.parse(&[rep, rep, rep, rep, rep]).unwrap());
    assert_eq!(
        rec.into_forest(),
        InstrBocage {
            num_sums: 11,
            num_leaves: 6,
            num_products: 2,
            num_leo_products: 8,
            num_summands: 11
        }
    );
}

#[test]
fn test_leo() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr] = external.sym();
    external
        .rule(start)
        .rhs([rr, rep])
        .rule(rr)
        .rhs([rep, rr])
        .rhs([rep]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest(&cfg, InstrBocage::new());
    assert!(rec.parse(&[rep, rep, rep, rep, rep]).unwrap());
    assert_eq!(
        rec.into_forest(),
        InstrBocage {
            num_sums: 11,
            num_leaves: 6,
            num_products: 2,
            num_leo_products: 8,
            num_summands: 11
        }
    );
}

#[test]
fn test_no_leo() {
    let _ = env_logger::try_init();
    let mut external = Cfg::new();
    let [start, rep, rr] = external.sym();
    external
        .rule(start)
        .rhs([rr, rep])
        .rule(rr)
        .rhs([rep, rr])
        .rhs([rep]);
    external.set_roots([start]);
    let cfg = DefaultGrammar::from_grammar(external);
    let mut rec = Recognizer::with_forest_and_policy(&cfg, InstrBocage::new(), NoLeoPerfHint);
    assert!(rec.parse(&[rep, rep, rep, rep, rep]).unwrap());
    assert_eq!(
        rec.into_forest(),
        InstrBocage {
            num_sums: 17,
            num_leaves: 6,
            num_products: 12,
            num_leo_products: 0,
            num_summands: 17
        }
    );
}

#[derive(Default, Debug, Eq, PartialEq)]
struct InstrBocage {
    num_sums: usize,
    num_leaves: usize,
    num_products: usize,
    num_leo_products: usize,
    num_summands: usize,
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

    fn nulling(&self, token: cfg::Symbol) -> Self::NodeRef {
        //
    }

    fn product(
        &mut self,
        left_factor: Self::NodeRef,
        right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
        self.num_products += 1;
    }

    fn leo_product(
        &mut self,
        left_factor: Self::NodeRef,
        right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
        self.num_leo_products += 1;
    }

    fn push_summand(&mut self, item: gearley::Item<Self::NodeRef>) {
        self.num_summands += 1;
    }

    fn sum(&mut self, lhs_sym: cfg::Symbol, origin: u32) -> Self::NodeRef {
        self.num_sums += 1;
    }
}
