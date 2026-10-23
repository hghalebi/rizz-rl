//! The learn-value lesson: train, compare with the exact value, and print a report.
//!
//! Question: how far is the learned value from the exact one, and what was actually visited?
//! Example: 10,000 episodes from JustMet with horizon 5 visit (h = 5, JustMet) 10,000 times.
//! Predict: an entry shows value 0 and 0 visits. Was 0 learned, or never tested?

use std::fmt;

use super::config::{Discount, LearningConfig};
use super::table::ValueTable;
use super::training::train;
use super::LearningError;
use crate::model::{DatingWorld, State};
use crate::values::evaluate_value;

/// The finished learn-value experiment. `run` does all of the computation;
/// `Display` only formats these numbers for the lesson.
pub(crate) struct LearningReport {
    config: LearningConfig,
    exact: f64,
    table: ValueTable,
}

pub(crate) fn run(config: &LearningConfig) -> Result<LearningReport, LearningError> {
    let table = train(config)?;
    // Only comparison code sees exact values; neither the sampler nor the
    // learner is initialized from, or updated using, these reference answers.
    // This checks learning against the same toy model, not whether the model
    // describes real interactions. Predict: what might one lucky seed conceal?
    let exact = evaluate_value(
        &config.policy,
        &DatingWorld,
        &State::JustMet,
        config.horizon,
        config.discount.0,
    )
    .expected_return;
    Ok(LearningReport {
        config: *config,
        exact,
        table,
    })
}

impl fmt::Display for LearningReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LearningConfig {
            policy,
            horizon,
            discount: Discount(discount),
            episodes,
            seed,
        } = self.config;
        let boldness = policy.boldness();
        let learned = self.table.value(&State::JustMet, horizon);
        let visits = self
            .table
            .entry(&State::JustMet, horizon)
            .map_or(0, |entry| entry.visits);
        writeln!(f, "Sampled TD(0) evaluation of a fixed policy.")?;
        writeln!(
            f,
            "Boldness: {boldness} | Horizon: {horizon} | Discount: {discount} | Episodes: {episodes} | Seed: {seed}"
        )?;
        writeln!(f, "Exact V_h(JustMet): {:.6}", self.exact)?;
        writeln!(f, "Learned V_h(JustMet): {learned:.6}")?;
        writeln!(f, "Absolute error: {:.6}", (learned - self.exact).abs())?;
        writeln!(f, "Initial-state visits: {visits}")?;
        if horizon == 0 {
            writeln!(
                f,
                "Horizon 0 is a known base case: no transitions or learning updates."
            )?;
        }
        writeln!(
            f,
            "Table coverage (all nonterminal states, including unreachable entries):"
        )?;
        writeln!(f, "h state              value    visits status")?;
        for steps_remaining in 1..=horizon {
            for state in State::ALL.iter().filter(|state| !state.is_terminal()) {
                let (value, visits, status) = self
                    .table
                    .entry(state, steps_remaining)
                    .map_or((0.0, 0, "unvisited"), |entry| {
                        (entry.value, entry.visits, "visited")
                    });
                // Derived Debug ignores width, so format the label before padding it.
                let state_label = format!("{state:?}");
                writeln!(
                    f,
                    "{steps_remaining} {state_label:<14} {value:>10.6} {visits:>9} {status}"
                )?;
            }
        }
        writeln!(f, "Terminal and h=0 values are fixed at zero; unvisited entries retain the initial estimate.")?;
        writeln!(f, "Learning rate = 1 / visits for each (h, state); targets change as later-state estimates change.")?;
        writeln!(
            f,
            "This learns values for a fixed policy, not an improved policy."
        )?;
        writeln!(
            f,
            "Compare several seeds and episode budgets; individual errors need not decrease."
        )
    }
}
