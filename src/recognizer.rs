use std::cmp::Ordering;

use bit_matrix::BitMatrix;
use cfg::*;

use events::{PredictedSymbols, MedialItems};
use forest::{Forest, NodeBuilder, NullForest};
use grammar::{InternalGrammar, DotKind};
use item::{CompletedItem, Item, Origin};
use util::array::sort_and_dedup;
use util::binary_heap::BinaryHeap;

/// The recognizer implements the Earley algorithm. It parses the given input according
/// to the `grammar`. The `forest` is used to construct a parse result.
///
/// To save memory, it only retains those parts of the Earley table that may be useful
/// in the future.
pub struct Recognizer<'f, 'g, F = NullForest> where F: Forest<'f> + 'f {
    // The forest.
    forest: &'f F,
    // The grammar.
    pub(in super) grammar: &'g InternalGrammar,

    // Chart's items.

    // Predicted items are stored in a bit matrix. The bit matrix has a row for every Earley set.
    predicted: BitMatrix,
    // Medial items.
    //
    // N.B. This structure could be moved into its own module.
    medial: Vec<Item<F::NodeRef>>,
    // Completed items are stored for the latest Earley set.
    // They are ordered by (origin, dot), starting with highest
    // origin and dot. The creation of a completed item can only be caused
    // by a scan or a completion of an item that has a higher (origin, dot)
    // pair value.
    complete: BinaryHeap<CompletedItem<F::NodeRef>>,

    // Chart's indices. They point to the beginning of each Earley set.
    indices: Vec<usize>,
    // Index that points to the beginning of the latest set. Equivalent to
    // the last element of `indices`.
    current_medial_start: usize,

    // The latest succesful parse result. It may be unused if the user always calls fn
    // `advance_without_completion` instead of fn `advance`.
    finished_node: Option<F::NodeRef>,

    // The input location.
    earleme: usize,
}

impl<'f, 'g, F> Recognizer<'f, 'g, F> where F: Forest<'f> + 'f {
    /// Creates a new recognizer for a given grammar and forest. The recognizer has an initial
    /// Earley set that predicts the grammar's start symbol.
    pub fn new(grammar: &'g InternalGrammar, forest: &'f F) -> Recognizer<'f, 'g, F> {
        let mut recognizer = Recognizer {
            forest,
            grammar,
            finished_node: None,
            // The initial location is 0.
            earleme: 0,
            // The first Earley set begins at 0 and ends at 0. The second Earley set begins at 0.
            indices: vec![0, 0],
            current_medial_start: 0,
            // Reserve some capacity for vectors.
            predicted: BitMatrix::new(8, grammar.num_syms()),
            medial: Vec::with_capacity(32),
            complete: BinaryHeap::with_capacity(32),
        };
        recognizer.predict(grammar.start_sym());
        recognizer
    }

