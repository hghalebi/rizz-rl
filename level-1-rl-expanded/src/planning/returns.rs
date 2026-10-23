//! Checkpoint 7: score one history by discounting its later rewards.
//!
//! Question: what is one history worth when later rewards count for less?
//! Example: rewards [1, 2, 4] at discount 0.5 give 1 + 1 + 1 = 3.
//! Predict: what is left at discount 0? What changes at discount 1?

use crate::model::Transition;

// ============================================================================
// CHECKPOINT 7 — DISCOUNTED RETURN: SCORE ONE TRAJECTORY
// ============================================================================
// G = r₀ + discount*r₁ + discount²*r₂ + ...
/// Score consecutive rewards along one history, before considering its chance.
/// With rewards [1, 2, 4] and discount 0.5: 1 + 0.5*2 + 0.25*4 = 3.
/// Borrowing a slice lets us score a whole trajectory, or any prefix of one,
/// without consuming or copying its transitions.
/// Predict: would swapping the first and last rewards preserve this return?
pub(crate) fn discounted_return(trajectory: &[Transition], discount: f64) -> f64 {
    let mut total_return = 0.0;
    let mut current_discount_weight = 1.0;

    for transition in trajectory {
        let discounted_reward = current_discount_weight * transition.reward;

        total_return += discounted_reward;
        current_discount_weight *= discount;
    }

    total_return
}
