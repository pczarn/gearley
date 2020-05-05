use std::cmp::Ordering;
use std::convert::TryInto;

use policy::PerformancePolicy;

pub type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct Item<N, P> where P: PerformancePolicy {
    pub(crate) origin: Origin,
    pub(crate) dot: P::Dot,
    // pub(crate) rhs1: P::Symbol,
    pub node: N,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItem<N> {
    /// The dot position.
    pub(crate) dot: u32,
    /// The origin location.
    /// It comes after `dot`, so that (origin, dot) can be compared in a single instruction
    /// on little-endian systems.
    pub(crate) origin: Origin,
    /// Left bocage node.
    pub left_node: N,
    /// Right bocage node.
    pub right_node: Option<N>,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItemLinked<N> {
    /// Left item idx.
    pub idx: u32,
    /// Right bocage node.
    pub node: Option<N>,
}

impl<N, P: PerformancePolicy> PartialEq for Item<N, P> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<N, P: PerformancePolicy> Eq for Item<N, P> {}

impl<N, P: PerformancePolicy> PartialOrd for Item<N, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, P: PerformancePolicy> Ord for Item<N, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}

impl<N> PartialEq for CompletedItem<N> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<L> Eq for CompletedItem<L> {}

impl<L> PartialOrd for CompletedItem<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.origin, self.dot).cmp(&(other.origin, other.dot)))
    }
}

impl<L> Ord for CompletedItem<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}

impl<N, P: PerformancePolicy> Into<Item<N, P>> for CompletedItem<N> {
    fn into(self) -> Item<N, P> {
        Item {
            origin: self.origin,
            dot: self.dot.try_into().ok().unwrap(),
            node: self.left_node,
        }
    }
}
