//! Weighted alternatives, composition, and expectation.
//!
//! Picture a list of possible futures with a probability written beside each.
//! `map` changes what each future contains; `and_then` lets it branch again.
//! These operations keep all represented possibilities. Randomly choosing one
//! possibility is a separate operation in `experience.rs`.

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

    // `map` changes every value while preserving every branch probability.
    //
    //     T @ p  ----transform---->  U @ p
    //
    // Example: ASCII labels "cat" @ 0.7 and "rust" @ 0.3 become byte lengths
    // 3 @ 0.7 and 4 @ 0.3. The branch weights stay; only their values change.
    // Taking `self` gives the loop ownership of each T, so it can consume a
    // String without cloning it. After this call, use the returned distribution.
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
    //
    // Ask has probability 0.7; Engaged conditional on Ask has probability 0.8.
    // Their complete branch has weight 0.7 * 0.8 = 0.56. This uses a conditional
    // probability; it does not assume the world's response is independent of Ask.
    // Total mass stays one if the input and every continuation are normalized.
    // Predict: what happens to a weight of 0.2 when its continuation splits evenly?
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
