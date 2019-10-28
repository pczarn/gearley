pub mod null_forest;
pub mod order;
pub mod traverse;
pub(crate) mod bocage;
pub(crate) mod node;

pub use self::bocage::Bocage;
pub use self::null_forest::NullForest;

use std::fmt;
use cfg::Symbol;

use item::CompletedItem;

pub trait Forest {
    /// Reference to a node.
    type NodeRef: Copy + fmt::Debug;
    type LeafValue;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize;

    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>);

    fn sum(&mut self, lhs_sym: Symbol, origin: u32) -> Self::NodeRef;

    fn leaf(&mut self, token: Symbol, pos: u32, value: Self::LeafValue) -> Self::NodeRef;

    fn nulling(&self, token: Symbol) -> Self::NodeRef;
}
