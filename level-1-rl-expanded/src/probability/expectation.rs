//! Checkpoint 8: expectation, which collapses many possible futures into one score.
//!
//! Question: how do many weighted outcomes become one number we can compare?
//! Example: +1 with chance 0.74 and -2 with chance 0.26 give 0.74 - 0.52 = 0.22.
//! Predict: can any single outcome in that example equal 0.22?

use super::Distribution;

// ============================================================================
// CHECKPOINT 8 — EXPECTATION: COLLAPSE MANY FUTURES INTO ONE SCORE
// ============================================================================
// E[value] = Σ probability(outcome) × value(outcome)
// Read this as: score each possible outcome, then average using its chance.
// A reward of +1 with probability 0.74 and -2 with probability 0.26 has expectation
// 0.22. No single outcome needs to equal that average. Unlike a sample average,
// this loop uses the supplied probabilities, so it needs no division by row count.
// `&Distribution<T>` and `&T` borrow the data: scoring leaves it available for
// another question, such as the probability of a particular event.
pub(crate) fn expected_value<T>(
    distribution: &Distribution<T>,
    value_of: impl Fn(&T) -> f64,
) -> f64 {
    let mut weighted_sum = 0.0;

    for (outcome, probability) in &distribution.outcomes {
        weighted_sum += probability * value_of(outcome);
    }

    weighted_sum
}
