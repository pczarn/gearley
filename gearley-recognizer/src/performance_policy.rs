use std::mem;

use crate::utils::vec2d::Vec2dCapacity;

use gearley_forest::completed_item::CompletedItem;

use super::item::Item;

const ITEMS_PER_SET: usize = 16;
const DEFAULT_TOKEN_LIMIT: usize = 64;
const DEFAULT_NUM_SYMS: usize = 128;
const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

// TODO: rename (to PerformanceHint??)

pub trait PerformancePolicy {
    fn completion_capacity(&self) -> usize;

    fn medial_capacity(&self) -> Vec2dCapacity;
}

pub struct DefaultPerformancePolicy {
    memory_limit: usize,
    token_limit: Option<usize>,
    num_syms: Option<usize>,
}

impl Default for DefaultPerformancePolicy {
    fn default() -> Self {
        Self {
            memory_limit: 1024,
            token_limit: None,
            num_syms: None,
        }
    }
}

impl DefaultPerformancePolicy {
    pub fn new(memory_limit: usize) -> Self {
        Self {
            memory_limit,
            token_limit: None,
            num_syms: None,
        }
    }

    pub fn with_token_count_and_num_syms(memory_limit: usize, token_count: usize, num_syms: usize) -> Self {
        Self {
            memory_limit,
            token_limit: Some(token_count),
            num_syms: Some(num_syms),
        }
    }

    fn indices_capacity(&self) -> usize {
        self.token_limit.unwrap_or(DEFAULT_TOKEN_LIMIT) + 1
    }

    fn bytes_per_set(&self) -> usize {
        (self.num_syms.unwrap_or(DEFAULT_NUM_SYMS) + 31) / 32 * 4
            + ITEMS_PER_SET * mem::size_of::<Item<()>>()
    }

    fn forest_use(&self) -> usize {
        self.memory_limit * FOREST_BYTES_PER_RECOGNIZER_BYTE / (FOREST_BYTES_PER_RECOGNIZER_BYTE + 1)
    }

    fn chart_use(&self) -> usize {
        self.memory_limit
            - self.forest_use()
            - self.indices_capacity() * mem::size_of::<usize>()
            - self.completion_capacity() * mem::size_of::<CompletedItem<usize>>()
    }

    fn set_use(&self) -> usize {
        self.chart_use() / self.bytes_per_set()
    }
}

impl PerformancePolicy for DefaultPerformancePolicy {
    fn completion_capacity(&self) -> usize {
        match self.memory_limit {
            0..=999 => 16,
            1000..=499_999 => 32,
            500_000..=2_000_000 => 64,
            _ => 128,
        }
    }

    fn medial_capacity(&self) -> Vec2dCapacity {
        Vec2dCapacity {
            chart_capacity: self.set_use() * ITEMS_PER_SET,
            indices_capacity: self.set_use(),
        }
    }
}


