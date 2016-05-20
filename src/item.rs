use std::cmp::Ordering;

pub type Dot = u32;
pub type Origin = u32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Item<N> {
    pub origin: Origin,
    pub dot: Dot,
    pub node: N,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItem<N> {
    pub dot: Dot,
    pub origin: Origin,
    pub left_node: N,
    pub right_node: Option<N>,
}

impl<L> PartialEq for CompletedItem<L> {
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
