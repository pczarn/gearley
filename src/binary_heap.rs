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

use forest::Forest;
use item::CompletedItem;
use recognizer::Recognizer;
use policy::PerformancePolicy;

impl<'g, F, P> Recognizer<'g, F, P>
    where F: Forest,
          P: PerformancePolicy,
{
    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    #[inline]
    pub fn heap_peek(&self) -> Option<CompletedItem<F::NodeRef>> {
        self.complete.get(0).cloned()
    }

    #[inline(always)]
    fn heap_get(&self, idx_idx: usize) -> Option<&CompletedItem<F::NodeRef>> {
        self.complete.get(idx_idx)
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    pub fn heap_pop(&mut self) -> Option<CompletedItem<F::NodeRef>> {
        if let Some(mut right_item) = self.complete.pop() {
            if !self.complete.is_empty() {
                swap(&mut right_item, &mut self.complete[0]);
                self.sift_down(0);
            }
            Some(right_item)
        } else {
            None
        }
    }

    // /// Pushes an item onto the binary heap.
    // pub fn heap_push(&mut self, item: CompletedItem<F::NodeRef>) {
    //     let old_indices_len = self.complete.len();
    //     let old_medial_len = self.medial.len();
    //     assert!(old_medial_len as u64 <= u32::MAX.into());
    //     self.complete.push(CompletedItemLinked {
    //         idx: old_medial_len as u32,
    //         node: item.right_node,
    //     });
    //     self.sift_up(0, old_indices_len);
    // }

    /// Pushes an item onto the binary heap.
    pub fn heap_push(&mut self, item: CompletedItem<F::NodeRef>) {
        let old_indices_len = self.complete.len();
        self.complete.push(item);
        self.sift_up(0, old_indices_len);
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    fn sift_up(&mut self, start: usize, mut pos: usize) {
        let element = self.complete[pos];
        while pos > start {
            let parent = (pos - 1) / 2;
            let parent_idx = self.complete[parent];
            if element <= parent_idx {
                break;
            }
            self.complete[pos] = parent_idx;
            pos = parent;
        }
        self.complete[pos] = element;
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, mut pos: usize, end: usize) {
        let element = self.complete[pos];
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            // compare with the greater of the two children
            if right < end && !(self.heap_get(child).unwrap() > self.heap_get(right).unwrap()) {
                child = right;
            }
            // if we are already in order, stop.
            if &element >= self.heap_get(child).unwrap() {
                break;
            }
            self.complete[pos] = self.complete[child];
            pos = child;
            child = 2 * pos + 1;
        }
        self.complete[pos] = element;
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.complete.len();
        self.sift_down_range(pos, len);
    }
}
