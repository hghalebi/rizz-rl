//! Shared acceptance cases for the unfinished exercises and separate solutions.
//! These contain observable inputs and expected results, not implementations.

type BellmanBackup = fn(&[(f64, f64, f64)], f64) -> f64;

fn assert_close(case: &str, actual: f64, expected: f64) {
    assert!(
        actual.is_finite(),
        "{case}: expected finite {expected}, actual {actual}"
    );
    assert!(
        (actual - expected).abs() < 1e-10,
        "{case}: expected {expected}, actual {actual}"
    );
}

fn assert_probabilities(case: &str, actual: &[f64], expected: &[f64]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{case}: expected one probability per action {expected:?}, actual {actual:?}"
    );
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(&format!("{case}; action index={index}"), actual, expected);
        assert!(
            (0.0..=1.0).contains(&actual),
            "{case}; action index={index}: expected probability in [0, 1], actual {actual}"
        );
    }
    if !actual.is_empty() {
        assert_close(
            &format!("{case}; total probability"),
            actual.iter().sum(),
            1.0,
        );
    }
}

fn assert_optional_close(case: &str, actual: Option<f64>, expected: f64) {
    let actual = actual.unwrap_or_else(|| panic!("{case}: expected Some({expected}), actual None"));
    assert_close(case, actual, expected);
}

pub(super) fn exercise_1(discounted_return: fn(&[f64], f64) -> f64) {
    assert_close(
        "empty rewards; input: discounted_return(&[], 0.9)",
        discounted_return(&[], 0.9),
        0.0,
    );
    assert_close(
        "one immediate reward; input: discounted_return(&[3.0], 0.5)",
        discounted_return(&[3.0], 0.5),
        3.0,
    );
    assert_close(
        "later rewards discounted; input: discounted_return(&[1.0, 2.0, 4.0], 0.5)",
        discounted_return(&[1.0, 2.0, 4.0], 0.5),
        3.0,
    );
    assert_close(
        "zero discount; input: discounted_return(&[1.0, 2.0, 4.0], 0.0)",
        discounted_return(&[1.0, 2.0, 4.0], 0.0),
        1.0,
    );
    assert_close(
        "unit discount; input: discounted_return(&[1.0, 2.0, 4.0], 1.0)",
        discounted_return(&[1.0, 2.0, 4.0], 1.0),
        7.0,
    );
    assert_close(
        "negative immediate reward; input: discounted_return(&[-2.0, 2.0], 0.5)",
        discounted_return(&[-2.0, 2.0], 0.5),
        -1.0,
    );
}

pub(super) fn exercise_2(weighted_expectation: fn(&[(f64, f64)]) -> f64) {
    assert_close(
        "empty weighted sum; input: weighted_expectation(&[])",
        weighted_expectation(&[]),
        0.0,
    );
    assert_close(
        "certain outcome; input: weighted_expectation(&[(7.0, 1.0)])",
        weighted_expectation(&[(7.0, 1.0)]),
        7.0,
    );
    assert_close(
        "unequal outcome weights; input: weighted_expectation(&[(8.0, 0.75), (-2.0, 0.25)])",
        weighted_expectation(&[(8.0, 0.75), (-2.0, 0.25)]),
        5.5,
    );
    assert_close(
        "reordered outcomes; input: weighted_expectation(&[(-2.0, 0.25), (8.0, 0.75)])",
        weighted_expectation(&[(-2.0, 0.25), (8.0, 0.75)]),
        5.5,
    );
    assert_close(
        "zero-probability outcome; input: weighted_expectation(&[(100.0, 0.0), (-3.0, 1.0)])",
        weighted_expectation(&[(100.0, 0.0), (-3.0, 1.0)]),
        -3.0,
    );
}

pub(super) fn exercise_3(td_target: fn(f64, f64, f64, bool) -> f64) {
    assert_close(
        "continuing task; input: td_target(2.0, 0.9, 10.0, false)",
        td_target(2.0, 0.9, 10.0, false),
        11.0,
    );
    assert_close(
        "true termination; input: td_target(2.0, 0.9, 10.0, true)",
        td_target(2.0, 0.9, 10.0, true),
        2.0,
    );
    // A time limit in a continuing task still bootstraps.
    let terminated = false;
    assert_close(
        "observation cutoff; input: reward=2, gamma=0.9, next_value=10, terminated=false",
        td_target(2.0, 0.9, 10.0, terminated),
        11.0,
    );
    assert_close(
        "zero discount with negative reward; input: td_target(-2.0, 0.0, 10.0, false)",
        td_target(-2.0, 0.0, 10.0, false),
        -2.0,
    );
    assert_close(
        "unit discount; input: td_target(-2.0, 1.0, 10.0, false)",
        td_target(-2.0, 1.0, 10.0, false),
        8.0,
    );
    assert_close(
        "terminal negative reward; input: td_target(-2.0, 1.0, 10.0, true)",
        td_target(-2.0, 1.0, 10.0, true),
        -2.0,
    );
}

