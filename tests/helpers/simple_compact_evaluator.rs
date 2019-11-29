use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;

use cfg::Symbol;

use gearley::forest::compact_bocage::traverse::{LeafHandle, NullingHandle, SumHandle, Traverse};
use gearley::forest::node_handle::NodeHandle;

use gearley::grammar::InternalGrammar;

use super::cartesian_product::CartesianProduct;

pub struct SimpleCompactEvaluator<V, F, G, H> {
    values: Vec<V>,
    evaluated: BTreeMap<NodeHandle, Vec<V>>,
    leaf: F,
    rule: G,
    null: H,
}

impl<V, FLeaf, FRule, FNull> SimpleCompactEvaluator<V, FLeaf, FRule, FNull>
where
    FLeaf: FnMut(Symbol) -> V,
    FRule: FnMut(u32, &[&V]) -> V,
    FNull: for<'r> FnMut(Symbol, &'r mut Vec<V>),
    V: fmt::Debug + Clone,
{
    pub fn new(leaf: FLeaf, rule: FRule, null: FNull) -> Self {
        SimpleCompactEvaluator {
            values: vec![],
            evaluated: BTreeMap::new(),
            leaf,
            rule,
            null,
        }
    }

    pub fn traverse<'f, G>(&mut self, traverse: &mut Traverse<'f, G>, root: NodeHandle) -> Vec<V>
    where
        G: Borrow<InternalGrammar>,
    {
        while let Some(mut item) = traverse.next_node() {
            match &mut item.item {
                &mut SumHandle(ref mut products) => {
                    while let Some(product) = products.next_product() {
                        let mut cartesian_product = CartesianProduct::new();
                        for &(_sym, handle) in product.factors {
                            cartesian_product.push(&self.evaluated[&handle][..]);
                        }
                        loop {
                            let v = (self.rule)(product.action, cartesian_product.as_slice());
                            self.values.push(v);
                            if !cartesian_product.advance() {
                                break;
                            }
                        }
                    }
                }
                &mut NullingHandle => {
                    (self.null)(item.symbol, &mut self.values);
                }
                &mut LeafHandle => {
                    let v = (self.leaf)(item.symbol);
                    self.values.push(v);
                }
            }
            self.evaluated
                .insert(item.handle(), mem::replace(&mut self.values, vec![]));
            item.end_evaluation();
        }
        self.evaluated[&root].clone()
    }
}
