use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::fmt;

use cfg::{Symbol, Symbol16};

pub trait PerformancePolicy: Default + fmt::Debug + Copy + Clone {
    type Dot: fmt::Debug + Default + Ord + Clone + Copy + Into<u32> + TryInto<usize> + TryFrom<usize> + TryFrom<u32>;
    type Symbol: fmt::Debug + Default + Ord + Clone + Copy + From<Symbol> + Into<Symbol>;

    const MEDIAL_SORT_THRESHOLD: usize;
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct DefaultPerformancePolicy;

impl PerformancePolicy for DefaultPerformancePolicy {
    type Dot = u32;
    type Symbol = Symbol;

    const MEDIAL_SORT_THRESHOLD: usize = 16;
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct PerformancePolicy16;

impl PerformancePolicy for PerformancePolicy16 {
    type Dot = u16;
    type Symbol = Symbol16;

    const MEDIAL_SORT_THRESHOLD: usize = 16;
}
