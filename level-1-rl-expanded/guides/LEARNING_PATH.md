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

**Purpose:** separate what the policy chooses from what the world does next. No cross-module tracing is needed yet.

Here is the local model at `JustMet`. Each response probability is conditional on its row's action:

| Action | Chance of choosing it | Engaged afterward | LowInterest afterward |
| --- | ---: | ---: | ---: |
| Ask | 0.70 | 0.80 | 0.20 |
| Story | 0.30 | 0.60 | 0.40 |

These numbers come from the `JustMet` arm in [`Policy::possible_actions`](../src/model/policy.rs) and the `(JustMet, Ask)` / `(JustMet, Story)` arms in [`DatingWorld::possible_next_states`](../src/model/world.rs).

**1. Predict:** what is the probability of the complete path `JustMet -> Ask -> Engaged`?

**2. Do:** run the one-step experiment.

```bash
cargo run -- first-step
```

**3. Check:** compare your prediction with the `Ask -> Engaged` row. Check that the four displayed branch probabilities sum to one. Leave the reward summary for Activity 2B.

**4. Explain:** on paper, change the action probabilities to Ask `0.5` and Story `0.5`, keeping the response probabilities fixed. Predict the new Ask-to-Engaged path. Explain which probability changed and which did not.

<details>
<summary>Optional help and Rust detail</summary>

A complete path needs both its action and its conditional response. This does not assume the response is independent of the action. Different paths reaching the same event contribute separately to its total probability.

