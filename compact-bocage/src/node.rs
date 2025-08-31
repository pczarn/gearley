pub use self::Node::*;
use cfg_symbol::Symbol;
use gearley_forest::node_handle::NodeHandle;

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone, Debug)]
pub enum Node {
    BeginSum,
    EndSum,
    Product {
        /// 8 bytes.
        action: u32,
        factors: NodeHandle,
    },
    Rule {
        // 8 bytes.
        left_factor: NodeHandle,
        right_factor: NodeHandle,
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
}

pub const NULL_ACTION: u32 = u32::MAX;
