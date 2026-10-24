//! Learn a finite-horizon value from observations, then compare it with a model.
//! Think of each value as a prediction that one new observation can correct.
//! The policy stays fixed: correcting a value does not change action probabilities.
//!
//! The sampler owns policy/world access. `ValueTable::observe` only receives a
//! transition and its remaining horizon. It never calls the model or evaluator.
//! Each (h, state) entry uses its own visit count and learning rate 1 / visits.
//!
//! Read the files in this order:
//! 1. `experience/config.rs`: the settings for one learning run.
//! 2. `experience/table.rs`: `ValueTable::observe`, one TD(0) update.
//! 3. `experience/sampling.rs`: how one transition is drawn at random.
//! 4. `experience/training.rs`: many episodes, each feeding `observe`.
//! 5. `experience/report.rs`: the learn-value lesson and its printout.
//! 6. `experience/trace.rs`: the td-trace lesson, with supplied numbers.
//! 7. `experience/error.rs`: every reason a learning step can be refused.

mod config;
mod error;
mod report;
mod sampling;
mod table;
mod trace;
mod training;

#[cfg(test)]
mod tests;

pub(crate) use config::{ConfigError, LearningConfig, MAX_EPISODES};
pub(crate) use error::LearningError;
pub(crate) use report::run;
pub(crate) use trace::trace;
