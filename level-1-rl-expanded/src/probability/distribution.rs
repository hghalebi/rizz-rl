//! Checkpoint 1: `Distribution`, a list of possible values with a chance beside each.
//!
//! Question: how do we write down several possible values, each with its own chance?
//! Example: `Playful` @ 0.55, `Engaged` @ 0.25, `LowInterest` @ 0.20 is one `Distribution<State>`.
//! Predict: what is `total_probability()` for that example, and for `certain(Goodbye)`?
//!
//! Its two composition operations, `map` and `and_then`, live in `composition.rs`.

// ============================================================================
// CHECKPOINT 1 — REPRESENTING UNCERTAINTY
// ============================================================================
/// Several possible values of `T`, each paired with its probability.
/// ```text
/// Distribution<State>
/// ├── Playful     @ 0.55
/// ├── Engaged     @ 0.25
/// └── LowInterest @ 0.20
/// ```
/// Invariant: callers supply finite, nonnegative weights that sum to 1; this type does not check.
#[derive(Debug, Clone)]
pub struct Distribution<T> {
    /// One row per branch, so two rows may hold the same value.
    pub(crate) outcomes: Vec<(T, f64)>,
}

impl<T> Distribution<T> {
    /// One certain value: a single branch with probability 1.
    pub(crate) fn certain(value: T) -> Self {
        Self {
            outcomes: vec![(value, 1.0)],
        }
    }

    /// How many branches are represented, not how many distinct values.
    pub(crate) const fn number_of_outcomes(&self) -> usize {
        self.outcomes.len()
    }

    /// The total probability mass, which should stay close to 1 as the tree branches.
    /// `Σ probability`
    pub(crate) fn total_probability(&self) -> f64 {
        self.outcomes
            .iter()
            .map(|(_, probability)| probability)
            .sum()
    }
}

impl<T: PartialEq> Distribution<T> {
    /// The probability of one event, summed over every branch that produces it.
    /// `P(target) = Σ probability  over rows where value == target`
    /// Invariant: `Ask` and `Story` can both reach `Engaged`; equality decides which rows belong.
    pub(crate) fn probability_of(&self, target: &T) -> f64 {
        self.outcomes
            .iter()
            .filter(|(value, _)| value == target)
            .map(|(_, probability)| probability)
            .sum()
    }
}
