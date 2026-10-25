//! Turn command-line text into a validated `Command`, or explain what is wrong.
//!
//! Question: how do argument words become one validated `Command`, and where does checking stop?
//! Example: `evaluate --horizon 7` is refused with the accepted range before any tree is expanded.
//! Predict: with `--horizon 2 --horizon 3`, is the second value even parsed?

use super::command::{default_policy, Command, Evaluation, Options};
use super::command_name::CommandName;
use super::error::CliError;
use super::flag::Flag;
use super::flag_value::{integer_in, parse_policy, set_once, switch_on, unit_interval, value};
use crate::experience::{LearningConfig, MAX_EPISODES};
use crate::MAX_HORIZON;

/// Turn the argument words into a validated `Command`, or the first `CliError` met.
/// Invariant: every value is checked before tracing starts or any tree is expanded.
pub(super) fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Options, CliError> {
    let mut arguments = arguments.into_iter();
    let mut name = None;
    let mut policy = None;
    let mut horizon = None;
    let mut discount = None;
    let mut episodes = None;
    let mut seed = None;
    let mut verbose = false;
    let mut help = false;

    while let Some(argument) = arguments.next() {
        let Some(flag) = Flag::from_argument(&argument) else {
            if name.replace(argument.parse::<CommandName>()?).is_some() {
                return Err(CliError::MultipleCommands);
            }
            continue;
        };
        // `set_once` rejects a repeated flag before the closure runs; its value is never parsed.
        match flag {
            Flag::Boldness => set_once(&mut policy, flag, || {
                parse_policy(flag, value(&mut arguments, flag)?)
            })?,
            Flag::Discount => set_once(&mut discount, flag, || {
                unit_interval(flag, value(&mut arguments, flag)?)
            })?,
            Flag::Horizon => set_once(&mut horizon, flag, || {
                integer_in(flag, value(&mut arguments, flag)?, 0..=MAX_HORIZON)
            })?,
            Flag::Episodes => set_once(&mut episodes, flag, || {
                integer_in(flag, value(&mut arguments, flag)?, 1..=MAX_EPISODES)
            })?,
            Flag::Seed => set_once(&mut seed, flag, || {
                let raw = value(&mut arguments, flag)?;
                raw.parse::<u64>().map_err(|_| CliError::InvalidSeed(raw))
            })?,
            Flag::Verbose => switch_on(&mut verbose, flag)?,
            Flag::Help => switch_on(&mut help, flag)?,
        }
    }

    let name = name.unwrap_or_default();
    for (flag, provided) in [
        (Flag::Boldness, policy.is_some()),
        (Flag::Horizon, horizon.is_some()),
        (Flag::Discount, discount.is_some()),
        (Flag::Episodes, episodes.is_some()),
        (Flag::Seed, seed.is_some()),
    ] {
        if provided && !name.accepts(flag) {
            return Err(CliError::NotApplicable {
                flag,
                command: name,
            });
        }
    }

    let settings = Evaluation {
        policy: policy.unwrap_or_else(default_policy),
        horizon: horizon.unwrap_or(5),
        discount: discount.unwrap_or(0.95),
    };
    let command = if help {
        Command::Help
    } else {
        match name {
            CommandName::FirstStep => Command::FirstStep,
            CommandName::Returns => Command::Returns {
                discount: settings.discount,
            },
            CommandName::Evaluate => Command::Evaluate(settings),
            CommandName::Sweep => Command::Sweep {
                horizon: settings.horizon,
                discount: settings.discount,
            },
            CommandName::Compare => Command::Compare(settings),
            CommandName::LearnValue => Command::LearnValue(LearningConfig::new(
                settings.policy,
                settings.horizon,
                settings.discount,
                episodes.unwrap_or(10_000),
                seed.unwrap_or(7),
            )?),
            CommandName::TdTrace => Command::TdTrace,
        }
    };
    Ok(Options { command, verbose })
}
