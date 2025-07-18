use std::cmp::Ordering;

pub type Dot = u32;
pub type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct Item<N, O = Origin> {
    pub(crate) origin: O, // u32
    pub(crate) dot: Dot, // u32
    pub node: N, // u32
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItemLinked<N> {
    /// Left item idx.
    pub idx: u32,
    /// Right bocage node.
    pub node: Option<N>,
}

impl<L> PartialEq for Item<L> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<L> Eq for Item<L> {}

impl<L> PartialOrd for Item<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L> Ord for Item<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}
