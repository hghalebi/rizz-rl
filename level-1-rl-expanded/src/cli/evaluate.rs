//! The evaluate experiment: score one policy over every possible history.
//!
//! Question: what is the expected return of one boldness setting over h steps?
//! Example: horizon 1 from `JustMet` averages the four first-step rewards: 0.22.
//! Predict: at horizon 0 there is one empty history. What is its return?

use std::io::{self, Write};

use super::command::Evaluation;
use crate::model::{DatingWorld, State};
use crate::planning::{discounted_return, enumerate_trajectories};
use crate::probability::expected_value;

/// Enumerate every history for one policy, then print the probability-weighted return.
/// `E[G] = Σ P(history) · G(history)`
pub(super) fn print(
    out: &mut impl Write,
    world: DatingWorld,
    settings: Evaluation,
) -> io::Result<()> {
    let policy = settings.policy;
    let trajectories = enumerate_trajectories(policy, world, State::JustMet, settings.horizon);
    let score = expected_value(&trajectories, |trajectory| {
        discounted_return(trajectory, settings.discount)
    });
    writeln!(
        out,
        "Boldness: {} | Horizon: {} transitions | Discount: {}",
        policy.boldness(),
        settings.horizon,
        settings.discount
    )?;
    writeln!(
        out,
        "Represented trajectories: {}",
        trajectories.number_of_outcomes()
    )?;
    writeln!(
        out,
        "Total probability mass: {:.9}",
        trajectories.total_probability()
    )?;
    writeln!(out, "Expected return E[G]: {score:.6}")?;
    writeln!(
        out,
        "E[G] averages complete returns using their path probabilities."
    )?;
    writeln!(
        out,
        "Optional experiment: change only --horizon, then explain the change in E[G]."
    )?;
    Ok(())
}