pub(super) fn exercise_4(is_probability_distribution: fn(&[f64]) -> bool) {
    for (case, valid) in [
        ("single certain event", vec![1.0]),
        ("zero-probability event", vec![0.0, 1.0]),
        ("three normalized weights", vec![0.2, 0.3, 0.5]),
        ("permuted normalized weights", vec![0.5, 0.2, 0.3]),
        (
            "total slightly above one within tolerance",
            vec![0.5, 0.5 + 4e-10],
        ),
        (
            "total slightly below one within tolerance",
            vec![0.5, 0.5 - 4e-10],
        ),
    ] {
        let actual = is_probability_distribution(&valid);
        assert!(
            actual,
            "{case}; input: weights={valid:?}: expected true, actual {actual}"
        );
    }
    for (case, invalid) in [
        ("empty weights", vec![]),
        ("NaN weight", vec![f64::NAN]),
        ("positive infinity", vec![f64::INFINITY]),
        ("negative infinity", vec![f64::NEG_INFINITY]),
        ("negative and above-one weights", vec![-0.1, 1.1]),
        ("tiny negative weight", vec![-1e-12, 1.0]),
        ("weight slightly above one", vec![1.0 + 1e-12]),
        ("total below one", vec![0.2, 0.3]),
        ("total above one", vec![0.6, 0.6]),
        ("total above tolerance", vec![0.5, 0.5 + 2e-9]),
        ("total below tolerance", vec![0.5, 0.5 - 2e-9]),
    ] {
        let actual = is_probability_distribution(&invalid);
        assert!(
            !actual,
            "{case}; input: weights={invalid:?}: expected false, actual {actual}"
        );
    }
}

pub(super) fn exercise_5(greedy_action: fn(&[f64]) -> Option<usize>) {
    let actual = greedy_action(&[]);
    assert_eq!(
        actual, None,
        "no available actions; input: values=&[]: expected None, actual {actual:?}"
    );
    let actual = greedy_action(&[4.0]);
    assert_eq!(
        actual,
        Some(0),
        "single action; input: values=&[4.0]: expected Some(0), actual {actual:?}"
    );
    let actual = greedy_action(&[1.0, 5.0, 3.0]);
    assert_eq!(actual, Some(1), "best action in the middle; input: values=&[1.0, 5.0, 3.0]: expected Some(1), actual {actual:?}");
    let actual = greedy_action(&[1.0, 3.0, 5.0]);
    assert_eq!(actual, Some(2), "best action at the end; input: values=&[1.0, 3.0, 5.0]: expected Some(2), actual {actual:?}");
    let actual = greedy_action(&[4.0, 4.0, 1.0]);
    assert_eq!(actual, Some(0), "first index wins an initial tie; input: values=&[4.0, 4.0, 1.0]: expected Some(0), actual {actual:?}");
    let actual = greedy_action(&[1.0, 4.0, 4.0]);
    assert_eq!(actual, Some(1), "first index wins a later tie; input: values=&[1.0, 4.0, 4.0]: expected Some(1), actual {actual:?}");
    let actual = greedy_action(&[-4.0, -2.0, -3.0]);
    assert_eq!(actual, Some(1), "all-negative action values; input: values=&[-4.0, -2.0, -3.0]: expected Some(1), actual {actual:?}");
    let actual = greedy_action(&[0.0, 0.0, 0.0]);
    assert_eq!(
        actual,
        Some(0),
        "all-zero tie; input: values=&[0.0, 0.0, 0.0]: expected Some(0), actual {actual:?}"
    );
}

