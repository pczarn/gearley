use cfg_symbol::Symbol;

use crate::node::Node;

pub trait Order<S> {
    /// Apply the order to sum node alternatives.
    fn sum<'b>(&mut self, alternatives: &'b [Node]) -> &'b [Node] {
        alternatives
    }

    /// Apply the order to product node factors.
    fn product(&mut self, _factors: &[(Symbol, u32)]) -> Option<usize> {
        None
    }
}

#[derive(Default)]
pub struct NullOrder;

impl<S> Order<S> for NullOrder {}

impl NullOrder {
    pub fn new() -> Self {
        NullOrder
    }
}
