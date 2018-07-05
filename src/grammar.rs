use std::borrow::Cow;
use std::iter;
use std::mem::{align_of, size_of};
use std::slice;
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;

use bit_matrix::BitMatrix;
use bit_matrix::submatrix::{BitSubMatrix, BitSubMatrixMut};
use cfg::{
    Cfg,
    BinarizedCfg,
    ContextFree,
    ContextFreeRef,
    GrammarRule,
    Symbol,
};
use cfg::rule::builder::RuleBuilder;
use cfg::rule::container::RuleContainer;
use cfg::sequence::builder::SequenceRuleBuilder;
use cfg::history::*;
use cfg::sequence::Sequence;
use cfg::usefulness::Usefulness;
use cfg::remap::{Remap, Mapping};
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
// Property (4): Rule IDs are ordered by the LHS symbol IDs.
//
// Property (5): IDs of unit rules are smaller than IDs of rules which they predict.
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
// RHS0 is unused during recognition, thanks to inverse prediction tables. Move or remove it.
// Store RHS1 and LHS in row-major instead of column-major order, so that the least significant bit
// tells us whether a dot is medial or completed. Or don't.
//
// Parameterize the representation over symbol type (u32, u16, u8).
//
// Use independent indices into RHS1 and LHS. Add a translation table.

/// Drop-in replacement for `cfg::Cfg` that traces relations between user-provided
/// and internal grammars.
#[derive(Default)]
pub struct Grammar {
    inherit: Cfg<History, History>,
    start: Option<Symbol>,
}

impl Grammar {
    pub fn new() -> Self {
        Grammar {
            inherit: Cfg::new(),
            start: None,
        }
    }

    pub fn set_start(&mut self, start: Symbol) {
        self.start = Some(start);
    }

    pub fn start(&self) -> Symbol {
        self.start.unwrap()
    }

    pub fn rule(&mut self, lhs: Symbol) -> RuleBuilder<&mut Cfg<History, History>, BuildHistory> {
        let rule_count = self.inherit.rules().count() + self.sequence_rules().len();
        self.inherit.rule(lhs).default_history(BuildHistory::new(rule_count))
    }

    pub fn sequence(&mut self, lhs: Symbol)
        -> SequenceRuleBuilder<History, &mut Vec<Sequence<History>>, BuildHistory>
    {
        let rule_count = self.inherit.rules().count() + self.sequence_rules().len();
        self.inherit.sequence(lhs).default_history(BuildHistory::new(rule_count))
    }

    pub fn binarize(&self) -> BinarizedGrammar {
        BinarizedGrammar {
            inherit: self.inherit.binarize(),
            start: self.start,
        }
    }

    pub fn to_internal_grammar(&self) -> InternalGrammar {
        InternalGrammar::from_grammar(self)
    }
}

impl Deref for Grammar {
    type Target = Cfg<History, History>;
    fn deref(&self) -> &Self::Target {
        &self.inherit
    }
}

impl DerefMut for Grammar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inherit
    }
}

#[derive(Clone, Default)]
pub struct BinarizedGrammar {
    inherit: BinarizedCfg<History>,
    start: Option<Symbol>,
}

impl BinarizedGrammar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_start(&mut self, start: Symbol) {
        self.start = Some(start);
    }

    pub fn start(&self) -> Symbol {
        self.start.unwrap()
    }
}

impl Deref for BinarizedGrammar {
    type Target = BinarizedCfg<History>;
    fn deref(&self) -> &Self::Target {
        &self.inherit
    }
}

impl DerefMut for BinarizedGrammar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inherit
    }
}

/// Default history.
pub struct BuildHistory {
    num_rules: usize,
}

impl BuildHistory {
    /// Creates default history.
    pub(in super) fn new(num_rules: usize) -> Self {
        BuildHistory { num_rules }
    }
}

impl HistorySource<History> for BuildHistory {
    fn build(&mut self, _lhs: Symbol, rhs: &[Symbol]) -> History {
        // for sequences, rhs.len() will be 1 or 2
        let ret = History::new(self.num_rules as u32, rhs.len());
        self.num_rules += 1;
        ret
    }
}

#[derive(Copy, Clone, Debug)]
pub(in super) struct PredictionTransition {
    pub symbol: Symbol,
    pub dot: Dot,
}

impl PredictionTransition {
    #[inline(always)]
    pub fn dot(self) -> DotKind {
        let top_bit = 0x8000_0000;
        if self.dot & top_bit == top_bit {
            DotKind::Completed(self.dot & !top_bit)
        } else {
            DotKind::Medial(self.dot)
        }
    }
}

