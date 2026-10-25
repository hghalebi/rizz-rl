# Your Rust + Hugging Face RL learning path

**First visit:** start with [one decision](#session-1-follow-one-decision-15-minutes). **Returning:** open your last [quick restart note](LEARNING_RECORD.md#quick-restart) and follow its next action.

This page owns the learning route. All commands run **from the repository root**; setup and checks live in the [lab README](../README.md). Choose one activity, not the whole page. The time estimates are optional planning aids. If you have only a few minutes, make the prediction and leave a restart note.

| Activity | What you will do |
| --- | --- |
| [1. One decision](#session-1-follow-one-decision-15-minutes) | Follow one action and its consequence. |
| [2A. Return](#return-activity) | Score one reward sequence, then attempt exercise 1. |
| [2B. Expectation](#expectation-activity) | Weight alternative outcomes, then attempt exercise 2. |
| [3. Task endings](#session-3-find-the-end-of-the-episode-20-minutes) | Distinguish termination from a horizon cutoff. |
| [4. Bellman values](#session-4-replace-repeated-futures-with-values-25-minutes) | Reuse a smaller future, then attempt exercise 10. |
| [5. Policy comparison](#session-5-compare-policies-without-overclaiming-20-minutes) | Compare a fixed grid of behavior rules. |
| [6. Recall checkpoint](#session-6-write-the-small-pieces-yourself-25-minutes) | Explain a fresh case without the reference. |
| [Composition](#composition-lesson-combine-the-pieces-20-to-40-minutes) | Connect weighted branches with Rust ownership. |
| [Experience](#experience-capstone-learn-values-from-observations-25-minutes) | Update values from observations. |

## Choose the support you need

If the difficulty is **Rust ownership**, use the [ownership probe](COMPOSITION_LESSON.md#1-optional-diagnostic-assess-two-skills-separately). If it is **conditional probability**, use the probability probe in that same section. Follow only the support your attempt calls for, then return to the current activity. Neither probe is a prerequisite exam.

For a learning check, use an unaided explanation and a correct changed case. Confidence alone is not a pass mark. Stopping is always allowed, including midway through a task. Record help used; a successful assisted attempt is useful practice, but does not establish independent recall.

## Session 1: Follow one decision (15 minutes)

**Goal:** separate what the policy chooses from what the world does next.

**You need:** only the table below; no Rust yet. Allow 15 minutes; stopping after any step is fine.

Here is the local model at `JustMet`. Each response probability is conditional on its row's action:

| Action | Chance of choosing it | Engaged afterward | LowInterest afterward |
| --- | ---: | ---: | ---: |
| Ask | 0.70 | 0.80 | 0.20 |
| Story | 0.30 | 0.60 | 0.40 |

**1. Predict:** what is the probability of the complete path `JustMet -> Ask -> Engaged`?

**2. Do:** run the one-step experiment.

```bash
cargo run -- first-step
```

**3. Check:** find the row `JustMet --Ask / +1.0--> Engaged`. Its percentage is the path probability; compare it with your prediction. The four branch percentages should add up to 100%. Leave the reward line at the bottom for Activity 2B.

**4. Explain:** on paper, change the action probabilities to Ask `0.5` and Story `0.5`, keeping the response probabilities fixed. Predict the new Ask-to-Engaged path. Explain which probability changed and which did not.

<details>
<summary>Stuck on the probability? (no Rust)</summary>

A complete path needs both its action and its conditional response. This does not assume the response is independent of the action. Different paths reaching the same event contribute separately to its total probability. The [probability probe](COMPOSITION_LESSON.md#1-optional-diagnostic-assess-two-skills-separately) offers one more small case.

</details>

<details>
<summary>After your attempt: see it in the code</summary>

Compare with the [worked one-step calculation](DEEP_RL_COMPANION.md#36-a-complete-one-step-calculation). The table's numbers come from the `JustMet` arm in [`Policy::possible_actions`](../src/model/policy.rs) and the `(JustMet, Ask)` / `(JustMet, Story)` arms in [`DatingWorld::possible_next_states`](../src/model/world.rs).

**From the table to `and_then`.** [`possible_transitions`](../src/planning/transitions.rs) builds the four rows that `first-step` prints, with [`Distribution::and_then`](../src/probability/composition.rs):

| Stage | Produces | Rows |
| --- | --- | --- |
| `policy.possible_actions(state)` | `Ask @ 0.70`, `Story @ 0.30` | 2 |
| `.and_then(...)` | for each action, every world response, at chance `action chance · response chance` | 4 |
| inside it, `.map(...)` | each branch as a `Transition` with its reward | the 4 printed rows |

In Rust, `State` and `Action` are enum alternatives, and a `Vec<(T, f64)>` holds weighted branches. [Exercise 4](EXERCISES.md#4-probability-validation) is optional practice in validating those weights.

**Confirm:** [`first_step_joint_probabilities_match_the_hand_calculation`](../src/tests.rs) checks this claim in code: the paths `JustMet -> Ask -> Engaged` and `JustMet -> Story -> LowInterest` have probabilities 0.56 and 0.12. Run it after your attempt, then explain why each assertion follows:

```bash
cargo test tests::first_step_joint_probabilities_match_the_hand_calculation -- --exact
```

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `cargo run -- first-step`, then step 4 on paper. The default next activity is [return](#return-activity).

<a id="session-2-score-a-future-then-score-uncertainty-20-minutes"></a>
## Session 2: Return and expectation, one activity at a time

<a id="return-activity"></a>
### 2A. Score one reward sequence

**Goal:** turn the rewards along one path into one number, the return.

**You need:** a slice and a `for` loop, nothing from the planner. Allow 10 to 20 minutes; stopping after any step is fine.

Use rewards `[1, 2, 4]` and discount `0.5`. The first reward counts immediately. Each step of delay multiplies a reward by one more factor of the discount. These are consecutive rewards on one path, not alternative outcomes.

**1. Predict:** calculate this path's return on paper. Which rewards would still matter if the discount were zero?

**2. Do:** in [`src/exercises/arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), implement only `discounted_return`. Its comment states the empty-input rule. Leave the other TODOs as they are.

**3. Check:** run its exact starter test.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_1_discounted_return \
  -- --exact --ignored
```

<a id="test-signals"></a>

| You see | It means |
| --- | --- |
| `not yet implemented: Exercise 1: implement discounted return` | The expected start: the TODO is still there. |
| `test result: ok. 1 passed` | Your function passes the shared cases. Go to step 4. |
| `test result: ok. 0 passed` with `1 ignored`, or `running 0 tests` | Nothing was checked: `--ignored` or part of the name is missing. Recopy the whole command. |
| A compiler error or an assertion mismatch | Use the [feedback table](EXERCISES.md#read-the-feedback). |

**4. Explain:** for the new case `[2, -1, 4]`, predict the return at discount `0` and at `1`. Add each prediction as an assertion at the `// YOUR CASE` comment in `exercise_1_discounted_return`, then rerun step 3. Explain why the first reward is never discounted.

<details>
<summary>Stuck on the return idea? (math, no Rust)</summary>

Ask which weight applies before any time has passed, then how that weight changes after one more step. The [exercise-1 card](EXERCISES.md#1-discounted-return) has a different two-reward example to complete on paper.

</details>

<details>
<summary>Stuck on the Rust? (borrowing, no math)</summary>

`_rewards: &[f64]` is a borrowed view of the caller's numbers: your function reads them, and the caller keeps them. The leading underscore only silences the starter's unused-parameter warning; remove it once you use the parameter. A `for` loop over a `&[f64]` hands you a `&f64` on each turn. If the compiler mentions `&f64`, that is a borrowing question, not a mistake in your return formula. For more practice with moves and borrows, use the [ownership probe](COMPOSITION_LESSON.md#1-optional-diagnostic-assess-two-skills-separately).

</details>

<details>
<summary>After your attempt: compare, then read the production version</summary>

Compare with `discounted_return` in the [separate solutions](../src/exercises/arithmetic/solutions.rs). Then print the same sequence one term per row:

```bash
cargo run -- returns --discount 0.5
```

<a id="loop-to-pipeline"></a>
**From your loop to the pipeline.** The production [`discounted_return`](../src/planning/returns.rs) does the same work as your loop, in four stages. Each stage produces one column of that output:

| Stage | Produces | For `[1, 2, 4]` at `0.5` | Column |
| --- | --- | --- | --- |
| `discount_weights(0.5)` | weights, made one at a time on request | `1, 0.5, 0.25, ...` | `weight` |
| `.zip(trajectory)` | `(weight, &Transition)` pairs | `(1, &step 0)`, `(0.5, &step 1)`, `(0.25, &step 2)` | one row each |
| `.map(...)` | contributions, `weight * transition.reward` | `1, 1, 1` | `contribution` |
| `.fold(0.0, ...)` | the accumulated total, starting at 0 | `0 -> 1 -> 2 -> 3` | `Discounted return G` |

**Where borrowing happens:** the function takes `trajectory: &[Transition]`, so the caller keeps its history. `zip` walks that borrowed slice, so each pair holds a `&Transition`, and `transition.reward` copies one `f64` out of it.

**Why it stops:** `discount_weights` never ends on its own, but it is lazy: it computes a weight only when `zip` asks for the next pair. `zip` stops as soon as either side runs out, so the three-step trajectory ends the pipeline after three pairs. Background: the [Rust Book on iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) and the [`zip` contract](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip).

**Transfer:** if the history has one transition, how many contributions reach `fold`, and what stops the iterator?

**Confirm:** [`the_greedy_action_can_depend_on_remaining_horizon`](../src/values/tests.rs) checks this claim in code: the same supplied history is worth 0 with one step counted but 2.5 with two. Run it after your attempt, then explain why each assertion follows:

```bash
cargo test values::tests::the_greedy_action_can_depend_on_remaining_horizon -- --exact
```

The [return reference](DEEP_RL_COMPANION.md#33-discounted-return) adds the notation.

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart). For this activity it can read:

```text
Where I stopped: Activity 2A, discounted_return.
What I observed or still do not understand: one line, plus any help used.
Next action: src/exercises/arithmetic/practice.rs, discounted_return, then run
  cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
```

The default next activity is [expectation](#expectation-activity).

<a id="expectation-activity"></a>
### 2B. Score alternative outcomes

**Goal:** score alternative outcomes with one probability-weighted number, the expectation.

**You need:** Activity 2A's difference between rewards along one path and a choice between paths, and a pair `(value, probability)`. Allow 10 to 20 minutes; stopping after any step is fine.

Suppose one outcome is worth `8` with probability `0.75`, and another is worth `-2` with probability `0.25`. These are alternatives. Expectation weights each value by how likely that outcome is; it does not apply time discounting again.

**1. Predict:** calculate the expected value. Must it equal either possible outcome?

**2. Do:** in [`src/exercises/arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), implement only `weighted_expectation`. Its pairs are `(value, probability)`, in that order.

**3. Check:** run just exercise 2.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_2_weighted_expectation \
  -- --exact --ignored
```

Read the result with the [signal table](#test-signals) from Activity 2A. Here the expected start is `not yet implemented: Exercise 2: implement weighted expectation`.

**4. Explain:** split the first outcome into two rows of value `8`, with probabilities `0.25` and `0.50`. Keep the other row. Predict whether the expectation changes, then add that case at the `// YOUR CASE` comment in `exercise_2_weighted_expectation`. Explain why dividing by the number of rows would answer the wrong question.

<details>
<summary>Stuck on the expectation idea? (math, no Rust)</summary>

Keep time weights inside a path and probability weights across alternatives. For an empty input, the exercise's weighted sum is zero; this does not make an empty list a valid probability distribution. The [exercise-2 card](EXERCISES.md#2-weighted-expectation) has a hint and an analogous example to complete on paper.

</details>

<details>
<summary>Stuck on the Rust? (pairs and borrowing, no math)</summary>

`_outcomes: &[(f64, f64)]` borrows the caller's pairs; as in 2A, drop the leading underscore once you use it. Destructuring a pair, as in `let (value, probability) = pair;`, names both fields so their order cannot be mixed up. If the compiler mentions `&(f64, f64)`, that is a borrowing question, not a probability one.

</details>

<details>
<summary>After your attempt: compare, then read the production version</summary>

Compare with `weighted_expectation` in the [separate solutions](../src/exercises/arithmetic/solutions.rs). The production [`expected_value`](../src/probability/expectation.rs) does the same work in two stages: `.map(...)` turns each `(outcome, probability)` into `probability * value_of(outcome)`, and `.sum()` adds them. Its `value_of` is a `Fn(&T) -> f64`, so the scoring function borrows each outcome instead of taking it.

You have seen this calculation printed already. In `cargo run -- first-step`, multiply each row's percentage by its reward, add the four results, and compare with the `Expected immediate reward` line. The [expectation reference](DEEP_RL_COMPANION.md#8-expected-value-collapse-uncertainty-into-one-comparison-number) connects this to policy evaluation.

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `src/exercises/arithmetic/practice.rs`, `weighted_expectation`, then `cargo test exercises::arithmetic::practice::exercise_2_weighted_expectation -- --exact --ignored`. The default next activity is [task endings](#session-3-find-the-end-of-the-episode-20-minutes).

## Session 3: Find the end of the episode (20 minutes)

**Goal:** tell the world ending apart from the calculation stopping.

**You need:** the return idea from Activity 2A; no Rust. Allow 20 minutes; stopping after any step is fine.

Here `Goodbye` is terminal. `MutualInterest` still has a final transition to `Goodbye`. The horizon is the number of transitions left in the objective; it can reach zero while the world state is nonterminal.

**1. Predict:** what should an evaluation with zero remaining transitions score?

**2. Do:** evaluate that boundary.

```bash
cargo run -- evaluate --horizon 0
```

**3. Check:** find `Represented trajectories` and `Expected return E[G]`, and compare them with your prediction. The one trajectory there contains no transitions yet. That differs from erasing a reward already received on entering a terminal state.

**4. Explain:** a robot task ends at its deadline. A different robot keeps working, but its log file stops. Explain why only the first event necessarily ends the task's future reward.

<details>
<summary>Stuck on the stopping rules? (math, no Rust)</summary>

Ask separately whether the world can continue and whether this objective counts value beyond the remaining horizon. `MutualInterest` and `Goodbye` have different roles here. The [terminal-state reference](DEEP_RL_COMPANION.md#23-terminal-and-absorbing-states) expands the distinction.

</details>

<details>
<summary>Stuck on the Rust? (ownership in the rollout, no math)</summary>

In [`extend_one_step`](../src/planning/rollout.rs), identify the consumed path, the copied policy and world, and the cloned histories. `Policy` and `DatingWorld` are small `Copy` values, so passing one hands over a copy. Each branch clones the shared past because moving it into one child would leave none for the others.

</details>

<details>
<summary>After your attempt: one more step, then see it in the code</summary>

Predict one-step behavior, then run:

```bash
cargo run -- evaluate --horizon 1
```

A longer horizon can include penalties as well as rewards; it need not improve the score. [`State::is_terminal`](../src/model/state.rs) defines the task ending, and [`extend_one_step`](../src/planning/rollout.rs) preserves terminal histories.

**Confirm:** [`reaching_goodbye_stops_history_without_losing_probability`](../src/tests.rs) checks this claim in code: every history from `LowInterest` has exactly one step, ends in `Goodbye`, and still keeps its reward of 2. Run it after your attempt, then explain why each assertion follows:

```bash
cargo test tests::reaching_goodbye_stops_history_without_losing_probability -- --exact
```

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `cargo run -- evaluate --horizon 0`, then step 4. The default next activity is [Bellman values](#session-4-replace-repeated-futures-with-values-25-minutes).

## Session 4: Replace repeated futures with values (25 minutes)

**Goal:** reuse the answer to a smaller remaining problem.

**You need:** return (2A), expectation (2B), and the two stopping rules (Session 3). Allow 25 minutes; stopping after step 4 is fine.

If two histories reach the same state with the same number of steps left, this model gives them the same future value. A Bellman evaluator calculates that continuation once and reuses it. The immediate reward still counts separately.

**1. Predict:** should it agree with enumerating every path when policy, model, discount, and horizon match?

**2. Do:** compare both exact methods through two remaining steps.

```bash
cargo run -- compare --horizon 2
```

**3. Check:** read the two value columns. The absolute difference measures numerical disagreement; `cached subproblems` is a count, not a return. Small rounding differences are expected.

**4. Explain:** why would a cache keyed only by state confuse a one-step question with a two-step question? Agreement between algorithms still does not show that this invented world is realistic.

<a id="bellman-practice"></a>
**5. Do one backup:** in [`src/exercises/arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), implement `bellman_backup`. Each tuple is `(probability, immediate_reward, next_value)`. Probability already includes the policy and world. Predict the result for `[(0.5, 2.0, 10.0), (0.5, -2.0, 0.0)]` at discount `0.9` before coding.

**6. Check:** run only exercise 10.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_10_bellman_backup \
  -- --exact --ignored
```

Read the result with the [signal table](#test-signals); here the expected start is `not yet implemented: Exercise 10: compute a Bellman expectation backup`. Then change both continuation values to zero and explain which information still determines the answer. The [exercise-10 card](EXERCISES.md#10-one-bellman-backup) has local help and the separate solution link.

<details>
<summary>Stuck on the recurrence? (math, no Rust)</summary>

**One case by hand:** take `V(1, JustMet)` with boldness `0.3` and discount `0.95`. Use the chances from the Session 1 table, with reward `+1` for reaching `Engaged` and `-2` for reaching `LowInterest`. Calculate the expected immediate reward, and say what continuation value is left when no steps remain after this one. Then inspect the `H = 1` row:

```bash
cargo run -- compare --horizon 1
```

**Transfer:** which part of your calculation changes at horizon `2`? The [dynamic-programming reference](DEEP_RL_COMPANION.md#40-dynamic-programming) gives the full recurrence.

</details>

<details>
<summary>Stuck on the Rust? (the cache, no math)</summary>

The cache is a `HashMap<(State, usize), f64>`: its key is the whole question, the state and the steps left. `value_for` takes `cache: &mut ValueCache` and passes that one mutable borrow down every recursive call, so all calls share one table instead of copying it.

</details>

<details>
<summary>After your attempt: see it in the code</summary>

Read [`values::evaluate_value`](../src/values.rs) beside [`expected_return_of`](../src/planning/evaluation.rs). Both average actions under the supplied policy; taking a maximum would answer a different question. The cached key includes state and remaining horizon, while policy and discount stay fixed for this evaluation. `value_for` is your `bellman_backup` with one change: each next value comes from a recursive call with one step fewer.

**Confirm:** [`bellman_matches_complete_tree_across_states_horizons_and_parameters`](../src/values/tests.rs) is the broader software check: both evaluators agree in all 360 combinations of state, horizon, discount, and boldness it tries. Run it afterward; its cases are not a prediction task:

```bash
cargo test values::tests::bellman_matches_complete_tree_across_states_horizons_and_parameters -- --exact
```

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `src/exercises/arithmetic/practice.rs`, `bellman_backup`, then `cargo test exercises::arithmetic::practice::exercise_10_bellman_backup -- --exact --ignored`. The default next activity is [policy comparison](#session-5-compare-policies-without-overclaiming-20-minutes).

## Session 5: Compare policies without overclaiming (20 minutes)

**Goal:** tell evaluating one behavior rule apart from searching among rules.

**You need:** Session 4's idea of one policy's value; no Rust. Allow 20 minutes; stopping after any step is fine.

The `sweep` command checks eleven boldness values from `0` to `1` in steps of `0.1`, under identical world and horizon settings.

**1. Predict:** must the largest boldness have the highest expected return?

**2. Do:** run the grid comparison.

```bash
cargo run -- sweep
```

**3. Check:** find the `Best on this grid` line and compare its boldness with your prediction. It establishes the best on this grid for these settings, not a global optimum.

**4. Explain:** what evidence would distinguish trying known policies in a model from changing behavior through experience? A stochastic policy alone does not establish a learning exploration strategy, and a good mean does not guarantee a good individual outcome.

**5. Change one setting:** predict whether the winner must stay the same with horizon `1`, then check:

```bash
cargo run -- sweep --horizon 1
```

<details>
<summary>Stuck on what the search shows? (no Rust)</summary>

A search can establish a winner only among the candidates it evaluates. Separately, ask whether the policy changes in response to observations. Use the [grid reference](DEEP_RL_COMPANION.md#38-grid-search) and the [exploration distinction](DEEP_RL_COMPANION.md#important-nuance-stochasticity-is-not-automatically-exploration).

</details>

<details>
<summary>After your attempt: see it in the code</summary>

[`evaluate_policy_grid` and `best_policy`](../src/planning/search.rs) evaluate the candidates and select the best available one. A Rust question for this file: why does `reduce` return an `Option`, and what does it return for an empty grid? [`Policy::possible_actions`](../src/model/policy.rs) defines what boldness changes.

**Confirm:** [`zero_discount_does_not_create_a_spurious_preference_for_boldness`](../src/tests.rs) checks this claim in code: at discount 0 all eleven candidates score 0.22, and the reported winner is boldness 0.0. Run it after your attempt, then explain why each assertion follows:

```bash
cargo test tests::zero_discount_does_not_create_a_spurious_preference_for_boldness -- --exact
```

</details>

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `cargo run -- sweep --horizon 1`, then step 5's comparison. The default next activity is the [recall checkpoint](#session-6-write-the-small-pieces-yourself-25-minutes).

<a id="session-6-write-the-small-pieces-yourself-25-minutes"></a>
## Session 6: Recall one small piece (up to 25 minutes)

**Goal:** find out what you can reconstruct without the reference.

**You need:** your working `discounted_return` from Activity 2A. Allow up to 25 minutes. Finishing all ten exercises is not required.

**1. Predict:** with notes and solutions closed, score `[3, -2, 1]` at discount `0.5` and explain each weight.

**2. Do:** add this case at the `// YOUR CASE` comment in `exercise_1_discounted_return`, in [`src/exercises/arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), using your predicted result.

**3. Check:** run the same exact exercise test.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_1_discounted_return \
  -- --exact --ignored
```

`test result: ok. 1 passed` means your function agrees with the value you entered. Check the arithmetic independently too; matching your own incorrect expected value is not sufficient evidence.

**4. Explain:** close the source and distinguish that return from an expectation over two possible paths. If either part is unclear, revisit only Activity 2A or 2B and try a new case.

**Stop here or continue.** Record correctness, explanation, and help separately in the optional [assessment](LEARNING_RECORD.md#record-an-attempt), or save a [quick restart note](LEARNING_RECORD.md#quick-restart) naming the case you checked. The default next activity is [composition](#composition-lesson-combine-the-pieces-20-to-40-minutes). Tasks 3, 7, and 8 appear in the experience lesson. Tasks 5, 6, and 9 are an [optional control extension](EXERCISES.md#5-greedy-action), beyond this fixed-policy learner.

## Composition lesson: combine the pieces (20 to 40 minutes)

**Goal:** connect Rust ownership with the probability calculations you have already used.

**You need:** Session 1's complete-path probability; the lesson introduces the Rust it uses. Allow 20 to 40 minutes; stopping between tasks is fine.

**Predict and do:** start with the independent task in the [composition lesson](COMPOSITION_LESSON.md). A tiny concrete transformation introduces the idea first. If you get stuck, its optional ownership and probability probes help you tell which kind of difficulty it is. Continue to a navigation case and then law checks when the pipeline is clear.

**Check and explain:** compare observable event probabilities, not vector order. Explain one move and one complete-path weight. Names such as identity and associativity describe the behavior you have just compared; selected floating-point cases are not a general proof for arbitrary Rust closures.

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart) naming the composition task and its exact command from the lesson. Answers stay in the [separate composition solutions](../src/exercises/composition/solutions.rs). When return and terminal behavior are clear, the default next activity is [experience](#experience-capstone-learn-values-from-observations-25-minutes).

## Experience capstone: learn values from observations (25 minutes)

**Goal:** move from evaluating known possibilities to correcting an estimate using observed transitions.

**You need:** return, expectation, and Bellman values (Sessions 2 to 4). Allow 25 minutes; stopping after one prediction is fine.

**Predict and do:** open [the experience lesson](EXPERIENCE_LESSON.md). It puts the supplied update's data before your prediction, then gives one command at a time. Tasks 3 and 8 support the TD rule; task 7 distinguishes a mean of observed returns from an estimate using changing targets.

**Check and explain:** the sampler uses the model, but the learner receives observations. Its values use both state and remaining horizon. Compare several seeds and budgets without expecting error to decrease in every run. The policy stays fixed throughout training.

**Confirm:** [`supplied_trace_uses_bootstrap_and_per_entry_visit_counts`](../src/experience/tests.rs) checks both `JustMet` updates from the supplied trace. Each row is one update:

| `JustMet` visit | Previous value | Target | Learning rate | New value |
| --- | ---: | ---: | ---: | ---: |
| First | 0 | 3 | 1 | 3 |
| Second | 3 | 1 | 0.5 | 2 |

Run it after your attempt, then explain why each row follows:

```bash
cargo test experience::tests::supplied_trace_uses_bootstrap_and_per_entry_visit_counts -- --exact
```

**Stop here or continue.** Save a [quick restart note](LEARNING_RECORD.md#quick-restart); for this activity, its next action is `cargo run -- td-trace` and the record you were explaining. At your next available session, try one [delayed prompt](LEARNING_RECORD.md#next-available-session). Its feedback lives separately. Learner results remain pending until actual attempts are assessed.

## Keep the vocabulary attached to code

| Idea | Symbol | Rust location |
| --- | --- | --- |
| State / action | `s`, `a` | [`State`, `Action`](../src/model/state.rs) |
| Policy / world response | `π(a ∣ s)`, `P(s' ∣ s,a)` | [`Policy::possible_actions`](../src/model/policy.rs), [`DatingWorld::possible_next_states`](../src/model/world.rs) |
| Immediate reward | `r` | [`DatingWorld::reward_for`](../src/model/reward.rs), [`Transition::reward`](../src/model/transition.rs) |
| Return / expectation | `G`, `E[G]` | [`discounted_return`](../src/planning/returns.rs), [`expected_value`](../src/probability/expectation.rs) |
| Discount / remaining horizon | `γ`, `h` | `discount: f64`, `horizon: usize` |
| Exact / learned value | `V_h(s)` / estimate of `V_h(s)` | [`values.rs`](../src/values.rs), [`experience/table.rs`](../src/experience/table.rs) |

An `f64` is not automatically a valid probability. Finite nonnegative weights must sum to approximately one; numerical comparisons need a tolerance.

## Connect this lab to Hugging Face

Official unit introductions checked on **2026-10-08**:

| Course reading | Use this project for | Work still outside this lab |
| --- | --- | --- |
| [Unit 1: Introduction to Deep RL](https://huggingface.co/learn/deep-rl-course/en/unit1/introduction) | Sessions 1 to 3: agent/environment, rewards, returns, and task boundaries. | The course's LunarLander training with Stable-Baselines3. |
| [Unit 2: Introduction to Q-learning](https://huggingface.co/learn/deep-rl-course/en/unit2/introduction) | Sessions 4 to 6 and the capstone: Bellman reasoning and TD evaluation of a fixed policy. | Q-learning control, exploration while improving behavior, and the FrozenLake/Taxi exercises. |
| [Unit 3: Deep Q-learning](https://huggingface.co/learn/deep-rl-course/en/unit3/introduction) | Explain why storing a separate value for every state eventually becomes impractical. | Neural Q-value approximation and Atari training. |

This lab is a Rust companion to the concepts, not a port of those course notebooks. Exact model-based planning averages known possibilities; the capstone learner estimates values from observed transitions. Follow the official hands-on work separately when you reach it.

## Save a restart cue

Use the [three-line quick restart](LEARNING_RECORD.md#quick-restart) whenever you stop. Keep the [full assessment](LEARNING_RECORD.md#record-an-attempt) for a deliberate explanation or delayed check. A note helps you resume; it does not by itself establish learning. The [companion](DEEP_RL_COMPANION.md) and [extension labs](INTERACTIVE_LABS.md) remain optional depth, not prerequisites for finishing a small activity.
