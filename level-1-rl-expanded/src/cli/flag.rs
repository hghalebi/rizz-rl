//! Flags: the options a user can add, such as `--horizon 4`.

use std::fmt;

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
