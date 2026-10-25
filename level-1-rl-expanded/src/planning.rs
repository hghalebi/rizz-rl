//! Exact trajectory evaluation and search across the fixed policy grid.
//!
//! `Ask` two separate questions: what does one possible history earn, and how
//! good is the policy across all histories? The data flow keeps them separate:
//! possible histories -> one return per history -> probability-weighted score.
//!
//! Read the files in this order:
//! 1. `planning/transitions.rs`: join the policy and the world into one step.
//! 2. `planning/rollout.rs`: Checkpoint 6, grow every possible history.
//! 3. `planning/returns.rs`: Checkpoint 7, score one history.
//! 4. `planning/evaluation.rs`: average those scores into one policy value.
//! 5. `planning/search.rs`: Checkpoint 9, compare eleven policies.

mod evaluation;
mod returns;
mod rollout;
mod search;
mod transitions;

pub use evaluation::expected_return_of;
pub use returns::{discount_weights, discounted_return};
pub use rollout::{enumerate_trajectories, Trajectory};
#[cfg(test)]
pub use search::find_best_policy;
pub use search::{best_policy, evaluate_policy_grid};
pub use transitions::possible_transitions;
