use std::cmp::Ordering;
use std::ops::Range;

use bit_matrix::BitMatrix;
use bit_matrix::row::BitVecSlice;
use cfg::*;

use events::{MedialItems, PredictedSymbols};
use forest::{Forest, NullForest};
use grammar::InternalGrammar;
use item::{CompletedItem, CompletedItemLinked, Item, Origin};
// use policy::{PerformancePolicy, NullPerformancePolicy};

/// The recognizer implements the Earley algorithm. It parses the given input according
/// to the `grammar`. The parse result is constructed inside the `forest`.
///
/// To save memory, it only retains those parts of the Earley table that may be useful
/// in the future.
pub struct Recognizer<'g, F = NullForest>
where
    F: Forest,
{
    // The forest.
    pub forest: F,
    // The grammar.
    pub grammar: &'g InternalGrammar,
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
    pub(super) medial: Vec<Item<F::NodeRef>>,
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

    pub(super) lookahead_hint: Option<Option<Symbol>>,
}

impl<'g, F> Recognizer<'g, F>
where
    F: Forest,
{
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn new(grammar: &'g InternalGrammar, forest: F) -> Recognizer<'g, F> {
        let mut recognizer = Recognizer {
            forest,
            grammar,
            // The initial location is 0.
            earleme: 0,
            // The first Earley set begins at 0 and ends at 0. The second Earley set begins at 0.
            indices: vec![0, 0],
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(8, grammar.num_syms()),
            medial: Vec::with_capacity(256),
            complete: Vec::with_capacity(32),
            lookahead_hint: None,
        };
        recognizer.predict(grammar.start_sym());
        recognizer
    }

    /// Makes the current Earley set predict a given symbol.
    pub fn predict(&mut self, symbol: Symbol) {
        self.predicted[self.earleme].predict(symbol, self.grammar.predict(symbol));
    }

    pub fn begin_earleme(&mut self) {
        // nothing to do
    }

    /// Reads a token. Creates a leaf bocage node with the given value. After reading one or more
    /// tokens, the parse can be advanced.
    pub fn scan(&mut self, symbol: Symbol, value: F::LeafValue) {
        // This method is a part of the scan pass.
        if let Some(internal) = self.grammar.to_internal(symbol) {
            let earleme = self.earleme as Origin;
            // Add a leaf node to the forest with the given value.
            let node = self.forest.leaf(symbol, earleme + 1, value);
            self.complete(earleme, internal, node);
        }
    }

    #[inline]
    pub fn lookahead_hint(&mut self, lookahead: Option<Symbol>) {
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
        self.sort_medial_items();
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
        let origin = |item: &Item<F::NodeRef>| item.origin as usize;
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
        let row = &mut self.predicted[self.earleme];
        for ei in iter {
            let postdot = self.grammar.get_rhs1(ei.dot).unwrap();
            row.predict(postdot, self.grammar.predict(postdot));
        }
    }

    /// Complete items.
    pub fn complete(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        debug_assert!(sym != self.grammar.eof());
        if self.predicted[set_id as usize].get(sym.usize()) {
            self.complete_medial_items(set_id, sym, rhs_link);
            self.complete_predictions(set_id, sym, rhs_link);
        }
    }

    /// Complete medial items in a given Earley set.
    fn complete_medial_items(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        // Iterate through medial items to complete them.
        let set_range = self.medial_item_set_range(set_id, sym);
        if let Some(hint) = self.lookahead_hint {
            for idx in set_range {
                // New completed item.
                // from A ::= B • C
                // to   A ::= B   C •
                //
                // We might link to medial items by index, here.
                let dot = self.medial[idx].dot;
                if !self.grammar.can_follow(self.grammar.get_lhs(dot), hint) {
                    continue;
                }
                self.heap_push_linked(CompletedItemLinked {
                    idx: idx as u32,
                    node: Some(rhs_link),
                });
            }
        } else {
            for idx in set_range {
                // New completed item.
                // from A ::= B • C
                // to   A ::= B   C •
                //
                // We might link to medial items by index, here.
                self.heap_push_linked(CompletedItemLinked {
                    idx: idx as u32,
                    node: Some(rhs_link),
                });
            }
        }
    }

    fn medial_item_set_range(&mut self, set_id: Origin, sym: Symbol) -> Range<usize> {
        // Huh, can we reduce complexity here?
        let outer_start = self.indices[set_id as usize];
        let outer_end = self.indices[set_id as usize + 1];
        let specific_set = &self.medial[outer_start..outer_end];

        let inner_start = if specific_set.len() >= 16 {
            // When the set has 16 or more items, we use binary search to narrow down the range of
            // items.
            let set_idx = specific_set.binary_search_by(|ei| {
                (self.grammar.get_rhs1(ei.dot), Ordering::Greater).cmp(&(Some(sym), Ordering::Less))
            });
            match set_idx {
                Ok(idx) | Err(idx) => idx,
            }
        } else {
            specific_set
                .iter()
                .take_while(|ei| self.grammar.get_rhs1(ei.dot).unwrap() < sym)
                .count()
        };

        // The range contains items that have the same RHS1 symbol.
        let inner_end = specific_set[inner_start..]
            .iter()
            .take_while(|ei| self.grammar.get_rhs1(ei.dot) == Some(sym))
            .count();
        outer_start + inner_start..outer_start + inner_start + inner_end
    }

    /// Complete predicted items that have a common postdot symbol.
    fn complete_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        // New item, either completed or pre-terminal. Ensure uniqueness.
        // from A ::= • B   c
        // to   A ::=   B • c
        self.complete_unary_predictions(set_id, sym, rhs_link);
        self.complete_binary_predictions(set_id, sym, rhs_link);
    }

    /// Complete an item if predicted at rhs0.
    fn complete_unary_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        for trans in self.grammar.unary_completions(sym) {
            if self.predicted[set_id as usize].get(trans.symbol.usize()) {
                // No checks for uniqueness, because `medial` will be deduplicated.
                // from A ::= • B
                // to   A ::=   B •
                // ---
                // We could push to `medial` as well and link from `complete` to `medial`.

                if let Some(hint) = self.lookahead_hint {
                    if !self
                        .grammar
                        .can_follow(self.grammar.get_lhs(trans.dot), hint)
                    {
                        continue;
                    }
                }
                self.heap_push(CompletedItem {
                    origin: set_id,
                    dot: trans.dot,
                    left_node: rhs_link,
                    right_node: None,
                });
            }
        }
    }

    /// Complete an item if predicted at rhs1.
    fn complete_binary_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        for trans in self.grammar.binary_completions(sym) {
            if self.predicted[set_id as usize].get(trans.symbol.usize()) {
                if let Some(hint) = self.lookahead_hint {
                    if !self
                        .grammar
                        .first(self.grammar.get_rhs1(trans.dot).unwrap(), hint)
                    {
                        continue;
                    }
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
        self.predict(self.grammar.start_sym());
        // Indices reset to [0, 0].
        self.indices.clear();
        self.indices.push(0);
        self.indices.push(0);
        // Current medial start reset to 0.
        self.current_medial_start = 0;
        // Remove items.
        self.medial.clear();
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
        if self.grammar.has_trivial_derivation() && self.earleme == 0 {
            Some(self.forest.nulling(self.grammar.externalized_start_sym()))
        } else {
            let has_dot_before_eof = |item: &&Item<_>| item.dot == self.grammar.dot_before_eof();
            let item_node = |item: &Item<_>| item.node;
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
    pub fn medial_items(&self) -> MedialItems<F::NodeRef> {
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
    pub fn next_sum<'r>(&'r mut self) -> Option<CompleteSum<'g, 'r, F>> {
        if let Some(ei) = self.heap_peek() {
            let lhs_sym = self.grammar.get_lhs(ei.dot);
            Some(CompleteSum {
                origin: ei.origin,
                lhs_sym,
                recognizer: self,
            })
        } else {
            None
        }
    }
}

/// A group of completed items.
pub struct CompleteSum<'g, 'r, F>
where
    F: Forest,
{
    /// The origin location of this completion.
    origin: Origin,
    /// The symbol of this completion.
    lhs_sym: Symbol,
    /// The recognizer.
    recognizer: &'r mut Recognizer<'g, F>,
}

impl<'g, 'r, F> CompleteSum<'g, 'r, F>
where
    F: Forest,
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
            let completion_lhs_sym = self.recognizer.grammar.get_lhs(completion.dot);
            if self.origin == completion.origin && self.lhs_sym == completion_lhs_sym {
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
        self.recognizer.complete(self.origin, self.lhs_sym, node);
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

trait Predict {
    fn predict(&mut self, sym: Symbol, source: &BitVecSlice);
}

impl Predict for BitVecSlice {
    fn predict(&mut self, sym: Symbol, source: &BitVecSlice) {
        if !self[sym.usize()] {
            // The source in the prediction matrix is the row that corresponds to the predicted
            // symbol.
            //
            // The destination in `predicted` is now the `self` that corresponds to the current
            // location.
            for (dst, &src) in self.iter_mut().zip(source.iter()) {
                *dst |= src;
            }
        }
    }
}
