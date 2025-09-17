use std::fmt::{Debug, Display};

use cfg::{Cfg, Symbol};
use cfg_load::advanced::AdvancedGrammar;
use gearley_default_grammar::DefaultGrammar;

#[cfg(feature = "simple-bocage")]
use gearley_forest::Forest;
use gearley_forest::NullForest;
use gearley_grammar::Grammar;
use gearley_recognizer::{lookahead::Lookahead, perf_hint::PerfHint, Recognizer};

pub trait RecognizerParseExt {
    fn parse(&mut self, tokens: &[Symbol]) -> Result<bool, ParseError>;
}

#[cfg(feature = "log")]
use log::trace;
#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($tts:tt)*) => {};
}

#[derive(Debug)]
pub enum ParseError {
    Parse {
        msg: &'static str,
        tokens: Vec<Symbol>,
        i: usize,
    },
    Tokenize {
        msg: &'static str,
        word: String,
    },
    Finish {
        msg: &'static str,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Parse { msg, tokens, i } => {
                f.write_str(msg)?;
                f.write_str(": ")?;
                write!(f, "{:?}@{}", tokens, i)?;
                Ok(())
            }
            ParseError::Tokenize { msg, word } => {
                f.write_str(msg)?;
                f.write_str(": ")?;
                f.write_str(word)?;
                Ok(())
            }
            ParseError::Finish { msg } => f.write_str(msg),
        }
    }
}

impl<G, F, P> RecognizerParseExt for Recognizer<G, F, P>
where
    Self: Debug,
    G: Grammar,
    F: Forest,
    P: PerfHint,
{
    #[inline]
    fn parse(&mut self, tokens: &[Symbol]) -> Result<bool, ParseError> {
        self.begin_earleme();
        self.scan(self.grammar().sof(), Default::default());
        if !self.end_earleme() {
            return Err(ParseError::Parse {
                msg: "failed to read SOF",
                tokens: vec![self.grammar().sof()],
                i: 0,
            });
        }
        let mut iter = tokens.iter().enumerate().peekable();
        while let Some((i, &token)) = iter.next() {
            self.begin_earleme();
            if let Some((_i, t)) = iter.peek() {
                trace!("utils.lookahead_set_hint: {:?}", **t);
                let s = self.grammar().to_internal(**t).unwrap();
                self.lookahead().set_hint(s);
            } else {
                trace!("utils.lookahead_clear_hint: None null");
                self.lookahead().clear_hint();
            }
            self.scan(self.grammar().to_internal(token).unwrap(), i as u32);
            if !self.end_earleme() {
                return Err(ParseError::Parse {
                    msg: "failed to parse",
                    tokens: vec![token],
                    i,
                });
            }
        }
        // self.begin_earleme();
        // self.scan(self.grammar().eof(), 0);
        // if !self.end_earleme() {
        //     return Err(ParseError::Parse { msg: "failed to read EOF", token: self.grammar().eof(), i: 0 });
        // }

        trace!("utils.finished: {:?}", &*self);

        Ok(self.is_finished())
    }
}

pub fn parse_terminal_list<'a>(
    cfg: Cfg,
    grammar: DefaultGrammar,
    terminal_list: impl Iterator<Item = &'a str>,
) -> Result<bool, ParseError> {
    let mut recognizer = Recognizer::with_forest(&grammar, simple_bocage::Bocage::new(&grammar));
    let name_map = cfg.sym_source().name_map();
    let mut tokens = vec![];
    for word in terminal_list {
        if let Some(token) = name_map.get(word) {
            tokens.push(*token);
        } else {
            return Err(ParseError::Tokenize {
                msg: "failed to tokenize",
                word: word.to_string(),
            });
        }
    }
    let result = recognizer.parse(&tokens);
    if let Some(node) = recognizer.finished_node() {
        trace!("utils.finished_node: NodeHandle {{ handle: {:?} }}", node);
    } else {
        return Err(ParseError::Finish {
            msg: "failed to get finished node",
        });
    }
    trace!("utils.bocage: {:?}", recognizer.into_forest());
    result
}

pub fn parse_tokenizing(
    mut loaded: AdvancedGrammar,
    grammar: DefaultGrammar,
    input: &str,
) -> Result<bool, ParseError> {
    loaded.lexer_map.compute();
    let mut recognizer = Recognizer::with_forest(&grammar, simple_bocage::Bocage::new(&grammar));

    recognizer.begin_earleme();
    recognizer.scan(recognizer.grammar().sof(), 0);
    if !recognizer.end_earleme() {
        return Err(ParseError::Parse {
            msg: "failed to read SOF",
            tokens: vec![recognizer.grammar().sof()],
            i: 0,
        });
    }
    for (i, ch) in input.chars().enumerate() {
        recognizer.begin_earleme();
        for &terminal in loaded.lexer_map.get(ch) {
            recognizer.scan(
                recognizer.grammar().to_internal(terminal).unwrap(),
                ch as u32,
            );
        }
        if !recognizer.end_earleme() {
            return Err(ParseError::Parse {
                msg: "failed to recognize",
                tokens: loaded.lexer_map.get(ch).to_vec(),
                i,
            });
        }
    }
    trace!("utils.finished: {:?}", recognizer);
    let result = recognizer.is_finished();

    if let Some(node) = recognizer.finished_node() {
        trace!("utils.finished_node: NodeHandle {{ handle: {:?} }}", node);
    } else {
        return Err(ParseError::Finish {
            msg: "failed to get finished node",
        });
    }
    trace!("utils.bocage: {:?}", recognizer.into_forest());

    Ok(result)
}
