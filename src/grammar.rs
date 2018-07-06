use std::iter;

use bit_matrix::BitMatrix;
use cfg::{ContextFreeRef, GrammarRule, Symbol};
use cfg::earley::{Grammar, BinarizedGrammar};
use cfg::rule::container::RuleContainer;
use cfg::remap::Mapping;
use optional::Optioned;

use item::Dot;

// For efficiency, the recognizer works on processed grammars. Grammars described by the user
// are transformed to meet the following properties:
//
// Property (1): Right-hand-sides of all rules have at least one symbol.
//
// Thanks to property (2), this results in linear, not exponential increase in the
// number of symbols and dotted rules.
//
// Property (2): Right-hand-sides of all rules have at most two symbols.
//
// That is, all rules are of the form
// `A ::= B C`
// or
// `D ::= E`.
//
// Property (3): There are no cycles among unit rules.
//
// That is, for any nonterminals `A`…`Z`, the set of rules doesn't have a subset
// such as {`A ::= B`, `B ::= C`, …, `Y ::= Z`, `Z ::= A`}.
//
// In other words, for any nonterminal `A`, `A` doesn't derive `A` in two or more steps.
//
// Property (4): Dot numbers for pre-RHS0 dots are ordered by the LHS symbol IDs.
//
// Property (5): Dot numbers for pre-RHS1 dots are ordered by their RHS1 symbol IDs.
//
// Property (6): IDs of unit rules are smaller than IDs of rules which they predict.
//
// Internal symbols must be remapped, because this property may interfere with (4).
// This property also requires (3).
//
// # Similarities to other parsers
//
// * (1) is required by some Earley parsers, including Marpa.
// * (2) is required for recognition in CYK parsers, and in a roundabout way for construction
//   of bocages.
// * (3) is required by PEG and some other parsers.
// * (4) and (5) are specific to gearley.