pub(super) fn exercise_6(epsilon_greedy: fn(&[f64], f64) -> Vec<f64>) {
    let actual = epsilon_greedy(&[], 0.0);
    assert!(
        actual.is_empty(),
        "empty action set; input: values=[], epsilon=0.0: expected [], actual {actual:?}"
    );
    let actual = epsilon_greedy(&[], 1.0);
    assert!(
        actual.is_empty(),
        "empty action set; input: values=[], epsilon=1.0: expected [], actual {actual:?}"
    );
    assert_probabilities(
        "zero exploration with tied best actions; input: &epsilon_greedy(&[2.0, 5.0, 5.0], 0.0)",
        &epsilon_greedy(&[2.0, 5.0, 5.0], 0.0),
        &[0.0, 1.0, 0.0],
    );
    assert_probabilities(
        "full exploration over four actions; input: &epsilon_greedy(&[2.0, 5.0, 1.0, 4.0], 1.0)",
        &epsilon_greedy(&[2.0, 5.0, 1.0, 4.0], 1.0),
        &[0.25, 0.25, 0.25, 0.25],
    );
    assert_probabilities(
        "partial exploration; input: &epsilon_greedy(&[1.0, 3.0, 2.0], 0.3)",
        &epsilon_greedy(&[1.0, 3.0, 2.0], 0.3),
        &[0.1, 0.8, 0.1],
    );
    assert_probabilities(
        "first tied action receives exploitation; input: &epsilon_greedy(&[3.0, 3.0, 1.0], 0.3)",
        &epsilon_greedy(&[3.0, 3.0, 1.0], 0.3),
        &[0.8, 0.1, 0.1],
    );
    assert_probabilities(
        "negative action values; input: &epsilon_greedy(&[-4.0, -2.0, -3.0], 0.3)",
        &epsilon_greedy(&[-4.0, -2.0, -3.0], 0.3),
        &[0.1, 0.8, 0.1],
    );
    for epsilon in [0.0, 0.3, 1.0] {
        assert_probabilities(
            &format!("single available action; input: values=[9], epsilon={epsilon}"),
            &epsilon_greedy(&[9.0], epsilon),
            &[1.0],
        );
    }
}

pub(super) fn exercise_7(update_mean: fn(f64, f64, usize) -> f64) {
    assert_close(
        "first sample ignores initial mean; input: update_mean(100.0, 4.0, 1)",
        update_mean(100.0, 4.0, 1),
        4.0,
    );
    assert_close(
        "second sample; input: update_mean(4.0, 8.0, 2)",
        update_mean(4.0, 8.0, 2),
        6.0,
    );
    assert_close(
        "third sample with negative return; input: update_mean(6.0, -3.0, 3)",
        update_mean(6.0, -3.0, 3),
        3.0,
    );
    assert_close(
        "negative sample mean; input: update_mean(-4.0, -2.0, 2)",
        update_mean(-4.0, -2.0, 2),
        -3.0,
    );
    for returns in [[4.0, 8.0, -3.0], [-3.0, 4.0, 8.0], [8.0, -3.0, 4.0]] {
        let mut observed_mean = 0.0;
        for (index, observed_return) in returns.into_iter().enumerate() {
            observed_mean = update_mean(observed_mean, observed_return, index + 1);
        }
        assert_close(
            &format!("order-independent mean; input: returns={returns:?}, initial mean=0"),
            observed_mean,
            3.0,
        );
    }
}

pub(super) fn exercise_8(td_update: fn(f64, f64, f64, f64, bool, f64) -> f64) {
    assert_close(
        "positive TD error; input: td_update(4.0, 2.0, 0.9, 10.0, false, 0.25)",
        td_update(4.0, 2.0, 0.9, 10.0, false, 0.25),
        5.75,
    );
    assert_close(
        "terminal update; input: td_update(4.0, 2.0, 0.9, 10.0, true, 0.25)",
        td_update(4.0, 2.0, 0.9, 10.0, true, 0.25),
        3.5,
    );
    assert_close(
        "zero discount; input: td_update(4.0, 2.0, 0.0, 10.0, false, 0.25)",
        td_update(4.0, 2.0, 0.0, 10.0, false, 0.25),
        3.5,
    );
    assert_close(
        "zero learning rate; input: td_update(4.0, 2.0, 0.9, 10.0, false, 0.0)",
        td_update(4.0, 2.0, 0.9, 10.0, false, 0.0),
        4.0,
    );
    assert_close(
        "unit learning rate; input: td_update(4.0, 2.0, 0.9, 10.0, false, 1.0)",
        td_update(4.0, 2.0, 0.9, 10.0, false, 1.0),
        11.0,
    );
    assert_close(
        "negative TD error; input: td_update(10.0, -2.0, 0.5, 4.0, false, 0.2)",
        td_update(10.0, -2.0, 0.5, 4.0, false, 0.2),
        8.0,
    );
    assert_close(
        "negative estimates; input: td_update(-4.0, -2.0, 0.5, 0.0, false, 0.5)",
        td_update(-4.0, -2.0, 0.5, 0.0, false, 0.5),
        -3.0,
    );
    let mut current = 0.0;
    for expected in [5.0, 7.5, 8.75, 9.375] {
        let next = td_update(current, 10.0, 1.0, 0.0, true, 0.5);
        assert_close(&format!("repeated update; input: current={current}, reward=10, gamma=1, next_value=0, terminated=true, alpha=0.5"), next, expected);
        assert!(
            next >= current && next <= 10.0,
            "repeated update; input: current={current}, target=10, alpha=0.5: expected value in [{current}, 10], actual {next}"
        );
        current = next;
    }
}

