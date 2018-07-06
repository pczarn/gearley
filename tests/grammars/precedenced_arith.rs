use cfg::Symbol;
use cfg::earley::Grammar;

pub fn grammar() -> Grammar {
    let mut bnf = Grammar::new();
    let (sum, product, factor, number, plus, minus, mul, div, lparen, rparen) = bnf.sym();
    bnf.rule(sum).rhs([sum, plus, product])
                 .rhs([sum, minus, product])
                 .rhs([product])
       .rule(product).rhs([product, mul, factor])
                     .rhs([product, div, factor])
                     .rhs([factor])
       .rule(factor).rhs([lparen, sum, rparen])
                    .rhs([number]);
    for _ in 0..10 {
        let sym = bnf.sym();
        bnf.rule(number).rhs(&[sym, number])
                        .rhs(&[sym]);
    }
    bnf.set_start(sum);
    bnf
}

pub fn leaf(sym: Symbol) -> i32 {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9][sym.usize()]
}

pub fn rule(rule: u32, args: &[&i32]) -> i32 {
    let a0 = args.get(0).map(|f| **f).unwrap_or(!0);
    let a1 = args.get(1).map(|f| **f).unwrap_or(!0);
    let a2 = args.get(2).map(|f| **f).unwrap_or(!0);
    match rule {
        0 => a0 + a2,
        1 => a0 - a2,
        2 => a0,

        3 => a0 * a2,
        4 => a0 / a2,
        5 => a0,

        6 => a1,
        7 => a0,

        8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 | 26 => a0 * 10 + a1,
        9 | 11 | 13 | 15 | 17 | 19 | 21 | 23 | 25 | 27 => a0,
        _ => unreachable!(),
    }
}

#[macro_export]
macro_rules! precedenced_arith_rhs_elem {
    ('+') => (0);
    ('-') => (1);
    ('*') => (2);
    ('/') => (3);
    ('(') => (4);
    (')') => (5);
    ('0') => (6);
    ('1') => (7);
    ('2') => (8);
    ('3') => (9);
    ('4') => (10);
    ('5') => (11);
    ('6') => (12);
    ('7') => (13);
    ('8') => (14);
    ('9') => (15);
    ($e:expr) => ($e);
}

#[macro_export]
macro_rules! precedenced_arith {
    ($($e:tt)+) => (
        &[$(precedenced_arith_rhs_elem!($e) + 4,)+]
    )
}
