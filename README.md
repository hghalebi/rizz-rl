# Learn Hugging Face RL with Rust

A small Rust lab for understanding the ideas behind the Hugging Face Deep RL course. Start with one visible decision, then work toward policy evaluation and Bellman values.

## Your first 15 minutes

Run these commands **from this repository's root**:

```bash
cargo test --manifest-path level-1-rl-expanded/Cargo.toml
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- first-step
```

Before reading the output, predict the probability of `JustMet → Ask → Engaged`. Then open [Session 1 of the learning path](level-1-rl-expanded/guides/LEARNING_PATH.md#session-1-follow-one-decision-15-minutes).

You need Rust and Cargo installed. The first build downloads the dependencies; subsequent lessons run locally without a model API, GPU, or Hugging Face account.

## Choose the right code

| Location | Purpose |
| --- | --- |
| [Expanded lab](level-1-rl-expanded/README.md) | Tested reference program, focused commands, and short learning sessions. Start here. |
| [Root `src/main.rs`](src/main.rs) | Your original scratch implementation, preserved for your own experiments. |
| [Practice file](level-1-rl-expanded/examples/practice.rs) | Three unfinished exercises with opt-in tests. Try them after the guided sessions. |

These are separate Cargo packages. Plain `cargo run` runs the root scratch program; the commands above select the expanded lab explicitly. The two programs have different policies and rewards, so their results are not interchangeable. The scratch policy currently assigns total probability `0.8` in `LowInterest`; use the expanded lab as the baseline when checking probability conservation.

If a command checks `rizz-rl` instead of `level_1_rl_expanded`, check the manifest path. The root scratch file currently has unused imports that fail Clippy when warnings are denied; that is separate from the expanded lab's checks.

## What you will learn

Follow the [six short sessions](level-1-rl-expanded/guides/LEARNING_PATH.md): probabilities, returns, episode boundaries, Bellman evaluation, policy search, and independent Rust practice. Each session has a prediction, a runnable experiment, and an explanation to give aloud.

The program evaluates a **known toy model exactly**. It does not yet learn from sampled experience or train a neural network. Its conversation states are invented teaching labels, not a validated model of people. The [course map](level-1-rl-expanded/guides/LEARNING_PATH.md#connect-this-lab-to-hugging-face) explains how this foundation connects to the actual Hugging Face exercises.
