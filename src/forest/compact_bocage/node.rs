use std::cell::Cell;
use std::hint;
use std::mem;

use cfg::symbol::Symbol;

pub use self::Node::*;
use self::Tag::*;
use forest::node_handle::{NodeHandle, NULL_HANDLE};

pub struct Graph {
    pub(crate) vec: Vec<Cell<u16>>,
    pub(crate) nulling_leaf_limit: u32,
}

impl Graph {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Graph {
            vec: Vec::with_capacity(capacity),
            nulling_leaf_limit: 0,
        }
    }

    pub(crate) fn push(&mut self, node: Node) -> NodeHandle {
        let position = self.vec.len() as u32;
        // println!("push @{} {:?}", position, node);
        // let (node_repr, size) = node.to_repr(position);
        // unsafe {
        //     self.vec
        //         .extend(node_repr.fields[..size].iter().cloned().map(Cell::new));
        // }

        let tag = node.classify(position);
        unsafe {
            match (node, tag.erase_first()) {
                (
                    Product {
                        left_factor,
                        right_factor: None,
                        action,
                        ..
                    },
                    SmallLinkTag,
                ) => {
                    let mut result = NodeRepr {
                        small_link: SmallLinkRepr {
                            distance: (position - left_factor.0) as u16,
                            action: action as u16,
                        },
                    };
                    result.fields[0] |= tag.to_u16();
                    self.vec.push(Cell::new(result.fields[0]));
                    self.vec.push(Cell::new(result.fields[1]));
                },
                (
                    Product {
                        left_factor,
                        right_factor: Some(right),
                        action,
                        ..
                    },
                    SmallProductTag,
                ) => {
                    let mut result = NodeRepr {
                        small_product: SmallProductRepr {
                            right_distance: (position - right.0) as u8,
                            left_distance: (position - left_factor.0) as u8,
                            action: action as u16,
                        },
                    };
                    result.fields[0] |= tag.to_u16();
                    self.vec.push(Cell::new(result.fields[0]));
                    self.vec.push(Cell::new(result.fields[1]));
                }
                (
                    Product {
                        left_factor,
                        right_factor,
                        action,
                        ..
                    },
                    ProductTag,
                ) => {
                    let mut result = NodeRepr {
                        product: ProductRepr {
                            upper_action: (action >> 16) as u16,
                            lower_action: action as u16,
                            left_factor,
                            right_factor: right_factor.unwrap_or(NULL_HANDLE),
                        },
                    };
                    result.fields[0] |= tag.to_u16();
                    self.vec.extend(result.fields[..].iter().map(|&f| Cell::new(f)));
                    // self.vec.push(Cell::new(result.fields[0]));
                    // self.vec.push(Cell::new(result.fields[1]));
                    // self.vec.push(Cell::new(result.fields[2]));
                    // self.vec.push(Cell::new(result.fields[3]));
                    // self.vec.push(Cell::new(result.fields[4]));
                    // self.vec.push(Cell::new(result.fields[5]));
                }
                (NullingLeaf { symbol }, LeafTag) | (Evaluated { symbol }, LeafTag) => {
                    let mut result = NodeRepr {
                        leaf: LeafRepr { symbol },
                    };
                    result.fields[0] |= tag.to_u16();
                    self.vec.push(Cell::new(result.fields[0]));
                    self.vec.push(Cell::new(result.fields[1]));
                }
                _ => unreachable!(),
            }
        }
        NodeHandle(position)
    }

    // pub(crate) fn set_up(&mut self, mut handle: NodeHandle, node: Node) {
    //     let (node_repr, size) = node.to_repr(handle.0);
    //     let mut current_handle = handle;
    //     while current_handle.usize() < handle.usize() + size {
    //         let current_node = self.get(current_handle);
    //         self.push(current_node);
    //         current_handle.0 += current_node.classify(current_handle.0).size() as u32;
    //     }
    //     for i in 0..size {
    //         unsafe {
    //             self.vec[handle.usize() + i].set(node_repr.fields[i]);
    //         }
    //     }
    //     handle.0 += size as u32;
    //     while handle.0 < current_handle.0 {
    //         self.vec[handle.usize()].set(NopTag.to_u16());
    //         handle.0 += 1;
    //     }
    // }

    pub(crate) fn get(&self, handle: NodeHandle) -> Node {
        self.iter_from(handle).next().unwrap()
    }

    pub(crate) fn iter_from(&self, handle: NodeHandle) -> Iter {
        Iter {
            vec: &self.vec[..],
            handle,
            nulling_leaf_limit: self.nulling_leaf_limit,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Iter<'a> {
    pub(crate) vec: &'a [Cell<u16>],
    pub(crate) handle: NodeHandle,
    pub(crate) nulling_leaf_limit: u32,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        unsafe {
            let head = if let Some(head) = self.vec.get(self.handle.usize()).cloned() {
                head.get()
            } else {
                return None;
            };
            let (tag, head) = get_and_erase_tag(head);
            let mut node_repr = NodeRepr { fields: [0; 6] };
            node_repr.fields[0] = head;
            let slice = &self.vec[self.handle.usize() + 1..self.handle.usize() + tag.size()];
            for (i, val) in slice.iter().enumerate() {
                node_repr.fields[1 + i] = val.get();
            }
            let result = node_repr.expand(tag, self.handle.0, self.nulling_leaf_limit);
            self.handle.0 += tag.size() as u32;
            Some(result)
        }
    }
}

