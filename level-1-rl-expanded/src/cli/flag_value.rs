//! Read the value after a flag and check it, so `parse` can stay short.
//! Each helper either returns a checked value or a `CliError` naming the flag.

use std::ops::RangeInclusive;

use super::error::CliError;
use super::flag::Flag;
use crate::model::Policy;

/// Store a flag's value once; a repeated flag is refused before its second value is parsed.
pub(super) fn set_once<T>(
    slot: &mut Option<T>,
    flag: Flag,
    parse_value: impl FnOnce() -> Result<T, CliError>,
) -> Result<(), CliError> {
    if slot.is_some() {
        return Err(CliError::Repeated(flag));
    }
    *slot = Some(parse_value()?);
    Ok(())
}

/// Turn a boolean flag on once; a repeated switch is refused.
pub(super) fn switch_on(switch: &mut bool, flag: Flag) -> Result<(), CliError> {
    // `mem::replace` sets the switch and hands back its old value in one step.
    if std::mem::replace(switch, true) {
        Err(CliError::Repeated(flag))
    } else {
        Ok(())
    }
}

/// The word after a flag, refusing a missing value or another flag in its place.
pub(super) fn value(
    arguments: &mut impl Iterator<Item = String>,
    flag: Flag,
) -> Result<String, CliError> {
    arguments
        .next()
        .filter(|argument| !argument.starts_with("--"))
        .ok_or(CliError::MissingValue(flag))
}

/// Parse `--boldness` through `Policy::try_new`, which owns the range rule.
/// Invariant: the CLI only adds which flag and which text were refused, so the fix is obvious.
pub(super) fn parse_policy(flag: Flag, raw: String) -> Result<Policy, CliError> {
    match raw.parse::<f64>().map(Policy::try_new) {
        Ok(Ok(policy)) => Ok(policy),
        _ => Err(CliError::NotUnitInterval { flag, raw }),
    }
}

/// Parse a discount: a finite number in [0, 1].
/// Invariant: "NaN" and "inf" parse as `f64` but are refused; a storable number is not a valid discount.
pub(super) fn unit_interval(flag: Flag, raw: String) -> Result<f64, CliError> {
    match raw.parse::<f64>() {
        Ok(number) if number.is_finite() && (0.0..=1.0).contains(&number) => Ok(number),
        _ => Err(CliError::NotUnitInterval { flag, raw }),
    }
}

/// Parse an integer and check that it lies in `range`.
pub(super) fn integer_in(
    flag: Flag,
    raw: String,
    range: RangeInclusive<usize>,
) -> Result<usize, CliError> {
    match raw.parse::<usize>() {
        Ok(number) if range.contains(&number) => Ok(number),
        _ => Err(CliError::IntegerOutOfRange { flag, range, raw }),
    }
}
