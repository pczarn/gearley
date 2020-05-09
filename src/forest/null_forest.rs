use cfg::symbol::Symbol;

use forest::Forest;

/// An empty forest.
pub struct NullForest;

impl Forest for NullForest {
    type NodeRef = ();
    type LeafValue = ();

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 0;

    #[inline(always)]
    fn leaf(&mut self, _: Symbol, _: u32, _: ()) {}
    #[inline(always)]
    fn nulling(&self, _: Symbol) {}
    #[inline(always)]
    fn product(&mut self, action: u32, left_node: Self::NodeRef, right_node: Option<Self::NodeRef>) -> Self::NodeRef {
        ()
    }
    #[inline(always)]
    fn begin_sum(&mut self, _lhs_sym: Symbol, _origin: u32) {}
    #[inline(always)]
    fn push_summand(&mut self, _product: Self::NodeRef) {}
    #[inline(always)]
    fn end_sum(&mut self, _lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        ()
    }
    fn end_earleme(&mut self) {}
}
