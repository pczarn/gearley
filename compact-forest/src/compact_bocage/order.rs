use std::cell::Cell;

use cfg::symbol::Symbol;

pub trait Order {
    /// Apply the order to sum node alternatives.
    fn sum<'b>(&mut self, alternatives: &'b [Cell<u16>]) -> &'b [Cell<u16>] {
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
