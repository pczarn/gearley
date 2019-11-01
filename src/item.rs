use std::cmp::Ordering;

pub type Dot = u32;
pub type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct Item<N> {
    pub(in super) origin: Origin,
    pub(in super) dot: Dot,
    pub node: N,
}

#[derive(Clone, Copy, Debug)]
pub struct CompletedItem<N> {
    /// The dot position.
    pub(in super) dot: Dot,
    /// The origin location.
    /// It comes after `dot`, so that (origin, dot) can be compared in a single instruction
    /// on little-endian systems.
    pub(in super) origin: Origin,
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

impl<N> Into<Item<N>> for CompletedItem<N> {
    fn into(self) -> Item<N> {
        Item {
            origin: self.origin,
            dot: self.dot,
            node: self.left_node,
        }
    }
}