pub(in super) enum DotKind {
    Medial(Dot),
    Completed(Dot),
}

#[derive(Clone)]
pub struct InternalGrammar {
    parts: InternalGrammarParts,
    prediction_matrix_ptr: *mut u32,
    // Inverse prediction lookup.
    inverse_prediction: *mut [PredictionTransition],
    inverse_prediction_index: *mut [u32],
    // array of events
    events_flat: *mut [Event],
    // 2-dimensional array for tracing
    tracing_flat: *mut [Option<ExternalDottedRule>],
    // Each rule can have only one eliminated nulling symbol.
    nulling_eliminated: *mut [NullingEliminated],
    // Rules stored in column-major order.
    rules_flat: *mut [Option<Symbol>],
    eval: *mut [RuleOrigin],
    // Mapping between external and internal symbols.
    to_external: *mut [Symbol],
    to_internal: *mut [Option<Symbol>],
    nulling_intermediate: *mut [NullingIntermediateRule],
}

#[derive(Clone)]
pub struct InternalGrammarParts {
    pub storage: Cow<'static, [u8]>,
    pub num_syms: usize,
    pub num_rules: usize,
    pub num_internal_syms: usize,
    pub num_external_syms: usize,
    pub num_nulling_intermediate: usize,
    pub start_sym: Symbol,
    pub trivial_derivation: bool,
}

pub(in super) type ExternalDottedRule = (u32, u32);
type RuleOrigin = Option<u32>;
type EventId = Optioned<u32>;
type MinimalDistance = Optioned<u32>;
pub(in super) type Event = (EventId, MinimalDistance);
type NullingEliminated = Option<(Symbol, bool)>;
type NullingIntermediateRule = (Symbol, Symbol, Symbol);

#[derive(Debug)]
struct InternalGrammarSlices<'a> {
    prediction_matrix: BitSubMatrix<'a>,
    // Inverse prediction lookup.
    inverse_prediction: &'a [PredictionTransition],
    inverse_prediction_index: &'a [u32],
    // array of events
    events_prediction: &'a [Event],
    // Length = num_rules
    events1: &'a [Event],
    events2: &'a [Event],
    eval: &'a [RuleOrigin],
    // Unzipped rules stored in column-major order.
    rules: RuleSlices<'a>,
    // array with information about dotted rules
    tracing_pred: &'a [Option<ExternalDottedRule>],
    nulling_eliminated: &'a [NullingEliminated],
    // Other length
    tracing: &'a [Option<ExternalDottedRule>],
    to_external: &'a [Symbol],
    to_internal: &'a [Option<Symbol>],
    nulling_intermediate: &'a [NullingIntermediateRule],
}

struct InternalGrammarSlicesMut<'a> {
    pub prediction_matrix: BitSubMatrixMut<'a>,
    // Inverse prediction lookup.
    pub inverse_prediction: &'a mut [PredictionTransition],
    pub inverse_prediction_index: &'a mut [u32],
    // array of events
    pub events_pred: &'a mut [Event],
    pub events1: &'a mut [Event],
    pub events2: &'a mut [Event],
    // array with information about dotted rules
    pub tracing_pred: &'a mut [Option<ExternalDottedRule>],
    pub tracing: &'a mut [Option<ExternalDottedRule>],
    // nulling
    pub nulling_eliminated: &'a mut [NullingEliminated],
    // Unzipped rules stored in column-major order.
    pub rules: RuleSlicesMut<'a>,
    pub eval: &'a mut [RuleOrigin],
    to_external: &'a mut [Symbol],
    to_internal: &'a mut [Option<Symbol>],
    nulling_intermediate: &'a mut [NullingIntermediateRule],
}

#[derive(Clone, Default, Debug)]
pub struct History {
    dots: Vec<RuleDot>,
    origin: RuleOrigin,
    nullable: NullingEliminated,
}

#[derive(Copy, Clone, Debug)]
struct RuleDot {
    event: Option<(EventId, ExternalDottedRule)>,
    distance: MinimalDistance,
}

impl RuleDot {
    fn new(id: u32, pos: usize) -> Self {
        RuleDot {
            event: Some((Optioned::none(), (id, pos as u32))),
            distance: Optioned::none(),
        }
    }

    pub fn none() -> Self {
        RuleDot {
            event: None,
            distance: Optioned::none(),
        }
    }

    pub(in super) fn trace(&self) -> Option<ExternalDottedRule> {
        self.event.map(|x| x.1)
    }
}

impl Into<Event> for RuleDot {
    fn into(self) -> Event {
        (self.event.and_then(|x| x.0.into()).into(), self.distance)
    }
}

