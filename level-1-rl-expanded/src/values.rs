//! Evaluate the same policy without storing every possible history.
//!
//! If two histories reach the same state with the same number of steps left,
//! their remaining expected return is identical in this Markov model.
//! Cache that answer and reuse it: this is finite-horizon dynamic programming.
//!
//! ```text
//! V_0(s) = 0; V_h(terminal) = 0
//! V_h(s) = sum_a pi(a|s) sum_s' P(s'|s,a) [R(s,a,s') + gamma V_(h-1)(s')]
//! ```
//!
//! `possible_transitions` supplies the joint probability `pi * P` and reward.
//! `steps_remaining` is h; the recursive call supplies the future value.
//! The immediate reward is not discounted. Only the future value is.
//! This evaluates a known model; it does not learn from experience.

use super::{possible_transitions, DatingWorld, Policy, State};

pub(super) struct ValueEstimate {
    pub(super) expected_return: f64,
    /// Distinct nonterminal (state, steps_remaining) subproblems evaluated.
    /// Zero-horizon and terminal base cases do not count.
    pub(super) evaluated_states: usize,
}

struct CachedValue {
    state: State,
    steps_remaining: usize,
    expected_return: f64,
}

/// Evaluate a fixed policy with a fresh cache. The caller supplies gamma in [0, 1].
/// A small Vec makes cache inspection easy without requiring State to be Hash.
pub(super) fn evaluate_value(
    policy: &Policy,
    world: &DatingWorld,
    initial_state: &State,
    horizon: usize,
    discount: f64,
) -> ValueEstimate {
    let mut cache = Vec::new();
    let expected_return = value_for(policy, world, initial_state, horizon, discount, &mut cache);
    ValueEstimate {
        expected_return,
        evaluated_states: cache.len(),
    }
}

fn value_for(
    policy: &Policy,
    world: &DatingWorld,
    state: &State,
    steps_remaining: usize,
    discount: f64,
    cache: &mut Vec<CachedValue>,
) -> f64 {
    if steps_remaining == 0 || state.is_terminal() {
        return 0.0;
    }

    if let Some(cached) = cache
        .iter()
        .find(|entry| entry.state == *state && entry.steps_remaining == steps_remaining)
    {
        return cached.expected_return;
    }

    let mut expected_return = 0.0;
    for (transition, probability) in possible_transitions(policy, world, state).outcomes {
        if probability == 0.0 {
            continue;
        }
        let future_value = value_for(
            policy,
            world,
            &transition.to_state,
            steps_remaining - 1,
            discount,
            cache,
        );
        expected_return += probability * (transition.reward + discount * future_value);
    }

    cache.push(CachedValue {
        state: state.clone(),
        steps_remaining,
        expected_return,
    });
    expected_return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{discounted_return, enumerate_trajectories, expected_value};

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected:.12}, got {actual:.12}"
        );
    }

    #[test]
    fn one_step_weights_rewards_without_discounting_them() {
        // Engaged receives +1 with probability .74; LowInterest receives -2 with .26.
        let policy = Policy { boldness: 0.5 };
        for discount in [0.0, 0.95, 1.0] {
            let value = evaluate_value(&policy, &DatingWorld, &State::JustMet, 1, discount);
            assert_close(value.expected_return, 0.22);
            assert_eq!(value.evaluated_states, 1);
        }
    }

    #[test]
    fn terminal_and_zero_horizon_have_no_future_reward() {
        let policy = Policy { boldness: 0.5 };
        for (state, horizon) in [(State::Goodbye, 4), (State::Engaged, 0)] {
            let value = evaluate_value(&policy, &DatingWorld, &state, horizon, 0.95);
            assert_close(value.expected_return, 0.0);
            assert_eq!(value.evaluated_states, 0);
        }
        // Reward for entering Goodbye still counts; only rewards after it stop.
        let value = evaluate_value(&policy, &DatingWorld, &State::LowInterest, 4, 0.95);
        assert_close(value.expected_return, 2.0);
        assert_eq!(value.evaluated_states, 1);
    }

    #[test]
    fn bellman_matches_complete_tree_across_states_horizons_and_parameters() {
        let states = [
            State::JustMet,
            State::Engaged,
            State::Playful,
            State::LowInterest,
            State::MutualInterest,
            State::Goodbye,
        ];
        for state in &states {
            for horizon in 0..=4 {
                for discount in [0.0, 0.95, 1.0] {
                    for boldness in [0.0, 0.3, 0.5, 1.0] {
                        let policy = Policy { boldness };
                        let tree = enumerate_trajectories(&policy, &DatingWorld, state, horizon);
                        let tree_value =
                            expected_value(&tree, |path| discounted_return(path, discount));
                        let bellman =
                            evaluate_value(&policy, &DatingWorld, state, horizon, discount);
                        assert_close(bellman.expected_return, tree_value);
                        // Five nonterminal states times h possible remaining-step values.
                        assert!(bellman.evaluated_states <= 5 * horizon);
                    }
                }
            }
        }
    }
}
