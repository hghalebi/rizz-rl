//! Shared acceptance cases for the unfinished exercises and separate solutions.
//! These contain observable inputs and expected results, not implementations.

type BellmanBackup = fn(&[(f64, f64, f64)], f64) -> f64;

fn assert_close(actual: f64, expected: f64) {
    assert!(actual.is_finite(), "expected a finite result, got {actual}");
    assert!(
        (actual - expected).abs() < 1e-10,
        "expected {expected}, got {actual}"
    );
}

fn assert_probabilities(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len(), "one probability per action");
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_close(actual, expected);
        assert!((0.0..=1.0).contains(&actual));
    }
    if !actual.is_empty() {
        assert_close(actual.iter().sum(), 1.0);
    }
}

fn assert_optional_close(actual: Option<f64>, expected: f64) {
    assert_close(
        actual.expect("a target should exist for this input"),
        expected,
    );
}

pub(super) fn exercise_1(discounted_return: fn(&[f64], f64) -> f64) {
    assert_close(discounted_return(&[], 0.9), 0.0);
    assert_close(discounted_return(&[3.0], 0.5), 3.0);
    assert_close(discounted_return(&[1.0, 2.0, 4.0], 0.5), 3.0);
    assert_close(discounted_return(&[1.0, 2.0, 4.0], 0.0), 1.0);
    assert_close(discounted_return(&[1.0, 2.0, 4.0], 1.0), 7.0);
    assert_close(discounted_return(&[-2.0, 2.0], 0.5), -1.0);
}

pub(super) fn exercise_2(weighted_expectation: fn(&[(f64, f64)]) -> f64) {
    assert_close(weighted_expectation(&[]), 0.0);
    assert_close(weighted_expectation(&[(7.0, 1.0)]), 7.0);
    assert_close(weighted_expectation(&[(8.0, 0.75), (-2.0, 0.25)]), 5.5);
    assert_close(weighted_expectation(&[(-2.0, 0.25), (8.0, 0.75)]), 5.5);
    assert_close(weighted_expectation(&[(100.0, 0.0), (-3.0, 1.0)]), -3.0);
}

pub(super) fn exercise_3(td_target: fn(f64, f64, f64, bool) -> f64) {
    assert_close(td_target(2.0, 0.9, 10.0, false), 11.0);
    assert_close(td_target(2.0, 0.9, 10.0, true), 2.0);
    // A time limit in a continuing task still bootstraps.
    let terminated = false;
    assert_close(td_target(2.0, 0.9, 10.0, terminated), 11.0);
    assert_close(td_target(-2.0, 0.0, 10.0, false), -2.0);
    assert_close(td_target(-2.0, 1.0, 10.0, false), 8.0);
    assert_close(td_target(-2.0, 1.0, 10.0, true), -2.0);
}

pub(super) fn exercise_4(is_probability_distribution: fn(&[f64]) -> bool) {
    for valid in [
        vec![1.0],
        vec![0.0, 1.0],
        vec![0.2, 0.3, 0.5],
        vec![0.5, 0.2, 0.3],
        vec![0.5, 0.5 + 4e-10],
        vec![0.5, 0.5 - 4e-10],
    ] {
        assert!(is_probability_distribution(&valid), "rejected {valid:?}");
    }
    for invalid in [
        vec![],
        vec![f64::NAN],
        vec![f64::INFINITY],
        vec![f64::NEG_INFINITY],
        vec![-0.1, 1.1],
        vec![-1e-12, 1.0],
        vec![1.0 + 1e-12],
        vec![0.2, 0.3],
        vec![0.6, 0.6],
        vec![0.5, 0.5 + 2e-9],
        vec![0.5, 0.5 - 2e-9],
    ] {
        assert!(
            !is_probability_distribution(&invalid),
            "accepted {invalid:?}"
        );
    }
}

pub(super) fn exercise_5(greedy_action: fn(&[f64]) -> Option<usize>) {
    assert_eq!(greedy_action(&[]), None);
    assert_eq!(greedy_action(&[4.0]), Some(0));
    assert_eq!(greedy_action(&[1.0, 5.0, 3.0]), Some(1));
    assert_eq!(greedy_action(&[1.0, 3.0, 5.0]), Some(2));
    assert_eq!(greedy_action(&[4.0, 4.0, 1.0]), Some(0));
    assert_eq!(greedy_action(&[1.0, 4.0, 4.0]), Some(1));
    assert_eq!(greedy_action(&[-4.0, -2.0, -3.0]), Some(1));
    assert_eq!(greedy_action(&[0.0, 0.0, 0.0]), Some(0));
}

