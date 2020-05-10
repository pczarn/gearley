#![allow(dead_code)]

mod cartesian_product;
pub mod parse;
mod simple_compact_evaluator;
mod simple_evaluator;
mod traversal_description;

pub use self::parse::Parse;
pub use self::simple_compact_evaluator::{SimpleCompactEvaluator, compact_traversal_description};
pub use self::simple_evaluator::{SimpleEvaluator, traversal_description};
