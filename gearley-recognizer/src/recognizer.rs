use std::cmp;
use std::marker::PhantomData;

use bit_matrix::BitMatrix;

use cfg_symbol::{Symbol, SymbolSource};
use gearley_forest::completed_item::CompletedItem;
use gearley_forest::{Forest, NullForest};
use gearley_grammar::Grammar;
use crate::item::{Item, CompletedItemLinked, Origin};
use gearley_vec2d::Vec2d;

use crate::local_prelude::*;
use crate::predict::Predict;

use crate::{binary_heap::BinaryHeap, lookahead::{DefaultLookahead, Lookahead}};

#[cfg(feature = "log")]
use log::trace;
#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($tt:tt)*) => {};
}

/// The recognizer implements the Earley algorithm. It parses the given input according
/// to the `grammar`. The parse result is constructed inside the `forest`.
///
/// To save memory, it only retains those parts of the Earley table that may be useful
/// in the future.
pub struct Recognizer<G, F = NullForest, P = DefaultPerfHint>
where
    F: Forest,
    G: Grammar,
{
    // The grammar.
    pub(crate) grammar: G,
    // The forest.
    pub(crate) forest: F,
    // Lookahead.
    pub(crate) lookahead: DefaultLookahead,
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

impl<G> Recognizer<G, NullForest, DefaultPerfHint>
where
    G: Grammar,
{
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn new(grammar: G) -> Recognizer<G, NullForest, DefaultPerfHint> {
        Recognizer::with_forest_and_policy(grammar, NullForest, DefaultPerfHint::new(8192))
    }
}

impl<F, G> Recognizer<G, F, DefaultPerfHint>
where
    F: Forest,
    G: Grammar,
{
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn with_forest(grammar: G, forest: F) -> Recognizer<G, F, DefaultPerfHint> {
        Recognizer::with_forest_and_policy(grammar, forest, DefaultPerfHint::new(8192))
    }
}

impl<F, G, P> Recognizer<G, F, P>
where
    F: Forest,
    G: Grammar,
    P: PerfHint,
{
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
        recognizer.predicted[0].predict(recognizer.grammar.start_sym(), recognizer.grammar.prediction_row(recognizer.grammar.start_sym()));
        recognizer
        // TODO: symbols start_of_input and end_of_input first set scan start_of_input
    }

    pub fn begin_earleme(&mut self) {
        self.medial.next_set();
        let earleme = self.earleme();
        let rows = format!("{:?}", self.predicted.sub_matrix(earleme .. earleme + 1));
        trace!("recognizer.predicted: BitSubMatrix {{ row: {:?} }}", rows.replace('\n', " "));
        // from now on, the `earleme` points to the last fully done set
        // ----------
        // new
        // earleme == -1 ?
        // begin_earleme
        // earleme == 0
        // medial.indices.len() == 2
        // predicted[0] = predict(start_sym)
        // 
    }

    /// Reads an internal token. Creates a leaf bocage node with the given value. After reading one or more
    /// tokens, the parse can be advanced.
    pub fn scan(&mut self, symbol: Symbol, value: F::LeafValue) {
        // This method is a part of the scan pass.
        let earleme = self.earleme() as Origin;
        // Add a leaf node to the forest with the given value.
        let trace_value = format!("{:?}", value);
        let node = self.forest.leaf(symbol, earleme + 1, value);
        trace!("recognizer.scan: Scan {{ symbol: {:?}, node: {:?}, value: {} }}", symbol, node, trace_value);
        self.complete(earleme, symbol, node);
    }

    #[inline]
    pub fn lookahead(&mut self) -> impl Lookahead + '_ {
        self.lookahead.mut_with_grammar(&self.grammar)
    }

    /// Advances the parse. Calling this method may set the finished node, which can be accessed
    /// through the `finished_node` method.
    pub fn end_earleme(&mut self) -> bool {
        if self.is_exhausted() {
            false
        } else {
            trace!("recognizer.completions_after_scan: {:?}", self.complete);
            // Completion pass, which saves successful parses.
            self.complete_all_sums_entirely();
            trace!("recognizer.medial_after_completion: {:?}", self.medial.last());
            // Do the rest.
            self.advance_after_completion();
            trace!("recognizer.prediction_result: {:?}", self.predicted.sub_matrix(self.earleme() .. self.earleme() + 1));
            true
        }
    }

    /// Advances the parse. Omits the completion pass, which should be done through
    /// the `completions` method. Keep in mind that calling this method may not set
    /// the finished node, which should be tracked externally.
    pub fn advance_after_completion(&mut self) {
        self.sort_medial_items();
        self.remove_unary_medial_items();
        // self.remove_unreachable_sets();
        trace!("recognizer.medial: Vec {:?}", self.medial.last());
        // `earleme` is now at least 1.
        // Prediction pass.
        self.prediction_pass();
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
        self.medial.last_mut().sort_unstable_by_key(|item: &Item<<F as Forest>::NodeRef>| {
            (grammar.get_rhs1_cmp(item.dot), item.dot, item.origin)
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
        let new_earleme = max_origin;
        if self.earleme() > new_earleme {
            trace!("remove_unreachable_sets {:?} > {:?}", self.earleme(), new_earleme);
            // ------------------------------
            //         earleme = 0
            // | A B | C D
            //   P0
            //
            // new_earleme = 0 - do nothing
            // ------------------------------
            //               earleme = 2
            // | A B | C D | E F | G H
            //   P0    P1    P3
            //
            // max_origin = 1
            // new_earleme = 1
            // copy from last to 2
            // truncate to 3
            // | A B | C D | G H
            //   P0    P1
            //                    earleme = 2
            // | A B | C D | G H |
            //   P0    P1    Px
            // ------------------------------
            // let (mut body, mut tail) = self.predicted.split_at_mut(self.earleme());
            // for (dst, src) in body[new_earleme].iter_blocks_mut().zip(tail[0].iter_blocks_mut()) {
            //     *dst = *src;
            // }
            // self.predicted[new_earleme].clear()
            self.predicted.truncate(new_earleme + 1);
            self.medial.truncate(new_earleme + 1);
            debug_assert_eq!(self.medial.len(), new_earleme + 2);
            debug_assert_eq!(self.earleme(), new_earleme);
            // earleme == new_earleme - 2
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
        trace!("recognizer.earleme: {}, rows_and_cols: {:?}", earleme, self.predicted.size());
        let row = &mut self.predicted[earleme + 1];
        for ei in iter {
            let postdot = self.grammar.get_rhs1(ei.dot).unwrap();
            row.predict(postdot, self.grammar.prediction_row(postdot));
        }
    }

    /// Resets the recognizer to its initial state by removing all contents.
    pub fn reset(&mut self) {
        // Remove items.
        self.medial.clear();
        self.complete.clear();
        // Earleme is now equal 0.
        // Reset predictions.
        self.predicted[0].clear();
        self.predicted[0].predict(self.grammar.start_sym(), self.grammar.prediction_row(self.grammar.start_sym()));

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
            self.medial.last().first().filter(has_dot_before_eof).map(item_node)
        }
    }

    // Event access.

    /// Accesses predicted symbols.
    pub fn predicted_symbols(&self) -> impl Iterator<Item = Symbol> + use<'_, F, G, P> {
        let earleme = self.earleme();
        self.predicted.iter_row(earleme).zip(SymbolSource::generate_fresh()).filter_map(|(is_present, sym)| if is_present { Some(sym) } else { None })
    }

    /// Accesses medial items.
    pub fn medial_items(&self) -> impl Iterator<Item = Item<F::NodeRef>> + use<'_, F, G, P> {
        self.medial[self.earleme()].iter().copied()
    }

    // Accessors.

    /// Returns the current location number.
    pub fn earleme(&self) -> usize {
        self.medial.len().saturating_sub(2)
    }

    pub fn into_forest(self) -> F {
        self.forest
    }

    pub fn grammar(&self) -> &G {
        &self.grammar
    }
}

