use cfg::Cfg;
use gearley::utils::RecognizerParseExt;
use gearley::{DefaultGrammar, Forest, Recognizer};

#[test]
fn test_leo() {
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
            num_sums: 18,
            num_leaves: 5,
            num_products: 10,
            num_summands: 18
        }
    );
}

#[derive(Default, Debug, Eq, PartialEq)]
struct InstrBocage {
    num_sums: usize,
    num_leaves: usize,
    num_products: usize,
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

    fn push_summand(&mut self, item: gearley::Item<Self::NodeRef>) {
        self.num_summands += 1;
    }

    fn sum(&mut self, lhs_sym: cfg::Symbol, origin: u32) -> Self::NodeRef {
        self.num_sums += 1;
    }
}
