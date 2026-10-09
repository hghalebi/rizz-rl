//! SPOILERS: reference solutions for this track's practice.rs.
//!
//! Try an exercise first, then compare the rule and the Rust implementation.
//! Each function has the same contract and acceptance cases as its starter.
//! Unless stated otherwise, inputs are finite, discounts and learning rates
//! lie in [0, 1], and supplied distributions are normalized unless empty.

use super::cases;

/// Exercise 1: each extra time step contributes one factor of discount.
/// A mutable weight avoids needing integer powers or indexing the slice.
pub(super) fn discounted_return(rewards: &[f64], discount: f64) -> f64 {
    let mut total = 0.0;
    let mut weight = 1.0;
    for &reward in rewards {
        total += weight * reward;
        weight *= discount;
    }
    total
}

/// Exercise 2: an expectation weights each value by its probability.
/// An iterator sum starts at zero, including when the slice is empty.
pub(super) fn weighted_expectation(outcomes: &[(f64, f64)]) -> f64 {
    outcomes
        .iter()
        .map(|&(value, probability)| value * probability)
        .sum()
}

/// Exercise 3: termination ends future value, but keeps the immediate reward.
/// A time limit in a continuing task uses `terminated = false`.
pub(super) fn td_target(reward: f64, discount: f64, next_value: f64, terminated: bool) -> f64 {
    if terminated {
        reward
    } else {
        reward + discount * next_value
    }
}

/// Exercise 4: inspect both individual weights and the total mass.
/// The tolerance allows rounding error, without changing the supplied weights.
pub(super) fn is_probability_distribution(weights: &[f64]) -> bool {
    !weights.is_empty()
        && weights
            .iter()
            .all(|weight| weight.is_finite() && (0.0..=1.0).contains(weight))
        && (weights.iter().sum::<f64>() - 1.0).abs() <= 1e-9
}

/// Exercise 5: seed the maximum from an actual value, which may be negative.
/// Strict `>` preserves the first index when action values tie.
/// Input action values must be finite; an empty slice has no greedy action.
pub(super) fn greedy_action(q_values: &[f64]) -> Option<usize> {
    let &first_value = q_values.first()?;
    let mut best_index = 0;
    let mut best_value = first_value;
    for (index, &value) in q_values.iter().enumerate().skip(1) {
        if value > best_value {
            best_index = index;
            best_value = value;
        }
    }
    Some(best_index)
}

/// Exercise 6: all actions receive exploration probability, even the greedy one.
/// The remaining mass exploits the first greedy action. This constructs a
/// distribution; it does not sample an action. Epsilon must lie in [0, 1].
pub(super) fn epsilon_greedy(q_values: &[f64], epsilon: f64) -> Vec<f64> {
    let Some(greedy_index) = greedy_action(q_values) else {
        return Vec::new();
    };
    let exploration_probability = epsilon / q_values.len() as f64;
    let mut probabilities = vec![exploration_probability; q_values.len()];
    probabilities[greedy_index] += 1.0 - epsilon;
    probabilities
}

/// Exercise 7: move the old mean by one nth of the new observation's error.
/// `sample_count` includes the new observation and must be at least one.
/// On the first sample there is no prior mean to retain.
pub(super) fn update_mean(previous_mean: f64, new_return: f64, sample_count: usize) -> f64 {
    if sample_count == 1 {
        new_return
    } else {
        previous_mean + (new_return - previous_mean) / sample_count as f64
    }
}

/// Exercise 8: the TD error is target minus current estimate.
/// Alpha controls how far the estimate moves toward that target.
pub(super) fn td_update(
    current_value: f64,
    reward: f64,
    discount: f64,
    next_value: f64,
    terminated: bool,
    learning_rate: f64,
) -> f64 {
    let target = td_target(reward, discount, next_value, terminated);
    let error = target - current_value;
    current_value + learning_rate * error
}

/// Exercise 9: Q-learning bootstraps from the highest next action value.
/// Terminal transitions need no next actions. A continuing transition without
/// next actions is undefined by this exercise's contract, even at discount zero.
pub(super) fn q_learning_target(
    reward: f64,
    discount: f64,
    next_action_values: &[f64],
    terminated: bool,
) -> Option<f64> {
    if terminated {
        return Some(reward);
    }
    let best_index = greedy_action(next_action_values)?;
    Some(reward + discount * next_action_values[best_index])
}

/// Exercise 10: average one-step targets across a known distribution.
/// Tuples contain (probability, reward, next value); terminal next values are
/// already zero. This exact model backup differs from a single sampled target.
pub(super) fn bellman_backup(outcomes: &[(f64, f64, f64)], discount: f64) -> f64 {
    outcomes
        .iter()
        .map(|&(probability, reward, next_value)| probability * (reward + discount * next_value))
        .sum()
}

#[test]
fn exercise_1_discounted_return() {
    cases::exercise_1(discounted_return);
}

#[test]
fn exercise_2_weighted_expectation() {
    cases::exercise_2(weighted_expectation);
}

#[test]
fn exercise_3_td_target() {
    cases::exercise_3(td_target);
}

#[test]
fn exercise_4_is_probability_distribution() {
    cases::exercise_4(is_probability_distribution);
}

#[test]
fn exercise_5_greedy_action() {
    cases::exercise_5(greedy_action);
}

#[test]
fn exercise_6_epsilon_greedy() {
    cases::exercise_6(epsilon_greedy);
}

#[test]
fn exercise_7_update_mean() {
    cases::exercise_7(update_mean);
}

#[test]
fn exercise_8_td_update() {
    cases::exercise_8(td_update);
}

#[test]
fn exercise_9_q_learning_target() {
    cases::exercise_9(q_learning_target);
}

#[test]
fn exercise_10_bellman_backup() {
    cases::exercise_10(bellman_backup);
}
