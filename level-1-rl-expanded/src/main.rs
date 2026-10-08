#![allow(dead_code)]

//! # Level 1.5 — A visible reinforcement-learning system
//!
//! This program is deliberately small enough to simulate mentally.
//!
//! It demonstrates:
//!
//! - a finite Markov decision process;
//! - a stochastic, parameterized policy;
//! - a known transition model;
//! - exact finite-horizon trajectory enumeration;
//! - discounted return;
//! - exact policy evaluation;
//! - memoized Bellman policy evaluation in `values.rs`;
//! - grid-search policy optimization.
//!
//! It does **not** yet learn from experience. The transition probabilities and
//! rewards are written by the programmer. This makes it a model-based planning
//! and policy-evaluation example, the conceptual ground underneath later
//! techniques such as Monte Carlo, temporal-difference
//! learning, Q-learning, policy gradients, actor-critic, and Dyna.
//!
//! Pedagogical rule:
//!
//! ```text
//! domain meaning first
//!     ↓
//! mechanism second
//!     ↓
//! formal RL vocabulary third
//! ```
//!
use tracing::{debug, debug_span, info, info_span, warn, Level};

mod learning;
mod values;

// ============================================================================
// CHECKPOINT 0 — OBSERVABILITY
// ============================================================================
// Tracing lets us watch the algorithm without changing its decisions.
fn init_tracing(verbose: bool) {
    tracing_subscriber::fmt()
        .with_max_level(if verbose { Level::DEBUG } else { Level::WARN })
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .compact()
        .init();
}

