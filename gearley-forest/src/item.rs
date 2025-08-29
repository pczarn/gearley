use std::cmp;

pub type Dot = u32;
pub type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct Item<N, O = Origin> {
    /// The dot position.
    pub dot: Dot,
    /// The origin location.
    /// It comes after `dot`, so that (origin, dot) can be compared in a single instruction
    /// on little-endian systems.
    pub origin: O,
    /// Bocage node.
    pub node: N,
}

impl<L> PartialEq for Item<L> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<L> Eq for Item<L> {}

impl<L> PartialOrd for Item<L> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some((self.origin, self.dot).cmp(&(other.origin, other.dot)))
    }
}

impl<L> Ord for Item<L> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}