impl History {
    pub fn new(id: u32, len: usize) -> Self {
        History {
            origin: Some(id),
            dots: (0 .. len + 1).map(|i| RuleDot::new(id, i)).collect(),
            ..History::default()
        }
    }
}

impl Action for History {
    fn no_op(&self) -> Self {
        History::default()
    }
}

impl Binarize for History {
    fn binarize<R>(&self, _rule: &R, depth: usize) -> Self {
        let none = RuleDot::none();
        let dots = if self.dots.is_empty() {
            [none; 3]
        } else {
            let dot_len = self.dots.len();
            if depth == 0 {
                if dot_len == 2 {
                    [self.dots[0], none, self.dots[1]]
                } else if dot_len >= 3 {
                    [self.dots[0], self.dots[dot_len - 2], self.dots[dot_len - 1]]
                } else {
                    [self.dots[0], none, none]
                }
            } else {
                [none, self.dots[dot_len - 2 - depth], none]
            }
        };

        let origin = if depth == 0 {
            self.origin
        } else {
            None
        };

        History {
            origin,
            dots: dots[..].to_vec(),
            nullable: self.nullable,
        }
    }
}

impl EliminateNulling for History {
    fn eliminate_nulling<R>(&self, rule: &R, subset: BinarizedRhsSubset) -> Self where
                R: GrammarRule {
        if let BinarizedRhsSubset::All = subset {
            History {
                origin: self.origin,
                ..History::default()
            }
        } else {
            let right = if let BinarizedRhsSubset::Right = subset { true } else { false };
            let sym = rule.rhs()[right as usize];
            History {
                nullable: Some((sym, right)),
                ..self.clone()
            }
        }
    }
}

#[derive(Copy, Clone)]
enum SymKind {
    Element,
    Separator,
    Other,
}

impl RewriteSequence for History {
    type Rewritten = History;

    fn top(&self, rhs: Symbol, sep: Option<Symbol>, new_rhs: &[Symbol]) -> Self {
        let mut bottom = self.bottom(rhs, sep, new_rhs);
        bottom.origin = self.origin;
        bottom
    }

    fn bottom(&self, rhs: Symbol, sep: Option<Symbol>, new_rhs: &[Symbol]) -> Self {
        //  -  sym (1) Sep (2)
        //  -  lhs (1) Sep (2) Rhs (1)
        //  -  lhs (0) Rhs (1)
        // (0) Rhs (1)
        // (0) Rhs (1) Sep (2) Rhs (1)
        // (0) Rhs (1) Rhs (1)
        let syms = new_rhs.iter().map(|&sym| {
            if sym == rhs {
                SymKind::Element
            } else if Some(sym) == sep {
                SymKind::Separator
            } else {
                SymKind::Other
            }
        }).chain(iter::once(SymKind::Other));
        let mut to_left = SymKind::Other;
        let dots = syms.map(|to_right| {
            let dot = match (to_left, to_right) {
                (_, SymKind::Separator) => self.dots[1],
                (SymKind::Separator, _) => self.dots[2],
                (SymKind::Element, _)   => self.dots[1],
                (_, SymKind::Element)   => self.dots[0],
                _ => RuleDot::none()
            };
            to_left = to_right;
            dot
        }).collect();
        History {
            dots,
            ..History::default()
        }
    }
}

impl BinarizedGrammar {
    fn make_proper(mut self: BinarizedGrammar) -> BinarizedGrammar {
        let start = self.start();
        {
            let mut usefulness = Usefulness::new(&mut *self).reachable([start]);
            if !usefulness.all_useful() {
                println!("warning: grammar has useless rules");
                usefulness.remove_useless_rules();
            }
        };
        self
    }

    pub fn generate_special_start(mut self: BinarizedGrammar) -> BinarizedGrammar {
        let previous_start = self.start();
        let new_start = self.sym();
        let new_history = History {
            dots: vec![RuleDot::none(), RuleDot::none()],
            origin: None,
            nullable: None,
        };
        self.rule(new_start).rhs_with_history([previous_start], new_history);
        self.set_start(new_start);
        self
    }

    pub fn eliminate_nulling(mut self: BinarizedGrammar) -> (BinarizedGrammar, BinarizedGrammar) {
        let nulling_grammar = BinarizedGrammar {
            inherit: self.eliminate_nulling_rules(),
            start: Some(self.start()),
        };
        (self, nulling_grammar)
    }

