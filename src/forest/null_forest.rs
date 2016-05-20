use cfg::symbol::Symbol;

use forest::{Forest, NodeBuilder};
use item::CompletedItem;

/// An empty forest.
pub struct NullForest;
pub struct NullBuilder;

impl<'a> Forest<'a> for NullForest {
    type NodeRef = ();
    type NodeBuilder = NullBuilder;
    type LeafValue = ();

    #[inline(always)] fn build(&'a self, _: usize) -> NullBuilder { NullBuilder }
    #[inline(always)] fn leaf(&'a self, _: Symbol, _: u32, _: ()) {}
    #[inline(always)] fn nulling(&'a self, _: Symbol) {}
}

impl NodeBuilder for NullBuilder {
    type NodeRef = ();

    fn push(&mut self, _item: CompletedItem<Self::NodeRef>) {}
    fn sum(&mut self, _origin: u32) -> Self::NodeRef { () }
    fn reserve(&mut self, _len: usize) {}
}
