# Learn a value from observations

This capstone connects the probability model to **tabular TD(0) evaluation of a fixed policy**. The policy's boldness stays fixed while a table of value estimates changes. No improved policy or neural network is learned.

Start after you can explain a discounted return, a terminal transition, and one TD target. If one of those is unclear, revisit only the matching task in [the exercise guide](EXERCISES.md). Use [the learning record](LEARNING_RECORD.md) to distinguish an assisted attempt from an independent one.

## 1. Separate the responsibilities

| Part | Receives | Does |
| --- | --- | --- |
| Sampler | Policy, world model, seed, and horizon | Samples actions and consequences to produce observed transitions. |
| Learner | Observations, remaining steps, and discount | Updates a value table and visit counts. It does not inspect transition probabilities or exact values. |
| Exact evaluator | The same policy, world, initial state, horizon, and discount | Computes a separate reference value for comparison after training. |

The environment simulator uses the known model, while the learner sees observations. This boundary is what makes the experiment learning from experience even though the toy world was written by hand. Inspect [`ValueTable::observe` in experience.rs](../src/experience.rs) beside the separate [exact evaluator](../src/values.rs).

The learned entry is `V(h, state)`, where `h` is the number of transitions remaining. A state with one step remaining and the same state with five remaining can have different values. Treating those as one table entry would change the objective.

## 2. Predict one update before training (15 minutes)

All commands run **from the repository root**. Open the supplied trace in [`experience.rs`](../src/experience.rs), identify one update's inputs, and predict its target and new estimate before running:

```bash
cargo run -- td-trace
```

<details>
<summary>Conceptual hint before reading the output</summary>

Separate the observed reward from the continuation estimate already stored in the table. Identify which entry owns the visit count before computing a learning rate.

</details>

These are separate deterministic update records with supplied rewards, not one world-generated episode. For the row you predicted, explain the current estimate, observed reward, continuation estimate, visit count, and learning rate. Compare your prediction with the printed update.

The update uses a TD target: immediate reward plus discounted continuation value when continuation exists. The learning rate is `1 / visits`, using the count for the particular `(h, state)` entry, including the current visit. The continuation estimate can change during training, so this is not simply averaging complete episode returns.

**Explain aloud:** why must visits to another state or another horizon leave this entry's count unchanged? Which value is supplied by observation and which comes from the learner's current table?

True termination and `h = 0` end continuation for this finite-horizon objective. The last reward still counts. An episode stops at termination or the selected horizon; there is no separate interruption limit in this lesson. This differs from externally truncating an otherwise continuing task.

## 3. Run one reproducible experiment (20 minutes)

**Predict:** should a finite sample produce exactly the Bellman value, and does an unvisited entry deserve to be described as learned?

```bash
cargo run -- learn-value --boldness 0.3 --horizon 5 --discount 0.95 --episodes 10000 --seed 7
```

Read the initial-state estimate, exact reference, their difference, and the visit counts. A zero-initialized entry with no visits has no empirical support. Some displayed entries are unreachable from the chosen initial state and horizon, so complete table coverage is not expected. Terminal and zero-horizon values, in contrast, are fixed by the task's boundary rules.

Repeat the same command to check reproducibility for the same program and parameters. A seed controls the random sequence; it is not a guarantee that the resulting estimate is accurate. Reproducibility across arbitrary future RNG or program versions is not promised.

Use `cargo run -- --help` for the authoritative command defaults and bounds. This lesson holds boldness `0.3`, horizon `5`, and discount `0.95` fixed unless a step explicitly changes them.

## 4. Compare budgets and seeds (20 minutes)

**Predict:** which changes affect sampling variability, and which change the value being estimated?

```bash
cargo run -- learn-value --episodes 100 --seed 7
cargo run -- learn-value --episodes 10000 --seed 7
cargo run -- learn-value --episodes 100 --seed 42
cargo run -- learn-value --episodes 10000 --seed 42
```

Keep boldness, horizon, and discount fixed while comparing seed and episode budget. Record the actual errors, including any worse result. Error need not decrease at every budget or for every seed; these short runs do not prove convergence. Two seeds are a useful first comparison, not a statistical accuracy guarantee.

Integration runs on **2026-10-08** produced the absolute errors below, rounded to six decimals. Every run used boldness `0.3`, horizon `5`, and discount `0.95`; the exact initial-state value was `3.103947335269`. Each run began with an empty learned table. The seeded sampler uses pinned `rand 0.9.2` and `rand_chacha 0.9.0` dependencies.

| Episodes | Seed 7 | Seed 42 | Seed 2026 |
| --- | --- | --- | --- |
| 100 | 1.349282 | 0.200549 | 0.884758 |
| 1,000 | 0.557351 | 0.119144 | 0.030353 |
| 10,000 | 0.199918 | 0.070203 | 0.118850 |

Seed 2026 has more error at 10,000 episodes than at 1,000. Reproduce that comparison before explaining why a favorable single run is insufficient:

```bash
cargo run -- learn-value --episodes 1000 --seed 2026
cargo run -- learn-value --episodes 10000 --seed 2026
```

These are observations about this program, not measurements of learner understanding or a statistical convergence study.

Now change only the discount and check that the exact reference changes to the same objective:

```bash
cargo run -- learn-value --discount 0 --episodes 10000 --seed 7
```

**Explain aloud:** what could cause a mismatch besides sampling variation? Check parameter agreement, remaining horizon, terminal handling, and visit counts before blaming the learning algorithm.

## 5. Apply the rule without the trace

Close the source and output. In a navigation task, a transition enters a terminal goal and produces a reward. Explain which parts of a TD target still apply, then write one update using your own finite numbers. Change the transition to a continuing state and explain what additional information is needed.

Advance after an unaided explanation and a correct changed case. If a concept is unclear, use the smallest matching worked example in the [separate solutions](../src/exercises/arithmetic/solutions.rs), then try a fresh case. Keep confidence as a separate observation, not a pass threshold. Record assistance and return for a [delayed check](LEARNING_RECORD.md#next-available-session).

The CLI tests and numerical reference checks establish program behavior for their covered cases. Your prediction, explanation, and delayed changed-case attempt establish different evidence about your learning. Learner results remain pending until you make and record those attempts.
