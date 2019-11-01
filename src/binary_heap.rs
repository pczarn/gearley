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
use std::u32;

use forest::Forest;
use item::{CompletedItem, CompletedItemLinked, Item};
use recognizer::Recognizer;

impl<'g, F> Recognizer<'g, F>
    where F: Forest,
{
    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    #[inline]
    pub fn heap_peek(&self) -> Option<CompletedItem<F::NodeRef>> {
        self.complete.get(0).and_then(|&right_item|
            self.medial.get(right_item.idx as usize).map(|left_item|
                CompletedItem {
                    origin: left_item.origin,
                    dot: left_item.dot,
                    left_node: left_item.node,
                    right_node: right_item.node,
                }
            )
        )
    }

    #[inline(always)]
    fn heap_get(&self, idx_idx: usize) -> Option<&Item<F::NodeRef>> {
        self.complete.get(idx_idx).and_then(|&item| self.medial.get(item.idx as usize))
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    pub fn heap_pop(&mut self) -> Option<CompletedItem<F::NodeRef>> {
        self.complete.pop().and_then(move |mut right_item| {
            if !self.complete.is_empty() {
                swap(&mut right_item, &mut self.complete[0]);
                self.sift_down(0);
            }
            self.medial.get(right_item.idx as usize).map(|left_item|
                CompletedItem {
                    origin: left_item.origin,
                    dot: left_item.dot,
                    left_node: left_item.node,
                    right_node: right_item.node,
                }
            )
        })
    }

    /// Pushes an item onto the binary heap.
    pub fn heap_push(&mut self, item: CompletedItem<F::NodeRef>) {
        let old_indices_len = self.complete.len();
        let old_medial_len = self.medial.len();
        assert!(old_medial_len as u64 <= u32::MAX.into());
        self.medial.push(item.into());
        self.complete.push(CompletedItemLinked {
            idx: old_medial_len as u32,
            node: item.right_node,
        });
        self.sift_up(0, old_indices_len);
    }

    /// Pushes an item onto the binary heap.
    pub fn heap_push_linked(&mut self, item: CompletedItemLinked<F::NodeRef>) {
        let old_indices_len = self.complete.len();
        self.complete.push(item);
        self.sift_up(0, old_indices_len);
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    fn sift_up(&mut self, start: usize, mut pos: usize) {
        let element_idx = self.complete[pos];
        let element = &self.medial[element_idx.idx as usize];
        while pos > start {
            let parent = (pos - 1) / 2;
            let parent_idx = self.complete[parent];
            if *element <= self.medial[parent_idx.idx as usize] {
                break;
            }
            self.complete[pos] = parent_idx;
            pos = parent;
        }
        self.complete[pos] = element_idx;
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, mut pos: usize, end: usize) {
        let element_idx = self.complete[pos];
        let element = &self.medial[element_idx.idx as usize];
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
            self.complete[pos] = self.complete[child];
            pos = child;
            child = 2 * pos + 1;
        }
        self.complete[pos] = element_idx;
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.complete.len();
        self.sift_down_range(pos, len);
    }
}
