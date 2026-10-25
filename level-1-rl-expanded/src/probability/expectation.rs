//! Checkpoint 8: expectation, which collapses many possible futures into one score.
//!
//! Question: how do many weighted outcomes become one number we can compare?
//! Example: +1 with chance 0.74 and -2 with chance 0.26 give 0.74 - 0.52 = 0.22.
//! Predict: can any single outcome in that example equal 0.22?

use super::Distribution;

// ============================================================================
// CHECKPOINT 8 — EXPECTATION: COLLAPSE MANY FUTURES INTO ONE SCORE
// ============================================================================
/// Score every outcome, then average the scores using each outcome's own chance.
/// `E[value] = Σ probability(outcome) · value(outcome)`
/// Invariant: no division by the row count; the probabilities already sum to 1.
pub fn expected_value<T>(distribution: &Distribution<T>, value_of: impl Fn(&T) -> f64) -> f64 {
    distribution
        .outcomes
        .iter()
        .map(|(outcome, probability)| probability * value_of(outcome))
        .sum()
}
