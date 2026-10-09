//! Command-line parsing and presentation for one-concept experiments.

use std::process::ExitCode;

use tracing::Level;

use crate::model::{Action, DatingWorld, Policy, State, Transition};
use crate::planning::{
    best_policy, discounted_return, enumerate_trajectories, evaluate_policy_grid,
    expected_return_of, possible_transitions, Trajectory,
};
use crate::probability::expected_value;
use crate::MAX_HORIZON;

const HELP: &str = "A small laboratory for planning and learning policy values.
Planning commands evaluate a known model exactly; learn-value uses sampled episodes.

From the repository root:
  cargo run -- [COMMAND] [OPTIONS]

Commands:
  first-step  Follow action and next-state probabilities, then average rewards
  returns     Compute discounted return for the supplied rewards [1, 2, 4]
  evaluate    Score one policy by enumerating all represented trajectories
  sweep       Score all 11 boldness values from 0.0 to 1.0 (default command)
  compare     Compare trajectory enumeration with Bellman evaluation at H=0..H
  learn-value Estimate a fixed policy's value from sampled episodes
  td-trace    Follow a supplied deterministic TD update, including termination

Options:
  --boldness B  Finite number in [0, 1]; default 0.3 (evaluate, compare, learn-value)
  --horizon H   Integer in [0, 6]; default 5 (evaluate, sweep, compare, learn-value)
  --discount G  Finite number in [0, 1]; default 0.95 (returns, evaluate, sweep, compare, learn-value)
  --episodes N Integer in [1, 1000000]; default 10000 (learn-value)
  --seed S     Unsigned 64-bit integer; default 7 (learn-value)
  --verbose     Show algorithm traces on stderr
  --help, -h    Show this help

The horizon is bounded because exact trajectory enumeration grows rapidly.
Example: cargo run -- compare --horizon 4 --discount 0.5";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Evaluation {
    boldness: f64,
    horizon: usize,
    discount: f64,
}

#[derive(Debug, PartialEq)]
enum Command {
    FirstStep,
    Returns { discount: f64 },
    Evaluate(Evaluation),
    Sweep { horizon: usize, discount: f64 },
    Compare(Evaluation),
    LearnValue(super::experience::LearningConfig),
    TdTrace,
    Help,
}

#[derive(Debug, PartialEq)]
struct Options {
    command: Command,
    verbose: bool,
}

// The parser is the only construction path for command settings. Every value
// is validated before tracing or potentially expensive tree expansion begins.
fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut arguments = arguments.into_iter();
    let mut name = None;
    let mut boldness = None;
    let mut horizon = None;
    let mut discount = None;
    let mut episodes = None;
    let mut seed = None;
    let mut verbose = false;
    let mut help = false;

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--boldness" => {
                unique(boldness.is_some(), &argument)?;
                boldness = Some(unit_interval(
                    &value(&mut arguments, &argument)?,
                    &argument,
                )?);
            }
            "--discount" => {
                unique(discount.is_some(), &argument)?;
                discount = Some(unit_interval(
                    &value(&mut arguments, &argument)?,
                    &argument,
                )?);
            }
            "--horizon" => {
                unique(horizon.is_some(), &argument)?;
                let raw = value(&mut arguments, &argument)?;
                horizon = Some(
                    raw.parse::<usize>()
                        .ok()
                        .filter(|steps| *steps <= MAX_HORIZON)
                        .ok_or_else(|| {
                            format!(
                                "--horizon requires an integer in [0, {MAX_HORIZON}]; got '{raw}'"
                            )
                        })?,
                );
            }
            "--episodes" => {
                unique(episodes.is_some(), &argument)?;
                let raw = value(&mut arguments, &argument)?;
                episodes = Some(
                    raw.parse::<usize>()
                        .ok()
                        .filter(|count| (1..=super::experience::MAX_EPISODES).contains(count))
                        .ok_or_else(|| {
                            format!(
                                "--episodes requires an integer in [1, {}]; got '{raw}'",
                                super::experience::MAX_EPISODES,
                            )
                        })?,
                );
            }
            "--seed" => {
                unique(seed.is_some(), &argument)?;
                let raw = value(&mut arguments, &argument)?;
                seed = Some(raw.parse::<u64>().map_err(|_| {
                    format!("--seed requires an unsigned 64-bit integer; got '{raw}'")
                })?);
            }
            "--verbose" => {
                unique(verbose, &argument)?;
                verbose = true;
            }
            "--help" | "-h" => {
                unique(help, "--help")?;
                help = true;
            }
            "first-step" | "returns" | "evaluate" | "sweep" | "compare" | "learn-value"
            | "td-trace" => {
                if name.is_some() {
                    return Err("choose exactly one command; use --help to see examples".into());
                }
                name = Some(argument);
            }
            _ => {
                return Err(format!(
                    "unknown argument '{argument}'; use --help to see commands and options"
                ))
            }
        }
    }

    let name = name.as_deref().unwrap_or("sweep");
    for (provided, flag, allowed) in [
        (
            boldness.is_some(),
            "--boldness",
            matches!(name, "evaluate" | "compare" | "learn-value"),
        ),
        (
            horizon.is_some(),
            "--horizon",
            matches!(name, "evaluate" | "sweep" | "compare" | "learn-value"),
        ),
        (
            discount.is_some(),
            "--discount",
            !matches!(name, "first-step" | "td-trace"),
        ),
        (episodes.is_some(), "--episodes", name == "learn-value"),
        (seed.is_some(), "--seed", name == "learn-value"),
    ] {
        if provided && !allowed {
            return Err(format!(
                "{flag} does not apply to '{name}'; use --help for supported options"
            ));
        }
    }

    let settings = Evaluation {
        boldness: boldness.unwrap_or(0.3),
        horizon: horizon.unwrap_or(5),
        discount: discount.unwrap_or(0.95),
    };
    let command = if help {
        Command::Help
    } else {
        match name {
            "first-step" => Command::FirstStep,
            "returns" => Command::Returns {
                discount: settings.discount,
            },
            "evaluate" => Command::Evaluate(settings),
            "compare" => Command::Compare(settings),
            "learn-value" => Command::LearnValue(super::experience::LearningConfig::new(
                settings.boldness,
                settings.horizon,
                settings.discount,
                episodes.unwrap_or(10_000),
                seed.unwrap_or(7),
            )?),
            "td-trace" => Command::TdTrace,
            _ => Command::Sweep {
                horizon: settings.horizon,
                discount: settings.discount,
            },
        }
    };
    Ok(Options { command, verbose })
}

