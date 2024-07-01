use std::cmp;

use cfg_symbol::Symbol;

use crate::local_prelude::*;
use gearley_forest::completed_item::CompletedItem;
use super::{item::{CompletedItemLinked, Origin}, performance_policy::PerformancePolicy, Recognizer};
use super::lookahead::Lookahead;

/// A group of completed items.
pub struct CompleteSum<'r, F, G, P>
where
    F: Forest,
    G: Grammar,
    P: PerformancePolicy,
{
    /// The origin location of this completion.
    origin: Origin,
    /// The symbol of this completion.
    lhs_sym: Symbol,
    /// The recognizer.
    recognizer: &'r mut Recognizer<G, F, P>,
}

impl<G, F, P> Recognizer<G, F, P>
    where F: Forest,
    G: Grammar,
    P: PerformancePolicy,
{
    /// Complete items.
    pub fn complete(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        debug_assert!(sym != self.grammar.eof());
        if self.predicted[set_id as usize].get(sym.usize()) {
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

        // TODO: try to simplify this
        let inner_start = if specific_set.len() >= 16 {
            // When the set has 16 or more items, we use binary search to narrow down the range of
            // items.
            // todo branchless binary search
            let set_idx = specific_set.binary_search_by(|ei| {
                (self.grammar.get_rhs1(ei.dot), cmp::Ordering::Greater).cmp(&(Some(sym), cmp::Ordering::Less))
            });
            match set_idx {
                Ok(idx) | Err(idx) => idx,
            }
        } else {
            // TODO: simplify by removing this branch
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
        let set_range = inner_start..inner_start + inner_end;
        let start = self.medial.index_at(set_id as usize) as u32;
        for idx in set_range {
            // New completed item.
            // from A ::= B • C
            // to   A ::= B   C •
            let dot = self.medial[set_id as usize][idx].dot;
            if self.grammar.lr_set(dot)[self.lookahead.sym().usize()] {
                self.complete.heap_push_linked(CompletedItemLinked {
                    idx: start + idx as u32,
                    node: Some(rhs_link),
                }, &self.medial);
            }
        }
    }

    /// Complete predicted items that have a common postdot symbol.
    fn complete_predictions(&mut self, set_id: Origin, sym: Symbol, rhs_link: F::NodeRef) {
        let mut unary: u32 = 0;
        for trans in self.grammar.completions(sym) {
            let was_predicted = self.predicted[set_id as usize].get(trans.symbol.usize());
            let will_be_useful = self.grammar.lr_set(trans.dot)[self.lookahead.sym().usize()];
            if was_predicted && will_be_useful {
                // No checks for uniqueness, because `medial` will be deduplicated.
                // --- UNARY
                // from A ::= • B
                // to   A ::=   B •
                // --- BINARY
                // No checks for uniqueness, because `medial` will be deduplicated.
                // from A ::= • B   C
                // to   A ::=   B • C
                // Where C is terminal or nonterminal.
                self.medial.push_item(Item {
                    origin: set_id,
                    dot: trans.dot,
                    node: rhs_link,
                });
                unary += trans.is_unary as u32;
            }
        }
        for idx in self.medial.len() as u32 - unary .. self.medial.len() as u32 {
            self.complete.heap_push_linked(CompletedItemLinked { idx, node: None }, &self.medial)
        }
    }

    // Completion

    /// Performs the completion pass.
    pub fn complete_all_sums_entirely(&mut self) {
        while let Some(mut completion) = self.next_sum() {
            // Include all items in the completion.
            completion.complete_entire_sum();
        }
        self.lookahead.clear_hint();
    }

    /// Allows iteration through groups of completions that have unique symbol and origin.
    pub fn next_sum<'r>(&'r mut self) -> Option<CompleteSum<'r, F, G, P>> {
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

impl<'r, F, G, P> CompleteSum<'r, F, G, P>
where
    F: Forest,
    G: Grammar,
    P: PerformancePolicy,
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