pub(super) fn exercise_6(epsilon_greedy: fn(&[f64], f64) -> Vec<f64>) {
    assert!(epsilon_greedy(&[], 0.0).is_empty());
    assert!(epsilon_greedy(&[], 1.0).is_empty());
    assert_probabilities(&epsilon_greedy(&[2.0, 5.0, 5.0], 0.0), &[0.0, 1.0, 0.0]);
    assert_probabilities(
        &epsilon_greedy(&[2.0, 5.0, 1.0, 4.0], 1.0),
        &[0.25, 0.25, 0.25, 0.25],
    );
    assert_probabilities(&epsilon_greedy(&[1.0, 3.0, 2.0], 0.3), &[0.1, 0.8, 0.1]);
    assert_probabilities(&epsilon_greedy(&[3.0, 3.0, 1.0], 0.3), &[0.8, 0.1, 0.1]);
    assert_probabilities(&epsilon_greedy(&[-4.0, -2.0, -3.0], 0.3), &[0.1, 0.8, 0.1]);
    for epsilon in [0.0, 0.3, 1.0] {
        assert_probabilities(&epsilon_greedy(&[9.0], epsilon), &[1.0]);
    }
}

pub(super) fn exercise_7(update_mean: fn(f64, f64, usize) -> f64) {
    assert_close(update_mean(100.0, 4.0, 1), 4.0);
    assert_close(update_mean(4.0, 8.0, 2), 6.0);
    assert_close(update_mean(6.0, -3.0, 3), 3.0);
    assert_close(update_mean(-4.0, -2.0, 2), -3.0);
    for returns in [[4.0, 8.0, -3.0], [-3.0, 4.0, 8.0], [8.0, -3.0, 4.0]] {
        let mut observed_mean = 0.0;
        for (index, observed_return) in returns.into_iter().enumerate() {
            observed_mean = update_mean(observed_mean, observed_return, index + 1);
        }
        assert_close(observed_mean, 3.0);
    }
}

pub(super) fn exercise_8(td_update: fn(f64, f64, f64, f64, bool, f64) -> f64) {
    assert_close(td_update(4.0, 2.0, 0.9, 10.0, false, 0.25), 5.75);
    assert_close(td_update(4.0, 2.0, 0.9, 10.0, true, 0.25), 3.5);
    assert_close(td_update(4.0, 2.0, 0.0, 10.0, false, 0.25), 3.5);
    assert_close(td_update(4.0, 2.0, 0.9, 10.0, false, 0.0), 4.0);
    assert_close(td_update(4.0, 2.0, 0.9, 10.0, false, 1.0), 11.0);
    assert_close(td_update(10.0, -2.0, 0.5, 4.0, false, 0.2), 8.0);
    assert_close(td_update(-4.0, -2.0, 0.5, 0.0, false, 0.5), -3.0);
    let mut current = 0.0;
    for expected in [5.0, 7.5, 8.75, 9.375] {
        let next = td_update(current, 10.0, 1.0, 0.0, true, 0.5);
        assert_close(next, expected);
        assert!(
            next >= current && next <= 10.0,
            "update overshot or moved away"
        );
        current = next;
    }
}

pub(super) fn exercise_9(q_learning_target: fn(f64, f64, &[f64], bool) -> Option<f64>) {
    assert_optional_close(q_learning_target(2.0, 0.5, &[1.0, 4.0, 2.0], false), 4.0);
    assert_optional_close(q_learning_target(2.0, 0.5, &[4.0, 2.0, 1.0], false), 4.0);
    assert_optional_close(q_learning_target(2.0, 0.5, &[-4.0, -1.0, -3.0], false), 1.5);
    assert_optional_close(q_learning_target(2.0, 0.0, &[10.0], false), 2.0);
    assert_optional_close(q_learning_target(2.0, 1.0, &[10.0], false), 12.0);
    assert_optional_close(q_learning_target(2.0, 0.9, &[10.0], true), 2.0);
    assert_optional_close(q_learning_target(-2.0, 0.9, &[], true), -2.0);
    assert_eq!(q_learning_target(2.0, 0.9, &[], false), None);
    assert_eq!(q_learning_target(2.0, 0.0, &[], false), None);
}

pub(super) fn exercise_10(bellman_backup: BellmanBackup) {
    assert_close(bellman_backup(&[], 0.9), 0.0);
    assert_close(bellman_backup(&[(1.0, 2.0, 10.0)], 0.9), 11.0);
    assert_close(bellman_backup(&[(1.0, 2.0, 10.0)], 0.0), 2.0);
    let branches = [(0.75, 8.0, 0.0), (0.25, -2.0, 4.0)];
    assert_close(bellman_backup(&branches, 0.0), 5.5);
    assert_close(bellman_backup(&branches, 0.5), 6.0);
    assert_close(bellman_backup(&branches, 1.0), 6.5);
    assert_close(bellman_backup(&[branches[1], branches[0]], 0.5), 6.0);
    assert_close(
        bellman_backup(&[(0.5, 2.0, 4.0), (0.5, -2.0, 8.0)], 0.5),
        3.0,
    );
    assert_close(
        bellman_backup(&[(0.0, 100.0, 100.0), (1.0, -2.0, 0.0)], 1.0),
        -2.0,
    );
}
