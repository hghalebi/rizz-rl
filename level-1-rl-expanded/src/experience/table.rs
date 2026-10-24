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

/// One estimate and how many observations have shaped it so far.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ValueEntry {
    pub(super) value: f64,
    pub(super) visits: usize,
}

/// Estimates keyed by the whole question: which state, and how many steps are left.
/// `(state, steps_remaining) ──► ValueEntry`
/// Invariant: Engaged with one step left and Engaged with five are two separate entries.
#[derive(Debug, PartialEq)]
pub(super) struct ValueTable {
    discount: Discount,
    pub(super) entries: HashMap<(State, usize), ValueEntry>,
}

/// Every number from one TD(0) update, kept so a lesson can print the arithmetic.
/// `target = reward + γ · bootstrap`,  `value = previous + alpha · (target - previous)`
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
    /// An empty table: every estimate starts at 0 with 0 visits.
    pub(super) fn new(discount: Discount) -> Self {
        Self {
            discount,
            entries: HashMap::new(),
        }
    }

    /// The stored entry for (state, steps remaining), if that entry was ever visited.
    pub(super) fn entry(&self, state: &State, steps_remaining: usize) -> Option<&ValueEntry> {
        self.entries.get(&(*state, steps_remaining))
    }

    /// The current estimate for (state, steps remaining).
    /// `V(0, s) = V(h, terminal) = 0`; otherwise the stored value, or the initial 0 if unvisited
    /// Invariant: an unvisited 0 is the declared starting point, not evidence of learning.
    pub(super) fn value(&self, state: &State, steps_remaining: usize) -> f64 {
        if steps_remaining == 0 || state.is_terminal() {
            return 0.0;
        }
        self.entry(state, steps_remaining)
            .map_or(0.0, |entry| entry.value)
    }

    /// Move one entry toward a one-transition target, using that entry's own visit count.
    /// `target = r + γ · V(h-1, s')`,  `alpha = 1 / visits`,  `value' = value + alpha · (target - value)`
    /// Invariant: every check runs before the single write, so a refused update changes nothing.
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
        let Discount(discount) = self.discount;
        let bootstrap = self.value(&observation.to_state, steps_remaining - 1);
        let target = observation.reward + discount * bootstrap;
        let previous = self.value(&observation.from_state, steps_remaining);
        let visits = self
            .entry(&observation.from_state, steps_remaining)
            .map_or(0, |entry| entry.visits)
            .checked_add(1)
            .ok_or(LearningError::VisitCountOverflow)?;
        let alpha = 1.0 / visits as f64;
        let value = previous + alpha * (target - previous);
        if !target.is_finite() || !value.is_finite() {
            return Err(LearningError::NonFiniteUpdate);
        }
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