    /// Makes the current Earley set predict a given symbol.
    pub fn predict(&mut self, symbol: Symbol) {
        // The source in the prediction matrix is the row that corresponds to the predicted symbol.
        let source = &self.grammar.prediction_matrix()[symbol.usize()];
        // The destination in `predicted` is the row that corresponds to the current location.
        let destination = &mut self.predicted[self.earleme];
        for (dst, src) in destination.iter_mut().zip(source.iter()) {
            *dst |= *src;
        }
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

    /// Advances the parse. Calling this method may set the finished node, which can be accessed
    /// through the `finished_node` method.
    pub fn advance(&mut self) -> bool {
        if self.is_exhausted() {
            false
        } else {
            // Completion pass, which saves successful parses.
            self.finished_node = self.completion_pass();
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

    /// Provides access to completions, which can be used to perform a completion pass.
    pub fn completions<'r>(&'r mut self) -> Completions<'f, 'g, 'r, F> {
        // Create a builder for bocage node slices.
        let products = self.forest.build(self.complete.len());
        Completions {
            products,
            recognizer: self,
        }
    }

    /// Performs the completion pass.
    #[inline]
    fn completion_pass(&mut self) -> Option<F::NodeRef> {
        let mut finished_node = None;
        let mut completions = self.completions();
        while let Some(mut completion) = completions.next_completion() {
            // Include all items in the completion.
            let node = completion.complete_all();
            // If this completion is a possible end of parse, save its bocage node.
            if completion.origin == 0 && completion.lhs_sym == self.grammar.start_sym() {
                finished_node = Some(node);
            }
        }
        finished_node
    }

    /// Sorts medial items with deduplication.
    fn sort_medial_items(&mut self) {
        let grammar = &self.grammar;
        // Build index by postdot
        // These medial positions themselves are NOT sorted by postdot symbol.
        sort_and_dedup(&mut self.medial, self.current_medial_start, |item| {
            (grammar.get_rhs1(item.dot), item.dot, item.origin)
        });
    }

    /// Performs the prediction pass.
    #[inline]
    fn prediction_pass(&mut self) {
        // Add a row to the matrix.
        self.predicted.grow(1, false);
        // Iterate through medial items in the current set.
        let mut iter = self.medial[self.current_medial_start..].iter();
        // For each medial item in the current set, predict its postdot symbol.
        while let Some(ei) = iter.next() {
            let postdot = self.grammar.get_rhs1(ei.dot);
            // Skip medial items that have the same postdot symbol as `ei`.
            while let Some(ei) = iter.as_slice().get(0) {
                let cur_postdot = self.grammar.get_rhs1(ei.dot);
                if postdot == cur_postdot {
                    iter.next();
                } else {
                    break;
                }
            }
            // Prediction happens here. We can't use fn `predict`, because `self.medial`
            // is borrowed.
            let source = &self.grammar.prediction_matrix()[postdot.unwrap().usize()];
            let destination = &mut self.predicted[self.earleme];
            for (dst, &src) in destination.iter_mut().zip(source.iter()) {
                *dst |= src;
            }
        }
    }

    /// Complete items.
    #[inline]
    pub fn complete(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        self.complete_medial_items(set_id, sym, rhs_link);
        self.complete_predicted_symbols(set_id, sym, rhs_link);
    }

    /// Complete medial items in a given Earley set.
    #[inline]
    fn complete_medial_items(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        let range = self.indices[set_id as usize] .. self.indices[set_id as usize + 1];
        let mut set_medial = &self.medial[range];
        if set_medial.len() >= 8 {
            // When the set has 8 or more items, we use binary search to narrow down the range of
            // items.
            let set_idx = set_medial.binary_search_by(|ei| {
                (self.grammar.get_rhs1(ei.dot), Ordering::Greater).cmp(&(Some(sym), Ordering::Less))
            });
            match set_idx {
                Ok(idx) | Err(idx) => {
                    set_medial = &set_medial[idx..];
                    // The range contains items that have the same RHS1 symbol. In the future, try
                    // avoiding repeated RHS1 table accesses.
                    let end = set_medial.iter().take_while(|ei| {
                        self.grammar.get_rhs1(ei.dot) == Some(sym)
                    }).count();
                    set_medial = &set_medial[..end];
                }
            }
        }
        // Iterate through medial items to complete them.
        for &Item { dot, origin, node } in set_medial {
            if self.grammar.complete_over(dot, sym) {
                // New completed item.
                // from A ::= B • C
                // to   A ::= B   C •
                self.complete.push(CompletedItem {
                    dot,
                    origin,
                    left_node: node,
                    right_node: Some(rhs_link),
                });
            }
        }
    }

    /// Complete predicted items that have a common postdot symbol.
    #[inline]
    fn complete_predicted_symbols(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        if self.predicted[set_id as usize].get(sym.usize()) {
            for trans in self.grammar.inverse_prediction_of(sym) {
                if self.predicted[set_id as usize].get(trans.symbol.usize()) {
                    self.complete_predicted_item(set_id, trans.dot(), rhs_link);
                }
            }
        }
    }

    /// Complete a predicted item.
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    #[inline]
    fn complete_predicted_item(&mut self, set_id: Origin, dot_kind: DotKind, rhs_link: F::NodeRef) {
        // New item, either completed or pre-terminal. Ensure uniqueness.
        // from A ::= • B   c
        // to   A ::=   B • c
        match dot_kind {
            DotKind::Completed(dot) => {
                // from A ::= • B
                // to   A ::=   B •
                self.complete.push(CompletedItem {
                    dot,
                    origin: set_id,
                    left_node: rhs_link,
                    right_node: None,
                });
            }
            DotKind::Medial(dot) => {
                // No checks for uniqueness, because `medial` will be deduplicated.
                // from A ::= • B   C
                // to   A ::=   B • C
                // Where C is terminal or nonterminal.
                self.medial.push(Item {
                    dot,
                    origin: set_id,
                    node: rhs_link,
                });
            }
        }
    }

    /// Resets the recognizer to its initial state by removing all contents.
    pub fn reset(&mut self) {
        self.earleme = 0;
        self.predict(self.grammar.start_sym());
        self.finished_node = None;
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
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.grammar.has_trivial_derivation() && self.earleme == 0 || self.finished_node.is_some()
    }

    /// Retrieves the bocage node that represents the parse that has finished at the current
    /// location.
    ///
    /// # Panics
    ///
    /// Panics when the parse has not finished at the current location.
    pub fn finished_node(&self) -> F::NodeRef {
        if self.grammar.has_trivial_derivation() && self.earleme == 0 {
            self.forest.nulling(self.grammar.to_external(self.grammar.start_sym()))
        } else {
            self.finished_node.expect("expected a final completed item")
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

    /// Returns a reference to the internal grammar.
    pub fn grammar(&self) -> &'g InternalGrammar {
        self.grammar
    }
}

// Completion

/// A tool for completing items.
pub struct Completions<'f, 'g, 'r, F>
    where F: Forest<'f> + 'f,
          'f: 'r,
          'g: 'r,
{
    /// A builder that creates bocage node slices for completed items.
    products: F::NodeBuilder,
    /// The recognizer.
    recognizer: &'r mut Recognizer<'f, 'g, F>,
}

/// A group of completed items.
pub struct Completion<'c, 'f, 'g, 'r, F>
    where F: Forest<'f> + 'f,
          'f: 'r,
          'g: 'r,
          'r: 'c,
{
    /// The origin location of this completion.
    origin: Origin,
    /// The symbol of this completion.
    lhs_sym: Symbol,
    /// A reference that gives access to the recognizer and the bocage.
    completions: &'c mut Completions<'f, 'g, 'r, F>,
}

impl<'f, 'g, 'r, F> Completions<'f, 'g, 'r, F> where F: Forest<'f> + 'f {
    /// Allows iteration through groups of completions that have unique symbol and origin.
    pub fn next_completion<'c>(&'c mut self) -> Option<Completion<'c, 'f, 'g, 'r, F>> {
        if let Some(&ei) = self.recognizer.complete.peek() {
            Some(Completion::new(self, ei))
        } else {
            None
        }
    }
}

impl<'c, 'f, 'g, 'r, F> Completion<'c, 'f, 'g, 'r, F>
    where F: Forest<'f> + 'f,
          'f: 'r,
          'g: 'r,
          'r: 'c,
{
    /// Creates a completion.
    fn new(completions: &'c mut Completions<'f, 'g, 'r, F>, ei: CompletedItem<F::NodeRef>) -> Self {
        // items with LHS symbol equal to ei's LHS symbol
        let lhs_sym = completions.recognizer.grammar().get_lhs(ei.dot);
        Completion {
            origin: ei.origin,
            lhs_sym,
            completions,
        }
    }

    /// Completes all items.
    pub fn complete_all(&mut self) -> F::NodeRef {
        // For each item, include it in the completion.
        while let Some(item) = self.next() {
            self.push(item);
        }
        // Use all items for completion.
        self.complete()
    }

    /// Allows iteration through completed items.
    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<CompletedItem<F::NodeRef>> {
        if let Some(&completion) = self.completions.recognizer.complete.peek() {
            let completion_lhs_sym = self.completions.recognizer.grammar().get_lhs(completion.dot);
            if self.origin == completion.origin && self.lhs_sym == completion_lhs_sym {
                self.completions.recognizer.complete.pop();
                Some(completion)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Includes an item in the completion.
    pub fn push(&mut self, completed_item: CompletedItem<F::NodeRef>) {
        self.completions.products.push(completed_item);
    }

    /// Uses the completion to complete items in the recognizer.
    pub fn complete(&mut self) -> F::NodeRef {
        let node = self.completions.products.sum(self.origin);
        self.completions.recognizer.complete(self.origin, self.lhs_sym, node);
        self.completions.products.reserve(self.completions.recognizer.complete.len() + 1);
        node
    }

    /// Returns the origin location of this completion.
    pub fn origin(&self) -> Origin {
        self.origin
    }

    /// Returns the symbol of this completion.
    pub fn symbol(&self) -> Symbol {
        self.lhs_sym
    }
}
