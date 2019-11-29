pub mod bocage;
pub mod compact_bocage;
pub mod node_handle;
pub mod null_forest;

pub use self::bocage::Bocage;
pub use self::compact_bocage::CompactBocage;
pub use self::null_forest::NullForest;

use cfg::Symbol;
use std::fmt;

use item::CompletedItem;

pub trait Forest {
    /// Reference to a node.
    type NodeRef: Copy + fmt::Debug;
    type LeafValue;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize;

    fn begin_sum(&mut self);

    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>);

    fn sum(&mut self, lhs_sym: Symbol, origin: u32) -> Self::NodeRef;

    fn leaf(&mut self, token: Symbol, pos: u32, value: Self::LeafValue) -> Self::NodeRef;

    fn nulling(&self, token: Symbol) -> Self::NodeRef;
}
