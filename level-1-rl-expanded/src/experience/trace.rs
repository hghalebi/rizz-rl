//! The td-trace lesson: three supplied observations and the updates they cause.
//!
//! Question: what happens inside three hand-picked updates, with every number visible?
//! Example: reward 1 plus 0.5 times a stored next value of 4 gives target 3.
//! Predict: the second update to (h = 2, JustMet) uses which learning rate?

use std::fmt;

use super::config::Discount;
use super::table::{Update, ValueTable};
use super::LearningError;
use crate::model::{Action, State, Transition};

/// The discount every supplied update uses.
const TRACE_DISCOUNT: f64 = 0.5;

/// One supplied observation, its remaining horizon, and the update it produced.
struct TraceStep {
    steps_remaining: usize,
    observation: Transition,
    update: Update,
}

/// The supplied TD trace: computed once by `trace`, formatted by `Display`.
pub(crate) struct TdTrace {
    steps: Vec<TraceStep>,
}

/// Apply three supplied observations to an empty table and keep every update.
/// `(h, from, reward, to) ──observe──► Update`, in the order listed
/// Invariant: the rewards are supplied, not drawn from DatingWorld; the update rule is training's.
pub(crate) fn trace() -> Result<TdTrace, LearningError> {
    let mut table = ValueTable::new(Discount(TRACE_DISCOUNT));
    let observations = [
        (1, State::Engaged, 4.0, State::Goodbye),
        (2, State::JustMet, 1.0, State::Engaged),
        (2, State::JustMet, -1.0, State::Engaged),
    ];
    let steps = observations
        .into_iter()
        .map(
            |(steps_remaining, from_state, reward, to_state)| -> Result<TraceStep, LearningError> {
                let observation = Transition {
                    from_state,
                    action: Action::Ask,
                    reward,
                    to_state,
                };
                let update = table.observe(steps_remaining, &observation)?;
                Ok(TraceStep {
                    steps_remaining,
                    observation,
                    update,
                })
            },
        )
        // Collecting into a `Result` stops at the first refused update and returns its error.
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TdTrace { steps })
}

impl fmt::Display for TdTrace {
    /// Print the header, one line per update, then the boundary rule.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Supplied TD trace, discount={TRACE_DISCOUNT}, all estimates initially zero."
        )?;
        writeln!(
            f,
            "These are separate update records, not one world-generated episode."
        )?;
        for TraceStep {
            steps_remaining: h,
            observation,
            update,
        } in &self.steps
        {
            writeln!(
                f,
                "h={h} {:?} -> {:?}: reward={:.6} bootstrap={:.6} target={:.6} visits={} alpha={:.6} previous={:.6} value={:.6}",
                observation.from_state, observation.to_state, observation.reward,
                update.bootstrap, update.target, update.visits, update.alpha,
                update.previous, update.value,
            )?;
        }
        writeln!(
            f,
            "A terminal next state or h-1=0 gives zero bootstrap. The arriving reward still counts."
        )
    }
}