impl<'a> Iter<'a> {
    #[inline]
    pub(crate) fn peek(&mut self) -> Option<Node> {
        self.clone().next()
    }
}

// Node variants `Sum`/`Product` are better known in literature as `OR`/`AND`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Node {
    Product {
        /// 12+ bytes.
        action: u32,
        left_factor: NodeHandle,
        right_factor: Option<NodeHandle>,
        first: bool,
    },
    NullingLeaf {
        /// 4 bytes.
        symbol: Symbol,
    },
    Evaluated {
        /// 4 bytes.
        symbol: Symbol,
    },
}

#[derive(Clone, Copy)]
union NodeRepr {
    fields: [u16; 6],
    small_link: SmallLinkRepr,
    // medium_link: MediumLinkRepr,
    small_product: SmallProductRepr,
    // small_leaf: SmallLeafRepr,
    product: ProductRepr,
    leaf: LeafRepr,
}

// #[derive(Clone, Copy)]
// struct SmallSumRepr {
//     nonterminal: u8,
//     // smaller (big end position)
//     count: u8,
// }

// #[derive(Clone, Copy)]
// struct SumRepr {
//     count: u32,
//     nonterminal: Symbol,
// }

// #[derive(Clone, Copy)]
// struct SmallLinkRepr {
//     action: u8,
//     // smaller (big end position)
//     distance: u8,
// }

#[derive(Clone, Copy)]
struct SmallLinkRepr {
    distance: u16,
    action: u16,
}

#[derive(Clone, Copy)]
struct SmallProductRepr {
    left_distance: u8,
    // smaller (big end position)
    right_distance: u8,
    action: u16,
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct ProductRepr {
    upper_action: u16,
    lower_action: u16,
    left_factor: NodeHandle,
    right_factor: NodeHandle,
}

// #[derive(Clone, Copy)]
// struct SmallNullingLeafRepr {
//     symbol: u16,
// }

#[derive(Clone, Copy)]
struct LeafRepr {
    symbol: Symbol,
}

// #[derive(Clone, Copy)]
// struct SmallLeafRepr {
//     symbol: u16,
// }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum Tag {
    SmallProductTag,
    SmallProductTagFirst,
    SmallLinkTag,
    SmallLinkTagFirst,
    // SmallLeafTag = 0b011 << TAG_BIT,
    // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
    // SmallNullingLeafTag = 0b100 << TAG_BIT,
    ProductTag,
    ProductTagFirst,
    LeafTag,
}

impl Tag {
    #[inline]
    fn from_u16(num: u16) -> Option<Self> {
        let n = num & TAG_MASK;
        let tags = &[
            SmallProductTag,
            SmallProductTagFirst,
            SmallLinkTag,
            SmallLinkTagFirst,
            ProductTag,
            ProductTagFirst,
            LeafTag,
        ];
        for &tag in tags {
            if n == tag.to_u16() {
                return Some(tag);
            }
        }
        None
    }

