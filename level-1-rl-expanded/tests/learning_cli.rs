//! Check the same executable boundary used by the learning guide.
use std::process::{Command, Output};

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_level_1_rl_expanded"))
        .args(arguments)
        .output()
        .expect("the compiled learning binary should start")
}

fn success(arguments: &[&str]) -> String {
    let output = run(arguments);
    assert!(
        output.status.success(),
        "{arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "normal lessons should be quiet");
    String::from_utf8(output.stdout).expect("lesson output is UTF-8")
}

#[test]
fn default_search_preserves_the_known_baseline_without_debug_noise() {
    let output = success(&[]);
    assert!(output.contains("3.103947"));
    assert!(output.contains("0.3"));
    assert!(!output.contains("DEBUG"));
}

#[test]
fn evaluate_reports_probability_mass_and_path_count() {
    let output = success(&["evaluate", "--boldness", "0.3"]);
    assert!(output.contains("3.103947"));
    assert!(output.contains("3448"));
    assert!(output.contains("1.000000"));
}

#[test]
fn worked_examples_match_independent_hand_calculations() {
    let first_step = success(&["first-step"]);
    assert!(first_step.contains("0.220000"));
    let returns = success(&["returns", "--discount", "0.5"]);
    // 1 + 0.5 * 2 + 0.25 * 4 = 3.
    assert!(returns.contains("3.000000"));
}

#[test]
fn two_evaluators_agree_through_the_default_horizon() {
    let output = success(&["compare"]);
    for quantity in [
        "Tree E[G]",
        "Bellman V_h",
        "absolute difference",
        "cached subproblems",
    ] {
        assert!(output.contains(quantity));
    }
    assert!(output.matches("3.103947").count() >= 2);
}

#[test]
fn zero_horizon_and_zero_discount_are_valid_experiments() {
    let empty = success(&["evaluate", "--horizon", "0"]);
    assert!(empty.contains("0.000000"));
    let immediate = success(&["evaluate", "--boldness", "1", "--discount", "0"]);
    assert!(immediate.contains("0.220000"));
}

#[test]
fn help_is_available_without_running_a_policy_search() {
    let output = success(&["--help"]);
    for command in [
        "first-step",
        "returns",
        "evaluate",
        "sweep",
        "compare",
        "learn-value",
        "td-trace",
    ] {
        assert!(output.contains(command));
    }
    assert!(!output.contains("3.103947"));
}

#[test]
fn malformed_experiments_fail_before_printing_lesson_results() {
    for arguments in [
        vec!["missing-command"],
        vec!["evaluate", "--horizon", "7"],
        vec!["evaluate", "--horizon", "-1"],
        vec!["evaluate", "--horizon", "2.5"],
        vec!["evaluate", "--discount", "NaN"],
        vec!["evaluate", "--discount", "inf"],
        vec!["evaluate", "--discount", "-0.1"],
        vec!["evaluate", "--discount", "1.1"],
        vec!["evaluate", "--boldness", "1.1"],
        vec!["evaluate", "--boldness"],
        vec!["evaluate", "--discount", "0.5", "--discount", "0.9"],
        vec!["returns", "--horizon", "3"],
        vec!["sweep", "--boldness", "0.3"],
        vec!["learn-value", "--episodes", "0"],
        vec!["learn-value", "--episodes", "1000001"],
        vec!["learn-value", "--episodes", "-1"],
        vec!["learn-value", "--episodes", "1.5"],
        vec!["learn-value", "--episodes"],
        vec!["learn-value", "--episodes", "1", "--episodes", "2"],
        vec!["learn-value", "--seed", "-1"],
        vec!["learn-value", "--seed", "1.5"],
        vec!["learn-value", "--seed", "18446744073709551616"],
        vec!["learn-value", "--seed"],
        vec!["learn-value", "--seed", "1", "--seed", "2"],
        vec!["learn-value", "--horizon", "7"],
        vec!["learn-value", "--discount", "NaN"],
        vec!["learn-value", "--boldness", "inf"],
        vec!["evaluate", "--episodes", "10"],
        vec!["sweep", "--seed", "1"],
        vec!["td-trace", "--episodes", "1"],
        vec!["td-trace", "--seed", "1"],
        vec!["td-trace", "--discount", "0.5"],
        vec!["td-trace", "--horizon", "1"],
        vec!["td-trace", "--boldness", "0.3"],
    ] {
        let output = run(&arguments);
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert!(!output.stderr.is_empty(), "{arguments:?}");
    }
}

#[test]
fn sampled_learning_is_repeatable_for_the_same_seed_and_settings() {
    let arguments = ["learn-value", "--episodes", "256", "--seed", "19"];
    let first = success(&arguments);
    assert_eq!(first, success(&arguments));
    for label in [
        "Sampled TD(0) evaluation of a fixed policy.",
        "Episodes: 256 | Seed: 19",
        "Exact V_h(JustMet): 3.103947",
        "Learned V_h(JustMet):",
        "Absolute error:",
        "Initial-state visits: 256",
    ] {
        assert!(first.contains(label), "missing {label:?} from {first}");
    }
    assert!(!first.contains("Exact finite-horizon planning with a known model."));
}

#[test]
fn zero_horizon_learning_reports_a_base_case_without_visits() {
    let output = success(&[
        "learn-value",
        "--horizon",
        "0",
        "--episodes",
        "1",
        "--seed",
        "18446744073709551615",
    ]);
    for label in [
        "Exact V_h(JustMet): 0.000000",
        "Learned V_h(JustMet): 0.000000",
        "Absolute error: 0.000000",
        "Initial-state visits: 0",
    ] {
        assert!(output.contains(label), "missing {label:?} from {output}");
    }
}

#[test]
fn supplied_td_trace_exposes_terminal_targets_and_repeated_updates() {
    let output = success(&["td-trace"]);
    for calculation in [
        "target=4.000000",
        "target=3.000000",
        "visits=2 alpha=0.500000",
        "value=2.000000",
    ] {
        assert!(
            output.contains(calculation),
            "missing {calculation:?} from {output}"
        );
    }
    assert!(!output.contains("Exact finite-horizon planning with a known model."));
}

#[test]
fn verbose_mode_adds_diagnostics_without_changing_results() {
    let quiet = success(&["evaluate"]);
    let verbose = run(&["evaluate", "--verbose"]);
    assert!(verbose.status.success());
    assert_eq!(String::from_utf8_lossy(&verbose.stdout), quiet);
    assert!(String::from_utf8_lossy(&verbose.stderr).contains("DEBUG"));
}

#[test]
fn a_closed_output_pipe_ends_quietly_instead_of_panicking() {
    // A reader such as `head` can stop early. Dropping the read end before the
    // lesson starts makes its first write fail with a broken pipe every time.
    let (reader, writer) = std::io::pipe().expect("create an output pipe");
    drop(reader);
    let output = Command::new(env!("CARGO_BIN_EXE_level_1_rl_expanded"))
        .arg("sweep")
        .stdout(writer)
        .output()
        .expect("the compiled learning binary should start");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    assert!(
        stderr.is_empty(),
        "a closed reader is not a lesson error: {stderr}"
    );
}
