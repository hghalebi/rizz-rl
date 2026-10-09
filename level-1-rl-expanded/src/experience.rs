//! Learn a finite-horizon value from observations, then compare it with a model.
//!
//! The sampler owns policy/world access. `ValueTable::observe` only receives a
//! transition and its remaining horizon. It never calls the model or evaluator.
//! Each (h, state) entry uses its own visit count and learning rate 1 / visits.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::model::{DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;
use crate::MAX_HORIZON;

pub(super) const MAX_EPISODES: usize = 1_000_000;
const MASS_TOLERANCE: f64 = 1e-12;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Discount(f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct LearningConfig {
    boldness: f64,
    horizon: usize,
    discount: Discount,
    episodes: usize,
    seed: u64,
}

impl LearningConfig {
    pub(super) fn new(
        boldness: f64,
        horizon: usize,
        discount: f64,
        episodes: usize,
        seed: u64,
    ) -> Result<Self, String> {
        for (name, value) in [("boldness", boldness), ("discount", discount)] {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(format!("{name} must be finite and in [0, 1]"));
            }
        }
        if horizon > MAX_HORIZON {
            return Err(format!("horizon must be in [0, {MAX_HORIZON}]"));
        }
        if !(1..=MAX_EPISODES).contains(&episodes) {
            return Err(format!("episodes must be in [1, {MAX_EPISODES}]"));
        }
        Ok(Self {
            boldness,
            horizon,
            discount: Discount(discount),
            episodes,
            seed,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ValueEntry {
    state: State,
    steps_remaining: usize,
    value: f64,
    visits: usize,
}

#[derive(Debug, PartialEq)]
struct ValueTable {
    discount: Discount,
    entries: Vec<ValueEntry>,
}

#[derive(Debug, PartialEq)]
struct Update {
    previous: f64,
    bootstrap: f64,
    target: f64,
    alpha: f64,
    visits: usize,
    value: f64,
}

impl ValueTable {
    fn new(discount: Discount) -> Self {
        Self {
            discount,
            entries: Vec::new(),
        }
    }

    fn entry(&self, state: &State, steps_remaining: usize) -> Option<&ValueEntry> {
        self.entries
            .iter()
            .find(|entry| entry.state == *state && entry.steps_remaining == steps_remaining)
    }

    fn value(&self, state: &State, steps_remaining: usize) -> f64 {
        if steps_remaining == 0 || state.is_terminal() {
            return 0.0;
        }
        // Zero is the declared initial estimate, not evidence of having learned.
        self.entry(state, steps_remaining)
            .map_or(0.0, |entry| entry.value)
    }

    fn observe(
        &mut self,
        steps_remaining: usize,
        observation: &Transition,
    ) -> Result<Update, String> {
        if !(1..=MAX_HORIZON).contains(&steps_remaining) {
            return Err(format!(
                "an observation needs a remaining horizon in [1, {MAX_HORIZON}]"
            ));
        }
        if observation.from_state.is_terminal() {
            return Err("cannot learn an outgoing transition from a terminal state".into());
        }
        if !observation.reward.is_finite() {
            return Err("an observed reward must be finite".into());
        }
        let bootstrap = self.value(&observation.to_state, steps_remaining - 1);
        let target = observation.reward + self.discount.0 * bootstrap;
        let previous = self.value(&observation.from_state, steps_remaining);
        let visits = self
            .entry(&observation.from_state, steps_remaining)
            .map_or(0, |entry| entry.visits)
            .checked_add(1)
            .ok_or("visit count overflow")?;
        let alpha = 1.0 / visits as f64;
        let value = previous + alpha * (target - previous);
        if !target.is_finite() || !value.is_finite() {
            return Err("TD update produced a non-finite value; table was not changed".into());
        }
        if let Some(entry) = self.entries.iter_mut().find(|entry| {
            entry.state == observation.from_state && entry.steps_remaining == steps_remaining
        }) {
            entry.value = value;
            entry.visits = visits;
        } else {
            self.entries.push(ValueEntry {
                state: observation.from_state.clone(),
                steps_remaining,
                value,
                visits,
            });
        }
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

/// Inverse cumulative sampling with a supplied draw makes boundaries testable.
/// Reject malformed distributions before drawing. Scale by their accepted mass
/// to account only for roundoff within MASS_TOLERANCE; do not repair bad models.
fn select_outcome<T>(distribution: &Distribution<T>, draw: f64) -> Result<&T, String> {
    if !draw.is_finite() || !(0.0..1.0).contains(&draw) {
        return Err("a sampling draw must be finite and in [0, 1)".into());
    }
    if distribution.outcomes.is_empty()
        || distribution
            .outcomes
            .iter()
            .any(|(_, p)| !p.is_finite() || !(0.0..=1.0).contains(p))
    {
        return Err("sampling needs nonempty finite probabilities in [0, 1]".into());
    }
    let mass = distribution.total_probability();
    if (mass - 1.0).abs() > MASS_TOLERANCE {
        return Err(format!("sampling probabilities must sum to 1; got {mass}"));
    }
    let threshold = draw * mass;
    let mut cumulative = 0.0;
    let mut last_positive = None;
    for (outcome, probability) in &distribution.outcomes {
        cumulative += probability;
        if *probability > 0.0 {
            last_positive = Some(outcome);
            if threshold < cumulative {
                return Ok(outcome);
            }
        }
    }
    // A valid draw can round to mass during multiplication. Never select a
    // zero-probability final branch as a floating-point boundary fallback.
    last_positive.ok_or_else(|| "sampling found no positive probability".into())
}

fn sample_transition(
    policy: &Policy,
    world: &DatingWorld,
    state: &State,
    action_draw: f64,
    next_state_draw: f64,
) -> Result<Transition, String> {
    if state.is_terminal() {
        return Err("cannot sample an outgoing transition from a terminal state".into());
    }
    let actions = policy.possible_actions(state);
    let action = select_outcome(&actions, action_draw)?;
    let next_states = world.possible_next_states(state, action);
    let next_state = select_outcome(&next_states, next_state_draw)?;
    Ok(Transition {
        from_state: state.clone(),
        action: action.clone(),
        reward: world.reward_for(state, action, next_state),
        to_state: next_state.clone(),
    })
}

fn train(config: &LearningConfig) -> Result<ValueTable, String> {
    let policy = Policy {
        boldness: config.boldness,
    };
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
    let mut table = ValueTable::new(config.discount);
    for _ in 0..config.episodes {
        let mut state = State::JustMet;
        for steps_remaining in (1..=config.horizon).rev() {
            // Sample one action and one consequence, then update online. The
            // later update in this episode cannot retroactively change this one.
            let observation =
                sample_transition(&policy, &DatingWorld, &state, rng.random(), rng.random())?;
            table.observe(steps_remaining, &observation)?;
            state = observation.to_state;
            if state.is_terminal() {
                break;
            }
        }
    }
    Ok(table)
}

pub(super) fn run(config: &LearningConfig) -> Result<String, String> {
    let table = train(config)?;
    // Only comparison code sees exact values; neither the sampler nor the
    // learner is initialized from, or updated using, these reference answers.
    let exact = super::values::evaluate_value(
        &Policy {
            boldness: config.boldness,
        },
        &DatingWorld,
        &State::JustMet,
        config.horizon,
        config.discount.0,
    )
    .expected_return;
    let learned = table.value(&State::JustMet, config.horizon);
    let visits = table
        .entry(&State::JustMet, config.horizon)
        .map_or(0, |entry| entry.visits);
    let mut output = format!(
        "Sampled TD(0) evaluation of a fixed policy.\n\
         Boldness: {} | Horizon: {} | Discount: {} | Episodes: {} | Seed: {}\n\
         Exact V_h(JustMet): {exact:.6}\
         \nLearned V_h(JustMet): {learned:.6}\
         \nAbsolute error: {:.6}\
         \nInitial-state visits: {visits}\n",
        config.boldness,
        config.horizon,
        config.discount.0,
        config.episodes,
        config.seed,
        (learned - exact).abs(),
    );
    if config.horizon == 0 {
        output.push_str("Horizon 0 is a known base case: no transitions or learning updates.\n");
    }
    output.push_str("Table coverage (all nonterminal states, including unreachable entries):\n");
    output.push_str("h state              value    visits status\n");
    for steps_remaining in 1..=config.horizon {
        for state in [
            State::JustMet,
            State::Engaged,
            State::Playful,
            State::LowInterest,
            State::MutualInterest,
        ] {
            let (value, visits, status) = table
                .entry(&state, steps_remaining)
                .map_or((0.0, 0, "unvisited"), |entry| {
                    (entry.value, entry.visits, "visited")
                });
            let state_label = format!("{state:?}");
            output.push_str(&format!(
                "{steps_remaining} {state_label:<14} {value:>10.6} {visits:>9} {status}\n"
            ));
        }
    }
    output.push_str("Terminal and h=0 values are fixed at zero; unvisited entries retain the initial estimate.\n\
        Learning rate = 1 / visits for each (h, state); targets change as later-state estimates change.\n\
        This learns values for a fixed policy, not an improved policy.\n\
        Compare several seeds and episode budgets; individual errors need not decrease.\n");
    Ok(output)
}

pub(super) fn trace() -> String {
    use crate::model::Action;
    let mut table = ValueTable::new(Discount(0.5));
    // These supplied rewards isolate the arithmetic. They are not generated by
    // DatingWorld. Both this trace and sampled training use the same update.
    let observations = [
        (1, State::Engaged, 4.0, State::Goodbye),
        (2, State::JustMet, 1.0, State::Engaged),
        (2, State::JustMet, -1.0, State::Engaged),
    ];
    let mut output = String::from(
        "Supplied TD trace, discount=0.5, all estimates initially zero.\n\
         These are separate update records, not one world-generated episode.\n",
    );
    for (h, from_state, reward, to_state) in observations {
        let observation = Transition {
            from_state,
            action: Action::Ask,
            reward,
            to_state,
        };
        match table.observe(h, &observation) {
            Ok(update) => output.push_str(&format!(
                "h={h} {:?} -> {:?}: reward={reward:.6} bootstrap={:.6} target={:.6} visits={} alpha={:.6} previous={:.6} value={:.6}\n",
                observation.from_state, observation.to_state, update.bootstrap,
                update.target, update.visits, update.alpha, update.previous, update.value,
            )),
            Err(error) => return format!("Supplied trace failed: {error}\n"),
        }
    }
    output.push_str(
        "A terminal next state or h-1=0 gives zero bootstrap. The arriving reward still counts.\n",
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Action;

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    }

    fn observation(from_state: State, reward: f64, to_state: State) -> Transition {
        Transition {
            from_state,
            action: Action::Ask,
            reward,
            to_state,
        }
    }

    #[test]
    fn supplied_trace_uses_bootstrap_and_per_entry_visit_counts() {
        let mut table = ValueTable::new(Discount(0.5));
        let future = observation(State::Engaged, 4.0, State::Goodbye);
        let future_update = table.observe(1, &future).unwrap();
        close(future_update.bootstrap, 0.0);
        close(future_update.value, 4.0);
        let first = table
            .observe(2, &observation(State::JustMet, 1.0, State::Engaged))
            .unwrap();
        close(first.target, 3.0);
        assert_eq!(first.visits, 1);
        let second = table
            .observe(2, &observation(State::JustMet, -1.0, State::Engaged))
            .unwrap();
        close(second.target, 1.0);
        close(second.alpha, 0.5);
        close(second.value, 2.0);
        assert_eq!(second.visits, 2);
        assert_eq!(table.entry(&State::Engaged, 1).unwrap().visits, 1);
    }

    #[test]
    fn horizon_keys_are_distinct_and_last_step_does_not_bootstrap() {
        let mut table = ValueTable::new(Discount(1.0));
        table
            .observe(1, &observation(State::Engaged, 10.0, State::Goodbye))
            .unwrap();
        let update = table
            .observe(1, &observation(State::JustMet, 2.0, State::Engaged))
            .unwrap();
        close(update.value, 2.0);
        close(update.bootstrap, 0.0);
        let longer = table
            .observe(2, &observation(State::JustMet, 2.0, State::Engaged))
            .unwrap();
        close(longer.value, 12.0);
        assert_eq!(longer.visits, 1);
        close(table.value(&State::JustMet, 1), 2.0);
        close(table.value(&State::Engaged, 0), 0.0);
        close(table.value(&State::Goodbye, 3), 0.0);
    }

    #[test]
    fn zero_discount_uses_only_observed_reward() {
        let mut table = ValueTable::new(Discount(0.0));
        table
            .observe(1, &observation(State::Engaged, 100.0, State::Goodbye))
            .unwrap();
        let update = table
            .observe(2, &observation(State::JustMet, -2.0, State::Engaged))
            .unwrap();
        close(update.target, -2.0);
    }

    #[test]
    fn terminal_arrival_keeps_reward_at_any_positive_horizon() {
        let mut table = ValueTable::new(Discount(0.95));
        let update = table
            .observe(6, &observation(State::LowInterest, 2.0, State::Goodbye))
            .unwrap();
        close(update.target, 2.0);
        close(update.bootstrap, 0.0);
    }

    #[test]
    fn invalid_observations_leave_the_table_unchanged() {
        let mut table = ValueTable::new(Discount(0.95));
        let good = observation(State::JustMet, 1.0, State::Engaged);
        for horizon in [0, 7, usize::MAX] {
            assert!(table.observe(horizon, &good).is_err());
        }
        assert!(table
            .observe(1, &observation(State::Goodbye, 1.0, State::Engaged))
            .is_err());
        for reward in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(table
                .observe(1, &observation(State::JustMet, reward, State::Engaged))
                .is_err());
        }
        assert!(table.entries.is_empty());
        table
            .observe(1, &observation(State::Engaged, f64::MAX, State::Goodbye))
            .unwrap();
        assert!(table
            .observe(2, &observation(State::JustMet, f64::MAX, State::Engaged))
            .is_err());
        assert_eq!(table.entries.len(), 1);
    }

    #[test]
    fn sampling_boundaries_skip_zero_mass_and_respect_cumulative_weights() {
        let distribution = Distribution {
            outcomes: vec![(0, 0.0), (1, 0.25), (2, 0.75), (3, 0.0)],
        };
        for (draw, expected) in [(0.0, 1), (0.249, 1), (0.25, 2), (1.0 - f64::EPSILON, 2)] {
            assert_eq!(*select_outcome(&distribution, draw).unwrap(), expected);
        }
        for draw in [-0.1, 1.0, f64::NAN, f64::INFINITY] {
            assert!(select_outcome(&distribution, draw).is_err());
        }
    }

    #[test]
    fn sampling_rejects_malformed_mass_even_when_draw_hits_an_earlier_branch() {
        for probabilities in [
            vec![],
            vec![0.0],
            vec![0.8],
            vec![1.1],
            vec![1.0, -0.1],
            vec![1.0, f64::NAN],
        ] {
            let distribution = Distribution {
                outcomes: probabilities.into_iter().map(|p| ((), p)).collect(),
            };
            assert!(select_outcome(&distribution, 0.0).is_err());
        }
    }

    #[test]
    fn supplied_draws_choose_action_then_world_outcome_and_reward() {
        let policy = Policy { boldness: 0.3 };
        let ask = sample_transition(&policy, &DatingWorld, &State::JustMet, 0.0, 0.0).unwrap();
        assert_eq!(ask.action, Action::Ask);
        assert_eq!(ask.to_state, State::Engaged);
        close(ask.reward, 1.0);
        let story = sample_transition(&policy, &DatingWorld, &State::JustMet, 0.7, 0.6).unwrap();
        assert_eq!(story.action, Action::Story);
        assert_eq!(story.to_state, State::LowInterest);
        close(story.reward, -2.0);
        assert!(sample_transition(&policy, &DatingWorld, &State::Goodbye, 0.0, 0.0).is_err());
    }

    #[test]
    fn config_rejects_invalid_bounds_before_allocating_or_sampling() {
        for bad in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
            assert!(LearningConfig::new(bad, 5, 0.95, 10, 7).is_err());
            assert!(LearningConfig::new(0.3, 5, bad, 10, 7).is_err());
        }
        assert!(LearningConfig::new(0.3, 7, 0.95, 10, 7).is_err());
        for episodes in [0, MAX_EPISODES + 1, usize::MAX] {
            assert!(LearningConfig::new(0.3, 5, 0.95, episodes, 7).is_err());
        }
        assert!(LearningConfig::new(1.0, 6, 1.0, MAX_EPISODES, u64::MAX).is_ok());
    }

    #[test]
    fn same_seed_repeats_training_with_bounded_table_and_horizon_zero_is_empty() {
        let config = LearningConfig::new(0.3, 5, 0.95, 100, 7).unwrap();
        let first = train(&config).unwrap();
        assert_eq!(first, train(&config).unwrap());
        assert!(first.entries.len() <= 5 * config.horizon);
        assert_eq!(first.entry(&State::JustMet, 5).unwrap().visits, 100);
        let empty = train(&LearningConfig::new(0.3, 0, 0.95, 100, 7).unwrap()).unwrap();
        assert!(empty.entries.is_empty());
    }

    #[test]
    fn online_td_does_not_backfill_an_earlier_update_in_the_same_episode() {
        let mut table = ValueTable::new(Discount(0.5));
        table
            .observe(2, &observation(State::JustMet, 1.0, State::Engaged))
            .unwrap();
        table
            .observe(1, &observation(State::Engaged, 4.0, State::Goodbye))
            .unwrap();
        close(table.value(&State::JustMet, 2), 1.0);
        table
            .observe(2, &observation(State::JustMet, 1.0, State::Engaged))
            .unwrap();
        close(table.value(&State::JustMet, 2), 2.0);
    }
}
