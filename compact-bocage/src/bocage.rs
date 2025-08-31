use cfg_symbol::{Symbol, SymbolSource};
use gearley_forest::{item::Item, node_handle::NodeHandle};
use gearley_forest::Forest;
use gearley_grammar::{ForestInfo, Grammar};

use crate::graph::Graph;
use crate::node::{Node, NULL_ACTION};

#[derive(Debug)]
pub struct Bocage {
    pub(crate) graph: Graph,
    pub(crate) forest_info: ForestInfo,
    sum: Option<u32>,
    pub(crate) nulling_limit: u32,
}

impl Bocage {
    pub fn new<G: Grammar>(grammar: G) -> Self {
        Self::with_capacity(grammar, 1024)
    }

    pub fn with_capacity<G: Grammar>(grammar: G, graph_cap: usize) -> Self {
        let mut result = Bocage {
            graph: Graph::with_capacity(graph_cap),
            forest_info: grammar.forest_info(),
            sum: None,
            nulling_limit: 0,
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
        let mut nulling: Vec<_> = SymbolSource::generate_fresh().take(nulling_leaf_count + 1).map(|symbol| {
            Node::NullingLeaf {
                symbol,
            }
        }).collect();
        for &[lhs, rhs0, rhs1] in &self.forest_info.nulling_intermediate_rules {
            println!("RULE_NULLING at {:?}", lhs);
            nulling.push(Node::Rule { left_factor: NodeHandle::nulling(rhs0),
                    right_factor: NodeHandle::nulling(rhs1) });
            let factors = NodeHandle(nulling.len() as u32 - 1);
            nulling[NodeHandle::nulling(lhs).usize()] = Node::Product {
                    factors,
                    action: NULL_ACTION,
            };
        }
        for node in nulling {
            match node {
                Node::Product { action, factors } => {
                    self.graph.push_expanded(Node::Product { action, factors: factors.mul() });
                }
                Node::Rule { left_factor, right_factor } => {
                    self.graph.push_expanded(Node::Rule { left_factor: left_factor.mul(), right_factor: right_factor.mul() });
                }
                Node::NullingLeaf { .. } => {
                    self.graph.push_expanded(node);
                }
                _ => unreachable!()
            }
        }
        self.nulling_limit = self.graph.len();
    }

    fn nulling_symbol_count(&self) -> usize {
        self.forest_info.max_nulling_symbol().unwrap_or(0)
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
    pub(crate) fn postprocess_product_tree_node(&mut self, node: Node) -> Node {
        if let Node::Product {
            factors,
            action,
        } = node
        {
            if let Node::Rule { left_factor, right_factor } = self.get(factors) {
                return node;
            }
            if action == NULL_ACTION {
                return node;
            }
            // Add omitted phantom syms here.
            if let Some((sym, dir)) = self.forest_info.nulling_eliminated[action as usize] {
                // println!("NODE {:?}", self[NodeHandle::nulling(sym)]);
                let (left, right) = if dir {
                    (factors, NodeHandle::nulling(sym).mul())
                } else {
                    (NodeHandle::nulling(sym).mul(), factors)
                };
                let rule = self.product(left, right);
                Node::Product {
                    factors: rule,
                    action,
                }
            } else {
                node
            }
        } else {
            node
        }
    }

    pub(crate) fn get(&self, index: NodeHandle) -> Node {
        self.graph.get(index.0 as usize)
    }
}

impl Forest for Bocage {
    type NodeRef = NodeHandle;
    type LeafValue = u32;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

    #[inline]
    fn begin_sum(&mut self) {
        self.sum = Some(self.graph.len());
        self.graph.push(Node::BeginSum);
    }

    #[inline]
    fn push_summand(&mut self, item: Item<Self::NodeRef>) {
        self.graph.push(
            Node::Product {
                action: item.dot,
                factors: item.node,
            }
        );
    }

    #[inline]
    fn product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef) -> Self::NodeRef {
        let result = NodeHandle(self.graph.len());
        self.graph.push(Node::Rule { left_factor, right_factor });
        result
    }

    #[inline]
    fn sum(&mut self, _lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        let result = self.sum.unwrap();
        self.sum = None;
        self.graph.push(Node::EndSum);
        NodeHandle(result)
    }

    #[inline]
    fn leaf(&mut self, token: Symbol, _pos: u32, value: Self::LeafValue) -> Self::NodeRef {
        let result = NodeHandle(self.graph.len());
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
        NodeHandle((token.usize() * 9) as u32)
    }
}

// impl IndexMut<NodeHandle> for Bocage {
//     fn index_mut(&mut self, index: NodeHandle) -> &mut Self::Output {
//         &mut self.graph[index.usize()]
//     }
// }
