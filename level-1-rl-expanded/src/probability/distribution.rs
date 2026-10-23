//! Checkpoint 1: `Distribution`, a list of possible values with a chance beside each.
//!
//! Question: how do we write down several possible values, each with its own chance?
//! Example: Playful @ 0.55, Engaged @ 0.25, LowInterest @ 0.20 is one `Distribution<State>`.
//! Predict: what is `total_probability()` for that example, and for `certain(Goodbye)`?
//!
//! Its two composition operations, `map` and `and_then`, live in `composition.rs`.

// ============================================================================
// CHECKPOINT 1 — REPRESENTING UNCERTAINTY
// ============================================================================
// Distribution<T> means:
//
//     possible values of T
//     +
//     the probability of each value
//
// Example:
//
// Distribution<State>
// ├── Playful     @ 0.55
// ├── Engaged     @ 0.25
// └── LowInterest @ 0.20
//
// A row represents a branch, so two rows may contain the same value. This small
// teaching type does not validate its weights: callers must supply finite,
// nonnegative probabilities whose total is one before interpreting them as such.
#[derive(Debug, Clone)]
pub(crate) struct Distribution<T> {
    pub(crate) outcomes: Vec<(T, f64)>,
}

impl<T> Distribution<T> {
    // One certain value is represented as one branch with probability 1.0.
    pub(crate) fn certain(value: T) -> Self {
        Self {
            outcomes: vec![(value, 1.0)],
        }
    }

    // Count represented branches, not distinct values or observed samples.
    pub(crate) fn number_of_outcomes(&self) -> usize {
        self.outcomes.len()
    }

    // Probability mass should remain approximately 1.0 as the tree branches.
    pub(crate) fn total_probability(&self) -> f64 {
        self.outcomes
            .iter()
            .map(|(_, probability)| probability)
            .sum()
    }
}

impl<T: PartialEq> Distribution<T> {
    // Different branches can produce the same value. Sum all such branches.
    // For example, Ask and Story can both reach Engaged. Those alternative paths
    // contribute to one event; equality (`PartialEq`) tells us which rows belong.
    pub(crate) fn probability_of(&self, target: &T) -> f64 {
        self.outcomes
            .iter()
            .filter(|(value, _)| value == target)
            .map(|(_, probability)| probability)
            .sum()
    }
}
