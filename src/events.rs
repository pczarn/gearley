use std::iter;
use std::slice;

use bit_matrix::row;
use cfg::symbol::Symbol;

use forest::Forest;
use grammar::{ExternalDottedRule, Event};
use item::Item;
use recognizer::Recognizer;

pub type RawMedialItems<'a, L> = slice::Iter<'a, Item<L>>;

pub struct RawPredictedItems<'a> {
    iter: row::Iter<'a>,
    idx: usize,
}

pub struct PredictionEvents<'a, T: 'a> {
    iter: iter::Zip<row::Iter<'a>, slice::Iter<'a, T>>,
    origin: usize,
}

pub struct MedialEvents<'a, T: 'a, N: 'a> {
    events: &'a [T],
    items: RawMedialItems<'a, N>,
}

pub struct Events<'a, L: 'a> {
    iter: iter::Chain<PredictionEvents<'a, Event>, MedialEvents<'a, Event, L>>,
}

pub struct Distances<'a, L: 'a> {
    iter: iter::Chain<PredictionEvents<'a, Event>, MedialEvents<'a, Event, L>>,
}

pub struct Tracing<'a, L: 'a> {
    iter: iter::Chain<
        PredictionEvents<'a, Option<ExternalDottedRule>>,
        MedialEvents<'a, Option<ExternalDottedRule>, L>
    >
}

pub struct ExpectedTerminals<'a, N: 'a> {
    prev_scan_iter: RawMedialItems<'a, N>,
    rhs1: &'a [Option<Symbol>],
}

impl<'a> RawPredictedItems<'a> {
    pub fn new(row: row::Iter<'a>) -> Self {
        RawPredictedItems {
            iter: row,
            idx: 0,
        }
    }
}

impl<'a> Iterator for RawPredictedItems<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        for is_present in &mut self.iter {
            let older_idx = self.idx;
            self.idx += 1;
            if is_present {
                return Some(Symbol::from(older_idx));
            }
        }
        None
    }
}

impl<'a, T> PredictionEvents<'a, T> {
    fn new<'b>(events: &'b [T], pred: row::Iter<'b>, origin: usize) -> PredictionEvents<'b, T> {
        PredictionEvents {
            iter: pred.zip(events.iter()),
            origin: origin,
        }
    }
}

impl<'a, T> Iterator for PredictionEvents<'a, T> {
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

impl<'a, T, L> MedialEvents<'a, T, L> {
    fn new<'b>(events: &'b [T], items: RawMedialItems<'b, L>) -> MedialEvents<'b, T, L> {
        MedialEvents {
            events: events,
            items: items,
        }
    }
}

impl<'a, T, L> Iterator for MedialEvents<'a, T, L> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let events = &self.events;
        self.items.next().map(|ei| {
            (&events[ei.dot as usize], ei.origin as usize)
        })
    }
}

impl<'a, L> Iterator for Events<'a, L> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(event_id, _), _origin) in &mut self.iter {
            if event_id.is_some() {
                return event_id.into();
            }
        }
        None
    }
}

impl<'a, L> Iterator for Distances<'a, L> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for (&(_, distance), _) in &mut self.iter {
            if distance.is_some() {
                return distance.into();
            }
        }
        None
    }
}

impl<'a, L> Iterator for Tracing<'a, L> {
    type Item = (ExternalDottedRule, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for (&external_dr_opt, origin) in &mut self.iter {
            if let Some(external_dotted_rule) = external_dr_opt {
                return Some((external_dotted_rule, origin));
            }
        }
        None
    }
}

impl<'a, N> Iterator for ExpectedTerminals<'a, N> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_scan_iter.next().map(|item| {
            self.rhs1[item.dot as usize].unwrap()
        })
    }
}

impl<'f, 'g, F> Recognizer<'f, 'g, F> where F: Forest<'f> + 'f {
    pub fn trace(&self) -> Tracing<F::NodeRef> {
        let trace = self.grammar().trace();
        let pred = PredictionEvents::new(trace[0],
                                         self.raw_predicted_items().iter,
                                         self.earleme());
        let medial = MedialEvents::new(trace[1], self.raw_medial_items());
        Tracing {
            iter: pred.chain(medial),
        }
    }

    pub fn events(&self) -> Events<F::NodeRef> {
        let (events_pred, events_flat) = self.grammar().events();
        let pred = PredictionEvents::new(events_pred,
                                         self.raw_predicted_items().iter,
                                         self.earleme());
        let items = MedialEvents::new(events_flat, self.raw_medial_items());
        Events {
            iter: pred.chain(items),
        }
    }

    pub fn minimal_distances(&self) -> Distances<F::NodeRef> {
        let (events_pred, events_flat) = self.grammar().events();
        let pred = PredictionEvents::new(events_pred,
                                         self.raw_predicted_items().iter,
                                         self.earleme());
        let items = MedialEvents::new(events_flat, self.raw_medial_items());
        Distances {
            iter: pred.chain(items),
        }
    }

    pub fn expected_terminals(&self) -> ExpectedTerminals<F::NodeRef> {
        ExpectedTerminals {
            prev_scan_iter: self.raw_medial_items(),
            rhs1: self.grammar().rules().rhs1,
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
    let mut pred = PredictionEvents::new(&ev[0..5],
                                         bit_m.iter_row(0),
                                         123);
    assert_eq!(pred.next(), Some((&2, 123)));
    assert_eq!(pred.next(), None);
}
