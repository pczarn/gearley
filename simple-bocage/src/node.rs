pub use self::Node::*;
use gearley_forest::node_handle::NodeHandle;

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone, Debug)]
pub enum Node {
    Sum {
        /// 8 bytes.
        /// Invariant: count > 1.
        /// Invariant: This node can only be directly followed by `Product`.
        nonterminal: Symbol,
        count: u32,
    },
    Product {
        /// 12+ bytes.
        action: u32,
        left_factor: NodeHandle,
        right_factor: Option<NodeHandle>,
    },
    Leaf {
        /// 8 bytes.
        symbol: Symbol,
        values: u32,
    },
    NullingLeaf {
        /// 4 bytes.
        symbol: Symbol,
    },
    Evaluated {
        /// 8 bytes.
        symbol: Symbol,
        values: u32,
    },
}

pub const NULL_ACTION: u32 = u32::MAX;
