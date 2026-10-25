//! Checks for TD(0) updates, sampling, configuration, and training.

use super::config::{ConfigError, Discount, LearningConfig, MAX_EPISODES};
use super::sampling::{sample_transition, select_outcome};
use super::table::ValueTable;
use super::training::train;
use super::LearningError;
use crate::model::{Action, DatingWorld, Policy, State, Transition};
use crate::probability::Distribution;

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

fn policy(boldness: f64) -> Policy {
    Policy::try_new(boldness).expect("test boldness lies in [0, 1]")
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
    assert_eq!(table.entry(State::Engaged, 1).unwrap().visits, 1);
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
    close(table.value(State::JustMet, 1), 2.0);
    close(table.value(State::Engaged, 0), 0.0);
    close(table.value(State::Goodbye, 3), 0.0);
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
        assert_eq!(
            table.observe(horizon, &good),
            Err(LearningError::RemainingHorizonOutOfRange)
        );
    }
    assert_eq!(
        table.observe(1, &observation(State::Goodbye, 1.0, State::Engaged)),
        Err(LearningError::LearnFromTerminal)
    );
    for reward in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            table.observe(1, &observation(State::JustMet, reward, State::Engaged)),
            Err(LearningError::NonFiniteReward)
        );
    }
    assert!(table.entries.is_empty());
    table
        .observe(1, &observation(State::Engaged, f64::MAX, State::Goodbye))
        .unwrap();
    assert_eq!(
        table.observe(2, &observation(State::JustMet, f64::MAX, State::Engaged)),
        Err(LearningError::NonFiniteUpdate)
    );
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
        assert_eq!(
            select_outcome(&distribution, draw),
            Err(LearningError::InvalidDraw)
        );
    }
}

#[test]
fn sampling_rejects_malformed_mass_even_when_draw_hits_an_earlier_branch() {
    for (probabilities, expected) in [
        (vec![], LearningError::InvalidProbabilities),
        (vec![0.0], LearningError::UnnormalizedMass(0.0)),
        (vec![0.8], LearningError::UnnormalizedMass(0.8)),
        (vec![1.1], LearningError::InvalidProbabilities),
        (vec![1.0, -0.1], LearningError::InvalidProbabilities),
        (vec![1.0, f64::NAN], LearningError::InvalidProbabilities),
    ] {
        let distribution = Distribution {
            outcomes: probabilities.into_iter().map(|p| ((), p)).collect(),
        };
        assert_eq!(select_outcome(&distribution, 0.0), Err(expected));
    }
}

#[test]
fn supplied_draws_choose_action_then_world_outcome_and_reward() {
    let policy = policy(0.3);
    let ask = sample_transition(policy, DatingWorld, State::JustMet, 0.0, 0.0).unwrap();
    assert_eq!(ask.action, Action::Ask);
    assert_eq!(ask.to_state, State::Engaged);
    close(ask.reward, 1.0);
    let story = sample_transition(policy, DatingWorld, State::JustMet, 0.7, 0.6).unwrap();
    assert_eq!(story.action, Action::Story);
    assert_eq!(story.to_state, State::LowInterest);
    close(story.reward, -2.0);
    assert_eq!(
        sample_transition(policy, DatingWorld, State::Goodbye, 0.0, 0.0).map(|_| ()),
        Err(LearningError::SampleFromTerminal)
    );
}

#[test]
fn config_rejects_invalid_bounds_before_allocating_or_sampling() {
    for bad in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        assert_eq!(
            LearningConfig::new(policy(0.3), 5, bad, 10, 7),
            Err(ConfigError::InvalidDiscount)
        );
    }
    assert_eq!(
        LearningConfig::new(policy(0.3), 7, 0.95, 10, 7),
        Err(ConfigError::HorizonTooLong)
    );
    for episodes in [0, MAX_EPISODES + 1, usize::MAX] {
        assert_eq!(
            LearningConfig::new(policy(0.3), 5, 0.95, episodes, 7),
            Err(ConfigError::EpisodesOutOfRange)
        );
    }
    assert!(LearningConfig::new(policy(1.0), 6, 1.0, MAX_EPISODES, u64::MAX).is_ok());
}

#[test]
fn same_seed_repeats_training_with_bounded_table_and_horizon_zero_is_empty() {
    let config = LearningConfig::new(policy(0.3), 5, 0.95, 100, 7).unwrap();
    let first = train(&config).unwrap();
    assert_eq!(first, train(&config).unwrap());
    assert!(first.entries.len() <= 5 * config.horizon);
    assert_eq!(first.entry(State::JustMet, 5).unwrap().visits, 100);
    let empty = train(&LearningConfig::new(policy(0.3), 0, 0.95, 100, 7).unwrap()).unwrap();
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
    close(table.value(State::JustMet, 2), 1.0);
    table
        .observe(2, &observation(State::JustMet, 1.0, State::Engaged))
        .unwrap();
    close(table.value(State::JustMet, 2), 2.0);
}
