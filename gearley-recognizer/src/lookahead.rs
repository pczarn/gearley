use gearley_grammar::Grammar;

pub trait Lookahead<S> {
    fn sym(&self) -> S;

    fn set_hint<G: Grammar<Symbol = S>>(&mut self, hint: S, grammar: &G);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead<S> {
    next_symbol: S,
    useless_symbol: S,
}

impl<S: Copy> Lookahead<S> for DefaultLookahead<S> {
    fn sym(&self) -> S {
        self.next_symbol
    }

    fn set_hint<G: Grammar<Symbol = S>>(&mut self, hint: S, grammar: &G) {
        self.next_symbol = grammar.to_internal(hint).unwrap();
    }

    fn clear_hint(&mut self) {
        self.next_symbol = self.useless_symbol;
    }
}

impl<S: Copy> DefaultLookahead<S> {
    pub(crate) fn new<G: Grammar<Symbol = S>>(grammar: &G) -> Self {
        let useless_symbol = grammar.useless_symbol();
        DefaultLookahead {
            next_symbol: useless_symbol,
            useless_symbol,
        }
    }
}

impl<'a, S, L: Lookahead<S>> Lookahead<S> for &'a mut L {
    fn clear_hint(&mut self) {
        (**self).clear_hint()
    }

    fn set_hint<G: Grammar<Symbol = S>>(&mut self, hint: S, grammar: &G) {
        (**self).set_hint(hint, grammar)
    }

    fn sym(&self) -> S {
        (**self).sym()
    }
}
