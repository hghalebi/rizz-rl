//! SPOILERS: runnable demonstrations using the separate reference solutions.
//! Select one concept with its exact name; guides/EXERCISES.md lists commands.
//! `demonstrations` deliberately runs all ten reference examples.

use super::solutions;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() < 1e-10,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn discounted_return() {
    println!("SPOILERS: reference demonstration for discounted_return.");
    let discounted = solutions::discounted_return(&[1.0, 2.0, 4.0], 0.5);
    println!("1. Discounted return: [1, 2, 4], gamma=0.5 -> {discounted:.3}");
    assert_close(discounted, 3.0);
}

#[test]
fn weighted_expectation() {
    println!("SPOILERS: reference demonstration for weighted_expectation.");
    let expectation = solutions::weighted_expectation(&[(8.0, 0.75), (-2.0, 0.25)]);
    println!("2. Expectation: 75% of 8 plus 25% of -2 -> {expectation:.3}");
    assert_close(expectation, 5.5);
}

#[test]
fn td_target() {
    println!("SPOILERS: reference demonstration for td_target.");
    let continuing = solutions::td_target(2.0, 0.9, 10.0, false);
    let terminal = solutions::td_target(2.0, 0.9, 10.0, true);
    println!(
        "3. TD target: reward=2, gamma=0.9, next value=10 -> continuing {continuing:.3}, terminal {terminal:.3}"
    );
    assert_close(continuing, 11.0);
    assert_close(terminal, 2.0);
}

#[test]
fn probability_validation() {
    println!("SPOILERS: reference demonstration for probability_validation.");
    let valid = solutions::is_probability_distribution(&[0.7, 0.3]);
    let invalid = solutions::is_probability_distribution(&[0.8, 0.3]);
    println!("4. Probability validation: [0.7, 0.3] -> {valid}; [0.8, 0.3] -> {invalid}");
    assert!(valid);
    assert!(!invalid);
}

#[test]
fn greedy_action() {
    println!("SPOILERS: reference demonstration for greedy_action.");
    let greedy = solutions::greedy_action(&[-4.0, -2.0, -2.0]);
    println!(
        "5. Greedy action: values [-4, -2, -2] -> {greedy:?} (zero-based index; first tie wins)"
    );
    assert_eq!(greedy, Some(1));
}

#[test]
fn epsilon_greedy() {
    println!("SPOILERS: reference demonstration for epsilon_greedy.");
    let probabilities = solutions::epsilon_greedy(&[1.0, 4.0, 2.0], 0.3);
    println!("6. Epsilon-greedy: values [1, 4, 2], epsilon=0.3 -> {probabilities:.3?}");
    assert_eq!(probabilities.len(), 3);
    for (&actual, expected) in probabilities.iter().zip([0.1, 0.8, 0.1]) {
        assert_close(actual, expected);
    }
}

#[test]
fn update_mean() {
    println!("SPOILERS: reference demonstration for update_mean.");
    let mean = solutions::update_mean(2.0, 6.0, 4);
    println!("7. Running mean: first 3 returns average 2; fourth return is 6 -> {mean:.3}");
    assert_close(mean, 3.0);
}

#[test]
fn td_update() {
    println!("SPOILERS: reference demonstration for td_update.");
    let updated = solutions::td_update(4.0, 2.0, 0.9, 10.0, false, 0.2);
    println!("8. TD update: current=4, target=2+0.9*10=11, alpha=0.2 -> {updated:.3}");
    assert_close(updated, 5.4);
}

#[test]
fn q_learning_target() {
    println!("SPOILERS: reference demonstration for q_learning_target.");
    let q_target = solutions::q_learning_target(1.0, 0.5, &[-4.0, -2.0], false);
    println!("9. Q-learning target: reward=1, gamma=0.5, next values [-4, -2] -> {q_target:.3?}");
    assert_close(
        q_target.expect("continuing state has available actions"),
        0.0,
    );
}

#[test]
fn bellman_backup() {
    println!("SPOILERS: reference demonstration for bellman_backup.");
    let backup = solutions::bellman_backup(&[(0.5, 2.0, 10.0), (0.5, -2.0, 0.0)], 0.9);
    println!(
        "10. Bellman backup: equally likely (reward=2,next=10) or (reward=-2,next=0), gamma=0.9 -> {backup:.3}"
    );
    assert_close(backup, 4.5);
}

#[test]
fn demonstrations() {
    println!("SPOILERS: all ten arithmetic reference demonstrations.\n");
    discounted_return();
    weighted_expectation();
    td_target();
    probability_validation();
    greedy_action();
    epsilon_greedy();
    update_mean();
    td_update();
    q_learning_target();
    bellman_backup();
    println!("\nExplain one result without looking at the code, then try different inputs.");
}
