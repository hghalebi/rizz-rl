//! What a validated command line means: one experiment and its settings.

use crate::experience::LearningConfig;
use crate::model::Policy;

/// Settings shared by evaluate and compare: which policy, how far ahead, how much the future counts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Evaluation {
    pub(super) policy: Policy,
    pub(super) horizon: usize,
    pub(super) discount: f64,
}

/// One experiment, fully validated, with the settings it needs.
#[derive(Debug, PartialEq)]
pub(super) enum Command {
    FirstStep,
    Returns { discount: f64 },
    Evaluate(Evaluation),
    Sweep { horizon: usize, discount: f64 },
    Compare(Evaluation),
    LearnValue(LearningConfig),
    TdTrace,
    Help,
}

impl Command {
    /// Whether this command prints the exact-planning banner; help and sampled learning do not.
    pub(super) fn plans_exactly(&self) -> bool {
        !matches!(self, Self::Help | Self::LearnValue(_) | Self::TdTrace)
    }
}

/// The parsed command line: the command plus the global verbosity switch.
#[derive(Debug, PartialEq)]
pub(super) struct Options {
    pub(super) command: Command,
    pub(super) verbose: bool,
}

/// The lesson's default dial setting, boldness 0.3; `try_new` accepts it because 0.3 lies in [0, 1].
pub(super) fn default_policy() -> Policy {
    Policy::try_new(0.3).expect("the default boldness 0.3 lies in [0, 1]")
}
