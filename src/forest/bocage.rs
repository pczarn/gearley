use std::borrow::Borrow;

use bit_vec::BitVec;
use cfg::symbol::Symbol;
use ref_slice::ref_slice;

use item::CompletedItem;
use grammar::InternalGrammar;
use forest::Forest;

use super::node::{Node, NodeHandle, CompactNode, NULL_ACTION};
use super::order::Order;
use super::node::Node::*;

pub struct Bocage<G> {
    pub(super) graph: Vec<CompactNode>,
    pub(super) gc: MarkAndSweep,
    pub(super) grammar: G,
    pub(super) nulling_leaf_count: usize,
    pub(super) summand_count: u32,
}

pub(super) struct MarkAndSweep {
    pub(super) liveness: BitVec,
    // List for DFS and/or maybe relocation of stuff in the future.
    pub(super) dfs: Vec<NodeHandle>,
}

impl<G> Bocage<G> where G: Borrow<InternalGrammar> {
    pub fn new(grammar: G) -> Self {
        let mut result = Bocage {
            graph: Vec::with_capacity(512),
            gc: MarkAndSweep {
                liveness: BitVec::with_capacity(512),
                dfs: vec![],
            },
            grammar,
            nulling_leaf_count: 0,
            summand_count: 0,
        };
        result.initialize_nulling();
        result
    }

    fn initialize_nulling(&mut self) {
        // TODO trivial grammar check
        self.nulling_leaf_count = self.nulling_symbol_count();
        // Ensure that `max` is not ridiculously large.
        assert!(self.nulling_leaf_count < (1 << 20), "invalid nullable symbol");
        self.graph.extend(
            (0 ..= self.nulling_leaf_count).map(|i|
                NullingLeaf { symbol: Symbol::from(i) }.compact()
            )
        );
        for &(lhs, rhs0, rhs1) in self.grammar.borrow().eliminated_nulling_intermediate() {
            self.set(
                NodeHandle::nulling(lhs),
                Product {
                    left_factor: NodeHandle::nulling(rhs0),
                    right_factor: Some(NodeHandle::nulling(rhs1)),
                    action: NULL_ACTION,
                }
            );
        }
    }

    fn nulling_symbol_count(&self) -> usize {
        self.grammar.borrow().max_nulling_symbol().unwrap_or(0)
    }

    pub fn mark_alive<O: Order>(&mut self, root: NodeHandle, mut order: O) {
        #[inline]
        fn summands(graph: &Vec<CompactNode>, node: NodeHandle) -> &[CompactNode] {
            match graph[node.usize()].get() {
                Sum { count, .. } => {
                    // back
                    // let start = node.usize() - count as usize - 1;
                    // let end = node.usize() - 1;
                    let start = node.usize() + 1;
                    let end = node.usize() + count as usize + 1;
                    &graph[start .. end]
                }
                _ => ref_slice(&graph[node.usize()]),
            }
        }

        self.gc.liveness.grow(self.graph.len(), false);
        self.gc.dfs.push(root);
        while let Some(node) = self.gc.dfs.pop() {
            if let Some(false) = self.gc.liveness.get(node.usize()) {
                self.gc.liveness.set(node.usize(), true);
                let summands = summands(&self.graph, node);
                let summands = order.sum(summands);
                for summand in summands {
                    self.postprocess_product_tree_node(summand);
                    // TODO: use order for products.
                    self.gc.dfs_queue_factors(summand);
                }
            }
        }
    }

    #[inline]
    fn postprocess_product_tree_node(&self, node: &CompactNode) {
        if let Product { left_factor: factor, right_factor: None, action } = node.get() {
            // Add omitted phantom syms here.
            if let Some((sym, dir)) = self.grammar.borrow().nulling(action) {
                let (left, right) = if dir {
                    (factor, NodeHandle::nulling(sym))
                } else {
                    (NodeHandle::nulling(sym), factor)
                };
                node.set(
                    Product {
                        left_factor: left,
                        right_factor: Some(right),
                        action,
                    }
                );
            }
        }
    }

    #[inline]
    fn set(&self, idx: NodeHandle, node: Node) {
        self.graph[idx.usize()].set(node);
    }

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
        match summand.get() {
            Product { left_factor, right_factor, .. } => {
                self.dfs.extend(right_factor);
                self.dfs.push(left_factor);
            }
            NullingLeaf { .. } | Evaluated { .. } => {}
            Sum { .. } => unreachable!()
        }
    }
}

impl<G> Forest for Bocage<G> {
    type NodeRef = NodeHandle;
    type LeafValue = u32;

    fn push_summand(&mut self, item: CompletedItem<Self::NodeRef>) {
        if self.summand_count == 1 {
            let first_node = self.graph.pop().unwrap();
            self.graph.push(Sum {
                nonterminal: Symbol::from(0u32),
                count: 0,
            }.compact());
            self.graph.push(first_node);
        }
        self.graph.push(Product {
            action: item.dot,
            left_factor: item.left_node,
            right_factor: item.right_node,
        }.compact());
        self.summand_count += 1;
    }

    fn sum(&mut self, lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        let distance = if self.summand_count == 1 {
            0
        } else {
            self.summand_count as usize
        };
        let node_idx = self.graph.len() - distance - 1;
        if self.summand_count >= 2 {
            // Slower case: ambiguous node.
            self.graph[node_idx].set(
                Sum {
                    nonterminal: lhs_sym,
                    count: self.summand_count as u32,
                }
            );
        }
        self.summand_count = 0;
        NodeHandle(node_idx as u32)
    }

    fn leaf(&mut self, token: Symbol, _pos: u32, value: Self::LeafValue) -> Self::NodeRef {
        let result = NodeHandle(self.graph.len() as u32);
        self.graph.push(Evaluated { symbol: token, values: value }.compact());
        result
    }

    fn nulling(&self, token: Symbol) -> Self::NodeRef {
        NodeHandle::nulling(token)
    }
}
