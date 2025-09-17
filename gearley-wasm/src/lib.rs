#[macro_use]
extern crate cfg_if;

mod error_logger;

use wasm_bindgen::prelude::*;

use talc::*;
use spin;

use gearley::{DefaultGrammar, Recognizer, Bocage, RecognizerParseExt, utils};
use cfg_grammar::SymbolBitSet;
use cfg_load::advanced::{AdvancedGrammar, LexerMap};

use std::cell::RefCell;
use std::sync::LazyLock;
use std::sync::mpsc;
use std::panic;
use std::fmt::Write;
use log::trace;

static mut ARENA: [u8; 200_000_000] = [0; 200_000_000];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    // if we're in a hosted environment, the Rust runtime may allocate before
    // main() is called, so we need to initialize the arena automatically
    ClaimOnOom::new(Span::from_const_array(core::ptr::addr_of!(ARENA)))
}).lock();

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

use wasm_bindgen::prelude::*;

use cfg_load::{CfgLoadExt, CfgLoadAdvancedExt, LoadError};
use cfg_grammar::Cfg;

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_BUFFER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::with_capacity(100_000_000)));

struct StringLogger;

impl log::Log for StringLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace // capture all levels
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut buffer = LOG_BUFFER.lock().unwrap();
            let msg = format!("[{}] - {}\n", record.level(), record.args());
            buffer.push_str(&msg);
        }
    }

    fn flush(&self) {}
}

static LOGGER: StringLogger = StringLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

pub fn get_logs() -> String {
    std::mem::replace(&mut LOG_BUFFER.lock().unwrap(), String::new())
}

fn load(grammar: &str) -> Result<(Cfg, DefaultGrammar), LoadError> {
    let _ = init_logger();
    let cfg = Cfg::load(&grammar[..])?;
    let grammar = DefaultGrammar::from_grammar(cfg.clone());
    Ok((cfg, grammar))
}

fn loadAdvanced(grammar: &str) -> Result<(AdvancedGrammar, DefaultGrammar), LoadError> {
    let _ = init_logger();
    let loaded = Cfg::load_advanced(&grammar[..])?;
    let grammar = DefaultGrammar::from_grammar(loaded.cfg.clone());
    Ok((loaded, grammar))
}

#[wasm_bindgen(start)]
fn start() {
    error_logger::set_panic_hook();
}

#[wasm_bindgen]
pub fn getGrammars() -> Vec<String> {
    gearley_example_grammars::BNFS.into_iter().flat_map(|bnf| bnf.into_iter().map(|s| s.to_string())).collect::<Vec<_>>()
}

#[wasm_bindgen]
pub fn getExamples(id: String) -> Vec<String> {
    gearley_example_grammars::get_examples(&id).expect("wrong example id").into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn c_lexer(input: &str) -> Vec<&'static str> {
    use c_lexer_logos::token::Token::*;
    use c_lexer_logos::Lexer;
    Lexer::lex(&input[..])
    .expect("could not lex c input")
    .into_iter()
    .filter_map(|token| {
        match token {
            LBrace => Some("lbrace"),
            RBrace => Some("rbrace"),
            LParen => Some("lparen"),
            RParen => Some("rparen"),
            LBracket => Some("lbracket"),
            RBracket => Some("rbracket"),
            Semicolon => Some("semicolon"),
            Assign => Some("equal"),
            Lt => Some("langle"),
            Gt => Some("rangle"),
            Minus => Some("minus"),
            Tilde => Some("tilde"),
            Exclamation => Some("exclamation"),
            Plus => Some("plus"),
            Multi => Some("star"),
            Slash => Some("slash"),
            Colon => Some("colon"),
            QuestionMark => Some("question"),
            Comma => Some("comma"),
            Dot => Some("dot"),
            SingleAnd => Some("ampersand"),
            InclusiveOr => Some("pipe"),
            ExclusiveOr => Some("xor"),
            Mod => Some("percent"),
            Identifier(i_str) => Some("identifier"),
            NumericLiteral(num) => Some("constant"),
            StringLiteral(s) => Some("string_literal"),
            FuncName => None,
            SIZEOF => Some("sizeof_"),
            PtrOp => Some("ptr_op"),
            IncOp => Some("inc_op"),
            DecOp => Some("dec_op"),
            LeftOp => Some("left_op"),
            RightOp => Some("right_op"),
            LeOp => Some("le_op"),
            GeOp => Some("ge_op"),
            EqOp => Some("eq_op"),
            NeOp => Some("ne_op"),
            AndOp => Some("and_op"),
            OrOp => Some("or_op"),
            MulAssign => Some("mul_assign"),
            DivAssign => Some("div_assign"),
            ModAssign => Some("mod_assign"),
            AddAssign => Some("add_assign"),
            SubAssign => Some("sub_assign"),
            LeftAssign => Some("left_assign"),
            RightAssign => Some("right_assign"),
            AndAssign => Some("and_assign"),
            XorAssign => Some("xor_assign"),
            OrAssign => Some("or_assign"),
            // TODO: this should be done when we found this is a typedef name,
            //       typedef LL int, then LL is typedef_name
            TypedefName => Some("identifier"),
            ELLIPSIS => Some("elipsis"),       // ...
            EnumerationConstant(..) => None, // TODO: add check
            LineTerminator => None,
            EOF => None,

            TYPEDEF => Some("typedef"),
            EXTERN => Some("extern_"),
            STATIC => Some("static_"),
            // AUTO => Some("auto_"),
            REGISTER => Some("register"),
            INLINE => Some("inline"),
            CONST => Some("const_"),
            RESTRICT => Some("restrict"),
            VOLATILE => Some("volatile"),
            BOOL => Some("bool_"),
            CHAR => Some("char_"),
            SHORT => Some("short"),
            INT => Some("int"),
            LONG => Some("long"),
            SIGNED => Some("signed"),
            UNSIGNED => Some("unsigned"),
            FLOAT => Some("float"),
            DOUBLE => Some("double"),
            VOID => Some("void"),
            COMPLEX => Some("complex"),
            IMAGINARY => Some("imaginary"),
            STRUCT => Some("struct_"),
            UNION => Some("union"),
            ENUM => Some("enum_"),
            CASE => Some("case"),
            DEFAULT => Some("default"),
            IF => Some("if_"),
            ELSE => Some("else_"),
            SWITCH => Some("switch"),
            WHILE => Some("while_"),
            DO => Some("do_"),
            FOR => Some("for_"),
            GOTO => Some("goto"),
            CONTINUE => Some("continue_"),
            BREAK => Some("break_"),
            RETURN => Some("return_"),
            // ALIGNAS => Some("alignas"),
            // ALIGNOF => Some("alignof"),
            // ATOMIC => Some("atomic"),
            // GENERIC => Some("generic"),
            // NORETURN,
            // StaticAssert,
            // ThreadLocal,
            _ => None,
        }
    })
    .collect()
}

