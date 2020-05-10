use std::borrow::Borrow;
use std::convert::TryInto;
use std::iter;

use bit_vec::BitVec;
use cfg::symbol::Symbol;

use forest::compact_bocage::node::Node::*;
use forest::compact_bocage::node::{Iter, Node, Tag};
use forest::node_handle::NodeHandle;
use forest::CompactBocage;
use grammar::InternalGrammar;
use policy::PerformancePolicy;

pub use self::HandleVariant::*;

impl<G, P> CompactBocage<G, P> {
    // Once node liveness is marked, you may traverse the nodes.
    pub fn traverse(&self) -> Traverse<G, P> {
        Traverse {
            bocage: self,
            graph_iter: self.graph.iter_from(NodeHandle(0)),
            liveness: &self.gc.liveness,
            factor_stack: vec![],
            factor_traversal: vec![],
        }
    }
}

pub struct Traverse<'f, G, P> {
    bocage: &'f CompactBocage<G, P>,
    // main iterators
    graph_iter: Iter<'f>,
    liveness: &'f BitVec,
    // Space for unrolling factors
    factor_stack: Vec<(Symbol, NodeHandle)>,
    // Scratch space for traversal
    factor_traversal: Vec<NodeHandle>,
}

impl<'f, G, P: PerformancePolicy> Traverse<'f, G, P>
where
    G: Borrow<InternalGrammar<P>>,
{
    pub fn next_node<'t>(&'t mut self) -> Option<TraversalHandle<'f, 't, G, P>> {
        while let Some(node) = self.graph_iter.peek() {
            let iter = self.graph_iter;
            let alive = self.liveness[self.graph_iter.handle.usize()];
            // println!(
            //     "next_node @{:?} {:?} {}",
            //     self.graph_iter.handle, node, alive
            // );
            self.graph_iter.next();
            if !alive {
                continue;
            }
            match node {
                Product { action, .. } => {
                    if self.bocage.is_transparent(action) {
                        continue;
                    }
                    let mut consecutive_iter = self.graph_iter;
                    let consecutive_count = consecutive_iter.take_while(|node| {
                        match node {
                            Product { first, .. } => !first,
                            _ => false,
                        }
                    }).count();
                    let products = iter.take(1 + consecutive_count);
                    return Some(TraversalHandle {
                        iter,
                        symbol: self.bocage.grammar.borrow().get_lhs(action.try_into().ok().unwrap()).into(),
                        item: SumHandle(Products {
                            products,
                            traverse: self,
                        }),
                    });
                }
                // Sum {
                //     nonterminal: symbol,
                //     count,
                // } => {
                //     let products = self.graph_iter.take(count as usize);
                //     for _ in 0..count {
                //         let p = self.graph_iter.handle;
                //         let n = self.graph_iter.next();
                //         println!("next_node product @{:?} {:?}", p, n);
                //     }
                //     return Some(TraversalHandle {
                //         iter,
                //         symbol,
                //         item: SumHandle(Products {
                //             products,
                //             traverse: self,
                //         }),
                //     });
                // }
                NullingLeaf { symbol } => {
                    return Some(TraversalHandle {
                        iter,
                        symbol,
                        item: NullingHandle,
                    });
                }
                Evaluated { symbol, .. } => {
                    return Some(TraversalHandle {
                        iter,
                        symbol,
                        item: LeafHandle,
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
                (
                    Product {
                        left_factor,
                        right_factor,
                        ..
                    },
                    _,
                ) => {
                    self.enqueue_for_unfold(left_factor, right_factor);
                }
                (Evaluated { symbol }, handle) | (NullingLeaf { symbol }, handle) => {
                    self.factor_stack.push((symbol, handle));
                }
                other => {
                    println!("OTHER {:?}", other);
                    unreachable!();
                }
            }
        }
    }

    fn enqueue_for_unfold(&mut self, left: NodeHandle, right: Option<NodeHandle>) {
        if let Some(right) = right {
            self.factor_traversal.push(right);
        }
        self.factor_traversal.push(left);
    }

    fn pop_for_unfold(&mut self) -> Option<(Node, NodeHandle)> {
        self.factor_traversal
            .pop()
            .map(|handle| (self.bocage.graph.get(handle), handle))
    }
}

pub struct TraversalHandle<'f, 't, G, P> {
    pub(crate) iter: Iter<'f>,
    pub symbol: Symbol,
    pub item: HandleVariant<'f, 't, G, P>,
}

pub enum HandleVariant<'f, 't, G, P> {
    SumHandle(Products<'f, 't, G, P>),
    NullingHandle,
    LeafHandle,
}

pub struct Products<'f, 't, G, P> {
    products: iter::Take<Iter<'f>>,
    traverse: &'t mut Traverse<'f, G, P>,
}

pub struct ProductHandle<'t> {
    pub action: u32,
    pub factors: &'t [(Symbol, NodeHandle)],
}

impl<'f, 't, G, P: PerformancePolicy> Products<'f, 't, G, P>
where
    G: Borrow<InternalGrammar<P>>,
{
    pub fn next_product<'p>(&'p mut self) -> Option<ProductHandle> {
        while let Some(node) = self.products.next() {
            match node {
                Product {
                    left_factor,
                    right_factor,
                    action,
                    ..
                } => {
                    let origin = self
                        .traverse
                        .bocage
                        .grammar
                        .borrow()
                        .external_origin(action.try_into().ok().unwrap());
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

impl<'f, 't, G, P> TraversalHandle<'f, 't, G, P> {
    pub fn end_evaluation(&self) {
        println!("end_eval {:?}", self.iter.handle);
        self.iter.vec[self.iter.handle.usize()].set(Tag::LeafTag.to_u16());
    }

    pub fn handle(&self) -> NodeHandle {
        self.iter.handle
    }
}
