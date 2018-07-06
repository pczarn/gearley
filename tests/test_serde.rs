#![cfg(feature = "serde")]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate cfg;
extern crate gearley;
extern crate serde;

mod grammars;

use gearley::forest::NullForest;
use gearley::grammar::Grammar;
use gearley::recognizer::Recognizer;

use grammars::*;

use serde::de::value::StringDeserializer;
use serde::de::IntoDeserializer;

#[test]
fn test_serde() {
    let x = InternalGrammar::deserialize(String::into_deserializer(""));
    assert!(true);
}
