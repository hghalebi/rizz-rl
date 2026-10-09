# Rust RL lab: running and checking the project

For what to study next, use the [learning path](guides/LEARNING_PATH.md). This README covers operating the reference lab. All commands run **from the repository root**. The root workspace selects this package by default and excludes the [historical scratch program](../scratch/original/README.md).

## Run one concept

```bash
cargo run -- first-step
cargo run -- returns --discount 0.5
cargo run -- evaluate --boldness 0.3 --horizon 5
cargo run -- compare --horizon 5
cargo run -- sweep
cargo run -- td-trace
cargo run -- learn-value --episodes 10000 --seed 7
cargo run -- --help
```

`first-step` exposes conditional probabilities. `returns` scores the supplied sequence `[1, 2, 4]`. `evaluate` scores one policy; `compare` checks trajectory enumeration against Bellman evaluation. `sweep`, also the default command, compares eleven boldness values. `td-trace` explains supplied updates; `learn-value` samples episodes and estimates finite-horizon values for a fixed policy.

Use `--help` as the command and parameter reference. Add `--verbose` when you want expansion details. The horizon is bounded because trajectory enumeration grows quickly.

The unmodified default sweep selects boldness `0.3`, with expected return approximately `3.103947`, probability mass approximately `1.0`, and 3,448 stored trajectories. It establishes the best candidate on that grid for those settings. Changing the model, horizon, or discount can change the winner.

## Source map

| File | Responsibility |
| --- | --- |
| [`main.rs`](src/main.rs) | Module wiring and the entry point |
| [`cli.rs`](src/cli.rs) | Commands, validation, and presentation |
| [`probability.rs`](src/probability.rs) | `Distribution`, composition, and expectation |
| [`model.rs`](src/model.rs) | States, actions, policy, world, and rewards |
| [`planning.rs`](src/planning.rs) | Transition composition, trajectories, returns, and policy grid search |
| [`values.rs`](src/values.rs) | Exact finite-horizon Bellman evaluation |
| [`experience.rs`](src/experience.rs) | Sampling and observation-driven TD value updates |
| [`exercises/`](src/exercises/mod.rs) | Arithmetic and composition practice, separate solutions, shared cases, and worked examples |

The planner and TD learner target the same finite-horizon objective. The sampler uses the toy model; the learner receives observations; the exact evaluator provides a separate comparison. Q-learning control, SARSA, policy gradients, and neural networks are outside the implemented training loop. Arithmetic exercises cover some of their building blocks.

## Exercise commands

Use the exact selector listed in the [arithmetic catalog](guides/EXERCISES.md) or [composition lesson](guides/COMPOSITION_LESSON.md):

```bash
cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
cargo test exercises::arithmetic::solutions::exercise_1_discounted_return -- --exact
cargo test exercises::composition::practice::map_laws -- --exact --ignored
cargo test exercises::composition::solutions::map_laws -- --exact
```

The 17 starter tests are deliberately ignored in ordinary checks. An untouched selected starter fails at `todo!()`. Reference solutions run in normal `cargo test`. Filtering selects which tests run; all code in that target must still compile, so an unrelated syntax or type error can block the selected task. Keep other TODOs intact while working on one task.

## Check the lab

The complete local check is:

```bash
python3 scripts/check.py
```

It requires Python 3.9 or later, Rust, Cargo, rustfmt, and Clippy. It checks formatting, strict Clippy, locked tests, documentation links and anchors, documented lesson commands, and exercise selectors. It reports unfinished starter tasks separately from reference failures. The GitHub Actions workflow uses the same script; a local success does not establish that hosted CI ran.

The underlying Rust checks are:

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
```

Tests cover exact evaluator agreement, probability and terminal behavior, sampled TD mechanics, composition, CLI errors, and a compile-fail ownership fixture. They establish behavior for the covered cases. Learner understanding and retention require actual attempts recorded in the [learning record](guides/LEARNING_RECORD.md).

## Reference and maintenance

- [Deep RL companion](guides/DEEP_RL_COMPANION.md): concept reference, not required reading before starting.
- [Interactive labs](guides/INTERACTIVE_LABS.md): optional extensions; several ask you to implement new behavior.
- [Command migration](guides/maintenance/COMMAND_MIGRATION.md): equivalents for older commands and source paths.
- [Structure review and plan](guides/maintenance/STRUCTURE_REVIEW_AND_PLAN.md): current structural delivery status.
- [Pedagogy review](guides/PEDAGOGY_REVIEW.md) and [improvement plan](guides/IMPROVEMENT_PLAN.md): dated research and earlier delivery history, with learner outcomes still pending.

If a command selects `rizz-rl`, you are explicitly using the historical scratch manifest. Return to the repository root and use the commands above. The scratch program has different rewards and an incomplete probability distribution, so its numerical output is not the reference baseline.
