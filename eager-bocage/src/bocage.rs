use std::mem;
use std::ops::{Index, IndexMut};

use cfg_symbol::{Symbol, SymbolSource};
use gearley_forest::{node_handle::NodeHandle, Forest};
use gearley_forest::item::Item;
use gearley_grammar::{ForestInfo, Grammar};

use crate::node::{NULL_ACTION};

#[derive(Debug)]
pub struct Bocage<E: Evaluate> {
    pub(crate) values: Vec<Node<E::Value>>,
    pub(crate) forest_info: ForestInfo,
    pub(crate) summand_count: u32,
    stack: Vec<(NodeHandle, bool)>,
    args: Vec<E::Value>,
    evaluate: E,
}

#[derive(Clone, Copy, Debug)]
pub enum Node<V> {
    Nulling,
    Rule {
        left_factor: NodeHandle,
        right_factor: NodeHandle,
    },
    Evaluated(V),
}

pub trait Evaluate {
    type Value: Clone;
    fn leaf(&mut self, token: Symbol, value: u32) -> Self::Value;
    fn product(&mut self, action: u32, args: impl Iterator<Item = Self::Value>) -> Self::Value;
    fn nulling(&mut self, nulling: Symbol) -> Self::Value;
}

impl<E: Evaluate> Bocage<E> {
    pub fn new<G: Grammar>(grammar: G, eval: E) -> Self {
        Self::with_capacity(grammar, eval, 1024)
    }

    pub fn with_capacity<G: Grammar>(grammar: G, eval: E, graph_cap: usize) -> Self {
        let mut result = Bocage {
            values: Vec::with_capacity(graph_cap),
            forest_info: grammar.forest_info(),
            summand_count: 0,
            stack: vec![],
            args: vec![],
            evaluate: eval,
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
        self.values = vec![Node::Nulling; nulling_leaf_count];
        for &[lhs, rhs0, rhs1] in &self.forest_info.nulling_intermediate_rules {
            self.values[NodeHandle::nulling(lhs).usize()] = Node::Rule {
                left_factor: NodeHandle::nulling(rhs0),
                right_factor: NodeHandle::nulling(rhs1),
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
}

impl<E: Evaluate> Forest for Bocage<E> {
    type NodeRef = NodeHandle;

    const FOREST_BYTES_PER_RECOGNIZER_BYTE: usize = 2;

    #[inline]
    fn begin_sum(&mut self) {
        // nothing to do
    }

    #[inline]
    fn push_summand(&mut self, item: Item<Self::NodeRef>) {
        self.stack.clear();
        self.stack.push((item.node, true));
        while let Some((handle, is_root)) = self.stack.pop() {
            match mem::replace(&mut self.values[handle.usize()], Node::Nulling) {
                Node::Rule { left_factor, right_factor } => {
                    self.stack.push((right_factor, false));
                    self.stack.push((left_factor, false));
                }
                Node::Evaluated(val) => {
                    if is_root {
                        if let Some((sym, dir)) = self.forest_info.nulling_eliminated[item.dot as usize] {
                            let nulling = self.nulling(sym);
                            if dir {
                                self.args.push(val);
                                self.stack.push((nulling, false));
                            } else {
                                self.stack.push((nulling, false));
                                self.stack.push((handle, false));
                            };
                        }
                    } else {
                        self.args.push(val);
                    }
                }
                Node::Nulling => {
                    unreachable!()
                }
            }
        }
        self.values.push(Node::Evaluated(self.evaluate.product(item.dot, self.args.drain(..))));
        self.summand_count += 1;
    }

    #[inline]
    fn product(
        &mut self,
        left_factor: Self::NodeRef,
        right_factor: Self::NodeRef,
    ) -> Self::NodeRef {
        self.values.push(Node::Rule {
            left_factor,
            right_factor,
        });
        NodeHandle(self.values.len() as u32 - 1)
    }

    #[inline]
    fn sum(&mut self, lhs_sym: Symbol, _origin: u32) -> Self::NodeRef {
        let result = {
            match self.summand_count {
                0 => unreachable!(),
                1 => NodeHandle(self.values.len() as u32 - 1),
                _ => unreachable!()
            }
        };
        self.summand_count = 0;
        result
    }

    #[inline]
    fn leaf(&mut self, token: Symbol, _pos: u32, value: u32) -> Self::NodeRef {
        let result = NodeHandle(self.values.len() as u32);
        self.values.push(Node::Evaluated(self.evaluate.leaf(token, value)));
        result
    }

    #[inline]
    fn nulling(&mut self, token: Symbol) -> Self::NodeRef {
        NodeHandle(token.into())
    }

    fn leo_product(&mut self, left_factor: Self::NodeRef, right_factor: Self::NodeRef) -> Self::NodeRef {
        todo!()
    }
}

impl<E: Evaluate> Index<NodeHandle> for Bocage<E> {
    type Output = Node<E::Value>;

    fn index(&self, index: NodeHandle) -> &Self::Output {
        &self.values[index.usize()]
    }
}

impl<E: Evaluate> IndexMut<NodeHandle> for Bocage<E> {
    fn index_mut(&mut self, index: NodeHandle) -> &mut Self::Output {
        &mut self.values[index.usize()]
    }
}
