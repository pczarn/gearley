pub mod ambiguous_math;
pub mod precedenced_math;

pub static BNFS: &'static [[&'static str; 3]] = &[
    ["ambiguous_math", "v0.1", crate::ambiguous_math::BNF],
    ["precedenced_math", "v0.1", crate::precedenced_math::BNF],
];
