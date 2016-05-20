use std::cmp::Ordering;

use bit_matrix::BitMatrix;
use cfg::*;

use events::{RawPredicted, RawItems};
use forest::{Forest, NodeBuilder, NullForest};
use grammar::{InternalGrammar, DotKind};
use item::{CompletedItem, Item, Origin};
use util::sort_and_dedup;
use util::binary_heap::BinaryHeap;

pub struct Recognizer<'f, 'g, F = NullForest> where F: Forest<'f> + 'f {
    forest: &'f F,
    g: &'g InternalGrammar,
    finished_node: Option<F::NodeRef>,
    // The input location.
    earleme: usize,
    // Chart's items.
    predicted: BitMatrix,
    medial: Vec<Item<F::NodeRef>>,
    // Completed items are ordered by (origin, dot), starting with highest
    // origin and dot. The creation of a completed item can only be caused
    // by a scan or a completion of an item that has a higher (origin, dot)
    // pair value.
    complete: BinaryHeap<CompletedItem<F::NodeRef>>,
    // Chart's indices.
    indices: Vec<usize>,
    current_medial_start: usize,
}

impl<'f, 'g, F> Recognizer<'f, 'g, F> where F: Forest<'f> + 'f {
    pub fn new(grammar: &'g InternalGrammar, forest: &'f F) -> Recognizer<'f, 'g, F> {
        let mut recognizer = Recognizer {
            forest: forest,
            g: grammar,
            finished_node: None,
            earleme: 0,
            indices: vec![0, 0],
            current_medial_start: 0,
            predicted: BitMatrix::new(8, grammar.num_syms()),
            medial: Vec::with_capacity(16),
            complete: BinaryHeap::with_capacity(32),
        };
        recognizer.predict(grammar.start_sym());
        recognizer
    }

    pub fn predict(&mut self, symbol: Symbol) {
        let row = &self.g.prediction_matrix()[symbol.usize()];
        let predicted = &mut self.predicted[self.earleme];
        for (dst, field) in predicted.iter_mut().zip(row.iter()) {
            *dst |= *field;
        }
    }

    // Public

    // Part of the scan pass
    pub fn scan(&mut self, external: Symbol, value: F::LeafValue) {
        if let Some(internal) = self.g.to_internal(external) {
            let earleme = self.earleme as u32;
            let node = self.forest.leaf(external, earleme + 1, value);
            self.complete(earleme, internal, node);
        }
    }

    pub fn advance(&mut self) -> bool {
        if self.is_exhausted() {
            false
        } else {
            // Completion pass, which saves successful parses.
            self.finished_node = self.completion_pass();
            // The rest.
            self.advance_without_completion();
            true
        }
    }

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

    #[inline]
    pub fn is_exhausted(&self) -> bool {
        self.medial.len() == self.current_medial_start && self.complete.is_empty()
    }

