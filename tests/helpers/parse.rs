use std::borrow::Borrow;
use std::fmt::Debug;

use cfg::Symbol;
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;
use gearley::forest::{Bocage, NullForest};
use gearley::forest::order::NullOrder;

pub trait Parse {
    fn parse(&mut self, tokens: &[u32]) -> bool;
}

impl<'g, G> Parse for Recognizer<'g, Bocage<G>>
    where Self: Debug,
          G: Borrow<InternalGrammar>,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        for (i, &token) in tokens.iter().enumerate() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(Symbol::from(token), i as u32);
            trace!("before pass 2 {:?}", &*self);
            assert!(self.end_earleme());
        }
        trace!("finished {:?}", &*self);

        if self.is_finished() {
            self.forest.mark_alive(self.finished_node().unwrap(), NullOrder::new());
        }
        self.is_finished()
    }
}

impl<'g> Parse for Recognizer<'g, NullForest>
    where Self: Debug,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        for &token in tokens.iter() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(Symbol::from(token), ());
            trace!("before pass 2 {:?}", &*self);
            assert!(self.end_earleme());
        }
        trace!("finished {:?}", &*self);

        self.is_finished()
    }
}
