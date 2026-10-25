//! Entry point for the exact-planning and sampled-value learning experiments.
//!
//! Start with one question: how good are the futures produced by a behavior rule?
//! Read `model` for the choices and consequences, then `probability` for how
//! uncertain branches combine. `planning` scores histories up to the selected
//! horizon; `values` reuses smaller exact answers; `experience` learns from
//! observations.
//! The learning path in `guides/LEARNING_PATH.md` turns that reading into tasks.
//! Each lesson file opens with a question, an example, and a prediction, and the
//! README lists the code's eleven numbered checkpoints in reading order.
//!
//! At each step, predict one small result before running the corresponding
//! command. Then change one input and explain whether the result should change.

mod cli;
mod experience;
mod model;
mod planning;
mod probability;
mod values;

#[cfg(test)]
mod exercises;
#[cfg(test)]
mod tests;

/// The largest horizon any command accepts, so exact enumeration stays affordable.
const MAX_HORIZON: usize = 6;

fn main() -> std::process::ExitCode {
    cli::run()
}
