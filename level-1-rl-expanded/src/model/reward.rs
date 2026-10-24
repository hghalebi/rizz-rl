//! Checkpoint 4, part 2: rewards, R(s, a, s'), say how valuable each consequence is.
//!
//! Question: how much is one step worth, given where it started and where it landed?
//! Example: Coffee from Engaged earns +8 on MutualInterest and -8 on LowInterest.
//! Predict: with chances 0.15 and 0.85, is Coffee's average immediate reward positive?

use super::{Action, DatingWorld, State};

impl DatingWorld {
    /// The reward for one step: how valuable it was, given where it started and where it landed.
    /// `R(s, a, s')`;  Coffee from Engaged averages `0.15 · 8 + 0.85 · (-8) = -5.6`
    /// Invariant: arms go from most specific to broadest, so a penalty wins over a general reward.
    pub(crate) fn reward_for(&self, state: &State, action: &Action, next_state: &State) -> f64 {
        match (state, action, next_state) {
            // Off-policy guard: the policy never picks this, but the reward model still prices it.
            (State::LowInterest, Action::Coffee, _) => -12.0,

            (State::Engaged, Action::Coffee, State::LowInterest) => -8.0,

            (_, _, State::MutualInterest) => 8.0,

            (State::Engaged, Action::Tease, State::Playful) => 3.0,

            (State::LowInterest, Action::Space | Action::Goodbye, State::Goodbye) => 2.0,

            (_, _, State::LowInterest) => -2.0,

            (_, Action::Ask | Action::Story, State::Playful | State::Engaged) => 1.0,

            _ => 0.0,
        }
    }
}
