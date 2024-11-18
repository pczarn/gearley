use std::cmp::Ordering;

pub type Dot = u32;
pub type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct Item<S, N> {
    pub(crate) origin: Origin,
    pub(crate) dot: Dot,
    pub node: N,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItemLinked<N> {
    /// Left item idx.
    pub idx: u32,
    /// Right bocage node.
    pub node: Option<N>,
}

impl<S, L> PartialEq for Item<S, L> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<S, L> Eq for Item<S, L> {}

impl<S, L> PartialOrd for Item<S, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, L> Ord for Item<S, L> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}
