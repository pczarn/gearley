use gearley_grammar::Grammar;

pub trait Lookahead<S> {
    fn sym(&self) -> S;

    fn set_hint(&mut self, hint: S);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead<S> {
    next_symbol: S,
    useless_symbol: S,
}

pub(crate) struct LookaheadWithGrammar<'a, 'b, G: Grammar, L> {
    lookahead: &'b mut L,
    grammar: &'a G,
}

impl<'a, 'b, G: Grammar> Lookahead<G::Symbol> for LookaheadWithGrammar<'a, 'b, G, DefaultLookahead<G::Symbol>> {
    fn sym(&self) -> G::Symbol {
        self.lookahead.next_symbol
    }

    fn set_hint(&mut self, hint: G::Symbol) {
        self.lookahead.next_symbol = self.grammar.to_internal(hint).unwrap();
    }

    fn clear_hint(&mut self) {
        self.lookahead.next_symbol = self.lookahead.useless_symbol;
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

    pub(crate) fn mut_with_grammar<'a, 'b, G: Grammar<Symbol = S>>(&'b mut self, grammar: &'a G) -> LookaheadWithGrammar<'a, 'b, G, Self> {
        LookaheadWithGrammar {
            lookahead: self,
            grammar,
        }
    }
}

impl<'a, S, L: Lookahead<S>> Lookahead<S> for &'a mut L {
    fn clear_hint(&mut self) {
        (**self).clear_hint()
    }

    fn set_hint(&mut self, hint: S) {
        (**self).set_hint(hint)
    }

    fn sym(&self) -> S {
        (**self).sym()
    }
}
