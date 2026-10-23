//! Checkpoint 4: the world model, which turns an action into possible consequences.
//!
//! Question: given a state and a chosen action, how likely is each next state?
//! Example: Ask from JustMet reaches Engaged with chance 0.80 and LowInterest with 0.20.
//! Predict: does changing boldness change either of those two chances?
//!
//! The reward for each consequence lives next door, in `model/reward.rs`.

use super::{Action, State};
use crate::probability::Distribution;

// ============================================================================
// CHECKPOINT 4 — THE WORLD MODEL: HOW CONSEQUENCES HAPPEN
// ============================================================================
// The policy controls action probabilities.
// The world controls consequence probabilities.
//
//     (State, Action) ----P(s'|s,a)----> Distribution<NextState>
// For example, an Ask from JustMet reaches Engaged with probability 0.80.
// That is conditional on having chosen Ask; the policy's chance of choosing
// Ask is applied later, when `planning::possible_transitions` joins the stages.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DatingWorld;

impl DatingWorld {
    pub(crate) fn possible_next_states(
        &self,
        state: &State,
        action: &Action,
    ) -> Distribution<State> {
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

            (State::LowInterest, Action::Space | Action::Goodbye) => {
                Distribution::certain(State::Goodbye)
            }

            (State::MutualInterest, Action::Goodbye) => Distribution::certain(State::Goodbye),

            (_, Action::Space | Action::Goodbye) => Distribution::certain(State::Goodbye),

            // Pedagogical fallback: unsupported actions leave the state unchanged.
            // A stricter production model should return an error or make illegal
            // state/action combinations impossible to construct.
            (state, _) => Distribution::certain(*state),
        }
    }
}
