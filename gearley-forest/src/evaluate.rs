pub trait Evaluate<S> {
    type Elem: Default;

    fn leaf(&self, terminal: S, values: u32) -> Self::Elem;
    fn product<'a>(&self, action: u32, args: impl Iterator<Item = &'a Self::Elem>) -> Self::Elem where Self::Elem: 'a;
    fn nulling<'r>(&self, symbol: S, results: &'r mut Vec<Self::Elem>);
}

