use std::{cmp, iter};

use bit_matrix::row::BitSlice;
use bit_matrix::BitMatrix;
use cfg::classify::CfgClassifyExt;
use cfg::predict_sets::{FirstSets, FollowSets, PredictSets};
use cfg::symbol_bit_matrix::CfgSymbolBitMatrixExt;
use cfg_earley_history::HistoryGraphEarleyExt;
use cfg_symbol::intern::Mapping;
use miniserde::{Serialize, Deserialize};

use cfg::earley_history::{History, Event, ExternalDottedRule, NullingEliminated, ExternalOrigin};
use cfg::{Cfg, Symbol, Symbolic};

use gearley_grammar::{Grammar, PredictionTransition, MaybePostdot, NullingIntermediateRule};
use gearley_vec2d::Vec2d;

type Dot = u32;

type CompletionTable = Vec<Vec<PredictionTransition>>;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DefaultGrammar {
    start_sym: Symbol,
    original_start_sym: Symbol,
    has_trivial_derivation: bool,
    eof_sym: Symbol,
    dot_before_eof: Dot,
    size: DefaultGrammarSize,

    prediction_matrix: BitMatrix,
    // Inverse prediction lookup.
    completions: Vec2d<PredictionTransition>,

    lr_sets: BitMatrix,

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
    nulling_intermediate_rules: Vec<NullingIntermediateRule<Symbol>>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DefaultGrammarSize {
    pub syms: usize,
    pub gen_syms: usize,
    pub rules: usize,
    pub internal_syms: usize,
    pub external_syms: usize,
}

impl DefaultGrammar {
    fn new() -> Self {
        Default::default()
    }

    pub fn from_grammar(mut grammar: Cfg) -> Self {
        let orig_num_syms = grammar.num_syms();
        grammar.limit_rhs_len(Some(2));
        let num_gen_syms = grammar.num_syms() - orig_num_syms;
        Self::from_binarized_grammar(grammar, num_gen_syms)
    }

    pub fn from_binarized_grammar(mut grammar: Cfg, num_gen_syms: usize) -> Self {
        grammar.make_proper();
        Self::from_proper_binarized_grammar(grammar, num_gen_syms)
    }

    pub fn from_proper_binarized_grammar(mut grammar: Cfg, num_gen_syms: usize) -> Self {
        let nulling = grammar.binarize_and_eliminate_nulling_rules();
        grammar.wrap_input();
        Self::from_processed_grammar(grammar, &nulling, num_gen_syms)
    }

    pub fn from_processed_grammar(mut grammar: Cfg, nulling: &Cfg, num_gen_syms: usize) -> Self {
        // `order` describes relation `A < B`.
        let mut order = grammar.empty_matrix();
        for rule in grammar.rules() {
            if rule.rhs.len() == 1 {
                let left = rule.lhs.usize();
                let right = rule.rhs[0].usize();
                match left.cmp(&right) {
                    cmp::Ordering::Less => {
                        order.set(left, right, true);
                    }
                    cmp::Ordering::Greater => {
                        order.set(right, left, true);
                    }
                    cmp::Ordering::Equal => {}
                }
            }
        }
        let maps = grammar.reorder_symbols(order);
        Self::from_processed_grammar_with_maps(grammar, maps, nulling, num_gen_syms)
    }

    pub fn from_processed_grammar_with_maps(
        mut grammar: Cfg,
        maps: Mapping,
        nulling: &Cfg,
        num_gen_syms: usize
    ) -> Self {
        grammar.sort_by(|a, b| a.lhs.cmp(&b.lhs));
        let mut result = DefaultGrammar::new();
        result.populate_sizes(&grammar, &maps, num_gen_syms);
        result.populate_maps(maps);
        result.populate_grammar(&grammar);
        result.populate_nulling(nulling);
        result
    }

    fn populate_sizes(&mut self, grammar: &Cfg, maps: &Mapping, gen_syms: usize) {
        self.size = DefaultGrammarSize {
            rules: grammar.rules().count(),
            syms: grammar.sym_source().num_syms() - gen_syms,
            gen_syms,
            external_syms: maps.to_internal.len(),
            internal_syms: maps.to_external.len(),
        }
    }

    fn populate_grammar(&mut self, grammar: &Cfg) {
        self.populate_start_sym(grammar);
        self.populate_grammar_with_lhs(grammar);
        self.populate_grammar_with_rhs(grammar);
        self.populate_grammar_with_history(grammar);
        self.populate_predictions(grammar);
    }

    fn populate_start_sym(&mut self, grammar: &Cfg) {
        assert_eq!(grammar.roots().len(), 1);
        let wrapped_root = grammar.wrapped_roots().first().copied().expect("start symbol not found");
        self.start_sym = wrapped_root.root;
        self.eof_sym = wrapped_root.end_of_input;
        self.dot_before_eof = grammar.rules().position(|rule| rule.rhs.get(1) == Some(&wrapped_root.end_of_input)).unwrap() as u32;
        self.original_start_sym = wrapped_root.inner_root;
    }

    fn populate_grammar_with_lhs(&mut self, grammar: &Cfg) {
        self.lhs
            .extend(grammar.rules().map(|rule| Some(rule.lhs)));
    }

    fn populate_grammar_with_rhs(&mut self, grammar: &Cfg) {
        self.rhs0
            .extend(grammar.rules().map(|rule| rule.rhs.get(0).cloned()));
        self.rhs1
            .extend(grammar.rules().map(|rule| rule.rhs.get(1).cloned()));
    }

    fn populate_grammar_with_history(&mut self, grammar: &Cfg) {
        let histories = grammar.history_graph().final_history();
        self.eval
            .extend(grammar.rules().map(|rule| histories[rule.history_id.get()].origin()));
        self.nulling_eliminated
            .extend(grammar.rules().map(|rule| histories[rule.history_id.get()].nullable()));

        self.populate_grammar_with_events_rhs(grammar, &histories[..]);
        self.populate_grammar_with_trace_rhs(grammar, &histories[..]);
    }

    fn populate_grammar_with_events_rhs(&mut self, grammar: &Cfg, histories: &[History]) {
        self.events_rhs[1].extend(
            grammar
                .rules()
                .map(|rule| histories[rule.history_id.get()].dot(1).event_without_tracing()),
        );
        self.events_rhs[2].extend(
            grammar
                .rules()
                .map(|rule| histories[rule.history_id.get()].dot(2).event_without_tracing()),
        );
    }

    fn populate_grammar_with_trace_rhs(&mut self, grammar: &Cfg, histories: &[History]) {
        self.trace_rhs[1].extend(grammar.rules().map(|rule| histories[rule.history_id.get()].dot(1).trace()));
        self.trace_rhs[2].extend(grammar.rules().map(|rule| histories[rule.history_id.get()].dot(2).trace()));
    }

    fn populate_maps(&mut self, maps: Mapping) {
        self.sym_maps = maps;
    }

    fn populate_predictions(&mut self, grammar: &Cfg) {
        self.populate_prediction_matrix(grammar);
        self.populate_prediction_events(grammar);
        self.populate_completion_tables(grammar);
        self.populate_lr_sets(grammar);
    }

    fn populate_prediction_matrix(&mut self, grammar: &Cfg) {
        self.prediction_matrix = BitMatrix::new(self.size.syms, self.size.syms);
        // Precompute DFA.
        for rule in grammar.rules() {
            self.prediction_matrix
                .set(rule.lhs.usize(), rule.rhs[0].usize(), true);
        }
        // Prediction relation is transitive.
        self.prediction_matrix.transitive_closure();
        // Prediction relation is reflexive.
        self.prediction_matrix.reflexive_closure();
    }

    fn populate_lr_sets(&mut self, grammar: &Cfg) {
        let mut follow_matrix = BitMatrix::new(self.size.syms * 2, self.size.syms);
        let mut first_matrix = BitMatrix::new(self.size.syms, self.size.syms);
        let first_sets = FirstSets::new(grammar);
        for (outer, inner) in first_sets.predict_sets() {
            for inner_sym in inner.iter().copied() {
                first_matrix.set(outer.usize(), inner_sym.usize(), true);
            }
        }
        first_matrix.reflexive_closure();
        let follow_sets = FollowSets::new(grammar, first_sets.predict_sets());
        for (before, after) in follow_sets.predict_sets().into_iter() {
            for after_sym in after.iter().copied() {
                follow_matrix.set(before.usize(), after_sym.usize(), true);
            }
        }
        self.lr_sets = BitMatrix::new(self.size.syms * 2, self.size.syms);
        for i in 0 .. self.size.syms {
            for (dst, &src) in self.lr_sets[i * 2].iter_blocks_mut().zip(first_matrix[i].iter_blocks()) {
                *dst = src;
            }
            for (dst, &src) in self.lr_sets[i * 2 + 1].iter_blocks_mut().zip(follow_matrix[i].iter_blocks()) {
                *dst = src;
            }
        }
    }

    fn populate_completion_tables(&mut self, grammar: &Cfg) {
        let table = self.compute_completion_table(grammar);
        let iter_table = table.into_iter().map(|v| v.into_iter());
        self.completions.extend(iter_table);
    }

    fn compute_completion_table(&self, grammar: &Cfg) -> CompletionTable {
        let mut table = iter::repeat(vec![])
            .take(self.size.syms)
            .collect::<Vec<_>>();

        let mut unary_rules = vec![];
        let mut binary_rules = vec![];
        // check for ordering same as self.rules
        for (dot, rule) in grammar.rules().enumerate() {
            let is_unary = rule.rhs.get(1).is_none();
            let rhs0_sym = rule.rhs[0];
            if is_unary {
                unary_rules.push((rhs0_sym.usize(), rule.lhs, dot as u32, true));
            } else {
                binary_rules.push((rhs0_sym.usize(), rule.lhs, dot as u32, false));
            }
        }
        // order is very important: first all binary, then all unary
        for (rhs0_sym, symbol, dot, is_unary) in binary_rules.into_iter().chain(unary_rules.into_iter()) {
            table[rhs0_sym].push(PredictionTransition {
                symbol,
                dot,
                is_unary,
            });
        }
        table
    }

    fn populate_prediction_events(&mut self, grammar: &Cfg) {
        let iter_events_pred =
            iter::repeat((None, None)).take(self.size.syms);
        self.events_rhs[0].extend(iter_events_pred);
        let iter_trace_pred = iter::repeat(None).take(self.size.syms);
        self.trace_rhs[0].extend(iter_trace_pred);
        let histories = grammar.history_graph().final_history();
        for rule in grammar.rules() {
            if let Some(&(pred_event, pred_tracing)) = histories[rule.history_id.get()].dot(0).event().as_ref() {
                // Prediction event and tracing.
                self.events_rhs[0][rule.lhs.usize()] =
                    (pred_event, histories[rule.history_id.get()].dot(0).distance());
                self.trace_rhs[0][rule.lhs.usize()] = Some(pred_tracing);
            }
        }
    }

    fn populate_nulling(&mut self, nulling: &Cfg) {
        self.has_trivial_derivation = !nulling.is_empty();
        let histories = nulling.history_graph().final_history();

        let iter_nulling_intermediate = nulling.rules().filter_map(|rule| {
            if histories[rule.history_id.get()].origin().is_none() && rule.rhs.len() == 2 {
                Some([rule.lhs, rule.rhs[0], rule.rhs[1]])
            } else {
                None
                
            }
        });
        self.nulling_intermediate_rules
            .extend(iter_nulling_intermediate);
    }
}

impl Grammar for DefaultGrammar {
    type Symbol = Symbol;

    #[inline]
    fn eof(&self) -> Symbol {
        self.eof_sym
    }

    fn lr_set(&self, dot: Dot) -> &BitSlice {
        match self.get_rhs1(dot) {
            Some(rhs1) => {
                &self.lr_sets[rhs1.usize() * 2]
            }
            None => {
                &self.lr_sets[self.get_lhs(dot).usize() * 2 + 1]
            }
        }
    }

    fn useless_symbol(&self) -> Symbol {
        self.start_sym
    }

    #[inline]
    fn prediction_row(&self, sym: Symbol) -> &BitSlice {
        &self.prediction_matrix[sym.usize()]
    }

    #[inline]
    fn num_syms(&self) -> usize {
        self.size.syms
    }

    #[inline]
    fn num_rules(&self) -> usize {
        self.size.rules
    }

    #[inline]
    fn start_sym(&self) -> Symbol {
        self.start_sym
    }

    #[inline]
    fn externalized_start_sym(&self) -> Symbol {
        self.to_external(self.original_start_sym)
    }

    #[inline]
    fn has_trivial_derivation(&self) -> bool {
        self.has_trivial_derivation
    }

    #[inline]
    fn nulling(&self, pos: u32) -> NullingEliminated {
        self.nulling_eliminated.get(pos as usize).and_then(|&ne| ne)
    }

    #[inline]
    fn events(&self) -> (&[Event], &[Event]) {
        (&self.events_rhs[1][..], &self.events_rhs[2][..])
    }

    #[inline]
    fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3] {
        [
            &self.trace_rhs[0][..],
            &self.trace_rhs[1][..],
            &self.trace_rhs[2][..],
        ]
    }

    #[inline]
    fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        self.rhs1[dot as usize]
    }

    #[inline]
    fn get_rhs1_cmp(&self, dot: Dot) -> MaybePostdot<Symbol> {
        match self.rhs1[dot as usize] {
            None => MaybePostdot::Unary,
            Some(rhs1) => MaybePostdot::Binary(rhs1),
        }
    }

    #[inline]
    fn rhs1(&self) -> &[Option<Symbol>] {
        &self.rhs1[..]
    }

    #[inline]
    fn get_lhs(&self, dot: Dot) -> Symbol {
        self.lhs[dot as usize].unwrap()
    }

    #[inline]
    fn external_origin(&self, dot: Dot) -> ExternalOrigin {
        self.eval.get(dot as usize).cloned().unwrap()
    }

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule<Symbol>] {
        &*self.nulling_intermediate_rules
    }

    #[inline(always)]
    fn completions(&self, sym: Symbol) -> &[PredictionTransition] {
        &self.completions[sym.usize()]
    }

    #[inline(always)]
    fn to_internal(&self, symbol: Symbol) -> Option<Symbol> {
        if self.sym_maps.to_internal.is_empty() {
            Some(symbol)
        } else {
            self.sym_maps.to_internal[symbol.usize()]
        }
    }

    #[inline]
    fn to_external(&self, symbol: Symbol) -> Symbol {
        if self.sym_maps.to_external.is_empty() {
            symbol
        } else {
            self.sym_maps.to_external[symbol.usize()]
        }
    }

    fn max_nulling_symbol(&self) -> Option<usize> {
        (0..self.num_rules())
            .filter_map(|action| self.nulling(action as u32).map(|(sym, _dir)| sym.usize()))
            .chain(
                self.eliminated_nulling_intermediate()
                    .iter()
                    .map(|&[_lhs, rhs0, _rhs1]| rhs0.usize()),
            )
            .max()
    }

    fn dot_before_eof(&self) -> Dot {
        self.dot_before_eof
    }
}
