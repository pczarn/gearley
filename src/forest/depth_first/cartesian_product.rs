use std::marker::PhantomData;

use forest::depth_first::Evaluated;
use super::evaluate::ProductHandle;

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
                start: start,
                end: start.offset(slice.len() as isize),
                marker: PhantomData,
            }
        }
    }

    fn advance(&mut self, ptr: &mut &'a V) -> bool {
        unsafe {
            *ptr = &*(*ptr as *const _).offset(1);
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

    /// Initialize the cartesian product from a production.
    pub fn from_production<'t, 'f, T>(&mut self, production: &ProductHandle<'a, 't, 'f, T, V>)
        where T: Copy
    {
        self.ranges.clear();
        self.ptrs.clear();
        let slices = production.factors.iter().map(|factor| {
            match factor.get() {
                Evaluated { values } => {
                    values
                }
                _ => unreachable!()
            }
        });
        self.ranges.extend(slices.map(|slice| Factor::new(slice)));
        unsafe {
            self.ptrs.extend(self.ranges.iter().map(|factor| &*factor.start));
        }
    }

    pub fn as_slice(&self) -> &[&'a V] {
        &self.ptrs[..]
    }
}

impl<'a, V> Iterator for CartesianProduct<'a, V> {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        for (ptr, factor) in self.ptrs.iter_mut().zip(&mut self.ranges) {
            if !factor.advance(ptr) {
                return Some(());
            }
        }
        None
    }
}

#[test]
fn test_cartesian_product() {
    use forest::depth_first::NodeRef;
    let (a, b, c) = ([1, 2, 3], [1, 2,], [1, 2, 3]);
    let (a, b, c) = (
        Evaluated { values: &a }.into(),
        Evaluated { values: &b }.into(),
        Evaluated { values: &c }.into()
    );
    let factors: &[NodeRef<(), i32>] = &[&a, &b, &c];
    let production = ProductHandle { action: 0, factors: factors };
    let mut cartesian_product = CartesianProduct::new();
    cartesian_product.from_production(&production);
    let mut result = vec![];
    loop {
        {
            let val = cartesian_product.as_slice();
            result.push(*val[0] * 100 + *val[1] * 10 + *val[2]);
        };
        if cartesian_product.next().is_none() {
            break;
        }
    }
    assert_eq!(&result[..], &[111, 211, 311, 121, 221, 321,
                              112, 212, 312, 122, 222, 322,
                              113, 213, 313, 123, 223, 323,]);
}
