use std::marker::PhantomData;

use forest::depth_first::Node;

pub trait Order<'a, 'f, T, V> where T: Copy {
    /// Apply the order to sum node alternatives.
    fn sum<'b>(&mut self, alternatives: &'b [Node<'a, 'f, T, V>]) -> &'b [Node<'a, 'f, T, V>] {
    	alternatives
    }

    /// Apply the order to product node factors.
    fn product(&mut self, _factors: &[&Node<'a, 'f, T, V>]) -> Option<usize> {
    	None
    }
}

pub struct NullOrder<'a, 'f, T, V> {
	marker: PhantomData<(&'a (), &'f (), T, V)>,
}

impl<'a, 'f, T, V> Order<'a, 'f, T, V> for NullOrder<'a, 'f, T, V> where T: Copy {}

impl<'a, 'f, T, V> NullOrder<'a, 'f, T, V> {
    pub fn new() -> Self {
    	NullOrder { marker: PhantomData }
    }
}
