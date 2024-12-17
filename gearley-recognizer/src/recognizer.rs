use std::marker::PhantomData;

use bit_matrix::BitMatrix;

use gearley_forest::{Forest, NullForest};
use gearley_grammar::Grammar;
use crate::item::{Item, CompletedItemLinked, Origin};
use gearley_vec2d::Vec2d;

use crate::event::{MedialItems, PredictedSymbols};
use crate::local_prelude::*;
use crate::predict::Predict;

use crate::{binary_heap::BinaryHeap, lookahead::{DefaultLookahead, Lookahead}};

/// The recognizer implements the Earley algorithm. It parses the given input according
/// to the `grammar`. The parse result is constructed inside the `forest`.
///
/// To save memory, it only retains those parts of the Earley table that may be useful
/// in the future.
pub struct Recognizer<G, F = NullForest, P = DefaultPerfHint>
where
    F: Forest<G::Symbol>,
    G: Grammar,
{
    // The grammar.
    pub(crate) grammar: G,
    // The forest.
    pub(crate) forest: F,
    // Lookahead.
    pub(crate) lookahead: DefaultLookahead<G::Symbol>,
    // The policy.
    policy: PhantomData<P>,

    // Chart's items.

    // Predicted items are stored in a bit matrix. The bit matrix has a row for every Earley set.
    //
    // Length of `predicted` is earleme + 1, so that earleme points to the last
    pub(crate) predicted: BitMatrix,

    // Medial items, charted, and chart indices.
    //
    // Vec of medial items stored as a flat dynamic array of medial items.
    //
    // Vec chart's indices stored as a flat dynamic array of indices.
    // They mark the beginning of each Earley set in the array of medial items.
    //
    // Length of `indices` is `earleme` + 2, so that the index at earleme points to
    // the beginning of the range of indices for the last range.
    //
    // Has the index that points to the beginning of the latest set in the chart.
    // Equivalent to the last element of `indices`.
    pub(crate) medial: Vec2d<Item<F::NodeRef>>,
    // Gearley's secret sauce: we have a binary heap for online sorting.
    //
    // Completed items are stored for the latest Earley set.
    
    // They are ordered by (origin, dot), starting with highest
    // origin and dot. The creation of a completed item can only be caused
    // by a scan or a completion of an item that has a higher (origin, dot)
    // pair value.
    pub(crate) complete: BinaryHeap<CompletedItemLinked<F::NodeRef>>,
}

