//! Checkpoint 2: the states the agent can be in and the actions it can take.
//!
//! Question: what situations can the agent be in, and what can it choose to do?
//! Example: from JustMet the agent can choose Ask; Goodbye is the only terminal state.
//! Predict: from which states can the next step still produce a reward?

// ============================================================================
// CHECKPOINT 2 — STATES AND ACTIONS
// ============================================================================
// State  = the situation the agent currently believes it is in.
// Action = a choice available to the agent.
// Our Markov assumption is that this state contains all the history needed to
// predict the next response given an action. A real task might need a richer state.
// Rust's enum gives these named cases a type, so a match can cover every variant.
// The variants carry no data, so `Copy` makes passing a state as cheap as an integer.
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
    // One list of every state keeps tables and exhaustive checks in sync with the enum.
    pub(crate) const ALL: [Self; 6] = [
        Self::JustMet,
        Self::Engaged,
        Self::Playful,
        Self::LowInterest,
        Self::MutualInterest,
        Self::Goodbye,
    ];

    // A terminal state ends the meaningful interaction.
    // MutualInterest still has a final Goodbye transition in this model.
    // The reward for arriving at Goodbye can count even though no future follows.
    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self, Self::Goodbye)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Action {
    Ask,
    Story,
    Tease,
    Coffee,
    Space,
    Goodbye,
}
