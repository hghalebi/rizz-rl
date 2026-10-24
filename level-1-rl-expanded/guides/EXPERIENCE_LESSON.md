# Learn a value from observations

**Start here:** follow [one supplied update](#2-predict-one-update-before-training-15-minutes). **Returning:** use your [quick restart note](LEARNING_RECORD.md#quick-restart).

This lesson learns **values for a fixed policy**. Its boldness stays fixed while estimates change. It does not learn an improved policy or a neural network. The [learning path](LEARNING_PATH.md#experience-capstone-learn-values-from-observations-25-minutes) supplies the prerequisites: return, expectation, and terminal behavior.

All commands run **from the repository root**. Take one activity at a time; the time estimates are adjustable. A short visit can end after one prediction.

## 1. Separate the responsibilities

You have already evaluated all possible futures using the model. Here the learner gets one observed transition at a time and corrects its estimate.

| Part | Information it receives | Its job |
| --- | --- | --- |
| Sampler | Policy, world model, seed, and horizon | Produce observed actions, rewards, and next states. |
| Learner | Observations, remaining steps, and discount | Update a value table and its visit counts. |
| Exact evaluator | The same policy, world, initial state, horizon, and discount | Supply a separate comparison after training. |

The learner does not inspect model probabilities or exact answers. Its table entry is `V(h, state)`, where `h` means transitions remaining. The same state with one step left and with five steps left asks two different questions.

## 2. Predict one update before training (15 minutes)

**Purpose:** see how one observation corrects one estimate. A *target* combines the reward just observed with the discounted estimate of what follows. An *update* moves the old estimate toward that target.

The command below contains three supplied records. Focus on the **second** record. An earlier record has already set the one-step estimate of `Engaged` to `4`:

| Input to this update | Value |
| --- | --- |
| Entry being updated | `V(2, JustMet)` |
| Previous estimate | `0` |
| Observed reward | `1` |
| Next state | `Engaged`, with one transition remaining |
| Current estimate of that continuation | `4` |
| Discount | `0.5` |
| Visit count for this entry, including this observation | `1` |

The learning rate is `1 / visits` for this entry. The update is `previous + learning_rate * (target - previous)`.

**1. Predict:** calculate the target and the new estimate for this record.

**2. Do:** run the supplied trace.

```bash
cargo run -- td-trace
```

**3. Check:** find the second record's `target`, `visits`, and `value`. Compare them with your prediction. These are separate deterministic update records, not one world-generated episode; the supplied rewards isolate the arithmetic.

**4. Explain:** suppose this observation instead ended at `Goodbye`, with the same reward. Which part of the target would disappear, and why would the arriving reward still count?

<details>
<summary>Optional support: target, update, and Rust details</summary>

If the target is unclear, attempt [exercise 3](EXERCISES.md#3-terminal-aware-td-target), `td_target`. It computes a target without updating stored values. If moving the old estimate is unclear, attempt [exercise 8](EXERCISES.md#8-one-td-update), `td_update`. Each card has its own exact command and separate solution.

[`ValueTable::observe`](../src/experience/table.rs) borrows the table mutably and only reads the observation. Counts belong to `(h, state)`, not to the whole run. The next-state estimate is looked up at `h - 1`.

A terminal next state or zero remaining horizon gives zero continuation. The last reward still counts. Episodes stop at termination or the selected horizon; this lesson has no separate interruption limit. An external cutoff in a continuing task would require a different interpretation.

</details>

**Stop or next:** save the record you were explaining and `cargo run -- td-trace` as your restart action. Next, [run one sampled experiment](#3-run-one-reproducible-experiment-20-minutes).

## 3. Run one reproducible experiment (20 minutes)

**Purpose:** estimate the value of one fixed policy from sampled episodes, then compare it with the exact answer to the same question.

This experiment fixes boldness `0.3`, horizon `5`, discount `0.95`, and seed `7`. It starts with an empty learned table and collects 10,000 episodes. Use `cargo run -- --help` for command defaults and bounds.

**1. Predict:** must this finite sample reproduce the exact Bellman value?

**2. Do:** run the experiment.

```bash
cargo run -- learn-value --boldness 0.3 --horizon 5 --discount 0.95 --episodes 10000 --seed 7
```

**3. Check:** read `Learned V_h(JustMet)`, `Exact V_h(JustMet)`, and `Absolute error`. A difference measures this estimate's error for the declared model and objective, not whether the model describes real people.

**4. Explain:** choose a table row marked `unvisited`. Why is its zero an initial estimate rather than evidence that its value was learned?

**5. Repeat:** rerun the same command. Check whether the result repeats. A seed supports reproducibility under the same program and dependencies; it does not establish accuracy.

<details>
<summary>Optional depth: coverage, means, and ownership</summary>

Some displayed entries cannot be reached from the chosen initial state and horizon, so complete table coverage is not expected. Terminal and zero-horizon values are different: their zeros are fixed by the task's boundary rules.

With learning rate `1 / visits`, each entry averages the targets it has seen. Those targets use continuation estimates that can change during training. This is not simply a mean of complete observed episode returns. [Exercise 7](EXERCISES.md#7-incremental-mean-return), `update_mean`, isolates the latter arithmetic; explain where its new return would come from before comparing the two uses.

In [`experience/sampling.rs`](../src/experience/sampling.rs), follow `sample_transition` only if you want the Rust detail: the sampler borrows choices, then copies the selected values into an owned observation. `ValueTable::observe` never calls the world or [exact evaluator](../src/values.rs).

</details>

**Stop or next:** note the command and the row you inspected. Next, [compare two budgets](#4-compare-budgets-and-seeds-20-minutes).

## 4. Compare budgets and seeds (20 minutes)

**Purpose:** distinguish a reproducible estimate from a consistently accurate one. Keep the policy, horizon, and discount fixed while changing only the sampling budget or seed.

**1. Predict:** must increasing an episode budget reduce the error for every seed?

**2. Do:** run 1,000 episodes with seed `2026` and note the absolute error.

```bash
cargo run -- learn-value --episodes 1000 --seed 2026
```

**3. Change one setting:** increase only the episode budget.

```bash
cargo run -- learn-value --episodes 10000 --seed 2026
```

**4. Check:** compare the errors. Keep a worse result in your notes too. This pair illustrates behavior of a particular run, not a statistical guarantee or a convergence proof.

**5. Explain:** why can an online learner's error rise at a particular budget even while it receives more observations? Distinguish a changing estimate from the fixed exact value being estimated.

<details>
<summary>Optional next comparison: use a different seed</summary>

Predict what should stay fixed, then change only the seed from the 10,000-episode run:

```bash
cargo run -- learn-value --episodes 10000 --seed 42
```

Check that the exact reference remains the same. To change the objective instead, predict the effect of discount zero and run:

```bash
cargo run -- learn-value --discount 0 --episodes 10000 --seed 42
```

Now the exact comparison must use that changed discount too. Before attributing a mismatch to sampling, check policy, discount, remaining horizon, terminal handling, and visit counts. Several seeds and budgets are more informative than one favorable run, but a few examples still provide no statistical accuracy guarantee.

</details>

<details>
<summary>Optional reference: previously measured seed and budget results</summary>

Integration runs on **2026-10-08** produced these absolute errors, rounded to six decimals. Every run used boldness `0.3`, horizon `5`, and discount `0.95`; the exact initial-state value was `3.103947335269`. Each run began with an empty table. The sampler used pinned `rand 0.9.2` and `rand_chacha 0.9.0` dependencies.

| Episodes | Seed 7 | Seed 42 | Seed 2026 |
| --- | --- | --- | --- |
| 100 | 1.349282 | 0.200549 | 0.884758 |
| 1,000 | 0.557351 | 0.119144 | 0.030353 |
| 10,000 | 0.199918 | 0.070203 | 0.118850 |

These historical measurements describe program behavior. They do not measure learner understanding, and reproducibility across arbitrary future program or RNG versions is not promised.

</details>

**Stop or next:** record the two settings and your explanation. Next, [try a new context](#5-apply-the-rule-without-the-trace).

## 5. Apply the rule without the trace

**Purpose:** check the rule independently of the supplied output. Close the source and earlier trace before attempting this case.

A navigation observation enters a terminal goal and gives reward `2`. Your old estimate is `4`, discount is `0.5`, and this is the entry's second visit.

**1. Predict:** write the target and updated estimate.

**2. Do:** check your arithmetic using `td_update` in the marked learner-case space for [exercise 8](EXERCISES.md#8-one-td-update). If you have not implemented that task, attempt it with the guide's support first; record the help used.

**3. Check:** run its exact test.

```bash
cargo test exercises::arithmetic::practice::exercise_8_td_update -- --exact --ignored
```

**4. Explain:** change the next state to a continuing state with estimate `6` and at least one remaining transition. Recalculate the target and explain which extra information now matters. Check the arithmetic independently as well as testing your implementation.

**Stop or next:** record correctness, explanation, and assistance in the optional [assessment](LEARNING_RECORD.md#record-an-attempt). After an attempt, compare only the matching `td_update` function in the [separate solutions](../src/exercises/arithmetic/solutions.rs). At the next available session, try one [delayed prompt](LEARNING_RECORD.md#next-available-session); its feedback remains separate.

Working code, independent explanation, and delayed performance are different evidence. Learner outcomes remain pending until you actually attempt and assess them.
