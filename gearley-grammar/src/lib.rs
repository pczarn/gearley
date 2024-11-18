use bit_matrix::row::BitSlice;
use cfg_symbol::{Symbol, Symbolic};
use miniserde::{Serialize, Deserialize};

type Dot = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct PredictionTransition {
    pub symbol: Symbol,
    pub dot: Dot,
    pub is_unary: bool,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum MaybePostdot<S> {
    Binary(S),
    Unary,
}

pub type Id = Symbol;

pub type ExternalDottedRule = (u32, u32);
pub type Event = (EventId, MinimalDistance);

pub type ExternalOrigin = Option<Id>;
pub type EventId = Option<Id>;
pub type MinimalDistance = Option<Id>;
pub type NullingEliminated<S> = Option<(S, bool)>;
pub type NullingIntermediateRule<S> = [S; 3];

pub trait Grammar {
    type Symbol: Symbolic;

    fn eof(&self) -> Self::Symbol;

    fn lr_set(&self, dot: Dot) -> &BitSlice;

    fn prediction_row(&self, sym: Self::Symbol) -> &BitSlice;

    fn num_syms(&self) -> usize;

    fn num_rules(&self) -> usize;

    fn start_sym(&self) -> Self::Symbol;

    fn externalized_start_sym(&self) -> Self::Symbol;

    fn has_trivial_derivation(&self) -> bool;

    fn nulling(&self, pos: u32) -> NullingEliminated<Self::Symbol>;

    fn events(&self) -> (&[Event], &[Event]);

    fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3];

    fn get_rhs1(&self, dot: Dot) -> Option<Self::Symbol>;

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot<Self::Symbol>;

    fn rhs1(&self) -> &[Option<Self::Symbol>];

    fn get_lhs(&self, dot: Dot) -> Self::Symbol;

    fn external_origin(&self, dot: Dot) -> ExternalOrigin;

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule<Self::Symbol>];

    fn completions(&self, sym: Self::Symbol) -> &[PredictionTransition];

    fn to_internal(&self, symbol: Self::Symbol) -> Option<Self::Symbol>;

    fn to_external(&self, symbol: Self::Symbol) -> Self::Symbol;

    fn max_nulling_symbol(&self) -> Option<usize>;

    fn dot_before_eof(&self) -> Dot;

    fn useless_symbol(&self) -> Self::Symbol;
}
