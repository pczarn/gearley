use std::convert::TryInto;
use std::iter::{Zip, Chain};
use std::slice;

use bit_matrix;
use cfg::symbol::Symbol;

use forest::Forest;
use grammar::{ExternalDottedRule, Event};
use item::Item;
use recognizer::Recognizer;
use policy::PerformancePolicy;

type IterPredictionBitfield<'a> = bit_matrix::row::Iter<'a>;

pub struct PredictedSymbols<'a> {
    pub(in super) iter: IterPredictionBitfield<'a>,
    pub(in super) idx: usize,
}

pub struct MedialItems<'a, N: 'a, P> where P: PerformancePolicy {
    pub(in super) iter: slice::Iter<'a, Item<N, P>>,
}

pub struct Prediction<'a, T: 'a> {
    iter: Zip<IterPredictionBitfield<'a>, slice::Iter<'a, T>>,
    origin: usize,
}

pub struct Medial<'a, T: 'a, N: 'a, P> where P: PerformancePolicy {
    events: &'a [T],
    items: MedialItems<'a, N, P>,
}

pub struct Events<'a, N: 'a, P> where P: PerformancePolicy {
    iter: Chain<
        Prediction<'a, Event>,
        Medial<'a, Event, N, P>
    >
}

pub struct Distances<'a, N: 'a, P> where P: PerformancePolicy {
    iter: Chain<
        Prediction<'a, Event>,
        Medial<'a, Event, N, P>
    >
}

pub struct Trace<'a, N: 'a, P> where P: PerformancePolicy {
    iter: Chain<
        Prediction<'a, Option<ExternalDottedRule>>,
        Medial<'a, Option<ExternalDottedRule>, N, P>
    >
}

pub struct ExpectedTerminals<'a, N: 'a, P> where P: PerformancePolicy {
    prev_scan_iter: MedialItems<'a, N, P>,
    rhs1: &'a [Option<P::Symbol>],
}

impl<'a> Iterator for PredictedSymbols<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        for is_present in &mut self.iter {
            let symbol = Symbol::from(self.idx);
            self.idx += 1;
            if is_present {
                return Some(symbol);
            }
        }
        None
    }
}

impl<'a, N, P: PerformancePolicy> Iterator for MedialItems<'a, N, P> {
    type Item = &'a Item<N, P>;

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

impl<'a, T, L, P: PerformancePolicy> Iterator for Medial<'a, T, L, P> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let events = &self.events;
        self.items.next().map(|ei| {
            (&events[ei.dot.try_into().ok().unwrap()], ei.origin as usize)
        })
    }
}

impl<'a, L, P: PerformancePolicy> Iterator for Events<'a, L, P> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(event_id, _distance), _origin) in &mut self.iter {
            if event_id.is_some() {
                return event_id.into();
            }
        }
        None
    }
}

impl<'a, L, P: PerformancePolicy> Iterator for Distances<'a, L, P> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(_event_id, distance), _origin) in &mut self.iter {
            if distance.is_some() {
                return distance.into();
            }
        }
        None
    }
}

impl<'a, N, P: PerformancePolicy> Iterator for Trace<'a, N, P> {
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

impl<'a, N, P: PerformancePolicy> Iterator for ExpectedTerminals<'a, N, P> {
    type Item = P::Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_scan_iter.next().map(|item| {
            self.rhs1[item.dot.try_into().ok().unwrap()].unwrap()
        })
    }
}

impl<'g, F, P> Recognizer<'g, F, P>
    where F: Forest,
          P: PerformancePolicy,
{
    pub fn trace(&self) -> Trace<F::NodeRef, P> {
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

    pub fn events(&self) -> Events<F::NodeRef, P> {
        let (events_predict, events_flat) = self.grammar.events();
        let prediction = Prediction {
            iter: self.predicted_symbols().iter.zip(events_predict.iter()),
            origin: self.earleme(),
        };
        let medial = Medial {
            events: events_flat,
            items: self.medial_items(),
        };
        Events {
            iter: prediction.chain(medial),
        }
    }

    pub fn minimal_distances(&self) -> Distances<F::NodeRef, P> {
        Distances {
            iter: self.events().iter,
        }
    }

    pub fn expected_terminals(&self) -> ExpectedTerminals<F::NodeRef, P> {
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
