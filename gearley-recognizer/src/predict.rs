use bit_matrix::row::BitSlice;

use crate::local_prelude::*;

pub(super) trait Predict {
    fn predict(&mut self, sym: Symbol, source: &BitSlice);

    fn clear(&mut self);
}

impl Predict for BitSlice {
    fn predict(&mut self, sym: Symbol, source: &BitSlice) {
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
