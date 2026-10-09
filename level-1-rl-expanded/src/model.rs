//! The concrete states, actions, policy, and world model used by every experiment.
//!
//! Read one interaction as: state -> choose an action -> observe a consequence
//! -> receive a reward. The policy controls the choice; the world model describes
//! the response. Keeping these roles separate lets us compare behavior rules in
//! the same world. All numbers and conversation labels here are invented for
//! teaching, not measurements or advice about real people.

use crate::probability::Distribution;

// ============================================================================
// CHECKPOINT 2 — STATES AND ACTIONS
// ============================================================================
// State  = the situation the agent currently believes it is in.
// Action = a choice available to the agent.
// Our Markov assumption is that this state contains all the history needed to
// predict the next response given an action. A real task might need a richer state.
// Rust's enum gives these named cases a type, so a match can cover every variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    JustMet,
    Engaged,
    Playful,
    LowInterest,
    MutualInterest,
    Goodbye,
}

impl State {
    // A terminal state ends the meaningful interaction.
    // MutualInterest still has a final Goodbye transition in this model.
    // The reward for arriving at Goodbye can count even though no future follows.
    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self, Self::Goodbye)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    Ask,
    Story,
    Tease,
    Coffee,
    Space,
    Goodbye,
}

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
#[derive(Clone, Copy, Debug)]
pub(crate) struct Policy {
    pub(crate) boldness: f64,
}

impl Policy {
    pub(crate) fn possible_actions(&self, state: &State) -> Distribution<Action> {
        // The grid and validated CLI settings only construct values in [0, 1].
        // This debug assertion makes that modeling invariant visible.
        debug_assert!(
            self.boldness.is_finite() && (0.0..=1.0).contains(&self.boldness),
            "boldness must be a finite number in [0, 1]"
        );

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
            (state, _) => Distribution::certain(state.clone()),
        }
    }

    // Reward R(s, a, s') defines the objective the algorithm actually optimizes.
    // Probability answers "how likely?"; reward answers "how valuable?".
    // Here Coffee from Engaged has expected immediate reward
    // 0.15 * 8 + 0.85 * (-8) = -5.6, despite a positive best-case outcome.
    // Match order matters: specific penalties precede the broader reward rules.
    pub(crate) fn reward_for(&self, state: &State, action: &Action, next_state: &State) -> f64 {
        match (state, action, next_state) {
            // The current policy never chooses Coffee from LowInterest.
            // This defensive rule shows that a reward model may still define
            // the cost of off-policy or externally supplied actions.
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
