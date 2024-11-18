use bit_matrix::row::BitSlice;
use cfg_symbol::Symbolic;

use gearley_forest::Forest;
use gearley_grammar::Grammar;

use crate::local_prelude::*;

pub(super) trait Predict {
    fn predict<S: Symbolic>(&mut self, sym: S, source: &BitSlice);

    fn clear(&mut self);
}

impl<F, G, P> Recognizer<G, F, P>
    where F: Forest<G::Symbol>,
    G: Grammar,
    P: PerfHint,
{
    /// Makes the current Earley set predict a given symbol.
    pub fn predict(&mut self, symbol: G::Symbol) {
        let earleme = self.earleme();
        self.predicted[earleme].predict(symbol, self.grammar.prediction_row(symbol));
    }
}

impl Predict for BitSlice {
    fn predict<S: Symbolic>(&mut self, sym: S, source: &BitSlice) {
        if !self[sym.usize()] {
            // The source in the prediction matrix is the row that corresponds to the predicted
            // symbol.
            //
            // The destination in `predicted` is now the `self` that corresponds to the current
            // location.
            for (dst, &src) in self.iter_blocks_mut().zip(source.iter_blocks()) {
                *dst |= src;
            }
        }
    }

    fn clear(&mut self) {
        for dst in self.iter_blocks_mut() {
            *dst = 0;
        }
    }
}
