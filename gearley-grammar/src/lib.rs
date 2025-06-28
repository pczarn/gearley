use bit_matrix::row::BitSlice;
use cfg_symbol::Symbol;
use miniserde::{Serialize, Deserialize};

pub use cfg_history::earley::{EventAndDistance, ExternalDottedRule, NullingEliminated, ExternalOrigin};

type Dot = u32;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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

pub type NullingIntermediateRule = [Symbol; 3];

pub trait Grammar {
    fn sof(&self) -> Symbol;

    fn eof(&self) -> Symbol;

    fn lr_set(&self, dot: Dot) -> &BitSlice;

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

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot;

    fn rhs1(&self) -> &[Option<Symbol>];

    fn get_lhs(&self, dot: Dot) -> Symbol;

    fn external_origin(&self, dot: Dot) -> ExternalOrigin;

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule];

    fn completions(&self, sym: Symbol) -> &[PredictionTransition];

    fn gen_completion(&self, sym: Symbol) -> PredictionTransition;

    fn to_internal(&self, symbol: Symbol) -> Option<Symbol>;

    fn to_external(&self, symbol: Symbol) -> Symbol;

    fn dot_before_eof(&self) -> Dot;

    fn useless_symbol(&self) -> Symbol;

    fn forest_info(&self) -> ForestInfo;
}

impl<'a, G> Grammar for &'a G where G: Grammar {
    fn sof(&self) -> Symbol {
        (**self).sof()
    }

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

    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot {
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

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
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


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ForestInfo {
    pub eval: Vec<ExternalOrigin>,
    pub nulling_intermediate_rules: Vec<NullingIntermediateRule>,
    // Each rule can have only one eliminated nulling symbol.
    pub nulling_eliminated: Vec<NullingEliminated>,
}

impl ForestInfo {
    pub fn max_nulling_symbol(&self) -> Option<usize> {
        self.nulling_eliminated.iter().flatten().map(|&(sym, _dir)| sym.usize())
            .chain(
                self.nulling_intermediate_rules
                    .iter()
                    .map(|&[_lhs, rhs0, _rhs1]| rhs0.usize()),
            )
            .max()
    }

    pub fn external_origin(&self, dot: u32) -> Option<ExternalOrigin> {
        self.eval.get(dot as usize).copied()
    }
}
