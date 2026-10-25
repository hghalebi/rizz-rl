//! Every reason a learning step can be refused.
//!
//! Question: what can go wrong during learning, and what does the user read when it does?
//! Example: a reward of NaN is refused before the table changes.
//! Predict: which error does sampling a transition out of `Goodbye` produce?

use std::fmt;

use crate::MAX_HORIZON;

/// Why an observation or a sampling step was refused. Every check runs before
/// the table changes, so an error never leaves a half-applied update behind.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LearningError {
    RemainingHorizonOutOfRange,
    LearnFromTerminal,
    SampleFromTerminal,
    NonFiniteReward,
    VisitCountOverflow,
    NonFiniteUpdate,
    InvalidDraw,
    InvalidProbabilities,
    UnnormalizedMass(f64),
    NoPositiveProbability,
}

impl fmt::Display for LearningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RemainingHorizonOutOfRange => write!(
                f,
                "an observation needs a remaining horizon in [1, {MAX_HORIZON}]"
            ),
            Self::LearnFromTerminal => {
                f.write_str("cannot learn an outgoing transition from a terminal state")
            }
            Self::SampleFromTerminal => {
                f.write_str("cannot sample an outgoing transition from a terminal state")
            }
            Self::NonFiniteReward => f.write_str("an observed reward must be finite"),
            Self::VisitCountOverflow => f.write_str("visit count overflow"),
            Self::NonFiniteUpdate => {
                f.write_str("TD update produced a non-finite value; table was not changed")
            }
            Self::InvalidDraw => f.write_str("a sampling draw must be finite and in [0, 1)"),
            Self::InvalidProbabilities => {
                f.write_str("sampling needs nonempty finite probabilities in [0, 1]")
            }
            Self::UnnormalizedMass(mass) => {
                write!(f, "sampling probabilities must sum to 1; got {mass}")
            }
            Self::NoPositiveProbability => f.write_str("sampling found no positive probability"),
        }
    }
}

impl std::error::Error for LearningError {}
