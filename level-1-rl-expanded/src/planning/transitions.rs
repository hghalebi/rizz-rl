//! Join the policy and the world model into one weighted step.
//!
//! Question: how do the agent's choice and the world's response become one branch?
//! Example: P(Ask) = 0.70 times P(Engaged | Ask) = 0.80 gives the branch chance 0.56.
//! Predict: how many branches does JustMet produce? Count actions times next states.

use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

// Compose:
//
// State --policy--> Distribution<Action>
// Action --world--> Distribution<NextState>
//
// into:
//
// State -----------> Distribution<Transition>
//
// From JustMet, Ask has chance 0.70 and Engaged given Ask has chance 0.80.
// Their complete branch weighs 0.70 * 0.80 = 0.56; the second chance is conditional.
// Each possible consequence owns a Transition. State and Action are Copy, so
// every branch receives its own copy of the shared state and action.
// `*state` reads through the borrow `&State` and copies the value it points to.
// Predict: if Ask's chance halves and the remaining mass moves to Story, does
// the conditional chance of Engaged given Ask change?
pub(crate) fn possible_transitions(
    policy: &Policy,
    world: &DatingWorld,
    state: &State,
) -> Distribution<Transition> {
    let possible_actions = policy.possible_actions(state);

    possible_actions.and_then(|action| {
        let possible_next_states = world.possible_next_states(state, &action);

        possible_next_states.map(|next_state| {
            let transition_reward = world.reward_for(state, &action, &next_state);

            Transition {
                from_state: *state,
                action,
                reward: transition_reward,
                to_state: next_state,
            }
        })
    })
}
