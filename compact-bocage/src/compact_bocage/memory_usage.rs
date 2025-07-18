use memreport

impl<G> CompactBocage<G> {
    fn memory_use(&self) -> usize {
        self.graph.vec.memory_use() + self.gc.liveness.memory_use() + self.gc.dfs.memory_use()
    }
}
