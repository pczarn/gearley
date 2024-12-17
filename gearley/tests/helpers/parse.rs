use std::fmt::Debug;

use gearley::{Bocage, Grammar, NullForest, Recognizer};
use gearley_recognizer::lookahead::Lookahead;

pub trait Parse {
    fn parse(&mut self, tokens: &[u32]) -> bool;
}

impl<G> Parse for Recognizer<G, Bocage<G>>
where
    Self: Debug,
    G: Grammar,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        let mut iter = tokens.iter().enumerate().peekable();
        while let Some((i, &token)) = iter.next() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(G::Symbol::from(token as usize), i as u32);
            trace!("before pass 2 {:?}", &*self);
            if let Some((_i, &t)) = iter.peek() {
                self.lookahead().set_hint(G::Symbol::from(t as usize));
            } else {
                self.lookahead().clear_hint();
            }
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

// impl<G> Parse for Recognizer<CompactBocage<G>, G>
// where
//     Self: Debug,
//     G: Grammar,
// {
//     #[inline]
//     fn parse(&mut self, tokens: &[u32]) -> bool {
//         let mut iter = tokens.iter().enumerate().peekable();
//         while let Some((i, &token)) = iter.next() {
//             self.begin_earleme();
//             trace!("before pass 1 {:?}", &*self);
//             self.scan(Symbol::from(token), i as u32);
//             trace!("before pass 2 {:?}", &*self);
//             self.lookahead_hint(iter.peek().map(|(_i, &t)| Symbol::from(t)));
//             assert!(self.end_earleme(), "failed to parse after {}@{}", token, i);
//         }
//         trace!("finished {:?}", &*self);

//         if self.is_finished() {
//             self.forest
//                 .mark_alive(self.finished_node().unwrap(), CompactNullOrder::new());
//         }
//         self.is_finished()
//     }
// }

impl<G> Parse for Recognizer<G, NullForest>
where
    Self: Debug,
    G: Grammar,
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        for &token in tokens.iter() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(G::Symbol::from(token as usize), ());
            trace!("before pass 2 {:?}", &*self);
            assert!(self.end_earleme());
        }
        trace!("finished {:?}", &*self);

        self.is_finished()
    }
}
