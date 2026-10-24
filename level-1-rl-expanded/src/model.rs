//! The concrete states, actions, policy, and world model used by every experiment.
//!
//! Read one interaction as: state -> choose an action -> observe a consequence
//! -> receive a reward. The policy controls the choice; the world model describes
//! the response. Keeping these roles separate lets us compare behavior rules in
//! the same world. All numbers and conversation labels here are invented for
//! teaching, not measurements or advice about real people.
//!
//! Read the files in this order, one checkpoint each:
//! 1. `model/state.rs`: Checkpoint 2, the states and actions.
//! 2. `model/policy.rs`: Checkpoint 3, how the agent chooses an action.
//! 3. `model/world.rs`: Checkpoint 4, how the world responds to an action.
//! 4. `model/reward.rs`: Checkpoint 4, part 2, how valuable each response is.
//! 5. `model/transition.rs`: Checkpoint 5, one step recorded as data.

mod policy;
mod reward;
mod state;
mod transition;
mod world;

#[cfg(test)]
pub(crate) use policy::InvalidBoldness;
pub(crate) use policy::Policy;
pub(crate) use state::{Action, State};
pub(crate) use transition::Transition;
pub(crate) use world::DatingWorld;
