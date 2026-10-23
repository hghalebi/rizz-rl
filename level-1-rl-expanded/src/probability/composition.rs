//! Composition: `map` changes each value; `and_then` lets each value branch again.
//!
//! Question: how do we transform or extend every outcome without losing probability?
//! Example: "ask" @ 0.7, then "engaged" given ask @ 0.8, is one path with chance 0.56.
//! Predict: does `map` change the total probability? Does `and_then`?

use super::Distribution;

impl<T> Distribution<T> {
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
}
