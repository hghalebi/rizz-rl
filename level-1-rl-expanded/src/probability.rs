//! Weighted alternatives, composition, and expectation.
//!
//! Picture a list of possible futures with a probability written beside each.
//! `map` changes what each future contains; `and_then` lets it branch again.
//! These operations keep all represented possibilities. Randomly choosing one
//! possibility is a separate operation in `experience/sampling.rs`.
//!
//! Read the files in this order:
//! 1. `probability/distribution.rs`: Checkpoint 1, the `Distribution` type itself.
//! 2. `probability/composition.rs`: `map` and `and_then`, which build new distributions.
//! 3. `probability/expectation.rs`: Checkpoint 8, one score from many weighted outcomes.

mod composition;
mod distribution;
mod expectation;

pub(crate) use distribution::Distribution;
pub(crate) use expectation::expected_value;
