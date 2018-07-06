use std::borrow::Borrow;
use std::fmt;
use std::mem;

use cfg::Symbol;

use gearley::forest::traverse::{
    Traverse,
    SumHandle,
    NullingHandle,
    LeafHandle,
};

use gearley::grammar::InternalGrammar;

use super::cartesian_product::CartesianProduct;

pub struct SimpleEvaluator<V, F, G, H> {
    values: Vec<V>,
    evaluated: Vec<Vec<V>>,
    leaf: F,
    rule: G,
    null: H,
}

impl<V, FLeaf, FRule, FNull> SimpleEvaluator<V, FLeaf, FRule, FNull>
    where FLeaf: FnMut(Symbol) -> V,
          FRule: FnMut(u32, &[&V]) -> V,
          FNull: for<'r> FnMut(Symbol, &'r mut Vec<V>),
          V: fmt::Debug,
{
    pub fn new(leaf: FLeaf, rule: FRule, null: FNull) -> Self {
        SimpleEvaluator {
            values: vec![],
            evaluated: vec![],
            leaf,
            rule,
            null,
        }
    }

    pub fn traverse<'f, G>(
        &mut self,
        traverse: &mut Traverse<'f, G>,
    )
        -> Vec<V>
        where G: Borrow<InternalGrammar>,
    {
        while let Some(mut item) = traverse.next_node() {
            match &mut item.item {
                &mut SumHandle(ref mut products) => {
                    while let Some(product) = products.next_product() {
                        let mut cartesian_product = CartesianProduct::new();
                        for &(_sym, values_idx) in product.factors {
                            cartesian_product.push(&self.evaluated[values_idx as usize][..]);
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
                &mut LeafHandle(_) => {
                    let v = (self.leaf)(item.symbol);
                    self.values.push(v);
                }
            }
            let result = self.evaluated.len() as u32;
            self.evaluated.push(mem::replace(&mut self.values, vec![]));
            item.set_evaluation_result(result);
        }
        self.evaluated.pop().unwrap()
    }
}