    pub fn completions<'r>(&'r mut self) -> Completions<'f, 'g, 'r, F> {
        let products = self.forest.build(self.complete.len());
        Completions {
            products: products,
            recognizer: self,
        }
    }

    #[inline]
    fn completion_pass(&mut self) -> Option<F::NodeRef> {
        let mut finished_node = None;
        let mut completions = self.completions();
        while let Some(mut completion) = completions.next_completion() {
            while let Some(item) = completion.next() {
                completion.push(item);
            }
            let node = completion.complete();
            if completion.origin == 0 && completion.lhs_sym == self.g.start_sym() {
                finished_node = Some(node);
            }
        }
        finished_node
    }

    fn sort_medial_items(&mut self) {
        let g = &self.g;
        // Build index by postdot
        // These medial positions themselves are NOT sorted by postdot symbol.
        sort_and_dedup(&mut self.medial, self.current_medial_start, |item| {
            (g.get_rhs1(item.dot), item.dot, item.origin)
        });
    }

    #[inline]
    fn prediction_pass(&mut self) {
        self.predicted.grow(1, false);

        let mut iter = self.medial[self.current_medial_start..].iter();

        while let Some(ei) = iter.next() {
            let postdot = self.g.get_rhs1(ei.dot);
            while let Some(ei) = iter.as_slice().get(0) {
                let cur_postdot = self.g.get_rhs1(ei.dot);
                if postdot == cur_postdot {
                    iter.next();
                } else {
                    break;
                }
            }
            // actual prediction
            let row = &self.g.prediction_matrix()[postdot.unwrap().usize()];
            let predicted = &mut self.predicted[self.earleme];
            for (dst, &word) in predicted.iter_mut().zip(row.iter()) {
                *dst |= word;
            }
        }
    }

    #[inline]
    pub fn complete(&mut self, set_id: u32, sym: Symbol, rhs_link: F::NodeRef) {
        self.complete_medial(set_id, sym, rhs_link);
        self.complete_predicted(set_id, sym, rhs_link);
    }

    #[inline]
    fn complete_medial(&mut self, set_id: u32, sym: Symbol, rhs_link: F::NodeRef) {
        let range = self.indices[set_id as usize] .. self.indices[set_id as usize + 1];
        let mut set_medial = &self.medial[range];
        if set_medial.len() >= 8 {
            match set_medial.binary_search_by(|ei| {
                (self.g.get_rhs1(ei.dot), Ordering::Greater).cmp(&(Some(sym), Ordering::Less))
            }) {
                Ok(idx) | Err(idx) => {
                    set_medial = &set_medial[idx..];
                    let end = set_medial.iter().take_while(|ei|
                        self.g.get_rhs1(ei.dot) == Some(sym)
                    ).count();
                    set_medial = &set_medial[..end];
                }
            }
        }

        for &Item { dot, origin, node } in set_medial {
            if self.g.complete_over(dot, sym) {
                // New completed item.
                self.complete.push(CompletedItem {
                    dot: dot,
                    origin: origin,
                    left_node: node,
                    right_node: Some(rhs_link),
                });
            }
        }
    }

    #[inline]
    fn complete_predicted(&mut self, set_id: u32, sym: Symbol, rhs_link: F::NodeRef) {
        let predicted = &self.predicted[set_id as usize];
        if predicted.get(sym.usize()) {
            for trans in self.g.inverse_prediction_of(sym) {
                if predicted.get(trans.symbol.usize()) {
                    // New item, either completed or pre-terminal. Ensure uniqueness.
                    // from A ::= • B   c?
                    // to   A ::=   B • c?
                    match trans.dot() {
                        DotKind::Completed(dot) => {
                            // from A ::= • B
                            // to   A ::=   B •
                            self.complete.push(CompletedItem {
                                dot: dot,
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
                                dot: dot,
                                origin: set_id,
                                node: rhs_link,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.earleme = 0;
        self.predict(self.g.start_sym());
        self.finished_node = None;
        self.indices.clear();
        self.indices.push(0);
        self.indices.push(0);
        self.current_medial_start = 0;
        self.medial.clear();
        self.complete.clear();
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.g.has_trivial_derivation() && self.earleme == 0 || self.finished_node.is_some()
    }

    pub fn finished_node(&self) -> F::NodeRef {
        if self.g.has_trivial_derivation() && self.earleme == 0 {
            self.forest.nulling(self.g.to_external(self.g.start_sym()))
        } else {
            self.finished_node.expect("expected a final completed item")
        }
    }

    pub fn raw_predicted_items(&self) -> RawPredicted {
        let earleme = self.earleme();
        RawPredicted::new(self.predicted.iter_row(earleme))
    }

    pub fn raw_medial_items(&self) -> RawItems<F::NodeRef> {
        let indices_len = self.indices.len();
        // Next-to-last index.
        let items_start = self.indices[indices_len - 2];
        self.medial[items_start..].iter()
    }

    // Public accessors

    pub fn earleme(&self) -> usize {
        self.earleme
    }

    pub fn grammar(&self) -> &'g InternalGrammar {
        self.g
    }
}

// Completion

pub struct Completions<'f, 'g, 'r, F>
    where F: Forest<'f> + 'f,
          'f: 'r,
          'g: 'r,
{
    products: F::NodeBuilder,
    recognizer: &'r mut Recognizer<'f, 'g, F>,
}

pub struct Completion<'c, 'f, 'g, 'r, F>
    where F: Forest<'f> + 'f,
          'f: 'r,
          'g: 'r,
          'r: 'c,
{
    lhs_sym: Symbol,
    origin: Origin,
    completions: &'c mut Completions<'f, 'g, 'r, F>,
}

impl<'f, 'g, 'r, F> Completions<'f, 'g, 'r, F> where F: Forest<'f> + 'f {
    pub fn next_completion<'c>(&'c mut self) -> Option<Completion<'c, 'f, 'g, 'r, F>> {
        if let Some(&ei) = self.recognizer.complete.peek() {
            // items with LHS symbol equal to ei's LHS symbol
            let lhs_sym = self.recognizer.grammar().get_lhs(ei.dot);
            Some(Completion {
                lhs_sym: lhs_sym,
                origin: ei.origin,
                completions: self,
            })
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

    pub fn push(&mut self, completed_item: CompletedItem<F::NodeRef>) {
        self.completions.products.push(completed_item);
    }

    pub fn complete(&mut self) -> F::NodeRef {
        let node = self.completions.products.sum(self.origin);
        self.completions.recognizer.complete(self.origin, self.lhs_sym, node);
        self.completions.products.reserve(self.completions.recognizer.complete.len() + 1);
        node
    }

    pub fn origin(&self) -> Origin {
        self.origin
    }

    pub fn symbol(&self) -> Symbol {
        self.lhs_sym
    }
}
