//! Join the policy and the world model into one weighted step.
//!
//! Question: how do the agent's choice and the world's response become one branch?
//! Example: P(`Ask`) = 0.70 times P(`Engaged` | `Ask`) = 0.80 gives the branch chance 0.56.
//! Predict: how many branches does `JustMet` produce? Count actions times next states.

use crate::model::{DatingWorld, Policy, RewardModel, State, Transition, TransitionModel};
use crate::probability::Distribution;

/// Join the agent's choice and the world's response into one weighted step.
/// ```text
/// State ──policy──► Distribution<Action> ──world──► Distribution<next State>
///   collapses to   State ──► Distribution<Transition>,  P(a, s'|s) = π(a|s) · P(s'|s, a)
/// ```
/// Invariant: `and_then` multiplies the chances: `Ask` 0.70 · `Engaged` given `Ask` 0.80 = 0.56.
/// Predict: if `Ask`'s chance halves and `Story` gets the rest, does P(`Engaged` | `Ask`) change?
pub fn possible_transitions(
    policy: Policy,
    world: DatingWorld,
    state: State,
) -> Distribution<Transition> {
    let possible_actions = policy.possible_actions(state);

    possible_actions.and_then(|action| {
        let possible_next_states = world.possible_next_states(state, action);

        possible_next_states.map(|next_state| {
            let transition_reward = world.reward_for(state, action, next_state);

            Transition {
                from_state: state,
                action,
                reward: transition_reward,
                to_state: next_state,
            }
        })
    })
}
