use cfg_symbol::Symbol;
use gearley_grammar::Grammar;

pub trait Lookahead {
    fn sym(&self) -> Symbol;

    fn set_hint(&mut self, hint: Symbol);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead {
    next_symbol: Symbol,
    useless_symbol: Symbol,
}

pub(crate) struct LookaheadWithGrammar<'a, 'b, G: Grammar, L> {
    lookahead: &'b mut L,
    grammar: &'a G,
}

impl<'a, 'b, G: Grammar> Lookahead for LookaheadWithGrammar<'a, 'b, G, DefaultLookahead> {
    fn sym(&self) -> Symbol {
        self.lookahead.next_symbol
    }

    fn set_hint(&mut self, hint: Symbol) {
        self.lookahead.next_symbol = self.grammar.to_internal(hint).unwrap();
    }

    fn clear_hint(&mut self) {
        self.lookahead.next_symbol = self.lookahead.useless_symbol;
    }
}

impl DefaultLookahead {
    pub(crate) fn new<G: Grammar>(grammar: &G) -> Self {
        let useless_symbol = grammar.useless_symbol();
        DefaultLookahead {
            next_symbol: useless_symbol,
            useless_symbol,
        }
    }

    pub(crate) fn mut_with_grammar<'a, 'b, G: Grammar>(&'b mut self, grammar: &'a G) -> LookaheadWithGrammar<'a, 'b, G, Self> {
        LookaheadWithGrammar {
            lookahead: self,
            grammar,
        }
    }
}

impl<'a, L: Lookahead> Lookahead for &'a mut L {
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