    #[inline]
    pub(super) fn to_u16(self) -> u16 {
        match self {
            SmallProductTag => 0b000 << TAG_BIT,
            SmallProductTagFirst => 0b001 << TAG_BIT,
            SmallLinkTag => 0b010 << TAG_BIT,
            SmallLinkTagFirst => 0b011 << TAG_BIT,
            ProductTag => 0b100 << TAG_BIT,
            // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
            ProductTagFirst => 0b101 << TAG_BIT,
            LeafTag => 0b110 << TAG_BIT,
            // ProductTag => 0b111 << TAG_BIT,
        }
    }

    #[inline]
    pub(super) fn erase_first(self) -> Tag {
        match self {
            SmallProductTag => SmallProductTag,
            SmallProductTagFirst => SmallProductTag,
            SmallLinkTag => SmallLinkTag,
            SmallLinkTagFirst => SmallLinkTag,
            ProductTag => ProductTag,
            // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
            ProductTagFirst => ProductTag,
            LeafTag => LeafTag,
            // ProductTag => 0b111 << TAG_BIT,
        }
    }

    #[inline]
    pub(super) fn is_first(self) -> bool {
        match self {
            SmallProductTag => false,
            SmallProductTagFirst => true,
            SmallLinkTag => false,
            SmallLinkTagFirst => true,
            ProductTag => false,
            // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
            ProductTagFirst => true,
            LeafTag => true,
            // ProductTag => 0b111 << TAG_BIT,
        }
    }

    #[inline]
    fn mask(self) -> u16 {
        TAG_MASK
        // match self {
        //     SmallSumTag => TAG_MASK,
        //     SmallLinkTag => TAG_MASK,
        //     MediumLinkTag => TAG_MASK,
        //     SmallProductTag => TAG_MASK,
        //     // SmallLeafTag => SMALL_LEAF_TAG_MASK,
        //     SmallLeafTag => TAG_MASK,
        //     // SmallNonnullingLeaf = 0b1000 << (TAG_BIT - 1),
        //     // SmallNullingLeafTag => SMALL_LEAF_TAG_MASK,
        //     SmallNullingLeafTag => TAG_MASK,
        //     LeafTag => TAG_MASK,
        //     SumTag => TAG_MASK,
        //     ProductTag => TAG_MASK,
        //     NopTag => TAG_MASK,
        // }
    }

    #[inline]
    pub(super) fn size(self) -> usize {
        let bytes = match self {
            SmallLinkTag | SmallLinkTagFirst => mem::size_of::<SmallLinkRepr>(),
            // MediumLinkTag => 2,
            SmallProductTag | SmallProductTagFirst => mem::size_of::<SmallProductRepr>(),
            // SmallLeafTag => 1,
            // SmallNullingLeafTag => 1,
            LeafTag => mem::size_of::<LeafRepr>(),
            ProductTag | ProductTagFirst => mem::size_of::<ProductRepr>(),
        };
        bytes / 2
    }
}

const TAG_BIT: usize = 5 + 8;
// const FIRST_BIT: usize = 4 + 8;
const TAG_MASK: u16 = 0b111 << TAG_BIT;
// const FIRST_MASK: u16 = 1 << FIRST_BIT;
// const SMALL_LEAF_TAG_MASK: u16 = 0b1111 << (TAG_BIT - 1);
pub(super) const NULL_ACTION: u32 = !((TAG_MASK as u32) << 16);