/// A set of completed items with all having a common triple **(Symbol; start input location ..
/// end input location)**, varying only in their rule ID.
pub struct CompleteGroup<'r, F, G, P>
where
    F: Forest,
    G: Grammar,
    P: PerfHint,
{
    /// The **start input location** of this completion.
    origin: Origin,
    /// The **Symbol** of this completion.
    lhs_sym: Symbol,
    /// The recognizer.
    recognizer: &'r mut Recognizer<G, F, P>,
}

impl<G, F, P> Recognizer<G, F, P>
    where F: Forest,
    G: Grammar,
    P: PerfHint,
{
    /// Complete items.
    pub fn complete(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        // debug_assert!(sym != self.grammar.eof());
        trace!("complete_predicted: {:?}", self.predicted.sub_matrix(set_id as usize .. set_id as usize + 1));
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Complete { set_id: Origin, sym: Symbol }
        trace!("recognizer.complete: {:?}", Complete { set_id, sym });
        if sym.usize() >= self.grammar.num_syms() {
            // New item after a generated symbol, either completed or medial.
            // from A ::= • g42   c
            // to   A ::=   g42 • c
            self.complete_generated_binary_predictions(set_id, sym, rhs_link);
        } else if self.predicted[set_id as usize].get(sym.usize()) {
            // New item, either completed or medial.
            // from A ::=   B • C
            // to   A ::=   B   C •
            self.complete_medial_items(set_id, sym, rhs_link);
            // New item, either completed or medial.
            // from A ::= • B   c
            // to   A ::=   B • c
            self.complete_predictions(set_id, sym, rhs_link);
        }
    }

    /// Complete medial items in a given Earley set.
    fn complete_medial_items(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        // Iterate through medial items to complete them.
        // Huh, can we reduce complexity here?
        // let outer_start = self.medial.indices()[set_id as usize];
        // let outer_end: usize = self.medial.indices()[set_id as usize + 1];
        let specific_set = &self.medial[set_id as usize];
        trace!("complete_specific_set: {:?}", specific_set);

        // When the set has 16 or more items, we use binary search to narrow down the range of
        // items.
        // todo branchless binary search
        let set_idx = specific_set.binary_search_by(|ei| {
            (self.grammar.get_rhs1(ei.dot), cmp::Ordering::Greater).cmp(&(Some(sym), cmp::Ordering::Less))
        });
        let inner_start = match set_idx {
            Ok(idx) | Err(idx) => idx,
        };

        // The range contains items that have the same RHS1 symbol.
        let inner_end = specific_set[inner_start..]
            .iter()
            .take_while(|ei| self.grammar.get_rhs1(ei.dot) == Some(sym))
            .count();
        let start: u32 = self.medial.index_at(set_id as usize) as u32;
        trace!("complete_inner: {:?}", &self.medial[set_id as usize][inner_start .. inner_start + inner_end]);
        for idx in inner_start .. inner_start + inner_end {
            // New completed item.
            // from A ::= B • C
            // to   A ::= B   C •
            let dot = self.medial[set_id as usize][idx].dot;
            // let will_be_useful = self.lookahead.mut_with_grammar(&self.grammar).sym().map_or(true, |sym| self.grammar.lr_set(dot)[sym.usize()]);
            let will_be_useful = true;
            trace!("dot: {:?}", dot);
            trace!("will_be_useful: {:?}", will_be_useful);
            if will_be_useful {
                self.complete.heap_push_linked(CompletedItemLinked {
                    idx: start + idx as u32,
                    node: Some(rhs_link),
                }, &mut self.medial);
            }
        }
    }

    /// Complete predicted items that have a common postdot symbol.
    fn complete_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        let mut unary: u32 = 0;
        for trans in self.grammar.completions(sym) {
            let was_predicted = self.predicted[set_id as usize].get(trans.symbol.usize());
            let will_be_useful = true;//self.lookahead.mut_with_grammar(&self.grammar).sym().map_or(true, |sym| self.grammar.lr_set(trans.dot)[sym.usize()]);
            if was_predicted && will_be_useful {
                // No checks for uniqueness, because completions are deduplicated.
                // --- UNARY
                // from A ::= • B
                // to   A ::=   B •
                // --- BINARY
                // from A ::= • B   C
                // to   A ::=   B • C
                // Where C is terminal or nonterminal.
                trace!("recognizer.new_medial_item: Item {{ origin: {}, dot: {} }}", set_id, trans.dot);
                self.medial.push_item(Item {
                    origin: set_id,
                    dot: trans.dot,
                    node: rhs_link,
                });
                unary += trans.is_unary as u32;
            }
        }
        for idx in self.medial.item_count() as u32 - unary .. self.medial.item_count() as u32 {
            self.complete.heap_push_linked(CompletedItemLinked { idx, node: None }, &self.medial)
        }
    }

    /// Attempt to complete a predicted item with a postdot gensym.
    fn complete_generated_binary_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        let trans = self.grammar.gen_completion(sym);
        let was_predicted = self.predicted[set_id as usize].get(trans.symbol.usize());
        // let will_be_useful = self.lookahead.mut_with_grammar(&self.grammar).sym().map_or(true, |sym| self.grammar.lr_set(trans.dot)[sym.usize()]);
        if was_predicted {
            // No checks for uniqueness, because completions are deduplicated.
            // --- UNARY
            // from A ::= • g42
            // to   A ::=   g42 •
            // --- BINARY
            // from A ::= • g42   C
            // to   A ::=   g42 • C
            // Where g42 is a gensym, and C is terminal or nonterminal.
            trace!("recognizer.new_medial_item: Item {{ origin: {}, dot: {} }}", set_id, trans.dot);
            self.medial.push_item(Item {
                origin: set_id,
                dot: trans.dot,
                node: rhs_link,
            });
            if trans.is_unary {
                self.complete.heap_push_linked(CompletedItemLinked { idx: self.medial.item_count() as u32 - 1, node: None }, &mut self.medial);
            }
        }
    }

    // Completion

    /// Performs the completion pass.
    pub fn complete_all_sums_entirely(&mut self) {
        while let Some(mut completion) = self.next_sum() {
            // Include all items in the completion.
            completion.complete_entire_sum();
        }
        self.lookahead.mut_with_grammar(&self.grammar).clear_hint();
    }

    /// Allows iteration through groups of completions that have unique symbol and origin.
    pub fn next_sum<'r>(&'r mut self) -> Option<CompleteGroup<'r, F, G, P>> {
        if let Some(ei) = self.heap_peek() {
            let lhs_sym = self.grammar.get_lhs(ei.dot);
            Some(CompleteGroup {
                origin: ei.origin,
                lhs_sym,
                recognizer: self,
            })
        } else {
            None
        }
    }
}

impl<'r, F, G, P> CompleteGroup<'r, F, G, P>
where
    F: Forest,
    G: Grammar,
    P: PerfHint,
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
