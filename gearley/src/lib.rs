#![cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]

pub mod utils;
pub mod recognizer;

pub use gearley_forest::{Forest, NullForest};
pub use gearley_grammar::Grammar;
pub use crate::recognizer::{Recognizer, item::Item, item::Origin};
pub use crate::recognizer::performance_policy::PerformancePolicy;

pub(crate) mod local_prelude {
    pub use gearley_forest::{Forest, NullForest};
    pub use gearley_grammar::Grammar;
    pub use crate::recognizer::{Recognizer, item::Item, item::Origin};
    pub use crate::recognizer::performance_policy::PerformancePolicy;
}
