use std::cell::Cell;
use std::hint;

use cfg::symbol::Symbol;

pub use self::Node::*;
use self::Tag::*;

pub struct Graph {
    vec: Vec<u8>,
}

impl Graph {
    fn push(node: Node) {
        let position = self.vec.len() as u32;
        let tag = node.classify();
        let result = match (node, tag) {
            (Sum { nonterminal, count }, SmallSumTag) => {
                NodeRepr {
                    small_sum: SmallSumRepr {
                        count: count as u8,
                        nonterminal: nonterminal.usize() as u8,
                    }
                }
            }
            (Sum { nonterminal, count }, SumTag) => {
                NodeRepr {
                    sum: SumRepr {
                        count,
                        nonterminal,
                    }
                }
            }
            (Product { left_node, right_node, action }, SmallLinkTag) => {
                NodeRepr {
                    small_link: SmallLinkRepr {
                        distance: (position - left_node.0) as u8,
                        action: action as u8,
                    }
                }
            }
            (Product { left_node, right_node, action }, MediumLinkTag) => {
                NodeRepr {
                    medium_link: MediumLinkRepr {
                        distance: (position - left_node.0) as u16,
                        action: action as u16,
                    }
                }
            }
            (Product { left_node, right_node: Some(right), action }, SmallProductTag) => {
                NodeRepr {
                    small_product: SmallProductRepr {
                        right_distance: (position - right.0) as u8,
                        left_distance: (position - left_node.0) as u8,
                        action: action as u16,
                    }
                }                
            }
            (Product { left_node, right_node, action }, ProductTag) => {
                NodeRepr {
                    product: ProductRepr {
                        action,
                        left_node,
                        right_node: right_node.unwrap_or(NULL_HANDLE),
                    }
                }
            }
            (NullingLeaf { symbol }, SmallNullingLeafTag) => {
                NodeRepr {
                    small_nulling_leaf: SmallNullingLeafRepr {
                        symbol: symbol.usize() as u16,
                    }
                }
            }
            (NullingLeaf { symbol }, LeafTag) => {
                NodeRepr {
                    leaf: LeafRepr {
                        symbol,
                        values: 0,
                    },
                }
            }
            (Evaluated { symbol, values: 0 }, SmallLeafTag) => {
                NodeRepr {
                    small_leaf: SmallLeafRepr {
                        symbol: symbol.usize() as u16,
                    }
                }
            }
            (Evaluated { symbol, values }, LeafTag) => {
                NodeRepr {
                    leaf: LeafRepr {
                        symbol,
                        values,
                    }
                }
            }
            _ => unreachable!()
        };
        result.fields[0] |= tag.to_u16();
        self.vec.extend_from_slice(&result.fields[.. tag.size()]);
    }

    fn get(&self, handle: NodeHandle) -> Node {
        self.iter_from(handle).next().unwrap()
    }

    fn iter_from(&self, handle: NodeHandle) -> Iter {
        Iter {
            vec: &self.vec[..],
            handle,
        }
    }
}

struct Iter {
    vec: &[u16],
    handle: NodeHandle,
}

