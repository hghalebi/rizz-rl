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
// A stochastic policy maps a state to a probability distribution over actions:
//
//     State ----policy π(a|s)----> Distribution<Action>
//
// `boldness` parameterizes the entire policy.
// Think of it as a dial for a behavior rule, not an action taken on one step.
// Changing this dial leaves the world's conditional response probabilities alone.
//
// Why is `boldness` private?
//   Outside this file, `Policy { boldness: 1.5 }` does not compile.
//   The only way to build a Policy is `Policy::try_new`, which checks the range.
//   So every Policy you receive already has boldness in [0, 1].
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
    // Why check at all? An f64 can hold values that make no sense as a dial:
    //   NaN, infinity, or 1.5.
    // Example: boldness 1.5 in Playful gives Coffee 0.2 + 0.65 * 1.5 = 1.175.
    // That leaves 1 - 1.175 = -0.175 for Ask and Tease: a negative probability.
    // Predict: what does boldness -0.5 do to Tease from Engaged?
    pub(crate) fn try_new(boldness: f64) -> Result<Self, InvalidBoldness> {
        if boldness.is_finite() && (0.0..=1.0).contains(&boldness) {
            Ok(Self { boldness })
        } else {
            Err(InvalidBoldness)
        }
    }

    pub(crate) fn boldness(&self) -> f64 {
        self.boldness
    }

    pub(crate) fn possible_actions(&self, state: &State) -> Distribution<Action> {
        // No range check needed here: try_new already guaranteed [0, 1].
        let boldness = self.boldness;

        match state {
            State::JustMet => Distribution {
                outcomes: vec![(Action::Ask, 0.70), (Action::Story, 0.30)],
            },

            State::Engaged => {
                // Linear growth: each boldness increment adds the same amount.
                let tease_probability = 0.15 + 0.35 * boldness;

                // Quadratic growth: Coffee remains low at first, then rises faster.
                let coffee_probability = 0.30 * boldness * boldness;

                // Ask and Story divide the probability that remains.
                // A useful budget check: at boldness 0, Tease takes 0.15 and
                // Coffee takes 0; Ask and Story must share the remaining 0.85.
                // Predict how raising boldness changes that remaining budget.
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
