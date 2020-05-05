use std::cmp::Ordering;
use std::convert::TryInto;
use std::ops::Range;

use bit_matrix::{BitMatrix, row::BitVecSlice};
use cfg::*;

use events::{MedialItems, PredictedSymbols};
use forest::{Forest, NullForest};
use grammar::InternalGrammar;
use item::{CompletedItem, CompletedItemLinked, Item, Origin};
use policy::{PerformancePolicy, DefaultPerformancePolicy};

/// The recognizer implements the Earley algorithm. It parses the given input according
/// to the `grammar`. The `forest` is used to construct a parse result.
///
/// To save memory, it only retains those parts of the Earley table that may be useful
/// in the future.
pub struct Recognizer<'g, F = NullForest, P = DefaultPerformancePolicy>
where
    F: Forest,
    P: PerformancePolicy,
{
    // The forest.
    pub forest: F,
    // The grammar.
    pub grammar: &'g InternalGrammar<P>,
    // The policy.
    // policy: P,

    // Chart's items.

    // Predicted items are stored in a bit matrix. The bit matrix has a row for every Earley set.
    //
    // Length of `predicted` is earleme + 1, so that earleme points to the last
    pub(super) predicted: BitMatrix,

    // Medial items.
    //
    // N.B. This structure could be moved into its own module.
    pub(super) medial: Vec<Item<F::NodeRef, P>>,
    // Gearley's secret sauce: we have a binary heap for online sorting.
    //
    // Completed items are stored for the latest Earley set.
    // They are ordered by (origin, dot), starting with highest
    // origin and dot. The creation of a completed item can only be caused
    // by a scan or a completion of an item that has a higher (origin, dot)
    // pair value.
    pub(super) complete: Vec<CompletedItemLinked<F::NodeRef>>,

    // Chart's indices. They point to the beginning of each Earley set.
    //
    // Length of `indices` is `earleme` + 2, so that earleme points to
    // the beginning of the range of indices for the last range.
    pub(super) indices: Vec<usize>,
    // Index that points to the beginning of the latest set. Equivalent to
    // the last element of `indices`.
    pub(super) current_medial_start: usize,

    // The input location.
    pub(super) earleme: usize,

    pub(super) lookahead_hint: Option<Option<P::Symbol>>,
}

#[derive(Copy, Clone)]
pub(crate) enum Predicted<S: Into<Symbol>> {
    Any(S),
    Medial(S),
    Unary(S),
    Binary(S),
}

impl<S: Into<Symbol>> Predicted<S> {
    #[inline]
    pub(crate) fn usize(self) -> usize {
        match self {
            Predicted::Any(sym) => sym.into().usize() * 4,
            Predicted::Medial(sym) => sym.into().usize() * 4 + 1,
            Predicted::Unary(sym) => sym.into().usize() * 4 + 2,
            Predicted::Binary(sym) => sym.into().usize() * 4 + 3,
        }
    }

    #[inline]
    fn any(sym: S, row: &BitVecSlice) -> bool {
        // if predicted.small_slice(Predicted::Medial(trans.symbol).usize(), 3) != 0
        row[Predicted::Any(sym).usize()]
    }
}

