use std::fmt;

use cfg_symbol::Symbol;

use crate::item::Item;

pub trait Forest<S = Symbol> {
    /// Reference to a node.
    type NodeRef: Copy + fmt::Debug;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize;

    fn begin_sum(&mut self);

    fn push_summand(&mut self, item: Item<Self::NodeRef>);

    fn sum(&mut self, lhs_sym: S, origin: u32) -> Self::NodeRef;

    fn product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef)
        -> Self::NodeRef;

    fn leo_product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef)
        -> Self::NodeRef;

    fn leaf(&mut self, token: S, pos: u32, value: u32) -> Self::NodeRef;

    fn nulling(&self, token: S) -> Self::NodeRef;
}
