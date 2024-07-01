// Copyright 2019 The Rust Project Developers, Piotr Czarnecki.
// See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A priority queue implemented with a binary heap.

#![allow(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(nonminimal_bool))]

use std::mem::swap;

use gearley_forest::completed_item::CompletedItem;

use crate::local_prelude::*;
use crate::recognizer::item::CompletedItemLinked;
use crate::utils::vec2d::Vec2d;

use super::performance_policy::PerformancePolicy;

#[derive(Debug)]
pub(super) struct BinaryHeap<T>(pub(super) Vec<T>);

impl<G, F, P> Recognizer<G, F, P>
where
    F: Forest,
    G: Grammar,
    P: PerformancePolicy,
{
    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    #[inline]
    pub fn heap_peek(&self) -> Option<CompletedItem<F::NodeRef>> {
        self.complete.0.get(0).and_then(|&right_item| {
            self.medial
                .get_item(right_item.idx as usize)
                .map(|left_item| CompletedItem {
                    origin: left_item.origin,
                    dot: left_item.dot,
                    left_node: left_item.node,
                    right_node: right_item.node,
                })
        })
    }

    #[inline(always)]
    fn heap_get(&self, idx_idx: usize) -> Option<&Item<F::NodeRef>> {
        self.complete.0
            .get(idx_idx)
            .and_then(|&item| self.medial.get_item(item.idx as usize))
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    pub fn heap_pop(&mut self) -> Option<CompletedItem<F::NodeRef>> {
        self.complete.0.pop().and_then(move |mut right_item| {
            if !self.complete.0.is_empty() {
                swap(&mut right_item, &mut self.complete.0[0]);
                self.sift_down(0);
            }
            self.medial
                .get_item(right_item.idx as usize)
                .map(|left_item| CompletedItem {
                    origin: left_item.origin,
                    dot: left_item.dot,
                    left_node: left_item.node,
                    right_node: right_item.node,
                })
        })
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, mut pos: usize, end: usize) {
        let element_idx = self.complete.0[pos];
        let element = self.medial.get_item(element_idx.idx as usize).expect("invalid Item index");
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            // compare with the greater of the two children
            if right < end && !(self.heap_get(child).unwrap() > self.heap_get(right).unwrap()) {
                child = right;
            }
            // if we are already in order, stop.
            if element >= self.heap_get(child).unwrap() {
                break;
            }
            self.complete.0[pos] = self.complete.0[child];
            pos = child;
            child = 2 * pos + 1;
        }
        self.complete.0[pos] = element_idx;
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.complete.0.len();
        self.sift_down_range(pos, len);
    }
}

impl<R> BinaryHeap<CompletedItemLinked<R>> where R: Clone + Copy {
    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn clear(&mut self) {
        self.0.clear()
    }

    /// Pushes an item onto the binary heap.
    pub(super) fn heap_push_linked(&mut self, item: CompletedItemLinked<R>, medial: &Vec2d<Item<R>>) {
        let old_indices_len = self.0.len();
        self.0.push(item);
        self.sift_up(0, old_indices_len, medial);
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    fn sift_up(&mut self, start: usize, mut pos: usize, medial: &Vec2d<Item<R>>) {
        let element_idx = self.0[pos];
        let element = medial.get_item(element_idx.idx as usize).expect("invalid Item index");
        while pos > start {
            let parent = (pos - 1) / 2;
            let parent_idx = self.0[parent];
            if *element <= *medial.get_item(parent_idx.idx as usize).expect("invalid Item index") {
                break;
            }
            self.0[pos] = parent_idx;
            pos = parent;
        }
        self.0[pos] = element_idx;
    }
}