impl<'g, F, P> Recognizer<'g, F, P>
where
    F: Forest,
    P: PerformancePolicy,
{
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn new(grammar: &'g InternalGrammar<P>, forest: F) -> Recognizer<'g, F, P> {
        // println!("new");
        let mut recognizer = Recognizer::empty(grammar, forest);
        recognizer.indices = Vec::with_capacity(64);
        recognizer.predicted = BitMatrix::new(8, grammar.predicted_row_size());
        recognizer.medial = Vec::with_capacity(256);
        recognizer.complete = Vec::with_capacity(32);
        recognizer.initialize();
        recognizer
    }

    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    #[inline]
    pub fn empty(grammar: &'g InternalGrammar<P>, forest: F) -> Recognizer<'g, F, P> {
        // println!("empty");
        Recognizer {
            forest,
            grammar,
            // The initial location is 0.
            earleme: 0,
            indices: Vec::new(),
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(0, grammar.predicted_row_size()),
            medial: Vec::new(),
            complete: Vec::new(),
            lookahead_hint: None,
        }
    }

    pub(super) fn initialize(&mut self) {
        // The first Earley set begins at 0 and ends at 0.
        self.indices.push(0);
        // The second Earley set begins at 0.
        self.indices.push(0);
        // println!("predict {:?}", self.grammar.start_sym().usize());
        self.predict(self.grammar.start_sym());
    }

    /// Makes the current Earley set predict a given symbol.
    pub fn predict(&mut self, symbol: P::Symbol) {
        // The source in the prediction matrix is the row that corresponds to the predicted symbol.
        let source = &self.grammar.prediction_matrix()[symbol.into().usize()];
        // The destination in `predicted` is the row that corresponds to the current location.
        let destination = &mut self.predicted[self.earleme];
        for (dst, src) in destination.iter_mut().zip(source.iter()) {
            *dst |= *src;
        }
    }

    pub fn begin_earleme(&mut self) {
        // nothing to do
    }

    /// Reads a token. Creates a leaf bocage node with the given value. After reading one or more
    /// tokens, the parse can be advanced.
    pub fn scan(&mut self, symbol: P::Symbol, value: F::LeafValue) {
        // This method is a part of the scan pass.
        if let Some(internal) = self.grammar.to_internal(symbol) {
            let earleme = self.earleme as Origin;
            // Add a leaf node to the forest with the given value.
            let node = self.forest.leaf(symbol.into(), earleme + 1, value);
            self.complete(earleme, internal, node);
        }
    }

    #[inline]
    pub fn lookahead_hint(&mut self, lookahead: Option<P::Symbol>) {
        let to_internal = |sym| self.grammar.to_internal(sym).unwrap();
        self.lookahead_hint = Some(lookahead.map(to_internal));
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
        if self.medial.len() - self.current_medial_start >= P::MEDIAL_SORT_THRESHOLD {
            self.sort_medial_items();
        }
        self.remove_unary_medial_items();
        self.remove_unreachable_sets();
        self.earleme += 1;
        // `earleme` is now at least 1.
        // Prediction pass.
        self.prediction_pass();
        // Store the index.
        self.current_medial_start = self.medial.len();
        self.indices.push(self.current_medial_start);
    }

    /// Checks whether the recognizer is exhausted. The recognizer is exhausted when it can't accept
    /// more input.
    #[inline]
    pub fn is_exhausted(&self) -> bool {
        self.medial.len() == self.current_medial_start && self.complete.is_empty()
    }

    /// Sorts medial items with deduplication.
    fn sort_medial_items(&mut self) {
        let grammar = &self.grammar;
        // Build index by postdot
        // These medial positions themselves are sorted by postdot symbol.
        self.medial[self.current_medial_start..].sort_unstable_by(|a, b| {
            (grammar.get_rhs1_cmp(a.dot), a.dot, a.origin).cmp(&(
                grammar.get_rhs1_cmp(b.dot),
                b.dot,
                b.origin,
            ))
        });
    }

    fn remove_unary_medial_items(&mut self) {
        while let Some(&item) = self.medial.last() {
            if self.grammar.get_rhs1(item.dot).is_some() {
                break;
            }
            self.medial.pop();
        }
    }

    fn remove_unreachable_sets(&mut self) {
        let origin = |item: &Item<F::NodeRef, P>| item.origin as usize;
        let max_origin = self.medial[self.current_medial_start..]
            .iter()
            .map(origin)
            .max()
            .unwrap_or(self.earleme);
        let diff = self.earleme - max_origin;
        if diff <= 1 {
            return;
        }
        // | 0 | 1 | 2 | 3 |
        //               ^ current_medial_start
        //   _________diff = 2
        //       ____drop = 1
        //           ^ self.earleme = 2
        //   ^ m = 0
        // | 0 | 1 | 2 |
        let drop = diff - 1;
        let new_medial_start = self.indices[self.indices.len() - 1 - drop];
        self.indices.truncate(self.indices.len() - drop);
        let current_medial_length = self.medial.len() - self.current_medial_start;
        for i in 0..current_medial_length {
            self.medial[new_medial_start as usize + i] = self.medial[self.current_medial_start + i];
        }
        self.medial
            .truncate(new_medial_start as usize + current_medial_length);
        self.current_medial_start = new_medial_start as usize;
        self.earleme -= drop;
        self.predicted.truncate(self.earleme + 1);
        for dst in self.predicted[self.earleme].iter_mut() {
            *dst = 0;
        }
    }

    /// Performs the prediction pass.
    fn prediction_pass(&mut self) {
        // Add a row to the matrix.
        self.predicted.grow(1, false);
        // Iterate through medial items in the current set.
        let iter = self.medial[self.current_medial_start..].iter();
        // For each medial item in the current set, predict its postdot symbol.
        let destination = &mut self.predicted[self.earleme];
        for ei in iter {
            let postdot = if let Some(rhs1) = self.grammar.get_rhs1(ei.dot) {
                rhs1
            } else {
                continue;
            };
            if !destination[Predicted::Medial(postdot).usize()] {
                // Prediction happens here. We would prefer to call `self.predict`, but we can't,
                // because `self.medial` is borrowed by `iter`.
                let source = &self.grammar.prediction_matrix()[postdot.into().usize()];
                for (dst, &src) in destination.iter_mut().zip(source.iter()) {
                    *dst |= src;
                }
            }
        }
    }

    /// Complete items.
    pub fn complete(&mut self, set_id: Origin, sym: P::Symbol, rhs_link: F::NodeRef) {
        debug_assert!(sym != self.grammar.eof());
        let predicted = unsafe {
            &*(&self.predicted[set_id as usize] as *const BitVecSlice)
        };
        let slice = predicted.small_slice_aligned(Predicted::Medial(sym).usize(), 3) as u8;
        // New completed item.
        // from A ::= B • C
        // to   A ::= B   C •
        if slice & 0b001 != 0 {
            self.complete_medial_items(set_id, sym, rhs_link);
        }
        // New item, either completed or pre-terminal. Ensure uniqueness.
        // from A ::= • B   c
        // to   A ::=   B • c
        if slice & 0b010 != 0 {
            self.complete_unary_predictions(set_id, sym, rhs_link, predicted);
        }
        if slice & 0b100 != 0 {
            self.complete_binary_predictions(set_id, sym, rhs_link, predicted);
        }
    }

    /// Complete medial items in a given Earley set.
    fn complete_medial_items(&mut self, set_id: Origin, sym: P::Symbol, rhs_link: F::NodeRef) {
        // Iterate through medial items to complete them.
        let (set_range, is_sorted) = self.medial_item_set_range(set_id, sym);
        if is_sorted {
            for idx in set_range {
                let dot = self.medial[idx].dot;
                if self.grammar.get_rhs1(dot) != Some(sym) {
                    break;
                }
                if !self.grammar.can_follow(self.grammar.get_lhs(dot), self.lookahead_hint) {
                    continue;
                }
                self.heap_push_linked(CompletedItemLinked {
                    idx: idx as u32,
                    node: Some(rhs_link),
                });
            }
        } else {
            for idx in set_range {
                let dot = self.medial[idx].dot;
                if self.grammar.get_rhs1(dot) != Some(sym) {
                    continue;
                }
                if !self.grammar.can_follow(self.grammar.get_lhs(dot), self.lookahead_hint) {
                    continue;
                }
                self.heap_push_linked(CompletedItemLinked {
                    idx: idx as u32,
                    node: Some(rhs_link),
                });
            }
        }
    }

    fn medial_item_set_range(&self, set_id: Origin, sym: P::Symbol) -> (Range<usize>, bool) {
        let outer_start = self.indices[set_id as usize];
        let outer_end = self.indices[set_id as usize + 1];
        let specific_set = &self.medial[outer_start..outer_end];

        if specific_set.len() >= P::MEDIAL_SORT_THRESHOLD {
            // When the set has X or more items, we use binary search to narrow down the range of
            // items.
            let set_idx = specific_set.binary_search_by(|ei| {
                (self.grammar.get_rhs1(ei.dot), Ordering::Greater).cmp(&(Some(sym), Ordering::Less))
            });
            match set_idx {
                Ok(idx) | Err(idx) => (outer_start + idx .. outer_end, true),
            }
        } else {
            // The beginning of the range points to items that have the same RHS1 symbol.
            (outer_start .. outer_end, false)
        }
    }

    // /// Complete predicted items that have a common postdot symbol.
    // fn complete_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
    // }

    /// Complete an item if predicted at rhs0.
    fn complete_unary_predictions(&mut self, set_id: Origin, sym: P::Symbol, rhs_link: F::NodeRef, predicted: &BitVecSlice) {
        for trans in self.grammar.unary_completions(sym) {
            if Predicted::any(trans.symbol, predicted) {
                // No checks for uniqueness, because `medial` will be deduplicated.
                // from A ::= • B
                // to   A ::=   B •
                // ---
                // We could push to `medial` as well and link from `complete` to `medial`.
                if !self.grammar.can_follow(self.grammar.get_lhs(trans.dot), self.lookahead_hint) {
                    continue;
                }
                self.heap_push(CompletedItem {
                    origin: set_id,
                    dot: trans.dot.into(),
                    left_node: rhs_link,
                    right_node: None,
                });
            }
        }
    }

    /// Complete an item if predicted at rhs1.
    fn complete_binary_predictions(&mut self, set_id: Origin, sym: P::Symbol, rhs_link: F::NodeRef, predicted: &BitVecSlice) {
        for trans in self.grammar.binary_completions(sym) {
            if Predicted::any(trans.symbol, predicted) {
                if !self.grammar.first(self.grammar.get_rhs1(trans.dot).unwrap(), self.lookahead_hint) {
                    continue;
                }
                // No checks for uniqueness, because `medial` will be deduplicated.
                // from A ::= • B   C
                // to   A ::=   B • C
                // Where C is terminal or nonterminal.

                self.medial.push(Item {
                    origin: set_id,
                    dot: trans.dot,
                    node: rhs_link,
                });
            }
        }
    }

    /// Resets the recognizer to its initial state by removing all contents.
    pub fn reset(&mut self) {
        self.earleme = 0;
        // Indices reset to [0, 0].
        self.indices.clear();
        // Current medial start reset to 0.
        self.current_medial_start = 0;
        // Remove items.
        self.medial.clear();
        self.complete.clear();
        self.initialize();
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
        if self.grammar.has_trivial_derivation() && self.earleme == 0 {
            Some(self.forest.nulling(self.grammar.externalized_start_sym().into()))
        } else {
            let has_dot_before_eof = |item: &&Item<_, _>| item.dot == self.grammar.dot_before_eof();
            let item_node = |item: &Item<_, _>| item.node;
            self.medial.last().filter(has_dot_before_eof).map(item_node)
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
    pub fn medial_items(&self) -> MedialItems<F::NodeRef, P> {
        let indices_len = self.indices.len();
        // Next-to-last index, which points to the beginning of the set before the current set.
        // The current set is empty.
        let items_start = self.indices[indices_len - 2];
        MedialItems {
            iter: self.medial[items_start..].iter(),
        }
    }

    // Accessors.

    /// Returns the current location number.
    pub fn earleme(&self) -> usize {
        self.earleme
    }

    // Completion

    /// Performs the completion pass.
    pub fn complete_all_sums_entirely(&mut self) {
        while let Some(mut completion) = self.next_sum() {
            // Include all items in the completion.
            completion.complete_entire_sum();
        }
        self.lookahead_hint = None;
    }

    /// Allows iteration through groups of completions that have unique symbol and origin.
    pub fn next_sum<'r>(&'r mut self) -> Option<CompleteSum<'g, 'r, F, P>> {
        if let Some(ei) = self.heap_peek() {
            let lhs_sym = self.grammar.get_lhs(ei.dot.try_into().ok().unwrap());
            Some(CompleteSum {
                origin: ei.origin,
                lhs_sym: lhs_sym.into(),
                recognizer: self,
            })
        } else {
            None
        }
    }
}

/// A group of completed items.
pub struct CompleteSum<'g, 'r, F, P>
where
    F: Forest,
    P: PerformancePolicy,
{
    /// The origin location of this completion.
    origin: Origin,
    /// The symbol of this completion.
    lhs_sym: Symbol,
    /// The recognizer.
    recognizer: &'r mut Recognizer<'g, F, P>,
}

impl<'g, 'r, F, P> CompleteSum<'g, 'r, F, P>
where
    F: Forest,
    P: PerformancePolicy,
    'g: 'r,
{
    /// Completes all items.
    pub fn complete_entire_sum(&mut self) {
        self.recognizer.forest.begin_sum();
        // For each item, include it in the completion.
        while let Some(item) = self.next_summand() {
            self.push_summand(item);
        }
        // Use all items for completion.
        self.complete_sum();
    }

    /// Skips all items.
    pub fn skip_entire_sum(&mut self) {
        // For each item, include it in the completion.
        while let Some(_) = self.next_summand() {}
    }

    /// Allows iteration through completed items.
    #[inline]
    pub fn next_summand(&mut self) -> Option<CompletedItem<F::NodeRef>> {
        if let Some(completion) = self.recognizer.heap_peek() {
            let completion_lhs_sym = self.recognizer.grammar.get_lhs(completion.dot.try_into().ok().unwrap());
            if self.origin == completion.origin && self.lhs_sym == completion_lhs_sym.into() {
                self.recognizer.heap_pop();
                Some(completion)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Includes an item in the completion.
    #[inline]
    pub fn push_summand(&mut self, completed_item: CompletedItem<F::NodeRef>) {
        self.recognizer.forest.push_summand(completed_item);
    }

    /// Uses the completion to complete items in the recognizer.
    #[inline]
    pub fn complete_sum(&mut self) -> F::NodeRef {
        let node = self.recognizer.forest.sum(self.lhs_sym, self.origin);
        self.recognizer.complete(self.origin, self.lhs_sym.into(), node);
        node
    }

    /// Returns the origin location of this completion.
    #[inline]
    pub fn origin(&self) -> Origin {
        self.origin
    }

    /// Returns the symbol of this completion.
    #[inline]
    pub fn symbol(&self) -> Symbol {
        self.lhs_sym
    }
}
