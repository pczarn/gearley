use crate::local_prelude::*;
use crate::memory_usage::MemoryUse;

use super::vec2d::Vec2d;
use super::binary_heap::BinaryHeap;

impl<F, G, P> MemoryReport for Recognizer<F, G, P>
where
    F: MemoryReport + Forest,
    G: MemoryReport + Grammar,
    P: PerformancePolicy,
{
    fn indirect(&self) -> usize {
        self.forest.memory_use()
            + self.predicted.memory_use()
            + self.medial.indirect()
            + self.complete.memory_use()
    }
}

impl<T> MemoryReport for Vec2d<T> {
    fn indirect(&self) -> usize {
        self.chart.indirect() + self.indices.indirect()
    }
}

impl<T> MemoryReport for BinaryHeap<T> {
    fn indirect(&self) -> usize {
        self.0.indirect()
    }
}
