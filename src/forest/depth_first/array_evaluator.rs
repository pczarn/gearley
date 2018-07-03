use typed_arena::Arena;
use ref_slice::ref_slice;
use cfg::Symbol;

use util::slice_builder::SliceBuilder;
use super::action_closure::ActionEvaluator;
use super::evaluate::{Evaluate, SumHandle, LeafHandle};
use super::{Traversal, NodeRef, TraversalBottom, Order};

pub struct ValueArray<V> {
    arena: Arena<V>,
}

impl<V> Default for ValueArray<V> {
    fn default() -> Self {
        ValueArray {
            arena: Arena::new()
        }
    }
}

pub struct ArrayEvaluator<'a, V: 'a, E> {
    value_array: &'a ValueArray<V>,
    eval: E,
}

impl<V> ValueArray<V> {
    pub fn new() -> Self {
        ValueArray {
            arena: Arena::with_capacity(2048)
        }
    }

    pub fn build_slice(&self, len: usize) -> SliceBuilder<V> {
        SliceBuilder::new(&self.arena, len)
    }
}

impl<'a, V, E> ArrayEvaluator<'a, V, E> {
    pub fn new(value_array: &'a ValueArray<V>, eval: E) -> Self {
        ArrayEvaluator {
            value_array,
            eval,
        }
    }

    pub fn traverse<'f, 'g, T, O>(
        &mut self,
        traversal: &mut Traversal<'a, 'f, 'g, T, V, O>,
        root: NodeRef<'a, 'f, T, V>)
        -> &'a [V]
        where T: 'a + Copy,
              O: Order<'a, 'f, T, V>,
              E: ActionEvaluator<'a, T, V>,
    {
        traversal.traverse(root);
        loop {
            if let Some(deps) = traversal.traverse_deps() {
                for node in deps {
                    match node {
                        TraversalBottom::Leaf(terminal) => {
                            let values = self.evaluate_terminal(LeafHandle {
                                factor: terminal.factor,
                                terminal: terminal.terminal,
                                value: terminal.value,
                            });
                            terminal.result(values);
                        }
                        TraversalBottom::Null(nulling) => {
                            let values = self.evaluate_nulling(nulling.symbol);
                            nulling.result(values);
                        }
                    }
                }
            } else {
                break;
            }
            for sum in traversal.traverse_sum() {
                let values = self.evaluate(SumHandle {
                    node: sum.node,
                    summands: sum.summands,
                    factor_stack: sum.factor_stack,
                    grammar: sum.grammar,
                });
                sum.result(values);
            }
        }
        traversal.finish(root)
    }
}

impl<'a, T, V, E> Evaluate<'a, T, V> for ArrayEvaluator<'a, V, E>
    where E: ActionEvaluator<'a, T, V>,
          T: Copy
{
    fn evaluate<'t, 'f, 'g>(&mut self, sum: SumHandle<'a, 't, 'f, 'g, T, V>) -> &'a [V] {
        let count = sum.iter().map(|alt| alt.len()).sum();
        let mut slice_builder = self.value_array.build_slice(count);
        // Evaluate.
        for summand in sum.iter() {
            self.eval.production(&summand);
            while let Some(value) = self.eval.next() {
                // placement new?
                slice_builder.push(value);
            }
        }
        slice_builder.advance_slice()
    }

    fn evaluate_terminal(&mut self, leaf: LeafHandle<T, V>) -> &'a [V] {
        let result = self.value_array.arena.alloc(self.eval.leaf(leaf.terminal, Some(&leaf.value)));
        ref_slice(result)
    }

    fn evaluate_nulling(&mut self, symbol: Symbol) -> &'a [V] {
        let mut builder = self.value_array.build_slice(0);
        self.eval.nulling(symbol, &mut builder);
        builder.into_slice()
    }
}
