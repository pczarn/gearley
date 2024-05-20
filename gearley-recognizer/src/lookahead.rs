use cfg_symbol::Symbol;
use gearley_grammar::Grammar;

pub(crate) trait Lookahead {
    fn sym(&self) -> Symbol;

    fn set_hint<G: Grammar>(&mut self, hint: Symbol, grammar: &G);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead {
    next_symbol: Symbol,
    useless_symbol: Symbol,
}

impl Lookahead for DefaultLookahead {
    fn sym(&self) -> Symbol {
        self.next_symbol
    }

    fn set_hint<G: Grammar>(&mut self, hint: Symbol, grammar: &G) {
        self.next_symbol = grammar.to_internal(hint).unwrap();
    }

    fn clear_hint(&mut self) {
        self.next_symbol = self.useless_symbol;
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
}

impl<'a, L: Lookahead> Lookahead for &'a mut L {
    fn clear_hint(&mut self) {
        (**self).clear_hint()
    }

    fn set_hint<G: Grammar>(&mut self, hint: Symbol, grammar: &G) {
        (**self).set_hint(hint, grammar)
    }

    fn sym(&self) -> Symbol {
        (**self).sym()
    }
}
