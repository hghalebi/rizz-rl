//! How the sampler draws one transition at random from the known model.
//!
//! Question: how does one random draw in [0, 1) pick one branch of a distribution?
//! Example: with weights [0.25, 0.75], draw 0.2 picks the first branch and 0.8 the second.
//! Predict: which branch does the draw exactly 0.25 pick?

use super::LearningError;
use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

/// Total probability may miss 1 by this much and still count as normalized.
const MASS_TOLERANCE: f64 = 1e-12;

/// Pick one branch by inverse cumulative sampling: the draw lands in one segment of [0, 1).
/// `|──0.25──|────0.75────|  draw 0.2 ──► first;  0.25 or 0.8 ──► second`
/// Invariant: malformed input is refused before drawing; a zero-mass branch is never chosen.
/// Predict: which segment owns the draw exactly at 0.25?
pub(super) fn select_outcome<T>(
    distribution: &Distribution<T>,
    draw: f64,
) -> Result<&T, LearningError> {
    if !draw.is_finite() || !(0.0..1.0).contains(&draw) {
        return Err(LearningError::InvalidDraw);
    }
    if distribution.outcomes.is_empty()
        || distribution
            .outcomes
            .iter()
            .any(|(_, probability)| !probability.is_finite() || !(0.0..=1.0).contains(probability))
    {
        return Err(LearningError::InvalidProbabilities);
    }
    let mass = distribution.total_probability();
    if (mass - 1.0).abs() > MASS_TOLERANCE {
        return Err(LearningError::UnnormalizedMass(mass));
    }
    // Scaling by the accepted mass absorbs roundoff within MASS_TOLERANCE; it repairs nothing.
    let threshold = draw * mass;
    let first_segment_past_threshold = distribution
        .outcomes
        .iter()
        .scan(0.0, |upper_edge, (outcome, probability)| {
            *upper_edge += probability;
            Some((outcome, *probability, *upper_edge))
        })
        .filter(|(_, probability, _)| *probability > 0.0)
        .find(|(_, _, upper_edge)| threshold < *upper_edge)
        .map(|(outcome, _, _)| outcome);
    // Roundoff can push `threshold` up to `mass` itself; then the last positive branch owns it.
    let last_positive_branch = || {
        distribution
            .outcomes
            .iter()
            .rev()
            .find(|(_, probability)| *probability > 0.0)
            .map(|(outcome, _)| outcome)
    };
    first_segment_past_threshold
        .or_else(last_positive_branch)
        .ok_or(LearningError::NoPositiveProbability)
}

/// Draw one action from the policy, then one next state from the world, and record the step.
/// `state ──policy, draw 1──► action ──world, draw 2──► next_state`
/// Invariant: the sampler sees the model; the learner later sees only this Transition.
pub(super) fn sample_transition(
    policy: &Policy,
    world: &DatingWorld,
    state: &State,
    action_draw: f64,
    next_state_draw: f64,
) -> Result<Transition, LearningError> {
    if state.is_terminal() {
        return Err(LearningError::SampleFromTerminal);
    }
    let actions = policy.possible_actions(state);
    let action = select_outcome(&actions, action_draw)?;
    let next_states = world.possible_next_states(state, action);
    let next_state = select_outcome(&next_states, next_state_draw)?;
    // The selections borrow local distributions; `*` copies the chosen values out before they drop.
    Ok(Transition {
        from_state: *state,
        action: *action,
        reward: world.reward_for(state, action, next_state),
        to_state: *next_state,
    })
}
