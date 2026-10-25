//! Checks for argument parsing, help text, error messages, and output failures.

use std::io::{self, Write};

use super::command::{Command, Evaluation, Options};
use super::command_name::CommandName;
use super::error::CliError;
use super::flag::Flag;
use super::help::HELP;
use super::parse::parse;
use super::runner::{execute, RunError};
use crate::experience::{LearningConfig, MAX_EPISODES};
use crate::model::Policy;

fn arguments(args: &[&str]) -> Result<Options, CliError> {
    parse(args.iter().map(|argument| (*argument).to_owned()))
}

#[test]
fn defaults_preserve_the_original_search_settings() {
    assert_eq!(
        arguments(&[]).unwrap(),
        Options {
            command: Command::Sweep {
                horizon: 5,
                discount: 0.95
            },
            verbose: false,
        }
    );
}

#[test]
fn boundaries_and_options_before_the_command_are_supported() {
    assert_eq!(
        arguments(&[
            "--verbose",
            "compare",
            "--boldness",
            "1",
            "--horizon",
            "6",
            "--discount",
            "0"
        ])
        .unwrap(),
        Options {
            command: Command::Compare(Evaluation {
                policy: Policy::try_new(1.0).unwrap(),
                horizon: 6,
                discount: 0.0
            }),
            verbose: true,
        }
    );
    assert!(arguments(&[
        "evaluate",
        "--boldness",
        "0",
        "--horizon",
        "0",
        "--discount",
        "1"
    ])
    .is_ok());
}

#[test]
fn invalid_values_are_rejected_before_evaluation() {
    for flag in ["--boldness", "--discount"] {
        for raw in ["NaN", "inf", "-inf", "-0.1", "1.1", "1e999", "abc"] {
            assert!(
                arguments(&["evaluate", flag, raw]).is_err(),
                "accepted {flag} {raw}"
            );
        }
    }
    for raw in ["7", "1000000", "-1", "1.5", "abc"] {
        assert!(
            arguments(&["evaluate", "--horizon", raw]).is_err(),
            "accepted horizon {raw}"
        );
    }
}

#[test]
fn sampled_learning_defaults_and_boundaries_are_validated() {
    assert_eq!(
        arguments(&["learn-value"]).unwrap().command,
        Command::LearnValue(
            LearningConfig::new(Policy::try_new(0.3).unwrap(), 5, 0.95, 10_000, 7).unwrap()
        )
    );
    assert_eq!(
        arguments(&[
            "learn-value",
            "--boldness",
            "1",
            "--horizon",
            "6",
            "--discount",
            "0",
            "--episodes",
            &MAX_EPISODES.to_string(),
            "--seed",
            &u64::MAX.to_string(),
        ])
        .unwrap()
        .command,
        Command::LearnValue(
            LearningConfig::new(
                Policy::try_new(1.0).unwrap(),
                6,
                0.0,
                MAX_EPISODES,
                u64::MAX
            )
            .unwrap()
        )
    );
    assert!(arguments(&[
        "learn-value",
        "--episodes",
        "1",
        "--horizon",
        "0",
        "--seed",
        "0"
    ])
    .is_ok());
}

#[test]
fn rejects_silent_mistakes() {
    for args in [
        vec!["returns", "--boldness", "0.5"],
        vec!["returns", "--horizon", "1"],
        vec!["first-step", "--discount", "0.5"],
        vec!["sweep", "--boldness", "0.5"],
        vec!["evaluate", "--horizon"],
        vec!["evaluate", "--discount", "--verbose"],
        vec!["evaluate", "--horizon", "1", "--horizon", "2"],
        vec!["evaluate", "--discount", "0", "--discount", "1"],
        vec!["evaluate", "--boldness", "0", "--boldness", "1"],
        vec!["--verbose", "--verbose"],
        vec!["--help", "-h"],
        vec!["evaluate", "sweep"],
        vec!["evalute"],
        vec!["--unknown"],
    ] {
        assert!(arguments(&args).is_err(), "accepted {args:?}");
    }
}

#[test]
fn help_is_available_for_each_command() {
    for name in [
        "first-step",
        "returns",
        "evaluate",
        "sweep",
        "compare",
        "learn-value",
        "td-trace",
    ] {
        assert_eq!(arguments(&[name, "--help"]).unwrap().command, Command::Help);
    }
    assert_eq!(arguments(&["-h"]).unwrap().command, Command::Help);
}

#[test]
fn help_lists_exactly_the_commands_that_accept_each_flag() {
    // HELP is plain text for readers; `accepts` is what the parser enforces.
    // This test keeps the two descriptions of the same rule in agreement.
    for flag in [
        Flag::Boldness,
        Flag::Horizon,
        Flag::Discount,
        Flag::Episodes,
        Flag::Seed,
    ] {
        let line = HELP
            .lines()
            .find(|line| line.starts_with(&format!("  {flag} ")))
            .expect("each value flag has one help line");
        let listed = line
            .rsplit_once('(')
            .and_then(|(_, commands)| commands.strip_suffix(')'))
            .expect("each value flag's help line ends with its commands");
        let accepted: Vec<&str> = CommandName::ALL
            .into_iter()
            .filter(|command| command.accepts(flag))
            .map(CommandName::as_str)
            .collect();
        assert_eq!(listed, accepted.join(", "), "{flag}");
    }
}

struct ClosedOutput(io::ErrorKind);

impl Write for ClosedOutput {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(self.0.into())
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(self.0.into())
    }
}

#[test]
fn write_failures_are_returned_instead_of_panicking() {
    for kind in [io::ErrorKind::BrokenPipe, io::ErrorKind::StorageFull] {
        let result = execute(&Command::FirstStep, &mut ClosedOutput(kind));
        assert!(
            matches!(&result, Err(RunError::Output(error)) if error.kind() == kind),
            "{result:?}"
        );
    }
}

#[test]
fn errors_name_the_flag_and_the_accepted_values() {
    for (args, message) in [
        (
            &["evaluate", "--horizon", "7"][..],
            "--horizon requires an integer in [0, 6]; got '7'",
        ),
        (
            &["learn-value", "--episodes", "0"],
            "--episodes requires an integer in [1, 1000000]; got '0'",
        ),
        (
            &["evaluate", "--discount", "NaN"],
            "--discount requires a finite number in [0, 1]; got 'NaN'",
        ),
        (
            &["sweep", "--boldness", "0.5"],
            "--boldness does not apply to 'sweep'; use --help for supported options",
        ),
        (
            &["-h", "--help"],
            "--help was supplied more than once; provide it once",
        ),
        (
            &["evalute"],
            "unknown argument 'evalute'; use --help to see commands and options",
        ),
    ] {
        assert_eq!(arguments(args).unwrap_err().to_string(), message);
    }
}
