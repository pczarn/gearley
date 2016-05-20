use typed_arena::Arena;

use forest::NodeBuilder;
use forest::depth_first::{Node, NodeRef, Sum, Product};
use forest::depth_first::node::Factors;
use item::CompletedItem;
use util::slice_builder::SliceBuilder;

pub struct SumBuilder<'a, 'f, T: 'f + Copy, V: 'a> where 'a: 'f {
    slice_builder: SliceBuilder<'f, Node<'a, 'f, T, V>>,
}

impl<'a, 'f, T, V> SumBuilder<'a, 'f, T, V> where T: Copy {
    pub fn new(arena: &'f Arena<Node<'a, 'f, T, V>>, len: usize) -> Self {
        SumBuilder {
            slice_builder: SliceBuilder::new(arena, len),
        }
    }
}

impl<'a, 'f, T, V> NodeBuilder for SumBuilder<'a, 'f, T, V> where T: Copy {
    type NodeRef = NodeRef<'a, 'f, T, V>;

    #[inline]
    fn push(&mut self, item: CompletedItem<Self::NodeRef>) {
        let node = Product {
            action: item.dot,
            factors: Factors {
                left: item.left_node,
                right: item.right_node,
            }
        }.into();
        self.slice_builder.push(node);
    }

    fn sum(&mut self, _origin: u32) -> Self::NodeRef {
        let alternatives = self.slice_builder.advance_slice();
        if alternatives.len() == 1 {
            // Faster case: unambiguous node.
            &alternatives[0]
        } else {
            // See that the array `node.init` contains valid values.
            self.reserve(1);
            let sum = Sum {
                summands: alternatives,
            };
            self.slice_builder.push(sum.into());
            &self.slice_builder.advance_slice()[0]
        }
    }

    #[inline]
    fn reserve(&mut self, len: usize) {
        self.slice_builder.reserve(len);
    }
}
