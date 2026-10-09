//! Entry point for the exact-planning and sampled-value learning experiments.
//! Follow the learning path for the order in which to inspect each module.

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

// Keep all experiments comparable within the small exact planner's work bound.
const MAX_HORIZON: usize = 6;

fn main() -> std::process::ExitCode {
    cli::run()
}
