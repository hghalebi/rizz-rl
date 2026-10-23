//! The settings for one learning run, checked before any episode is sampled.
//!
//! Question: what does one learning run need, and which values are refused?
//! Example: 0 episodes is refused; 1 to 1,000,000 episodes are accepted.
//! Predict: why does this file receive a `Policy` instead of a raw boldness number?

use std::fmt;

use crate::model::Policy;
use crate::MAX_HORIZON;

pub(crate) const MAX_EPISODES: usize = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Discount(pub(super) f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LearningConfig {
    pub(super) policy: Policy,
    pub(super) horizon: usize,
    pub(super) discount: Discount,
    pub(super) episodes: usize,
    pub(super) seed: u64,
}

impl LearningConfig {
    // Taking a Policy rather than a raw boldness means boldness was already
    // validated, and it cannot be passed in the discount position by mistake.
    pub(crate) fn new(
        policy: Policy,
        horizon: usize,
        discount: f64,
        episodes: usize,
        seed: u64,
    ) -> Result<Self, ConfigError> {
        if !discount.is_finite() || !(0.0..=1.0).contains(&discount) {
            return Err(ConfigError::InvalidDiscount);
        }
        if horizon > MAX_HORIZON {
            return Err(ConfigError::HorizonTooLong);
        }
        if !(1..=MAX_EPISODES).contains(&episodes) {
            return Err(ConfigError::EpisodesOutOfRange);
        }
        Ok(Self {
            policy,
            horizon,
            discount: Discount(discount),
            episodes,
            seed,
        })
    }
}

/// Settings rejected before any table is allocated or any episode is sampled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ConfigError {
    InvalidDiscount,
    HorizonTooLong,
    EpisodesOutOfRange,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDiscount => f.write_str("discount must be finite and in [0, 1]"),
            Self::HorizonTooLong => write!(f, "horizon must be in [0, {MAX_HORIZON}]"),
            Self::EpisodesOutOfRange => write!(f, "episodes must be in [1, {MAX_EPISODES}]"),
        }
    }
}

impl std::error::Error for ConfigError {}
