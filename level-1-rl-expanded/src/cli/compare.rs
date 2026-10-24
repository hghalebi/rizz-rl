//! The compare experiment: tree enumeration and Bellman evaluation, side by side.
//!
//! Question: do two different exact methods give the same value for the same policy?
//! Example: at H = 1 both give 0.22, the first-step average; at H = 0 both give 0.
//! Predict: the "cached subproblems" count at H = 1. Which single subproblem is it?

use std::io::{self, Write};

use super::command::Evaluation;
use crate::model::{DatingWorld, State};
use crate::planning::expected_return_of;
use crate::values::evaluate_value;

/// For each horizon from 0 to H, print the tree value, the Bellman value, and their gap.
/// Invariant: both columns evaluate the same model and policy; gaps come only from floating point.
pub(super) fn print(
    out: &mut impl Write,
    world: &DatingWorld,
    settings: Evaluation,
) -> io::Result<()> {
    let policy = settings.policy;
    writeln!(
        out,
        "Boldness: {} | Discount: {}",
        policy.boldness(),
        settings.discount
    )?;
    writeln!(
        out,
        "V_0(s) = 0; V_h(s) = sum p(a,s'|s) * [r + discount*V_(h-1)(s')]"
    )?;
    writeln!(
        out,
        "   H       Tree E[G]   Bellman V_h   absolute difference   cached subproblems"
    )?;
    for horizon in 0..=settings.horizon {
        let score = expected_return_of(&policy, world, horizon, settings.discount);
        let value = evaluate_value(&policy, world, &State::JustMet, horizon, settings.discount);
        writeln!(
            out,
            "{horizon:>4}   {score:>13.6}   {:>11.6}   {:>19.3e}   {:>18}",
            value.expected_return,
            (score - value.expected_return).abs(),
            value.evaluated_states
        )?;
    }
    writeln!(out, "Both columns evaluate the same model and policy; small differences come from floating-point arithmetic.")?;
    writeln!(out, "Bellman evaluation reuses (state, steps remaining) values; the cache count excludes base cases.")?;
    writeln!(
        out,
        "Optional experiment: explain why V_0 is zero and V_1 matches first-step."
    )?;
    Ok(())
}
