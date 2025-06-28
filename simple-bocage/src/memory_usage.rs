use gearley_grammar::ForestInfo;
use memusage::MemoryReport;

use crate::node::Node;

use super::Bocage;

// impl MemoryReport for Bocage {
//     fn children(&self) -> usize {
//         self.graph.children() + self.forest_info.children()
//     }
// }

// impl MemoryReport for Node {}

// impl MemoryReport for ForestInfo {
//     fn children(&self) -> usize {
//         self.eval.indirect()
//     }
// }
