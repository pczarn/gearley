#![deny(unsafe_code)]

#[cfg(feature = "gearley-default-grammar")]
pub use gearley_default_grammar::DefaultGrammar;
pub use gearley_forest::item::{Item, Origin};
pub use gearley_forest::{Forest, NullForest};
pub use gearley_grammar::Grammar;
pub use gearley_recognizer as recognizer;
pub use gearley_recognizer::perf_hint::{DefaultPerfHint, PerfHint};
pub use gearley_recognizer::Recognizer;
pub use gearley_utils as utils;
pub use gearley_utils::RecognizerParseExt;
#[cfg(feature = "simple-bocage")]
pub use simple_bocage::Bocage;
