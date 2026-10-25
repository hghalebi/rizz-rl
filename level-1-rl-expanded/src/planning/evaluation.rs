//! Score a whole policy: average every history's return, weighted by its chance.
//!
//! Question: how good is a policy across all of its possible histories?
//! Example: returns 2 (chance 0.75) and 6 (chance 0.25) average to 3.
//! Predict: must the chances of all histories still sum to 1 at every horizon?

use tracing::{debug, debug_span, warn};

use super::{discounted_return, enumerate_trajectories};
use crate::model::{DatingWorld, Policy, State};
use crate::probability::expected_value;

/// Score a policy from `JustMet`: discount inside each history, then weight histories by chance.
/// `E[G] = Σ P(history) · G(history)`;  returns 2 @ 0.75 and 6 @ 0.25 give 3
/// Invariant: the histories' chances still sum to 1; branching moves mass, never creates it.
/// Predict: could a guaranteed return of 3 hide a different risk from this pair?
pub fn expected_return_of(
    policy: Policy,
    world: DatingWorld,
    horizon: usize,
    discount: f64,
) -> f64 {
    let _evaluation_span = debug_span!(
        "evaluate_policy",
        boldness = policy.boldness(),
        horizon,
        discount,
    )
    .entered();

    let possible_trajectories = enumerate_trajectories(policy, world, State::JustMet, horizon);

    let probability_mass = possible_trajectories.total_probability();

    if (probability_mass - 1.0).abs() > 1e-9 {
        warn!(
            probability_mass,
            "trajectory probability mass is not conserved"
        );
    }

    let expected_return = expected_value(&possible_trajectories, |trajectory| {
        discounted_return(trajectory, discount)
    });

    debug!(
        number_of_trajectories = possible_trajectories.number_of_outcomes(),
        probability_mass, expected_return, "policy evaluation completed"
    );

    expected_return
}
