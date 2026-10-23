//! Checkpoint 5: one step of experience, recorded as plain data.
//!
//! Question: what is the smallest record of "something happened"?
//! Example: JustMet --Ask / +1--> Engaged is one Transition.
//! Predict: why does a Transition carry no probability of its own?

use super::{Action, State};

// ============================================================================
// CHECKPOINT 5 — TRANSITIONS AND TRAJECTORIES
// ============================================================================
// One transition:
//
//     from_state ----action / reward----> to_state
// Store all four pieces so we can score a history or learn from one observation.
// A Transition contains no branch probability: the exact planner stores that
// weight alongside it, while the sampled learner receives only what happened.
#[derive(Clone, Debug)]
pub(crate) struct Transition {
    pub(crate) from_state: State,
    pub(crate) action: Action,
    pub(crate) reward: f64,
    pub(crate) to_state: State,
}
