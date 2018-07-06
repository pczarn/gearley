use cfg::Symbol;
use cfg::earley::Grammar;

pub fn grammar() -> Grammar {
    let mut bnf = Grammar::new();
    let (expr, op, num, plus, minus, mul, div) = bnf.sym();
    bnf.rule(expr).rhs([expr, op, expr])
                  .rhs([num]);
    bnf.rule(op).rhs([plus])
                .rhs([minus])
                .rhs([mul])
                .rhs([div]);

    for _ in 0..10 {
        let sym = bnf.sym();
        bnf.rule(num).rhs([sym, num])
                     .rhs([sym]);
    }
    bnf.set_start(expr);
    bnf
}

pub fn leaf(sym: Symbol) -> i32 {
    [0, 0, 0, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9][sym.usize()]
}

pub fn rule(rule: u32, args: &[&i32]) -> i32 {
    let a0 = args.get(0).map(|f| **f).unwrap_or(!0);
    let a1 = args.get(1).map(|f| **f).unwrap_or(!0);
    let a2 = args.get(2).map(|f| **f).unwrap_or(!0);

    match rule {
        0 => {
            match a1 {
                0 => a0 + a2,
                1 => a0 - a2,
                2 => a0 * a2,
                3 => a0 / a2,
                _ => unreachable!(),
            }
        }
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

#[macro_export]
macro_rules! ambiguous_arith_rhs_elem {
    ('+') => (0);
    ('-') => (1);
    ('*') => (2);
    ('/') => (3);
    ('0') => (4);
    ('1') => (5);
    ('2') => (6);
    ('3') => (7);
    ('4') => (8);
    ('5') => (9);
    ('6') => (10);
    ('7') => (11);
    ('8') => (12);
    ('9') => (13);
    ($e:expr) => ($e);
}

#[macro_export]
macro_rules! ambiguous_arith {
    ($($e:tt)+) => (
        &[$(ambiguous_arith_rhs_elem!($e) + 3,)+]
    )
}
