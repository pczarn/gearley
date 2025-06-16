use std::mem;

use bumpalo::Bump;

use gearley_forest::evaluate::Evaluate;
use gearley_forest::node_handle::{NodeHandle, NULL_HANDLE};

use allocator_api2::vec::Vec as AVec;

use crate::node::Node;
use crate::bocage::Bocage;

struct WorkNode<'a> {
    node: NodeHandle,
    child: u32,
    parent: usize,
    results: AVec<u32, &'a Bump>,
}

impl Bocage {
    // Once node liveness is marked, you may traverse the nodes.
    //
    // We get a list of linked lists of vecs. For each linked list, run .
    pub fn evaluate<E: Evaluate>(&mut self, eval: E, root_node: NodeHandle) -> Vec<E::Elem> {
        // let mut all_nodes = vec![];
        let alloc = Bump::new();
        let mut results: Vec<E::Elem> = vec![];
        let mut work_stack = vec![WorkNode { node: NULL_HANDLE, child: 0, parent: 0, results: AVec::new_in(&alloc) }, WorkNode { node: root_node, child: 0, parent: 0, results: AVec::new_in(&alloc) }];
        while let Some(mut work) = work_stack.pop() {
            let node = work.node;
            match (self.postprocess_product_tree_node(&self[work.node]), work.child) {
                (Node::Sum { count, .. }, n) if n < count => {
                    work.child += 1;
                    let parent = work.parent;
                    work_stack.push(work);
                    work_stack.push(WorkNode { node: NodeHandle(node.0 + n + 1), child: 0, parent, results: AVec::new_in(&alloc) });
                }
                (Node::Product { left_factor, .. }, 0) => {
                    work.child += 1;
                    work_stack.push(work);
                    work_stack.push(WorkNode { node: left_factor, child: 0, parent: work_stack.len() - 1, results: AVec::new_in(&alloc) });
                }
                (Node::Product { right_factor: Some(right), .. }, 1) => {
                    work.child += 1;
                    work_stack.push(work);
                    work_stack.push(WorkNode { node: right, child: 0, parent: work_stack.len() - 1, results: AVec::new_in(&alloc) });
                }
                (Node::Product { action, .. }, _) if self.is_transparent(action) => {
                    // nothing to do
                }
                (Node::Evaluated { values, .. }, _) => {
                    work_stack[work.parent].results.push(values);
                }
                (Node::Sum { .. }, _) => {
                    // nothing to do
                }
                (Node::Leaf { symbol, values }, _) => {
                    let result = eval.leaf(symbol, values);
                    results.push(result);
                    self[work.node] = Node::Evaluated { symbol, values: results.len() as u32 - 1 }
                }
                (Node::Product { action, .. }, _) => {
                    let result = eval.product(action, work.results.iter().copied().map(|v| &results[v as usize]));
                    results.push(result);
                    work_stack[work.parent].results.push(results.len() as u32 - 1);

                }
                (Node::NullingLeaf { symbol }, _) => {
                    let values = results.len() as u32;
                    eval.nulling(symbol, &mut results);
                    self[work.node] = Node::Evaluated { symbol, values }
                }
            }
        }
        work_stack[0].results.iter().copied().map(|v| mem::replace(&mut results[v as usize], Default::default())).collect()
    }
}
