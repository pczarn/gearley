use cfg::Symbol;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NodeHandle(pub(crate) u32);

pub(super) const NULL_HANDLE: NodeHandle = NodeHandle(0xFFFF_FFFF);

impl NodeHandle {
    #[inline]
    pub(super) fn nulling(symbol: Symbol) -> Self {
        NodeHandle(symbol.usize() as u32)
    }

    #[inline]
    pub(super) fn usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub(super) fn to_option(self) -> Option<NodeHandle> {
        if self == NULL_HANDLE {
            None
        } else {
            Some(self)
        }
    }
}
