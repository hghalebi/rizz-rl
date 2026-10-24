//! Checkpoint 6: grow every possible history, one step at a time.
//!
//! Question: what are all the histories that can unfold within h steps?
//! Example: horizon 0 is one empty history; each step replaces every leaf with its continuations.
//! Predict: does a history that has already reached Goodbye keep growing?

use tracing::{debug, debug_span};

use super::possible_transitions;
use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

/// One complete possible history: its transitions, in the order they happened.
pub(crate) type Trajectory = Vec<Transition>;

/// A leaf of the growing tree: where this history is now, and how it got there.
#[derive(Clone, Debug)]
struct ConversationPath {
    current_state: State,
    trajectory_so_far: Trajectory,
}

/// The tree before any step: one empty history with probability 1.
fn start_from(initial_state: &State) -> Distribution<ConversationPath> {
    Distribution::certain(ConversationPath {
        current_state: *initial_state,
        trajectory_so_far: Vec::new(),
    })
}

/// Replace one leaf by all of its possible next leaves.
/// ```text
/// shared past ──┬── shared past + transition A   @ p_A
///               └── shared past + transition B   @ p_B
/// ```
/// Invariant: a history that has reached Goodbye is kept as it is, its final reward included.
/// Predict: must the other, still-active histories stop at the same time?
fn extend_one_step(
    policy: &Policy,
    world: &DatingWorld,
    path: ConversationPath,
) -> Distribution<ConversationPath> {
    if path.current_state.is_terminal() {
        return Distribution::certain(path);
    }

    let current_state = path.current_state;
    let trajectory_so_far = path.trajectory_so_far;

    possible_transitions(policy, world, &current_state).map(|transition| {
        let next_state = transition.to_state;

        // Each branch clones the shared past; moving it into one child leaves none for the others.
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
/// Grow the complete tree of histories, one layer per step: exact enumeration, not sampling.
/// `layer₀ = {empty history @ 1}`,  `layerₕ = layerₕ₋₁.and_then(extend_one_step)`
/// Invariant: every layer still has total probability 1; branching only redistributes it.
pub(crate) fn enumerate_trajectories(
    policy: &Policy,
    world: &DatingWorld,
    initial_state: &State,
    horizon: usize,
) -> Distribution<Trajectory> {
    (1..=horizon)
        .fold(start_from(initial_state), |possible_paths, step_number| {
            let _step_span = debug_span!("expand_future_tree", step = step_number).entered();

            let expanded = possible_paths.and_then(|path| extend_one_step(policy, world, path));

            debug!(
                number_of_paths = expanded.number_of_outcomes(),
                probability_mass = expanded.total_probability(),
                "one rollout layer completed"
            );
            expanded
        })
        .map(|path| path.trajectory_so_far)
}