fn unique(already_present: bool, flag: &str) -> Result<(), String> {
    if already_present {
        Err(format!(
            "{flag} was supplied more than once; provide it once"
        ))
    } else {
        Ok(())
    }
}

fn value(arguments: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    arguments
        .next()
        .filter(|argument| !argument.starts_with("--"))
        .ok_or_else(|| format!("{flag} requires a value; use --help for its range"))
}

fn unit_interval(raw: &str, flag: &str) -> Result<f64, String> {
    raw.parse::<f64>()
        .ok()
        .filter(|number| number.is_finite() && (0.0..=1.0).contains(number))
        .ok_or_else(|| format!("{flag} requires a finite number in [0, 1]; got '{raw}'"))
}

pub(super) fn run() -> ExitCode {
    let options = match parse(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("Error: {error}");
            return ExitCode::from(2);
        }
    };
    init_tracing(options.verbose);
    if options.command == Command::Help {
        println!("{HELP}");
        return ExitCode::SUCCESS;
    }
    if !matches!(&options.command, Command::LearnValue(_) | Command::TdTrace) {
        println!("Exact finite-horizon planning with a known model.\n");
    }
    let world = DatingWorld;
    match options.command {
        Command::FirstStep => first_step(&world),
        Command::Returns { discount } => returns(discount),
        Command::Evaluate(settings) => evaluate(&world, settings),
        Command::Sweep { horizon, discount } => sweep(&world, horizon, discount),
        Command::Compare(settings) => compare(&world, settings),
        Command::LearnValue(config) => match super::experience::run(&config) {
            Ok(output) => print!("{output}"),
            Err(error) => {
                eprintln!("Error: {error}");
                return ExitCode::from(2);
            }
        },
        Command::TdTrace => print!("{}", super::experience::trace()),
        Command::Help => {}
    }
    ExitCode::SUCCESS
}

fn init_tracing(verbose: bool) {
    tracing_subscriber::fmt()
        .with_max_level(if verbose { Level::DEBUG } else { Level::WARN })
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .compact()
        .init();
}

// A concrete one-step worked example.
//
// From JustMet:
//
//     P(Ask)   = 0.70
//     P(Story) = 0.30
//
// The world then branches conditionally:
//
//     P(Engaged | Ask)      = 0.80
//     P(LowInterest | Ask)  = 0.20
//     P(Engaged | Story)    = 0.60
//     P(LowInterest | Story)= 0.40
//
// `and_then` combines them into joint path probabilities:
//
//     P(Ask and Engaged) = 0.70 × 0.80 = 0.56
//
// This function prints the same calculation from the real program data.
fn print_first_step_walkthrough(policy: &Policy, world: &DatingWorld) {
    let transitions = possible_transitions(policy, world, &State::JustMet);

    println!(
        r#"
----------------------------------------------------------------
FIRST-STEP PROBABILITY CHECKPOINT
----------------------------------------------------------------
The policy chooses an action.
The world responds conditionally to that action.
A complete branch probability is:

P(action and next_state)
=
P(action | state) * P(next_state | state, action)
----------------------------------------------------------------
"#
    );

    for (transition, probability) in &transitions.outcomes {
        println!(
            "{:>6.1}%  {:?} --{:?} / {:+.1}--> {:?}",
            probability * 100.0,
            transition.from_state,
            transition.action,
            transition.reward,
            transition.to_state,
        );
    }

    println!(
        "Total probability mass: {:.6}\n",
        transitions.total_probability(),
    );
}

