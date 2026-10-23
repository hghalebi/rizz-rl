//! Checkpoint 11: the value table and `observe`, one TD(0) update per observed transition.
//!
//! Question: how does one observed transition correct one stored estimate?
//! Example: estimate 4, target 10, fourth visit: 4 + (1/4) * (10 - 4) = 5.5.
//! Predict: do (h = 2, JustMet) and (h = 1, JustMet) share one visit count?

use std::collections::HashMap;

use super::config::Discount;
use super::LearningError;
use crate::model::{State, Transition};
use crate::MAX_HORIZON;

// Engaged with one step left and Engaged with five steps left ask different
// questions. Both the estimate and its observation count belong to (h, state),
// which is why the table is keyed by the pair rather than by the state alone.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ValueEntry {
    pub(super) value: f64,
    pub(super) visits: usize,
}

#[derive(Debug, PartialEq)]
pub(super) struct ValueTable {
    discount: Discount,
    pub(super) entries: HashMap<(State, usize), ValueEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Update {
    pub(super) previous: f64,
    pub(super) bootstrap: f64,
    pub(super) target: f64,
    pub(super) alpha: f64,
    pub(super) visits: usize,
    pub(super) value: f64,
}

impl ValueTable {
    pub(super) fn new(discount: Discount) -> Self {
        Self {
            discount,
            entries: HashMap::new(),
        }
    }

    pub(super) fn entry(&self, state: &State, steps_remaining: usize) -> Option<&ValueEntry> {
        self.entries.get(&(*state, steps_remaining))
    }

    pub(super) fn value(&self, state: &State, steps_remaining: usize) -> f64 {
        if steps_remaining == 0 || state.is_terminal() {
            return 0.0;
        }
        // Zero is the declared initial estimate, not evidence of having learned.
        self.entry(state, steps_remaining)
            .map_or(0.0, |entry| entry.value)
    }

    /// Move this entry toward a one-transition target, not a full episode return.
    /// A prediction of 4 with target 10 on its fourth visit becomes
    /// 4 + (1/4)*(10 - 4) = 5.5. Other entries keep their own counts.
    /// The table is borrowed mutably; the observation is only read, not rewritten.
    /// Predict: is the first visit to a different entry also its fourth update?
    pub(super) fn observe(
        &mut self,
        steps_remaining: usize,
        observation: &Transition,
    ) -> Result<Update, LearningError> {
        if !(1..=MAX_HORIZON).contains(&steps_remaining) {
            return Err(LearningError::RemainingHorizonOutOfRange);
        }
        if observation.from_state.is_terminal() {
            return Err(LearningError::LearnFromTerminal);
        }
        if !observation.reward.is_finite() {
            return Err(LearningError::NonFiniteReward);
        }
        let bootstrap = self.value(&observation.to_state, steps_remaining - 1);
        // In the supplied trace, reward 1 plus 0.5 * an estimated next value of 4
        // gives target 3. That next value is today's estimate, not a known future.
        let target = observation.reward + self.discount.0 * bootstrap;
        let previous = self.value(&observation.from_state, steps_remaining);
        let visits = self
            .entry(&observation.from_state, steps_remaining)
            .map_or(0, |entry| entry.visits)
            .checked_add(1)
            .ok_or(LearningError::VisitCountOverflow)?;
        // This count belongs to one (h, state), not to the whole training run.
        // With 1/n steps, this entry averages the targets it has seen. Those
        // targets contain changing estimates, not complete observed returns.
        let alpha = 1.0 / visits as f64;
        let value = previous + alpha * (target - previous);
        if !target.is_finite() || !value.is_finite() {
            return Err(LearningError::NonFiniteUpdate);
        }
        // Every check has passed, so this single write is the only mutation.
        self.entries.insert(
            (observation.from_state, steps_remaining),
            ValueEntry { value, visits },
        );
        Ok(Update {
            previous,
            bootstrap,
            target,
            alpha,
            visits,
            value,
        })
    }
}
