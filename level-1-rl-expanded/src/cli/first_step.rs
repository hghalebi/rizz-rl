//! The first-step experiment: one decision, its branches, and their chances.
//!
//! Question: from JustMet, how likely is each (action, next state) pair, and what is
//! the average immediate reward?
//! Example: Ask (0.70) then Engaged given Ask (0.80) is one branch with chance 0.56.
//! Predict: what do the four branch chances sum to? Does boldness change any of them?

use std::io::{self, Write};

use super::command::default_policy;
use crate::model::{DatingWorld, State, Transition};
use crate::planning::possible_transitions;
use crate::probability::{expected_value, Distribution};

// A concrete one-step worked example.
//
// From JustMet:
//
//     P(Ask)   = 0.70
//     P(Story) = 0.30
//
// The world then branches conditionally:
//
//     P(Engaged | Ask)      = 0.80
//     P(LowInterest | Ask)  = 0.20
//     P(Engaged | Story)    = 0.60
//     P(LowInterest | Story)= 0.40
//
// `and_then` combines them into joint path probabilities:
//
//     P(Ask and Engaged) = 0.70 × 0.80 = 0.56
//
// This function prints the same calculation from the real program data.
fn print_first_step_walkthrough(
    out: &mut impl Write,
    transitions: &Distribution<Transition>,
) -> io::Result<()> {
    writeln!(
        out,
        r#"
----------------------------------------------------------------
FIRST-STEP PROBABILITY CHECKPOINT
----------------------------------------------------------------
The policy chooses an action.
The world responds conditionally to that action.
A complete branch probability is:

P(action and next_state)
=
P(action | state) * P(next_state | state, action)
----------------------------------------------------------------
"#
    )?;

    for (transition, probability) in &transitions.outcomes {
        writeln!(
            out,
            "{:>6.1}%  {:?} --{:?} / {:+.1}--> {:?}",
            probability * 100.0,
            transition.from_state,
            transition.action,
            transition.reward,
            transition.to_state,
        )?;
    }

    writeln!(
        out,
        "Total probability mass: {:.6}\n",
        transitions.total_probability(),
    )?;
    Ok(())
}

pub(super) fn print(out: &mut impl Write, world: &DatingWorld) -> io::Result<()> {
    let policy = default_policy();
    let transitions = possible_transitions(&policy, world, &State::JustMet);
    print_first_step_walkthrough(out, &transitions)?;
    let immediate_reward = expected_value(&transitions, |transition| transition.reward);
    writeln!(
        out,
        "Expected immediate reward = sum(branch probability * reward) = {immediate_reward:.6}"
    )?;
    writeln!(
        out,
        "Boldness has no effect on the first step: JustMet always uses Ask=0.7, Story=0.3."
    )?;
    writeln!(
        out,
        "Next experiment: evaluate --horizon 1, then evaluate --horizon 2."
    )?;
    Ok(())
}
