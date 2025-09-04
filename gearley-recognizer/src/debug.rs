use std::fmt;

use crate::local_prelude::*;
use gearley_forest::Forest;
use gearley_grammar::Grammar;

impl<F, G, P> fmt::Debug for Recognizer<G, F, P>
where
    F: Forest,
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
