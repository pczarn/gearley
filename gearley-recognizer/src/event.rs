use cfg_symbol::Symbol;
use gearley_forest::Forest;
use gearley_grammar::{Grammar, EventAndDistance, ExternalDottedRule};

use crate::local_prelude::*;

impl<F, G, P> Recognizer<G, F, P>
where
    F: Forest,
    G: Grammar,
    P: PerfHint,
{
    pub fn trace(&self) -> impl Iterator<Item = (ExternalDottedRule, usize)> + use<'_, F, G, P> {
        let [trace_predict, trace0, trace1] = self.grammar.trace();
        self.predicted_symbols().map(|sym| (trace_predict[sym.usize()], self.earleme())).chain(self.medial_items().map(|item| (trace0[item.dot as usize], item.origin as usize)))
    }

    pub fn events(&self) -> impl Iterator<Item = EventAndDistance> + use<'_, F, G, P> {
        let (events_predict, events_flat) = self.grammar.events();
        self.predicted_symbols().map(|sym| events_predict[sym.usize()]).chain(self.medial_items().map(|item| events_flat[item.dot as usize]))
    }

    pub fn minimal_distances(&self) -> impl Iterator<Item = u32> + use<'_, F, G, P> {
        self.events().map(|(_id, distance)| distance.distance)
    }

    pub fn expected_terminals(&self) -> impl Iterator<Item = Symbol> + use<'_, F, G, P> {
        let rhs1 = self.grammar.rhs1();
        self.medial_items().map(|item| rhs1[item.dot as usize].unwrap())
    }
}
