#[cfg(feature = "memusage")]
use memusage::MemoryReport;

use crate::forest::Forest;
use crate::completed_item::CompletedItem;

/// An empty forest.
pub struct NullForest;

impl<S: Copy> Forest<S> for NullForest {
    type NodeRef = ();
    type LeafValue = ();

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 0;

    #[inline(always)]
    fn leaf(&mut self, _: S, _: u32, _: ()) {}
    #[inline(always)]
    fn nulling(&self, _: S) {}
    #[inline(always)]
    fn begin_sum(&mut self) {}
    #[inline(always)]
    fn push_summand(&mut self, _item: CompletedItem<Self::NodeRef>) {}
    #[inline(always)]
    fn sum(&mut self, _lhs_sym: S, _origin: u32) -> Self::NodeRef {
        ()
    }
}

#[cfg(feature = "memusage")]
impl MemoryReport for NullForest {
    fn indirect(&self) -> usize {
        0
    }
}
