#[cfg(feature = "memusage")]
use memusage::MemoryReport;

use crate::forest::Forest;
use crate::item::Item;

/// An empty forest.
pub struct NullForest;

impl<S: Copy> Forest<S> for NullForest {
    type NodeRef = ();

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 0;

    #[inline(always)]
    fn leaf(&mut self, _: S, _: u32, _: u32) {}
    #[inline(always)]
    fn nulling(&self, _: S) {}
    #[inline(always)]
    fn begin_sum(&mut self) {}
    #[inline(always)]
    fn push_summand(&mut self, _item: Item<Self::NodeRef>) {}
    #[inline(always)]
    fn product(
        &mut self,
        _left_factor: Self::NodeRef,
        _right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
    }
    #[inline(always)]
    fn leo_product(
        &mut self,
        _left_factor: Self::NodeRef,
        _right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
    }
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