impl<F, G, P> Recognizer<G, F, P>
where
    F: Forest<G::Symbol>,
    G: Grammar,
    P: PerfHint,
{
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn new(grammar: G) -> Recognizer<G, F, P> where F: Default, P: Default {
        Self::with_forest_and_policy(grammar, F::default(), P::default())
    }

    pub fn with_forest(grammar: G, forest: F) -> Recognizer<G, F, P> where P: Default {
        Self::with_forest_and_policy(grammar, forest, P::default())
    }

    pub fn with_forest_and_policy(grammar: G, forest: F, policy: P) -> Recognizer<G, F, P> {
        // Reserve the right capacity for vectors.
        let mut recognizer = Recognizer {
            medial: Vec2d::with_capacity(policy.medial_capacity()),
            predicted: BitMatrix::new(8, grammar.num_syms()),
            complete: BinaryHeap(Vec::with_capacity(policy.completion_capacity())),
            lookahead: DefaultLookahead::new(&grammar),
            grammar,
            forest,
            policy: PhantomData,
        };
        recognizer.medial.next_set();
        recognizer.predict(recognizer.grammar.start_sym());
        recognizer
        // TODO: symbols start_of_input and end_of_input first set scan start_of_input
    }

    pub fn begin_earleme(&mut self) {
        // nothing to do
    }

    /// Reads a token. Creates a leaf bocage node with the given value. After reading one or more
    /// tokens, the parse can be advanced.
    pub fn scan(&mut self, symbol: G::Symbol, value: F::LeafValue) {
        // This method is a part of the scan pass.
        let earleme = self.earleme() as Origin;
        // Add a leaf node to the forest with the given value.
        let node = self.forest.leaf(symbol, earleme + 1, value);
        self.complete(earleme, symbol, node);
    }

    #[inline]
    pub fn lookahead(&mut self) -> impl Lookahead<G::Symbol> + '_ {
        self.lookahead.mut_with_grammar(&self.grammar)
    }

    /// Advances the parse. Calling this method may set the finished node, which can be accessed
    /// through the `finished_node` method.
    pub fn end_earleme(&mut self) -> bool {
        if self.is_exhausted() {
            false
        } else {
            // Completion pass, which saves successful parses.
            self.complete_all_sums_entirely();
            // Do the rest.
            self.advance_without_completion();
            true
        }
    }

    /// Advances the parse. Omits the completion pass, which should be done through
    /// the `completions` method. Keep in mind that calling this method may not set
    /// the finished node, which should be tracked externally.
    pub fn advance_without_completion(&mut self) {
        self.sort_medial_items();
        self.remove_unary_medial_items();
        self.remove_unreachable_sets();
        // `earleme` is now at least 1.
        // Prediction pass.
        self.prediction_pass();
        // Medial processing.
        self.medial.next_set();
    }

    /// Checks whether the recognizer is exhausted. The recognizer is exhausted when it can't accept
    /// more input.
    #[inline]
    pub fn is_exhausted(&self) -> bool {
        self.medial.last().is_empty() && self.complete.is_empty()
    }

    /// Sorts medial items with deduplication.
    fn sort_medial_items(&mut self) {
        let grammar = &self.grammar;
        // Build index by postdot
        // These medial positions themselves are sorted by postdot symbol.
        self.medial.last_mut().sort_unstable_by(|a: &Item<<F as Forest<G::Symbol>>::NodeRef>, b| {
            (grammar.get_rhs1_cmp(a.dot), a.dot, a.origin).cmp(&(
                grammar.get_rhs1_cmp(b.dot),
                b.dot,
                b.origin,
            ))
        });
    }

    fn remove_unary_medial_items(&mut self) {
        while let Some(&item) = self.medial.last_item() {
            if self.grammar.get_rhs1(item.dot).is_some() {
                break;
            }
            self.medial.pop_item();
        }
    }

    fn remove_unreachable_sets(&mut self) {
        let origin = |item: &Item<F::NodeRef>| item.origin as usize;
        let max_origin = self.medial.last()
            .iter()
            .map(origin)
            .max()
            .unwrap_or(self.earleme());
        let new_earleme = max_origin + 1;
        if self.earleme() > new_earleme {
            // | 0 | 1 | 2 | 3 |
            //               ^ current_medial_start
            //   _________diff = 2
            //       ____drop = 1
            //           ^ self.earleme = 2
            //   ^ m = 0
            // | 0 | 1 | 2 |
            self.predicted[new_earleme].clear();
            self.predicted.truncate(new_earleme);
            self.medial.truncate(new_earleme);
        }
    }

    /// Performs the prediction pass.
    fn prediction_pass(&mut self) {
        // Add a row to the matrix.
        self.predicted.grow(1, false);
        // Iterate through medial items in the current set.
        let iter = self.medial.last().iter();
        // For each medial item in the current set, predict its postdot symbol.
        let earleme = self.earleme();
        let row = &mut self.predicted[earleme];
        for ei in iter {
            let postdot = self.grammar.get_rhs1(ei.dot).unwrap();
            row.predict(postdot, self.grammar.prediction_row(postdot));
        }
    }

    /// Resets the recognizer to its initial state by removing all contents.
    pub fn reset(&mut self) {
        self.predict(self.grammar.start_sym());
        // Remove items.
        self.medial.clear();
        self.medial.next_set();
        self.complete.clear();
    }

    // Finished node access.

    /// Checks whether there is a valid parse that ends at the current
    /// position.
    pub fn is_finished(&self) -> bool {
        self.finished_node().is_some()
    }

    /// Retrieves the bocage node that represents the parse that has finished at the current
    /// location.
    ///
    /// # Panics
    ///
    /// Panics when the parse has not finished at the current location.
    pub fn finished_node(&self) -> Option<F::NodeRef> {
        if self.grammar.has_trivial_derivation() && self.earleme() == 0 {
            Some(self.forest.nulling(self.grammar.externalized_start_sym()))
        } else {
            let has_dot_before_eof = |item: &&Item<_>| item.dot == self.grammar.dot_before_eof();
            let item_node = |item: &Item<_>| item.node;
            self.medial.last_item().filter(has_dot_before_eof).map(item_node)
        }
    }

    // Event access.

    /// Accesses predicted symbols.
    pub fn predicted_symbols(&self) -> PredictedSymbols {
        let earleme = self.earleme();
        PredictedSymbols {
            iter: self.predicted.iter_row(earleme),
            idx: 0,
        }
    }

    /// Accesses medial items.
    pub fn medial_items(&self) -> MedialItems<F::NodeRef> {
        MedialItems {
            iter: self.medial[self.earleme()].iter(),
        }
    }

    // Accessors.

    /// Returns the current location number.
    pub fn earleme(&self) -> usize {
        self.medial.len() - 1
    }

    pub fn into_forest(self) -> F {
        self.forest
    }
}
