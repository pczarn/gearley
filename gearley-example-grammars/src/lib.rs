pub mod ambiguous_math;
pub mod c;
pub mod precedenced_math;

pub static BNFS: &'static [[&'static str; 4]] = &[
    [
        "ambiguous_math",
        "Ambiguous math",
        "advanced",
        crate::ambiguous_math::BNF,
    ],
    [
        "precedenced_math",
        "Precedenced math",
        "advanced",
        crate::precedenced_math::BNF,
    ],
    ["c", "C with lexer", "c-lexer", crate::c::BNF],
];

pub fn get_examples(id: &str) -> Option<&'static [&'static str]> {
    match id {
        "precedenced_math" => Some(crate::precedenced_math::INPUTS),
        "ambiguous_math" => Some(crate::ambiguous_math::INPUTS),
        "c" => Some(crate::c::INPUTS),
        _ => None,
    }
}
