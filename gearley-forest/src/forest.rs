use std::fmt;

use cfg_symbol::Symbol;

use crate::completed_item::CompletedItem;

pub trait Forest<S = Symbol> {
    /// Reference to a node.
    type NodeRef: Copy + fmt::Debug;
    type LeafValue: fmt::Debug;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize;

    fn begin_sum(&mut self);

    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>);

    fn sum(&mut self, lhs_sym: S, origin: u32) -> Self::NodeRef;

    fn leaf(&mut self, token: S, pos: u32, value: Self::LeafValue) -> Self::NodeRef;

    fn nulling(&self, token: S) -> Self::NodeRef;
}
