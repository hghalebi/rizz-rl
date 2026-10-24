//! Invariants shared by the concrete model and exact planner.

use crate::model::{Action, DatingWorld, InvalidBoldness, Policy, State};
use crate::planning::{
    best_policy, discounted_return, enumerate_trajectories, evaluate_policy_grid, find_best_policy,
    possible_transitions,
};
use crate::probability::Distribution;

const EPSILON: f64 = 1e-9;

fn policy(boldness: f64) -> Policy {
    Policy::try_new(boldness).expect("test boldness lies in [0, 1]")
}

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
fn boldness_outside_the_unit_interval_cannot_become_a_policy() {
    // Runtime values such as CLI text are checked here. The private field is
    // what stops other modules from skipping this check; the compiler enforces
    // that, and no runtime test can observe it.
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 1.1] {
        assert_eq!(Policy::try_new(invalid), Err(InvalidBoldness), "{invalid}");
    }
    for valid in [0.0, 0.3, 1.0] {
        assert_approximately_equal(policy(valid).boldness(), valid);
    }
}

#[test]
fn every_grid_policy_is_normalized() {
    for index in 0..=10 {
        let policy = policy(index as f64 / 10.0);

        for state in State::ALL {
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
    for state in State::ALL {
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
    let policy = policy(0.3);
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
    let policy = policy(0.5);
    let world = DatingWorld;

    let transitions = possible_transitions(&policy, &world, &State::JustMet);
    assert_approximately_equal(transitions.total_probability(), 1.0);

    // Project each transition onto the (action, next state) event, then reuse
    // probability_of to sum every branch that reaches that event.
    let events = transitions.map(|transition| (transition.action, transition.to_state));

    assert_approximately_equal(events.probability_of(&(Action::Ask, State::Engaged)), 0.56);
    assert_approximately_equal(
        events.probability_of(&(Action::Story, State::LowInterest)),
        0.12,
    );
}

#[test]
fn the_current_model_selects_boldness_point_three() {
    let world = DatingWorld;
    let best = find_best_policy(&world, 5, 0.95).expect("the grid contains candidate policies");

    assert_approximately_equal(best.policy.boldness(), 0.3);
    assert_approximately_equal(best.expected_return, 3.103947335269);
}

#[test]
fn zero_discount_does_not_create_a_spurious_preference_for_boldness() {
    let scores = evaluate_policy_grid(&DatingWorld, 5, 0.0);
    for score in &scores {
        assert_approximately_equal(score.expected_return, 0.22);
    }
    let best = best_policy(&scores).expect("the grid has eleven policies");
    assert_approximately_equal(best.policy.boldness(), 0.0);
}
