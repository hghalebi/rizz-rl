//! Composition: `map` changes each value; `and_then` lets each value branch again.
//!
//! Question: how do we transform or extend every outcome without losing probability?
//! Example: "ask" @ 0.7, then "engaged" given ask @ 0.8, is one path with chance 0.56.
//! Predict: does `map` change the total probability? Does `and_then`?

use super::Distribution;

impl<T> Distribution<T> {
    /// Transform every value and keep every branch probability.
    /// `T @ p ──transform──► U @ p`
    /// Invariant: the list of probabilities is untouched, so the total mass is too.
    pub(crate) fn map<U>(self, transform: impl Fn(T) -> U) -> Distribution<U> {
        Distribution {
            // `into_iter` hands the closure each owned `T`, so a `String` is moved, not cloned.
            outcomes: self
                .outcomes
                .into_iter()
                .map(|(value, probability)| (transform(value), probability))
                .collect(),
        }
    }

    /// Continue every branch into its own sub-branches; also called `flat_map` or `bind`.
    /// `T @ p ──next_step──► U @ q   gives   U @ p·q`
    /// Invariant: if the input and every continuation sum to 1, so does the result.
    /// Predict: what happens to a weight of 0.2 when its continuation splits evenly?
    pub(crate) fn and_then<U>(self, next_step: impl Fn(T) -> Distribution<U>) -> Distribution<U> {
        Distribution {
            outcomes: self
                .outcomes
                .into_iter()
                .flat_map(|(current_value, probability_so_far)| {
                    // `move` copies `probability_so_far` into the inner closure, which outlives this one.
                    next_step(current_value).outcomes.into_iter().map(
                        move |(next_value, next_probability)| {
                            let path_probability = probability_so_far * next_probability;
                            (next_value, path_probability)
                        },
                    )
                })
                .collect(),
        }
    }
}
