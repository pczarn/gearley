use bit_matrix::row::BitSlice;
use cfg_symbol::Symbol;
use miniserde::{Deserialize, Serialize};

pub use cfg_history::earley::{
    EventAndDistance, ExternalDottedRule, ExternalOrigin, NullingEliminated,
};

type Dot = u32;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PredictionTransition {
    pub symbol: Symbol,
    pub dot: Dot,
    pub is_unary: bool,
}

pub type NullingIntermediateRule = [Symbol; 3];

pub trait Grammar {
    fn sof(&self) -> Symbol;

    fn eof(&self) -> Symbol;

    fn lhs_lr_set(&self, symbol: Symbol) -> &BitSlice;

    fn lookahead_set(&self, dot: Dot) -> &BitSlice;

    fn rhs1_or_lhs(&self, dot: Dot) -> Symbol;

    fn prediction_row(&self, sym: Symbol) -> &BitSlice;

    fn num_syms(&self) -> usize;

    fn num_gensyms(&self) -> usize;

    fn num_rules(&self) -> usize;

    fn start_sym(&self) -> Symbol;

    fn externalized_start_sym(&self) -> Symbol;

    fn has_trivial_derivation(&self) -> bool;

    fn nulling(&self, pos: u32) -> NullingEliminated;

    fn events(&self) -> (&[EventAndDistance], &[EventAndDistance]);

    fn trace(&self) -> [&[ExternalDottedRule]; 3];

    fn get_rhs1(&self, dot: Dot) -> Option<Symbol>;

    fn rhs1(&self) -> &[Option<Symbol>];

    fn get_lhs(&self, dot: Dot) -> Symbol;

    fn external_origin(&self, dot: Dot) -> ExternalOrigin;

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule];

    fn completions(&self, sym: Symbol) -> &[PredictionTransition];

    fn gen_completion(&self, sym: Symbol) -> [Option<PredictionTransition>; 2];

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol>;

    fn to_external(&self, symbol: Symbol) -> Symbol;

    fn dot_before_eof(&self) -> Dot;

    fn useless_symbol(&self) -> Symbol;

    fn forest_info(&self) -> ForestInfo;
}

impl<'a, G> Grammar for &'a G
where
    G: Grammar,
{
    fn sof(&self) -> Symbol {
        (**self).sof()
    }

    fn eof(&self) -> Symbol {
        (**self).eof()
    }

    fn lhs_lr_set(&self, symbol: Symbol) -> &BitSlice {
        (**self).lhs_lr_set(symbol)
    }

    fn lookahead_set(&self, dot: Dot) -> &BitSlice {
        (**self).lookahead_set(dot)
    }

    fn rhs1_or_lhs(&self, dot: Dot) -> Symbol {
        (**self).rhs1_or_lhs(dot)
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

    fn nulling(&self, pos: u32) -> NullingEliminated {
        (**self).nulling(pos)
    }

    fn events(&self) -> (&[EventAndDistance], &[EventAndDistance]) {
        (**self).events()
    }

    fn trace(&self) -> [&[ExternalDottedRule]; 3] {
        (**self).trace()
    }

    fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        (**self).get_rhs1(dot)
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

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
        (**self).eliminated_nulling_intermediate()
    }

    fn completions(&self, sym: Symbol) -> &[PredictionTransition] {
        (**self).completions(sym)
    }

    fn gen_completion(&self, sym: Symbol) -> [Option<PredictionTransition>; 2] {
        (**self).gen_completion(sym)
    }

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol> {
        (**self).to_internal(symbol)
    }

    fn to_external(&self, symbol: Symbol) -> Symbol {
        (**self).to_external(symbol)
    }

    fn dot_before_eof(&self) -> Dot {
        (**self).dot_before_eof()
    }

    fn useless_symbol(&self) -> Symbol {
        (**self).useless_symbol()
    }

    fn forest_info(&self) -> ForestInfo {
        (**self).forest_info()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ForestInfo {
    pub eval: Vec<ExternalOrigin>,
    pub nulling_intermediate_rules: Vec<NullingIntermediateRule>,
    // Each rule can have only one eliminated nulling symbol.
    pub nulling_eliminated: Vec<NullingEliminated>,
    pub sof: Symbol,
    pub eof: Symbol,
}

impl ForestInfo {
    pub fn max_nulling_symbol(&self) -> Option<usize> {
        self.nulling_eliminated
            .iter()
            .flatten()
            .map(|&(sym, _dir)| sym.usize())
            .chain(
                self.nulling_intermediate_rules
                    .iter()
                    .flat_map(|&[lhs, rhs0, rhs1]| [lhs.usize(), rhs0.usize(), rhs1.usize()]),
            )
            .max()
    }

    pub fn external_origin(&self, dot: u32) -> Option<ExternalOrigin> {
        self.eval.get(dot as usize).copied()
    }
}
