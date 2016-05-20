pub mod binary_heap;
pub mod slice_builder;

use std::mem;

pub fn sort_and_dedup<H,K,T>(slice: &mut Vec<T>, start: usize, key: H)
    where H: Fn(&T) -> K,
          K: Ord,
          T: Copy,
{
    slice[start..].sort_by(|a, b| key(a).cmp(&key(b)));
    let end = dedup(&mut slice[start..], |a, b| key(a) == key(b));
    slice.truncate(start + end);
}

pub fn dedup<F, T>(slice: &mut [T], eq: F) -> usize where F: Fn(&T, &T) -> bool, T: Copy {
    unsafe {
        let ln = slice.len();
        if ln > 1 {
            // Avoid bounds checks by using raw pointers.
            let p = slice.as_mut_ptr();
            let mut r: usize = 1;
            let mut w: usize = 1;

            while r < ln {
                let p_r = p.offset(r as isize);
                let p_wm1 = p.offset((w - 1) as isize);
                if !eq(&*p_r, &*p_wm1) {
                    if r != w {
                        let p_w = p_wm1.offset(1);
                        mem::swap(&mut *p_r, &mut *p_w);
                    }
                    w += 1;
                }
                r += 1;
            }
            w
        } else {
            ln
        }
    }
}
