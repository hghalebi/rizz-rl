//! Checkpoint 9: compare eleven candidate policies under the same world.
//!
//! Question: which boldness setting, from 0.0 to 1.0 in steps of 0.1, earns the most?
//! Example: with the default horizon 5 and discount 0.95, boldness 0.3 wins.
//! Predict: does winning on this grid tell us anything about boldness 0.34?

use tracing::{debug, info, info_span};

use super::expected_return_of;
use crate::model::{Action, DatingWorld, Policy, State};

// ============================================================================
// CHECKPOINT 9 — POLICY SEARCH: COMPARE CANDIDATE BEHAVIORS
// ============================================================================
/// One candidate policy and the score it earned under the shared settings.
#[derive(Clone, Copy, Debug)]
pub struct PolicyEvaluation {
    pub(crate) policy: Policy,
    pub(crate) expected_return: f64,
}

/// Score eleven boldness settings under one world, horizon, and discount.
/// `boldness ∈ {0.0, 0.1, ..., 1.0} ──expected_return_of──► (policy, score)`
/// Invariant: only boldness differs between candidates; the world's probabilities never change.
/// Predict: if 0.3 wins this grid, what do we know about the untested value 0.34?
pub fn evaluate_policy_grid(
    world: DatingWorld,
    horizon: usize,
    discount: f64,
) -> Vec<PolicyEvaluation> {
    let _search_span = info_span!("policy_search", horizon, discount).entered();

    (0..=10)
        .map(|index| {
            // index / 10 lies in [0, 1] for every index in 0..=10, so this `expect` cannot fire.
            let candidate_policy = Policy::try_new(f64::from(index) / 10.0)
                .expect("every grid value index / 10 lies in [0, 1]");

            let engaged_actions = candidate_policy.possible_actions(State::Engaged);

            debug!(
                boldness = candidate_policy.boldness(),
                ask = engaged_actions.probability_of(&Action::Ask),
                story = engaged_actions.probability_of(&Action::Story),
                tease = engaged_actions.probability_of(&Action::Tease),
                coffee = engaged_actions.probability_of(&Action::Coffee),
                "candidate action probabilities from Engaged"
            );

            let candidate_expected_return =
                expected_return_of(candidate_policy, world, horizon, discount);

            info!(
                boldness = candidate_policy.boldness(),
                expected_return = candidate_expected_return,
                "candidate evaluated"
            );

            PolicyEvaluation {
                policy: candidate_policy,
                expected_return: candidate_expected_return,
            }
        })
        .collect()
}

/// Scores closer than this count as a tie.
/// Exact arithmetic ties all candidates at γ = 0; floating-point sums fall short by about 1e-16.
const TIE_TOLERANCE: f64 = 1e-12;

/// The first candidate that no later candidate beats by more than `TIE_TOLERANCE`.
/// `best ──► candidate  only if  candidate.score > best.score + TIE_TOLERANCE`
/// Invariant: ties keep the earlier, lower boldness; an empty grid has no best, so `None`.
pub fn best_policy(evaluations: &[PolicyEvaluation]) -> Option<PolicyEvaluation> {
    evaluations.iter().copied().reduce(|best, candidate| {
        if candidate.expected_return > best.expected_return + TIE_TOLERANCE {
            candidate
        } else {
            best
        }
    })
}

#[cfg(test)]
pub fn find_best_policy(
    world: DatingWorld,
    horizon: usize,
    discount: f64,
) -> Option<PolicyEvaluation> {
    best_policy(&evaluate_policy_grid(world, horizon, discount))
}
