# Your Rust + Hugging Face RL learning path

Work through one session at a time. Stop after its explanation check, even if you have not read every comment. All commands run **from the repository root**. The first build may take longer while Cargo downloads dependencies.

The recurring question is: **given a state and a behavior rule, how good are the possible futures?** Here the world model is already known. Learning from experience comes later.

## Session 1: Follow one decision (15 minutes)

**Goal:** separate what the agent chooses from what the environment does.

**Predict:** from `JustMet`, what is the probability of taking `Ask` and reaching `Engaged`? Read the two relevant probabilities before running:

```bash
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- first-step
```

Read only [`State`, `Action`, `Policy::possible_actions`, and `possible_transitions`](../src/main.rs). Trace one branch through `Distribution::and_then`: reaching a leaf requires an action **and** its consequence, so those probabilities multiply. Different branches reaching the same state add together.

**Explain aloud to pass:** who controls each probability? Why can two different branches reach `Engaged`? Calculate its total probability and check that all four branch probabilities sum to one. In Rust, identify the enum for alternatives and the `Vec<(T, f64)>` for weighted outcomes.

If stuck, use [the complete one-step calculation](DEEP_RL_COMPANION.md#36-a-complete-one-step-calculation).

## Session 2: Score a future, then score uncertainty (20 minutes)

**Goal:** distinguish an immediate reward, a trajectory's return, and an expected return.

**Predict:** for rewards `[1, 2, 4]`, which terms survive at discount `0`? What changes at discount `1`?

```bash
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- returns --discount 0
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- returns --discount 0.5
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- returns --discount 1
```

Read [`discounted_return` and `expected_value`](../src/main.rs). The first walks through time; the second combines alternative futures. Look at how the loop updates its discount weight and how `Fn(&T) -> f64` lets expectation score different kinds of outcomes without owning them.

**Explain aloud to pass:** why is the first reward undiscounted? Why do we multiply by time weights inside a path and probability weights across paths? Give an example where an expected return is not an outcome any single episode receives.

Reference: [discounted return](DEEP_RL_COMPANION.md#33-discounted-return) and [expected value](DEEP_RL_COMPANION.md#8-expected-value-collapse-uncertainty-into-one-comparison-number).

## Session 3: Find the end of the episode (20 minutes)

**Goal:** distinguish a terminal state from stopping a calculation early.

**Predict:** what should an evaluation with zero steps score? Does reaching the horizon mean the environment reached `Goodbye`?

```bash
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --horizon 0
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --horizon 1
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --horizon 5
```

Read [`State::is_terminal`, `extend_one_step`, and `enumerate_trajectories`](../src/main.rs). `Goodbye` ends the modeled interaction. The horizon limits how far we calculate; a path can still end in a nonterminal state at that cutoff. This finite-horizon objective assigns no value beyond the cutoff. `MutualInterest` is followed by `Goodbye`, so it is not itself terminal here.

**Explain aloud to pass:** why must an already terminal path stop collecting rewards? Why do branches clone their histories? Identify which value moves into `extend_one_step` and which values are borrowed. Explain why a longer horizon need not improve return when rewards can be negative.

Reference: [terminal and absorbing states](DEEP_RL_COMPANION.md#23-terminal-and-absorbing-states).

## Session 4: Replace repeated futures with values (25 minutes)

**Goal:** explain Bellman evaluation as reusing the value of a smaller remaining problem.

**Predict:** should the trajectory tree and Bellman evaluation agree when the policy, model, discount, and remaining horizon match?

```bash
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- compare --horizon 5
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- compare --horizon 5 --discount 0
```

Read [`values::evaluate_value`](../src/values.rs) beside [`expected_return_of`](../src/main.rs). Start at zero remaining steps, then inspect the next row. Values depend on both state and remaining horizon. Averaging over actions still uses the fixed policy; selecting a maximizing action would answer a different question.

**Explain aloud to pass:** why can two histories share a continuation value in this model? Why do the two methods agree only up to floating-point rounding? Name what they share, and explain why agreement does not prove that the invented world model is realistic.

Reference: [dynamic programming](DEEP_RL_COMPANION.md#40-dynamic-programming).

## Session 5: Compare policies without overclaiming (20 minutes)

**Goal:** distinguish evaluating one policy from searching among policies.

**Predict:** must the most aggressive boldness value have the highest expected return?

```bash
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- sweep
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --boldness 0.3 --horizon 5 --discount 0.95
cargo run --manifest-path level-1-rl-expanded/Cargo.toml -- evaluate --boldness 1 --horizon 5 --discount 0.95
```

Read [`evaluate_policy_grid`, `best_policy`, and `Policy::possible_actions`](../src/main.rs). Compare the cost of premature `Coffee` with its possible reward. The baseline sweep selects boldness `0.3` and scores approximately `3.103947`; a single evaluation of the same policy must reproduce that score.

**Explain aloud to pass:** why does this establish only the best of eleven candidates? Why does a stochastic policy here not mean the agent is learning through exploration? Why can the mean conceal a bad individual outcome? Point to the `Option` that represents having no best candidate yet.

Reference: [grid search](DEEP_RL_COMPANION.md#38-grid-search) and [stochasticity versus exploration](DEEP_RL_COMPANION.md#important-nuance-stochasticity-is-not-automatically-exploration).

## Session 6: Write the small pieces yourself (25 minutes)

**Goal:** retrieve the ideas without copying the reference implementation.

Open [`examples/practice.rs`](../examples/practice.rs). It contains three **EXERCISE ONLY** tasks: `discounted_return`, `weighted_expectation`, and `td_target`. Read each contract, write a prediction for its tests, and attempt one function at a time. Leave the other TODOs alone. Start with the first test group:

```bash
cargo test --manifest-path level-1-rl-expanded/Cargo.toml --example practice exercise_1 -- --ignored
```

Use `exercise_2` or `exercise_3` for the next task; omit the filter to run all three. The untouched template intentionally fails. These ignored tests are separate from the passing reference checks. No solutions are included in this practice file; consult the worked lessons only after making your own attempt.

**Explain aloud to pass:** justify each result, including empty inputs and terminal transitions. For the TD exercise, explain why a true terminal transition and an external time cutoff need different interpretations when estimating a continuing task. A correct target function alone does not implement TD learning: a learner also needs experience and repeated value updates.

If 25 minutes ends first, record the exact unfinished function and stop. Completing one task with an explanation is useful progress.

## Keep the vocabulary attached to code

| RL idea | Symbol | Concrete Rust location |
| --- | --- | --- |
| State / action | `s`, `a` | `State`, `Action` enums in [`main.rs`](../src/main.rs) |
| Policy | `π(a \| s)` | `Policy::possible_actions` returns `Distribution<Action>` |
| Transition model | `P(s' \| s, a)` | `DatingWorld::possible_next_states` |
| Immediate reward | `r` | `DatingWorld::reward_for`, `Transition::reward` |
| Discount / horizon | `γ`, `H` | `discount: f64`, `horizon: usize` |
| Return / expectation | `G`, `E[G]` | `discounted_return`, `expected_value` |
| State value with `h` steps left | `V_h(s)` | [`values::evaluate_value`](../src/values.rs) |

An `f64` is a number, not automatically a valid probability. Every distribution must have finite, nonnegative weights summing to approximately one. Floating-point checks use a tolerance, not exact equality.

## Connect this lab to Hugging Face

Official unit introductions checked on **2026-10-08**:

| Course reading | Use this project for | Work still outside this lab |
| --- | --- | --- |
| [Unit 1: Introduction to Deep RL](https://huggingface.co/learn/deep-rl-course/en/unit1/introduction) | Sessions 1 to 3: agent/environment, rewards, returns, and task boundaries. | The course's LunarLander training with Stable-Baselines3. |
| [Unit 2: Introduction to Q-learning](https://huggingface.co/learn/deep-rl-course/en/unit2/introduction) | Sessions 4 to 6: values, Bellman reasoning, and preparation for TD. | Learning a Q-table from experience, exploration, and the FrozenLake/Taxi exercises. |
| [Unit 3: Deep Q-learning](https://huggingface.co/learn/deep-rl-course/en/unit3/introduction) | Explain why storing a separate value for every state eventually becomes impractical. | Neural Q-value approximation and Atari training. |

This lab is a Rust companion to the concepts, not a port of those course notebooks. Exact model-based planning averages known possibilities; experience-based learning estimates values or a model from observed transitions. Follow the official hands-on work separately when you reach it.

## Save a restart cue

After each session, copy this into your own notes:

```text
Session / date:
My prediction:
Observed result and command:
My explanation in one sentence:
Confidence (1 = lost, 5 = can explain without notes):
Next time, open [file], find [symbol], and try [one change]:
```

Start the next session by repeating yesterday's explanation without opening the source. If confidence is 1 or 2, repeat the smallest experiment with one changed input. If it is 3 or higher, continue and return to the uncertain point later.
