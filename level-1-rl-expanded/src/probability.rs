//! Weighted alternatives, composition, and expectation.

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

    // `map` changes every value while preserving every branch probability.
    //
    //     T @ p  ----transform---->  U @ p
    pub(crate) fn map<U>(self, transform: impl Fn(T) -> U) -> Distribution<U> {
        let mut transformed_outcomes = Vec::with_capacity(self.outcomes.len());

        for (value, probability) in self.outcomes {
            let transformed_value = transform(value);
            transformed_outcomes.push((transformed_value, probability));
        }

        Distribution {
            outcomes: transformed_outcomes,
        }
    }

    // `and_then` continues every existing branch into more branches.
    // It is the same general operation often called `flat_map` or `bind`.
    //
    // Probabilities multiply because reaching a new leaf requires BOTH:
    //
    // 1. reaching the current branch;
    // 2. taking the next branch.
    pub(crate) fn and_then<U>(self, next_step: impl Fn(T) -> Distribution<U>) -> Distribution<U> {
        let mut combined_outcomes = Vec::new();

        for (current_value, probability_so_far) in self.outcomes {
            let possible_next_values = next_step(current_value);

            for (next_value, next_probability) in possible_next_values.outcomes {
                // P(whole path) = P(path so far) × P(next branch)
                let path_probability = probability_so_far * next_probability;

                combined_outcomes.push((next_value, path_probability));
            }
        }

        Distribution {
            outcomes: combined_outcomes,
        }
    }

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
    pub(crate) fn probability_of(&self, target: &T) -> f64 {
        self.outcomes
            .iter()
            .filter(|(value, _)| value == target)
            .map(|(_, probability)| probability)
            .sum()
    }
}

// ============================================================================
// CHECKPOINT 8 — EXPECTATION: COLLAPSE MANY FUTURES INTO ONE SCORE
// ============================================================================
// E[value] = Σ probability(outcome) × value(outcome)
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
