//! Checkpoint 10: evaluate the same policy without storing every possible history.
//!
//! Question: can we score a policy without listing every history first?
//! Example: rewards 0 now and 5 next are worth 2.5 at discount 0.5 with two steps left.
//! Predict: with only one step left, what is that same pair of rewards worth?
//!
//! If two histories reach the same state with the same number of steps left,
//! their remaining expected return is identical in this Markov model.
//! Cache that answer and reuse it: this is finite-horizon dynamic programming.
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

use std::collections::HashMap;

use crate::model::{DatingWorld, Policy, State};
use crate::planning::possible_transitions;

pub(super) struct ValueEstimate {
    pub(super) expected_return: f64,
    /// Distinct nonterminal (state, steps_remaining) subproblems evaluated.
    /// Zero-horizon and terminal base cases do not count.
    pub(super) evaluated_states: usize,
}

// The key is the whole subproblem: the same state with a different number of
// remaining steps asks a different question, so both parts identify the answer.
type ValueCache = HashMap<(State, usize), f64>;

/// Evaluate a fixed policy with a fresh cache. The caller supplies gamma in [0, 1].
/// Starting fresh prevents another policy or discount from inheriting these
/// answers. Recursive calls borrow this one cache instead of copying its entries.
pub(super) fn evaluate_value(
    policy: &Policy,
    world: &DatingWorld,
    initial_state: &State,
    horizon: usize,
    discount: f64,
) -> ValueEstimate {
    let mut cache = ValueCache::new();
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
    cache: &mut ValueCache,
) -> f64 {
    // This is the value of what comes AFTER arriving here. The caller still
    // counts the arriving reward. Horizon zero ends this finite objective,
    // even if the environment state itself is not terminal.
    if steps_remaining == 0 || state.is_terminal() {
        return 0.0;
    }

    let key = (*state, steps_remaining);
    if let Some(&cached) = cache.get(&key) {
        // An f64 is Copy: return the answer without taking its entry out of storage.
        return cached;
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

    // The key holds a copy of the state, so the cache does not borrow from the caller.
    cache.insert(key, expected_return);
    expected_return
}

#[cfg(test)]
mod tests;
