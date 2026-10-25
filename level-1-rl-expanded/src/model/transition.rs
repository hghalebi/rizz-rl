//! Checkpoint 5: one step of experience, recorded as plain data.
//!
//! Question: what is the smallest record of "something happened"?
//! Example: `JustMet` --`Ask` / +1--> `Engaged` is one Transition.
//! Predict: why does a Transition carry no probability of its own?

use super::{Action, State};

// ============================================================================
// CHECKPOINT 5 — TRANSITIONS AND TRAJECTORIES
// ============================================================================
/// One recorded step: where it started, what was chosen, what it earned, where it landed.
/// `from_state ──action / reward──► to_state`
/// Invariant: no probability inside; the planner stores that beside it, the learner never sees it.
#[derive(Clone, Debug)]
pub struct Transition {
    pub(crate) from_state: State,
    pub(crate) action: Action,
    pub(crate) reward: f64,
    pub(crate) to_state: State,
}
