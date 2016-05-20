#![allow(dead_code)]

#[macro_use]
pub mod ambiguous_arith;
#[macro_use]
pub mod precedenced_arith;

use cfg::Symbol;
use gearley::recognizer::Recognizer;
use gearley::forest::Forest;

pub trait Parse {
    fn parse(&mut self, tokens: &[u32]) -> bool;
}

impl<'f, 'g, F> Parse for Recognizer<'f, 'g, F>
    where F: Forest<'f> + 'f,
          F::LeafValue: Default
{
    #[inline]
    fn parse(&mut self, tokens: &[u32]) -> bool {
        for &token in tokens {
            self.scan(Symbol::from(token), Default::default());
            assert!(self.advance());
        }
        self.is_finished()
    }
}
