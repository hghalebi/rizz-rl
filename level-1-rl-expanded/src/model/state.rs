//! Checkpoint 2: the states the agent can be in and the actions it can take.
//!
//! Question: what situations can the agent be in, and what can it choose to do?
//! Example: from JustMet the agent can choose Ask; Goodbye is the only terminal state.
//! Predict: from which states can the next step still produce a reward?

// ============================================================================
// CHECKPOINT 2 — STATES AND ACTIONS
// ============================================================================
/// The situation the agent believes it is in: all the history needed to predict the next response.
/// `State ──policy──► Action ──world──► next State`
/// Invariant: the variants carry no data, so `Copy` makes passing a state as cheap as an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum State {
    JustMet,
    Engaged,
    Playful,
    LowInterest,
    MutualInterest,
    Goodbye,
}

impl State {
    /// Every state in one place, so tables and exhaustive checks stay in sync with the enum.
    pub(crate) const ALL: [Self; 6] = [
        Self::JustMet,
        Self::Engaged,
        Self::Playful,
        Self::LowInterest,
        Self::MutualInterest,
        Self::Goodbye,
    ];

    /// Whether the meaningful interaction has ended.
    /// `Goodbye ─► terminal`; MutualInterest still has one final Goodbye transition
    /// Invariant: the reward for arriving at Goodbye counts even though no future follows it.
    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self, Self::Goodbye)
    }
}

/// A choice available to the agent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Action {
    Ask,
    Story,
    Tease,
    Coffee,
    Space,
    Goodbye,
}