impl NodeRepr {
    fn expand(self, tag: Tag, position: u32, nulling_leaf_limit: u32) -> Node {
        // let first = self.fields[0] & FIRST_MASK != 0;
        unsafe {
            match (self, tag.erase_first()) {
                (
                    NodeRepr {
                        small_link: SmallLinkRepr { distance, action },
                    },
                    SmallLinkTag,
                ) => {
                    // println!("FAULT {} {}", position, distance);
                    Product {
                    action: action as u32,
                    left_factor: NodeHandle(position - distance as u32),
                    right_factor: None,
                    first: tag.is_first(),
                }},
                // (
                //     NodeRepr {
                //         medium_link: MediumLinkRepr { distance, action },
                //     },
                //     MediumLinkTag,
                // ) => Product {
                //     action: action as u32,
                //     left_factor: NodeHandle(position - distance as u32),
                //     right_factor: None,
                //     first,
                // },
                (
                    NodeRepr {
                        small_product:
                            SmallProductRepr {
                                right_distance,
                                left_distance,
                                action,
                            },
                    },
                    SmallProductTag,
                ) => Product {
                    action: action as u32,
                    left_factor: NodeHandle(position - left_distance as u32),
                    right_factor: Some(NodeHandle(position - right_distance as u32)),
                    first: tag.is_first(),
                },
                (
                    NodeRepr {
                        product:
                            ProductRepr {
                                upper_action,
                                lower_action,
                                left_factor,
                                right_factor,
                            },
                    },
                    ProductTag,
                ) => Product {
                    action: (upper_action as u32) << 16 | (lower_action as u32),
                    left_factor,
                    right_factor: right_factor.to_option(),
                    first: tag.is_first(),
                },
                // (
                //     NodeRepr {
                //         small_nulling_leaf: SmallNullingLeafRepr { symbol },
                //     },
                //     SmallNullingLeafTag,
                // ) => NullingLeaf {
                //     symbol: Symbol::from(symbol as u32),
                // },
                // (
                //     NodeRepr {
                //         small_leaf: SmallLeafRepr { symbol },
                //     },
                //     SmallLeafTag,
                // ) => Evaluated {
                //     symbol: Symbol::from(symbol as u32),
                // },
                (
                    NodeRepr {
                        leaf: LeafRepr { symbol },
                    },
                    LeafTag,
                ) => {
                    if position < nulling_leaf_limit {
                        NullingLeaf { symbol }
                    } else {
                        Evaluated { symbol }
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

impl Node {
    #[inline]
    fn to_repr(self, position: u32) -> (NodeRepr, usize) {
        let tag = self.classify(position);
        unsafe {
            let mut result = match (self, tag.erase_first()) {
                (
                    Product {
                        left_factor,
                        right_factor: None,
                        action,
                        ..
                    },
                    SmallLinkTag,
                ) => {
                    NodeRepr {
                    small_link: SmallLinkRepr {
                        distance: (position - left_factor.0) as u16,
                        action: action as u16,
                    },
                }},
                (
                    Product {
                        left_factor,
                        right_factor: Some(right),
                        action,
                        ..
                    },
                    SmallProductTag,
                ) => NodeRepr {
                    small_product: SmallProductRepr {
                        right_distance: (position - right.0) as u8,
                        left_distance: (position - left_factor.0) as u8,
                        action: action as u16,
                    },
                },
                (
                    Product {
                        left_factor,
                        right_factor,
                        action,
                        ..
                    },
                    ProductTag,
                ) => NodeRepr {
                    product: ProductRepr {
                        upper_action: (action >> 16) as u16,
                        lower_action: action as u16,
                        left_factor,
                        right_factor: right_factor.unwrap_or(NULL_HANDLE),
                    },
                },
                (NullingLeaf { symbol }, LeafTag) => NodeRepr {
                    leaf: LeafRepr { symbol },
                },
                (Evaluated { symbol }, LeafTag) => NodeRepr {
                    leaf: LeafRepr { symbol },
                },
                _ => unreachable!(),
            };
            result.fields[0] |= tag.to_u16();
            (result, tag.size())
        }
    }

    #[inline]
    pub(super) fn classify(self, position: u32) -> Tag {
        match self {
            Product {
                left_factor,
                right_factor,
                action,
                first,
            } => match right_factor {
                Some(handle) => {
                    if position >= handle.0
                        && position >= left_factor.0
                        && position - handle.0 < (1 << 5)
                        && position - left_factor.0 < (1 << 8)
                        && action < (1 << 16)
                    {
                        if first {
                            SmallProductTagFirst
                        } else {
                            SmallProductTag
                        }
                    } else {
                        if first {
                            ProductTagFirst
                        } else {
                            ProductTag
                        }
                    }
                }
                None => {
                    if position >= left_factor.0
                        && position - left_factor.0 < (1 << TAG_BIT)
                        && action < (1 << 16)
                    {
                        if first {
                            SmallLinkTagFirst
                        } else {
                            SmallLinkTag
                        }
                    } else {
                        if first {
                            ProductTagFirst
                        } else {
                            ProductTag
                        }
                    }
                }
            },
            NullingLeaf { symbol } | Evaluated { symbol } => {
                LeafTag
            }
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

#[inline]
unsafe fn get_and_erase_tag(field: u16) -> (Tag, u16) {
    let tag = unwrap_unchecked(Tag::from_u16(field));
    (tag, field & !tag.mask())
}
