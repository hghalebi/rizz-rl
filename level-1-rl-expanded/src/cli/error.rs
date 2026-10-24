//! Every reason command-line text can be refused, with the message the user sees.

use std::fmt;
use std::ops::RangeInclusive;

use super::command_name::CommandName;
use super::flag::Flag;
use crate::experience::ConfigError;

/// Why command-line text could not become a validated experiment.
#[derive(Debug, PartialEq)]
pub(super) enum CliError {
    UnknownArgument(String),
    MultipleCommands,
    MissingValue(Flag),
    Repeated(Flag),
    NotUnitInterval {
        flag: Flag,
        raw: String,
    },
    IntegerOutOfRange {
        flag: Flag,
        range: RangeInclusive<usize>,
        raw: String,
    },
    InvalidSeed(String),
    NotApplicable {
        flag: Flag,
        command: CommandName,
    },
    Config(ConfigError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownArgument(argument) => write!(
                f,
                "unknown argument '{argument}'; use --help to see commands and options"
            ),
            Self::MultipleCommands => {
                f.write_str("choose exactly one command; use --help to see examples")
            }
            Self::MissingValue(flag) => {
                write!(f, "{flag} requires a value; use --help for its range")
            }
            Self::Repeated(flag) => {
                write!(f, "{flag} was supplied more than once; provide it once")
            }
            Self::NotUnitInterval { flag, raw } => {
                write!(f, "{flag} requires a finite number in [0, 1]; got '{raw}'")
            }
            Self::IntegerOutOfRange { flag, range, raw } => write!(
                f,
                "{flag} requires an integer in [{}, {}]; got '{raw}'",
                range.start(),
                range.end()
            ),
            Self::InvalidSeed(raw) => {
                write!(f, "--seed requires an unsigned 64-bit integer; got '{raw}'")
            }
            Self::NotApplicable { flag, command } => write!(
                f,
                "{flag} does not apply to '{command}'; use --help for supported options"
            ),
            // The configuration message is complete on its own, so it is shown as is.
            Self::Config(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CliError {}

impl From<ConfigError> for CliError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}
