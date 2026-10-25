//! Checkpoint 4: the world model, which turns an action into possible consequences.
//!
//! Question: given a state and a chosen action, how likely is each next state?
//! Example: `Ask` from `JustMet` reaches `Engaged` with chance 0.80 and `LowInterest` with 0.20.
//! Predict: does changing boldness change either of those two chances?
//!
//! The reward for each consequence lives next door, in `model/reward.rs`.
//! Both halves are traits, `TransitionModel` and `RewardModel`: the P and R of an MDP.

use super::{Action, State};
use crate::probability::Distribution;

// ============================================================================
// CHECKPOINT 4 — THE WORLD MODEL: HOW CONSEQUENCES HAPPEN
// ============================================================================
/// The world's side of one step: which next states an action can lead to, and how likely each is.
/// `(State, Action) ──P(s'|s,a)──► Distribution<State>`
/// Invariant: these chances are conditional on the action; `possible_transitions` applies π(a|s).
#[derive(Debug, Clone, Copy)]
pub struct DatingWorld;

/// The transition half of a world model, `P(s'|s,a)`: what can happen next, and how likely.
/// `(state, action) ──► Distribution<State>`
/// Invariant: a known model like `DatingWorld` and a learned model answer this same question.
pub trait TransitionModel {
    /// The possible next states after taking `action` in `state`, each with its chance.
    fn possible_next_states(&self, state: State, action: Action) -> Distribution<State>;
}

impl TransitionModel for DatingWorld {
    /// The possible next states after taking `action` in `state`.
    /// `JustMet ──Ask──► Engaged @ 0.80 | LowInterest @ 0.20`
    /// Invariant: each arm sums to 1; an unsupported pair keeps the state, a teaching shortcut.
    fn possible_next_states(&self, state: State, action: Action) -> Distribution<State> {
        match (state, action) {
            (State::JustMet, Action::Ask) => Distribution {
                outcomes: vec![(State::Engaged, 0.80), (State::LowInterest, 0.20)],
            },

            (State::JustMet, Action::Story) => Distribution {
                outcomes: vec![(State::Engaged, 0.60), (State::LowInterest, 0.40)],
            },

            (State::Engaged, Action::Ask) => Distribution {
                outcomes: vec![
                    (State::Engaged, 0.65),
                    (State::Playful, 0.20),
                    (State::LowInterest, 0.15),
                ],
            },

            (State::Engaged, Action::Story) => Distribution {
                outcomes: vec![
                    (State::Engaged, 0.55),
                    (State::Playful, 0.30),
                    (State::LowInterest, 0.15),
                ],
            },

            (State::Engaged, Action::Tease) => Distribution {
                outcomes: vec![
                    (State::Playful, 0.55),
                    (State::Engaged, 0.25),
                    (State::LowInterest, 0.20),
                ],
            },

            (State::Engaged, Action::Coffee) => Distribution {
                outcomes: vec![(State::MutualInterest, 0.15), (State::LowInterest, 0.85)],
            },

            (State::Playful, Action::Ask) => Distribution {
                outcomes: vec![
                    (State::Playful, 0.55),
                    (State::Engaged, 0.35),
                    (State::LowInterest, 0.10),
                ],
            },

            (State::Playful, Action::Tease) => Distribution {
                outcomes: vec![
                    (State::Playful, 0.50),
                    (State::Engaged, 0.20),
                    (State::LowInterest, 0.30),
                ],
            },

            (State::Playful, Action::Coffee) => Distribution {
                outcomes: vec![(State::MutualInterest, 0.75), (State::LowInterest, 0.25)],
            },

            // `Space` and `Goodbye` end the conversation from any state that offers them.
            (_, Action::Space | Action::Goodbye) => Distribution::certain(State::Goodbye),

            // Teaching fallback: an unsupported pair keeps the state; a strict model would refuse.
            (state, _) => Distribution::certain(state),
        }
    }
}
