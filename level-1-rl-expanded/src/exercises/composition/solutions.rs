//! SPOILERS. Reference implementations for the separate practice file.
//! Attempt a task first if you want evidence of independent understanding.

use super::{
    cases, AssociationResults, BindIdentityResults, Distribution, IntegerBranch, IntegerMap,
    MapResults, StringBranch,
};

fn complete_ownership(input: Distribution<String>) -> (usize, Distribution<usize>) {
    // number_of_outcomes borrows input; map subsequently consumes it.
    let branch_count = input.number_of_outcomes();
    let lengths = input.map(|label| label.len());
    (branch_count, lengths)
}

fn complete_branching(input: Distribution<String>, branch: StringBranch) -> Distribution<String> {
    // and_then multiplies the input weight by each conditional branch weight.
    input.and_then(branch)
}

fn independent_pipeline(
    choices: Distribution<String>,
    respond: StringBranch,
) -> Distribution<usize> {
    choices.and_then(respond).map(|response| response.len())
}

fn navigation_pipeline(routes: Distribution<String>, travel: StringBranch) -> Distribution<bool> {
    routes
        .and_then(travel)
        .map(|destination| destination == "goal")
}

fn construct_map_laws(
    input: Distribution<i32>,
    first: IntegerMap,
    second: IntegerMap,
) -> MapResults {
    // Independent experiments each consume their own input. Clone for that
    // specific reason, not because map requires cloning each element.
    MapResults {
        identity: input.clone().map(|value| value),
        sequential: input.clone().map(first).map(second),
        composed: input.map(|value| second(first(value))),
    }
}

fn construct_bind_identities(
    input: Distribution<i32>,
    value: i32,
    branch: IntegerBranch,
) -> BindIdentityResults {
    BindIdentityResults {
        left_identity: Distribution::certain(value).and_then(branch),
        right_identity: input.and_then(Distribution::certain),
    }
}

fn construct_associativity(
    input: Distribution<i32>,
    first: IntegerBranch,
    second: IntegerBranch,
) -> AssociationResults {
    AssociationResults {
        sequential: input.clone().and_then(first).and_then(second),
        regrouped: input.and_then(|value| first(value).and_then(second)),
    }
}

#[test]
fn ownership_completion() {
    cases::ownership(complete_ownership);
}
#[test]
fn branching_completion() {
    cases::branching(complete_branching);
}
#[test]
fn independent() {
    cases::independent(independent_pipeline);
}
#[test]
fn transfer_navigation() {
    cases::navigation(navigation_pipeline);
}
#[test]
fn map_laws() {
    cases::map_laws(construct_map_laws);
}
#[test]
fn bind_identities() {
    cases::bind_identities(construct_bind_identities);
}
#[test]
fn associativity() {
    cases::associativity(construct_associativity);
}
