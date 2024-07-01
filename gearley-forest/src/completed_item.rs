use std::cmp;

type Dot = u32;
type Origin = u32;

#[derive(Clone, Copy, Debug)]
pub struct CompletedItem<N> {
    /// The dot position.
    pub dot: Dot,
    /// The origin location.
    /// It comes after `dot`, so that (origin, dot) can be compared in a single instruction
    /// on little-endian systems.
    pub origin: Origin,
    /// Left bocage node.
    pub left_node: N,
    /// Right bocage node.
    pub right_node: Option<N>,
}

impl<L> PartialEq for CompletedItem<L> {
    fn eq(&self, other: &Self) -> bool {
        (self.origin, self.dot) == (other.origin, other.dot)
    }
}

impl<L> Eq for CompletedItem<L> {}

impl<L> PartialOrd for CompletedItem<L> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some((self.origin, self.dot).cmp(&(other.origin, other.dot)))
    }
}

impl<L> Ord for CompletedItem<L> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.origin, self.dot).cmp(&(other.origin, other.dot))
    }
}

// impl<N> Into<Item<N>> for CompletedItem<N> {
//     fn into(self) -> Item<N> {
//         Item {
//             origin: self.origin,
//             dot: self.dot,
//             node: self.left_node,
//         }
//     }
// }

impl<N> CompletedItem<N> {
    pub fn dot(&self) -> Dot {
        self.dot
    }
}