fn first_step(world: &DatingWorld) {
    let policy = Policy { boldness: 0.3 };
    print_first_step_walkthrough(&policy, world);
    let transitions = possible_transitions(&policy, world, &State::JustMet);
    let immediate_reward = expected_value(&transitions, |transition| transition.reward);
    println!(
        "Expected immediate reward = sum(branch probability * reward) = {immediate_reward:.6}"
    );
    println!("Boldness has no effect on the first step: JustMet always uses Ask=0.7, Story=0.3.");
    println!("Next experiment: evaluate --horizon 1, then evaluate --horizon 2.");
}

fn returns(discount: f64) {
    // This synthetic trace isolates arithmetic. Its rewards are supplied by
    // the exercise rather than obtained by calling DatingWorld::reward_for.
    let trajectory: Trajectory = [1.0, 2.0, 4.0]
        .into_iter()
        .map(|reward| Transition {
            from_state: State::Engaged,
            action: Action::Ask,
            reward,
            to_state: State::Engaged,
        })
        .collect();
    println!("Supplied synthetic reward trace: [1, 2, 4]. Discount: {discount}");
    println!("G = r_0 + discount*r_1 + discount^2*r_2");
    println!("step   reward     weight   contribution");
    let mut weight = 1.0;
    for (step, transition) in trajectory.iter().enumerate() {
        println!(
            "{step:>4}   {:>6.2}   {weight:>8.4}   {:>12.4}",
            transition.reward,
            weight * transition.reward
        );
        weight *= discount;
    }
    println!(
        "Discounted return G: {:.6}",
        discounted_return(&trajectory, discount)
    );
    println!("Next experiment: predict G for --discount 0, 0.5, and 1 before running them.");
}

fn evaluate(world: &DatingWorld, settings: Evaluation) {
    let policy = Policy {
        boldness: settings.boldness,
    };
    let trajectories = enumerate_trajectories(&policy, world, &State::JustMet, settings.horizon);
    let score = expected_value(&trajectories, |trajectory| {
        discounted_return(trajectory, settings.discount)
    });
    println!(
        "Boldness: {} | Horizon: {} transitions | Discount: {}",
        settings.boldness, settings.horizon, settings.discount
    );
    println!(
        "Represented trajectories: {}",
        trajectories.number_of_outcomes()
    );
    println!(
        "Total probability mass: {:.9}",
        trajectories.total_probability()
    );
    println!("Expected return E[G]: {score:.6}");
    println!("E[G] averages complete returns using their path probabilities.");
    println!("Next experiment: change only --horizon, then explain the change in E[G].");
}

fn sweep(world: &DatingWorld, horizon: usize, discount: f64) {
    println!("Horizon: {horizon} transitions | Discount: {discount}");
    println!("boldness   expected return");
    let evaluations = evaluate_policy_grid(world, horizon, discount);
    for evaluation in &evaluations {
        let boldness = evaluation.policy.boldness;
        let score = evaluation.expected_return;
        println!("{boldness:>8.1}   {score:>15.6}");
    }
    if let Some(best) = best_policy(&evaluations) {
        let boldness = best.policy.boldness;
        let score = best.expected_return;
        println!("Best on this grid: boldness={boldness:.1}, expected return={score:.6}");
    }
    println!("Scores within 1e-12 tie and keep the lower boldness. This searches only the 11 displayed candidates.");
    println!("Next experiment: compare --boldness 0.3, or use --help for smaller steps.");
}

fn compare(world: &DatingWorld, settings: Evaluation) {
    let policy = Policy {
        boldness: settings.boldness,
    };
    println!(
        "Boldness: {} | Discount: {}",
        settings.boldness, settings.discount
    );
    println!("V_0(s) = 0; V_h(s) = sum p(a,s'|s) * [r + discount*V_(h-1)(s')]");
    println!("   H       Tree E[G]   Bellman V_h   absolute difference   cached subproblems");
    for horizon in 0..=settings.horizon {
        let score = expected_return_of(&policy, world, horizon, settings.discount);
        let value = super::values::evaluate_value(
            &policy,
            world,
            &State::JustMet,
            horizon,
            settings.discount,
        );
        println!(
            "{horizon:>4}   {score:>13.6}   {:>11.6}   {:>19.3e}   {:>18}",
            value.expected_return,
            (score - value.expected_return).abs(),
            value.evaluated_states
        );
    }
    println!("Both columns evaluate the same model and policy; small differences come from floating-point arithmetic.");
    println!("Bellman evaluation reuses (state, steps remaining) values; the cache count excludes base cases.");
    println!("Next experiment: explain why V_0 is zero and V_1 matches first-step.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arguments(args: &[&str]) -> Result<Options, String> {
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
                    boldness: 1.0,
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
        use super::super::experience::{LearningConfig, MAX_EPISODES};
        assert_eq!(
            arguments(&["learn-value"]).unwrap().command,
            Command::LearnValue(LearningConfig::new(0.3, 5, 0.95, 10_000, 7).unwrap())
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
            Command::LearnValue(LearningConfig::new(1.0, 6, 0.0, MAX_EPISODES, u64::MAX).unwrap())
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
}
