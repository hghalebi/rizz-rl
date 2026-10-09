//! WORKED EXAMPLES. Run the smallest relevant example after your prediction.
//! These use the same private Distribution as the planner, not a teaching copy.

use super::{cases, Distribution};

#[test]
fn probability_probe() {
    let joint_probability: f64 = 0.7 * 0.8;
    println!("Observed: P(ask AND engaged) = {joint_probability:.2}");
    let responses = cases::choices().and_then(cases::response);
    println!(
        "Observed: P(engaged through either choice) = {:.2}",
        responses.probability_of(&"engaged".into())
    );
    assert!((joint_probability - 0.56).abs() < 1e-9);
    assert!((responses.probability_of(&"engaged".into()) - 0.74).abs() < 1e-9);
}

#[test]
fn ownership_demo() {
    let labels = Distribution {
        outcomes: vec![(String::from("tea"), 0.4), (String::from("coffee"), 0.6)],
    };
    // Inspect: this borrows labels, so it remains available afterward.
    println!("Before mapping: {} branches", labels.number_of_outcomes());
    // Transform: map consumes labels and gives the closure each owned String.
    let lengths = labels.map(|label| label.len());
    // labels is no longer available here; lengths is the new owner of results.
    println!(
        "Byte lengths and unchanged probabilities: {:?}",
        lengths.outcomes
    );
    cases::assert_same_events(
        &lengths,
        &Distribution {
            outcomes: vec![(3, 0.4), (6, 0.6)],
        },
    );
}

#[test]
fn branching_demo() {
    // Branch: response returns a distribution conditional on one choice.
    let responses = cases::choices().and_then(cases::response);
    println!("Complete branches: {:?}", responses.outcomes);
    println!("Several branches may describe the same event; sum their weights.");
    cases::assert_same_events(
        &responses,
        &Distribution {
            outcomes: vec![("engaged".into(), 0.74), ("quiet".into(), 0.26)],
        },
    );
}

#[test]
fn combined_demo() {
    let choices = Distribution {
        outcomes: vec![(String::from("ASK"), 0.7), (String::from("STORY"), 0.3)],
    };
    let lengths = choices
        .map(|choice| choice.to_lowercase()) // Transform each owned outcome.
        .and_then(cases::response) // Branch and weight each complete path.
        .map(|response| response.len()); // Transform responses to byte lengths.
    println!(
        "Response byte lengths and path probabilities: {:?}",
        lengths.outcomes
    );
    println!("These lengths demonstrate a Rust transformation; they are not RL rewards.");
    cases::assert_same_events(
        &lengths,
        &Distribution {
            outcomes: vec![(7, 0.74), (5, 0.26)],
        },
    );
}
