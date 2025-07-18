use cfg_symbol::Symbol;

pub trait Evaluate {
    type Elem: Default;

    fn leaf(&self, terminal: Symbol, values: u32) -> Self::Elem;
    fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a;
    fn nulling<'r>(&self, symbol: Symbol, results: &'r mut Vec<Self::Elem>);
}

