//! Evaluate the same policy without storing every possible history.
//!
//! If two histories reach the same state with the same number of steps left,
//! their remaining expected return is identical in this Markov model.
//! Cache that answer and reuse it: this is finite-horizon dynamic programming.
//! In a separate deterministic example, rewards 0 now and 5 next are worth 2.5
//! at discount 0.5 with two steps left, but 0 with only one step left.
//! Predict: which answer would a cache keyed only by state wrongly reuse?
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

use crate::model::{DatingWorld, Policy, State};
use crate::planning::possible_transitions;

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
/// Starting fresh also prevents another policy or discount from inheriting these
/// answers. Recursive calls borrow this one cache instead of copying its entries.
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
    // This is the value of what comes AFTER arriving here. The caller still
    // counts the arriving reward. Horizon zero ends this finite objective,
    // even if the environment state itself is not terminal.
    if steps_remaining == 0 || state.is_terminal() {
        return 0.0;
    }

    if let Some(cached) = cache
        .iter()
        .find(|entry| entry.state == *state && entry.steps_remaining == steps_remaining)
    {
        // An f64 is Copy: return the answer without taking its entry out of storage.
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
        // First score the branch, then weight how likely it is. A hypothetical
        // reward 2 plus discounted future 0.5*4 gives 4; chance 0.25 contributes 1.
        // Predict: does zero continuation value also erase the immediate reward?
        expected_return += probability * (transition.reward + discount * future_value);
    }

    // The cache owns its state key after this borrowed state goes out of scope.
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
    use crate::model::{Action, Transition};
    use crate::planning::{discounted_return, enumerate_trajectories};
    use crate::probability::expected_value;

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
    fn the_greedy_action_can_depend_on_remaining_horizon() {
        // A separate deterministic toy model: TakeNow ends with reward 2;
        // Wait yields 0 now and 5 next. These are supplied reward traces,
        // not DatingWorld rewards. Use the same production return calculation.
        let take_now = vec![Transition {
            from_state: State::JustMet,
            action: Action::Goodbye,
            reward: 2.0,
            to_state: State::Goodbye,
        }];
        let wait = [
            Transition {
                from_state: State::JustMet,
                action: Action::Ask,
                reward: 0.0,
                to_state: State::Engaged,
            },
            Transition {
                from_state: State::Engaged,
                action: Action::Goodbye,
                reward: 5.0,
                to_state: State::Goodbye,
            },
        ];
        let gamma = 0.5;
        let now_with_one_step = discounted_return(&take_now, gamma);
        let wait_with_one_step = discounted_return(&wait[..1].to_vec(), gamma);
        assert_close(now_with_one_step, 2.0);
        assert_close(wait_with_one_step, 0.0);
        assert!(now_with_one_step > wait_with_one_step);

        // Terminal reward 2 receives no further rewards, even when h is 2.
        let now_with_two_steps = discounted_return(&take_now, gamma);
        let wait_with_two_steps = discounted_return(&wait.to_vec(), gamma);
        assert_close(now_with_two_steps, 2.0);
        assert_close(wait_with_two_steps, 2.5);
        assert!(wait_with_two_steps > now_with_two_steps);
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
