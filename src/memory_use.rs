use std::mem;

use bit_matrix::BitMatrix;
use bit_vec::BitVec;

use forest::node_handle::NodeHandle;
use forest::{Bocage, CompactBocage, Forest, NullForest};
use grammar::InternalGrammar;
use item::{CompletedItem, Item};
use recognizer::Recognizer;

const ITEMS_PER_SET: usize = 16;

pub trait MemoryUse {
    type Arg;

    fn memory_use(&self) -> usize;
    fn new_with_limit(arg: Self::Arg, memory_limit: usize) -> Self;
}

impl<'g, F> MemoryUse for Recognizer<'g, F>
where
    F: MemoryUse<Arg = &'g InternalGrammar> + Forest,
{
    type Arg = &'g InternalGrammar;

    fn memory_use(&self) -> usize {
        self.forest.memory_use()
            + self.predicted.memory_use()
            + self.medial.memory_use()
            + self.complete.memory_use()
            + self.indices.memory_use()
    }

    fn new_with_limit(grammar: &'g InternalGrammar, memory_limit: usize) -> Self {
        let forest_use_bytes = memory_limit * F::FOREST_BYTES_PER_RECOGNIZER_BYTE
            / (F::FOREST_BYTES_PER_RECOGNIZER_BYTE + 1);
        let complete_use = match memory_limit {
            0..=1000 => 16,
            1000..=500_000 => 32,
            500_000..=2_000_000 => 64,
            _ => 128,
        };
        let recognizer_use_bytes = memory_limit
            - forest_use_bytes
            - complete_use * mem::size_of::<CompletedItem<F::NodeRef>>();
        let bytes_per_set = mem::size_of::<usize>()
            + (grammar.num_syms() + 31) / 32 * 4
            + ITEMS_PER_SET * mem::size_of::<Item<F::NodeRef>>();
        let sets_use = recognizer_use_bytes / bytes_per_set;
        let mut recognizer = Recognizer {
            forest: F::new_with_limit(grammar, forest_use_bytes),
            grammar,
            // The initial location is 0.
            earleme: 0,
            // The first Earley set begins at 0 and ends at 0. The second Earley set begins at 0.
            indices: Vec::with_capacity(sets_use),
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(sets_use, grammar.num_syms()),
            medial: Vec::with_capacity(sets_use * ITEMS_PER_SET),
            complete: Vec::with_capacity(complete_use),
            lookahead_hint: None,
        };
        recognizer.indices.push(0);
        recognizer.indices.push(0);
        recognizer.predict(grammar.start_sym());
        recognizer
    }
}

impl<'g, F> Recognizer<'g, F>
where
    F: MemoryUse<Arg = &'g InternalGrammar> + Forest,
{
    #[inline]
    pub fn new_with_hint(grammar: &'g InternalGrammar, tokens: usize) -> Self {
        let forest_use_bytes = tokens * 16;
        let complete_use = match tokens {
            0..=200 => 16,
            200..=10_000 => 32,
            10_000..=100_000 => 64,
            _ => 128,
        };
        let mut recognizer = Recognizer {
            forest: F::new_with_limit(grammar, forest_use_bytes),
            grammar,
            // The initial location is 0.
            earleme: 0,
            // The first Earley set begins at 0 and ends at 0. The second Earley set begins at 0.
            indices: Vec::with_capacity(tokens + 1),
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(tokens + 1, grammar.num_syms()),
            medial: Vec::with_capacity(tokens * ITEMS_PER_SET),
            complete: Vec::with_capacity(complete_use),
            lookahead_hint: None,
        };
        recognizer.indices.push(0);
        recognizer.indices.push(0);
        recognizer.predict(grammar.start_sym());
        recognizer
    }
}

impl<'g> MemoryUse for Recognizer<'g, NullForest> {
    type Arg = &'g InternalGrammar;

    fn memory_use(&self) -> usize {
        self.forest.memory_use()
            + self.predicted.memory_use()
            + self.medial.memory_use()
            + self.complete.memory_use()
            + self.indices.memory_use()
    }

