use std::ops::{Index, IndexMut};

use gearley_forest::node_handle::NodeHandle;
use gearley_forest::Forest;
use gearley_grammar::{ForestInfo, Grammar};
use gearley_forest::completed_item::CompletedItem;

use crate::node::{Node, NULL_ACTION};

pub struct Bocage {
    pub(crate) graph: Vec<Node>,
    pub(crate) forest_info: ForestInfo,
    pub(crate) summand_count: u32,
}

impl Bocage {
    pub fn new<G: Grammar>(grammar: G) -> Self {
        Self::with_capacity(grammar, 1024)
    }

    pub fn with_capacity<G: Grammar>(grammar: G, graph_cap: usize) -> Self {
        let mut result = Bocage {
            graph: Vec::with_capacity(graph_cap),
            forest_info: grammar.forest_info(),
            summand_count: 0,
        };
        result.initialize_nulling();
        result
    }

    pub(crate) fn initialize_nulling(&mut self) {
        // TODO trivial grammar check
        // self.nulling_leaf_count = self.nulling_symbol_count();
        let nulling_leaf_count = self.nulling_symbol_count();
        // Ensure that `max` is not ridiculously large.
        assert!(nulling_leaf_count < (1 << 20), "invalid nullable symbol");
        self.graph.extend((0..=nulling_leaf_count).map(|i| {
            Node::NullingLeaf {
                symbol: i.into(),
            }
        }));
        for &[lhs, rhs0, rhs1] in self.forest_info.nulling_intermediate_rules {
            self.graph[NodeHandle::nulling(lhs).usize()] = Node::Product {
                    left_factor: NodeHandle::nulling(rhs0),
                    right_factor: Some(NodeHandle::nulling(rhs1)),
                    action: NULL_ACTION,
            };
        }
    }

    fn nulling_symbol_count(&self) -> usize {
        self.grammar.max_nulling_symbol().unwrap_or(0)
    }

    // #[inline]
    // fn summands(graph: &Node<G::Symbol>, node: NodeHandle) -> &[Node<G::Symbol>] {
    //     unsafe {
    //         match *graph.get_unchecked(node.usize()) {
    //             Node::Sum { count, .. } => {
    //                 // back
    //                 // let start = node.usize() - count as usize - 1;
    //                 // let end = node.usize() - 1;
    //                 let start = node.usize() + 1;
    //                 let end = node.usize() + count as usize + 1;
    //                 graph.get_unchecked(start..end)
    //             }
    //             _ => slice::from_ref(graph.get_unchecked(node.usize())),
    //         }
    //     }
    // }

    #[inline]
    pub(crate) fn postprocess_product_tree_node(&self, node: &Node<G::Symbol>) -> Node<G::Symbol> {
        if let &Node::Product {
            left_factor: factor,
            right_factor: None,
            action,
        } = node
        {
            // Add omitted phantom syms here.
            if let Some((sym, dir)) = self.grammar.nulling(action) {
                let (left, right) = if dir {
                    (factor, NodeHandle::nulling(sym))
                } else {
                    (NodeHandle::nulling(sym), factor)
                };
                Node::Product {
                    left_factor: left,
                    right_factor: Some(right),
                    action,
                }
            } else {
                *node
            }
        } else {
            *node
        }
    }

    #[inline]
    pub(crate) fn is_transparent(&self, action: u32) -> bool {
        action == NULL_ACTION || self.grammar.external_origin(action).is_none()
    }
}

impl Forest for Bocage {
    type NodeRef = NodeHandle;
    type LeafValue = u32;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

    #[inline]
    fn begin_sum(&mut self) {
        // nothing to do
    }

    #[inline]
    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>) {
        self.graph.push(
            Node::Product {
                action: item.dot(),
                left_factor: item.left_node,
                right_factor: item.right_node,
            }
        );
        self.summand_count += 1;
    }

    #[inline]
    fn sum(&mut self, lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        let result = {
            match self.summand_count {
                0 => unreachable!(),
                1 => NodeHandle(self.graph.len() as u32 - 1),
                summand_count => {
                    // Slower case: ambiguous node.
                    let first_summand_idx = self.graph.len() - summand_count as usize;
                    let first_summand = self.graph[first_summand_idx];
                    self.graph.push(first_summand);
                    self.graph[first_summand_idx] = Node::Sum {
                        nonterminal: lhs_sym,
                        count: self.summand_count as u32,
                    };
                    NodeHandle(first_summand_idx as u32)
                }
            }
        };
        self.summand_count = 0;
        result
    }

    #[inline]
    fn leaf(&mut self, token: Symbol, _pos: u32, value: Self::LeafValue) -> Self::NodeRef {
        let result = NodeHandle(self.graph.len() as u32);
        self.graph.push(
            Node::Leaf {
                symbol: token,
                values: value,
            }
        );
        result
    }

    #[inline]
    fn nulling(&self, token: Symbol) -> Self::NodeRef {
        NodeHandle::nulling(token)
    }
}

impl Index<NodeHandle> for Bocage {
    type Output = Node;

    fn index(&self, index: NodeHandle) -> &Self::Output {
        &self.graph[index.usize()]
    }
}

impl IndexMut<NodeHandle> for Bocage {
    fn index_mut(&mut self, index: NodeHandle) -> &mut Self::Output {
        &mut self.graph[index.usize()]
    }
}
