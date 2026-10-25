//! Command names: the experiment a user picks, such as `sweep` or `compare`.
//!
//! Question: which experiment did the user pick, and which flags may go with it?
//! Example: `sweep` is the default when no command is typed; `sweep --boldness 0.5` is refused.
//! Predict: what does the typo `evalute` produce?

use std::fmt;
use std::str::FromStr;

use super::error::CliError;
use super::flag::Flag;

/// The experiment named on the command line, before its settings are known.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum CommandName {
    FirstStep,
    Returns,
    Evaluate,
    #[default]
    Sweep,
    Compare,
    LearnValue,
    TdTrace,
}

impl CommandName {
    /// Every command name, in help order.
    pub(super) const ALL: [Self; 7] = [
        Self::FirstStep,
        Self::Returns,
        Self::Evaluate,
        Self::Sweep,
        Self::Compare,
        Self::LearnValue,
        Self::TdTrace,
    ];

    /// The spelling a user types.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::FirstStep => "first-step",
            Self::Returns => "returns",
            Self::Evaluate => "evaluate",
            Self::Sweep => "sweep",
            Self::Compare => "compare",
            Self::LearnValue => "learn-value",
            Self::TdTrace => "td-trace",
        }
    }

    /// Which flags this command accepts: the same table as `HELP`, enforced.
    /// Invariant: the match is exhaustive, so a new flag must decide which commands accept it.
    pub(super) fn accepts(self, flag: Flag) -> bool {
        match flag {
            Flag::Boldness => matches!(self, Self::Evaluate | Self::Compare | Self::LearnValue),
            Flag::Horizon => matches!(
                self,
                Self::Evaluate | Self::Sweep | Self::Compare | Self::LearnValue
            ),
            Flag::Discount => !matches!(self, Self::FirstStep | Self::TdTrace),
            Flag::Episodes | Flag::Seed => self == Self::LearnValue,
            Flag::Verbose | Flag::Help => true,
        }
    }
}

impl FromStr for CommandName {
    type Err = CliError;

    /// Match the typed word against every command's spelling.
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|command| command.as_str() == name)
            .ok_or_else(|| CliError::UnknownArgument(name.to_owned()))
    }
}

impl fmt::Display for CommandName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
