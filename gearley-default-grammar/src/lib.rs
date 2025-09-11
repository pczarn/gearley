#![forbid(unsafe_code)]

use std::{cmp, iter};

use bit_matrix::row::BitSlice;
use bit_matrix::BitMatrix;
use cfg::classify::CfgClassifyExt;
use cfg::predict_sets::{FirstSets, FollowSets, PredictSets};
use cfg::symbol_bit_matrix::{CfgSymbolBitMatrixExt, Remap};
use cfg_symbol::intern::Mapping;

use cfg::history::earley::{
    EventAndDistance, EventId, ExternalDottedRule, ExternalOrigin, MinimalDistance,
    NullingEliminated,
};
use cfg::{Cfg, CfgRule, Symbol, SymbolBitSet, SymbolName, SymbolSource};

use gearley_grammar::{ForestInfo, Grammar, NullingIntermediateRule, PredictionTransition};
use gearley_vec2d::Vec2d;

use log::trace;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(miniserde::Serialize, miniserde::Deserialize, Clone, Default, Debug)]
struct Column {
    syms: Vec<Option<Symbol>>,
    events: Vec<EventAndDistance>,
    tracing: Vec<ExternalDottedRule>,
}

type Dot = u32;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(miniserde::Serialize, miniserde::Deserialize, Clone, Default, Debug)]
struct DotInfo {
    // For column 0: lhs.
    // For column 1 and 2: rhs.
    predot: Option<Symbol>,
    event: Option<Symbol>,
    tracing: Option<ExternalDottedRule>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(miniserde::Serialize, miniserde::Deserialize, Clone, Default, Debug)]
pub struct DefaultGrammar {
    start_sym: Symbol,
    original_start_sym: Symbol,
    has_trivial_derivation: bool,
    sof_sym: Symbol,
    eof_sym: Symbol,
    dot_before_eof: Dot,
    size: DefaultGrammarSize,

    prediction_matrix: BitMatrix,
    // Inverse prediction lookup.
    completions: Vec2d<PredictionTransition>,
    gen_completions: Vec<[Option<PredictionTransition>; 2]>,

    lr_sets: BitMatrix,

    // Rules stored in column-major order.
    // Arrays of events and tracing info.
    columns: [Column; 3],
    // Rule origin preserved for post-parse actions.
    // Mapping between external and internal symbols.
    sym_maps: Mapping,

    scan_prediction_matrix: BitMatrix,

    // For the forest
    forest_info: ForestInfo,
}

struct CompletionTable {
    completions: Vec<Vec<PredictionTransition>>,
    gen_completions: Vec<[Option<PredictionTransition>; 2]>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(miniserde::Serialize, miniserde::Deserialize, Clone, Default, Debug)]
pub struct DefaultGrammarSize {
    pub syms: usize,
    pub gensyms: usize,
    pub rules: usize,
    pub internal_syms: usize,
    pub external_syms: usize,
}

impl DefaultGrammar {
    fn new() -> Self {
        Default::default()
    }

