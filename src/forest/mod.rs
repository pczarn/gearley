pub mod depth_first;
pub mod null_forest;

pub use self::depth_first::{Bocage, Traversal};
pub use self::null_forest::NullForest;
// pub use self::evaluator::{Evaluator, StackEvaluator};

use cfg::Symbol;

use item::CompletedItem;

pub trait Forest<'a> {
    /// Reference to a node.
    type NodeRef: Copy + 'a;
    type NodeBuilder: NodeBuilder<NodeRef=Self::NodeRef>;
    type LeafValue: 'a;

    fn build(&'a self, num_children: usize) -> Self::NodeBuilder;

    fn leaf(&'a self, token: Symbol, pos: u32, value: Self::LeafValue) -> Self::NodeRef;

    fn nulling(&'a self, token: Symbol) -> Self::NodeRef;
}

pub trait NodeBuilder {
    type NodeRef;

    fn push(&mut self, item: CompletedItem<Self::NodeRef>);

    fn sum(&mut self, origin: u32) -> Self::NodeRef;

    fn reserve(&mut self, len: usize);
}

impl<'f, F> Forest<'f> for &'f F where F: Forest<'f> {
    type NodeRef = F::NodeRef;
    type NodeBuilder = F::NodeBuilder;
    type LeafValue = F::LeafValue;

    fn build(&'f self, num_children: usize) -> Self::NodeBuilder {
        (**self).build(num_children)
    }

    fn leaf(&'f self, token: Symbol, pos: u32, value: Self::LeafValue) -> Self::NodeRef {
        (**self).leaf(token, pos, value)
    }

    fn nulling(&'f self, token: Symbol) -> Self::NodeRef {
        (**self).nulling(token)
    }
}
