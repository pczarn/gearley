use std::cell::Cell;
use std::convert::Into;

use ref_slice::ref_slice;

use cfg::Symbol;

use grammar::InternalGrammar;

pub use self::NodeInner::*;

#[derive(Debug)]
pub struct Node<'a, 'f, T: 'f, V: 'a> where 'a: 'f, T: Copy {
    cell: Cell<NodeInner<'a, 'f, T, V>>,
}

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Debug)]
pub enum NodeInner<'a, 'f, T: 'f, V: 'a> where 'a: 'f, T: Copy {
    Sum {
        /// Invariant: This slice is not empty.
        /// Invariant: This slice directly contains only `Product` or `ShallowProduct`.
        /// (GADTs would be useful here)
        summands: &'f [Node<'a, 'f, T, V>],
    },
    Product {
        action: u32,
        factors: Factors<'a, 'f, T, V>,
    },
    Leaf {
        symbol: Symbol,
    },
    LeafWithValue {
        symbol: Symbol,
        value: T,
    },
    Evaluated {
        /// The type depends on `symbol`
        /// We need HKT to express this neatly
        values: &'a [V],
    },
    // We have ShallowProduct to avoid traversing product trees twice.
    ShallowProduct {
        action: u32,
        factor_stack_bottom: usize,
    },
}

#[derive(Debug)]
pub struct Factors<'a: 'f, 'f, T: 'f + Copy, V: 'a> {
    pub left: &'f Node<'a, 'f, T, V>,
    pub right: Option<&'f Node<'a, 'f, T, V>>,
}

impl<'a, 'f, T, V> Clone for Node<'a, 'f, T, V> where T: Copy {
    fn clone(&self) -> Self { Node { cell: Cell::new(self.cell.get()) } }
}

impl<'a, 'f, T, V> Clone for NodeInner<'a, 'f, T, V> where T: Copy {
    fn clone(&self) -> Self { *self }
}

impl<'a, 'f, T, V> Copy for NodeInner<'a, 'f, T, V> where T: Copy {}

impl<'a, 'f, T, V> Clone for Factors<'a, 'f, T, V> where T: Copy {
    fn clone(&self) -> Self { *self }
}

impl<'a, 'f, T, V> Copy for Factors<'a, 'f, T, V> where T: Copy {}

impl<'a, 'f, T, V> Node<'a, 'f, T, V> where T: Copy {
    #[inline]
    pub fn nonterminal(&self, grammar: &InternalGrammar) -> u32 {
        match self.alternatives()[0].get() {
            Product { action, .. } | ShallowProduct { action, .. } => {
                grammar.get_lhs(action).usize() as u32
            }
            _ => unreachable!()
        }
    }

    #[inline]
    pub fn factor_stack_bottom(&self) -> Option<usize> {
        match self.alternatives()[0].get() {
            Product { .. } | Leaf { .. } => {
                None
            }
            ShallowProduct { factor_stack_bottom, .. } => {
                Some(factor_stack_bottom)
            }
            _ => unreachable!()
        }
    }

    #[inline]
    pub fn alternatives(&self) -> &[Self] {
        match self.get() {
            Sum { summands, .. } => {
                summands
            }
            Product { .. } | ShallowProduct { .. } | Leaf { .. } => {
                ref_slice(self)
            }
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn get(&self) -> NodeInner<'a, 'f, T, V> {
        self.cell.get()
    }

    #[inline(always)]
    pub fn set(&self, inner: NodeInner<'a, 'f, T, V>) {
        self.cell.set(inner)
    }
}

impl<'a, 'f, T, V> Into<Node<'a, 'f, T, V>> for NodeInner<'a, 'f, T, V> where T: Copy {
    fn into(self) -> Node<'a, 'f, T, V> {
        Node {
            cell: Cell::new(self)
        }
    }
}
