use bit_matrix::row::BitSlice;
use cfg_symbol::{Symbol, SymbolPrimitive};
use cfg_history::earley::{Event, ExternalDottedRule, NullingEliminated, ExternalOrigin, MinimalDistance};
use miniserde::{Serialize, Deserialize};

type Dot = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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

pub type NullingIntermediateRule<S> = [S; 3];

pub trait Grammar {
    fn eof(&self) -> Symbol;

    fn lr_set(&self, dot: Dot) -> &BitSlice;

    fn prediction_row(&self, sym: Symbol) -> &BitSlice;

    fn num_syms(&self) -> usize;

    fn num_gensyms(&self) -> usize;

    fn num_rules(&self) -> usize;

    fn start_sym(&self) -> Symbol;

    fn externalized_start_sym(&self) -> Symbol;

    fn has_trivial_derivation(&self) -> bool;

    fn nulling(&self, pos: u32) -> NullingEliminated<Symbol>;

    fn events(&self) -> (&[Event], &[Event]);

    fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3];

    fn get_rhs1(&self, dot: Dot) -> Option<Symbol>;

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot<Symbol>;

    fn rhs1(&self) -> &[Option<Symbol>];

    fn get_lhs(&self, dot: Dot) -> Symbol;

    fn external_origin(&self, dot: Dot) -> ExternalOrigin;

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule<Symbol>];

    fn completions(&self, sym: Symbol) -> &[PredictionTransition];

    fn gen_completion(&self, sym: Symbol) -> PredictionTransition;

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol>;

    fn to_external(&self, symbol: Symbol) -> Symbol;

    fn max_nulling_symbol(&self) -> Option<usize>;

    fn dot_before_eof(&self) -> Dot;

    fn useless_symbol(&self) -> Symbol;
}

impl<'a, G> Grammar for &'a G where G: Grammar {
    fn eof(&self) -> Symbol {
        (**self).eof()
    }

    fn lr_set(&self, dot: Dot) -> &BitSlice {
        (**self).lr_set(dot)
    }

    fn prediction_row(&self, sym: Symbol) -> &BitSlice {
        (**self).prediction_row(sym)
    }

    fn num_syms(&self) -> usize {
        (**self).num_syms()
    }

    fn num_gensyms(&self) -> usize {
        (**self).num_gensyms()
    }

    fn num_rules(&self) -> usize {
        (**self).num_rules()
    }

    fn start_sym(&self) -> Symbol {
        (**self).start_sym()
    }

    fn externalized_start_sym(&self) -> Symbol {
        (**self).externalized_start_sym()
    }

    fn has_trivial_derivation(&self) -> bool {
        (**self).has_trivial_derivation()
    }

    fn nulling(&self, pos: u32) -> NullingEliminated<Symbol> {
        (**self).nulling(pos)
    }

    fn events(&self) -> (&[Event], &[Event]) {
        (**self).events()
    }

    fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3] {
        (**self).trace()
    }

    fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        (**self).get_rhs1(dot)
    }

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot<Symbol> {
        (**self).get_rhs1_cmp(dot)
    }

    fn rhs1(&self) -> &[Option<Symbol>] {
        (**self).rhs1()
    }

    fn get_lhs(&self, dot: Dot) -> Symbol {
        (**self).get_lhs(dot)
    }

    fn external_origin(&self, dot: Dot) -> ExternalOrigin {
        (**self).external_origin(dot)
    }

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule<Symbol>] {
        (**self).eliminated_nulling_intermediate()
    }

    fn completions(&self, sym: Symbol) -> &[PredictionTransition] {
        (**self).completions(sym)
    }

    fn gen_completion(&self, sym: Symbol) -> PredictionTransition {
        (**self).gen_completion(sym)
    }

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol> {
        (**self).to_internal(symbol)
    }

    fn to_external(&self, symbol: Symbol) -> Symbol {
        (**self).to_external(symbol)
    }

    fn max_nulling_symbol(&self) -> Option<usize> {
        (**self).max_nulling_symbol()
    }

    fn dot_before_eof(&self) -> Dot {
        (**self).dot_before_eof()
    }

    fn useless_symbol(&self) -> Symbol {
        (**self).useless_symbol()
    }
}
