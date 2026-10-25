//! EXERCISE ONLY. Implement one function; other TODOs may remain untouched.
//! Read `guides/COMPOSITION_LESSON.md` for contracts, routes, and optional hints.
//! Shared cases contain expected outputs; solutions.rs contains worked answers.
//! Add your own cases in the marked test wrapper for your current task. Keep
//! its shared-case call: your examples supplement the acceptance checks.
//! Filtering selects execution; syntax or type errors elsewhere in this test
//! target still prevent compilation. Untouched TODOs remain compilable.

use super::{
    cases, AssociationResults, BindIdentityResults, Distribution, IntegerBranch, IntegerMap,
    MapResults, StringBranch,
};

// Completion A: inspect the branch count before consuming the distribution.
// Complete only the transformation from one owned String to its byte length.
fn complete_ownership(input: Distribution<String>) -> (usize, Distribution<usize>) {
    let branch_count = input.number_of_outcomes();
    let lengths = input.map(|_label| todo!("Return this label's byte length"));
    (branch_count, lengths)
}

// Completion B: the supplied branch function maps one choice to its responses.
// Complete the closure body without discarding branch probabilities.
fn complete_branching(input: Distribution<String>, _branch: StringBranch) -> Distribution<String> {
    input.and_then(|_choice| todo!("Continue this choice through the supplied branch"))
}

// Independent: apply the supplied response model to the choices, then return
// the distribution of response label byte lengths. Use the actual map/and_then.
fn independent_pipeline(
    _choices: Distribution<String>,
    _respond: StringBranch,
) -> Distribution<usize> {
    todo!("Construct the response-length distribution")
}

// Changed context: travel maps a route to possible destination labels.
// Return a distribution of whether the destination equals "goal".
fn navigation_pipeline(_routes: Distribution<String>, _travel: StringBranch) -> Distribution<bool> {
    todo!("Construct the goal-arrival distribution")
}

// Construct three results using map: identity, first then second in separate
// maps, and the same two transformations in one map. Input numbers are small.
fn construct_map_laws(
    _input: Distribution<i32>,
    _first: IntegerMap,
    _second: IntegerMap,
) -> MapResults {
    todo!("Construct all three map results; each experiment needs its own input")
}

// Left: start with certain(value), then use branch. Right: start with input,
// then add a certain continuation for each outcome. Return both results.
fn construct_bind_identities(
    _input: Distribution<i32>,
    _value: i32,
    _branch: IntegerBranch,
) -> BindIdentityResults {
    todo!("Construct the left and right identity experiments")
}

// Construct both groupings: complete first and then second sequentially;
// or make one continuation that performs first followed by second.
fn construct_associativity(
    _input: Distribution<i32>,
    _first: IntegerBranch,
    _second: IntegerBranch,
) -> AssociationResults {
    todo!("Construct both parenthesizations without changing either branch function")
}

#[test]
#[ignore = "EXERCISE ONLY: complete the ownership transformation"]
fn ownership_completion() {
    cases::ownership(complete_ownership);
    // YOUR CASE: predict byte lengths for a new label distribution, then call
    // complete_ownership and compare the branch count and result probabilities.
}

#[test]
#[ignore = "EXERCISE ONLY: complete the probability continuation"]
fn branching_completion() {
    cases::branching(complete_branching);
    // YOUR CASE: supply new choice weights to complete_branching with a branch
    // function of your own. Predict one complete path before checking an event.
}

#[test]
#[ignore = "EXERCISE ONLY: attempt the independent pipeline"]
fn independent() {
    cases::independent(independent_pipeline);
    // YOUR CASE: call independent_pipeline with new choice weights and your own
    // response function. Predict the resulting byte-length probabilities first.
}

#[test]
#[ignore = "EXERCISE ONLY: attempt the changed-context transfer check"]
fn transfer_navigation() {
    cases::navigation(navigation_pipeline);
    // YOUR CASE: define new route weights and a travel function here. Predict
    // the goal probability before calling navigation_pipeline and asserting it.
}

#[test]
#[ignore = "EXERCISE ONLY: construct the map law experiments"]
fn map_laws() {
    cases::map_laws(construct_map_laws);
    // YOUR CASE: define a small input and two integer transforms here. Compare
    // the result fields by event probability, not by their stored Vec order.
}

#[test]
#[ignore = "EXERCISE ONLY: construct the bind identity experiments"]
fn bind_identities() {
    cases::bind_identities(construct_bind_identities);
    // YOUR CASE: choose a new input, starting integer, and branch function.
    // Predict both identity results before checking their event probabilities.
}

#[test]
#[ignore = "EXERCISE ONLY: construct the associativity experiments"]
fn associativity() {
    cases::associativity(construct_associativity);
    // YOUR CASE: choose a new input and two branch functions. Keep their order
    // fixed, then predict and compare the probabilities under both groupings.
}
