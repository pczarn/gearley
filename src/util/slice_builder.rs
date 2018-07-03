// use std::intrinsics::needs_drop;
use std::ptr;
use std::mem;
use std::slice;

use typed_arena::Arena;

pub struct SliceBuilder<'a, T: 'a> {
    arena: &'a Arena<T>,
    init: &'a [T],
    uninit_ptr: *mut T,
    uninit_end: *mut T,
}

impl<'a, T> SliceBuilder<'a, T> {
    pub fn new(arena: &'a Arena<T>, len: usize) -> Self {
        let uninit = unsafe {
            arena.alloc_uninitialized(len)
        };
        unsafe {
            SliceBuilder {
                arena,
                init: slice::from_raw_parts((&*uninit).as_ptr(), 0),
                uninit_ptr: (*uninit).as_mut_ptr(),
                uninit_end: (*uninit).as_mut_ptr().offset((*uninit).len() as isize),
            }
        }
    }

    #[inline]
    pub fn push(&mut self, elem: T) {
        assert!(self.uninit_ptr != self.uninit_end);
        unsafe {
            ptr::write(self.uninit_ptr, elem);
            self.uninit_ptr = self.uninit_ptr.offset(1);
            self.init = slice::from_raw_parts(self.init.as_ptr(), self.init.len() + 1);
        }
    }

    #[inline]
    pub fn advance_slice(&mut self) -> &'a [T] {
        let prev_init = self.init;
        unsafe {
            self.init = slice::from_raw_parts(self.uninit_ptr, 0);
        }
        prev_init
    }

    #[inline]
    pub fn into_slice(self) -> &'a [T] {
        self.init
    }

    #[inline]
    pub fn uninit_len(&self) -> usize {
        (self.uninit_end as usize - self.uninit_ptr as usize) / mem::size_of::<T>()
    }

    #[inline]
    pub unsafe fn extend(&mut self, new_slice: *mut [T]) {
        assert_eq!(self.uninit_end, (*new_slice).as_mut_ptr());
        self.uninit_end = self.uninit_end.offset((*new_slice).len() as isize);
    }

    #[inline]
    pub fn reserve(&mut self, len: usize) {
        if self.uninit_len() < len {
            let extra_needed = len - self.uninit_len();
            let available = unsafe {
                (&*self.arena.uninitialized_array()).len()
            };
            if available >= extra_needed {
                // enough to reserve
                unsafe {
                    let new_slice = self.arena.alloc_uninitialized(extra_needed);
                    self.extend(new_slice);
                }
            } else {
                *self = SliceBuilder::new(self.arena, len);
            }
        }
    }
}