    fn remap_symbols(mut self: BinarizedGrammar) -> (BinarizedGrammar, Mapping) {
        let num_syms = self.sym_source().num_syms();
        // `order` describes relation `A < B`.
        let mut order = BitMatrix::new(num_syms, num_syms);
        for rule in self.rules() {
            if rule.rhs().len() == 1 {
                let left = rule.lhs().usize();
                let right = rule.rhs()[0].usize();
                match left.cmp(&right) {
                    Ordering::Less => {
                        order.set(left, right, true);
                    }
                    Ordering::Greater => {
                        order.set(right, left, true);
                    }
                    Ordering::Equal => {}
                }
            }
        }
        // the order above is not transitive.
        // We modify it so that if `A < B` and `B < C` then `A < C`
        order.transitive_closure();
        let mut maps = {
            let mut remap = Remap::new(&mut *self);
            remap.remove_unused_symbols();
            remap.reorder_symbols(|left, right| {
                let (left, right) = (left.usize(), right.usize());
                if order[(left, right)] {
                    Ordering::Less
                } else if order[(right, left)] {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            remap.get_mapping()
        };
        let start = self.start();
        if let Some(internal_start) = maps.to_internal[start.usize()] {
            self.set_start(internal_start);
        } else {
            // The trivial grammar is a unique edge case -- the start symbol was removed.
            let internal_start = Symbol::from(maps.to_external.len());
            maps.to_internal[start.usize()] = Some(internal_start);
            maps.to_external.push(start);
            self.set_start(internal_start);
        }
        (self, maps)
    }

    // useless?
    pub fn process(self: BinarizedGrammar) -> (BinarizedGrammar, BinarizedGrammar) {
        let (grammar, nulling) = self.make_proper().eliminate_nulling();
        (grammar, nulling)
    }
}

impl InternalGrammar {
    pub fn from_grammar(grammar: &Grammar) -> Self {
        Self::from_binarized_grammar(grammar.binarize())
    }

    pub fn from_binarized_grammar(grammar: BinarizedGrammar) -> Self {
        let grammar = grammar.make_proper();
        Self::from_proper_binarized_grammar(grammar)
    }

    pub fn from_proper_binarized_grammar(grammar: BinarizedGrammar) -> Self {
        let (grammar, nulling) = grammar.eliminate_nulling();
        Self::from_processed_grammar(grammar, &nulling)
    }

    pub fn from_processed_grammar(grammar: BinarizedGrammar, nulling: &BinarizedGrammar) -> Self {
        let (grammar, maps) = grammar.remap_symbols();
        Self::from_processed_grammar_with_maps(grammar, &maps, nulling)
    }

    pub fn from_processed_grammar_with_maps(
        mut grammar: BinarizedGrammar,
        maps: &Mapping,
        nulling: &BinarizedGrammar)
        -> Self
    {
        grammar.sort_by(|a, b| a.lhs().cmp(&b.lhs()));
        let num_syms = grammar.sym_source().num_syms();
        let num_rules = grammar.rules().count();
        let trivial_derivation = nulling.rules().any(|rule| Some(rule.lhs()) == nulling.start);
        let nulling_intermediate = nulling.rules().filter_map(|rule| {
            if rule.history().origin.is_none() && rule.rhs().len() == 2 {
                Some((rule.lhs(), rule.rhs()[0], rule.rhs()[1]))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let mut result = InternalGrammar::from_parts(InternalGrammarParts {
            storage: Cow::Borrowed(&[]),
            num_syms,
            num_internal_syms: maps.to_external.len(),
            num_external_syms: maps.to_internal.len(),
            num_rules,
            start_sym: grammar.start(),
            trivial_derivation,
            num_nulling_intermediate: nulling_intermediate.len(),
        });
        result.populate_rules(&grammar, &maps);
        result.populate_nulling_rules(nulling_intermediate);
        let inverse_prediction = result.compute_inverse_prediction();
        result.populate_predictions(inverse_prediction);
        result
    }

    fn populate_rules(&mut self, grammar: &BinarizedGrammar, maps: &Mapping) {
        let num_rules = self.parts.num_rules;
        let mut slices = self.as_slices_mut();
        let (tracing1, tracing2) = slices.tracing.split_at_mut(num_rules);
        for elem in slices.events_pred.iter_mut() {
            *elem = (Optioned::none(), Optioned::none());
        }

        let iter = slices.rules.rules_mut()
                   .zip(slices.eval)
                   .zip(slices.events1)
                   .zip(slices.events2)
                   .zip(slices.nulling_eliminated)
                   .zip(tracing1.iter_mut())
                   .zip(tracing2.iter_mut());

        for (rule, ((((((mut dst_rule, eval), event1), event2), nulling), t1), t2))
                in grammar.rules().zip(iter) {
            *dst_rule.lhs = Some(rule.lhs());
            *dst_rule.rhs0 = Some(rule.rhs()[0]);
            *dst_rule.rhs1 = rule.rhs().get(1).cloned();
            *eval = rule.history().origin;
            *nulling = rule.history().nullable;
            if let Some(&(pred_event, pred_tracing)) = rule.history().dots[0].event.as_ref() {
                // Prediction event and tracing.
                slices.events_pred[rule.lhs().usize()] = (
                    pred_event,
                    rule.history().dots[0].distance
                );
                slices.tracing_pred[rule.lhs().usize()] = Some(pred_tracing);
            }
            *event1 = rule.history().dots[1].into();
            *event2 = rule.history().dots[2].into();
            *t1 = rule.history().dots[1].trace();
            *t2 = rule.history().dots[2].trace();
        }
        for (dst, src) in slices.to_external.iter_mut().zip(maps.to_external.iter()) {
            *dst = *src;
        }
        for (dst, src) in slices.to_internal.iter_mut().zip(maps.to_internal.iter()) {
            *dst = *src;
        }
    }

    fn populate_nulling_rules(&mut self, nulling_intermediate: Vec<(Symbol, Symbol, Symbol)>) {
        let slices = self.as_slices_mut();
        for (dst, src) in slices.nulling_intermediate.iter_mut().zip(nulling_intermediate) {
            *dst = src;
        }
    }

    fn compute_inverse_prediction(&self) -> Vec<Vec<PredictionTransition>> {
        let mut slices = self.as_slices();
        let mut inverse_prediction = iter::repeat(vec![]).take(self.parts.num_syms).collect::<Vec<_>>();

        for (dot, rule) in slices.rules.rules().enumerate() {
            let transition = PredictionTransition {
                symbol: rule.lhs,
                dot: if rule.rhs1() == None {
                    dot as Dot | 0x8000_0000
                } else {
                    dot as Dot
                }
            };
            inverse_prediction[rule.rhs0().usize()].push(transition);
        }
        inverse_prediction
    }

    fn populate_predictions(&mut self, inverse_prediction: Vec<Vec<PredictionTransition>>) {
        let num_syms = self.parts.num_syms;
        let mut slices = self.as_slices_mut();
        let prediction_matrix = &mut slices.prediction_matrix;
        // Precompute DFA.
        for rule in slices.rules.rules_mut() {
            prediction_matrix.set(rule.lhs().usize(), rule.rhs0().usize(), true);
        }
        slices.inverse_prediction_index[0] = 0;
        let mut idx = 0;
        let indices = slices.inverse_prediction_index[1..].iter_mut();
        for (src, dst) in inverse_prediction.iter().zip(indices) {
            idx += src.len() as u32;
            *dst = idx;
        }
        for (src, dst) in inverse_prediction.into_iter().flat_map(|v| v.into_iter())
                          .zip(slices.inverse_prediction.iter_mut()) {
            *dst = src;
        }

        prediction_matrix.transitive_closure();
        // Prediction relation is reflexive.
        for i in 0..num_syms {
            prediction_matrix.set(i, i, true);
        }
    }

    pub fn to_parts(&self) -> InternalGrammarParts {
        self.parts.clone()
    }

    #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
    pub fn from_parts(mut parts: InternalGrammarParts) -> Self {
        let (len, offset) = grammar_storage_offsets(&parts);
        let end = offset[LEN_LEN];
        if parts.storage.is_empty() {
            parts.storage = Cow::Owned(vec![0; end]);
        }
        assert!(parts.storage.len() >= end);
        let field = unsafe {(
            parts.storage[offset[0]..].as_ptr() as *mut _,
            slice::from_raw_parts_mut(parts.storage[offset[1]..].as_ptr() as *mut _, len[1]),
            slice::from_raw_parts_mut(parts.storage[offset[2]..].as_ptr() as *mut _, len[2]),
            slice::from_raw_parts_mut(parts.storage[offset[3]..].as_ptr() as *mut _, len[3]),
            slice::from_raw_parts_mut(parts.storage[offset[4]..].as_ptr() as *mut _, len[4]),
            slice::from_raw_parts_mut(parts.storage[offset[5]..].as_ptr() as *mut _, len[5]),
            slice::from_raw_parts_mut(parts.storage[offset[6]..].as_ptr() as *mut _, len[6]),
            slice::from_raw_parts_mut(parts.storage[offset[7]..].as_ptr() as *mut _, len[7]),
            slice::from_raw_parts_mut(parts.storage[offset[8]..].as_ptr() as *mut _, len[8]),
            slice::from_raw_parts_mut(parts.storage[offset[9]..].as_ptr() as *mut _, len[9]),
            slice::from_raw_parts_mut(parts.storage[offset[10]..].as_ptr() as *mut _, len[10]),
        )};
        InternalGrammar {
            prediction_matrix_ptr: field.0,
            rules_flat: field.1,
            eval: field.2,
            nulling_eliminated: field.3,
            inverse_prediction_index: field.4,
            inverse_prediction: field.5,
            events_flat: field.6,
            tracing_flat: field.7,
            to_external: field.8,
            to_internal: field.9,
            nulling_intermediate: field.10,
            parts,
        }
    }

    #[inline]
    fn as_slices_mut(&mut self) -> InternalGrammarSlicesMut {
        unsafe {
            let &InternalGrammarParts { num_rules, num_syms, .. } = &self.parts;
            let (rhs0, rest) = (*self.rules_flat).split_at_mut(num_rules);
            let (rhs1, lhs) = rest.split_at_mut(num_rules);
            let (events_pred, rest) = (*self.events_flat).split_at_mut(num_syms);
            let (events1, events2) = rest.split_at_mut(num_rules);
            let (tracing_pred, tracing) = (*self.tracing_flat).split_at_mut(num_syms);
            let prediction_matrix = BitSubMatrixMut::from_raw_parts(
                self.prediction_matrix_ptr as *mut _,
                num_syms,
                num_syms
            );
            InternalGrammarSlicesMut {
                prediction_matrix,
                // prediction lookup
                inverse_prediction: &mut *self.inverse_prediction,
                inverse_prediction_index: &mut *self.inverse_prediction_index,
                // array of events
                events_pred,
                events1,
                events2,
                //
                tracing_pred,
                tracing,
                // nulling
                nulling_eliminated: &mut *self.nulling_eliminated,
                // Unzipped rules stored in column-major order.
                rules: RuleSlicesMut {
                    lhs,
                    rhs0,
                    rhs1,
                },
                eval: &mut *self.eval,
                to_external: &mut *self.to_external,
                to_internal: &mut *self.to_internal,
                nulling_intermediate: &mut *self.nulling_intermediate,
            }
        }
    }

    #[inline(always)]
    fn as_slices(&self) -> InternalGrammarSlices {
        unsafe {
            let &InternalGrammarParts { num_rules, num_syms, .. } = &self.parts;
            let (events_pred, rest) = (*self.events_flat).split_at(num_syms);
            let (events1, events2) = rest.split_at(num_rules);
            let (tracing_pred, tracing) = (*self.tracing_flat).split_at(num_syms);
            let prediction_matrix = BitSubMatrix::from_raw_parts(
                self.prediction_matrix_ptr as *const _,
                num_syms,
                num_syms
            );
            InternalGrammarSlices {
                prediction_matrix,
                // prediction lookup
                inverse_prediction: &*self.inverse_prediction,
                inverse_prediction_index: &*self.inverse_prediction_index,
                // array of events
                events_prediction: events_pred,
                events1,
                events2,
                //
                tracing_pred,
                tracing,
                // nulling
                nulling_eliminated: &*self.nulling_eliminated,
                // Unzipped rules stored in column-major order.
                rules: self.rules(),
                eval: &*self.eval,
                to_external: &*self.to_external,
                to_internal: &*self.to_internal,
                nulling_intermediate: &*self.nulling_intermediate,
            }
        }
    }

    #[inline]
    pub(in super) fn prediction_matrix(&self) -> BitSubMatrix {
        self.as_slices().prediction_matrix
    }

    #[inline(always)]
    pub(in super) fn inverse_prediction(&self) -> &[PredictionTransition] {
        unsafe { &*self.inverse_prediction }
    }

    #[inline]
    pub(in super) fn num_syms(&self) -> usize {
        self.parts.num_syms
    }

    #[inline]
    pub(in super) fn num_rules(&self) -> usize {
        self.parts.num_rules
    }

    #[inline]
    pub(in super) fn num_pos(&self) -> usize {
        self.parts.num_rules * 2
    }

    #[inline]
    pub(in super) fn start_sym(&self) -> Symbol {
        self.parts.start_sym
    }

    #[inline]
    pub(in super) fn has_trivial_derivation(&self) -> bool {
        self.parts.trivial_derivation
    }

    #[inline]
    pub(in super) fn nulling(&self, pos: u32) -> NullingEliminated {
        unsafe {
            let nulling_eliminated = &*self.nulling_eliminated;
            nulling_eliminated.get(pos as usize).and_then(|&ne| ne)
        }
    }

    #[inline]
    pub(in super) fn events(&self) -> (&[Event], &[Event]) {
        unsafe {
            let events_flat = &*self.events_flat;
            events_flat.split_at(self.num_syms())
        }
    }

    #[inline]
    pub(in super) fn trace(&self) -> [&[Option<ExternalDottedRule>]; 3] {
        let slices = self.as_slices();
        let (trace1, trace2) = slices.tracing.split_at(self.num_rules());
        [slices.tracing_pred, trace1, trace2]
    }

    #[inline]
    pub(in super) fn complete_over(&self, dot: Dot, sym: Symbol) -> bool {
        let rhs1 = self.rules().rhs1;
        rhs1[dot as usize] == Some(sym)
    }

    #[inline]
    pub(in super) fn rules(&self) -> RuleSlices {
        unsafe {
            let (rhs0, rest) = (*self.rules_flat).split_at(self.num_rules());
            let (rhs1, lhs) = rest.split_at(self.num_rules());
            RuleSlices {
                lhs,
                rhs0,
                rhs1,
            }
        }
    }

    #[inline]
    pub(in super) fn get_rhs0(&self, dot: Dot) -> Symbol {
        let rules = self.rules().rhs0;
        rules[dot as usize].unwrap()
    }

    #[inline]
    pub(in super) fn get_rhs1(&self, dot: Dot) -> Option<Symbol> {
        let rules = self.rules().rhs1;
        rules[dot as usize]
    }

    #[inline]
    pub(in super) fn get_lhs(&self, dot: Dot) -> Symbol {
        let rules = self.rules().lhs;
        rules[dot as usize].unwrap()
    }

    #[inline]
    pub(in super) fn get_eval(&self, dot: Dot) -> RuleOrigin {
        unsafe {
            (&*self.eval).get(dot as usize).and_then(|&eval| eval)
        }
    }

    pub(in super) fn eliminated_nulling_intermediate(&self) -> &[NullingIntermediateRule] {
        unsafe {
            &*self.nulling_intermediate
        }
    }

    #[inline(always)]
    pub(in super) fn inverse_prediction_of(&self, sym: Symbol) -> &[PredictionTransition] {
        let slices = self.as_slices();
        let idxs = &slices.inverse_prediction_index[sym.usize() .. sym.usize() + 2];
        let range = idxs[0] as usize .. idxs[1] as usize;
        &self.inverse_prediction()[range]
    }

    #[inline(always)]
    pub(in super) fn to_internal(&self, symbol: Symbol) -> Option<Symbol> {
        if self.parts.num_external_syms == 0 {
            Some(symbol)
        } else {
            self.as_slices().to_internal[symbol.usize()]
        }
    }

    #[inline]
    pub(in super) fn to_external(&self, symbol: Symbol) -> Symbol {
        if self.parts.num_internal_syms == 0 {
            symbol
        } else {
            self.as_slices().to_external[symbol.usize()]
        }
    }
}

const LEN_LEN: usize = 11;
const OFFSET_LEN: usize = LEN_LEN + 1;

#[inline]
fn grammar_storage_offsets(parts: &InternalGrammarParts)
    -> ([usize; LEN_LEN], [usize; OFFSET_LEN])
{
    // num_rules * (3 * 1 + 2) * 4 bytes  =  num_rules * 20 bytes
    let &InternalGrammarParts {
        num_syms,
        num_rules,
        num_internal_syms,
        num_external_syms,
        num_nulling_intermediate,
        ..
    } = parts;
    let pred_columns = (num_syms + 32 - 1) / 32;
    let table = [
        // prediction matrix
        (pred_columns * num_syms,  size_and_align_of::<u32>()),
        (num_rules * 3,            size_and_align_of::<Option<Symbol>>()),
        (num_rules,                size_and_align_of::<RuleOrigin>()),
        (num_rules,                size_and_align_of::<NullingEliminated>()),
        // indices into inverse_prediction
        (num_syms + 1,             size_and_align_of::<u32>()),
        (num_rules,                size_and_align_of::<PredictionTransition>()),
        (num_syms + num_rules * 2, size_and_align_of::<Event>()),
        (num_syms + num_rules * 2, size_and_align_of::<Option<ExternalDottedRule>>()),
        (num_internal_syms,        size_and_align_of::<Symbol>()),
        (num_external_syms,        size_and_align_of::<Option<Symbol>>()),
        (num_nulling_intermediate, size_and_align_of::<NullingIntermediateRule>()),
    ];
    let mut len_ary = [0; LEN_LEN];
    let mut offset_ary = [0; OFFSET_LEN];
    let mut cur_offset = 0;
    {
        let src = table[..].iter().cloned().zip(
            table[1..].iter().map(|elem| (elem.1).1).chain(iter::once(1))
        );
        let dst = len_ary.iter_mut().zip(offset_ary[1..].iter_mut());

        for (((len, (size, _)),
              align),
             (dst_len, offset)) in src.zip(dst) {
            *dst_len = len;
            cur_offset = round_up_to_next(cur_offset + size * len, align);
            *offset = cur_offset;
        }
    };
    (len_ary, offset_ary)
}

#[inline]
fn size_and_align_of<T>() -> (usize, usize) {
    (size_of::<T>(), align_of::<T>())
}

#[inline]
fn round_up_to_next(unrounded: usize, target_alignment: usize) -> usize {
    assert!(target_alignment.is_power_of_two());
    (unrounded + target_alignment - 1) & !(target_alignment - 1)
}

pub(in super) struct BinaryRule {
    lhs: Symbol,
    rhs0: Symbol,
    rhs1: Option<Symbol>,
}

impl BinaryRule {
    fn new(lhs: Symbol, rhs0: Symbol, rhs1: Option<Symbol>) -> Self {
        BinaryRule {
            lhs,
            rhs0,
            rhs1,
        }
    }

    pub fn lhs(&self) -> Symbol {
        self.lhs
    }

    pub fn rhs0(&self) -> Symbol {
        self.rhs0
    }

    pub fn rhs1(&self) -> Option<Symbol> {
        self.rhs1
    }
}

struct BinaryRuleMut<'a> {
    lhs: &'a mut Option<Symbol>,
    rhs0: &'a mut Option<Symbol>,
    rhs1: &'a mut Option<Symbol>,
}

impl<'a> BinaryRuleMut<'a> {
    fn lhs(&self) -> Symbol {
        self.lhs.unwrap()
    }

    fn rhs0(&self) -> Symbol {
        self.rhs0.unwrap()
    }

    // fn rhs1(&self) -> Option<Symbol> {
    //     *self.rhs1
    // }
}

#[derive(Debug)]
pub(in super) struct RuleSlices<'a> {
    pub lhs: &'a [Option<Symbol>],
    pub rhs0: &'a [Option<Symbol>],
    pub rhs1: &'a [Option<Symbol>],
}

impl<'a> RuleSlices<'a> {
    pub fn rules(&mut self) -> Rules<'a> {
        Rules {
            lhs: self.lhs.iter(),
            rhs0: self.rhs0.iter(),
            rhs1: self.rhs1.iter(),
        }
    }
}

pub(in super) struct RuleSlicesMut<'a> {
    pub lhs: &'a mut [Option<Symbol>],
    pub rhs0: &'a mut [Option<Symbol>],
    pub rhs1: &'a mut [Option<Symbol>],
}

impl<'a> RuleSlicesMut<'a> {
    fn rules_mut(&'a mut self) -> RulesMut<'a> {
        RulesMut {
            lhs: self.lhs.iter_mut(),
            rhs0: self.rhs0.iter_mut(),
            rhs1: self.rhs1.iter_mut(),
        }
    }
}

pub(in super) struct Rules<'a> {
    lhs: slice::Iter<'a, Option<Symbol>>,
    rhs0: slice::Iter<'a, Option<Symbol>>,
    rhs1: slice::Iter<'a, Option<Symbol>>,
}

impl<'a> Iterator for Rules<'a> {
    type Item = BinaryRule;

    fn next(&mut self) -> Option<BinaryRule> {
        match (self.lhs.next(), self.rhs0.next(), self.rhs1.next()) {
            (Some(&Some(lhs)), Some(&Some(rhs0)), Some(&None)) => {
                Some(BinaryRule::new(lhs, rhs0, None))
            }
            (Some(&Some(lhs)), Some(&Some(rhs0)), Some(&Some(rhs1))) => {
                Some(BinaryRule::new(lhs, rhs0, Some(rhs1)))
            }
            _ => None
        }
    }
}

struct RulesMut<'a> {
    lhs: slice::IterMut<'a, Option<Symbol>>,
    rhs0: slice::IterMut<'a, Option<Symbol>>,
    rhs1: slice::IterMut<'a, Option<Symbol>>,
}

impl<'a> Iterator for RulesMut<'a> {
    type Item = BinaryRuleMut<'a>;

    fn next(&mut self) -> Option<BinaryRuleMut<'a>> {
        match (self.lhs.next(), self.rhs0.next(), self.rhs1.next()) {
            (Some(lhs), Some(rhs0), Some(rhs1)) => {
                Some(BinaryRuleMut {
                    lhs,
                    rhs0,
                    rhs1,
                })
            }
            _ => None
        }
    }
}