// # Future optimizations
//
// Store RHS1 and LHS in row-major instead of column-major order, so that the least significant bit
// tells us whether a dot is medial or completed. Or don't.
//
// Parameterize the representation over symbol type (u32, u16, u8).

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(in super) struct PredictionTransition {
    pub symbol: Symbol,
    pub dot: Dot,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct InternalGrammar {
    start_sym: Symbol,
    original_start_sym: Symbol,
    has_trivial_derivation: bool,
    dot_before_eof: Dot,
    size: InternalGrammarSize,

    prediction_matrix: BitMatrix,
    // Inverse prediction lookup.
    inverse_unary_prediction: Vec<PredictionTransition>,
    inverse_unary_prediction_index: Vec<u32>,

    inverse_binary_prediction: Vec<PredictionTransition>,
    inverse_binary_prediction_index: Vec<u32>,

    // array of events
    events_rhs: [Vec<Event>; 3],
    // 2-dimensional arrays for tracing
    trace_rhs: [Vec<Option<ExternalDottedRule>>; 3],
    // Each rule can have only one eliminated nulling symbol.
    nulling_eliminated: Vec<NullingEliminated>,
    // Rules stored in column-major order.
    lhs: Vec<Option<Symbol>>,
    rhs0: Vec<Option<Symbol>>,
    rhs1: Vec<Option<Symbol>>,
    // Rule origin preserved for post-parse actions.
    eval: Vec<ExternalOrigin>,
    // Mapping between external and internal symbols.
    sym_maps: Mapping,
    nulling_intermediate_rules: Vec<NullingIntermediateRule>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct InternalGrammarSize {
    pub syms: usize,
    pub rules: usize,
    pub internal_syms: usize,
    pub external_syms: usize,
}

pub(in super) type ExternalDottedRule = (u32, u32);
type ExternalOrigin = Option<u32>;
type EventId = Optioned<u32>;
type MinimalDistance = Optioned<u32>;
pub(in super) type Event = (EventId, MinimalDistance);
type NullingEliminated = Option<(Symbol, bool)>;
type NullingIntermediateRule = (Symbol, Symbol, Symbol);
type InversePredictionTable = Vec<Vec<PredictionTransition>>;

impl InternalGrammar {
    fn new() -> Self {
        Self::default()
    }

    pub fn from_grammar(grammar: &Grammar) -> Self {
        Self::from_binarized_grammar(grammar.binarize())
    }

    pub fn from_binarized_grammar(grammar: BinarizedGrammar) -> Self {
        let grammar = grammar.make_proper();
        Self::from_proper_binarized_grammar(grammar)
    }

    pub fn from_proper_binarized_grammar(grammar: BinarizedGrammar) -> Self {
        let (mut grammar, nulling) = grammar.eliminate_nulling();
        grammar.wrap_start();
        Self::from_processed_grammar(grammar, &nulling)
    }

    pub fn from_processed_grammar(grammar: BinarizedGrammar, nulling: &BinarizedGrammar) -> Self {
        let (grammar, maps) = grammar.remap_symbols();
        Self::from_processed_grammar_with_maps(grammar, maps, nulling)
    }

    pub fn from_processed_grammar_with_maps(
        mut grammar: BinarizedGrammar,
        maps: Mapping,
        nulling: &BinarizedGrammar)
        -> Self
    {
        grammar.sort_by(|a, b| a.lhs().cmp(&b.lhs()));
        let mut result = InternalGrammar::new();
        result.populate_sizes(&grammar, &maps);
        result.populate_grammar(&grammar);
        result.populate_maps(maps);
        result.populate_nulling(nulling);
        trace!("populated grammar {:?}", &result);
        result
    }

    fn populate_sizes(&mut self, grammar: &BinarizedGrammar, maps: &Mapping) {
        self.size = InternalGrammarSize {
            rules: grammar.rules().count(),
            syms: grammar.sym_source().num_syms(),
            external_syms: maps.to_internal.len(),
            internal_syms: maps.to_external.len(),
        }
    }

    fn populate_grammar(&mut self, grammar: &BinarizedGrammar) {
        self.populate_start_sym(grammar);
        self.populate_grammar_with_lhs(grammar);
        self.populate_grammar_with_rhs(grammar);
        self.populate_grammar_with_history(grammar);
        self.populate_predictions(grammar);
    }

    fn populate_start_sym(&mut self, grammar: &BinarizedGrammar) {
        let start = grammar.start();
        let rule = grammar.rules().enumerate().find(|(_, rule)| rule.lhs() == start).unwrap();
        self.dot_before_eof = rule.0 as u32;
        self.start_sym = start;
        self.original_start_sym = rule.1.rhs().get(0).cloned().unwrap();
    }

    fn populate_grammar_with_lhs(&mut self, grammar: &BinarizedGrammar) {
        self.lhs.extend(grammar.rules().map(|rule| Some(rule.lhs())));
    }

    fn populate_grammar_with_rhs(&mut self, grammar: &BinarizedGrammar) {
        self.rhs0.extend(grammar.rules().map(|rule| rule.rhs().get(0).cloned()));
        self.rhs1.extend(grammar.rules().map(|rule| rule.rhs().get(1).cloned()));
    }

    fn populate_grammar_with_history(&mut self, grammar: &BinarizedGrammar) {
        self.eval.extend(
            grammar.rules().map(|rule| rule.history().origin())
        );
        self.nulling_eliminated.extend(
            grammar.rules().map(|rule| rule.history().nullable())
        );

        self.populate_grammar_with_events_rhs(grammar);
        self.populate_grammar_with_trace_rhs(grammar);
    }

    fn populate_grammar_with_events_rhs(&mut self, grammar: &BinarizedGrammar) {
        self.events_rhs[1].extend(
            grammar.rules().map(|rule| rule.history().dot(1).event_without_tracing())
        );
        self.events_rhs[2].extend(
            grammar.rules().map(|rule| rule.history().dot(2).event_without_tracing())
        );
    }

    fn populate_grammar_with_trace_rhs(&mut self, grammar: &BinarizedGrammar) {
        self.trace_rhs[1].extend(
            grammar.rules().map(|rule| rule.history().dot(1).trace())
        );
        self.trace_rhs[2].extend(
            grammar.rules().map(|rule| rule.history().dot(2).trace())
        );
    }

    fn populate_maps(&mut self, maps: Mapping) {
        self.sym_maps = maps;
    }

    fn populate_predictions(&mut self, grammar: &BinarizedGrammar) {
        self.populate_prediction_matrix(grammar);
        self.populate_prediction_events(grammar);
        self.populate_inverse_prediction_tables(grammar);
    }

    fn populate_prediction_matrix(&mut self, grammar: &BinarizedGrammar) {
        self.prediction_matrix = BitMatrix::new(self.size.syms, self.size.syms);
        // Precompute DFA.
        for rule in grammar.rules() {
            self.prediction_matrix.set(rule.lhs().usize(), rule.rhs()[0].usize(), true);
        }
        self.prediction_matrix.transitive_closure();
        // Prediction relation is reflexive.
        for i in 0..self.size.syms {
            self.prediction_matrix.set(i, i, true);
        }
    }

    fn populate_inverse_prediction_tables(&mut self, grammar: &BinarizedGrammar) {
        self.populate_inverse_unary_prediction_table(grammar);
        self.populate_inverse_binary_prediction_table(grammar);
    }

    fn populate_inverse_unary_prediction_table(&mut self, grammar: &BinarizedGrammar) {
        let table = self.compute_inverse_unary_prediction_table(grammar);
        let mut current_idx = 0u32;
        self.inverse_unary_prediction_index.push(current_idx as u32);
        self.inverse_unary_prediction_index.extend(table.iter().map(|run| {
            current_idx = current_idx.checked_add(run.len() as u32).unwrap();
            current_idx
        }));
        let iter_table = table.into_iter().flat_map(|v| v.into_iter());
        self.inverse_unary_prediction.extend(iter_table);
    }

    fn compute_inverse_unary_prediction_table(&self, grammar: &BinarizedGrammar) -> InversePredictionTable {
        let mut table = iter::repeat(vec![]).take(self.size.syms).collect::<Vec<_>>();

        let mut unary_rules = vec![];
        // check for ordering same as self.rules
        for (dot, rule) in grammar.rules().enumerate() {
            let is_unary = rule.rhs().get(1).is_none();
            if is_unary {
                let rhs0_sym = rule.rhs()[0].usize();
                unary_rules.push((rhs0_sym, rule.lhs, dot));
            }
        }
        for (rhs0_sym, lhs_sym, dot) in unary_rules.into_iter() {
            table[rhs0_sym].push(PredictionTransition {
                symbol: lhs_sym,
                dot: dot as u32
            });
        }
        table
    }

    fn populate_inverse_binary_prediction_table(&mut self, grammar: &BinarizedGrammar) {
        let table = self.compute_inverse_binary_prediction_table(grammar);
        let mut current_idx = 0u32;
        self.inverse_binary_prediction_index.push(current_idx as u32);
        self.inverse_binary_prediction_index.extend(table.iter().map(|run| {
            current_idx = current_idx.checked_add(run.len() as u32).unwrap();
            current_idx
        }));
        let iter_table = table.into_iter().flat_map(|v| v.into_iter());
        self.inverse_binary_prediction.extend(iter_table);
    }

    fn compute_inverse_binary_prediction_table(&self, grammar: &BinarizedGrammar) -> InversePredictionTable {
        let mut table = iter::repeat(vec![]).take(self.size.syms).collect::<Vec<_>>();

        let mut binary_rules = vec![];
        // check for ordering same as self.rules
        for (dot, rule) in grammar.rules().enumerate() {
            let is_binary = rule.rhs().get(1).is_some();
            if is_binary {
                let rhs0_sym = rule.rhs()[0].usize();
                binary_rules.push((rhs0_sym, rule.lhs, dot));
            }
        }
        for (rhs0_sym, lhs_sym, dot) in binary_rules.into_iter() {
            table[rhs0_sym].push(PredictionTransition {
                symbol: lhs_sym,
                dot: dot as u32
            });
        }
        table
    }

    fn populate_prediction_events(&mut self, grammar: &BinarizedGrammar) {
        let iter_events_pred = iter::repeat((Optioned::none(), Optioned::none())).take(self.size.syms);
        self.events_rhs[0].extend(iter_events_pred);
        let iter_trace_pred = iter::repeat(None).take(self.size.syms);
        self.trace_rhs[0].extend(iter_trace_pred);
        for rule in grammar.rules() {
            if let Some(&(pred_event, pred_tracing)) = rule.history().dot(0).event().as_ref() {
                // Prediction event and tracing.
                self.events_rhs[0][rule.lhs().usize()] = (
                    pred_event,
                    rule.history().dot(0).distance()
                );
                self.trace_rhs[0][rule.lhs().usize()] = Some(pred_tracing);
            }
        }
    }

    fn populate_nulling(&mut self, nulling: &BinarizedGrammar) {
        self.has_trivial_derivation = !nulling.is_empty();
        let iter_nulling_intermediate = nulling.rules().filter_map(|rule| {
            if rule.history().origin().is_none() && rule.rhs().len() == 2 {
                Some((rule.lhs(), rule.rhs()[0], rule.rhs()[1]))
            } else {
                None
            }
        });
        self.nulling_intermediate_rules.extend(iter_nulling_intermediate);
    }

    #[inline]
    pub(in super) fn prediction_matrix(&self) -> &BitMatrix {
        &self.prediction_matrix
    }

    #[inline]
    pub(in super) fn num_syms(&self) -> usize {
        self.size.syms
    }

    #[inline]
    pub(in super) fn num_rules(&self) -> usize {
        self.size.rules
    }

    #[inline]
    pub fn start_sym(&self) -> Symbol {
        self.start_sym
    }

    pub fn externalized_start_sym(&self) -> Symbol {
        self.to_external(self.original_start_sym)
    }

    #[inline]
    pub(in super) fn has_trivial_derivation(&self) -> bool {
        self.has_trivial_derivation
    }

    #[inline]
    pub(in super) fn nulling(&self, pos: u32) -> NullingEliminated {
        self.nulling_eliminated.get(pos as usize).and_then(|&ne| ne)
    }

    #[inline]
    pub(in super) fn events(&self) -> (&[Event], &[Event]) {
        (&self.events_rhs[1][..], &self.events_rhs[2][..])
    }

    #[inline]
    pub(in super) fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3] {
        [&self.trace_rhs[0][..], &self.trace_rhs[1][..], &self.trace_rhs[2][..]]
    }

    #[inline]
    pub(in super) fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        self.rhs1[dot as usize]
    }

    #[inline]
    pub(in super) fn rhs1(&self) -> &[Option<Symbol>] {
        &self.rhs1[..]
    }

    #[inline]
    pub(in super) fn get_lhs(&self, dot: Dot) -> Symbol {
        self.lhs[dot as usize].unwrap()
    }

    #[inline]
    pub(in super) fn external_origin(&self, dot: Dot) -> ExternalOrigin {
        self.eval.get(dot as usize).cloned().unwrap()
    }

    pub(in super) fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
        &*self.nulling_intermediate_rules
    }

    #[inline(always)]
    pub(in super) fn inverse_unary_prediction(&self, sym: Symbol) -> &[PredictionTransition] {
        let idxs = &self.inverse_unary_prediction_index[sym.usize() .. sym.usize() + 2];
        let range = idxs[0] as usize .. idxs[1] as usize;
        &self.inverse_unary_prediction[range]
    }

    #[inline(always)]
    pub(in super) fn inverse_binary_prediction(&self, sym: Symbol) -> &[PredictionTransition] {
        let idxs = &self.inverse_binary_prediction_index[sym.usize() .. sym.usize() + 2];
        let range = idxs[0] as usize .. idxs[1] as usize;
        &self.inverse_binary_prediction[range]
    }

    #[inline(always)]
    pub(in super) fn to_internal(&self, symbol: Symbol) -> Option<Symbol> {
        if self.sym_maps.to_internal.is_empty() {
            Some(symbol)
        } else {
            self.sym_maps.to_internal[symbol.usize()]
        }
    }

    #[inline]
    pub(in super) fn to_external(&self, symbol: Symbol) -> Symbol {
        if self.sym_maps.to_external.is_empty() {
            symbol
        } else {
            self.sym_maps.to_external[symbol.usize()]
        }
    }

    pub(in super) fn max_nulling_symbol(&self) -> Option<usize> {
        (0 .. self.num_rules()).filter_map(|action| {
            self.nulling(action as u32).map(|(sym, _dir)| sym.usize())
        }).chain(
            self.eliminated_nulling_intermediate().iter().map(|&(_lhs, rhs0, _rhs1)| {
                rhs0.usize()
            })
        ).max()
    }

    pub(in super) fn dot_before_eof(&self) -> Dot {
        self.dot_before_eof
    }
}
