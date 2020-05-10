use std::borrow::Borrow;
use std::fmt::Debug;

use cfg::Symbol;
use gearley::forest::bocage::order::NullOrder;
use gearley::forest::compact_bocage::order::NullOrder as CompactNullOrder;
use gearley::forest::{Bocage, CompactBocage, NullForest};
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;
use gearley::policy::PerformancePolicy;

pub trait Parse {
    fn parse(&mut self, tokens: &[u32]) -> bool;
}

impl<'g, G, P> Parse for Recognizer<'g, Bocage<G, P>, P>
where
    Self: Debug,
    G: Borrow<InternalGrammar<P>>,
    P: PerformancePolicy,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        let mut iter = tokens.iter().enumerate().peekable();
        while let Some((i, &token)) = iter.next() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(P::Symbol::from(Symbol::from(token)), i as u32);
            trace!("before pass 2 {:?}", &*self);
            self.lookahead_hint(iter.peek().map(|(_i, &t)| P::Symbol::from(Symbol::from(t))));
            assert!(self.end_earleme(), "failed to parse after {}@{}", token, i);
        }
        trace!("finished {:?}", &*self);

        // if self.is_finished() {
        //     self.forest
        //         .mark_alive(self.finished_node().unwrap(), NullOrder::new());
        // }
        self.is_finished()
    }
}

impl<'g, G, P> Parse for Recognizer<'g, CompactBocage<G, P>, P>
where
    Self: Debug,
    G: Borrow<InternalGrammar<P>>,
    P: PerformancePolicy,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        let mut iter = tokens.iter().enumerate().peekable();
        while let Some((i, &token)) = iter.next() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(P::Symbol::from(Symbol::from(token)), i as u32);
            trace!("before pass 2 {:?}", &*self);
            self.lookahead_hint(iter.peek().map(|(_i, &t)| P::Symbol::from(Symbol::from(t))));
            assert!(self.end_earleme(), "failed to parse after {}@{}", token, i);
        }
        trace!("finished {:?}", &*self);

        // if self.is_finished() {
        //     self.forest
        //         .mark_alive(self.finished_node().unwrap(), CompactNullOrder::new());
        // }
        self.is_finished()
    }
}

impl<'g, P> Parse for Recognizer<'g, NullForest, P>
where
    Self: Debug,
    P: PerformancePolicy,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        for &token in tokens.iter() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(P::Symbol::from(Symbol::from(token)), ());
            trace!("before pass 2 {:?}", &*self);
            assert!(self.end_earleme());
        }
        trace!("finished {:?}", &*self);

        self.is_finished()
    }
}
