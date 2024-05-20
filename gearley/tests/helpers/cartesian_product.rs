use std::marker::PhantomData;

pub struct Factor<'a, V: 'a> {
    start: *const V,
    end: *const V,
    marker: PhantomData<&'a V>,
}

impl<'a, V> Factor<'a, V> {
    fn new(slice: &'a [V]) -> Self {
        let start = slice.as_ptr();
        unsafe {
            Factor {
                start,
                end: start.offset(slice.len() as isize),
                marker: PhantomData,
            }
        }
    }

    fn advance(&mut self, ptr: &mut &'a V) -> bool {
        unsafe {
            *ptr = &*(*ptr as *const V).offset(1);
            if *ptr as *const _ == self.end {
                *ptr = &*self.start;
                true
            } else {
                false
            }
        }
    }
}

pub struct CartesianProduct<'a, V: 'a> {
    ptrs: Vec<&'a V>,
    ranges: Vec<Factor<'a, V>>,
}

impl<'a, V> CartesianProduct<'a, V> {
    pub fn new() -> Self {
        CartesianProduct {
            ptrs: Vec::with_capacity(8),
            ranges: Vec::with_capacity(8),
        }
    }

    pub fn clear(&mut self) {
        self.ranges.clear();
        self.ptrs.clear();
    }

    /// Multiplies the cartesian product by a slice.
    pub fn push(&mut self, slice: &'a [V]) {
        self.ranges.push(Factor::new(slice));
        unsafe {
            self.ptrs
                .push(self.ranges.last().map(|factor| &*factor.start).unwrap());
        }
    }

    /// Multiplies the cartesian product by an iterator.
    pub fn extend<I>(&mut self, product: I)
    where
        I: Iterator<Item = &'a [V]>,
    {
        self.ranges.extend(product.map(|slice| Factor::new(slice)));
        unsafe {
            // FIXME wrong range
            self.ptrs
                .extend(self.ranges.iter().map(|factor| &*factor.start));
        }
    }

    pub fn as_slice(&self) -> &[&'a V] {
        &self.ptrs[..]
    }

    pub fn advance(&mut self) -> bool {
        for (ptr, factor) in self.ptrs.iter_mut().zip(&mut self.ranges) {
            if !factor.advance(ptr) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_cartesian_product() {
    let (a, b, c) = ([1, 2, 3], [1, 2], [1, 2, 3]);
    let factors: &[&[u32]] = &[&a[..], &b[..], &c[..]];
    let mut cartesian_product = CartesianProduct::new();
    cartesian_product.clear();
    cartesian_product.extend(factors.iter().cloned());
    let mut result = vec![];
    loop {
        {
            let val = cartesian_product.as_slice();
            result.push(*val[0] * 100 + *val[1] * 10 + *val[2]);
        };
        if !cartesian_product.advance() {
            break;
        }
    }
    assert_eq!(
        &result[..],
        &[
            111, 211, 311, 121, 221, 321, 112, 212, 312, 122, 222, 322, 113, 213, 313, 123, 223,
            323,
        ]
    );
}
