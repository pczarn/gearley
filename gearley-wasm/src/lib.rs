#[macro_use]
extern crate cfg_if;

mod error_logger;

use wasm_bindgen::prelude::*;

use talc::*;
use spin;

use gearley::{DefaultGrammar, Recognizer, Bocage, RecognizerParseExt, utils};

use std::cell::RefCell;
use std::sync::LazyLock;
use std::sync::mpsc;
use std::panic;
use std::fmt::Write;
use log::trace;

static mut ARENA: [u8; 10000_000] = [0; 10000_000];

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

use cfg_load::{CfgLoadExt, LoadError};
use cfg_grammar::Cfg;

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_BUFFER: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

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
    let result = LOG_BUFFER.lock().unwrap().clone();
    LOG_BUFFER.lock().unwrap().clear();
    result
}

fn load(grammar: &str) -> Result<(Cfg, DefaultGrammar), LoadError> {
    let _ = init_logger();
    let cfg = Cfg::load(&grammar[..])?;
    let grammar = DefaultGrammar::from_grammar(cfg.clone());
    Ok((cfg, grammar))
}

#[wasm_bindgen(start)]
fn start() {
    error_logger::set_panic_hook();
}

#[wasm_bindgen]
pub fn parse(input: &str, grammar: &str) -> String {
    error_logger::set_panic_hook();
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
