//! Run a parsed command: write its output, then choose the exit code.

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

pub(crate) fn run() -> ExitCode {
    let options = match parse(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => return fail(error),
    };
    init_tracing(options.verbose);
    match execute(options.command, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        // A reader such as `head` may close the pipe once it has the lines it
        // wants. That is the reader's choice, not a lesson failure, so stop
        // quietly, as Unix command-line tools conventionally do.
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

// Usage mistakes and refused learning steps share one channel: a message on
// stderr, nothing on stdout, and exit code 2.
fn fail(error: impl fmt::Display) -> ExitCode {
    eprintln!("Error: {error}");
    ExitCode::from(2)
}

// Why write to `out` instead of calling `println!`?
//   `println!` panics if stdout is closed.
//   Example: `cargo run -- sweep | head -3` closes stdout after three lines.
//   Writing to `out` turns that into an ordinary error that `run` handles.
//   Tests can also pass their own writer and inspect what happened.
pub(super) fn execute(command: Command, out: &mut impl Write) -> Result<(), RunError> {
    if command.plans_exactly() {
        writeln!(out, "Exact finite-horizon planning with a known model.\n")?;
    }
    let world = DatingWorld;
    match command {
        Command::Help => writeln!(out, "{HELP}")?,
        Command::FirstStep => first_step::print(out, &world)?,
        Command::Returns { discount } => returns::print(out, discount)?,
        Command::Evaluate(settings) => evaluate::print(out, &world, settings)?,
        Command::Sweep { horizon, discount } => sweep::print(out, &world, horizon, discount)?,
        Command::Compare(settings) => compare::print(out, &world, settings)?,
        Command::LearnValue(config) => write!(out, "{}", experience::run(&config)?)?,
        Command::TdTrace => write!(out, "{}", experience::trace()?)?,
    }
    out.flush()?;
    Ok(())
}

fn init_tracing(verbose: bool) {
    // Keep the experiment's answer on stdout and its detailed working on stderr.
    // Turning on diagnostics should help explain a result without changing it.
    tracing_subscriber::fmt()
        .with_max_level(if verbose { Level::DEBUG } else { Level::WARN })
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .compact()
        .init();
}
