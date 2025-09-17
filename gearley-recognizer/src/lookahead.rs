use crate::local_prelude::*;

pub trait Lookahead {
    fn sym(&self) -> Option<Symbol>;

    fn set_hint(&mut self, hint: Symbol);

    fn clear_hint(&mut self);
}

pub(crate) struct DefaultLookahead {
    next_symbol: Option<Symbol>,
}

impl Lookahead for DefaultLookahead {
    fn sym(&self) -> Option<Symbol> {
        self.next_symbol
    }

    fn set_hint(&mut self, hint: Symbol) {
        self.next_symbol = Some(hint);
    }

    fn clear_hint(&mut self) {
        self.next_symbol = None;
    }
}

impl DefaultLookahead {
    pub(crate) fn new<G: Grammar>(_grammar: &G) -> Self {
        DefaultLookahead { next_symbol: None }
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
