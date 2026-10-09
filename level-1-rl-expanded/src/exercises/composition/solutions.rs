//! SPOILERS. Reference implementations for the separate practice file.
//! Attempt a task first if you want evidence of independent understanding.
//! Trace one owned value and one branch probability before reading the full chain.
//! The law examples use pure functions and compare aggregated event probabilities
//! within a tolerance, not Vec layout. They do not prove laws for all Rust closures.

use super::{
    cases, AssociationResults, BindIdentityResults, Distribution, IntegerBranch, IntegerMap,
    MapResults, StringBranch,
};

// Mapping "ask" with probability 0.7 yields its byte length 3 with probability 0.7.
// We change what each outcome contains, while leaving its branch weight alone.
// Predict: why would moving the branch-count inspection below map fail to compile?
fn complete_ownership(input: Distribution<String>) -> (usize, Distribution<usize>) {
    // number_of_outcomes borrows input; map subsequently consumes it.
    let branch_count = input.number_of_outcomes();
    let lengths = input.map(|label| label.len());
    (branch_count, lengths)
}

// A 0.7 chance of "ask", then a 0.8 conditional chance of "engaged", makes a
// complete path of weight 0.56. The conditional chance alone misses the first step.
// `map(branch)` would leave a distribution inside each outcome; bind continues
// those inner branches and produces one distribution of responses.
fn complete_branching(input: Distribution<String>, branch: StringBranch) -> Distribution<String> {
    // and_then multiplies the input weight by each conditional branch weight.
    input.and_then(branch)
}

// First obtain possible responses; only then ask for each response's byte length.
// "engaged" becomes 7, while the probability of reaching that response is retained.
// The chain passes owned Strings through the supplied function and the closure.
// Paths with length 7 describe the same event; its probability sums their weights.
fn independent_pipeline(
    choices: Distribution<String>,
    respond: StringBranch,
) -> Distribution<usize> {
    choices.and_then(respond).map(|response| response.len())
}

// North contributes 0.6 * 0.75 = 0.45 to reaching goal; south adds 0.4 * 0.25 = 0.10.
// Mapping destinations to booleans asks one question of every possible arrival.
// It preserves each path's weight; the total probability of true is 0.55 even
// though the Vec can still contain several separate true branches.
fn navigation_pipeline(routes: Distribution<String>, travel: StringBranch) -> Distribution<bool> {
    routes
        .and_then(travel)
        .map(|destination| destination == "goal")
}

// Identity keeps each outcome. With value 2, adding 1 then doubling gives 6
// whether expressed as two maps or one composed transformation.
// The order still matters: doubling first and then adding 1 would give 5.
// Each map carries branch weights unchanged, so compare the resulting events.
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

// Certainty adds no extra uncertainty: 1 * 0.25 and 0.25 * 1 both remain 0.25.
// On the left, a certain starting value feeds the branch function. On the right,
// each existing outcome receives a single certain continuation of itself.
// The two fields check different identities; they need not equal each other.
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

// A path weighted 0.5, then 0.25, then 0.4 has weight 0.05 under either grouping.
// The closure groups the two continuations together without reversing their order.
// Comparing event probabilities allows duplicate branches and rounding differences;
// regrouping pure computations is the claim, not that every Vec must look identical.
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
