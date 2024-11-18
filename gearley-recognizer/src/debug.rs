use std::fmt;

use gearley_forest::Forest;
use gearley_grammar::Grammar;
use crate::local_prelude::*;

impl<F, G, P> fmt::Debug for Recognizer<G, F, P>
    where
        F: Forest<G::Symbol>,
        G: Grammar + fmt::Debug,
        P: PerfHint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Recognizer {{ grammar: {:?}, \
            predicted: {:?}, medial: {:?}, \
            complete: {:?}, \
            earleme: {:?} }}",
            self.grammar,
            &self.predicted,
            &self.medial,
            &self.complete,
            self.earleme(),
        )
    }
}