After your attempt, compare the [worked one-step calculation](DEEP_RL_COMPANION.md#36-a-complete-one-step-calculation). For the implementation, follow [`possible_transitions`](../src/planning/transitions.rs) into [`Distribution::and_then`](../src/probability/composition.rs). `State` and `Action` are enum alternatives; a `Vec<(T, f64)>` holds weighted branches. [Exercise 4](EXERCISES.md#4-probability-validation) is optional practice in validating those weights.

The same claim is a test, [`first_step_joint_probabilities_match_the_hand_calculation`](../src/tests.rs): it asserts the two path probabilities 0.56 and 0.12. Predict each number it asserts, then run:

```bash
cargo test tests::first_step_joint_probabilities_match_the_hand_calculation -- --exact
```

</details>

**Stop or next:** save a [quick restart note](LEARNING_RECORD.md#quick-restart). The default next activity is [return](#return-activity).

<a id="session-2-score-a-future-then-score-uncertainty-20-minutes"></a>
## Session 2: Return and expectation, one activity at a time

<a id="return-activity"></a>
### 2A. Score one reward sequence

**Purpose:** turn rewards along one path into one return. Allow roughly 10 to 20 minutes. You need a slice and a loop, not the full planner.

Use rewards `[1, 2, 4]` and discount `0.5`. The first reward counts immediately. Each step of delay adds another factor of the discount. These are consecutive rewards, not alternative outcomes.

**1. Predict:** calculate this path's return on paper. Which rewards would matter if the discount were zero?

**2. Do:** implement only `discounted_return` in [`arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs). Its empty-input contract is in the function comment. Leave the other TODOs intact.

**3. Check:** run its exact starter test.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_1_discounted_return \
  -- --exact --ignored
```

An untouched TODO is an expected starting failure. A failing assertion identifies a case to compare with your prediction. Zero selected tests is not a successful check; copy the full command. The [exercise feedback guide](EXERCISES.md#attempt-one-exercise) explains the other outcomes.

**4. Explain:** use `[2, -1, 4]` as a new case. Predict its return at discount `0` and at `1`, then check it using the marked learner-case space in your exercise-1 test. Explain why the first reward is undiscounted.

<details>
<summary>Optional help, reference, and Rust detail</summary>

Ask what weight applies before any time has passed, then how that weight changes after one step. The [exercise-1 card](EXERCISES.md#1-discounted-return) supplies local support.

After your attempt, compare `discounted_return` in the [separate solutions](../src/exercises/arithmetic/solutions.rs). To inspect the fixed reward sequence one term at a time, use:

```bash
cargo run -- returns --discount 0.5
```

The production version, [`discounted_return`](../src/planning/returns.rs), borrows a trajectory instead of a reward slice and takes its weights `1, γ, γ², ...` from `discount_weights`. Both score rewards across time. The [return reference](DEEP_RL_COMPANION.md#33-discounted-return) adds the notation.

The same claim is a test, [`the_greedy_action_can_depend_on_remaining_horizon`](../src/values/tests.rs): the same supplied history is worth 0 with one step counted but 2.5 with two. Predict each number it asserts, then run:

```bash
cargo test values::tests::the_greedy_action_can_depend_on_remaining_horizon -- --exact
```

</details>

**Stop or next:** your restart cue can name `discounted_return`, its unfinished case, and the command above. The next activity is [expectation](#expectation-activity).

<a id="expectation-activity"></a>
### 2B. Score alternative outcomes

**Purpose:** compare uncertain futures using a probability-weighted score. Allow roughly 10 to 20 minutes. Bring Activity 2A's distinction between a reward and a whole-path return.

Suppose one outcome is worth `8` with probability `0.75`, and another is worth `-2` with probability `0.25`. These are alternatives. Expectation weights each value by how likely that outcome is; it does not apply time discounting again.

**1. Predict:** calculate the expected value. Must it equal either possible outcome?

**2. Do:** implement `weighted_expectation` in [`arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs). Its pairs are `(value, probability)` in that order.

**3. Check:** run just exercise 2.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_2_weighted_expectation \
  -- --exact --ignored
```

**4. Explain:** split the first outcome into two rows of value `8`, with probabilities `0.25` and `0.50`. Keep the other row. Predict whether the expectation changes, then check this case in the marked exercise-2 test space. Explain why dividing by the number of rows would answer the wrong question.

<details>
<summary>Optional help, reference, and Rust detail</summary>

Keep time weights inside a path and probability weights across alternatives. For an empty input, the exercise's weighted sum is zero; this does not make an empty list a valid probability distribution.

Use the [exercise-2 card](EXERCISES.md#2-weighted-expectation) for help, then compare the matching [separate solution](../src/exercises/arithmetic/solutions.rs) after an attempt. [`expected_value`](../src/probability/expectation.rs) generalizes the same idea with `Fn(&T) -> f64`: the scoring function borrows each outcome. The [expectation reference](DEEP_RL_COMPANION.md#8-expected-value-collapse-uncertainty-into-one-comparison-number) connects this to policy evaluation.

</details>

**Stop or next:** leave a [quick restart note](LEARNING_RECORD.md#quick-restart), or continue to [task endings](#session-3-find-the-end-of-the-episode-20-minutes).

## Session 3: Find the end of the episode (20 minutes)

**Purpose:** distinguish the world ending from a calculation stopping. Bring the return idea from Activity 2A.

Here `Goodbye` is terminal. `MutualInterest` still has a final transition to `Goodbye`. The horizon is the number of transitions left in the objective; it can reach zero while the world state is nonterminal.

**1. Predict:** what should an evaluation with zero remaining transitions score?

**2. Do:** evaluate that boundary.

```bash
cargo run -- evaluate --horizon 0
```

**3. Check:** the output reports one represented trajectory and an expected return of zero. That trajectory contains no transitions yet. This differs from erasing a reward already received on entering a terminal state.

**4. Explain:** a robot task ends at its deadline. A different robot keeps working, but its log file stops. Explain why only the first event necessarily ends the task's future reward.

<details>
<summary>Optional comparison and Rust detail</summary>

Predict one-step behavior, then run:

```bash
cargo run -- evaluate --horizon 1
```

A longer horizon can include penalties as well as rewards. It need not improve the score. [`State::is_terminal`](../src/model/state.rs) defines the task ending; [`extend_one_step`](../src/planning/rollout.rs) preserves terminal histories. For Rust practice, identify the consumed path, borrowed model, and cloned histories needed for separate branches. The [terminal-state reference](DEEP_RL_COMPANION.md#23-terminal-and-absorbing-states) expands the distinction.

The same claim is a test, [`reaching_goodbye_stops_history_without_losing_probability`](../src/tests.rs): every history from `LowInterest` has exactly one step, ends in `Goodbye`, and still keeps its reward of 2. Predict each number it asserts, then run:

```bash
cargo test tests::reaching_goodbye_stops_history_without_losing_probability -- --exact
```

</details>

**Stop or next:** record which boundary you can explain. Continue to [Bellman values](#session-4-replace-repeated-futures-with-values-25-minutes).

## Session 4: Replace repeated futures with values (25 minutes)

**Purpose:** reuse the answer to a smaller remaining problem. Bring return, expectation, and the two stopping rules.

If two histories reach the same state with the same number of steps left, this model gives them the same future value. A Bellman evaluator calculates that continuation once and reuses it. The immediate reward still counts separately.

**1. Predict:** should it agree with enumerating every path when policy, model, discount, and horizon match?

**2. Do:** compare both exact methods through two remaining steps.

```bash
cargo run -- compare --horizon 2
```

**3. Check:** read the two value columns. The absolute difference measures numerical disagreement; `cached subproblems` is a count, not a return. Small rounding differences are expected.

**4. Explain:** why would a cache keyed only by state confuse a one-step question with a two-step question? Agreement between algorithms still does not show that this invented world is realistic.

<a id="bellman-practice"></a>
**5. Do one backup:** attempt `bellman_backup` in [`arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs). Each tuple is `(probability, immediate_reward, next_value)`. Probability already includes the policy and world. Predict the result for `[(0.5, 2.0, 10.0), (0.5, -2.0, 0.0)]` at discount `0.9` before coding.

**6. Check:** run only exercise 10.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_10_bellman_backup \
  -- --exact --ignored
```

Then change both continuation values to zero. Explain which information still determines the answer. The [exercise-10 card](EXERCISES.md#10-one-bellman-backup) has local help and the separate solution link.

<details>
<summary>Optional exact evaluator and Rust detail</summary>

Read [`values::evaluate_value`](../src/values.rs) beside [`expected_return_of`](../src/planning/evaluation.rs). Both average actions under the supplied policy; taking a maximum would answer a different question. The cached key includes state and remaining horizon, while policy and discount stay fixed for this evaluation. The [dynamic-programming reference](DEEP_RL_COMPANION.md#40-dynamic-programming) gives the recurrence.

The same claim is a test, [`bellman_matches_complete_tree_across_states_horizons_and_parameters`](../src/values/tests.rs): both evaluators agree for every state, horizon, discount, and boldness it tries. Predict each number it asserts, then run:

```bash
cargo test values::tests::bellman_matches_complete_tree_across_states_horizons_and_parameters -- --exact
```

</details>

**Stop or next:** stopping after step 4 is fine; leave exercise 10 as your exact restart action. After the backup, continue to [policy comparison](#session-5-compare-policies-without-overclaiming-20-minutes).

## Session 5: Compare policies without overclaiming (20 minutes)

**Purpose:** distinguish evaluating one behavior rule from searching among rules. This command checks eleven boldness values from `0` to `1` in steps of `0.1`, under identical world and horizon settings.

**1. Predict:** must the largest boldness have the highest expected return?

**2. Do:** run the grid comparison.

```bash
cargo run -- sweep
```

**3. Check:** locate the winning candidate and its return. This establishes the best on this grid for these settings, not a global optimum.

**4. Explain:** what evidence would distinguish trying known policies in a model from changing behavior through experience? A stochastic policy alone does not establish a learning exploration strategy, and a good mean does not guarantee a good individual outcome.

**5. Change one setting:** predict whether the winner must stay the same with horizon `1`, then check:

```bash
cargo run -- sweep --horizon 1
```

<details>
<summary>Optional source and Rust detail</summary>

[`evaluate_policy_grid` and `best_policy`](../src/planning/search.rs) evaluate candidates and select the best available one. Explain why `reduce` returns an `Option`, and what it returns for an empty grid. [`Policy::possible_actions`](../src/model/policy.rs) defines what boldness changes. Use the [grid reference](DEEP_RL_COMPANION.md#38-grid-search) and [exploration distinction](DEEP_RL_COMPANION.md#important-nuance-stochasticity-is-not-automatically-exploration) when needed.

The same claim is a test, [`zero_discount_does_not_create_a_spurious_preference_for_boldness`](../src/tests.rs): at discount 0 all eleven candidates score 0.22, and the reported winner is boldness 0.0. Predict each number it asserts, then run:

```bash
cargo test tests::zero_discount_does_not_create_a_spurious_preference_for_boldness -- --exact
```

</details>

**Stop or next:** note the settings behind your result. Continue to the [recall checkpoint](#session-6-write-the-small-pieces-yourself-25-minutes).

<a id="session-6-write-the-small-pieces-yourself-25-minutes"></a>
## Session 6: Recall one small piece (up to 25 minutes)

**Purpose:** find out what you can reconstruct after working through the examples. This is not a requirement to finish all ten exercises.

**1. Predict:** with notes and solutions closed, score `[3, -2, 1]` at discount `0.5` and explain each weight.

**2. Do:** add this case in the marked exercise-1 test space in [`arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), using your predicted result.

**3. Check:** run the same exact exercise test.

```bash
cargo test \
  exercises::arithmetic::practice::exercise_1_discounted_return \
  -- --exact --ignored
```

A passing assertion checks agreement with the expected result you entered. Independently check the arithmetic too; matching your own incorrect expected value is not sufficient evidence.

**4. Explain:** close the source and distinguish that return from an expectation over two possible paths. If either part is unclear, revisit only Activity 2A or 2B and try a new case.

**Stop or next:** record correctness, explanation, and help separately in the optional [assessment](LEARNING_RECORD.md#record-an-attempt). Continue to [composition](#composition-lesson-combine-the-pieces-20-to-40-minutes), or stop with a quick restart note. Tasks 3, 7, and 8 appear in the experience lesson. Tasks 5, 6, and 9 are an [optional control extension](EXERCISES.md#5-greedy-action), beyond this fixed-policy learner.

## Composition lesson: combine the pieces (20 to 40 minutes)

**Purpose:** connect Rust ownership with the probability calculations you have already used.

**Predict and do:** start with the independent task in the [composition lesson](COMPOSITION_LESSON.md). A tiny concrete transformation introduces the idea first. If you get stuck, its optional ownership and probability probes help you choose support. Continue to a navigation case and then law checks when the pipeline is clear.

**Check and explain:** compare observable event probabilities, not vector order. Explain one move and one complete-path weight. Names such as identity and associativity describe the behavior you have just compared; selected floating-point cases are not a general proof for arbitrary Rust closures.

**Stop or next:** answers remain in the [separate composition solutions](../src/exercises/composition/solutions.rs). When return and terminal behavior are clear, continue to [experience](#experience-capstone-learn-values-from-observations-25-minutes).

## Experience capstone: learn values from observations (25 minutes)

**Purpose:** move from evaluating known possibilities to correcting an estimate using observed transitions.

**Predict and do:** open [the experience lesson](EXPERIENCE_LESSON.md). It puts the supplied update's data before your prediction, then gives one command at a time. Tasks 3 and 8 support the TD rule; task 7 distinguishes a mean of observed returns from an estimate using changing targets.

**Check and explain:** the sampler uses the model, but the learner receives observations. Its values use both state and remaining horizon. Compare several seeds and budgets without expecting error to decrease in every run. The policy stays fixed throughout training.

The same claim is a test, [`supplied_trace_uses_bootstrap_and_per_entry_visit_counts`](../src/experience/tests.rs): for the supplied trace the target is 3, the learning rate 0.5, and the new value 2. Predict each number it asserts, then run:

```bash
cargo test experience::tests::supplied_trace_uses_bootstrap_and_per_entry_visit_counts -- --exact
```

**Stop or next:** at your next available session, try one [delayed prompt](LEARNING_RECORD.md#next-available-session). Its feedback lives separately. Learner results remain pending until actual attempts are assessed.

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
