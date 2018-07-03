mod action_closure;
mod array_evaluator;
pub mod cartesian_product;
mod evaluate;
pub mod order;
pub mod node;
mod sum_builder;
mod traversal;

use std::cell::Cell;

use typed_arena::Arena;
use cfg::symbol::Symbol;

use grammar::InternalGrammar;
use forest::Forest;
use util::slice_builder::SliceBuilder;

pub use self::action_closure::ActionClosureEvaluator;
pub use self::array_evaluator::{ArrayEvaluator, ValueArray};
pub use self::evaluate::{SumHandle, LeafHandle, NullHandle};
pub use self::node::{Node, Sum, Product, Leaf, LeafWithValue, Evaluated, ShallowProduct, Factors};
pub use self::order::{Order, NullOrder};
pub use self::traversal::{Traversal, TraversalBottom};
pub use self::sum_builder::SumBuilder;

pub type NodeRef<'a, 'f, T, V> = &'f Node<'a, 'f, T, V>;

pub struct Bocage<'a, 'f, 'g, T: 'f, V: 'a> where 'a: 'f, T: Copy {
    graph: Arena<Node<'a, 'f, T, V>>,
    grammar: &'g InternalGrammar,
    nulling_forests: Cell<&'f [Node<'a, 'f, T, V>]>,
}

impl<'a, 'f, 'g, T, V> Bocage<'a, 'f, 'g, T, V> where T: Copy {
    pub fn new(grammar: &'g InternalGrammar) -> Self {
        Bocage {
            graph: Arena::with_capacity(512),
            grammar,
            nulling_forests: Cell::new(&[]),
        }
    }

    pub fn initialize(&'f self) {
        // TODO trivial grammar check
        let max = (0 .. self.grammar.num_rules()).filter_map(|action| {
            self.grammar.nulling(action as u32).map(|(sym, _)| sym.usize())
        }).chain(
            self.grammar.eliminated_nulling_intermediate().iter().map(|rule| {
                rule.1.usize()
            })
        ).max().map_or(1, |max| max + 1);
        // Ensure that `max` is not too large.
        assert!(max < (1 << 20), "invalid nullable symbol");
        let external_syms = 0 .. max;
        let mut builder = SliceBuilder::new(&self.graph, max);
        for i in external_syms {
            builder.push(Leaf { symbol: Symbol::from(i) }.into());
        }
        let forests = builder.into_slice();
        for &(lhs, rhs0, rhs1) in self.grammar.eliminated_nulling_intermediate() {
            forests[lhs.usize()].set(Product {
                action: !0,
                factors: Factors {
                    left: &forests[rhs0.usize()],
                    right: Some(&forests[rhs1.usize()]),
                }
            });
        }
        self.nulling_forests.set(forests);
    }

    #[inline]
    fn product_tree_node(&'f self, node: &Node<'a, 'f, T, V>) {
        if let Product { action, mut factors } = node.get() {
            if factors.right.is_none() {
                // add omitted phantom syms here...
                if let Some((sym, dir)) = self.grammar.nulling(action) {
                    let nulling_forest = self.nulling(sym);
                    let (left, right) = if dir {
                        (factors.left, nulling_forest)
                    } else {
                        (nulling_forest, factors.left)
                    };
                    factors.left = left;
                    factors.right = Some(right);
                    node.set(Product {
                        action,
                        factors,
                    });
                }
            }
        }
    }
}

impl<'a, 'f, 'g, T, V> Forest<'f> for Bocage<'a, 'f, 'g, T, V> where T: Copy {
    type NodeRef = NodeRef<'a, 'f, T, V>;
    type NodeBuilder = SumBuilder<'a, 'f, T, V>;
    type LeafValue = T;

    fn build(&'f self, num_products: usize) -> Self::NodeBuilder {
        SumBuilder::new(&self.graph, num_products)
    }

    fn leaf(&'f self, token: Symbol, _pos: u32, value: Self::LeafValue) -> Self::NodeRef {
        &*self.graph.alloc(LeafWithValue {
            symbol: token,
            value,
        }.into())
    }

    fn nulling(&'f self, token: Symbol) -> Self::NodeRef {
        &self.nulling_forests.get()[token.usize()]
    }
}
