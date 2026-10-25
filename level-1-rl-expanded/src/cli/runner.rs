//! Run a parsed command: write its output, then choose the exit code.
//!
//! Question: once a command is validated, how does it run, and which exit code results?
//! Example: a usage mistake prints one line on stderr and exits 2; `sweep | head -3` exits 0.
//! Predict: when an error occurs, what appears on stdout?

use std::fmt;
use std::io::{self, Write};
use std::process::ExitCode;

use tracing::Level;

use super::command::Command;
use super::help::HELP;
use super::parse::parse;
use super::{compare, evaluate, first_step, returns, sweep};
use crate::experience::{self, LearningError};
use crate::model::DatingWorld;

/// Failures that can happen after the arguments were accepted.
#[derive(Debug)]
pub(super) enum RunError {
    Learning(LearningError),
    Output(io::Error),
}

impl From<LearningError> for RunError {
    fn from(error: LearningError) -> Self {
        Self::Learning(error)
    }
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Output(error)
    }
}

/// Parse the arguments, run the command, and turn the outcome into an exit code.
/// `usage error ──► 2`, `refused learning step ──► 2`, `closed pipe ──► 0`, `write error ──► 1`
/// Invariant: an error prints one line on stderr and nothing on stdout.
pub fn run() -> ExitCode {
    let options = match parse(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => return fail(error),
    };
    init_tracing(options.verbose);
    match execute(&options.command, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        // A reader such as `head` may close the pipe early: its choice, not a lesson failure.
        Err(RunError::Output(error)) if error.kind() == io::ErrorKind::BrokenPipe => {
            ExitCode::SUCCESS
        }
        Err(RunError::Output(error)) => {
            eprintln!("Error: could not write the lesson output: {error}");
            ExitCode::FAILURE
        }
        Err(RunError::Learning(error)) => fail(error),
    }
}

/// Report a usage mistake or a refused learning step: one line on stderr, exit code 2.
fn fail(error: impl fmt::Display) -> ExitCode {
    eprintln!("Error: {error}");
    ExitCode::from(2)
}

/// Write the chosen command's lesson to `out`, then flush.
/// Invariant: all output goes through `out`, never `println!`, so a closed stdout cannot panic.
pub(super) fn execute(command: &Command, out: &mut impl Write) -> Result<(), RunError> {
    if command.plans_exactly() {
        writeln!(out, "Exact finite-horizon planning with a known model.\n")?;
    }
    let world = DatingWorld;
    match *command {
        Command::Help => writeln!(out, "{HELP}")?,
        Command::FirstStep => first_step::print(out, world)?,
        Command::Returns { discount } => returns::print(out, discount)?,
        Command::Evaluate(settings) => evaluate::print(out, world, settings)?,
        Command::Sweep { horizon, discount } => sweep::print(out, world, horizon, discount)?,
        Command::Compare(settings) => compare::print(out, world, settings)?,
        Command::LearnValue(config) => write!(out, "{}", experience::run(&config)?)?,
        Command::TdTrace => write!(out, "{}", experience::trace()?)?,
    }
    out.flush()?;
    Ok(())
}

/// Send diagnostics to stderr, so `--verbose` explains a result without changing it.
fn init_tracing(verbose: bool) {
    tracing_subscriber::fmt()
        .with_max_level(if verbose { Level::DEBUG } else { Level::WARN })
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .compact()
        .init();
}
