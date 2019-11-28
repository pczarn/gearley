use std::slice;
use std::iter;
use std::borrow::Borrow;

use bit_vec;
use cfg::symbol::Symbol;
use ref_slice::ref_slice;

use grammar::InternalGrammar;
use super::Bocage;
use super::node::{NodeHandle, Node, Iter};
use super::node::Node::*;

pub use self::HandleVariant::*;

impl<G> Bocage<G> {
    // Once node liveness is marked, you may traverse the nodes.
    pub fn traverse(&self) -> Traverse<G> {
        Traverse {
            bocage: self,
            graph_iter: self.graph.iter_from(NodeHandle(0)),
            liveness_iter: self.gc.liveness.iter(),
            factor_stack: vec![],
            factor_traversal: vec![],
        }
    }
}

pub struct Traverse<'f, G> {
    bocage: &'f Bocage<G>,
    // main iterators
    graph_iter: Iter<'f>,
    liveness_iter: bit_vec::Iter<'f>,
    // Space for unrolling factors
    factor_stack: Vec<(Symbol, u32)>,
    // Scratch space for traversal
    factor_traversal: Vec<NodeHandle>,
}

impl<'f, G> Traverse<'f, G>
    where G: Borrow<InternalGrammar>,
{
    pub fn next_node<'t>(&'t mut self) -> Option<TraversalHandle<'f, 't, G>> {
        while let (Some(node), Some(alive)) = (self.graph_iter.peek(), self.liveness_iter.next()) {
            let iter = self.graph_iter;
            self.graph_iter.next();
            if !alive {
                continue;
            }
            println!("next_node {:?}", node);
            match node {
                Product { action, .. } => {
                    if self.bocage.is_transparent(action) {
                        continue;
                    }
                    let products = iter.take(1);
                    return Some(TraversalHandle {
                        iter,
                        symbol: self.bocage.grammar.borrow().get_lhs(action),
                        item: SumHandle(Products {
                            products,
                            traverse: self,
                        }),
                    });
                }
                Sum { nonterminal: symbol, count } => {
                    let products = self.graph_iter.take(count as usize);
                    for _ in 0..count {
                        let n = self.graph_iter.next();
                        println!("next_node product {:?}", n);
                        self.liveness_iter.next();
                    }
                    return Some(TraversalHandle {
                        iter,
                        symbol,
                        item: SumHandle(Products {
                            products,
                            traverse: self,
                        }),
                    });
                }
                NullingLeaf { symbol } => {
                    return Some(TraversalHandle {
                        iter,
                        symbol,
                        item: NullingHandle,
                    });
                }
                Evaluated { symbol, values } => {
                    return Some(TraversalHandle {
                        iter,
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
                Product { left_factor, right_factor, .. } => {
                    self.enqueue_for_unfold(left_factor, right_factor);
                }
                Evaluated { symbol, values } => {
                    self.factor_stack.push((symbol, values));
                }
                _ => unreachable!()
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
            self.bocage.graph.get(handle)
        })
    }
}

pub struct TraversalHandle<'f, 't, G> {
    pub(crate) iter: Iter<'f>,
    pub symbol: Symbol,
    pub item: HandleVariant<'f, 't, G>,
}

pub enum HandleVariant<'f, 't, G> {
    SumHandle(Products<'f, 't, G>),
    NullingHandle,
    LeafHandle(u32),
}

pub struct Products<'f, 't, G> {
    products: iter::Take<Iter<'f>>,
    traverse: &'t mut Traverse<'f, G>,
}

pub struct ProductHandle<'t> {
    pub action: u32,
    pub factors: &'t [(Symbol, u32)],
}

impl<'f, 't, G> Products<'f, 't, G>
    where G: Borrow<InternalGrammar>,
{
    pub fn next_product<'p>(&'p mut self) -> Option<ProductHandle> {
        while let Some(node) = self.products.next() {
            match node {
                Product { left_factor, right_factor, action } => {
                    let origin = self.traverse.bocage.grammar.borrow().external_origin(action);
                    if let Some(action) = origin {
                        self.traverse.unfold_factors(left_factor, right_factor);
                        return Some(ProductHandle {
                            action,
                            factors: &self.traverse.factor_stack[..],
                        });
                    }
                }
                _ => unreachable!()
            }
        }
        None
    }
}

impl<'f, 't, G> TraversalHandle<'f, 't, G> {
    pub fn set_evaluation_result(&self, values: u32) {
        self.iter.set(Evaluated {
            symbol: self.symbol,
            values,
        });
    }
}
