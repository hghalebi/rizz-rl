# Ten small Rust exercises for RL

Build one function at a time in [`src/exercises/arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs). The [learning path](LEARNING_PATH.md#session-6-write-the-small-pieces-yourself-25-minutes) introduces tasks 1 to 3 and places this bank in the wider route. Each task isolates one idea so you can explain it before combining it with others.

You need slices, `for` loops, tuples, conditionals, and basic `Option` matching. You do not need an RL library, a GPU, or a training environment. The functions use ordinary `f64` arithmetic and borrowed inputs.

## Attempt one exercise

All commands run **from the repository root**. For example, select exercise 4:

```bash
cargo test exercises::arithmetic::practice::exercise_4_is_probability_distribution -- --exact --ignored
```

Use the full selector printed under each task. `--exact` prevents accidental matches with another exercise. An untouched `todo!()` makes the selected starter fail intentionally; untouched TODOs elsewhere remain compilable. Filtering isolates execution, not compilation: a syntax or type error anywhere in the same test target can block your selected task.

1. Read the function's contract and answer its prediction below on paper.
2. Attempt an implementation without opening the worked solutions.
3. Run the selected tests, then explain one passing case and one boundary case aloud.
4. Add one small case of your own and predict it before running it.

Normal project checks skip the practice tests. Passing those checks does not mean you have completed the exercises. To check all your attempts explicitly:

```bash
cargo test exercises::arithmetic::practice:: -- --ignored
```

## Choose the next small step

The project links connect each task to a larger concept. Use them for review after attempting the task; they may contain worked explanations.

| Exercise | Objective | Rust focus | Time | Project connection |
| --- | --- | --- | --- | --- |
| 1. Discounted return | Score rewards across time. | Borrowed slices, loop state | 10 min | [`discounted_return`](../src/planning/returns.rs) |
| 2. Weighted expectation | Combine alternative outcomes. | Tuple destructuring | 10 min | [`expected_value`](../src/probability/expectation.rs) |
| 3. TD target | Distinguish terminal and continuing transitions. | Explicit branching | 10 min | [Terminal states](DEEP_RL_COMPANION.md#23-terminal-and-absorbing-states) |
| 4. Valid probabilities | Reject invalid probability distributions. | Predicates, finite values | 15 min | [`Distribution::total_probability`](../src/probability/distribution.rs) |
| 5. Greedy action | Select an action from estimated values. | Indices, `Option<usize>` | 10 min | [Policy improvement](DEEP_RL_COMPANION.md#39-policy-evaluation-versus-policy-improvement) |
| 6. Epsilon-greedy policy | Allocate action probabilities. | `Vec<f64>`, indexing | 15 min | [Exploration](DEEP_RL_COMPANION.md#50-exploration-versus-exploitation) |
| 7. Running mean | Incorporate another observed return. | Counts and numeric conversion | 10 min | [Monte Carlo evaluation](DEEP_RL_COMPANION.md#43-monte-carlo-evaluation) |
| 8. TD update | Move an estimate toward a target. | Function reuse | 10 min | [TD learning](DEEP_RL_COMPANION.md#44-temporal-difference-learning) |
| 9. Q-learning target | Use the best available next-action estimate. | `Option<f64>`, empty slices | 15 min | [Q-learning](DEEP_RL_COMPANION.md#47-q-learning) |
| 10. Bellman backup | Average immediate and future value. | Three-field tuples | 20 min | [`values::evaluate_value`](../src/values.rs) |

Each card below supplies a prerequisite, prediction, exact command, optional hint, and changed case. Edit only its named function in the practice file. After an attempt, use the [separate solution instructions](#after-your-attempt-worked-solutions); then close the solution before the changed case. Choose another task or a delayed retrieval prompt from your observed difficulty.

## Input contracts

Exercise 4 is explicitly about **validating untrusted probabilities**. For the other exercises, assume finite, reasonably sized numbers; discounts, epsilon, and learning rates lie in `[0, 1]`. Arithmetic exercises do not need to reimplement that validator. Read each function's contract for empty inputs and terminal behavior.

Nonempty probability inputs for exercises 2 and 10 are normalized. Their empty inputs return a weighted sum of zero by contract, even though an empty list is not a valid probability distribution in exercise 4. A mathematical helper's empty-sum behavior and an application's validation rule serve different purposes.

## 1. Discounted return

**Prerequisite:** Session 2 and a borrowed slice.

Implement `discounted_return`. The first reward is immediate; later rewards receive progressively stronger discounting. An empty reward slice returns zero.

**Predict:** for `[2.0, -1.0, 4.0]`, which rewards affect the result when the discount is zero?

```bash
cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Identify which reward arrives before any delay.

</details>

**Explain and change:** Use a new sequence with a delayed penalty; justify both discount endpoints.

## 2. Weighted expectation

**Prerequisite:** Session 1's alternative outcomes and tuple fields.

Implement `weighted_expectation` for `(value, probability)` pairs. Preserve that field order. It scores alternative outcomes, rather than consecutive rewards along one path.

**Predict:** does reversing the order of the pairs change the mathematical expectation, and what happens to an outcome with zero probability?

```bash
cargo test exercises::arithmetic::practice::exercise_2_weighted_expectation -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Inspect the meaning of each tuple field before multiplying numbers.

</details>

**Explain and change:** Split one outcome into two identical outcomes whose probabilities have the same total.

## 3. Terminal-aware TD target

**Prerequisite:** Session 3 and the difference between a target and a stored estimate.

Implement `td_target`. A true task termination removes the contribution from `next_value`. An observation cutoff in a continuing task uses `terminated = false`; a finite task whose deadline ends its objective really does terminate.

**Predict:** if reward and next-state estimate stay the same, can changing only `terminated` change the target?

```bash
cargo test exercises::arithmetic::practice::exercise_3_td_target -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Ask whether any future value belongs to the stated task after this transition.

</details>

**Explain and change:** Reuse your numbers in a deadline-limited robot task and a continuing task with a logging cutoff.

## 4. Probability validation

**Prerequisite:** Finite numbers, boolean predicates, and an absolute tolerance.

Implement `is_probability_distribution`. Accept only a nonempty list of finite values in `[0, 1]` whose sum differs from one by at most `1e-9`. Reject `NaN`, infinities, negative weights, weights above one, and an incorrect total.

**Predict:** is checking the total alone enough to reject `[-0.25, 1.25]`?

```bash
cargo test exercises::arithmetic::practice::exercise_4_is_probability_distribution -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

A correct total and valid individual weights are different conditions.

</details>

**Explain and change:** Construct one invalid list that has total one and a different invalid list whose entries individually look valid.

## 5. Greedy action

**Prerequisite:** Basic indices and `Option`; estimated values may be negative.

Implement `greedy_action`. Return the index of the largest action value; choose the lowest index on a tie. An empty slice returns `None`. Negative values are ordinary estimates and must participate correctly.

**Predict:** which index should win for `[-9.0, -4.0, -4.0]`, and why would initializing a best value to zero be risky?

```bash
cargo test exercises::arithmetic::practice::exercise_5_greedy_action -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

What does the first available action tell you that a made-up initial value does not?

</details>

**Explain and change:** Use a new all-negative list with a tied best value and explain the selected index.

## 6. Epsilon-greedy probabilities

**Prerequisite:** Task 5 and a vector of one probability per action.

Implement `epsilon_greedy`. Exploration spreads probability `epsilon` uniformly across **all** actions, including the greedy action. The remaining probability belongs to the greedy action selected with exercise 5's tie rule. Empty input returns an empty vector. The function returns probabilities; it does not sample an action.

**Predict:** with three actions and one clear winner, how should the distribution change between epsilon zero and epsilon one?

```bash
cargo test exercises::arithmetic::practice::exercise_6_epsilon_greedy -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Exploration can choose the greedy action too. Account for the whole probability budget.

</details>

**Explain and change:** Try a new tied input and both epsilon endpoints; explain probability conservation.

## 7. Incremental mean return

**Prerequisite:** A mean summarizing earlier returns and a count including the new sample.

Implement `update_mean` without storing previous returns. `sample_count` is at least one and includes the **new** sample. `previous_mean` describes the earlier samples.

**Predict:** when `sample_count` is one, should an arbitrary initial `previous_mean` influence the result?

```bash
cargo test exercises::arithmetic::practice::exercise_7_update_mean -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Compare the new observation with the summary of the observations that came before it.

</details>

**Explain and change:** Start a new two-sample sequence with an arbitrary initial mean and explain the count at each call.

## 8. One TD update

**Prerequisite:** Task 3 and the distinction between a target and an update.

Implement `td_update`. Use the current value, transition information, and learning rate `alpha` to return an updated estimate. Reuse your terminal-aware target function so the two tasks agree about episode boundaries.

**Predict:** how should alpha zero and alpha one differ when the target differs from the current estimate?

```bash
cargo test exercises::arithmetic::practice::exercise_8_td_update -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

The distance to the target and the fraction of that distance used are separate quantities.

</details>

**Explain and change:** Use a terminal transition and two different learning rates; explain which inputs should matter.

## 9. Q-learning target

**Prerequisite:** Tasks 3 and 5, plus matching an `Option`.

Implement `q_learning_target`. The continuation uses the highest value among available next actions. A terminal transition returns `Some(reward)`, even with an empty action slice. A continuing transition with no next actions returns `None`, including when discount is zero: that input is still missing the required action set.

**Predict:** why is an all-negative next-action slice different from an empty next-action slice?

```bash
cargo test exercises::arithmetic::practice::exercise_9_q_learning_target -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

No available action is missing information; a poor action still has a value.

</details>

**Explain and change:** Compare an empty continuation with an all-negative continuation and explain the different result types.

## 10. One Bellman backup

**Prerequisite:** Tasks 1 and 2, with a fixed policy already included in the weights.

Implement `bellman_backup` for `(probability, reward, next_value)` tuples. Notice that probability is the **first** field here and already includes both policy and world probabilities. Combine immediate rewards with discounted continuation values, then average across outcomes. An empty slice returns zero; terminal outcomes must be supplied with `next_value = 0`.

**Predict:** if every continuation value is zero, what information still determines the result?

```bash
cargo test exercises::arithmetic::practice::exercise_10_bellman_backup -- --exact --ignored
```

<details>
<summary>Conceptual hint</summary>

Keep the weight of an alternative separate from the time weight of its continuation.

</details>

**Explain and change:** Use a new pair of outcomes, one terminal, and explain both kinds of weights.

## After your attempt: worked solutions

The separate [worked solutions file](../src/exercises/arithmetic/solutions.rs) contains complete implementations and explanations. Open the function you attempted, compare its reasoning with yours, then close it and reconstruct your version. The solutions are available whenever you choose; keeping them separate lets you attempt the problem first.

Test one worked solution without `--ignored`:

```bash
cargo test exercises::arithmetic::solutions::exercise_4_is_probability_distribution -- --exact
```

Run all reference tests or inspect the worked demonstrations:

```bash
cargo test exercises::arithmetic::solutions::
cargo test exercises::arithmetic::worked::demonstrations -- --exact --nocapture
```

Practice and solutions use the same [contract cases](../src/exercises/arithmetic/cases.rs). Those cases contain expected results, so read them when you want to understand a failure or check your prediction. Passing them supports correctness for the covered cases; your explanation and an additional case help check understanding.

## Finish with a transfer check

These are building blocks, not a complete trained agent. Running means need observed returns; TD and Q-learning updates need transitions and repeated updates; an epsilon-greedy distribution needs sampling and an environment loop. The existing planner still evaluates a known model exactly.

Use the [learning record](LEARNING_RECORD.md) for your prediction, correctness, explanation, assistance, confidence, and exact next action. At the next available session, recreate one solved function from memory and explain how its inputs would arise in a learning loop. Record the actual delay. Return to the [learning path](LEARNING_PATH.md#composition-lesson-combine-the-pieces-20-to-40-minutes) when ready to combine ideas.
