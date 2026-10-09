//! Exact trajectory evaluation and search across the fixed policy grid.
//!
//! Ask two separate questions: what does one possible history earn, and how
//! good is the policy across all histories? The data flow keeps them separate:
//! possible histories -> one return per history -> probability-weighted score.

use tracing::{debug, debug_span, info, info_span, warn};

use crate::model::{Action, DatingWorld, Policy, State, Transition};
use crate::probability::{expected_value, Distribution};

// A trajectory is one complete possible history.
pub(crate) type Trajectory = Vec<Transition>;

// During tree expansion, each leaf needs its current state and its history.
#[derive(Clone, Debug)]
struct ConversationPath {
    current_state: State,
    trajectory_so_far: Trajectory,
}

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
// Each possible consequence owns a Transition, so the shared action is cloned.
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
                from_state: state.clone(),
                action: action.clone(),
                reward: transition_reward,
                to_state: next_state,
            }
        })
    })
}

fn start_from(initial_state: &State) -> Distribution<ConversationPath> {
    Distribution::certain(ConversationPath {
        current_state: initial_state.clone(),
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
        let next_state = transition.to_state.clone();

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
        let step_span = debug_span!("expand_future_tree", step = step_number);
        let _step_guard = step_span.enter();

        possible_paths = possible_paths.and_then(|path| extend_one_step(policy, world, path));

        debug!(
            number_of_paths = possible_paths.number_of_outcomes(),
            probability_mass = possible_paths.total_probability(),
            "one rollout layer completed"
        );
    }

    possible_paths.map(|path| path.trajectory_so_far)
}

// ============================================================================
// CHECKPOINT 7 — DISCOUNTED RETURN: SCORE ONE TRAJECTORY
// ============================================================================
// G = r₀ + discount*r₁ + discount²*r₂ + ...
/// Score consecutive rewards along one history, before considering its chance.
/// With rewards [1, 2, 4] and discount 0.5: 1 + 0.5*2 + 0.25*4 = 3.
/// Borrowing the trajectory lets us score it without consuming its transitions.
/// Predict: would swapping the first and last rewards preserve this return?
pub(crate) fn discounted_return(trajectory: &Trajectory, discount: f64) -> f64 {
    let mut total_return = 0.0;
    let mut current_discount_weight = 1.0;

    for transition in trajectory {
        let discounted_reward = current_discount_weight * transition.reward;

        total_return += discounted_reward;
        current_discount_weight *= discount;
    }

    total_return
}

/// Evaluate a fixed policy from JustMet by averaging its possible returns.
/// Suppose two histories return 2 and 6 with probabilities 0.75 and 0.25:
/// their expected return is 3, even though neither history earns exactly 3.
/// The closure borrows each history; time discounting stays inside that history,
/// while expected_value applies its probability weight outside it.
/// Predict: could a guaranteed return of 3 hide a different risk from this pair?
pub(crate) fn expected_return_of(
    policy: &Policy,
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> f64 {
    let evaluation_span = debug_span!(
        "evaluate_policy",
        boldness = policy.boldness,
        horizon,
        discount,
    );
    let _evaluation_guard = evaluation_span.enter();

    let possible_trajectories = enumerate_trajectories(policy, world, &State::JustMet, horizon);

    let probability_mass = possible_trajectories.total_probability();

    // Invariant: branching redistributes the original mass; it must not create
    // or destroy probability.
    if (probability_mass - 1.0).abs() > 1e-9 {
        warn!(
            probability_mass,
            "trajectory probability mass is not conserved"
        );
    }

    let expected_return = expected_value(&possible_trajectories, |trajectory| {
        discounted_return(trajectory, discount)
    });

    debug!(
        number_of_trajectories = possible_trajectories.number_of_outcomes(),
        probability_mass, expected_return, "policy evaluation completed"
    );

    expected_return
}

// ============================================================================
// CHECKPOINT 9 — POLICY SEARCH: COMPARE CANDIDATE BEHAVIORS
// ============================================================================
#[derive(Clone, Copy, Debug)]
pub(crate) struct PolicyEvaluation {
    pub(crate) policy: Policy,
    pub(crate) expected_return: f64,
}

/// Compare eleven behavior rules under the same world, horizon, and discount.
/// Only boldness changes; the search does not learn new world probabilities.
/// Predict: if 0.3 wins this grid, what do we know about the untested value 0.34?
pub(crate) fn evaluate_policy_grid(
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> Vec<PolicyEvaluation> {
    let search_span = info_span!("policy_search", horizon, discount);
    let _search_guard = search_span.enter();

    let mut evaluations = Vec::with_capacity(11);

    // Grid search over 0.0, 0.1, 0.2, ... 1.0.
    for index in 0..=10 {
        let candidate_policy = Policy {
            boldness: index as f64 / 10.0,
        };

        // Make the parameter concrete: show how it changes behavior in Engaged.
        let engaged_actions = candidate_policy.possible_actions(&State::Engaged);

        debug!(
            boldness = candidate_policy.boldness,
            ask = engaged_actions.probability_of(&Action::Ask),
            story = engaged_actions.probability_of(&Action::Story),
            tease = engaged_actions.probability_of(&Action::Tease),
            coffee = engaged_actions.probability_of(&Action::Coffee),
            "candidate action probabilities from Engaged"
        );

        let candidate_expected_return =
            expected_return_of(&candidate_policy, world, horizon, discount);

        info!(
            boldness = candidate_policy.boldness,
            expected_return = candidate_expected_return,
            "candidate evaluated"
        );

        evaluations.push(PolicyEvaluation {
            policy: candidate_policy,
            expected_return: candidate_expected_return,
        });
    }

    evaluations
}

pub(crate) fn best_policy(evaluations: &[PolicyEvaluation]) -> Option<PolicyEvaluation> {
    let mut best_evaluation: Option<PolicyEvaluation> = None;
    // Exact arithmetic would tie all candidates when gamma = 0. Summing many
    // floating-point branches can leave tiny differences. Treat differences
    // below this absolute tolerance as ties and retain the first candidate.
    for candidate in evaluations {
        let improves_best = match best_evaluation {
            Some(current) => candidate.expected_return > current.expected_return + 1e-12,
            None => true,
        };
        if improves_best {
            best_evaluation = Some(*candidate);
        }
    }
    best_evaluation
}

#[cfg(test)]
pub(crate) fn find_best_policy(
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> Option<PolicyEvaluation> {
    best_policy(&evaluate_policy_grid(world, horizon, discount))
}
