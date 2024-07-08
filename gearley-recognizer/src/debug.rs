use std::fmt;

use gearley_forest::Forest;
use gearley_grammar::Grammar;
use crate::Recognizer;

use super::performance_policy::PerformancePolicy;

impl<F, G, P> fmt::Debug for Recognizer<G, F, P>
    where
        F: Forest,
        G: Grammar + fmt::Debug,
        P: PerformancePolicy,
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
