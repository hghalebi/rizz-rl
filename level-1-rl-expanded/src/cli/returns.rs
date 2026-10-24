//! The returns experiment: discount one supplied reward sequence by hand.
//!
//! Question: what is the sequence [1, 2, 4] worth at a chosen discount?
//! Example: at discount 0.5 the weights are 1, 0.5, 0.25, so G = 1 + 1 + 1 = 3.
//! Predict: G at discount 0, then G at discount 1.

use std::io::{self, Write};

use crate::model::{Action, State, Transition};
use crate::planning::{discount_weights, discounted_return, Trajectory};

/// Print one row per supplied reward, then the discounted return of the whole sequence.
/// `contribution = weight · reward`, with the weights from `discount_weights`
/// Invariant: the rows and the final `G` use the same weight sequence as the planner does.
pub(super) fn print(out: &mut impl Write, discount: f64) -> io::Result<()> {
    // The rewards are supplied by the exercise, not produced by `DatingWorld::reward_for`.
    let trajectory: Trajectory = [1.0, 2.0, 4.0]
        .into_iter()
        .map(|reward| Transition {
            from_state: State::Engaged,
            action: Action::Ask,
            reward,
            to_state: State::Engaged,
        })
        .collect();
    writeln!(
        out,
        "Supplied synthetic reward trace: [1, 2, 4]. Discount: {discount}"
    )?;
    writeln!(out, "G = r_0 + discount*r_1 + discount^2*r_2")?;
    writeln!(out, "step   reward     weight   contribution")?;
    for (step, (weight, transition)) in discount_weights(discount).zip(&trajectory).enumerate() {
        writeln!(
            out,
            "{step:>4}   {:>6.2}   {weight:>8.4}   {:>12.4}",
            transition.reward,
            weight * transition.reward
        )?;
    }
    writeln!(
        out,
        "Discounted return G: {:.6}",
        discounted_return(&trajectory, discount)
    )?;
    writeln!(
        out,
        "Optional experiment: predict G for --discount 0, 0.5, and 1 before running them."
    )?;
    Ok(())
}
