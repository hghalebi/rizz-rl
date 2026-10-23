//! Read the value after a flag and check it, so `parse` can stay short.
//! Each helper either returns a checked value or a `CliError` naming the flag.

use std::ops::RangeInclusive;

use super::error::CliError;
use super::flag::Flag;
use crate::model::Policy;

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

pub(super) fn switch_on(switch: &mut bool, flag: Flag) -> Result<(), CliError> {
    if std::mem::replace(switch, true) {
        Err(CliError::Repeated(flag))
    } else {
        Ok(())
    }
}

pub(super) fn value(
    arguments: &mut impl Iterator<Item = String>,
    flag: Flag,
) -> Result<String, CliError> {
    arguments
        .next()
        .filter(|argument| !argument.starts_with("--"))
        .ok_or(CliError::MissingValue(flag))
}

// Policy::try_new owns the boldness rule. The CLI only adds which flag and
// which text were rejected, so the learner sees exactly what to correct.
pub(super) fn parse_policy(flag: Flag, raw: String) -> Result<Policy, CliError> {
    match raw.parse::<f64>().map(Policy::try_new) {
        Ok(Ok(policy)) => Ok(policy),
        _ => Err(CliError::NotUnitInterval { flag, raw }),
    }
}

pub(super) fn unit_interval(flag: Flag, raw: String) -> Result<f64, CliError> {
    // Parsing "NaN" or "inf" can succeed as an f64. A number that Rust can store
    // is not necessarily a meaningful discount for this lesson.
    match raw.parse::<f64>() {
        Ok(number) if number.is_finite() && (0.0..=1.0).contains(&number) => Ok(number),
        _ => Err(CliError::NotUnitInterval { flag, raw }),
    }
}

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