#[wasm_bindgen]
pub fn parse(input: &str, grammar: &str, mode: &str) -> String {
    error_logger::set_panic_hook();
    trace!("begin");
    match mode {
        "c-lexer" => {
            trace!("c-lexer load");
            match load(grammar) {
                Ok((cfg, default_grammar)) => {
                    trace!("loaded");
                    match utils::parse_terminal_list(cfg, default_grammar, c_lexer(input).into_iter()) {
                        Ok(true) => {
                            get_logs()
                        }
                        Ok(false) => {
                            "failed to finish the parse".to_string()
                        }
                        Err(parse_error) => {
                            let mut result = "Failed to parse:\n".to_string();
                            result.push_str(&parse_error.to_string());
                            result.push_str(&get_logs()[..]);
                            result
                        }
                    }
                },
                Err(mut load_error) => {
                    trace!("failed to load");
                    let mut result = "Failed to load:\n".to_string();
                    result.push_str(&load_error.to_string());
                    result.push_str(&get_logs()[..]);
                    result
                }
            }
        }
        "basic" => {
            match load(grammar) {
                Ok((cfg, default_grammar)) => {
                    match utils::parse_terminal_list(cfg, default_grammar, input.split(" ")) {
                        Ok(true) => {
                            get_logs()
                        }
                        Ok(false) => {
                            "failed to finish the parse".to_string()
                        }
                        Err(parse_error) => {
                            let mut result = "Failed to parse:\n".to_string();
                            result.push_str(&parse_error.to_string());
                            result.push_str(&get_logs()[..]);
                            result
                        }
                    }
                },
                Err(mut load_error) => {
                    let mut result = "Failed to load:\n".to_string();
                    result.push_str(&load_error.to_string());
                    result.push_str(&get_logs()[..]);
                    result
                }
            }
        }
        "advanced" => {
            match loadAdvanced(grammar) {
                Ok((loaded, grammar)) => {
                    match utils::parse_tokenizing(loaded, grammar, input) {
                        Ok(true) => {
                            get_logs()
                        }
                        Ok(false) => {
                            "failed to finish the parse".to_string()
                        }
                        Err(parse_error) => {
                            let mut result = "Failed to parse:\n".to_string();
                            result.push_str(&parse_error.to_string());
                            result.push_str(&get_logs()[..]);
                            result
                        }
                    }
                }
                Err(mut load_error) => {
                    let mut result = "Failed to load:\n".to_string();
                    result.push_str(&load_error.to_string());
                    result.push_str(&get_logs()[..]);
                    result
                }
            }
        }
        _ => return "Unknown mode".to_string()
    }
}
