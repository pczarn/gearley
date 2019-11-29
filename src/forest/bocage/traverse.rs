use std::borrow::Borrow;
use std::slice;

use bit_vec;
use cfg::symbol::Symbol;
use ref_slice::ref_slice;

use forest::bocage::node::Node::*;
use forest::bocage::node::{CompactNode, Node};
use forest::node_handle::NodeHandle;
use forest::Bocage;
use grammar::InternalGrammar;

pub use self::HandleVariant::*;

impl<G> Bocage<G> {
    // Once node liveness is marked, you may traverse the nodes.
    pub fn traverse(&self) -> Traverse<G> {
        Traverse {
            bocage: self,
            graph_iter: self.graph.iter(),
            liveness_iter: self.gc.liveness.iter(),
            factor_stack: vec![],
            factor_traversal: vec![],
        }
    }
}

pub struct Traverse<'f, G> {
    bocage: &'f Bocage<G>,
    // main iterators
    graph_iter: slice::Iter<'f, CompactNode>,
    liveness_iter: bit_vec::Iter<'f>,
    // Space for unrolling factors
    factor_stack: Vec<(Symbol, u32)>,
    // Scratch space for traversal
    factor_traversal: Vec<NodeHandle>,
}

impl<'f, G> Traverse<'f, G>
where
    G: Borrow<InternalGrammar>,
{
    pub fn next_node<'t>(&'t mut self) -> Option<TraversalHandle<'f, 't, G>> {
        while let (Some(node), Some(alive)) = (self.graph_iter.next(), self.liveness_iter.next()) {
            if !alive {
                continue;
            }
            match node.expand() {
                Product { action, .. } => {
                    if self.bocage.is_transparent(action) {
                        continue;
                    }
                    return Some(TraversalHandle {
                        node,
                        symbol: self.bocage.grammar.borrow().get_lhs(action),
                        item: SumHandle(Products {
                            products: ref_slice(node).iter(),
                            traverse: self,
                        }),
                    });
                }
                Sum {
                    nonterminal: symbol,
                    count,
                } => {
                    let products = self.graph_iter.as_slice()[..count as usize].iter();
                    for _ in 0..count {
                        self.graph_iter.next();
                        self.liveness_iter.next();
                    }
                    return Some(TraversalHandle {
                        node,
                        symbol,
                        item: SumHandle(Products {
                            products,
                            traverse: self,
                        }),
                    });
                }
                NullingLeaf { symbol } => {
                    return Some(TraversalHandle {
                        node,
                        symbol,
                        item: NullingHandle,
                    });
                }
                Evaluated { symbol, values } => {
                    return Some(TraversalHandle {
                        node,
                        symbol,
                        item: LeafHandle(values),
                    });
                }
            }
        }
        None
    }

    fn unfold_factors(&mut self, left: NodeHandle, right: Option<NodeHandle>) {
        self.factor_stack.clear();
        self.enqueue_for_unfold(left, right);
        while let Some(node) = self.pop_for_unfold() {
            match node {
                Product {
                    left_factor,
                    right_factor,
                    ..
                } => {
                    self.enqueue_for_unfold(left_factor, right_factor);
                }
                Evaluated { symbol, values } => {
                    self.factor_stack.push((symbol, values));
                }
                _ => unreachable!(),
            }
        }
    }

    fn enqueue_for_unfold(&mut self, left: NodeHandle, right: Option<NodeHandle>) {
        if let Some(right) = right {
            self.factor_traversal.push(right);
        }
        self.factor_traversal.push(left);
    }

    fn pop_for_unfold(&mut self) -> Option<Node> {
        self.factor_traversal.pop().map(|handle| {
            let node = self.bocage.graph[handle.usize()].clone();
            node.expand()
        })
    }
}

pub struct TraversalHandle<'f, 't, G> {
    pub node: &'f CompactNode,
    pub symbol: Symbol,
    pub item: HandleVariant<'f, 't, G>,
}

pub enum HandleVariant<'f, 't, G> {
    SumHandle(Products<'f, 't, G>),
    NullingHandle,
    LeafHandle(u32),
}

pub struct Products<'f, 't, G> {
    products: slice::Iter<'f, CompactNode>,
    traverse: &'t mut Traverse<'f, G>,
}

pub struct ProductHandle<'t> {
    pub action: u32,
    pub factors: &'t [(Symbol, u32)],
}

impl<'f, 't, G> Products<'f, 't, G>
where
    G: Borrow<InternalGrammar>,
{
    pub fn next_product<'p>(&'p mut self) -> Option<ProductHandle> {
        while let Some(node) = self.products.next() {
            match node.expand() {
                Product {
                    left_factor,
                    right_factor,
                    action,
                } => {
                    let origin = self
                        .traverse
                        .bocage
                        .grammar
                        .borrow()
                        .external_origin(action);
                    if let Some(action) = origin {
                        self.traverse.unfold_factors(left_factor, right_factor);
                        return Some(ProductHandle {
                            action,
                            factors: &self.traverse.factor_stack[..],
                        });
                    }
                }
                _ => unreachable!(),
            }
        }
        None
    }
}

impl<'f, 't, G> TraversalHandle<'f, 't, G> {
    pub fn set_evaluation_result(&self, values: u32) {
        self.node.set(Evaluated {
            symbol: self.symbol,
            values,
        });
    }
}
