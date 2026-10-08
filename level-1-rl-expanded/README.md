# Level 1.5: Visible Reinforcement Learning in Rust

A tiny exact planner you can trace by hand. The recommended entry point is the [six-session learning path](guides/LEARNING_PATH.md), not the full reference guide.

## Run one concept at a time

All commands here run **from the repository root**, one directory above this README:

```bash
cargo test --manifest-path level-1-rl-expanded/Cargo.toml
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- first-step
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- returns --discount 0.5
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --boldness 0.3 --horizon 5
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- compare --horizon 5
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- sweep
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- --help
```

`first-step` exposes conditional probabilities. `returns` scores the fixed reward sequence `[1, 2, 4]`. `evaluate` scores one policy; `compare` checks the trajectory tree against Bellman evaluation for horizons from zero to the requested horizon. `sweep` compares eleven boldness values from `0.0` to `1.0` in steps of `0.1`. Running without a command also selects `sweep`.

The defaults are boldness `0.3` for evaluation, horizon `5`, and discount `0.95`. The command line limits the horizon to `6` because the trajectory tree grows quickly. Add `--verbose` to inspect the expansion trace after you understand the compact output.

Baseline with the unmodified model:

```text
best boldness on the grid: 0.3
expected return:          3.103947
probability mass:         approximately 1.0
```

This is the best candidate on that grid for those settings, not a proof of a global optimum. Changing the model, horizon, or discount may change the winner.

## Reading and practice

1. [Learning path](guides/LEARNING_PATH.md): one prediction and one experiment at a time.
2. [Source](src/main.rs): numbered checkpoints connecting domain meaning, mechanism, and RL vocabulary; [Bellman values](src/values.rs) reuse the same model.
3. [Practice](examples/practice.rs): three answer-hidden exercises. Their tests are deliberately ignored during normal checks. Opt in only when ready:

   ```bash
   cargo test --manifest-path level-1-rl-expanded/Cargo.toml --example practice -- --ignored
   ```

   Untouched exercises fail with `todo!`; this is the starting point, not a broken reference program. Passing normal tests does not complete these exercises.
4. [Deep RL companion](guides/DEEP_RL_COMPANION.md) and [interactive labs](guides/INTERACTIVE_LABS.md): reference material and longer extensions. Their older `cargo run` examples assume you are inside `level-1-rl-expanded/`. Several labs describe future changes; they are not all implemented features.

Keep the rhythm: **predict → run → explain aloud → change one input → predict again**. Retype a small checkpoint only after you can describe what it does.

## Check the reference lab

From the repository root:

```bash
cargo fmt --manifest-path level-1-rl-expanded/Cargo.toml --check
cargo test --locked --manifest-path level-1-rl-expanded/Cargo.toml --all-targets
cargo clippy --locked --manifest-path level-1-rl-expanded/Cargo.toml --all-targets -- -D warnings
```

The tests compare Bellman values against complete trees and run the actual CLI,
including invalid inputs. The three practice tests remain ignored here until
you select them explicitly. Passing these checks establishes program behavior;
the session explanation checks assess your understanding.

## Scope

Implemented: a finite MDP, a stochastic parameterized policy, known transitions and rewards, exact finite-horizon trajectories, discounted expected return, Bellman policy evaluation, and grid search. All calculations use the hand-written model; no future is sampled.

Monte Carlo estimation, TD training, SARSA, Q-learning, policy gradients, actor-critic, Dyna, and neural networks remain later steps. The practice TD target is a preparatory exercise, not a training loop. Randomness in a policy does not by itself establish an exploration strategy for learning.
