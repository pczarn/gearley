pub mod node;
pub mod order;
pub mod traverse;

use std::borrow::Borrow;
use std::hint;

use bit_vec::BitVec;
use cfg::symbol::Symbol;

use forest::node_handle::NodeHandle;
use forest::Forest;
use grammar::InternalGrammar;
use item::CompletedItem;

use self::node::Node::*;
use self::node::{Graph, Node, NULL_ACTION};
use self::order::Order;

pub struct CompactBocage<G> {
    pub(crate) graph: Graph,
    pub(crate) gc: MarkAndSweep,
    pub(crate) grammar: G,
    pub(crate) first_summand: NodeHandle,
    pub(crate) summand_count: u32,
}

pub(crate) struct MarkAndSweep {
    pub(crate) liveness: BitVec,
    // List for DFS and/or maybe relocation of stuff in the future.
    pub(crate) dfs: Vec<NodeHandle>,
}

impl<G> CompactBocage<G>
where
    G: Borrow<InternalGrammar>,
{
    pub fn new(grammar: G) -> Self {
        Self::with_capacities(grammar, 1024, 32)
    }

    pub fn with_capacities(grammar: G, graph_cap: usize, dfs_cap: usize) -> Self {
        let mut result = CompactBocage {
            graph: Graph::with_capacity(graph_cap),
            gc: MarkAndSweep {
                liveness: BitVec::with_capacity(graph_cap),
                dfs: Vec::with_capacity(dfs_cap),
            },
            grammar,
            summand_count: 0,
            first_summand: NodeHandle(0),
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
        let mut graph: Vec<Node> = (0..nulling_leaf_count)
            .map(|i| NullingLeaf {
                symbol: Symbol::from(i),
            })
            .collect();
        for &(lhs, rhs0, rhs1) in self.grammar.borrow().eliminated_nulling_intermediate() {
            graph[lhs.usize()] = Product {
                left_factor: NodeHandle::nulling(rhs0),
                right_factor: Some(NodeHandle::nulling(rhs1)),
                action: NULL_ACTION,
            };
        }
        let mut pos = 0;
        let mut relocation = vec![];
        for node in &graph {
            relocation.push(NodeHandle(pos));
            pos += node.classify(pos).size() as u32;
        }
        for node in graph {
            match node {
                Product {
                    action,
                    left_factor,
                    right_factor,
                } => {
                    self.graph.push(Product {
                        action,
                        left_factor: relocation[left_factor.usize()],
                        right_factor: right_factor.map(|f| relocation[f.usize()]),
                    });
                }
                other => {
                    self.graph.push(other);
                }
            }
        }
    }

    fn nulling_symbol_count(&self) -> usize {
        // why 1?
        self.grammar
            .borrow()
            .max_nulling_symbol()
            .map_or(1, |m| m + 1)
    }

    #[inline]
    pub fn mark_alive<O: Order>(&mut self, root: NodeHandle, _order: O) {
        self.gc.liveness.clear();
        self.gc.liveness.grow(self.graph.vec.len(), false);
        self.gc.dfs.push(root);
        while let Some(node) = self.gc.dfs.pop() {
            self.gc.liveness.set(node.usize(), true);
            let summands = CompactBocage::<G>::summands(&self.graph, node);
            // let summands = order.sum(summands);
            for summand in summands {
                // TODO: use order for products.
                self.gc.dfs_queue_factors(summand);
            }
        }
    }

    #[inline]
    fn summands<'a>(graph: &'a Graph, node: NodeHandle) -> impl Iterator<Item = Node> + 'a {
        let mut iter = graph.iter_from(node);
        match iter.peek() {
            Some(Sum { count, .. }) => {
                iter.next();
                iter.take(count as usize)
            }
            _ => iter.take(1),
        }
    }

    #[inline]
    fn process_product_tree_node(&self, mut node: Node) -> Node {
        match node {
            Product {
                ref mut left_factor,
                ref mut right_factor,
                action,
            } => {
                if right_factor.is_none() {
                    // Add omitted phantom syms here.
                    if let Some((sym, dir)) = self.grammar.borrow().nulling(action) {
                        let (left, right) = if dir {
                            (*left_factor, NodeHandle::nulling(sym))
                        } else {
                            (NodeHandle::nulling(sym), *left_factor)
                        };
                        *left_factor = left;
                        *right_factor = Some(right);
                    }
                }
            }
            _ => {}
        }
        node
    }

    #[inline]
    pub(super) fn is_transparent(&self, action: u32) -> bool {
        action == NULL_ACTION || self.grammar.borrow().external_origin(action).is_none()
    }

    // fn mark_and_sweep(&mut self, root: NodeHandle) {
    //     self.mark_alive(root);
    //     self.sweep_garbage();
    //     self.update_nulling_leaf_count();
    // }

    // fn sweep_garbage(&mut self) {
    //     let count = self.relocate_marked();
    //     self.graph.truncate(count);
    // }

    // fn update_nulling_leaf_count(&mut self) {
    //     let prev_count = self.nulling_leaf_count;
    //     self.nulling_leaf_count = self.gc.liveness.iter().take(prev_count).filter(|x| x).count();
    // }

    // fn relocate_marked(&mut self) -> usize {
    //     let mut destination = self.graph.iter();
    //     let mut count = 0;
    //     // ... TODO: relocate
    //     for (alive, source) in self.gc.liveness.iter().zip(self.graph.iter()) {
    //         if alive {
    //             destination.next().unwrap().cell.set(*source);
    //             count += 1;
    //         }
    //     }
    //     count
    // }
}

impl MarkAndSweep {
    #[inline]
    fn dfs_queue_factors(&mut self, summand: Node) {
        match summand {
            Product {
                left_factor,
                right_factor,
                ..
            } => {
                if let Some(factor) = right_factor {
                    if let Some(false) = self.liveness.get(factor.usize()) {
                        self.dfs.push(factor);
                    }
                }
                if let Some(false) = self.liveness.get(left_factor.usize()) {
                    self.dfs.push(left_factor);
                }
            }
            NullingLeaf { .. } | Evaluated { .. } => {}
            Sum { .. } => unreachable!(),
        }
    }
}

impl<G> Forest for CompactBocage<G>
where
    G: Borrow<InternalGrammar>,
{
    type NodeRef = NodeHandle;
    type LeafValue = u32;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

    #[inline]
    fn begin_sum(&mut self) {
        self.first_summand = NodeHandle(self.graph.vec.len() as u32);
    }

    #[inline]
    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>) {
        self.graph.push(self.process_product_tree_node(Product {
            action: item.dot,
            left_factor: item.left_node,
            right_factor: item.right_node,
        }));
        self.summand_count += 1;
    }

    #[inline]
    fn sum(&mut self, lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        unsafe {
            match self.summand_count {
                0 => hint::unreachable_unchecked(),
                1 => {}
                summand_count => {
                    // Slower case: ambiguous node.
                    let sum = Sum {
                        nonterminal: lhs_sym,
                        count: summand_count,
                    };
                    self.graph.set_up(self.first_summand, sum);
                }
            }
        };
        let result = self.first_summand;
        self.summand_count = 0;
        result
    }

    #[inline]
    fn leaf(&mut self, token: Symbol, _pos: u32, _value: Self::LeafValue) -> Self::NodeRef {
        self.graph.push(Evaluated { symbol: token })
    }

    #[inline]
    fn nulling(&self, token: Symbol) -> Self::NodeRef {
        NodeHandle::nulling(token)
    }
}
