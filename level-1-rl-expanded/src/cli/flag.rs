//! Flags: the options a user can add, such as `--horizon 4`.
//!
//! Question: which options can a user add, and how are they spelled?
//! Example: `-h` and `--help` are the same flag.
//! Predict: is `--Horizon` a flag?

use std::fmt;

/// An option the user can pass; `from_argument` reads it and `Display` spells it back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Flag {
    Boldness,
    Horizon,
    Discount,
    Episodes,
    Seed,
    Verbose,
    Help,
}

impl Flag {
    /// Recognize a flag by its spelling; `-h` is an alias for `--help`.
    pub(super) fn from_argument(argument: &str) -> Option<Self> {
        match argument {
            "--boldness" => Some(Self::Boldness),
            "--horizon" => Some(Self::Horizon),
            "--discount" => Some(Self::Discount),
            "--episodes" => Some(Self::Episodes),
            "--seed" => Some(Self::Seed),
            "--verbose" => Some(Self::Verbose),
            "--help" | "-h" => Some(Self::Help),
            _ => None,
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Boldness => "--boldness",
            Self::Horizon => "--horizon",
            Self::Discount => "--discount",
            Self::Episodes => "--episodes",
            Self::Seed => "--seed",
            Self::Verbose => "--verbose",
            Self::Help => "--help",
        })
    }
}
