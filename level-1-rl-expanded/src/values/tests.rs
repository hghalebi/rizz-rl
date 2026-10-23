//! Checks for the exact Bellman evaluator.

use super::*;
use crate::model::{Action, Transition};
use crate::planning::{discounted_return, enumerate_trajectories};
use crate::probability::expected_value;

fn policy(boldness: f64) -> Policy {
    Policy::try_new(boldness).expect("test boldness lies in [0, 1]")
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected:.12}, got {actual:.12}"
    );
}

#[test]
fn one_step_weights_rewards_without_discounting_them() {
    // Engaged receives +1 with probability .74; LowInterest receives -2 with .26.
    let policy = policy(0.5);
    for discount in [0.0, 0.95, 1.0] {
        let value = evaluate_value(&policy, &DatingWorld, &State::JustMet, 1, discount);
        assert_close(value.expected_return, 0.22);
        assert_eq!(value.evaluated_states, 1);
    }
}

#[test]
fn terminal_and_zero_horizon_have_no_future_reward() {
    let policy = policy(0.5);
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
    let wait_with_one_step = discounted_return(&wait[..1], gamma);
    assert_close(now_with_one_step, 2.0);
    assert_close(wait_with_one_step, 0.0);
    assert!(now_with_one_step > wait_with_one_step);

    // Terminal reward 2 receives no further rewards, even when h is 2.
    let now_with_two_steps = discounted_return(&take_now, gamma);
    let wait_with_two_steps = discounted_return(&wait, gamma);
    assert_close(now_with_two_steps, 2.0);
    assert_close(wait_with_two_steps, 2.5);
    assert!(wait_with_two_steps > now_with_two_steps);
}

#[test]
fn bellman_matches_complete_tree_across_states_horizons_and_parameters() {
    for state in &State::ALL {
        for horizon in 0..=4 {
            for discount in [0.0, 0.95, 1.0] {
                for boldness in [0.0, 0.3, 0.5, 1.0] {
                    let policy = policy(boldness);
                    let tree = enumerate_trajectories(&policy, &DatingWorld, state, horizon);
                    let tree_value =
                        expected_value(&tree, |path| discounted_return(path, discount));
                    let bellman = evaluate_value(&policy, &DatingWorld, state, horizon, discount);
                    assert_close(bellman.expected_return, tree_value);
                    // Five nonterminal states times h possible remaining-step values.
                    assert!(bellman.evaluated_states <= 5 * horizon);
                }
            }
        }
    }
}
