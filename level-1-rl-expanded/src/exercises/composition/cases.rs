//! Shared behavioral cases. Read after predicting or when diagnosing a failure.
//! Expected probabilities are observable outcomes, not learner implementations.

use super::{
    AssociationResults, BindIdentityResults, Distribution, IntegerBranch, IntegerMap, MapResults,
    StringBranch,
};
use std::fmt::Debug;

const TOLERANCE: f64 = 1e-9;

fn assert_normalized<T>(distribution: &Distribution<T>) {
    assert!(
        !distribution.outcomes.is_empty(),
        "expected nonempty support"
    );
    for (_, probability) in &distribution.outcomes {
        assert!(
            probability.is_finite() && (0.0..=1.0).contains(probability),
            "invalid branch probability {probability}"
        );
    }
    assert!((distribution.total_probability() - 1.0).abs() <= TOLERANCE);
}

/// Compare event probabilities across the union of outcomes, not Vec layout.
/// Repeated checks for duplicate outcomes are harmless in these tiny fixtures.
pub(super) fn assert_same_events<T: Eq + Debug>(
    actual: &Distribution<T>,
    expected: &Distribution<T>,
) {
    assert_normalized(actual);
    assert_normalized(expected);
    for (outcome, _) in actual.outcomes.iter().chain(&expected.outcomes) {
        let actual_mass = actual.probability_of(outcome);
        let expected_mass = expected.probability_of(outcome);
        assert!(
            (actual_mass - expected_mass).abs() <= TOLERANCE,
            "event {outcome:?}: expected probability {expected_mass}, got {actual_mass}"
        );
    }
}

pub(super) fn choices() -> Distribution<String> {
    Distribution {
        outcomes: vec![("ask".into(), 0.7), ("story".into(), 0.3)],
    }
}

pub(super) fn response(choice: String) -> Distribution<String> {
    let engaged = match choice.as_str() {
        "ask" => 0.8,
        "story" => 0.6,
        _ => unreachable!("this fixture supplies only ask and story"),
    };
    Distribution {
        outcomes: vec![("engaged".into(), engaged), ("quiet".into(), 1.0 - engaged)],
    }
}

fn travel(route: String) -> Distribution<String> {
    let goal = match route.as_str() {
        "north" => 0.75,
        "south" => 0.25,
        _ => unreachable!("this fixture supplies only north and south"),
    };
    Distribution {
        outcomes: vec![("goal".into(), goal), ("delay".into(), 1.0 - goal)],
    }
}

pub(super) fn ownership(implementation: fn(Distribution<String>) -> (usize, Distribution<usize>)) {
    let (branch_count, lengths) = implementation(choices());
    assert_eq!(branch_count, 2);
    assert_same_events(
        &lengths,
        &Distribution {
            outcomes: vec![(3, 0.7), (5, 0.3)],
        },
    );
}

pub(super) fn branching(
    implementation: fn(Distribution<String>, StringBranch) -> Distribution<String>,
) {
    let result = implementation(choices(), response);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![("engaged".into(), 0.74), ("quiet".into(), 0.26)],
        },
    );
    let result = implementation(Distribution::certain("story".into()), response);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![("engaged".into(), 0.6), ("quiet".into(), 0.4)],
        },
    );
}

pub(super) fn independent(
    implementation: fn(Distribution<String>, StringBranch) -> Distribution<usize>,
) {
    let result = implementation(choices(), response);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![(7, 0.74), (5, 0.26)],
        },
    );
    let result = implementation(Distribution::certain("story".into()), response);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![(7, 0.6), (5, 0.4)],
        },
    );
}

pub(super) fn navigation(
    implementation: fn(Distribution<String>, StringBranch) -> Distribution<bool>,
) {
    let routes = Distribution {
        outcomes: vec![("north".into(), 0.6), ("south".into(), 0.4)],
    };
    let result = implementation(routes, travel);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![(true, 0.55), (false, 0.45)],
        },
    );
    let result = implementation(Distribution::certain("south".into()), travel);
    assert_same_events(
        &result,
        &Distribution {
            outcomes: vec![(true, 0.25), (false, 0.75)],
        },
    );
}

fn law_input() -> Distribution<i32> {
    // Duplicate outcomes and an explicit zero branch represent the same measure
    // as [(1, 0.5), (2, 0.5)]. They need not have the same Vec representation.
    Distribution {
        outcomes: vec![(1, 0.2), (1, 0.3), (2, 0.5), (99, 0.0)],
    }
}

fn add_one(value: i32) -> i32 {
    value + 1
}
fn double(value: i32) -> i32 {
    value * 2
}

fn first_branch(value: i32) -> Distribution<i32> {
    Distribution {
        outcomes: vec![(value, 0.25), (value + 1, 0.75)],
    }
}

fn second_branch(value: i32) -> Distribution<i32> {
    Distribution {
        outcomes: vec![(value * 2, 0.4), (value * 2 + 1, 0.6)],
    }
}

pub(super) fn map_laws(
    implementation: fn(Distribution<i32>, IntegerMap, IntegerMap) -> MapResults,
) {
    let result = implementation(law_input(), add_one, double);
    assert_same_events(&result.identity, &law_input());
    let expected = Distribution {
        outcomes: vec![(4, 0.5), (6, 0.5)],
    };
    assert_same_events(&result.sequential, &expected);
    assert_same_events(&result.composed, &expected);
    assert_same_events(&result.sequential, &result.composed);
}

pub(super) fn bind_identities(
    implementation: fn(Distribution<i32>, i32, IntegerBranch) -> BindIdentityResults,
) {
    let result = implementation(law_input(), 2, first_branch);
    assert_same_events(
        &result.left_identity,
        &Distribution {
            outcomes: vec![(2, 0.25), (3, 0.75)],
        },
    );
    assert_same_events(&result.right_identity, &law_input());
}

pub(super) fn associativity(
    implementation: fn(Distribution<i32>, IntegerBranch, IntegerBranch) -> AssociationResults,
) {
    let result = implementation(law_input(), first_branch, second_branch);
    let expected = Distribution {
        outcomes: vec![
            (2, 0.05),
            (3, 0.075),
            (4, 0.2),
            (5, 0.3),
            (6, 0.15),
            (7, 0.225),
        ],
    };
    assert_same_events(&result.sequential, &expected);
    assert_same_events(&result.regrouped, &expected);
    assert_same_events(&result.sequential, &result.regrouped);
}

#[test]
fn comparison_ignores_order_duplicates_and_zero_branches() {
    assert_same_events(
        &law_input(),
        &Distribution {
            outcomes: vec![(2, 0.5), (1, 0.5)],
        },
    );
}

#[test]
#[should_panic(expected = "event")]
fn comparison_checks_outcomes_present_on_only_one_side() {
    assert_same_events(&Distribution::certain(1), &Distribution::certain(2));
}

#[test]
#[should_panic(expected = "invalid branch probability")]
fn comparison_rejects_nonfinite_probabilities() {
    assert_same_events(
        &Distribution {
            outcomes: vec![(1, f64::NAN)],
        },
        &Distribution::certain(1),
    );
}
