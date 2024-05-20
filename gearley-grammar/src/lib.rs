use bit_matrix::row::BitSlice;
use cfg_symbol::Symbol;
use std::num::NonZeroU32;
use serde_derive::{Serialize, Deserialize};

type Dot = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct PredictionTransition {
    pub symbol: Symbol,
    pub dot: Dot,
    pub is_unary: bool,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum MaybePostdot {
    Binary(Symbol),
    Unary,
}

pub type ExternalDottedRule = (u32, u32);
pub type Event = (EventId, MinimalDistance);

pub type ExternalOrigin = Option<u32>;
pub type EventId = Option<NonZeroU32>;
pub type MinimalDistance = Option<NonZeroU32>;
pub type NullingEliminated = Option<(Symbol, bool)>;
pub type NullingIntermediateRule = (Symbol, Symbol, Symbol);

pub trait Grammar {
    fn eof(&self) -> Symbol;

    fn lr_set(&self, dot: Dot) -> &BitSlice;

    fn prediction_row(&self, sym: Symbol) -> &BitSlice;

    fn num_syms(&self) -> usize;

    fn num_rules(&self) -> usize;

    fn start_sym(&self) -> Symbol;

    fn externalized_start_sym(&self) -> Symbol;

    fn has_trivial_derivation(&self) -> bool;

    fn nulling(&self, pos: u32) -> NullingEliminated;

    fn events(&self) -> (&[Event], &[Event]);

    fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3];

    fn get_rhs1(&self, dot: Dot) -> Option<Symbol>;

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot;

    fn rhs1(&self) -> &[Option<Symbol>];

    fn get_lhs(&self, dot: Dot) -> Symbol;

    fn external_origin(&self, dot: Dot) -> ExternalOrigin;

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule];

    fn completions(&self, sym: Symbol) -> &[PredictionTransition];

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol>;

    fn to_external(&self, symbol: Symbol) -> Symbol;

    fn max_nulling_symbol(&self) -> Option<usize>;

    fn dot_before_eof(&self) -> Dot;

    fn useless_symbol(&self) -> Symbol;
}
