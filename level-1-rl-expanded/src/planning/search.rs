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
#[derive(Clone, Copy, Debug)]
pub(crate) struct PolicyEvaluation {
    pub(crate) policy: Policy,
    pub(crate) expected_return: f64,
}

/// Compare eleven behavior rules under the same world, horizon, and discount.
/// Only boldness changes; the search does not learn new world probabilities.
/// Predict: if 0.3 wins this grid, what do we know about the untested value 0.34?
pub(crate) fn evaluate_policy_grid(
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> Vec<PolicyEvaluation> {
    let _search_span = info_span!("policy_search", horizon, discount).entered();

    let mut evaluations = Vec::with_capacity(11);

    // Grid search over 0.0, 0.1, 0.2, ... 1.0.
    for index in 0..=10 {
        // index runs from 0 to 10, so index / 10 runs from 0.0 to 1.0.
        // try_new cannot reject a grid value; `expect` states that guarantee.
        let candidate_policy = Policy::try_new(index as f64 / 10.0)
            .expect("every grid value index / 10 lies in [0, 1]");

        // Make the parameter concrete: show how it changes behavior in Engaged.
        let engaged_actions = candidate_policy.possible_actions(&State::Engaged);

        debug!(
            boldness = candidate_policy.boldness(),
            ask = engaged_actions.probability_of(&Action::Ask),
            story = engaged_actions.probability_of(&Action::Story),
            tease = engaged_actions.probability_of(&Action::Tease),
            coffee = engaged_actions.probability_of(&Action::Coffee),
            "candidate action probabilities from Engaged"
        );

        let candidate_expected_return =
            expected_return_of(&candidate_policy, world, horizon, discount);

        info!(
            boldness = candidate_policy.boldness(),
            expected_return = candidate_expected_return,
            "candidate evaluated"
        );

        evaluations.push(PolicyEvaluation {
            policy: candidate_policy,
            expected_return: candidate_expected_return,
        });
    }

    evaluations
}

// Exact arithmetic would tie all candidates when gamma = 0. Summing many
// floating-point branches can leave tiny differences. Treat differences
// below this absolute tolerance as ties and retain the first candidate.
const TIE_TOLERANCE: f64 = 1e-12;

pub(crate) fn best_policy(evaluations: &[PolicyEvaluation]) -> Option<PolicyEvaluation> {
    // None until a candidate is seen: an empty grid has no best policy.
    let mut best_evaluation: Option<PolicyEvaluation> = None;
    for candidate in evaluations {
        let improves_best = match best_evaluation {
            Some(current) => candidate.expected_return > current.expected_return + TIE_TOLERANCE,
            None => true,
        };
        if improves_best {
            best_evaluation = Some(*candidate);
        }
    }
    best_evaluation
}

#[cfg(test)]
pub(crate) fn find_best_policy(
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> Option<PolicyEvaluation> {
    best_policy(&evaluate_policy_grid(world, horizon, discount))
}
