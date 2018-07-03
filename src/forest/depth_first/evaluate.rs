use std::slice;

use cfg::Symbol;

use forest::depth_first::{Node, NodeRef, ShallowProduct, Evaluated};
use grammar::InternalGrammar;

// In the future, these methods may need to return paramter type N (Node).
pub trait Evaluate<'a, T, V> where T: Copy {
    fn evaluate<'t, 'f, 'g>(&mut self, node: SumHandle<'a, 't, 'f, 'g, T, V>) -> &'a [V];
    fn evaluate_terminal(&mut self, node: LeafHandle<T, V>) -> &'a [V];
    fn evaluate_nulling(&mut self, symbol: Symbol) -> &'a [V];
}

pub struct SumHandle<'a: 'f, 't, 'f: 't, 'g, T: 'f + Copy, V: 'a> {
    pub node: NodeRef<'a, 'f, T, V>,
    pub summands: &'t [Node<'a, 'f, T, V>],
    pub factor_stack: &'t [NodeRef<'a, 'f, T, V>],
    pub grammar: &'g InternalGrammar,
}

pub struct SumIter<'a: 'f, 't, 'f: 't, 'g, T: 'f + Copy, V: 'a> {
    summands: slice::Iter<'t, Node<'a, 'f, T, V>>,
    factor_stack: &'t [NodeRef<'a, 'f, T, V>],
    grammar: &'g InternalGrammar,
}

pub struct ProductHandle<'a: 'f, 't, 'f: 't, T: 'f + Copy, V: 'a> {
    pub action: u32,
    pub factors: &'t [NodeRef<'a, 'f, T, V>],
    // TO DO, if needed? for accessing the symbol.
    // grammar: &'g InternalGrammar,
}

pub struct LeafHandle<'a: 'f, 'f, T: 'a + Copy, V: 'a> {
    pub factor: NodeRef<'a, 'f, T, V>,
    pub terminal: Symbol,
    pub value: T,
}

pub struct NullHandle<'a: 'f, 'f, T: 'f + Copy, V: 'a> {
    pub factor: NodeRef<'a, 'f, T, V>,
    pub symbol: Symbol,
}

impl<'a, 't, 'f, 'g, T, V> SumHandle<'a, 't, 'f, 'g, T, V> where T: Copy {
    pub fn result(&self, values: &'a [V]) {
        self.node.set(Evaluated {
            values
        });
    }
}

impl<'a, 'f, T, V> LeafHandle<'a, 'f, T, V> where T: Copy {
    pub fn result(&self, values: &'a [V]) {
        self.factor.set(Evaluated {
            values
        });
    }
}

impl<'a, 'f, T, V> NullHandle<'a, 'f, T, V> where T: Copy {
    pub fn result(&self, values: &'a [V]) {
        self.factor.set(Evaluated {
            values
        });
    }
}

impl<'a, 't, 'f, 'g, T, V> SumHandle<'a, 't, 'f, 'g, T, V> where T: Copy {
    /// Iterate through productions that are summed by this node.
    pub fn iter(&self) -> SumIter<'a, 't, 'f, 'g, T, V> {
        SumIter {
            summands: self.summands.iter(),
            factor_stack: self.factor_stack,
            grammar: self.grammar,
        }
    }

    /// Get the nonterminal symbol of this node.
    pub fn nonterminal(&self) -> Symbol {
        let symbol = self.node.nonterminal(self.grammar);
        self.grammar.to_external(Symbol::from(symbol))
    }
}

impl<'a, 't, 'f, 'g, T, V> Iterator for SumIter<'a, 't, 'f, 'g, T, V> where T: Copy {
    type Item = ProductHandle<'a, 't, 'f, T, V>;
    fn next(&mut self) -> Option<Self::Item> {
        self.summands.next_back().map(|summand| {
            match summand.get() {
                ShallowProduct { action, factor_stack_bottom } => {
                    let action = self.grammar.get_eval(action).unwrap_or_else(|| {
                        panic!("Null eval for action {}", action)
                    });
                    let (rest, factors) = self.factor_stack.split_at(factor_stack_bottom);
                    self.factor_stack = rest;
                    ProductHandle {
                        action,
                        factors,
                    }
                }
                _ => unreachable!()
            }
        })
    }
}

impl<'a, 't, 'f, T, V> ProductHandle<'a, 't, 'f, T, V> where T: Copy {
    /// Get the action ID of this production.
    pub fn action(&self) -> u32 {
        self.action
    }

    /// Get the number of parse trees that were partially evaluated as a part
    /// of this production.
    pub fn len(&self) -> usize {
        self.factors.iter().map(|alt| {
            match alt.get() {
                Evaluated { values } => values.len(),
                _ => unreachable!()
            }
        }).product()
    }
}
