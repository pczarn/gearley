use cfg::Symbol;

use util::slice_builder::SliceBuilder;
use super::cartesian_product::CartesianProduct;
use super::evaluate::ProductHandle;

pub trait ActionEvaluator<'a, T, V> where T: Copy {
    fn production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>);
    fn next(&mut self) -> Option<V>;
    fn leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V;
    fn nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>);
}

pub struct ActionClosureEvaluator<'a, V: 'a, F, G, H> {
    cartesian_product: CartesianProduct<'a, V>,
    action: u32,
    exhausted: bool,
    leaf: F,
    rule: G,
    null: H,
}

impl<'t, V, F, G, H> ActionClosureEvaluator<'t, V, F, G, H> {
    pub fn new(leaf: F, rule: G, null: H) -> Self {
        ActionClosureEvaluator {
            cartesian_product: CartesianProduct::new(),
            action: 0,
            exhausted: false,
            leaf: leaf,
            rule: rule,
            null: null,
        }
    }
}

impl<'a, T, V, F, G, H> ActionEvaluator<'a, T, V> for ActionClosureEvaluator<'a, V, F, G, H>
    where F: FnMut(Symbol, Option<&T>) -> V,
          G: FnMut(u32, &[&V]) -> V,
          H: for<'r> FnMut(Symbol, &'r mut SliceBuilder<'a, V>),
          T: Copy
{
    fn production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>) {
        self.cartesian_product.from_production(product);
        self.action = product.action();
        self.exhausted = false;
    }

    fn next(&mut self) -> Option<V> {
        if self.exhausted {
            None
        } else {
            let result = (self.rule)(self.action, self.cartesian_product.as_slice());
            self.exhausted |= self.cartesian_product.next().is_none();
            Some(result)
        }
    }

    fn leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V {
        (self.leaf)(terminal, arg)
    }

    fn nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>) {
        (self.null)(symbol, store)
    }
}

impl<'a, 'r, T, V, U> ActionEvaluator<'a, T, V> for &'r mut U
    where U: ActionEvaluator<'a, T, V>,
          T: Copy
{
    fn production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>) {
        (*self).production(product)
    }

    fn next(&mut self) -> Option<V> {
        (*self).next()
    }

    fn leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V {
        (*self).leaf(terminal, arg)
    }

    fn nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>) {
        (*self).nulling(symbol, store)
    }
}
