use memusage::MemoryReport;

use super::Bocage;

impl<G> MemoryReport for Bocage<G> {
    fn indirect(&self) -> usize {
        self.graph.memory_use() + self.gc.liveness.memory_use() + self.gc.dfs.memory_use()
    }
}
