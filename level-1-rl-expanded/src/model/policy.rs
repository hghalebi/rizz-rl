//! Checkpoint 3: the policy, a dial that turns a state into action chances.
//!
//! Question: given a state, how likely is each action?
//! Example: in Engaged at boldness 0, Tease has chance 0.15 and Coffee has chance 0.
//! Predict: at boldness 1, what are the Tease and Coffee chances in Engaged?

use std::fmt;

use super::{Action, State};
use crate::probability::Distribution;

// ============================================================================
// CHECKPOINT 3 — THE POLICY: HOW THE AGENT BEHAVES
// ============================================================================
/// A stochastic policy: one dial, `boldness`, sets the action chances in every state.
/// `State ──π(a|s)──► Distribution<Action>`
/// Invariant: `boldness` is private, so every `Policy` comes from `try_new` and lies in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Policy {
    boldness: f64,
}

/// Returned by `Policy::try_new` when boldness is not a finite number in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InvalidBoldness;

impl fmt::Display for InvalidBoldness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("boldness must be finite and in [0, 1]")
    }
}

impl std::error::Error for InvalidBoldness {}

impl Policy {
    /// Build a policy, refusing any dial setting that would break the action probabilities.
    /// `boldness 1.5 in Playful: Coffee = 0.2 + 0.65 · 1.5 = 1.175, leaving -0.175 for the rest`
    /// Invariant: NaN, infinities, and values outside [0, 1] never become a `Policy`.
    /// Predict: what does boldness -0.5 do to Tease from Engaged?
    pub(crate) fn try_new(boldness: f64) -> Result<Self, InvalidBoldness> {
        if boldness.is_finite() && (0.0..=1.0).contains(&boldness) {
            Ok(Self { boldness })
        } else {
            Err(InvalidBoldness)
        }
    }

    /// The dial setting, read-only.
    pub(crate) fn boldness(&self) -> f64 {
        self.boldness
    }

    /// The action chances in one state; boldness never touches the world's response chances.
    /// ```text
    /// Engaged:  Tease = 0.15 + 0.35·b (linear),  Coffee = 0.30·b² (quadratic),
    ///           Ask and Story split what remains 60 : 40
    /// Playful:  Coffee = 0.20 + 0.65·b,  Ask and Tease split what remains 50 : 50
    /// ```
    /// Invariant: each state's chances sum to 1 for every boldness in [0, 1].
    /// Predict: at boldness 0, how much do Ask and Story share in Engaged?
    pub(crate) fn possible_actions(&self, state: &State) -> Distribution<Action> {
        let boldness = self.boldness;

        match state {
            State::JustMet => Distribution {
                outcomes: vec![(Action::Ask, 0.70), (Action::Story, 0.30)],
            },

            State::Engaged => {
                let tease_probability = 0.15 + 0.35 * boldness;
                let coffee_probability = 0.30 * boldness * boldness;
                let remaining_probability = 1.0 - tease_probability - coffee_probability;

                Distribution {
                    outcomes: vec![
                        (Action::Ask, 0.60 * remaining_probability),
                        (Action::Story, 0.40 * remaining_probability),
                        (Action::Tease, tease_probability),
                        (Action::Coffee, coffee_probability),
                    ],
                }
            }

            State::Playful => {
                let coffee_probability = 0.20 + 0.65 * boldness;
                let remaining_probability = 1.0 - coffee_probability;

                Distribution {
                    outcomes: vec![
                        (Action::Ask, 0.50 * remaining_probability),
                        (Action::Tease, 0.50 * remaining_probability),
                        (Action::Coffee, coffee_probability),
                    ],
                }
            }

            State::LowInterest => Distribution {
                outcomes: vec![(Action::Space, 0.80), (Action::Goodbye, 0.20)],
            },

            State::MutualInterest | State::Goodbye => Distribution::certain(Action::Goodbye),
        }
    }
}