    fn new_with_limit(grammar: &'g InternalGrammar, memory_limit: usize) -> Self {
        let complete_use = match memory_limit {
            0..=1000 => 16,
            1000..=500_000 => 32,
            500_000..=2_000_000 => 64,
            _ => 128,
        };
        let recognizer_use_bytes =
            memory_limit - complete_use * mem::size_of::<CompletedItem<()>>();
        let bytes_per_set = mem::size_of::<usize>()
            + (grammar.num_syms() + 31) / 32 * 4
            + ITEMS_PER_SET * mem::size_of::<Item<()>>();
        let sets_use = recognizer_use_bytes / bytes_per_set;
        let mut recognizer = Recognizer {
            forest: NullForest,
            grammar,
            // The initial location is 0.
            earleme: 0,
            // The first Earley set begins at 0 and ends at 0. The second Earley set begins at 0.
            indices: Vec::with_capacity(sets_use),
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(sets_use, grammar.num_syms()),
            medial: Vec::with_capacity(sets_use * ITEMS_PER_SET),
            complete: Vec::with_capacity(complete_use),
            lookahead_hint: None,
        };
        recognizer.indices.push(0);
        recognizer.indices.push(0);
        recognizer.predict(grammar.start_sym());
        recognizer
    }
}

impl<T> MemoryUse for Vec<T> {
    type Arg = ();

    fn memory_use(&self) -> usize {
        self.capacity() * mem::size_of::<T>()
    }

    fn new_with_limit(_arg: (), memory_limit: usize) -> Self {
        let capacity = memory_limit / mem::size_of::<T>();
        Self::with_capacity(capacity)
    }
}

impl MemoryUse for BitMatrix {
    type Arg = usize;

    fn memory_use(&self) -> usize {
        let (rows, columns) = self.size();
        rows * ((columns + 31) / 32 * 4)
    }

    fn new_with_limit(num_columns: usize, memory_limit: usize) -> Self {
        let row_size = (num_columns + 31) / 32 * 4;
        let capacity = memory_limit / row_size;
        Self::new(capacity, num_columns)
    }
}

impl MemoryUse for BitVec {
    type Arg = ();

    fn memory_use(&self) -> usize {
        (self.capacity() + 31) / 32 * 4
    }

    fn new_with_limit(_arg: (), memory_limit: usize) -> Self {
        let capacity = memory_limit * 8;
        Self::with_capacity(capacity)
    }
}

impl MemoryUse for NullForest {
    type Arg = ();

    fn memory_use(&self) -> usize {
        0
    }

    fn new_with_limit(_arg: (), _memory_limit: usize) -> Self {
        NullForest
    }
}

impl<'g> MemoryUse for Bocage<&'g InternalGrammar> {
    type Arg = &'g InternalGrammar;

    fn memory_use(&self) -> usize {
        self.graph.memory_use() + self.gc.liveness.memory_use() + self.gc.dfs.memory_use()
    }

    fn new_with_limit(grammar: &'g InternalGrammar, memory_limit: usize) -> Self {
        let dfs_size = match memory_limit {
            0..=1000 => 8,
            1000..=100_000 => 32,
            _ => 64,
        };
        let remaining_use = memory_limit - dfs_size * std::mem::size_of::<NodeHandle>();
        let bytes_per_node = mem::size_of::<u16>() as f32 + 1.0 / 8.0;
        let graph_size = (remaining_use as f32 / bytes_per_node) as usize;
        Bocage::with_capacities(grammar, graph_size, dfs_size)
    }
}

impl<'g> MemoryUse for CompactBocage<&'g InternalGrammar> {
    type Arg = &'g InternalGrammar;

    fn memory_use(&self) -> usize {
        self.graph.vec.memory_use() + self.gc.liveness.memory_use() + self.gc.dfs.memory_use()
    }

    fn new_with_limit(grammar: &'g InternalGrammar, memory_limit: usize) -> Self {
        let dfs_size = match memory_limit {
            0..=1000 => 8,
            1000..=100_000 => 32,
            _ => 64,
        };
        let remaining_use = memory_limit - dfs_size * std::mem::size_of::<NodeHandle>();
        let bytes_per_node = mem::size_of::<u16>() as f32 + 1.0 / 8.0;
        let graph_size = (remaining_use as f32 / bytes_per_node) as usize;
        CompactBocage::with_capacities(grammar, graph_size, dfs_size)
    }
}