impl Iterator for Iter {
    type Item = 
}

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone, Debug)]
pub enum Node {
    Sum {
        /// 8 bytes.
        /// Invariant: count > 1.
        /// Invariant: This node can only be directly followed by `Product`.
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

union NodeRepr {
    fields: [u16; 6],
    small_sum: SmallSumRepr,
    small_link: SmallLinkRepr,
    medium_link: MediumLinkRepr,
    small_product: SmallProductRepr,
    small_leaf: SmallLeafRepr,
    sum: SumRepr,
    product: ProductRepr,
    leaf: LeafRepr,
}

struct SmallSumRepr {
    count: u8,
    nonterminal: u8,
}

struct SumRepr {
    count: u32,
    nonterminal: Symbol,
}

struct SmallLinkRepr {
    distance: u8,
    action: u8,
}

struct MediumLinkRepr {
    distance: u16,
    action: u16,
}

struct SmallProductRepr {
    right_distance: u8,
    left_distance: u8,
    action: u16,
}

struct ProductRepr {
    action: u32,
    left_node: NodeHandle,
    right_node: NodeHandle,
}

struct SmallNullingLeafRepr {
    symbol: u16,
}
            
struct LeafRepr {
    symbol: Symbol,
    values: u32,
}
            
struct SmallLeafRepr {
    symbol: u16,
}

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone)]
union CompactField {
    // // small sum
    // count: u5,
    // nonterminal: u8,
    // //small link
    // factor: u5, (relative)
    // action: u8,
    // //medium link
    // factor: u5 + u8,
    // action: u16,
    // //small product
    // right_factor: u5,
    // left_factor: u8,
    // action: u16,
    // //small leaf
    // symbol: u4 + u8,

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

#[derive(Copy, Clone)]
enum Tag {
    SmallSumTag = 0b000 << TAG_BIT,
    SmallLinkTag = 0b001 << TAG_BIT,
    MediumLinkTag = 0b010 << TAG_BIT
    SmallProductTag = 0b011 << TAG_BIT,
    SmallLeafTag = 0b100 << TAG_BIT,
    // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
    SmallNullingLeafTag = 0b1001 << (TAG_BIT - 1),
    LeafTag = 0b101 << TAG_BIT,
    SumTag = 0b110 << TAG_BIT,
    ProductTag = 0b111 << TAG_BIT,
}

impl Tag {
    #[inline]
    fn from_u16(num: u16) -> Option<Self> {
        let n = num & TAG_MASK;
        if n == LeafTag.to_u16() {
            Some(LeafTag)
        } else if n == SumTag.to_u16() {
            Some(SumTag)
        } else if n == ProductTag.to_u16() {
            Some(ProductTag)
        } else if n == SmallSumTag.to_u16() {
            Some(SmallSumTag)
        } else if n == SmallLinkTag.to_u16() {
            Some(SmallLinkTag)
        } else if n == MediumLinkTag.to_u16() {
            Some(MediumLinkTag)
        } else if n == SmallProductTag.to_u16() {
            Some(SmallProductTag)
        } else if n == SmallLeafTag.to_u16() {
            let n = num & SMALL_LEAF_TAG_MASK;
            if n == SmallLeafTag.to_u16() {
                Some(SmallLeafTag)
            } else if n == SmallNullingLeafTag.to_u16() {
                Some(SmallNullingLeafTag)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    fn to_u16(self) -> u16 {
        match self {
            SmallSumTag => 0b000 << TAG_BIT,
            SmallLinkTag => 0b001 << TAG_BIT,
            MediumLinkTag => 0b010 << TAG_BIT
            SmallProductTag => 0b011 << TAG_BIT,
            SmallLeafTag => 0b100 << TAG_BIT,
            // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
            SmallNullingLeafTag => 0b1001 << (TAG_BIT - 1),
            LeafTag => 0b101 << TAG_BIT,
            SumTag => 0b110 << TAG_BIT,
            ProductTag => 0b111 << TAG_BIT,
        }
    }

    fn mask(self) -> u16 {
        match self {
            SmallSumTag => TAG_MASK,
            SmallLinkTag => TAG_MASK,
            MediumLinkTag => TAG_MASK
            SmallProductTag => TAG_MASK,
            SmallLeafTag => SMALL_LEAF_TAG_MASK,
            // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
            SmallNullingLeafTag => SMALL_LEAF_TAG_MASK,
            LeafTag => TAG_MASK,
            SumTag => TAG_MASK,
            ProductTag => TAG_MASK,
        }
    }

    fn size(self) -> usize {
        match self {
            SmallSumTag => 1,
            SmallLinkTag => 1,
            MediumLinkTag => 2
            SmallProductTag => 2,
            SmallLeafTag => 1,
            SmallNullingLeafTag => 1,
            LeafTag => 4,
            SumTag => 4,
            ProductTag => 6,
        }
    }
}

const TAG_BIT: usize = 5 + 8;
const TAG_MASK: u32 = 0b111 << TAG_BIT;
const SMALL_LEAF_TAG_MASK: u32 = 0b1111 << (TAG_BIT - 1);
const NULL_VALUES: u32 = 0xFFFF_FFFF;
const NULL_HANDLE: NodeHandle = NodeHandle(0xFFFF_FFFF);
pub(super) const NULL_ACTION: u32 = !TAG_MASK;

impl Node {
    fn classify(&self, position: u32) -> Tag {
        match self {
            Product { left_factor, right_factor, action } => {
                match right_factor {
                    Some(handle) => {
                        if position - handle.0 < (1 << 5) && position - left_factor.0 < (1 << 8) && action < (1 << 16) {
                            SmallProductTag
                        } else {
                            ProductTag
                        }
                    }
                    None => {
                        if position - left_factor.0 < (1 << 5) && action < (1 << 8) {
                            SmallLinkTag
                        } else if position - left_factor.0 < (1 << (5 + 8)) && action < (1 << 16) {
                            MediumLinkTag
                        } else {
                            ProductTag
                        }
                    }
                }
            }
            NullingLeaf { symbol } => {
                if symbol.usize() < (1 << (4 + 8)) {
                    SmallNullingLeafTag
                } else {
                    LeafTag
                }
            }
            Evaluated { symbol, values: 0 } => {
                if symbol.usize() < (1 << (4 + 8)) {
                    SmallLeafTag
                } else {
                    LeafTag
                }
            }
            Evaluated { _symbol, values } => {
                LeafTag
            }
            Sum { nonterminal, count } => {
                if count < (1 << 5) && nonterminal.usize() < (1 << 8) {
                    SmallSumTag
                } else {
                    SumTag
                }
            }
        }
    }

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

    #[inline]
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
    pub(super) fn set(&self, node: Node) {
        self.cell.set(node.compact().cell.get());
    }

    #[inline]
    pub(super) fn expand(&self) -> Node {
        let mut fields = self.cell.get();
        unsafe {
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
}

impl NodeHandle {
    #[inline]
    pub(in super) fn nulling(symbol: Symbol) -> Self {
        NodeHandle(symbol.usize() as u32)
    }

    #[inline]
    pub(in super) fn usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    fn to_option(self) -> Option<NodeHandle> {
        if self == NULL_HANDLE {
            None
        } else {
            Some(self)
        }
    }
}

#[inline]
unsafe fn unwrap_unchecked<T>(opt: Option<T>) -> T {
    match opt {
        Some(val) => val,
        None => hint::unreachable_unchecked(),
    }
}

// #[inline]
// unsafe fn set_tag(fields: &mut u16, tag: Tag) {
//     *fields |= tag.to_u16();
// }

#[inline]
unsafe fn get_and_erase_tag(field: u16) -> (Tag, u16) {
    let tag = unwrap_unchecked(Tag::from_u16(extract_tag));
    (tag, field & !tag.mask())
}