    pub fn from_grammar(mut grammar: Cfg) -> Self {
        grammar.make_proper();
        trace!("make_proper: {:?}", grammar);
        grammar.wrap_input();
        trace!("wrap_input: {:?}", grammar);
        let nulling = grammar.binarize_and_eliminate_nulling_rules();
        println!(
            "{:?}",
            grammar
                .rules()
                .map(|rule| rule.history.nullable())
                .collect::<Vec<_>>()
        );
        trace!("binarize_and_eliminate_nulling_rules: {:?}", grammar);
        trace!("nulling: {:?}", nulling);
        let maps = Self::remap_symbols(&mut grammar);
        #[derive(Debug)]
        #[allow(dead_code)]
        struct SymWithName {
            sym: Symbol,
            name: Option<SymbolName>,
        }
        trace!(
            "to_external: mapping {:?}",
            maps.to_external
                .iter()
                .copied()
                .zip(grammar.sym_source().names())
                .map(|(sym, name)| SymWithName { sym, name })
                .collect::<Vec<_>>()
        );
        trace!("to_internal: mapping {:?}", maps.to_internal);
        trace!("remap_symbols: {:?}", grammar);
        Self::sort_rules_by_lhs(&mut grammar);
        trace!("sort_rules_by_lhs: {:?}", grammar);
        let mut has_predicts = DefaultGrammar::new();
        let num_gensyms = DefaultGrammar::find_gensyms(&grammar)
            .bit_vec()
            .iter()
            .rev()
            .filter(|is_gensym| *is_gensym)
            .count();
        has_predicts.size = DefaultGrammarSize {
            rules: grammar.rules().count(),
            syms: grammar.num_syms() - num_gensyms,
            gensyms: num_gensyms,
            external_syms: 0,
            internal_syms: 0,
        };
        let rules_by_rhs0 = has_predicts.compute_rules_by_rhs0(&grammar);
        has_predicts.populate_prediction_matrix(&grammar, &rules_by_rhs0[..]);
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Rule {
            lhs: Symbol,
            rhs0: Symbol,
            rhs1: Option<Symbol>,
        }
        #[derive(Debug)]
        #[allow(dead_code)]
        struct SymInfo {
            external: Symbol,
            internal: Symbol,
            name: Option<SymbolName>,
            rules_lhs: Vec<Rule>,
            rules_rhs: Vec<Rule>,
            predicts: Vec<Symbol>,
            predicts_by: Vec<Symbol>,
        }
        let sym_and_name_to_sym_info = |external, name, internal| {
            let rules_lhs = grammar
                .rules()
                .filter_map(|r| {
                    if r.lhs == internal {
                        Some(Rule {
                            lhs: r.lhs,
                            rhs0: r.rhs[0],
                            rhs1: r.rhs.get(1).copied(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            let rules_rhs = grammar
                .rules()
                .filter_map(|r| {
                    if r.rhs[0] == internal || r.rhs.get(1).copied() == Some(internal) {
                        Some(Rule {
                            lhs: r.lhs,
                            rhs0: r.rhs[0],
                            rhs1: r.rhs.get(1).copied(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            let mut predicts = vec![];
            let mut predicts_by = vec![];
            if internal.usize() < grammar.num_syms() - num_gensyms {
                predicts = has_predicts
                    .prediction_matrix
                    .iter_row(internal.usize())
                    .zip(SymbolSource::generate_fresh().take(grammar.num_syms() - num_gensyms))
                    .filter_map(|(bit, sym)| if bit { Some(sym) } else { None })
                    .collect();
                predicts_by = SymbolSource::generate_fresh()
                    .take(grammar.num_syms() - num_gensyms)
                    .filter(|&sym| has_predicts.prediction_matrix[(sym.usize(), internal.usize())])
                    .collect();
            }
            SymInfo {
                internal,
                external,
                name,
                rules_lhs,
                rules_rhs,
                predicts,
                predicts_by,
            }
        };
        for ((external, name), internal) in maps
            .to_external
            .iter()
            .copied()
            .take(grammar.num_syms() - num_gensyms)
            .zip(grammar.sym_source().names())
            .zip(SymbolSource::generate_fresh())
        {
            trace!(
                "symbols.sym: {:?}",
                sym_and_name_to_sym_info(external, name, internal)
            );
        }
        Self::from_processed_grammars(grammar, maps, &nulling)
    }

    fn remap_symbols(grammar: &mut Cfg) -> Mapping {
        let gensyms = Self::find_gensyms(grammar);
        trace!("gensyms: {:?}", gensyms);
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
        assert_eq!(grammar.wrapped_roots().len(), 1);
        let end_of_input = grammar.wrapped_roots().first().unwrap().end_of_input;
        let mut not_gensyms = gensyms.clone();
        not_gensyms.negate();
        for not_gensym in not_gensyms.iter() {
            // TODO fix argument order (????)
            for (dst, src) in (&mut *order)[not_gensym.usize()]
                .iter_blocks_mut()
                .zip(gensyms.bit_vec().blocks())
            {
                *dst |= src;
            }
            order.set(end_of_input.usize(), not_gensym.usize(), true);
        }
        // the order above is not transitive.
        // We modify it so that if `A < B` and `B < C` then `A < C`
        order.transitive_closure();
        trace!("order_transitive_closure: {:?}", order);
        let mut remap = Remap::new(grammar);
        remap.reorder_symbols(|left, right| {
            if order[(left, right)] {
                cmp::Ordering::Less
            } else if order[(right, left)] {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });
        remap.remove_unused_symbols();
        remap.get_mapping()
    }

    fn sort_rules_by_lhs(grammar: &mut Cfg) {
        grammar.sort_by(|a, b| a.lhs.cmp(&b.lhs));
    }

    fn find_gensyms(grammar: &Cfg) -> SymbolBitSet {
        // `order` describes relation `A < B`.
        let mut occurrences = vec![(0u32, 0u32, 0u32); grammar.num_syms()];
        let mut gensyms = SymbolBitSet::from_elem(&grammar, false);
        for rule in grammar.rules() {
            if rule.rhs.len() == 2 && rule.lhs != rule.rhs[0] {
                occurrences[rule.lhs.usize()].0 += 1;
                occurrences[rule.rhs[0].usize()].1 += 1;
            }
            for sym in rule.rhs.iter().skip(1) {
                occurrences[sym.usize()].2 += 1;
            }
        }
        for rule in grammar.rules() {
            if occurrences[rule.lhs.usize()] == (1, 1, 0) && rule.history.origin().is_null() {
                gensyms.set(rule.lhs, true);
            }
        }
        gensyms
    }

    pub fn from_processed_grammars(grammar: Cfg, maps: Mapping, nulling: &Cfg) -> Self {
        let mut result = DefaultGrammar::new();
        result.populate_sizes(&grammar, &maps);
        result.populate_maps(maps);
        result.populate_grammar(&grammar);
        result.populate_nulling(nulling);
        trace!("result: {:?}", result);
        result
    }

    fn populate_sizes(&mut self, grammar: &Cfg, maps: &Mapping) {
        let num_gensyms = Self::find_gensyms(grammar)
            .bit_vec()
            .iter()
            .rev()
            .filter(|is_gensym| *is_gensym)
            .count();
        self.size = DefaultGrammarSize {
            rules: grammar.rules().count(),
            syms: grammar.num_syms() - num_gensyms,
            gensyms: num_gensyms,
            external_syms: maps.to_internal.len(),
            internal_syms: maps.to_external.len(),
        };
        trace!("sizes: {:?}", self.size);
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
        let wrapped_root = grammar
            .wrapped_roots()
            .first()
            .copied()
            .expect("start symbol not found");
        self.start_sym = wrapped_root.root;
        self.sof_sym = wrapped_root.start_of_input;
        self.eof_sym = wrapped_root.end_of_input;
        self.dot_before_eof = grammar
            .rules()
            .position(|rule| rule.rhs.get(1) == Some(&wrapped_root.end_of_input))
            .unwrap() as u32;
        self.original_start_sym = wrapped_root.inner_root;
        self.forest_info.sof = wrapped_root.start_of_input;
        self.forest_info.eof = wrapped_root.end_of_input;
        self.forest_info.start = wrapped_root.inner_root;
    }

    fn populate_grammar_with_lhs(&mut self, grammar: &Cfg) {
        self.columns[0]
            .syms
            .extend(grammar.rules().map(|rule| Some(rule.lhs)));
    }

    fn populate_grammar_with_rhs(&mut self, grammar: &Cfg) {
        self.columns[1].syms = grammar.column(1).map(|dot| dot.postdot).collect();
        self.columns[2].syms = grammar.column(2).map(|dot| dot.postdot).collect();
    }

    fn populate_grammar_with_history(&mut self, grammar: &Cfg) {
        self.forest_info
            .eval
            .extend(grammar.rules().map(|rule| rule.history.origin()));
        println!(
            "{:?}",
            grammar
                .rules()
                .map(|rule| rule.history.origin())
                .collect::<Vec<_>>()
        );
        self.forest_info
            .nulling_eliminated
            .extend(grammar.rules().map(|rule| rule.history.nullable()));

        self.populate_grammar_with_events_rhs(grammar);
        self.populate_grammar_with_trace_rhs(grammar);
    }

    fn populate_grammar_with_events_rhs(&mut self, grammar: &Cfg) {
        self.columns[1].events = grammar
            .column(1)
            .map(|dot| dot.earley.unwrap().event_and_distance())
            .collect();
        self.columns[2].events = grammar
            .column(2)
            .map(|dot| dot.earley.unwrap().event_and_distance())
            .collect();
    }

    fn populate_grammar_with_trace_rhs(&mut self, grammar: &Cfg) {
        self.columns[1].tracing = grammar
            .column(1)
            .map(|dot| dot.earley.unwrap().trace())
            .collect();
        self.columns[2].tracing = grammar
            .column(2)
            .map(|dot| dot.earley.unwrap().trace())
            .collect();
    }

    fn populate_maps(&mut self, maps: Mapping) {
        self.sym_maps = maps;
    }

    fn populate_predictions(&mut self, grammar: &Cfg) {
        let rules_by_rhs0 = self.compute_rules_by_rhs0(grammar);
        self.populate_prediction_matrix(grammar, &rules_by_rhs0[..]);
        self.populate_prediction_events(grammar);
        self.populate_completion_tables(grammar, &rules_by_rhs0[..]);
        self.populate_lr_sets(grammar);
    }

    fn compute_rules_by_rhs0(&self, grammar: &Cfg) -> Vec<CfgRule> {
        let mut result: Vec<_> = grammar.rules().cloned().collect();
        result.sort_by_key(|rule| rule.rhs[0]);
        result
    }

    fn populate_prediction_matrix(&mut self, grammar: &Cfg, rules_by_rhs0: &[CfgRule]) {
        self.prediction_matrix = BitMatrix::new(self.size.syms, self.size.syms);
        // Precompute DFA.
        if grammar.rules().any(|r| r.rhs.len() == 0) {
            trace!("{}", grammar.stringify_to_bnf());
        }
        trace!("{}", grammar.stringify_to_bnf());
        let mut times = 100;
        for rule in grammar.rules() {
            if rule.rhs[0].usize() < self.size.syms {
                let mut lhs = rule.lhs.usize();
                while lhs >= self.size.syms {
                    if times > 0 {
                        trace!("{:?}", lhs);
                    }
                    let idx = rules_by_rhs0
                        .binary_search_by_key(&lhs, |elem| elem.rhs[0].usize())
                        .expect("lhs not found at rhs0 of any rule");
                    lhs = rules_by_rhs0[idx].lhs.usize();
                    if times > 0 {
                        trace!("{:?}", (lhs, idx, self.size.syms));
                        times -= 1;
                    }
                }
                self.prediction_matrix.set(lhs, rule.rhs[0].usize(), true);
            }
        }
        // Prediction relation is transitive.
        self.prediction_matrix.transitive_closure();
        // Prediction relation is reflexive.
        self.prediction_matrix.reflexive_closure();
    }

    fn populate_lr_sets(&mut self, grammar: &Cfg) {
        // A ::= gen0 B
        // gen0 ::= gen1 C
        // gen0 ::= gen1
        // gen1 ::= D E
        // input: D E C B
        // gens never have FIRST because they do not appear on RHS
        // gens only have FOLLOW
        // FOLLOW(gen1) = {FIRST(C), FOLLOW(gen0)}
        // FOLLOW(gen0) = {FIRST(B)}
        let syms = self.size.syms;
        let mut follow_matrix = BitMatrix::new(syms, syms);
        let mut first_matrix = BitMatrix::new(syms, syms);
        let first_sets = FirstSets::new(grammar);
        for (outer, inner) in first_sets.predict_sets() {
            for inner_sym in inner.iter().copied() {
                if outer.usize() < syms && inner_sym.usize() < syms {
                    first_matrix.set(outer.usize(), inner_sym.usize(), true);
                }
            }
        }
        first_matrix.reflexive_closure();
        let follow_sets = FollowSets::new(grammar, first_sets.predict_sets());
        for (before, after) in follow_sets.predict_sets().into_iter() {
            for after_sym in after.iter().copied() {
                if before.usize() < syms && after_sym.usize() < syms {
                    follow_matrix.set(before.usize(), after_sym.usize(), true);
                }
            }
        }
        self.lr_sets = BitMatrix::new(syms * 2, syms);
        for i in 0..self.size.syms {
            for (dst, &src) in self.lr_sets[i * 2]
                .iter_blocks_mut()
                .zip(first_matrix[i].iter_blocks())
            {
                *dst = src;
            }
            for (dst, &src) in self.lr_sets[i * 2 + 1]
                .iter_blocks_mut()
                .zip(follow_matrix[i].iter_blocks())
            {
                *dst = src;
            }
        }
    }

    fn populate_completion_tables(&mut self, grammar: &Cfg, rules_by_rhs0: &[CfgRule]) {
        let table = self.compute_completion_table(grammar, rules_by_rhs0);
        self.completions
            .extend(table.completions.into_iter().map(|v| v.into_iter()));
        self.gen_completions
            .extend(table.gen_completions.into_iter());
    }

    fn compute_completion_table(
        &self,
        grammar: &Cfg,
        rules_by_rhs0: &[CfgRule],
    ) -> CompletionTable {
        let mut table = CompletionTable {
            completions: iter::repeat(vec![]).take(self.size.syms).collect(),
            gen_completions: vec![[None; 2]; self.size.gensyms],
        };

        let mut unary_rules = vec![];
        let mut binary_rules = vec![];
        // check for ordering same as self.rules
        for (rule, dot) in grammar.rules().zip(0..) {
            let is_unary = rule.rhs.get(1).is_none();
            let rhs0_sym = rule.rhs[0];
            let mut lhs = rule.lhs;
            while lhs.usize() >= self.size.syms {
                let idx = rules_by_rhs0
                    .binary_search_by_key(&lhs.usize(), |elem| elem.rhs[0].usize())
                    .expect("lhs not found at rhs0 of any rule");
                lhs = rules_by_rhs0[idx].lhs;
            }
            if is_unary {
                unary_rules.push((
                    rhs0_sym.usize(),
                    PredictionTransition {
                        symbol: lhs,
                        dot,
                        is_unary,
                    },
                ));
            } else {
                binary_rules.push((
                    rhs0_sym.usize(),
                    PredictionTransition {
                        symbol: lhs,
                        dot,
                        is_unary,
                    },
                ));
            }
        }
        // order is very important: first all binary, then all unary
        for (rhs0_sym, transition) in binary_rules.into_iter().chain(unary_rules.into_iter()) {
            if rhs0_sym >= self.size.syms {
                table.gen_completions[rhs0_sym - self.size.syms][transition.is_unary as usize] =
                    Some(transition);
            } else {
                table.completions[rhs0_sym].push(transition);
            }
        }
        table
    }

    fn populate_prediction_events(&mut self, grammar: &Cfg) {
        let iter_events_pred = iter::repeat((EventId::null(), MinimalDistance::null()))
            .take(self.size.syms + self.size.gensyms + 1);
        self.columns[0].events.extend(iter_events_pred);
        let iter_trace_pred =
            iter::repeat(ExternalDottedRule::null()).take(self.size.syms + self.size.gensyms + 1);
        self.columns[0].tracing.extend(iter_trace_pred);
        // Prediction event and tracing.
        for (dot, rule) in grammar.column(0).zip(grammar.rules()) {
            if let Some(rule_dot) = dot.earley {
                self.columns[0].events[rule.lhs.usize()] = (rule_dot.event, rule_dot.distance);
                self.columns[0].tracing[rule.lhs.usize()] = rule_dot.trace;
            }
        }
    }

    fn populate_nulling(&mut self, nulling: &Cfg) {
        self.has_trivial_derivation = !nulling.is_empty();
        let iter_nulling_intermediate = nulling
            .rules()
            .filter_map(|rule| {
                if rule.history.origin().is_null() && rule.rhs.len() == 2 {
                    Some([
                        self.to_internal(rule.lhs).unwrap(),
                        self.to_internal(rule.rhs[0]).unwrap(),
                        self.to_internal(rule.rhs[1]).unwrap(),
                    ])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        self.forest_info
            .nulling_intermediate_rules
            .extend(iter_nulling_intermediate);
    }
}

impl Grammar for DefaultGrammar {
    #[inline]
    fn sof(&self) -> Symbol {
        self.sof_sym
    }

    #[inline]
    fn eof(&self) -> Symbol {
        self.eof_sym
    }

    fn lhs_lr_set(&self, lhs: Symbol) -> &BitSlice {
        &self.lr_sets[lhs.usize() * 2 + 1]
    }

    fn lookahead_set(&self, dot: Dot) -> &BitSlice {
        &self.lr_sets[self.rhs1_or_lhs(dot).usize() * 2 + self.get_rhs1(dot).is_none() as usize]
    }

    fn rhs1_or_lhs(&self, dot: Dot) -> Symbol {
        if let Some(rhs1) = self.get_rhs1(dot) {
            rhs1
        } else {
            self.get_lhs(dot)
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
    fn num_gensyms(&self) -> usize {
        self.size.gensyms
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
        self.forest_info
            .nulling_eliminated
            .get(pos as usize)
            .and_then(|&ne| ne)
    }

    #[inline]
    fn events(&self) -> (&[EventAndDistance], &[EventAndDistance]) {
        (&self.columns[1].events[..], &self.columns[2].events[..])
    }

    #[inline]
    fn trace(&self) -> [&[ExternalDottedRule]; 3] {
        [
            &self.columns[0].tracing[..],
            &self.columns[1].tracing[..],
            &self.columns[2].tracing[..],
        ]
    }

    #[inline]
    fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        self.columns[1].syms[dot as usize]
    }

    #[inline]
    fn rhs1(&self) -> &[Option<Symbol>] {
        &self.columns[1].syms[..]
    }

    #[inline]
    fn get_lhs(&self, dot: Dot) -> Symbol {
        self.columns[0].syms[dot as usize].unwrap()
    }

    #[inline]
    fn external_origin(&self, dot: Dot) -> ExternalOrigin {
        self.forest_info.eval.get(dot as usize).cloned().unwrap()
    }

    fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
        &*self.forest_info.nulling_intermediate_rules
    }

    #[inline(always)]
    fn completions(&self, sym: Symbol) -> &[PredictionTransition] {
        &self.completions[sym.usize()]
    }

    fn gen_completion(&self, sym: Symbol) -> [Option<PredictionTransition>; 2] {
        self.gen_completions[sym.usize() - self.size.syms]
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

    fn dot_before_eof(&self) -> Dot {
        self.dot_before_eof
    }

    fn forest_info(&self) -> ForestInfo {
        self.forest_info.clone()
    }
}
