use std::iter;
use std::slice;

use forest::depth_first::*;
use grammar::InternalGrammar;

pub struct Traversal<'a, 'f, 'g, T, V, O>
    where 'a: 'f,
          'g: 'f,
          T: 'f + Copy,
          V: 'a
{
    bocage: &'f Bocage<'a, 'f, 'g, T, V>,
    order: O,
    // topologically ordered
    dependencies_stack: Vec<NodeRef<'a, 'f, T, V>>,
    // for traversal
    factor_stack: Vec<NodeRef<'a, 'f, T, V>>,
    factor_stack_bottom: usize,
    dependencies_stack_bottom: usize,
    // Scratch space for traversal
    factor_traversal: Vec<NodeRef<'a, 'f, T, V>>,
}

pub struct DepsTraversal<'a: 'f, 't, 'f: 't, T: 'f + Copy, V: 'a> {
    factors: slice::Iter<'t, NodeRef<'a, 'f, T, V>>,
    dependencies_stack: &'t mut Vec<NodeRef<'a, 'f, T, V>>,
}

pub struct SumTraversal<'a: 'f, 't, 'f: 't, 'g, T: 'f + Copy, V: 'a> {
    fold: iter::Rev<iter::Enumerate<slice::Iter<'t, NodeRef<'a, 'f, T, V>>>>,
    factor_stack: &'t [NodeRef<'a, 'f, T, V>],
    factor_stack_bottom: &'t mut usize,
    dependencies_stack_bottom: &'t mut usize,
    grammar: &'g InternalGrammar,
}

pub enum TraversalBottom<'a: 'f, 'f, T: 'a + Copy, V: 'a> {
    Leaf(LeafHandle<'a, 'f, T, V>),
    Null(NullHandle<'a, 'f, T, V>),
}

impl<'a, 'f, 'g, T, V, O> Traversal<'a, 'f, 'g, T, V, O>
    where O: Order<'a, 'f, T, V>,
          T: Copy
{
    pub fn new(bocage: &'f Bocage<'a, 'f, 'g, T, V>, order: O) -> Self {
        bocage.initialize();
        Traversal {
            bocage,
            order,
            dependencies_stack: Vec::with_capacity(32),
            factor_traversal: Vec::with_capacity(16),
            factor_stack: Vec::with_capacity(128),
            dependencies_stack_bottom: 0,
            factor_stack_bottom: 0,
        }
    }

    #[inline]
    pub fn traverse(&mut self, root: NodeRef<'a, 'f, T, V>) {
        // Only from here, a Leaf can end up on a dependency stack.
        match root.get() {
            Sum { .. } | Product { .. } | Leaf { .. } => {
                self.dependencies_stack.push(root);
            }
            _ => {}
        }
        self.dependencies_stack_bottom = 1;
    }

    #[inline]
    pub fn traverse_deps<'t>(&'t mut self) -> Option<DepsTraversal<'a, 't, 'f, T, V>> {
        self.factor_stack.truncate(self.factor_stack_bottom);
        self.dependencies_stack.truncate(self.dependencies_stack_bottom);
        // Process nodes.
        if let Some(dependency) = self.dependencies_stack.last().cloned() {
            Some(self.unfold(dependency))
        } else {
            None
        }
    }

    #[inline]
    pub fn traverse_sum<'t>(&'t mut self) -> SumTraversal<'a, 't, 'f, 'g, T, V> {
        self.factor_stack_bottom = self.factor_stack.len();
        self.dependencies_stack_bottom = self.dependencies_stack.len();
        SumTraversal {
            fold: self.dependencies_stack.iter().enumerate().rev(),
            factor_stack: &self.factor_stack[..],
            factor_stack_bottom: &mut self.factor_stack_bottom,
            dependencies_stack_bottom: &mut self.dependencies_stack_bottom,
            grammar: self.bocage.grammar,
        }
    }

    #[inline]
    pub fn finish(&mut self, root: NodeRef<'a, 'f, T, V>) -> &'a [V] {
        // All nodes are now evaluated.
        match root.get() {
            Evaluated { values } => values,
            _ => unreachable!()
        }
    }

    fn unfold<'t>(&'t mut self, dependency: NodeRef<'a, 'f, T, V>)
        -> DepsTraversal<'a, 't, 'f, T, V>
    {
        // Apply order.
        let alternatives = self.order.sum(dependency.alternatives());
        let sum_bottom = self.factor_stack.len();
        for product in alternatives {
            let product_bottom = self.factor_stack.len();
            let product_action;
            match product.get() {
                Product { action, factors } => {
                    product.set(Product { action: action | (1 << 31), factors });
                    product_action = Some(action);
                }
                Leaf { .. } => {
                    product_action = None;
                }
                _ => unreachable!()
            };
            // Most unfolding happens here.
            self.unfold_factors(product);
            // Apply ordering.
            if let Some(len) = self.order.product(&self.factor_stack[product_bottom..]) {
                self.factor_stack.truncate(product_bottom + len);
            }
            if let Some(action) = product_action {
                product.set(ShallowProduct {
                    action,
                    factor_stack_bottom: product_bottom,
                });
            }
        }
        DepsTraversal {
            factors: self.factor_stack[sum_bottom..].iter(),
            dependencies_stack: &mut self.dependencies_stack,
        }
    }

    fn unfold_factors(&mut self, product: NodeRef<'a, 'f, T, V>) {
        self.factor_traversal.push(product);
        while let Some(node) = self.factor_traversal.pop() {
            self.bocage.prepare_product_tree_node(node);
            if let Some(factors) = self.intermediate_product(node) {
                if let Some(right) = factors.right {
                    self.factor_traversal.push(right);
                }
                self.factor_traversal.push(factors.left);
            } else {
                self.factor_stack.push(node);
            }
        }
    }

    #[inline]
    fn intermediate_product(&self, node: NodeRef<'a, 'f, T, V>) -> Option<Factors<'a, 'f, T, V>> {
        match node.get() {
            Product { action, factors } => {
                if self.bocage.grammar.get_eval(action).is_none() {
                    Some(factors)
                } else {
                    None
                }
            }
            // When a node is a Sum, we assume it has an action. The grammar
            // rewrites must not add ambiguous rules (sum rules) with null actions,
            // because it would break our assumption.
            _ => None
        }
    }
}

