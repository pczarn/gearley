use std::{cmp, mem};

use crate::local_prelude::*;
use gearley_vec2d::Vec2dCapacity;

const ITEMS_PER_SET: usize = 16;
const DEFAULT_TOKEN_LIMIT: usize = 64;
const DEFAULT_NUM_SYMS: usize = 128;
const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

// TODO: rename (to PerformanceHint??)

pub trait PerfHint {
    const LOOKAHEAD: bool;
    const LEO: bool;
    type Symbol;

    fn completion_capacity(&self) -> usize {
        32
    }

    fn medial_capacity(&self) -> Vec2dCapacity {
        Vec2dCapacity {
            chart_capacity: 512,
            indices_capacity: 128,
        }
    }
}

pub struct DefaultPerfHint {
    memory_limit: usize,
    token_limit: Option<usize>,
    num_syms: Option<usize>,
}

impl Default for DefaultPerfHint {
    fn default() -> Self {
        Self {
            memory_limit: 1024,
            token_limit: None,
            num_syms: None,
        }
    }
}

impl DefaultPerfHint {
    pub fn new(memory_limit: usize) -> Self {
        Self {
            memory_limit,
            token_limit: None,
            num_syms: None,
        }
    }

    pub fn with_token_count_and_num_syms(
        memory_limit: usize,
        token_count: usize,
        num_syms: usize,
    ) -> Self {
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
        self.memory_limit * FOREST_BYTES_PER_RECOGNIZER_BYTE
            / (FOREST_BYTES_PER_RECOGNIZER_BYTE + 1)
    }

    fn chart_use(&self) -> isize {
        self.memory_limit as isize
            - self.forest_use() as isize
            - (self.indices_capacity() * mem::size_of::<usize>()) as isize
            - (self.completion_capacity() * mem::size_of::<Item<usize>>()) as isize
    }

    fn set_use(&self) -> usize {
        cmp::max(self.chart_use() / self.bytes_per_set() as isize, 32) as usize
    }
}

impl PerfHint for DefaultPerfHint {
    const LOOKAHEAD: bool = true;
    const LEO: bool = true;
    type Symbol = Symbol;

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
