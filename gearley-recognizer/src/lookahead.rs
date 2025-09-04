use crate::local_prelude::*;

pub trait Lookahead {
    fn sym(&self) -> Option<Symbol>;

    fn set_hint(&mut self, hint: Symbol);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead {
    next_symbol: Option<Symbol>,
}

pub(crate) struct LookaheadWithGrammar<'a, 'b, G: Grammar, L> {
    lookahead: &'b mut L,
    grammar: &'a G,
}

impl<'a, 'b, G: Grammar> Lookahead for LookaheadWithGrammar<'a, 'b, G, DefaultLookahead> {
    fn sym(&self) -> Option<Symbol> {
        self.lookahead.next_symbol
    }

    fn set_hint(&mut self, hint: Symbol) {
        self.lookahead.next_symbol = Some(hint);
    }

    fn clear_hint(&mut self) {
        self.lookahead.next_symbol = None;
    }
}

impl DefaultLookahead {
    pub(crate) fn new<G: Grammar>(_grammar: &G) -> Self {
        DefaultLookahead { next_symbol: None }
    }

    pub(crate) fn mut_with_grammar<'a, 'b, G: Grammar>(
        &'b mut self,
        grammar: &'a G,
    ) -> LookaheadWithGrammar<'a, 'b, G, Self> {
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

    fn set_hint(&mut self, hint: Symbol) {
        (**self).set_hint(hint)
    }

    fn sym(&self) -> Option<Symbol> {
        (**self).sym()
    }
}
