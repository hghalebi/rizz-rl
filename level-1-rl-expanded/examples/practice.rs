//! EXERCISE ONLY: three small Rust tasks, with no solutions included.
//!
//! Work on one function at a time. Predict each test result before coding.
//! Run this example for commands. Tests are ignored until you select them;
//! an unfinished `todo!()` should fail the selected exercise, not normal checks.
//! Inputs are finite, discount is in [0, 1], and supplied probability
//! distributions are normalized unless empty. Input validation is out of scope.

/// Exercise 1: score a sequence of rewards with discounting.
/// The first reward is immediate. An empty sequence has return zero.
/// Rust focus: borrowing a slice and returning an f64 without changing inputs.
fn discounted_return(_rewards: &[f64], _discount: f64) -> f64 {
    todo!("Exercise 1: implement discounted return")
}

/// Exercise 2: compute expected value from (value, probability) pairs.
/// An empty distribution has weighted sum zero for this exercise.
/// Rust focus: tuple fields and iterating over borrowed values.
fn weighted_expectation(_outcomes: &[(f64, f64)]) -> f64 {
    todo!("Exercise 2: implement weighted expectation")
}

/// Exercise 3: calculate a one-step temporal-difference target.
/// `next_value` is an existing estimate, not an immediate reward.
/// A true task termination disables bootstrapping from that estimate.
/// Stopping observation only because of a time limit uses `terminated = false`
/// when the underlying task continues. This differs from a finite task whose
/// deadline really ends its objective. This function computes no value update.
/// Rust focus: express these two cases explicitly with a boolean condition.
fn td_target(_reward: f64, _discount: f64, _next_value: f64, _terminated: bool) -> f64 {
    todo!("Exercise 3: implement the temporal-difference target")
}

fn main() {
    // References keep the starter warning-free; the functions are never called here.
    let _ = (discounted_return, weighted_expectation, td_target);
    println!(
        "EXERCISE ONLY: edit examples/practice.rs.\n\
         Replace one todo!() at a time. Underscore-prefixed parameters are normal\n\
         named inputs; you can remove the underscores when implementing them.\n\n\
         1. cargo test --example practice exercise_1 -- --ignored\n\
         2. cargo test --example practice exercise_2 -- --ignored\n\
         3. cargo test --example practice exercise_3 -- --ignored\n\n\
         After all three: cargo test --example practice -- --ignored\n\
         Before implementation these selected tests intentionally fail.\n\
         Rate your confidence from 1 to 5 and explain one result in your own words."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-10,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    #[ignore = "EXERCISE ONLY: implement discounted_return, then run with --ignored"]
    fn exercise_1_discounted_return() {
        assert_close(discounted_return(&[], 0.9), 0.0);
        assert_close(discounted_return(&[3.0], 0.5), 3.0);
        assert_close(discounted_return(&[1.0, 2.0, 4.0], 0.5), 3.0);
        assert_close(discounted_return(&[1.0, 2.0, 4.0], 0.0), 1.0);
        assert_close(discounted_return(&[1.0, 2.0, 4.0], 1.0), 7.0);
        assert_close(discounted_return(&[-2.0, 2.0], 0.5), -1.0);
    }

    #[test]
    #[ignore = "EXERCISE ONLY: implement weighted_expectation, then run with --ignored"]
    fn exercise_2_weighted_expectation() {
        assert_close(weighted_expectation(&[]), 0.0);
        assert_close(weighted_expectation(&[(7.0, 1.0)]), 7.0);
        assert_close(weighted_expectation(&[(8.0, 0.75), (-2.0, 0.25)]), 5.5);
        assert_close(weighted_expectation(&[(-2.0, 0.25), (8.0, 0.75)]), 5.5);
        assert_close(weighted_expectation(&[(100.0, 0.0), (-3.0, 1.0)]), -3.0);
    }

    #[test]
    #[ignore = "EXERCISE ONLY: implement td_target, then run with --ignored"]
    fn exercise_3_td_target() {
        assert_close(td_target(2.0, 0.9, 10.0, false), 11.0);
        assert_close(td_target(2.0, 0.9, 10.0, true), 2.0);
        // Same observation at a time limit in a continuing task still bootstraps.
        let terminated = false;
        assert_close(td_target(2.0, 0.9, 10.0, terminated), 11.0);
        assert_close(td_target(-2.0, 0.0, 10.0, false), -2.0);
        assert_close(td_target(-2.0, 1.0, 10.0, false), 8.0);
        assert_close(td_target(-2.0, 1.0, 10.0, true), -2.0);
    }
}
