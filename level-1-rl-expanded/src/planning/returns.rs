//! Checkpoint 7: score one history by discounting its later rewards.
//!
//! Question: what is one history worth when later rewards count for less?
//! Example: rewards [1, 2, 4] at discount 0.5 give 1 + 1 + 1 = 3.
//! Predict: what is left at discount 0? What changes at discount 1?
//!
//! New to iterators? `guides/LEARNING_PATH.md#loop-to-pipeline` maps each stage to a column.

use std::iter::successors;

use crate::model::Transition;

// ============================================================================
// CHECKPOINT 7 — DISCOUNTED RETURN: SCORE ONE TRAJECTORY
// ============================================================================
/// The weight of each reward: one more factor of `discount` for every step into the future.
/// `1, γ, γ², γ³, ...`
/// Invariant: the first weight is always 1, so the first reward is never discounted.
pub(crate) fn discount_weights(discount: f64) -> impl Iterator<Item = f64> {
    // `move` copies `discount` into the closure, so the iterator can outlive this call.
    successors(Some(1.0), move |weight| Some(weight * discount))
}

/// Score one history: each reward times its weight, added up.
/// `G = r₀ + γ·r₁ + γ²·r₂ + ...`, so `[1, 2, 4]` at γ = 0.5 gives `1 + 1 + 1 = 3`
/// Invariant: a history's chance plays no part here; `expected_value` applies it later.
/// Predict: would swapping the first and last rewards preserve this return?
pub(crate) fn discounted_return(trajectory: &[Transition], discount: f64) -> f64 {
    discount_weights(discount)
        .zip(trajectory)
        .map(|(weight, transition)| weight * transition.reward)
        // Start from an explicit 0.0: an empty `sum()` is -0.0, which would print as -0.000000.
        .fold(0.0, |total, term| total + term)
}
