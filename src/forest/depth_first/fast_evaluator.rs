use typed_arena::Arena;
use ref_slice::ref_slice;
use cfg::Symbol;

use util::slice_builder::SliceBuilder;
use super::action::Invoke;
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

pub struct FastEvaluator<'a, I, V: 'a> {
    invoker: I,
    value_array: &'a ValueArray<V>,
}

impl<V> ValueArray<V> {
    pub fn new() -> Self {
        ValueArray {
            arena: Arena::with_capacity(2048)
        }
    }

    fn build_slice(&self, len: usize) -> SliceBuilder<V> {
        SliceBuilder::new(&self.arena, len)
    }
}

impl<'a, I, V> FastEvaluator<'a, I, V> {
    pub fn new(value_array: &'a ValueArray<V>, invoker: I) -> Self {
        FastEvaluator {
            value_array,
            invoker,
        }
    }

    pub fn traverse<'f, 'g, T, O>(
        &mut self,
        traversal: &mut Traversal<'a, 'f, 'g, T, V, O>,
        root: NodeRef<'a, 'f, T, V>)
        -> &'a [V]
        where T: 'a + Copy,
              O: Order<'a, 'f, T, V>,
              I: Invoke<'a, T, V>,
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
                            terminal.set_evaluation_result(values);
                        }
                        TraversalBottom::Null(nulling) => {
                            let values = self.evaluate_nulling(nulling.symbol);
                            nulling.set_evaluation_result(values);
                        }
                    }
                }
            } else {
                break;
            }
            for sum in traversal.traverse_sum() {
                let values = self.evaluate_sum(SumHandle {
                    node: sum.node,
                    summands: sum.summands,
                    factor_stack: sum.factor_stack,
                    grammar: sum.grammar,
                });
                sum.set_evaluation_result(values);
            }
        }
        root.values().unwrap()
    }
}

impl<'a, I, T, V> Evaluate<'a, T, V> for FastEvaluator<'a, I, V>
    where I: Invoke<'a, T, V>,
          T: Copy
{
    fn evaluate_sum<'t, 'f, 'g>(&mut self, sum: SumHandle<'a, 't, 'f, 'g, T, V>) -> &'a [V] {
        let count = sum.iter().map(|alt| alt.len()).sum();
        let mut slice_builder = self.value_array.build_slice(count);
        // Evaluate.
        for summand in sum.iter() {
            self.invoker.set_production(&summand);
            while let Some(value) = self.invoker.invoke_next_factor() {
                // placement new?
                slice_builder.push(value);
            }
        }
        slice_builder.advance_slice()
    }

    fn evaluate_terminal(&mut self, leaf: LeafHandle<T, V>) -> &'a [V] {
        let value = self.invoker.invoke_leaf(leaf.terminal, Some(&leaf.value));
        let result = self.value_array.arena.alloc(value);
        ref_slice(result)
    }

    fn evaluate_nulling(&mut self, symbol: Symbol) -> &'a [V] {
        let mut builder = self.value_array.build_slice(0);
        self.invoker.invoke_nulling(symbol, &mut builder);
        builder.into_slice()
    }
}
