//! Command-line parsing and presentation for one-concept experiments.
//!
//! Treat a command as a small controlled experiment: change one setting while
//! keeping the others fixed, and predict the result. This module translates
//! text into settings and displays results; the numerical rules live in the
//! domain modules, so the same rules can be tested without typing commands.
//!
//! Read the files in this order:
//! 1. `cli/help.rs`: the `--help` text, the user's view of every command.
//! 2. `cli/command_name.rs` and `cli/flag.rs`: the words a user can type.
//! 3. `cli/command.rs`: what a validated command line means.
//! 4. `cli/parse.rs`: turn text into a `Command`, or a `CliError`.
//!    `cli/flag_value.rs` reads and checks the value after each flag.
//! 5. `cli/error.rs`: every reason the text can be refused.
//! 6. `cli/runner.rs`: run the command, write its output, choose the exit code.
//! 7. One file per experiment: `first_step.rs`, `returns.rs`, `evaluate.rs`,
//!    `sweep.rs`, and `compare.rs`.

mod command;
mod command_name;
mod compare;
mod error;
mod evaluate;
mod first_step;
mod flag;
mod flag_value;
mod help;
mod parse;
mod returns;
mod runner;
mod sweep;

#[cfg(test)]
mod tests;

pub use runner::run;
