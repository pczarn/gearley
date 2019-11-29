use cfg::symbol::Symbol;

use super::node::CompactNode;

pub trait Order {
    /// Apply the order to sum node alternatives.
    fn sum<'b>(&mut self, alternatives: &'b [CompactNode]) -> &'b [CompactNode] {
        alternatives
    }

    /// Apply the order to product node factors.
    fn product(&mut self, _factors: &[(Symbol, u32)]) -> Option<usize> {
        None
    }
}

#[derive(Default)]
pub struct NullOrder;

impl Order for NullOrder {}

impl NullOrder {
    pub fn new() -> Self {
        NullOrder
    }
}
