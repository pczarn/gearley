use std::fmt::Debug;

use cfg::Cfg;
use cfg_symbol::Symbol;
use gearley_default_grammar::DefaultGrammar;
use log::trace;

#[cfg(feature = "simple-bocage")]
use simple_bocage::Bocage;
use gearley_forest::NullForest;
use gearley_grammar::Grammar;
use gearley_recognizer::{Recognizer, lookahead::Lookahead};

pub trait RecognizerParseExt {
    fn parse(&mut self, tokens: &[Symbol]) -> bool;
}

#[cfg(feature = "simple-bocage")]
impl<G> RecognizerParseExt for Recognizer<G, Bocage>
where
    Self: Debug,
    G: Grammar,
{
    #[inline]
    fn parse(&mut self, tokens: &[Symbol]) -> bool {
        let mut iter = tokens.iter().enumerate().peekable();
        while let Some((i, &token)) = iter.next() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(token, i as u32);
            trace!("before pass 2 {:?}", &*self);
            if let Some((_i, t)) = iter.peek() {
                self.lookahead().set_hint(**t);
            } else {
                self.lookahead().clear_hint();
            }
            assert!(self.end_earleme(), "failed to parse after {:?}@{}", token, i);
        }
        trace!("finished {:?}", &*self);

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

impl<G> RecognizerParseExt for Recognizer<G, NullForest> where
    Self: Debug,
    G: Grammar,
{
    #[inline]
    fn parse(&mut self, tokens: &[Symbol]) -> bool {
        for &token in tokens.iter() {
            self.begin_earleme();
            trace!("before pass 1 {:?}", &*self);
            self.scan(token, ());
            trace!("before pass 2 {:?}", &*self);
            assert!(self.end_earleme());
        }
        trace!("finished {:?}", &*self);

        self.is_finished()
    }
}

pub fn parse_terminal_list<'a>(cfg: Cfg, grammar: DefaultGrammar, terminal_list: impl Iterator<Item = &'a str>) -> bool {
    let mut recognizer = Recognizer::with_forest(&grammar, Bocage::new(&grammar));
    let name_map = cfg.sym_source().name_map();
    let mut tokens = vec![];
    for word in terminal_list {
        tokens.push(name_map[word]);
    }
    recognizer.parse(&tokens)
}