impl<'a, 't, 'f, 'g, T, V> Iterator for DepsTraversal<'a, 't, 'f, T, V> where T: 'a + Copy {
    type Item = TraversalBottom<'a, 'f, T, V>;

    fn next(&mut self) -> Option<Self::Item> {
        for factor in &mut self.factors {
            match factor.get() {
                Evaluated { .. } => {}
                LeafWithValue { symbol, value } => {
                    return Some(TraversalBottom::Leaf(LeafHandle {
                        factor,
                        terminal: symbol,
                        value,
                    }));
                }
                Leaf { symbol } => {
                    return Some(TraversalBottom::Null(NullHandle {
                        factor,
                        symbol,
                    }));
                }
                _ => {
                    self.dependencies_stack.push(factor)
                }
            }
        }
        None
    }
}

impl<'a, 't, 'f, 'g, T, V> Iterator for SumTraversal<'a, 't, 'f, 'g, T, V> where T: 'a + Copy {
    type Item = SumHandle<'a, 't, 'f, 'g, T, V>;
    fn next(&mut self) -> Option<Self::Item> {
        for (pos, dependency) in &mut self.fold {
            match dependency.get() {
                Evaluated { .. } => {}
                _ => {
                    match dependency.factor_stack_bottom() {
                        Some(newer_bottom) => {
                            let older_bottom = *self.factor_stack_bottom;
                            let sum = SumHandle {
                                node: dependency,
                                summands: dependency.alternatives(),
                                factor_stack: &self.factor_stack[..older_bottom],
                                grammar: self.grammar,
                            };
                            *self.factor_stack_bottom = newer_bottom;
                            return Some(sum);
                        }
                        None => {
                            *self.dependencies_stack_bottom = pos + 1;
                            return None;
                        }
                    }
                }
            }
        }
        *self.dependencies_stack_bottom = 0;
        None
    }
}
