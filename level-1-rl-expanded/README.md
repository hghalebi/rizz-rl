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

Results go to stdout. Diagnostics and errors go to stderr.

| Exit code | Meaning |
| --- | --- |
| `0` | The lesson ran. This includes a reader such as `head` that stops reading early. |
| `1` | The result could not be written. |
| `2` | An invalid argument or a refused learning step. The reason is on stderr, and no result is printed. |

The unmodified default sweep selects boldness `0.3`, with expected return approximately `3.103947`, probability mass approximately `1.0`, and 3,448 stored trajectories. It establishes the best candidate on that grid for those settings. Changing the model, horizon, or discount can change the winner.

## Source map

Each module file is a short table of contents. Its folder holds one concept per file, listed below in reading order.

| Module | Responsibility | Files, in reading order |
| --- | --- | --- |
| [`main.rs`](src/main.rs) | Module wiring and the entry point | |
| [`probability.rs`](src/probability.rs) | `Distribution`, composition, and expectation | [`distribution`](src/probability/distribution.rs), [`composition`](src/probability/composition.rs), [`expectation`](src/probability/expectation.rs) |
| [`model.rs`](src/model.rs) | States, actions, policy, world, and rewards | [`state`](src/model/state.rs), [`policy`](src/model/policy.rs), [`world`](src/model/world.rs), [`reward`](src/model/reward.rs), [`transition`](src/model/transition.rs) |
| [`planning.rs`](src/planning.rs) | Transition composition, trajectories, returns, and policy grid search | [`transitions`](src/planning/transitions.rs), [`rollout`](src/planning/rollout.rs), [`returns`](src/planning/returns.rs), [`evaluation`](src/planning/evaluation.rs), [`search`](src/planning/search.rs) |
| [`values.rs`](src/values.rs) | Exact finite-horizon Bellman evaluation | Single file; tests in [`values/tests.rs`](src/values/tests.rs) |
| [`experience.rs`](src/experience.rs) | Sampling and observation-driven TD value updates | [`config`](src/experience/config.rs), [`table`](src/experience/table.rs), [`sampling`](src/experience/sampling.rs), [`training`](src/experience/training.rs), [`report`](src/experience/report.rs), [`trace`](src/experience/trace.rs), [`error`](src/experience/error.rs) |
| [`cli.rs`](src/cli.rs) | Commands, validation, and presentation | [`help`](src/cli/help.rs), [`command_name`](src/cli/command_name.rs), [`flag`](src/cli/flag.rs), [`command`](src/cli/command.rs), [`parse`](src/cli/parse.rs), [`flag_value`](src/cli/flag_value.rs), [`error`](src/cli/error.rs), [`runner`](src/cli/runner.rs), then one file per experiment |
| [`exercises/`](src/exercises/mod.rs) | Arithmetic and composition practice, separate solutions, shared cases, and worked examples | Unchanged: learner files |

### Read the code in checkpoint order

Every lesson file opens with three lines: **Question**, **Example**, **Predict**. Read the question, work the example on paper, answer the prediction, then read the code to check. The checkpoints number the ideas in code order; the [learning path](guides/LEARNING_PATH.md) meets them in learning order, one session at a time.

| Checkpoint | Idea | File | Session |
| --- | --- | --- | --- |
| 1 | Several possible values, each with a chance | [`probability/distribution.rs`](src/probability/distribution.rs) | 1 |
| 2 | States and actions | [`model/state.rs`](src/model/state.rs) | 1 |
| 3 | The policy: a dial from state to action chances | [`model/policy.rs`](src/model/policy.rs) | 1 |
| 4 | The world's response, then its rewards | [`model/world.rs`](src/model/world.rs), [`model/reward.rs`](src/model/reward.rs) | 1 |
| 5 | One recorded step: a transition | [`model/transition.rs`](src/model/transition.rs) | 1 |
| 6 | Grow every possible history | [`planning/rollout.rs`](src/planning/rollout.rs) | 3 |
| 7 | Score one history: the discounted return | [`planning/returns.rs`](src/planning/returns.rs) | 2 |
| 8 | Score uncertainty: the expectation | [`probability/expectation.rs`](src/probability/expectation.rs) | 2 |
| 9 | Compare eleven policies | [`planning/search.rs`](src/planning/search.rs) | 5 |
| 10 | Reuse smaller answers: Bellman evaluation | [`values.rs`](src/values.rs) | 4 |
| 11 | Learn from observations: one TD(0) update | [`experience/table.rs`](src/experience/table.rs) | Capstone |

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

Tests cover exact evaluator agreement, probability and terminal behavior, sampled TD mechanics, composition, CLI errors, the boldness range enforced by `Policy::try_new`, output to a closed pipe, and a compile-fail ownership fixture. They establish behavior for the covered cases. Learner understanding and retention require actual attempts recorded in the [learning record](guides/LEARNING_RECORD.md).

## Reference and maintenance

- [Deep RL companion](guides/DEEP_RL_COMPANION.md): concept reference, not required reading before starting.
- [Interactive labs](guides/INTERACTIVE_LABS.md): optional extensions; several ask you to implement new behavior.
- [Command migration](guides/maintenance/COMMAND_MIGRATION.md): equivalents for older commands and source paths.
- [Structure review and plan](guides/maintenance/STRUCTURE_REVIEW_AND_PLAN.md): current structural delivery status.
- [Pedagogy review](guides/PEDAGOGY_REVIEW.md) and [improvement plan](guides/IMPROVEMENT_PLAN.md): dated research and earlier delivery history, with learner outcomes still pending.

If a command selects `rizz-rl`, you are explicitly using the historical scratch manifest. Return to the repository root and use the commands above. The scratch program has different rewards and an incomplete probability distribution, so its numerical output is not the reference baseline.
