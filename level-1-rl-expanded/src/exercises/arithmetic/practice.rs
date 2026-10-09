//! EXERCISE ONLY: ten small Rust tasks, with no solutions in this file.
//!
//! Work on one function at a time. Predict each test result before coding.
//! See guides/EXERCISES.md for commands. Tests are ignored until selected;
//! an unfinished `todo!()` should fail the selected exercise, not normal checks.
//! Inputs to arithmetic exercises are finite and modest in magnitude; discount,
//! learning rate, and epsilon are in [0, 1]. Supplied probability distributions
//! are normalized unless empty. Exercise 4 explicitly tests invalid inputs;
//! elsewhere, input validation is out of scope. Solutions are in solutions.rs.
//! Test filtering selects execution; syntax or type errors anywhere in this
//! test target still prevent compilation. Untouched TODOs remain compilable.

use super::cases;

/// Exercise 1: score a sequence of rewards with discounting.
/// The first reward is immediate. An empty sequence has return zero.
/// Rust focus: borrowing a slice and returning an f64 without changing inputs.
fn discounted_return(_rewards: &[f64], _discount: f64) -> f64 {
    todo!("Exercise 1: implement discounted return")
}

/// Exercise 2: compute expected value from (value, probability) pairs.
/// An empty distribution has weighted sum zero for this exercise.
/// Rust focus: tuple fields and iterating over borrowed values.
fn weighted_expectation(_outcomes: &[(f64, f64)]) -> f64 {
    todo!("Exercise 2: implement weighted expectation")
}

/// Exercise 3: calculate a one-step temporal-difference target.
/// `next_value` is an existing estimate, not an immediate reward.
/// A true task termination disables bootstrapping from that estimate.
/// Stopping observation only because of a time limit uses `terminated = false`
/// when the underlying task continues. This differs from a finite task whose
/// deadline really ends its objective. This function computes no value update.
/// Rust focus: express these two cases explicitly with a boolean condition.
fn td_target(_reward: f64, _discount: f64, _next_value: f64, _terminated: bool) -> f64 {
    todo!("Exercise 3: implement the temporal-difference target")
}

/// Exercise 4: check whether weights form a probability distribution.
/// Require at least one weight, every weight finite and in [0, 1], and the total
/// within 1e-9 of 1. Reject invalid inputs; do not repair or normalize them.
/// Rust focus: borrowed slices, f64::is_finite, and explicit validation.
fn is_probability_distribution(_weights: &[f64]) -> bool {
    todo!("Exercise 4: validate a probability distribution")
}

/// Exercise 5: choose the index of the highest estimated action value.
/// Empty input returns None. Exact ties choose the lowest index. Negative
/// values are valid, so the best value is not necessarily positive.
/// Rust focus: Option<usize> and indexing while borrowing a slice.
fn greedy_action(_q_values: &[f64]) -> Option<usize> {
    todo!("Exercise 5: choose a greedy action")
}

/// Exercise 6: return an epsilon-greedy distribution over action indices.
/// The exploration probability epsilon is shared uniformly across ALL actions,
/// including the greedy action. Remaining probability belongs to the first
/// greedy action from Exercise 5. Empty input returns an empty vector.
/// Return probabilities only; do not draw a random action.
/// Rust focus: allocate a Vec and reuse an earlier function.
fn epsilon_greedy(_q_values: &[f64], _epsilon: f64) -> Vec<f64> {
    todo!("Exercise 6: construct an epsilon-greedy policy")
}

/// Exercise 7: update the mean episode return after observing a new return.
/// sample_count includes the NEW sample and is at least 1. previous_mean
/// summarizes earlier samples; on sample 1 it is irrelevant. Do not keep the
/// full history. Only the updated mean is returned.
/// Rust focus: convert a count to f64 and keep a function free of side effects.
fn update_mean(_previous_mean: f64, _new_return: f64, _sample_count: usize) -> f64 {
    todo!("Exercise 7: update a running sample mean")
}

/// Exercise 8: move a value estimate toward the TD target from Exercise 3.
/// learning_rate 0 leaves the current estimate unchanged; 1 replaces it with
/// the target. True termination disables the next-state bootstrap. Return the
/// updated value; do not mutate the caller's storage.
/// Rust focus: reuse a function and name intermediate quantities clearly.
fn td_update(
    _current_value: f64,
    _reward: f64,
    _discount: f64,
    _next_value: f64,
    _terminated: bool,
    _learning_rate: f64,
) -> f64 {
    todo!("Exercise 8: perform a TD value update")
}

/// Exercise 9: calculate a Q-learning target using the best next action value.
/// A terminal transition returns Some(reward), even with no next actions.
/// A continuing transition with no next actions returns None, even if discount
/// is zero. It represents missing information, not a next value of zero.
/// All supplied actions are assumed legal; the function selects no real action.
/// Rust focus: Option, terminal branching, and reuse of greedy selection.
fn q_learning_target(
    _reward: f64,
    _discount: f64,
    _next_action_values: &[f64],
    _terminated: bool,
) -> Option<f64> {
    todo!("Exercise 9: compute a Q-learning target")
}

/// Exercise 10: evaluate one Bellman expectation backup for a fixed policy.
/// Each tuple is (joint_probability, immediate_reward, next_state_value).
/// Probabilities already include the policy and the world model. A terminal
/// branch supplies next_state_value = 0. An empty list has weighted sum zero.
/// Average over the alternatives; do not choose the maximum branch.
/// Rust focus: destructure tuples and distinguish time weights from likelihoods.
fn bellman_backup(_outcomes: &[(f64, f64, f64)], _discount: f64) -> f64 {
    todo!("Exercise 10: compute a Bellman expectation backup")
}

#[test]
#[ignore = "EXERCISE ONLY: implement discounted_return, then run with --ignored"]
fn exercise_1_discounted_return() {
    cases::exercise_1(discounted_return);
}

#[test]
#[ignore = "EXERCISE ONLY: implement weighted_expectation, then run with --ignored"]
fn exercise_2_weighted_expectation() {
    cases::exercise_2(weighted_expectation);
}

#[test]
#[ignore = "EXERCISE ONLY: implement td_target, then run with --ignored"]
fn exercise_3_td_target() {
    cases::exercise_3(td_target);
}

#[test]
#[ignore = "EXERCISE ONLY: implement is_probability_distribution, then run with --ignored"]
fn exercise_4_is_probability_distribution() {
    cases::exercise_4(is_probability_distribution);
}

#[test]
#[ignore = "EXERCISE ONLY: implement greedy_action, then run with --ignored"]
fn exercise_5_greedy_action() {
    cases::exercise_5(greedy_action);
}

#[test]
#[ignore = "EXERCISE ONLY: implement epsilon_greedy, then run with --ignored"]
fn exercise_6_epsilon_greedy() {
    cases::exercise_6(epsilon_greedy);
}

#[test]
#[ignore = "EXERCISE ONLY: implement update_mean, then run with --ignored"]
fn exercise_7_update_mean() {
    cases::exercise_7(update_mean);
}

#[test]
#[ignore = "EXERCISE ONLY: implement td_update, then run with --ignored"]
fn exercise_8_td_update() {
    cases::exercise_8(td_update);
}

#[test]
#[ignore = "EXERCISE ONLY: implement q_learning_target, then run with --ignored"]
fn exercise_9_q_learning_target() {
    cases::exercise_9(q_learning_target);
}

#[test]
#[ignore = "EXERCISE ONLY: implement bellman_backup, then run with --ignored"]
fn exercise_10_bellman_backup() {
    cases::exercise_10(bellman_backup);
}
