use cfg::Cfg;
use cfg::Symbol;
use cfg::Symbolic;
use gearley_forest::Evaluate;

pub fn grammar() -> Cfg {
    let mut bnf = Cfg::new();
    let [expr, op, num, plus, minus, mul, div] = bnf.sym();
    bnf.rule(expr).rhs([expr, op, expr]).rhs([num]);
    bnf.rule(op).rhs([plus]).rhs([minus]).rhs([mul]).rhs([div]);

    for _ in 0..10 {
        let [sym] = bnf.sym();
        bnf.rule(num).rhs([sym, num]).rhs([sym]);
    }
    bnf.set_roots([expr]);
    bnf
}

pub struct AmbiguousArithEvaluator;

impl Evaluate<Symbol> for AmbiguousArithEvaluator {
    type Elem = i32;

    fn leaf(&self, terminal: Symbol, _values: u32) -> Self::Elem {
        [0, 0, 0, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9][terminal.usize()]
    }

    fn product<'a>(&self, action: u32, mut args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem {
        let a0 = args.next().copied().unwrap_or(!0);
        let a1 = args.next().copied().unwrap_or(!0);
        let a2 = args.next().copied().unwrap_or(!0);
    
        match action {
            0 => match a1 {
                0 => a0 + a2,
                1 => a0 - a2,
                2 => a0 * a2,
                3 => a0 / a2,
                _ => unreachable!(),
            },
            1 => a0,
    
            2 => 0,
            3 => 1,
            4 => 2,
            5 => 3,
    
            6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 => a0 * 10 + a1,
            7 | 9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 => a0,
            _ => unreachable!(),
        }
    }

    fn nulling(&self, _symbol: Symbol, _results: &mut Vec<Self::Elem>) {
        unreachable!()
    }
}

#[macro_export]
macro_rules! ambiguous_arith_rhs_elem {
    ('+') => {
        0
    };
    ('-') => {
        1
    };
    ('*') => {
        2
    };
    ('/') => {
        3
    };
    ('0') => {
        4
    };
    ('1') => {
        5
    };
    ('2') => {
        6
    };
    ('3') => {
        7
    };
    ('4') => {
        8
    };
    ('5') => {
        9
    };
    ('6') => {
        10
    };
    ('7') => {
        11
    };
    ('8') => {
        12
    };
    ('9') => {
        13
    };
    ($e:expr) => {
        $e
    };
}

#[macro_export]
macro_rules! ambiguous_arith {
    ($($e:tt)+) => (
        &[$(Symbol::new(ambiguous_arith_rhs_elem!($e) as u32 + 3),)+]
    )
}