pub(super) fn exercise_9(q_learning_target: fn(f64, f64, &[f64], bool) -> Option<f64>) {
    assert_optional_close(
        "best next action; input: q_learning_target(2.0, 0.5, &[1.0, 4.0, 2.0], false)",
        q_learning_target(2.0, 0.5, &[1.0, 4.0, 2.0], false),
        4.0,
    );
    assert_optional_close(
        "reordered next actions; input: q_learning_target(2.0, 0.5, &[4.0, 2.0, 1.0], false)",
        q_learning_target(2.0, 0.5, &[4.0, 2.0, 1.0], false),
        4.0,
    );
    assert_optional_close(
        "all-negative next actions; input: q_learning_target(2.0, 0.5, &[-4.0, -1.0, -3.0], false)",
        q_learning_target(2.0, 0.5, &[-4.0, -1.0, -3.0], false),
        1.5,
    );
    assert_optional_close(
        "zero discount with available action; input: q_learning_target(2.0, 0.0, &[10.0], false)",
        q_learning_target(2.0, 0.0, &[10.0], false),
        2.0,
    );
    assert_optional_close(
        "unit discount; input: q_learning_target(2.0, 1.0, &[10.0], false)",
        q_learning_target(2.0, 1.0, &[10.0], false),
        12.0,
    );
    assert_optional_close(
        "terminal transition with actions; input: q_learning_target(2.0, 0.9, &[10.0], true)",
        q_learning_target(2.0, 0.9, &[10.0], true),
        2.0,
    );
    assert_optional_close(
        "terminal transition without actions; input: q_learning_target(-2.0, 0.9, &[], true)",
        q_learning_target(-2.0, 0.9, &[], true),
        -2.0,
    );
    let actual = q_learning_target(2.0, 0.9, &[], false);
    assert_eq!(actual, None, "missing continuing action set; input: reward=2, gamma=0.9, next_values=[], terminated=false: expected None, actual {actual:?}");
    let actual = q_learning_target(2.0, 0.0, &[], false);
    assert_eq!(actual, None, "missing continuing action set; input: reward=2, gamma=0.0, next_values=[], terminated=false: expected None, actual {actual:?}");
}

pub(super) fn exercise_10(bellman_backup: BellmanBackup) {
    assert_close(
        "empty Bellman sum; input: bellman_backup(&[], 0.9)",
        bellman_backup(&[], 0.9),
        0.0,
    );
    assert_close(
        "certain continuation; input: bellman_backup(&[(1.0, 2.0, 10.0)], 0.9)",
        bellman_backup(&[(1.0, 2.0, 10.0)], 0.9),
        11.0,
    );
    assert_close(
        "immediate reward at zero discount; input: bellman_backup(&[(1.0, 2.0, 10.0)], 0.0)",
        bellman_backup(&[(1.0, 2.0, 10.0)], 0.0),
        2.0,
    );
    let branches = [(0.75, 8.0, 0.0), (0.25, -2.0, 4.0)];
    assert_close("mixed branches at zero discount; branches=[(0.75, 8, 0), (0.25, -2, 4)]; input: bellman_backup(&branches, 0.0)", bellman_backup(&branches, 0.0), 5.5);
    assert_close("mixed branches at half discount; branches=[(0.75, 8, 0), (0.25, -2, 4)]; input: bellman_backup(&branches, 0.5)", bellman_backup(&branches, 0.5), 6.0);
    assert_close("mixed branches at unit discount; branches=[(0.75, 8, 0), (0.25, -2, 4)]; input: bellman_backup(&branches, 1.0)", bellman_backup(&branches, 1.0), 6.5);
    assert_close("reordered branches; branches=[(0.75, 8, 0), (0.25, -2, 4)]; input: bellman_backup(&[branches[1], branches[0]], 0.5)", bellman_backup(&[branches[1], branches[0]], 0.5), 6.0);
    assert_close(
        "two continuing outcomes; input: bellman_backup(&[(0.5, 2.0, 4.0), (0.5, -2.0, 8.0)], 0.5)",
        bellman_backup(&[(0.5, 2.0, 4.0), (0.5, -2.0, 8.0)], 0.5),
        3.0,
    );
    assert_close(
        "zero-probability branch; input: outcomes=[(0, 100, 100), (1, -2, 0)], gamma=1",
        bellman_backup(&[(0.0, 100.0, 100.0), (1.0, -2.0, 0.0)], 1.0),
        -2.0,
    );
}
