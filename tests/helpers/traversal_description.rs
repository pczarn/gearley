use cfg::Symbol;
use gearley::forest::node_handle::NodeHandle;

#[derive(Debug, Eq, PartialEq)]
pub enum TraversalItem {
    Sum {
        products: Vec<(u32, Vec<NodeHandle>)>,
    },
    Nulling {
        symbol: Symbol,
    },
    Leaf {
        symbol: Symbol,
    },
}
