use std::convert::TryInto;
use std::iter;

use bit_matrix::BitMatrix;
use cfg::{ContextFreeRef, GrammarRule, Symbol};
use cfg::rule::container::RuleContainer;
use cfg::remap::Mapping;
use cfg::prediction::{FirstSetsCollector, FollowSets};
use optional::Optioned;

pub use cfg::earley::{Grammar, BinarizedGrammar};
pub use cfg::earley::history::History;

use recognizer::Predicted;
use policy::{PerformancePolicy, DefaultPerformancePolicy};

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
pub(in super) struct PredictionTransition<P: PerformancePolicy> {
    pub symbol: P::Symbol,
    pub dot: P::Dot,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub(in super) enum MaybePostdot<S> {
    Binary(S),
    Unary,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct InternalGrammar<P: PerformancePolicy = DefaultPerformancePolicy> {
    start_sym: P::Symbol,
    original_start_sym: P::Symbol,
    has_trivial_derivation: bool,
    eof_sym: P::Symbol,
    dot_before_eof: P::Dot,
    size: InternalGrammarSize,

    prediction_matrix: BitMatrix,
    // Inverse prediction lookup.
    unary_completions: Vec<PredictionTransition<P>>,
    unary_completion_index: Vec<u32>,

    binary_completions: Vec<PredictionTransition<P>>,
    binary_completion_index: Vec<u32>,

    follow_sets: BitMatrix,
    first_sets: BitMatrix,

    // array of events
    events_rhs: [Vec<Event>; 3],
    // 2-dimensional arrays for tracing
    trace_rhs: [Vec<Option<ExternalDottedRule>>; 3],
    // Each rule can have only one eliminated nulling symbol.
    nulling_eliminated: Vec<NullingEliminated>,
    // Rules stored in column-major order.
    lhs: Vec<Option<P::Symbol>>,
    rhs0: Vec<Option<P::Symbol>>,
    rhs1: Vec<Option<P::Symbol>>,
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
type CompletionTable<P> = Vec<Vec<PredictionTransition<P>>>;

impl<P: PerformancePolicy> InternalGrammar<P> {
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
        result.populate_maps(maps);
        result.populate_grammar(&grammar);
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
        self.start_sym = start.into();
        self.eof_sym = grammar.eof().unwrap().into();
        self.dot_before_eof = grammar.dot_before_eof().unwrap().try_into().ok().unwrap();
        self.original_start_sym = grammar.original_start().unwrap().into();
    }

    fn populate_grammar_with_lhs(&mut self, grammar: &BinarizedGrammar) {
        self.lhs.extend(grammar.rules().map(|rule| Some(rule.lhs().into())));
    }

    fn populate_grammar_with_rhs(&mut self, grammar: &BinarizedGrammar) {
        self.rhs0.extend(grammar.rules().map(|rule| rule.rhs().get(0).cloned().map(|s| s.into())));
        self.rhs1.extend(grammar.rules().map(|rule| rule.rhs().get(1).cloned().map(|s| s.into())));
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
        let unary_table = self.compute_unary_completion_table(grammar);
        let binary_table = self.compute_binary_completion_table(grammar);
        self.populate_prediction_matrix(grammar, &unary_table, &binary_table);
        self.populate_prediction_events(grammar);
        self.populate_completion_tables(grammar, &unary_table, &binary_table);
        self.populate_first_and_follow_sets(grammar);
    }

    fn populate_prediction_matrix(&mut self, grammar: &BinarizedGrammar, unary: &CompletionTable<P>, binary: &CompletionTable<P>) {
        let mut general_prediction_matrix = BitMatrix::new(self.size.syms, self.size.syms);
        // Precompute DFA.
        for rule in grammar.rules() {
            general_prediction_matrix.set(rule.lhs().usize(), rule.rhs()[0].usize(), true);
        }
        general_prediction_matrix.transitive_closure();
        // Prediction relation is reflexive.
        for i in 0..self.size.syms {
            general_prediction_matrix.set(i, i, true);
        }
        self.prediction_matrix = BitMatrix::new(self.size.syms, self.size.syms * 4);
        for i in 0..self.size.syms {
            for j in 0..self.size.syms {
                if general_prediction_matrix[(i, j)] {
                    // println!("({}, {})", i, j);
                    self.prediction_matrix.set(i, Predicted::Any(j).usize(), true);
                    if i == j {
                        self.prediction_matrix.set(i, Predicted::Medial(i).usize(), true);
                    }
                    if unary[j].iter().any(|transition| general_prediction_matrix[(i, transition.symbol.into().usize())]) {
                        // println!("unary");
                        self.prediction_matrix.set(i, Predicted::Unary(j).usize(), true);
                    }
                    if binary[j].iter().any(|transition| general_prediction_matrix[(i, transition.symbol.into().usize())]) {
                        // println!("binary");
                        self.prediction_matrix.set(i, Predicted::Binary(j).usize(), true);
                    }
                }
            }
        }
    }

    fn populate_first_and_follow_sets(&mut self, grammar: &BinarizedGrammar) {
        self.follow_sets = BitMatrix::new(self.size.syms, self.size.syms + 1);
        self.first_sets = BitMatrix::new(self.size.syms, self.size.syms + 1);
        let first_sets = FirstSetsCollector::new(grammar);
        for (outer, inner) in first_sets.first_sets() {
            for elem_inner in inner.into_iter() {
                if let Some(inner_sym) = elem_inner {
                    self.first_sets.set(outer.usize(), inner_sym.usize(), true);
                }
            }
        }
        self.first_sets.reflexive_closure();
        let follow_sets = FollowSets::new(grammar, grammar.start(), first_sets.first_sets());
        for (before, after) in follow_sets.follow_sets().into_iter() {
            for elem_after in after.into_iter() {
                if let Some(after_sym) = elem_after {
                    self.follow_sets.set(before.usize(), after_sym.usize(), true);
                }
            }
        }
        for i in 0 .. self.size.syms {
            self.follow_sets.set(i, self.size.syms, true);
            self.first_sets.set(i, self.size.syms, true);
        }
    }

    fn populate_completion_tables(&mut self, grammar: &BinarizedGrammar, unary: &CompletionTable<P>, binary: &CompletionTable<P>) {
        self.populate_unary_completion_table(grammar, unary);
        self.populate_binary_completion_table(grammar, binary);
    }

    fn populate_unary_completion_table(&mut self, grammar: &BinarizedGrammar, table: &CompletionTable<P>) {
        self.populate_unary_completion_index(table);
        self.populate_unary_completions(table);
    }

    fn compute_unary_completion_table(&self, grammar: &BinarizedGrammar) -> CompletionTable<P> {
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
                symbol: lhs_sym.into(),
                dot: dot.try_into().ok().unwrap(),
            });
        }
        table
    }

    fn populate_unary_completion_index(&mut self, table: &CompletionTable<P>) {
        let mut current_idx = 0u32;
        self.unary_completion_index.push(0u32);
        self.unary_completion_index.extend(table.iter().map(|run| {
            current_idx = current_idx.checked_add(run.len() as u32).unwrap();
            current_idx
        }));
    }

    fn populate_unary_completions(&mut self, table: &CompletionTable<P>) {
        let iter_table = table.into_iter().flat_map(|v| v.into_iter());
        self.unary_completions.extend(iter_table);
    }

    fn populate_binary_completion_table(&mut self, grammar: &BinarizedGrammar, table: &CompletionTable<P>) {
        self.populate_binary_completion_index(table);
        self.populate_binary_completions(table);
    }

    fn compute_binary_completion_table(&self, grammar: &BinarizedGrammar) -> CompletionTable<P> {
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
                symbol: lhs_sym.into(),
                dot: dot.try_into().ok().unwrap(),
            });
        }
        table
    }

    fn populate_binary_completion_index(&mut self, table: &CompletionTable<P>) {
        let mut current_idx = 0u32;
        self.binary_completion_index.push(0u32);
        self.binary_completion_index.extend(table.iter().map(|run| {
            current_idx = current_idx.checked_add(run.len() as u32).unwrap();
            current_idx
        }));
    }

    fn populate_binary_completions(&mut self, table: &CompletionTable<P>) {
        let iter_table = table.into_iter().flat_map(|v| v.into_iter());
        self.binary_completions.extend(iter_table);
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
    pub(in super) fn eof(&self) -> P::Symbol {
        self.eof_sym
    }

    #[inline]
    fn neutral_sym(&self) -> P::Symbol {
        let sym: Symbol = (self.num_syms() as u32).into();
        sym.into()
    }

    #[inline]
    pub(in super) fn can_follow(&self, before: P::Symbol, after: Option<Option<P::Symbol>>) -> bool {
        let after = after.unwrap_or(Some(self.neutral_sym())).unwrap_or(self.eof()).into().usize();
        self.follow_sets[(before.into().usize(), after)]
    }

    #[inline]
    pub(in super) fn first(&self, outer: P::Symbol, maybe_inner: Option<Option<P::Symbol>>) -> bool {
        let inner = if let Some(inner) = maybe_inner.unwrap_or(Some(self.neutral_sym())) {
            inner
        } else {
            return outer == self.eof()
        };
        self.first_sets[(outer.into().usize(), inner.into().usize())]
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
    pub(in super) fn predicted_row_size(&self) -> usize {
        self.num_syms() * 4
    }

    #[inline]
    pub(in super) fn num_rules(&self) -> usize {
        self.size.rules
    }

    #[inline]
    pub fn start_sym(&self) -> P::Symbol {
        self.start_sym
    }

    pub fn externalized_start_sym(&self) -> P::Symbol {
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
    pub(crate) fn origin(&self, origin_and_lhs: u32) -> u32 {
        origin_and_lhs / self.num_syms() as u32
    }

    #[inline]
    pub(crate) fn lhs_sym(&self, origin_and_lhs: u32) -> P::Symbol {
        let result: Symbol = (origin_and_lhs % self.num_syms() as u32).into();
        result.into()
    }

    #[inline]
    pub(crate) fn origin_and_lhs(&self, origin: u32, lhs: P::Symbol) -> u32 {
        let lhs: Symbol = lhs.into();
        let lhs: u32 = lhs.into();
        origin * self.num_syms() as u32 + lhs
    }

    #[inline]
    pub(in super) fn get_rhs1(&self, dot: P::Dot) -> Option<P::Symbol> {
        self.rhs1[dot.into() as usize]
    }

    #[inline]
    pub(in super) fn get_rhs1_cmp(&self, dot: P::Dot) -> MaybePostdot<P::Symbol> {
        match self.rhs1[dot.into() as usize] {
            None => MaybePostdot::Unary,
            Some(rhs1) => MaybePostdot::Binary(rhs1),
        }
    }

    #[inline]
    pub(in super) fn rhs1(&self) -> &[Option<P::Symbol>] {
        &self.rhs1[..]
    }

    #[inline]
    pub(in super) fn get_lhs(&self, dot: P::Dot) -> P::Symbol {
        self.lhs[dot.into() as usize].unwrap()
    }

    #[inline]
    pub(in super) fn external_origin(&self, dot: P::Dot) -> ExternalOrigin {
        self.eval.get(dot.into() as usize).cloned().unwrap()
    }

    pub(in super) fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
        &*self.nulling_intermediate_rules
    }

    #[inline(always)]
    pub(in super) fn unary_completions(&self, sym: P::Symbol) -> &[PredictionTransition<P>] {
        let idxs = &self.unary_completion_index[sym.into().usize() .. sym.into().usize() + 2];
        let range = idxs[0] as usize .. idxs[1] as usize;
        &self.unary_completions[range]
    }

    #[inline(always)]
    pub(in super) fn binary_completions(&self, sym: P::Symbol) -> &[PredictionTransition<P>] {
        let idxs = &self.binary_completion_index[sym.into().usize() .. sym.into().usize() + 2];
        let range = idxs[0] as usize .. idxs[1] as usize;
        &self.binary_completions[range]
    }

    #[inline(always)]
    pub(in super) fn to_internal(&self, symbol: P::Symbol) -> Option<P::Symbol> {
        if self.sym_maps.to_internal.is_empty() {
            Some(symbol)
        } else {
            self.sym_maps.to_internal[symbol.into().usize()].map(|sym| sym.into())
        }
    }

    #[inline]
    pub fn to_external(&self, symbol: P::Symbol) -> P::Symbol {
        if self.sym_maps.to_external.is_empty() {
            symbol
        } else {
            self.sym_maps.to_external[symbol.into().usize()].into()
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

    pub(in super) fn dot_before_eof(&self) -> P::Dot {
        self.dot_before_eof
    }
}
