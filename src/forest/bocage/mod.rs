pub mod node;
pub mod order;
pub mod traverse;

use std::borrow::Borrow;
use std::hint;

use bit_vec::BitVec;
use cfg::symbol::Symbol;
use ref_slice::ref_slice;

use forest::node_handle::NodeHandle;
use forest::Forest;
use grammar::InternalGrammar;
use item::CompletedItem;

use self::node::Node::*;
use self::node::{CompactNode, Node, NULL_ACTION};
use self::order::Order;

pub struct Bocage<G> {
    pub(crate) graph: Vec<CompactNode>,
    pub(crate) gc: MarkAndSweep,
    pub(crate) grammar: G,
    pub(crate) summand_count: u32,
}

pub(crate) struct MarkAndSweep {
    pub(crate) liveness: BitVec,
    // List for DFS and/or maybe relocation of stuff in the future.
    pub(crate) dfs: Vec<NodeHandle>,
}

impl<G> Bocage<G>
where
    G: Borrow<InternalGrammar>,
{
    pub fn new(grammar: G) -> Self {
        Self::with_capacities(grammar, 1024, 32)
    }

    pub fn with_capacities(grammar: G, graph_cap: usize, dfs_cap: usize) -> Self {
        let mut result = Bocage {
            graph: Vec::with_capacity(graph_cap),
            gc: MarkAndSweep {
                liveness: BitVec::with_capacity(graph_cap),
                dfs: Vec::with_capacity(dfs_cap),
            },
            grammar,
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
            NullingLeaf {
                symbol: Symbol::from(i),
            }
            .compact()
        }));
        for &(lhs, rhs0, rhs1) in self.grammar.borrow().eliminated_nulling_intermediate() {
            self.set(
                NodeHandle::nulling(lhs),
                Product {
                    left_factor: NodeHandle::nulling(rhs0),
                    right_factor: Some(NodeHandle::nulling(rhs1)),
                    action: NULL_ACTION,
                },
            );
        }
    }

    fn nulling_symbol_count(&self) -> usize {
        self.grammar.borrow().max_nulling_symbol().unwrap_or(0)
    }

    #[inline]
    pub fn mark_alive<O: Order>(&mut self, root: NodeHandle, mut order: O) {
        self.gc.liveness.clear();
        self.gc.liveness.grow(self.graph.len(), false);
        self.gc.dfs.push(root);
        while let Some(node) = self.gc.dfs.pop() {
            self.gc.liveness.set(node.usize(), true);
            let summands = Bocage::<G>::summands(&self.graph, node);
            let summands = order.sum(summands);
            for summand in summands {
                self.postprocess_product_tree_node(summand);
                // TODO: use order for products.
                self.gc.dfs_queue_factors(summand);
            }
        }
    }

    #[inline]
    fn summands(graph: &Vec<CompactNode>, node: NodeHandle) -> &[CompactNode] {
        unsafe {
            match graph.get_unchecked(node.usize()).expand() {
                Sum { count, .. } => {
                    // back
                    // let start = node.usize() - count as usize - 1;
                    // let end = node.usize() - 1;
                    let start = node.usize() + 1;
                    let end = node.usize() + count as usize + 1;
                    graph.get_unchecked(start..end)
                }
                _ => ref_slice(graph.get_unchecked(node.usize())),
            }
        }
    }

    #[inline]
    fn postprocess_product_tree_node(&self, node: &CompactNode) {
        if let Product {
            left_factor: factor,
            right_factor: None,
            action,
        } = node.expand()
        {
            // Add omitted phantom syms here.
            if let Some((sym, dir)) = self.grammar.borrow().nulling(action) {
                let (left, right) = if dir {
                    (factor, NodeHandle::nulling(sym))
                } else {
                    (NodeHandle::nulling(sym), factor)
                };
                node.set(Product {
                    left_factor: left,
                    right_factor: Some(right),
                    action,
                });
            }
        }
    }

    #[inline]
    fn set(&self, idx: NodeHandle, node: Node) {
        self.graph[idx.usize()].set(node);
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
    fn dfs_queue_factors(&mut self, summand: &CompactNode) {
        match summand.expand() {
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

impl<G> Forest for Bocage<G> {
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
            Product {
                action: item.dot,
                left_factor: item.left_node,
                right_factor: item.right_node,
            }
            .compact(),
        );
        self.summand_count += 1;
    }

    #[inline]
    fn sum(&mut self, lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        let result = unsafe {
            match self.summand_count {
                0 => hint::unreachable_unchecked(),
                1 => NodeHandle(self.graph.len() as u32 - 1),
                summand_count => {
                    // Slower case: ambiguous node.
                    let first_summand_idx = self.graph.len() - summand_count as usize;
                    let first_summand = self.graph.get_unchecked(first_summand_idx).clone();
                    self.graph.push(first_summand);
                    *self.graph.get_unchecked_mut(first_summand_idx) = Sum {
                        nonterminal: lhs_sym,
                        count: self.summand_count as u32,
                    }
                    .compact();
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
            Evaluated {
                symbol: token,
                values: value,
            }
            .compact(),
        );
        result
    }

    #[inline]
    fn nulling(&self, token: Symbol) -> Self::NodeRef {
        NodeHandle::nulling(token)
    }
}
