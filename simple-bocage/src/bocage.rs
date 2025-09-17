use std::ops::{Index, IndexMut};

use cfg_symbol::{Symbol, SymbolSource};
use gearley_forest::Forest;
use gearley_forest::{item::Item, node_handle::NodeHandle};
use gearley_grammar::{ForestInfo, Grammar};

use crate::node::{Node, NULL_ACTION};

#[derive(Debug)]
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
        self.graph.extend(
            SymbolSource::generate_fresh()
                .take(nulling_leaf_count + 1)
                .map(|symbol| Node::NullingLeaf { symbol }),
        );
        for &[lhs, rhs0, rhs1] in &self.forest_info.nulling_intermediate_rules {
            println!("RULE_NULLING at {:?}", lhs);
            self.graph.push(Node::Rule {
                left_factor: NodeHandle::nulling(rhs0),
                right_factor: NodeHandle::nulling(rhs1),
            });
            let factors = NodeHandle(self.graph.len() as u32 - 1);
            self.graph[NodeHandle::nulling(lhs).usize()] = Node::Product {
                factors,
                action: NULL_ACTION,
            };
        }
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
        println!("POSTPROCESS {:?} {:?}", node, self.forest_info);
        if let Node::Product { factors, action } = node {
            if let Node::Rule {
                left_factor,
                right_factor,
            } = self[factors]
            {
                return node;
            }
            if action == NULL_ACTION {
                return node;
            }
            // Add omitted phantom syms here.
            if let Some((sym, dir)) = self.forest_info.nulling_eliminated[action as usize] {
                // println!("NODE {:?}", self[NodeHandle::nulling(sym)]);
                let (left, right) = if dir {
                    (factors, NodeHandle::nulling(sym))
                } else {
                    (NodeHandle::nulling(sym), factors)
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
}

impl Forest for Bocage {
    type NodeRef = NodeHandle;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

    #[inline]
    fn begin_sum(&mut self) {
        // nothing to do
    }

    #[inline]
    fn push_summand(&mut self, item: Item<Self::NodeRef>) {
        self.graph.push(Node::Product {
            action: item.dot,
            factors: item.node,
        });
        self.summand_count += 1;
    }

    #[inline]
    fn product(
        &mut self,
        left_factor: Self::NodeRef,
        right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
        self.graph.push(Node::Rule {
            left_factor,
            right_factor,
        });
        NodeHandle(self.graph.len() as u32 - 1)
    }

    #[inline]
    fn leo_product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef)
            -> Self::NodeRef {
        self.graph.push(Node::LeoRule {
            left_factor,
            right_factor,
        });
        NodeHandle(self.graph.len() as u32 - 1)
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
    fn leaf(&mut self, token: Symbol, _pos: u32, value: u32) -> Self::NodeRef {
        let result = NodeHandle(self.graph.len() as u32);
        self.graph.push(Node::Leaf {
            symbol: token,
            values: value,
        });
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
