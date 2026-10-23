//! Checkpoint 6: grow every possible history, one step at a time.
//!
//! Question: what are all the histories that can unfold within h steps?
//! Example: horizon 0 is one empty history; each step replaces every leaf with its continuations.
//! Predict: does a history that has already reached Goodbye keep growing?

use tracing::{debug, debug_span};

use super::possible_transitions;
use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

// A trajectory is one complete possible history.
pub(crate) type Trajectory = Vec<Transition>;

// During tree expansion, each leaf needs its current state and its history.
#[derive(Clone, Debug)]
struct ConversationPath {
    current_state: State,
    trajectory_so_far: Trajectory,
}

fn start_from(initial_state: &State) -> Distribution<ConversationPath> {
    Distribution::certain(ConversationPath {
        current_state: *initial_state,
        trajectory_so_far: Vec::new(),
    })
}

// Extend one leaf into all of its possible next leaves.
fn extend_one_step(
    policy: &Policy,
    world: &DatingWorld,
    path: ConversationPath,
) -> Distribution<ConversationPath> {
    // The reward for arriving at Goodbye is already in this history. Keeping
    // the stopped path preserves that reward and adds no further transitions.
    // Predict: must the other, still-active histories stop at the same time?
    if path.current_state.is_terminal() {
        return Distribution::certain(path);
    }

    let current_state = path.current_state;
    let trajectory_so_far = path.trajectory_so_far;

    possible_transitions(policy, world, &current_state).map(|transition| {
        let next_state = transition.to_state;

        // Moving the history into one child would leave none for the others.
        // Each branch copies the common past, then appends its own transition.
        let mut extended_trajectory = trajectory_so_far.clone();
        extended_trajectory.push(transition);

        ConversationPath {
            current_state: next_state,
            trajectory_so_far: extended_trajectory,
        }
    })
}

// ============================================================================
// CHECKPOINT 6 — ROLLOUT: GROW THE COMPLETE FUTURE TREE
// ============================================================================
// Horizon = how many interaction steps we inspect.
//
// This is exact enumeration, not Monte Carlo sampling.
// At horizon zero, the one empty history represents "nothing has happened yet".
// Consuming each layer lets and_then replace it with all its owned continuations.
pub(crate) fn enumerate_trajectories(
    policy: &Policy,
    world: &DatingWorld,
    initial_state: &State,
    horizon: usize,
) -> Distribution<Trajectory> {
    let mut possible_paths = start_from(initial_state);

    for step_number in 1..=horizon {
        let _step_span = debug_span!("expand_future_tree", step = step_number).entered();

        possible_paths = possible_paths.and_then(|path| extend_one_step(policy, world, path));

        debug!(
            number_of_paths = possible_paths.number_of_outcomes(),
            probability_mass = possible_paths.total_probability(),
            "one rollout layer completed"
        );
    }

    possible_paths.map(|path| path.trajectory_so_far)
}
