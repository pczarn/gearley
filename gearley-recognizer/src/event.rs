use std::iter::{Chain, Zip};
use std::slice;

use gearley_forest::Forest;
use gearley_grammar::{Grammar, EventAndDistance, ExternalDottedRule};
use crate::local_prelude::*;

pub struct MedialItems<'a, N: 'a> {
    pub(super) iter: slice::Iter<'a, Item<N>>,
}

pub struct Medial<'a, T: 'a, N: 'a> {
    events: &'a [T],
    items: MedialItems<'a, N>,
}

pub struct Events<'a, N: 'a> {
    iter: Chain<Prediction<'a, EventAndDistance>, Medial<'a, EventAndDistance, N>>,
}

pub struct Distances<'a, N: 'a> {
    iter: Chain<Prediction<'a, EventAndDistance>, Medial<'a, EventAndDistance, N>>,
}

pub struct Trace<'a, N: 'a> {
    iter: Chain<
        Prediction<'a, Option<ExternalDottedRule>>,
        Medial<'a, Option<ExternalDottedRule>, N>,
    >,
}

pub struct ExpectedTerminals<'a, N: 'a, S> {
    prev_scan_iter: MedialItems<'a, N>,
    rhs1: &'a [Option<S>],
}

impl<'a, N> Iterator for MedialItems<'a, N> {
    type Item = &'a Item<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> Iterator for Prediction<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for (is_present, elem) in &mut self.iter {
            if is_present {
                return Some((elem, self.origin));
            }
        }
        None
    }
}

impl<'a, T, L> Iterator for Medial<'a, T, L> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let events = &self.events;
        self.items
            .next()
            .map(|ei| (&events[ei.dot as usize], ei.origin as usize))
    }
}

impl<'a, L> Iterator for Events<'a, L> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(event_id, _distance), _origin) in &mut self.iter {
            if event_id.is_some() {
                return event_id.map(|nonzero| nonzero.into());
            }
        }
        None
    }
}

impl<'a, L> Iterator for Distances<'a, L> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(_event_id, distance), _origin) in &mut self.iter {
            if distance.is_some() {
                return distance.map(|nonzero| nonzero.into());
            }
        }
        None
    }
}

impl<'a, N> Iterator for Trace<'a, N> {
    type Item = (ExternalDottedRule, usize);

    fn next(&mut self) -> Option<(ExternalDottedRule, usize)> {
        for (&external_dr_opt, origin) in &mut self.iter {
            if let Some(external_dotted_rule) = external_dr_opt {
                return Some((external_dotted_rule, origin));
            }
        }
        None
    }
}

impl<'a, N, S> Iterator for ExpectedTerminals<'a, N, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_scan_iter
            .next()
            .map(|item| self.rhs1[item.dot as usize].unwrap())
    }
}

impl<F, G, P> Recognizer<G, F, P>
where
    F: Forest,
    G: Grammar,
    P: PerfHint,
{
    pub fn trace(&self) -> impl Iterator<Item = (ExternalDottedRule, usize)> {
        let trace = self.grammar.trace();
        let prediction = Prediction {
            iter: self.predicted_symbols().iter.zip(trace[0].iter()),
            origin: self.earleme(),
        };
        let medial = Medial {
            events: trace[1],
            items: self.medial_items(),
        };
        Trace {
            iter: prediction.chain(medial),
        }

    }

    pub fn events(&self) -> impl Iterator<Item = EventAndDistance> + use<'_, F, G, P> {
        let (events_predict, events_flat) = self.grammar.events();
        self.predicted_symbols().map(|sym| events_predict[sym.usize()]).chain(self.medial_items().map(|item| events_flat[item.dot as usize]))
    }

    pub fn minimal_distances(&self) -> impl Iterator<Item = u32> {
        self.events().map(|(id, distance)| distance.distance)
    }

    pub fn expected_terminals(&self) -> ExpectedTerminals<F::NodeRef, G::Symbol> {
        ExpectedTerminals {
            prev_scan_iter: self.medial_items(),
            rhs1: self.grammar.rhs1(),
        }
    }
}

#[test]
fn test_prediction_events() {
    use bit_matrix::BitMatrix;
    let mut bit_m = BitMatrix::new(1, 5);
    bit_m.set(0, 2, true);
    let mut row = bit_m.iter_row(0);
    assert_eq!(row.next(), Some(false));
    assert_eq!(row.next(), Some(false));
    assert_eq!(row.next(), Some(true));
    assert_eq!(row.next(), Some(false));
    assert_eq!(row.next(), Some(false));
    assert_eq!(row.next(), None);
    let ev = [0, 1, 2, 3, 4];
    let mut pred = Prediction {
        iter: bit_m.iter_row(0).zip(&ev[0..5]),
        origin: 123,
    };
    assert_eq!(pred.next(), Some((&2, 123)));
    assert_eq!(pred.next(), None);
}
