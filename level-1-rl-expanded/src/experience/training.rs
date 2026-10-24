//! Training: many sampled episodes, each one feeding `ValueTable::observe`.
//!
//! Question: how do many sampled episodes update the table, one transition at a time?
//! Example: with horizon 5, one episode makes at most 5 updates, fewer if Goodbye comes early.
//! Predict: does the same seed produce the same table twice?

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::config::LearningConfig;
use super::sampling::sample_transition;
use super::table::ValueTable;
use super::LearningError;
use crate::model::{DatingWorld, State};

/// Sample `episodes` episodes from JustMet and feed every transition to `observe`, online.
/// `each episode: for h = horizon down to 1: sample ──► observe ──► step to the next state`
/// Invariant: a loop, not a pipeline: each step draws from the RNG and may stop early at Goodbye.
pub(super) fn train(config: &LearningConfig) -> Result<ValueTable, LearningError> {
    let policy = config.policy;
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
    let mut table = ValueTable::new(config.discount);
    for _ in 0..config.episodes {
        let mut state = State::JustMet;
        for steps_remaining in (1..=config.horizon).rev() {
            let observation =
                sample_transition(&policy, &DatingWorld, &state, rng.random(), rng.random())?;
            // `observe` only borrows the record; `to_state` is then copied out for the next step.
            table.observe(steps_remaining, &observation)?;
            state = observation.to_state;
            if state.is_terminal() {
                break;
            }
        }
    }
    Ok(table)
}
