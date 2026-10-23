//! How the sampler draws one transition at random from the known model.
//!
//! Question: how does one random draw in [0, 1) pick one branch of a distribution?
//! Example: with weights [0.25, 0.75], draw 0.2 picks the first branch and 0.8 the second.
//! Predict: which branch does the draw exactly 0.25 pick?

use super::LearningError;
use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

const MASS_TOLERANCE: f64 = 1e-12;

/// Cut [0, 1) into segments whose lengths are the branch probabilities.
/// With weights [0.25, 0.75], draw 0.2 selects the first and 0.8 the second.
/// This is inverse cumulative sampling; supplied draws make boundaries testable.
/// Predict: which segment owns the draw exactly at 0.25?
/// Returning &T borrows the chosen value rather than cloning every alternative.
/// Reject malformed distributions before drawing. Scale by their accepted mass
/// to account only for roundoff within MASS_TOLERANCE; do not repair bad models.
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
            .any(|(_, p)| !p.is_finite() || !(0.0..=1.0).contains(p))
    {
        return Err(LearningError::InvalidProbabilities);
    }
    let mass = distribution.total_probability();
    if (mass - 1.0).abs() > MASS_TOLERANCE {
        return Err(LearningError::UnnormalizedMass(mass));
    }
    let threshold = draw * mass;
    let mut cumulative = 0.0;
    let mut last_positive = None;
    for (outcome, probability) in &distribution.outcomes {
        cumulative += probability;
        if *probability > 0.0 {
            last_positive = Some(outcome);
            if threshold < cumulative {
                return Ok(outcome);
            }
        }
    }
    // A valid draw can round to mass during multiplication. Never select a
    // zero-probability final branch as a floating-point boundary fallback.
    last_positive.ok_or(LearningError::NoPositiveProbability)
}

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
    // These selections borrow local distributions. Copy just the chosen values
    // into an owned observation that can outlive those temporary distributions.
    Ok(Transition {
        from_state: *state,
        action: *action,
        reward: world.reward_for(state, action, next_state),
        to_state: *next_state,
    })
}
