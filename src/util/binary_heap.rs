// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
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
use std::vec::Vec;
use std::u32;

/// A priority queue implemented with a binary heap.
///
/// This will be a max-heap.
#[derive(Debug)]
pub struct BinaryHeap<T> {
    /// Indices into `data`.
    indices: Vec<u32>,
    data: Vec<T>,
}

impl<T: Clone> Clone for BinaryHeap<T> {
    fn clone(&self) -> Self {
        BinaryHeap { indices: self.indices.clone(), data: self.data.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.indices.clone_from(&source.indices);
        self.data.clone_from(&source.data);
    }
}

impl<T> Default for BinaryHeap<T> {
    fn default() -> Self {
        BinaryHeap {
            indices: vec![],
            data: vec![],
        }
    }
}

impl<T: Ord> BinaryHeap<T> {
    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    pub fn with_capacity(capacity: usize) -> BinaryHeap<T> {
        BinaryHeap {
            indices: Vec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.indices.get(0).and_then(|&idx| self.data.get(idx as usize))
    }

    #[inline(always)]
    fn get(&self, idx_idx: usize) -> Option<&T> {
        self.indices.get(idx_idx).and_then(|&idx| self.data.get(idx as usize))
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `BinaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
        self.data.reserve(additional);
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    pub fn pop(&mut self) -> Option<&T> {
        self.indices.pop().and_then(move |mut idx| {
            if !self.is_empty() {
                swap(&mut idx, &mut self.indices[0]);
                self.sift_down(0);
            }
            self.data.get(idx as usize)
        })
    }

    /// Pushes an item onto the binary heap.
    // #[inline(always)]
    pub fn push(&mut self, item: T) {
        let old_indices_len = self.len();
        let old_data_len = self.data.len();
        assert!(old_data_len as u64 <= u32::MAX.into());
        self.data.push(item);
        self.indices.push(old_data_len as u32);
        self.sift_up(0, old_indices_len);
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    fn sift_up(&mut self, start: usize, mut pos: usize) {
        let element_idx = self.indices[pos];
        let element = &self.data[element_idx as usize];
        while pos > start {
            let parent = (pos - 1) / 2;
            let parent_idx = self.indices[parent];
            if *element <= self.data[parent_idx as usize] {
                break;
            }
            self.indices[pos] = parent_idx;
            pos = parent;
        }
        self.indices[pos] = element_idx;
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, mut pos: usize, end: usize) {
        let element_idx = self.indices[pos];
        let element = &self.data[element_idx as usize];
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            // compare with the greater of the two children
            if right < end && !(self.get(child).unwrap() > self.get(right).unwrap()) {
                child = right;
            }
            // if we are already in order, stop.
            if element >= self.get(child).unwrap() {
                break;
            }
            self.indices[pos] = self.indices[child];
            pos = child;
            child = 2 * pos + 1;
        }
        self.indices[pos] = element_idx;
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        self.sift_down_range(pos, len);
    }

    /// Returns the length of the binary heap.
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Checks if the binary heap is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Drops all items from the binary heap.
    pub fn clear(&mut self) {
        self.indices.clear();
        self.data.clear();
    }
}

impl<T: Ord> Extend<T> for BinaryHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        let iter = iterable.into_iter();
        let (lower, _) = iter.size_hint();

        self.reserve(lower);

        for elem in iter {
            self.push(elem);
        }
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for BinaryHeap<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}
