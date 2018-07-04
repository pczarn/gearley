use cfg::Symbol;

use util::slice_builder::SliceBuilder;
use super::cartesian_product::CartesianProduct;
use super::evaluate::ProductHandle;

pub trait Invoke<'a, T, V> where T: Copy {
    fn set_production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>);
    fn invoke_next_factor(&mut self) -> Option<V>;
    fn invoke_leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V;
    fn invoke_nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>);
}

pub struct ClosureInvoker<'a, V: 'a, F, G, H> {
    cartesian_product: CartesianProduct<'a, V>,
    action: u32,
    exhausted: bool,
    leaf: F,
    rule: G,
    null: H,
}

impl<'t, V, F, G, H> ClosureInvoker<'t, V, F, G, H> {
    pub fn new(leaf: F, rule: G, null: H) -> Self {
        ClosureInvoker {
            cartesian_product: CartesianProduct::new(),
            action: 0,
            exhausted: false,
            leaf,
            rule,
            null,
        }
    }
}

impl<'a, T, V, F, G, H> Invoke<'a, T, V> for ClosureInvoker<'a, V, F, G, H>
    where F: FnMut(Symbol, Option<&T>) -> V,
          G: FnMut(u32, &[&V]) -> V,
          H: for<'r> FnMut(Symbol, &'r mut SliceBuilder<'a, V>),
          T: Copy
{
    fn set_production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>) {
        self.cartesian_product.clear();
        self.cartesian_product.extend(product);
        self.action = product.action();
        self.exhausted = false;
    }

    fn invoke_next_factor(&mut self) -> Option<V> {
        if self.exhausted {
            None
        } else {
            let result = (self.rule)(self.action, self.cartesian_product.as_slice());
            self.exhausted |= self.cartesian_product.next().is_none();
            Some(result)
        }
    }

    fn invoke_leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V {
        (self.leaf)(terminal, arg)
    }

    fn invoke_nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>) {
        (self.null)(symbol, store)
    }
}

impl<'a, 'r, T, V, U> Invoke<'a, T, V> for &'r mut U
    where U: Invoke<'a, T, V>,
          T: Copy
{
    fn set_production<'t, 'f>(&mut self, product: &ProductHandle<'a, 't, 'f, T, V>) {
        (*self).set_production(product)
    }

    fn invoke_next_factor(&mut self) -> Option<V> {
        (*self).invoke_next_factor()
    }

    fn invoke_leaf(&mut self, terminal: Symbol, arg: Option<&T>) -> V {
        (*self).invoke_leaf(terminal, arg)
    }

    fn invoke_nulling(&mut self, symbol: Symbol, store: &mut SliceBuilder<'a, V>) {
        (*self).invoke_nulling(symbol, store)
    }
}
