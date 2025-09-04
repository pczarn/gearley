use bytemuck::{bytes_of, from_bytes};
use cfg_symbol::Symbol;
use gearley_forest::node_handle::NodeHandle;

use crate::node::Node;

#[derive(Debug, Default)]
pub(crate) struct Graph {
    vect: Vec<u8>,
}

impl Graph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { vect: Vec::with_capacity(cap) }
    }

    #[inline(always)]
    pub fn push(&mut self, node: Node) {
        let [discr, left, right] = match node {
            Node::BeginSum => {
                self.vect.push(0);
                return;
            }
            Node::EndSum => {
                self.vect.push(1);
                return;
            }
            Node::Product { action, factors } => {
                [2, action, factors.0]
            }
            Node::Rule { left_factor, right_factor } => {
                [3, left_factor.0, right_factor.0]
            }
            Node::Leaf { symbol, values } => {
                [4, symbol.usize() as u32, values]
            }
            Node::NullingLeaf { symbol } => {
                [5, symbol.usize() as u32, 0]
            }
        };
        let f = |mut x: u32| {
            let slice = u32::to_le_bytes(x);
            let mut i = 0;
            while x != 0 {
                i += 1;
                x >>= 8;
            }
            (i.max(1u8), slice)
        };
        let (l_len, slice_left) = f(left);
        let (r_len, slice_right) = f(right);
        let head = (discr as u8) | (l_len.saturating_sub(1) << 3) | (r_len.saturating_sub(1) << 5);
        let mut main_slice = [head, 0, 0, 0, 0, 0, 0, 0, 0];
        main_slice[1.. 1 + l_len as usize].copy_from_slice(&slice_left[0..l_len as usize]);
        main_slice[1 + l_len as usize..1 + l_len as usize + r_len as usize].copy_from_slice(&slice_right[0..r_len as usize]);
        self.vect.extend(
            main_slice[0 .. 1 + l_len as usize + r_len as usize].into_iter()
        );
    }

    pub fn len(&self) -> u32 {
        self.vect.len() as u32
    }

    pub fn get(&self, index: usize) -> Node {
        let head = self.vect[index];
        let discr = head & 0b111;
        let l_len = ((head >> 3) & 0b11) + (discr > 1) as u8;
        let r_len = ((head >> 5) & 0b11) + (discr > 1) as u8;
        let mut left_slice = [0; 4];
        let mid = index + 1 + l_len as usize;
        left_slice[0 .. l_len as usize].copy_from_slice(&self.vect[index + 1 .. mid]);
        let &left: &u32 = from_bytes(&left_slice[..]);
        let mut right_slice = [0; 4];
        right_slice[0 .. r_len as usize].copy_from_slice(&self.vect[mid .. mid + r_len as usize]);
        let &right: &u32 = from_bytes(&right_slice[..]);
        match discr {
            0 => Node::BeginSum,
            1 => Node::EndSum,
            2 => Node::Product { action: left, factors: NodeHandle(right) },
            3 => Node::Rule { left_factor: NodeHandle(left), right_factor: NodeHandle(right) },
            4 => Node::Leaf { symbol: Symbol::from_raw(left), values: right },
            5 => Node::NullingLeaf { symbol: Symbol::from_raw(left) },
            _ => unreachable!()
        }
    }

    pub fn get_node_size(&self, index: usize) -> u32 {
        let head = self.vect[index];
        let discr = head & 0b111;
        if discr < 2 { return 1; }
        let l_len = ((head >> 3) & 0b11) + 1;
        let r_len = ((head >> 5) & 0b11) + 1;
        1 + l_len as u32 + r_len as u32
    }

    pub fn push_expanded(&mut self, node: Node) {
        let [discr, left, right] = match node {
            Node::BeginSum => {
                self.vect.push(0);
                return;
            }
            Node::EndSum => {
                self.vect.push(1);
                return;
            }
            Node::Product { action, factors } => {
                [2, action, factors.0]
            }
            Node::Rule { left_factor, right_factor } => {
                [3, left_factor.0, right_factor.0]
            }
            Node::Leaf { symbol, values } => {
                [4, symbol.usize() as u32, values]
            }
            Node::NullingLeaf { symbol } => {
                [5, symbol.usize() as u32, 0]
            }
        };
        self.vect.push(discr as u8 | 0b1111000);
        self.vect.extend(bytes_of(&left));
        self.vect.extend(bytes_of(&right));
    }
}
