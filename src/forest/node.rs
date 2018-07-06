use std::cell::Cell;
use std::convert::Into;
use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

use cfg::symbol::Symbol;

pub use self::Node::*;
use self::Tag::*;

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone, Debug)]
pub enum Node {
    Sum {
        /// 8 bytes.
        /// Invariant: count > 0.
        /// Invariant: This node can directly follow only `Product` or `Link`.
        nonterminal: Symbol,
        count: u32,
    },
    Product {
        /// 12+ bytes.
        action: u32,
        left_factor: NodeHandle,
        right_factor: Option<NodeHandle>,
    },
    NullingLeaf {
        /// 4 bytes.
        symbol: Symbol,
    },
    Evaluated {
        /// 8 bytes.
        symbol: Symbol,
        values: u32,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeHandle(pub(in super) u32);

#[derive(Clone)]
pub struct CompactNode {
    cell: Cell<[CompactField; 3]>,
}

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone)]
union CompactField {
    // sum
    nonterminal: Symbol,
    count: u32,

    // product
    action: u32,
    factor: NodeHandle,
    // right_factor: NodeHandle,

    // leaf
    symbol: Symbol,
    values: u32,

    // tag
    tag: u32,
}

#[derive(FromPrimitive, ToPrimitive)]
enum Tag {
    LeafTag = 0b00 << TAG_BIT,
    SumTag = 0b01 << TAG_BIT,
    ProductTag = 0b10 << TAG_BIT,
}

const TAG_BIT: usize = 30;
const TAG_MASK: u32 = 0b11 << TAG_BIT;
const NULL_VALUES: u32 = 0xFFFF_FFFF;
const NULL_HANDLE: NodeHandle = NodeHandle(0xFFFF_FFFF);
pub(super) const NULL_ACTION: u32 = !TAG_MASK;

impl Node {
    #[inline]
    pub(super) fn compact(self) -> CompactNode {
        let mut fields = match self {
            Product { left_factor, right_factor, action } => {
                let right_factor = right_factor.unwrap_or(NULL_HANDLE);
                [
                    CompactField { action },
                    CompactField { factor: left_factor },
                    CompactField { factor: right_factor },
                ]
            }
            Sum { nonterminal, count } => {
                [
                    CompactField { nonterminal },
                    CompactField { count },
                    CompactField { tag: 0 },
                ]
            }
            NullingLeaf { symbol } => {
                [
                    CompactField { symbol },
                    CompactField { values: NULL_VALUES },
                    CompactField { tag: 0 },
                ]
            }
            Evaluated { symbol, values } => {
                [
                    CompactField { symbol },
                    CompactField { values },
                    CompactField { tag: 0 },
                ]
            }
        };
        unsafe {
            set_tag(&mut fields, self.tag());
        }
        CompactNode {
            cell: Cell::new(fields),
        }
    }

    fn tag(&self) -> Tag {
        match self {
            Product { .. } => ProductTag,
            Sum { .. } => SumTag,
            NullingLeaf { .. } | Evaluated { .. } => LeafTag,
        }
    }
}

impl CompactNode {
    #[inline]
    pub(super) fn get(&self) -> Node {
        unsafe {
            self.expand()
        }
    }

    #[inline]
    pub(super) fn set(&self, node: Node) {
        self.cell.set(node.compact().cell.get());
    }

    #[inline]
    unsafe fn expand(&self) -> Node {
        let mut fields = self.cell.get();
        let tag = get_and_erase_tag(&mut fields);
        match tag {
            LeafTag => {
                if fields[1].values == NULL_VALUES {
                    NullingLeaf {
                        symbol: fields[0].symbol,
                    }
                } else {
                    Evaluated {
                        symbol: fields[0].symbol,
                        values: fields[1].values,
                    }
                }
            }
            ProductTag => {
                Product {
                    action: fields[0].action,
                    left_factor: fields[1].factor,
                    right_factor: fields[2].factor.to_option(),
                }
            }
            SumTag => {
                Sum {
                    nonterminal: fields[0].nonterminal,
                    count: fields[1].count,
                }
            }
        }
    }
}

impl NodeHandle {
    pub(in super) fn nulling(symbol: Symbol) -> Self {
        NodeHandle(symbol.usize() as u32)
    }

    #[inline]
    pub(in super) fn usize(self) -> usize {
        self.0 as usize
    }

    fn to_option(self) -> Option<NodeHandle> {
        if self == NULL_HANDLE {
            None
        } else {
            Some(self)
        }
    }
}

unsafe fn set_tag(fields: &mut [CompactField; 3], tag: Tag) {
    fields[0].tag |= ToPrimitive::to_u32(&tag).unwrap();
}

unsafe fn get_and_erase_tag(fields: &mut [CompactField; 3]) -> Tag {
    let &mut CompactField { ref mut tag } = &mut fields[0];
    let extract_tag = *tag & TAG_MASK;
    *tag = *tag & !TAG_MASK;
    FromPrimitive::from_u32(extract_tag).unwrap()
}
