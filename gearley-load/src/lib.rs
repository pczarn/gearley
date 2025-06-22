use std::{collections::HashMap, fmt, rc::Rc};

use cfg::{Cfg, CfgRule, Symbol, SymbolSource};
use cfg_load::{CfgLoadExt, StringInterner};
use cfg_sequence::CfgSequenceExt;
use gearley_default_grammar::DefaultGrammar;
use gearley_forest::Evaluate;
use gearley_recognizer::Recognizer;
use simple_bocage::{node::Node::Sum, Bocage};

macro_rules! grammar {
    (
        T = [$($terminal_name:ident),+],
        N = [$($nonterminal_name:ident -> $variant_ty:ty),+],
        $(
            $bnf:literal
            fn $fn_name:ident($()*) -> $fn_return:ty $fn_block:block
        )+
    ) => {
        $(
            
        )+
        struct Evaluator {
            name_map: HashMap<Rc<str>, Symbol>,
            cfg: Cfg,
            $($terminal_name: Symbol,)+
        }

        impl Evaluator {
            fn new(name_map: HashMap<Rc<str>, Symbol>) -> Self {
                Self {
                    cfg: Cfg::new(),
                    $($terminal_name: name_map[stringify!($terminal_name)],)+
                    name_map,
                }
            }
        }

        #[derive(Clone, Debug)]
        enum Value {
            $($nonterminal_name($variant_ty)),+
            None,
        }

        impl Default for Value {
            fn default() -> Self {
                Value::None
            }
        }

        impl Evaluate for Evaluator {
            type Elem = Value;

            fn leaf(&self, terminal: cfg::Symbol, values: u32) -> Self::Elem {
                if terminal == self.alpha {
                    Value::Alpha(char::from_u32(values).unwrap())
                } else if terminal == self.digit {
                    Value::Digit(values as u8 as char)
                } else {
                    Value::None
                }
            }

            fn nulling<'r>(&self, _symbol: cfg::Symbol, results: &'r mut Vec<Self::Elem>) {
                results.push(Value::None);
            }

            fn product<'e>(&self, action: u32, mut args: impl Iterator<Item = &'e Self::Elem>) -> Self::Elem where Self::Elem: 'e {
                rule! {
                    "start ::= rule+;"
                    fn start(rules: Vec<Rule>) -> Vec<Rule> {
                        rules
                    }
                }
                rule! {
                    "rule ::= ident bnf_op rhs semicolon;"
                    fn rule(lhs: String, _: (), rhs: Vec<Vec<Fragment>>) -> Vec<Rule> {
                        rhs.into_iter().map(|rhs| Rule { lhs: lhs.clone(), rhs }).collect()
                    }
                }
                match (
                    action,
                    args.next().cloned().unwrap_or(Value::None),
                    args.next().cloned().unwrap_or(Value::None),
                    args.next().cloned().unwrap_or(Value::None),
                    args.next().cloned().unwrap_or(Value::None),
                ) {
                    // start ::= rule+;
                    (2, Value::Rules(mut rules), Value::Rules(rule), ..) => {
                        rules.extend(rule);
                        Value::Rules(rules)
                    }
                    // rule ::= lhs bnf_op rhs semicolon;
                    (3, Value::Ident(lhs), _, Value::Rhs(rhs), ..) => {
                        let rules = rhs.into_iter().map(|rhs| Rule { lhs: lhs.clone(), rhs }).collect();
                        Value::Rules(rules)
                    }
                    // rhs ::= rhs pipe alt;
                    (4, Value::Rhs(mut rhs), _, Value::Alt(alt), ..) => {
                        rhs.push(alt);
                        Value::Rhs(rhs)
                    }
                    // rhs ::= alt;
                    (5, Value::Alt(alt), ..) => {
                        Value::Rhs(vec![alt])
                    }
                    // alt ::= fragment*;
                    (7, Value::Alt(mut alt), Value::Fragment(fragment), ..) => {
                        alt.push(fragment);
                        Value::Alt(alt)
                    }
                    // alt ::= fragment* action;
                    (8, Value::Fragment(fragment), _, _) => {
                        Value::Alt(vec![fragment])
                    }
                    // fragment ::= ident op_plus;
                    (9, Value::Ident(ident), _, _) => {
                        Value::Fragment(Fragment { ident, rep: Rep::OneOrMore })
                    }
                    // fragment ::= ident op_mul;
                    (10, Value::Ident(ident), _, _) => {
                        Value::Fragment(Fragment { ident, rep: Rep::ZeroOrMore })
                    }
                    // fragment ::= ident;
                    (11, Value::Ident(ident), _, _) => {
                        Value::Fragment(Fragment { ident, rep: Rep::None })
                    }
                    // bnf_op ::= colon colon eq_op;
                    (12, _, _, _) => {
                        Value::None
                    }
                    // ident ::= alpha ident_tail;
                    (13, Value::Alpha(alpha), Value::Ident(ident), _) => {
                        let mut result = String::new();
                        result.push(alpha);
                        result.push_str(&ident[..]);
                        Value::Ident(result)
                    }
                    // ident ::= alpha;
                    (14, Value::Alpha(ch), _, _) => {
                        Value::Ident(ch.into())
                    }
                    // ident_tail ::= ident_tail alnum;
                    (15, Value::Ident(mut ident), Value::Alnum(ch), _) => {
                        ident.push(ch);
                        Value::Ident(ident)
                    }
                    // ident_tail ::= alnum;
                    (16, Value::Alnum(ch), _, _) => {
                        Value::Ident(ch.into())
                    }
                    // alnum ::= alpha;
                    (17, Value::Alpha(ch), _, _) => {
                        Value::Alnum(ch)
                    }
                    // alnum ::= digit;
                    (18, Value::Digit(digit), _, _) => {
                        Value::Digit(digit)
                    }
                    // esc ::= any_dq
                    (19, )
                    args => panic!("unknown rule id {:?} or args {:?}", action, args),
                }
            }
        }

        pub fn load_with_scanner(bnf: &str) -> Result<Cfg, LoadError> {
            let mut bnf_with_chars_and_actions_grammar = Cfg::load(r##"
                start ::= rule+;
                rule ::= ident bnf_op rhs semicolon;
                action ::= eq_op gt_op ident;
                rhs ::= rhs pipe alt;
                rhs ::= alt;
                alt ::= fragment*;
                alt ::= fragment* action;
                fragment ::= ident op_plus;
                fragment ::= ident op_mul;
                fragment ::= ident;
                fragment ::= char;
                fragment ::= string;
                string ::= double_quote esc* double_quote;
                esc ::= any_dq;
                esc ::= backslash double_quote;
                char ::= quote any_q quote;
                char ::= quote backslash quote quote;
                bnf_op ::= colon colon eq_op;
                ident ::= alpha ident_tail;
                ident ::= alpha;
                ident_tail ::= ident_tail alnum;
                ident_tail ::= alnum;
                alnum ::= alpha;
                alnum ::= digit;
            "##).unwrap();
            let name_map = bnf_with_chars_and_actions_grammar.sym_source().name_map();
            let grammar = DefaultGrammar::from_grammar(bnf_with_chars_and_actions_grammar);
            let mut recognizer = Recognizer::with_forest(&grammar, Bocage::new(&grammar));
            let mut line_no = 1;
            let mut col_no = 1;
            for ch in bnf.chars() {
                let terminal = match ch {
                    ':' => name_map["colon"],
                    ';' => name_map["semicolon"],
                    '=' => name_map["eq_op"],
                    '0'..='9' => name_map["digit"],
                    '|' => name_map["pipe"],
                    '*' => name_map["op_mul"],
                    '+' => name_map["op_plus"],
                    'a'..='z' | 'A'..='Z' => name_map["alpha"],
                    ' ' => continue,
                    '\n' => {
                        line_no += 1;
                        col_no = 1;
                        continue;
                    }
                    other => return Err(LoadError::Parse { reason: format!("invalid character {}", other), line: line_no, col: col_no }),
                };
                recognizer.scan(terminal, ch as u32);
                let success = recognizer.end_earleme();
                // if !success {
                //     self.recognizer.log_earley_set_diff();
                // }
                if !success {
                    return Err(LoadError::Parse { reason: "parse failed".to_string(), line: line_no, col: col_no });
                }
                col_no += 1;
                // assert!(success, "parse failed at character {}", i);
            }
            let finished_node = if let Some(node) = recognizer.finished_node() {
                node
            } else {
                return Err(LoadError::Parse { reason: "parse failed".to_string(), line: line_no, col: col_no });
            };
            let result = recognizer
                .into_forest()
                .evaluate(Evaluator::new(name_map), finished_node);
            assert_eq!(result.len(), 1);
            if let Value::Rules(rules) = &result[0] {
                let mut cfg = Cfg::new();
                let intern = StringInterner::new();
                let mut sym_map = HashMap::new();
                let mut intern_empty: bool = true;
                for rule in rules.clone() {
                    let lhs = intern.get_or_intern(&rule.lhs[..]);
                    let lhs_sym = *sym_map.entry(lhs).or_insert_with(|| cfg.sym_source_mut().next_sym(Some(rule.lhs[..].into())));
                    if intern_empty {
                        cfg.set_roots([lhs_sym]);
                        intern_empty = false;
                    }
                    let rhs_syms: Vec<_> = rule.rhs.into_iter().map(|fragment| {
                        let id = intern.get_or_intern(&fragment.ident[..]);
                        let rhs_sym = *sym_map.entry(id).or_insert_with(|| cfg.sym_source_mut().next_sym(Some(fragment.ident[..].into())));
                        match fragment.rep {
                            Rep::None => rhs_sym,
                            Rep::ZeroOrMore => {
                                let [new_sym] = cfg.sym();
                                cfg.sequence(new_sym).inclusive(0, None).rhs(rhs_sym);
                                new_sym
                            }
                            Rep::OneOrMore => {
                                let [new_sym] = cfg.sym();
                                cfg.sequence(new_sym).range(1..).rhs(rhs_sym);
                                new_sym
                            }
                        }
                    }).collect();
                    cfg.rule(lhs_sym).rhs(rhs_syms);
                }
                Ok(cfg)
            } else {
                return Err(LoadError::Eval { reason: format!("evaluation failed: Expected Value::Rules, got {:?}", result) });
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum LoadError {
    Parse {
        reason: String,
        line: u32,
        col: u32,
    },
    Eval {
        reason: String,
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::Parse { reason, line, col } => {
                write!(f, "Parse error at line {} column {}: reason: {}", line, col, reason)
            }
            LoadError::Eval { reason } => {
                write!(f, "Eval error. Reason: {}", reason)
            }
        }
    }
}

struct Evaluator {
    name_map: HashMap<Rc<str>, Symbol>,
    cfg: Cfg,
    alpha: Symbol,
    digit: Symbol,
    colon: Symbol,
    semicolon: Symbol,
    eq_op: Symbol,
    pipe: Symbol,
    gt_op: Symbol,
    op_plus: Symbol,
    op_mul: Symbol,
}

impl Evaluator {
    fn new(name_map: HashMap<Rc<str>, Symbol>) -> Self {
        Self {
            cfg: Cfg::new(),
            alpha: name_map["alpha"],
            digit: name_map["digit"],
            colon: name_map["colon"],
            semicolon: name_map["semicolon"],
            eq_op: name_map["eq_op"],
            pipe: name_map["pipe"],
            gt_op: name_map["gt_op"],
            op_plus: name_map["op_plus"],
            op_mul: name_map["op_mul"],
            name_map,
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Rules(Vec<Rule>),
    Digit(char),
    Alpha(char),
    Ident(String),
    Rhs(Vec<Vec<Fragment>>),
    Alt(Vec<Fragment>),
    Fragment(Fragment),
    Alnum(char),
    None,
}

#[derive(Clone, Debug)]
struct Rule {
    lhs: String,
    rhs: Vec<Fragment>,
    action: Option<String>,
}

#[derive(Clone, Debug)]
enum Rep {
    ZeroOrMore,
    OneOrMore,
    None,
}

#[derive(Clone, Debug)]
struct Fragment {
    ident: String,
    rep: Rep,
}

impl Default for Value {
    fn default() -> Self {
        Value::None
    }
}

impl Evaluate for Evaluator {
    type Elem = Value;

    fn leaf(&self, terminal: cfg::Symbol, values: u32) -> Self::Elem {
        if terminal == self.alpha {
            Value::Alpha(char::from_u32(values).unwrap())
        } else if terminal == self.digit {
            Value::Digit(values as u8 as char)
        } else {
            Value::None
        }
    }

    fn nulling<'r>(&self, _symbol: cfg::Symbol, results: &'r mut Vec<Self::Elem>) {
        results.push(Value::None);
    }

    fn product<'e>(&self, action: u32, mut args: impl Iterator<Item = &'e Self::Elem>) -> Self::Elem where Self::Elem: 'e {
        rule! {
            "start ::= rule+;"
            fn start(rules: Vec<Rule>) -> Vec<Rule> {
                rules
            }
        }
        rule! {
            "rule ::= ident bnf_op rhs semicolon;"
            fn rule(lhs: String, _: (), rhs: Vec<Vec<Fragment>>) -> Vec<Rule> {
                rhs.into_iter().map(|rhs| Rule { lhs: lhs.clone(), rhs }).collect()
            }
        }
        match (
            action,
            args.next().cloned().unwrap_or(Value::None),
            args.next().cloned().unwrap_or(Value::None),
            args.next().cloned().unwrap_or(Value::None),
            args.next().cloned().unwrap_or(Value::None),
        ) {
            // start ::= rule+;
            (2, Value::Rules(mut rules), Value::Rules(rule), ..) => {
                rules.extend(rule);
                Value::Rules(rules)
            }
            // rule ::= lhs bnf_op rhs semicolon;
            (3, Value::Ident(lhs), _, Value::Rhs(rhs), ..) => {
                let rules = rhs.into_iter().map(|rhs| Rule { lhs: lhs.clone(), rhs }).collect();
                Value::Rules(rules)
            }
            // rhs ::= rhs pipe alt;
            (4, Value::Rhs(mut rhs), _, Value::Alt(alt), ..) => {
                rhs.push(alt);
                Value::Rhs(rhs)
            }
            // rhs ::= alt;
            (5, Value::Alt(alt), ..) => {
                Value::Rhs(vec![alt])
            }
            // alt ::= fragment*;
            (7, Value::Alt(mut alt), Value::Fragment(fragment), ..) => {
                alt.push(fragment);
                Value::Alt(alt)
            }
            // alt ::= fragment* action;
            (8, Value::Fragment(fragment), _, _) => {
                Value::Alt(vec![fragment])
            }
            // fragment ::= ident op_plus;
            (9, Value::Ident(ident), _, _) => {
                Value::Fragment(Fragment { ident, rep: Rep::OneOrMore })
            }
            // fragment ::= ident op_mul;
            (10, Value::Ident(ident), _, _) => {
                Value::Fragment(Fragment { ident, rep: Rep::ZeroOrMore })
            }
            // fragment ::= ident;
            (11, Value::Ident(ident), _, _) => {
                Value::Fragment(Fragment { ident, rep: Rep::None })
            }
            // bnf_op ::= colon colon eq_op;
            (12, _, _, _) => {
                Value::None
            }
            // ident ::= alpha ident_tail;
            (13, Value::Alpha(alpha), Value::Ident(ident), _) => {
                let mut result = String::new();
                result.push(alpha);
                result.push_str(&ident[..]);
                Value::Ident(result)
            }
            // ident ::= alpha;
            (14, Value::Alpha(ch), _, _) => {
                Value::Ident(ch.into())
            }
            // ident_tail ::= ident_tail alnum;
            (15, Value::Ident(mut ident), Value::Alnum(ch), _) => {
                ident.push(ch);
                Value::Ident(ident)
            }
            // ident_tail ::= alnum;
            (16, Value::Alnum(ch), _, _) => {
                Value::Ident(ch.into())
            }
            // alnum ::= alpha;
            (17, Value::Alpha(ch), _, _) => {
                Value::Alnum(ch)
            }
            // alnum ::= digit;
            (18, Value::Digit(digit), _, _) => {
                Value::Digit(digit)
            }
            // esc ::= any_dq
            (19, )
            args => panic!("unknown rule id {:?} or args {:?}", action, args),
        }
    }
}

pub fn load_with_scanner(bnf: &str) -> Result<Cfg, LoadError> {
    let mut bnf_with_chars_and_actions_grammar = Cfg::load(r##"
        start ::= rule+;
        rule ::= ident bnf_op rhs semicolon;
        action ::= eq_op gt_op ident;
        rhs ::= rhs pipe alt;
        rhs ::= alt;
        alt ::= fragment*;
        alt ::= fragment* action;
        fragment ::= ident op_plus;
        fragment ::= ident op_mul;
        fragment ::= ident;
        fragment ::= char;
        fragment ::= string;
        string ::= double_quote esc* double_quote;
        esc ::= any_dq;
        esc ::= backslash double_quote;
        char ::= quote any_q quote;
        char ::= quote backslash quote quote;
        bnf_op ::= colon colon eq_op;
        ident ::= alpha ident_tail;
        ident ::= alpha;
        ident_tail ::= ident_tail alnum;
        ident_tail ::= alnum;
        alnum ::= alpha;
        alnum ::= digit;
    "##).unwrap();
    let name_map = bnf_with_chars_and_actions_grammar.sym_source().names().into_iter().zip(SymbolSource::generate_fresh()).filter_map(|(opt, i)| opt.map(|rc| (rc, i))).collect::<HashMap<_, _>>();
    let grammar = DefaultGrammar::from_grammar(bnf_with_chars_and_actions_grammar);
    let mut recognizer = Recognizer::with_forest(&grammar, Bocage::new(&grammar));
    let mut line_no = 1;
    let mut col_no = 1;
    for ch in bnf.chars() {
        let terminal = match ch {
            ':' => name_map["colon"],
            ';' => name_map["semicolon"],
            '=' => name_map["eq_op"],
            '0'..='9' => name_map["digit"],
            '|' => name_map["pipe"],
            '*' => name_map["op_mul"],
            '+' => name_map["op_plus"],
            'a'..='z' | 'A'..='Z' => name_map["alpha"],
            ' ' => continue,
            '\n' => {
                line_no += 1;
                col_no = 1;
                continue;
            }
            other => return Err(LoadError::Parse { reason: format!("invalid character {}", other), line: line_no, col: col_no }),
        };
        recognizer.scan(terminal, ch as u32);
        let success = recognizer.end_earleme();
        // if !success {
        //     self.recognizer.log_earley_set_diff();
        // }
        if !success {
            return Err(LoadError::Parse { reason: "parse failed".to_string(), line: line_no, col: col_no });
        }
        col_no += 1;
        // assert!(success, "parse failed at character {}", i);
    }
    let finished_node = if let Some(node) = recognizer.finished_node() {
        node
    } else {
        return Err(LoadError::Parse { reason: "parse failed".to_string(), line: line_no, col: col_no });
    };
    let result = recognizer
        .into_forest()
        .evaluate(Evaluator::new(name_map), finished_node);
    assert_eq!(result.len(), 1);
    if let Value::Rules(rules) = &result[0] {
        let mut cfg = Cfg::new();
        let intern = StringInterner::new();
        let mut sym_map = HashMap::new();
        let mut intern_empty: bool = true;
        for rule in rules.clone() {
            let lhs = intern.get_or_intern(&rule.lhs[..]);
            let lhs_sym = *sym_map.entry(lhs).or_insert_with(|| cfg.sym_source_mut().next_sym(Some(rule.lhs[..].into())));
            if intern_empty {
                cfg.set_roots([lhs_sym]);
                intern_empty = false;
            }
            let rhs_syms: Vec<_> = rule.rhs.into_iter().map(|fragment| {
                let id = intern.get_or_intern(&fragment.ident[..]);
                let rhs_sym = *sym_map.entry(id).or_insert_with(|| cfg.sym_source_mut().next_sym(Some(fragment.ident[..].into())));
                match fragment.rep {
                    Rep::None => rhs_sym,
                    Rep::ZeroOrMore => {
                        let [new_sym] = cfg.sym();
                        cfg.sequence(new_sym).inclusive(0, None).rhs(rhs_sym);
                        new_sym
                    }
                    Rep::OneOrMore => {
                        let [new_sym] = cfg.sym();
                        cfg.sequence(new_sym).range(1..).rhs(rhs_sym);
                        new_sym
                    }
                }
            }).collect();
            cfg.rule(lhs_sym).rhs(rhs_syms);
        }
        Ok(cfg)
    } else {
        return Err(LoadError::Eval { reason: format!("evaluation failed: Expected Value::Rules, got {:?}", result) });
    }
}
