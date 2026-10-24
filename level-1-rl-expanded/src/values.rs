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

/// The answer of one exact evaluation, plus how much work the cache did.
pub(super) struct ValueEstimate {
    pub(super) expected_return: f64,
    /// Distinct nonterminal (state, steps_remaining) subproblems evaluated; base cases do not count.
    pub(super) evaluated_states: usize,
}

/// Memo table keyed by the whole subproblem.
/// `(state, steps_remaining) ──► V`
/// Invariant: the same state with a different number of steps left is a different question.
type ValueCache = HashMap<(State, usize), f64>;

/// Evaluate a fixed policy from `initial_state` with a fresh cache.
/// `V(horizon, initial_state)`, computed by `value_for`
/// Invariant: a fresh cache per call, so another policy or discount cannot inherit answers.
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

/// The value of what comes after arriving in `state` with `steps_remaining` steps left.
/// `V(h, s) = Σ p · [r + γ · V(h-1, s')]`,  with  `V(0, s) = V(h, terminal) = 0`
/// Invariant: each (state, steps) subproblem is computed once; the cache answers every repeat.
/// Predict: does a zero continuation value also erase the immediate reward?
fn value_for(
    policy: &Policy,
    world: &DatingWorld,
    state: &State,
    steps_remaining: usize,
    discount: f64,
    cache: &mut ValueCache,
) -> f64 {
    if steps_remaining == 0 || state.is_terminal() {
        return 0.0;
    }

    let key = (*state, steps_remaining);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let expected_return = possible_transitions(policy, world, state)
        .outcomes
        .into_iter()
        .filter(|(_, probability)| *probability != 0.0)
        .map(|(transition, probability)| {
            // `map` takes `FnMut`, so this closure may reborrow `cache` mutably on every call.
            let future_value = value_for(
                policy,
                world,
                &transition.to_state,
                steps_remaining - 1,
                discount,
                cache,
            );
            probability * (transition.reward + discount * future_value)
        })
        .sum();

    cache.insert(key, expected_return);
    expected_return
}

#[cfg(test)]
mod tests;
