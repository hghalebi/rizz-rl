//! Test-only lessons using the planner's actual Distribution type.
//! Start at `guides/COMPOSITION_LESSON.md`; reference answers are in solutions.rs.

use crate::probability::Distribution;

type StringBranch = fn(String) -> Distribution<String>;
type IntegerBranch = fn(i32) -> Distribution<i32>;
type IntegerMap = fn(i32) -> i32;

mod cases;
mod practice;
mod solutions;
mod worked;

struct MapResults {
    identity: Distribution<i32>,
    sequential: Distribution<i32>,
    composed: Distribution<i32>,
}

struct BindIdentityResults {
    left_identity: Distribution<i32>,
    right_identity: Distribution<i32>,
}

struct AssociationResults {
    sequential: Distribution<i32>,
    regrouped: Distribution<i32>,
}