fn print_lesson_map() {
    println!(
        r#"
================================================================
LEVEL 1.5: A visible model-based RL planner
================================================================

Current state s
     |
     | policy pi(a | s)
     v
Possible actions
     |
     | world model P(s' | s, a)
     v
Possible next states s'
     |
     | reward R(s, a, s')
     v
Possible transitions
     |
     | repeat for a finite horizon H
     v
Possible trajectories
     |
     | discounted return G
     v
One score per trajectory
     |
     | expectation E[G]
     v
One score for the policy
     |
     | grid search over boldness
     v
Best candidate policy

THIS PROGRAM IS:
- finite-horizon;
- model-based;
- exact rather than sampled;
- policy evaluation plus simple policy search.

THIS PROGRAM IS NOT YET:
- learning the world model from data;
- Monte Carlo estimation;
- temporal-difference learning;
- Q-learning;
- policy-gradient or actor-critic training.

Important: no random future is sampled.
Every represented future is enumerated inside the horizon.
================================================================
"#,
    );
}

// ============================================================================
// CHECKPOINT 1 — REPRESENTING UNCERTAINTY
// ============================================================================
// Distribution<T> means:
//
//     possible values of T
//     +
//     the probability of each value
//
// Example:
//
// Distribution<State>
// ├── Playful     @ 0.55
// ├── Engaged     @ 0.25
// └── LowInterest @ 0.20
#[derive(Debug, Clone)]
struct Distribution<T> {
    outcomes: Vec<(T, f64)>,
}

impl<T> Distribution<T> {
    // One certain value is represented as one branch with probability 1.0.
    fn certain(value: T) -> Self {
        Self {
            outcomes: vec![(value, 1.0)],
        }
    }

    // `map` changes every value while preserving every branch probability.
    //
    //     T @ p  ----transform---->  U @ p
    fn map<U>(self, transform: impl Fn(T) -> U) -> Distribution<U> {
        let mut transformed_outcomes = Vec::with_capacity(self.outcomes.len());

        for (value, probability) in self.outcomes {
            let transformed_value = transform(value);
            transformed_outcomes.push((transformed_value, probability));
        }

        Distribution {
            outcomes: transformed_outcomes,
        }
    }

    // `and_then` continues every existing branch into more branches.
    // It is the same general operation often called `flat_map` or `bind`.
    //
    // Probabilities multiply because reaching a new leaf requires BOTH:
    //
    // 1. reaching the current branch;
    // 2. taking the next branch.
    fn and_then<U>(self, next_step: impl Fn(T) -> Distribution<U>) -> Distribution<U> {
        let mut combined_outcomes = Vec::new();

        for (current_value, probability_so_far) in self.outcomes {
            let possible_next_values = next_step(current_value);

            for (next_value, next_probability) in possible_next_values.outcomes {
                // P(whole path) = P(path so far) × P(next branch)
                let path_probability = probability_so_far * next_probability;

                combined_outcomes.push((next_value, path_probability));
            }
        }

        Distribution {
            outcomes: combined_outcomes,
        }
    }

    fn number_of_outcomes(&self) -> usize {
        self.outcomes.len()
    }

    // Probability mass should remain approximately 1.0 as the tree branches.
    fn total_probability(&self) -> f64 {
        self.outcomes
            .iter()
            .map(|(_, probability)| probability)
            .sum()
    }
}

impl<T: PartialEq> Distribution<T> {
    // Different branches can produce the same value. Sum all such branches.
    fn probability_of(&self, target: &T) -> f64 {
        self.outcomes
            .iter()
            .filter(|(value, _)| value == target)
            .map(|(_, probability)| probability)
            .sum()
    }
}

// ============================================================================
// CHECKPOINT 2 — STATES AND ACTIONS
// ============================================================================
// State  = the situation the agent currently believes it is in.
// Action = a choice available to the agent.
#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    JustMet,
    Engaged,
    Playful,
    LowInterest,
    MutualInterest,
    Goodbye,
}

impl State {
    // A terminal state ends the meaningful interaction.
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Goodbye)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Action {
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
#[derive(Clone, Copy, Debug)]
struct Policy {
    boldness: f64,
}

impl Policy {
    fn possible_actions(&self, state: &State) -> Distribution<Action> {
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
#[derive(Debug, Clone, Copy)]
struct DatingWorld;

impl DatingWorld {
    fn possible_next_states(&self, state: &State, action: &Action) -> Distribution<State> {
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
    fn reward_for(&self, state: &State, action: &Action, next_state: &State) -> f64 {
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
#[derive(Clone, Debug)]
struct Transition {
    from_state: State,
    action: Action,
    reward: f64,
    to_state: State,
}

// A trajectory is one complete possible history.
type Trajectory = Vec<Transition>;

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
fn possible_transitions(
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
    if path.current_state.is_terminal() {
        return Distribution::certain(path);
    }

    let current_state = path.current_state;
    let trajectory_so_far = path.trajectory_so_far;

    possible_transitions(policy, world, &current_state).map(|transition| {
        let next_state = transition.to_state.clone();

        // Each future branch needs an independent copy of the history.
        let mut extended_trajectory = trajectory_so_far.clone();
        extended_trajectory.push(transition);

        ConversationPath {
            current_state: next_state,
            trajectory_so_far: extended_trajectory,
        }
    })
}

// A concrete one-step worked example.
//
// From JustMet:
//
//     P(Ask)   = 0.70
//     P(Story) = 0.30
//
// The world then branches conditionally:
//
//     P(Engaged | Ask)      = 0.80
//     P(LowInterest | Ask)  = 0.20
//     P(Engaged | Story)    = 0.60
//     P(LowInterest | Story)= 0.40
//
// `and_then` combines them into joint path probabilities:
//
//     P(Ask and Engaged) = 0.70 × 0.80 = 0.56
//
// This function prints the same calculation from the real program data.
fn print_first_step_walkthrough(policy: &Policy, world: &DatingWorld) {
    let transitions = possible_transitions(policy, world, &State::JustMet);

    println!(
        r#"
----------------------------------------------------------------
FIRST-STEP PROBABILITY CHECKPOINT
----------------------------------------------------------------
The policy chooses an action.
The world responds conditionally to that action.
A complete branch probability is:

P(action and next_state)
=
P(action | state) * P(next_state | state, action)
----------------------------------------------------------------
"#
    );

    for (transition, probability) in &transitions.outcomes {
        println!(
            "{:>6.1}%  {:?} --{:?} / {:+.1}--> {:?}",
            probability * 100.0,
            transition.from_state,
            transition.action,
            transition.reward,
            transition.to_state,
        );
    }

    println!(
        "Total probability mass: {:.6}\n",
        transitions.total_probability(),
    );
}

// ============================================================================
// CHECKPOINT 6 — ROLLOUT: GROW THE COMPLETE FUTURE TREE
// ============================================================================
// Horizon = how many interaction steps we inspect.
//
// This is exact enumeration, not Monte Carlo sampling.
fn enumerate_trajectories(
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
fn discounted_return(trajectory: &Trajectory, discount: f64) -> f64 {
    let mut total_return = 0.0;
    let mut current_discount_weight = 1.0;

    for transition in trajectory {
        let discounted_reward = current_discount_weight * transition.reward;

        total_return += discounted_reward;
        current_discount_weight *= discount;
    }

    total_return
}

// ============================================================================
// CHECKPOINT 8 — EXPECTATION: COLLAPSE MANY FUTURES INTO ONE SCORE
// ============================================================================
// E[value] = Σ probability(outcome) × value(outcome)
fn expected_value<T>(distribution: &Distribution<T>, value_of: impl Fn(&T) -> f64) -> f64 {
    let mut weighted_sum = 0.0;

    for (outcome, probability) in &distribution.outcomes {
        weighted_sum += probability * value_of(outcome);
    }

    weighted_sum
}

// Evaluate a fixed policy from the JustMet state.
fn expected_return_of(policy: &Policy, world: &DatingWorld, horizon: usize, discount: f64) -> f64 {
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
struct PolicyEvaluation {
    policy: Policy,
    expected_return: f64,
}

fn evaluate_policy_grid(
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

fn best_policy(evaluations: &[PolicyEvaluation]) -> Option<PolicyEvaluation> {
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

fn find_best_policy(
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> Option<PolicyEvaluation> {
    best_policy(&evaluate_policy_grid(world, horizon, discount))
}

fn main() -> std::process::ExitCode {
    learning::run()
}

// ============================================================================
// EXECUTABLE UNDERSTANDING — TEST THE INVARIANTS
// ============================================================================
// These tests are not only regression checks. Each one states a concept that
// should remain true if the implementation is correct.
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn assert_approximately_equal(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= EPSILON,
            "expected {expected:.12}, got {actual:.12}"
        );
    }

    #[test]
    fn certainty_has_one_unit_of_probability_mass() {
        let distribution = Distribution::certain(State::Engaged);

        assert_eq!(distribution.number_of_outcomes(), 1);
        assert_approximately_equal(distribution.total_probability(), 1.0);
    }

    #[test]
    fn map_changes_values_but_preserves_probability_mass() {
        let numbers = Distribution {
            outcomes: vec![(2_i32, 0.25), (4_i32, 0.75)],
        };

        let doubled = numbers.map(|number| number * 2);

        assert_eq!(doubled.outcomes, vec![(4_i32, 0.25), (8_i32, 0.75)]);
        assert_approximately_equal(doubled.total_probability(), 1.0);
    }

    #[test]
    fn and_then_multiplies_branch_probabilities() {
        let first_stage = Distribution {
            outcomes: vec![("Ask", 0.70), ("Story", 0.30)],
        };

        let completed_paths = first_stage.and_then(|action| match action {
            "Ask" => Distribution {
                outcomes: vec![("Ask -> Engaged", 0.80), ("Ask -> LowInterest", 0.20)],
            },
            "Story" => Distribution {
                outcomes: vec![("Story -> Engaged", 0.60), ("Story -> LowInterest", 0.40)],
            },
            _ => unreachable!("the test defines only two actions"),
        });

        assert_approximately_equal(completed_paths.probability_of(&"Ask -> Engaged"), 0.56);
        assert_approximately_equal(
            completed_paths.probability_of(&"Story -> LowInterest"),
            0.12,
        );
        assert_approximately_equal(completed_paths.total_probability(), 1.0);
    }

    #[test]
    fn every_grid_policy_is_normalized() {
        for index in 0..=10 {
            let policy = Policy {
                boldness: index as f64 / 10.0,
            };

            for state in [
                State::JustMet,
                State::Engaged,
                State::Playful,
                State::LowInterest,
                State::MutualInterest,
                State::Goodbye,
            ] {
                let actions = policy.possible_actions(&state);

                assert!(actions.outcomes.iter().all(|(_, probability)| {
                    probability.is_finite() && (0.0..=1.0).contains(probability)
                }));

                assert_approximately_equal(actions.total_probability(), 1.0);
            }
        }
    }

    #[test]
    fn every_world_branch_is_a_valid_distribution() {
        let world = DatingWorld;
        for state in [
            State::JustMet,
            State::Engaged,
            State::Playful,
            State::LowInterest,
            State::MutualInterest,
            State::Goodbye,
        ] {
            for action in [
                Action::Ask,
                Action::Story,
                Action::Tease,
                Action::Coffee,
                Action::Space,
                Action::Goodbye,
            ] {
                let outcomes = world.possible_next_states(&state, &action);
                assert_approximately_equal(outcomes.total_probability(), 1.0);
                assert!(outcomes.outcomes.iter().all(|(_, probability)| {
                    probability.is_finite() && (0.0..=1.0).contains(probability)
                }));
            }
        }
    }

    #[test]
    fn reaching_goodbye_stops_history_without_losing_probability() {
        let policy = Policy { boldness: 0.3 };
        let world = DatingWorld;
        let paths = enumerate_trajectories(&policy, &world, &State::LowInterest, 5);

        assert_approximately_equal(paths.total_probability(), 1.0);
        for (trajectory, _) in paths.outcomes {
            assert_eq!(trajectory.len(), 1);
            assert_eq!(trajectory[0].to_state, State::Goodbye);
            assert_approximately_equal(discounted_return(&trajectory, 0.95), 2.0);
        }
    }

    #[test]
    fn first_step_joint_probabilities_match_the_hand_calculation() {
        let policy = Policy { boldness: 0.5 };
        let world = DatingWorld;

        let transitions = possible_transitions(&policy, &world, &State::JustMet);

        let ask_to_engaged_probability: f64 = transitions
            .outcomes
            .iter()
            .filter(|(transition, _)| {
                transition.action == Action::Ask && transition.to_state == State::Engaged
            })
            .map(|(_, probability)| probability)
            .sum();

        let story_to_low_interest_probability: f64 = transitions
            .outcomes
            .iter()
            .filter(|(transition, _)| {
                transition.action == Action::Story && transition.to_state == State::LowInterest
            })
            .map(|(_, probability)| probability)
            .sum();

        assert_approximately_equal(ask_to_engaged_probability, 0.56);
        assert_approximately_equal(story_to_low_interest_probability, 0.12);
        assert_approximately_equal(transitions.total_probability(), 1.0);
    }

    #[test]
    fn the_current_model_selects_boldness_point_three() {
        let world = DatingWorld;
        let best = find_best_policy(&world, 5, 0.95).expect("the grid contains candidate policies");

        assert_approximately_equal(best.policy.boldness, 0.3);
        assert_approximately_equal(best.expected_return, 3.103947335269);
    }

    #[test]
    fn zero_discount_does_not_create_a_spurious_preference_for_boldness() {
        let scores = evaluate_policy_grid(&DatingWorld, 5, 0.0);
        for score in &scores {
            assert_approximately_equal(score.expected_return, 0.22);
        }
        let best = best_policy(&scores).expect("the grid has eleven policies");
        assert_approximately_equal(best.policy.boldness, 0.0);
    }
}
