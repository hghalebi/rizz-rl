# Your Rust + Hugging Face RL learning path

This is the project's **single learning route**. Choose one session, make a prediction, run or edit a small computation, then explain a changed case. All commands run **from the repository root**. Use the [lab README](../README.md) for setup and checks, and the [learning record](LEARNING_RECORD.md) for every assessment and restart note.

The recurring question is: **given a state and a behavior rule, how good are the possible futures?** Sessions 1 to 5 make exact evaluation visible. Session 6 retrieves the arithmetic. Composition connects probability with Rust ownership; the capstone estimates values from observations.

## Choose the support you need

Start with Session 1. If ownership or conditional probability makes it difficult to follow a branch, use the two separate [composition probes](COMPOSITION_LESSON.md#1-optional-diagnostic-assess-two-skills-separately). Choose only the worked and completion support your attempt calls for, then return here. If both probes are easy to explain, you can attempt the independent composition task when you reach that lesson.

Advance after an **unaided explanation and a correct changed case**. Confidence is useful context, not a pass mark. If either check reveals a gap, revisit that step and try another comparable case. The timings are timeboxes, not speed requirements. You can stop with an exact restart cue rather than finish an entire exercise bank.

## Session 1: Follow one decision (15 minutes)

**Objective and prerequisite:** distinguish the agent's choice from the world's response. You need to recognize an enum variant and a weighted pair `(outcome, probability)`.

**Predict:** from `JustMet`, what is the probability of taking `Ask` and reaching `Engaged`? Read those two probabilities, then run:

```bash
cargo run -- first-step
```

**Read:** [`State`, `Action`, and `Policy::possible_actions`](../src/model.rs), then [`possible_transitions`](../src/planning.rs). Follow one use of [`Distribution::and_then`](../src/probability.rs).

<details>
<summary>Conceptual hint</summary>

Separate a probability conditional on choosing an action from the probability of the complete action-and-response path. Ask which events require both steps and which combine alternative paths.

</details>

**Explain and change:** who controls each probability? Why can different branches reach `Engaged`? Check total probability, then change one action probability on paper while preserving the total and predict the new result.

**Feedback and next:** after your attempt, compare [the worked one-step calculation](DEEP_RL_COMPANION.md#36-a-complete-one-step-calculation). Continue to Session 2 and record your explanation in the [learning record](LEARNING_RECORD.md).

## Session 2: Score a future, then score uncertainty (20 minutes)

**Objective and prerequisite:** distinguish reward, return, and expectation. Bring Session 1's distinction between one path and alternative paths.

**Predict:** for `[1, 2, 4]`, which rewards matter at discount `0`, and what changes at `1`?

```bash
cargo run -- returns --discount 0
cargo run -- returns --discount 0.5
cargo run -- returns --discount 1
```

**Read:** [`discounted_return`](../src/planning.rs) and [`expected_value`](../src/probability.rs). Identify the loop's evolving weight and the borrowed input to `Fn(&T) -> f64`.

<details>
<summary>Conceptual hint</summary>

Time weights act inside one path. Probability weights compare alternative paths. Identify which kind of weight each loop uses.

</details>

**Explain and change:** why is the first reward undiscounted? Can an expectation differ from every individual outcome? Predict a new reward sequence containing a negative reward and check it by hand.

**Feedback and next:** use the [return reference](DEEP_RL_COMPANION.md#33-discounted-return) or [expectation reference](DEEP_RL_COMPANION.md#8-expected-value-collapse-uncertainty-into-one-comparison-number) after predicting. Continue to Session 3.

## Session 3: Find the end of the episode (20 minutes)

**Objective and prerequisite:** distinguish a terminal state from a horizon cutoff. You need Session 2's definition of return.

**Predict:** what should zero remaining steps score? Does reaching the horizon mean the world reached `Goodbye`?

```bash
cargo run -- evaluate --horizon 0
cargo run -- evaluate --horizon 1
cargo run -- evaluate --horizon 5
```

**Read:** [`State::is_terminal`](../src/model.rs), then [`extend_one_step` and `enumerate_trajectories`](../src/planning.rs). Locate borrowed inputs, the consumed trajectory, and cloned branch histories.

<details>
<summary>Conceptual hint</summary>

Ask separately whether the world can continue and whether this objective assigns value beyond the remaining horizon. `MutualInterest` and `Goodbye` have different roles here.

</details>

**Explain and change:** why must terminal paths stop collecting rewards? Explain a robot task that ends at a deadline and a continuing task whose observation file merely stops. Would a longer horizon always improve a return containing penalties?

**Feedback and next:** consult [terminal and absorbing states](DEEP_RL_COMPANION.md#23-terminal-and-absorbing-states). Continue to Session 4.

## Session 4: Replace repeated futures with values (25 minutes)

**Objective and prerequisite:** interpret Bellman evaluation as reusing a smaller remaining problem. Bring returns and boundary behavior from Sessions 2 and 3.

**Predict:** should two exact evaluators agree when their policy, model, discount, and remaining horizon match?

```bash
cargo run -- compare --horizon 5
cargo run -- compare --horizon 5 --discount 0
```

**Read:** [`values::evaluate_value`](../src/values.rs) beside [`expected_return_of`](../src/planning.rs). Start at zero remaining steps, then inspect the next row.

<details>
<summary>Conceptual hint</summary>

Two histories can have the same remaining problem. Identify all the information needed to describe that problem, including the steps left.

</details>

**Explain and change:** why does a value depend on both state and horizon? Distinguish averaging actions under a policy from choosing a maximizing action. Change the discount and explain why evaluator agreement does not establish that the invented model is realistic.

**Feedback and next:** use the [dynamic programming reference](DEEP_RL_COMPANION.md#40-dynamic-programming). Continue to Session 5.

## Session 5: Compare policies without overclaiming (20 minutes)

**Objective and prerequisite:** distinguish evaluating a policy from searching among policies. You need Session 4's fixed-policy value.

**Predict:** must the most aggressive boldness value have the highest expected return?

```bash
cargo run -- sweep
cargo run -- evaluate --boldness 0.3 --horizon 5 --discount 0.95
cargo run -- evaluate --boldness 1 --horizon 5 --discount 0.95
```

**Read:** [`evaluate_policy_grid` and `best_policy`](../src/planning.rs), then [`Policy::possible_actions`](../src/model.rs). Find the `Option` representing an empty candidate set.

<details>
<summary>Conceptual hint</summary>

A search can establish a winner only among the candidates it evaluates. Separately ask whether the policy changes in response to observations.

</details>

**Explain and change:** what does the eleven-candidate result establish? Why is policy randomness alone insufficient to describe a learning exploration strategy? Try another horizon and justify comparing candidates under identical settings.

**Feedback and next:** after predicting, consult [grid search](DEEP_RL_COMPANION.md#38-grid-search) and [stochasticity versus exploration](DEEP_RL_COMPANION.md#important-nuance-stochasticity-is-not-automatically-exploration). Continue to Session 6.

## Session 6: Write the small pieces yourself (25 minutes)

**Objective and prerequisite:** retrieve return, expectation, and terminal-aware targets without copying. Bring the first five sessions and basic slices, loops, and conditionals.

**Predict and edit:** open [`arithmetic/practice.rs`](../src/exercises/arithmetic/practice.rs), read `discounted_return`'s contract, and predict a boundary case. Attempt just that function:

```bash
cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
```

**Support:** the [arithmetic catalog](EXERCISES.md) supplies exact selectors, contracts, conceptual hints, and changed-case prompts. Begin with tasks 1 to 3. Continue with the remaining tasks across later sessions; ten tasks are not a 25-minute requirement.

**Explain and change:** justify your result and one boundary case, then solve a new case without notes. For a terminal-aware target, distinguish actual task termination from an external observation cutoff.

**Separate solution and next:** after an attempt, compare the matching function in [`arithmetic/solutions.rs`](../src/exercises/arithmetic/solutions.rs), then close it and reconstruct your reasoning. Untouched starter tests intentionally fail when selected. Passing reference tests does not complete your attempt. Continue to composition or schedule a retrieval check.

## Composition lesson: combine the pieces (20 to 40 minutes)

**Objective and prerequisite:** compose owned values and conditional outcomes, then explain concrete map and bind laws. Bring Session 1's probability reasoning; use the separate Rust and probability probes to choose support.

**Predict, run, and edit:** follow the [composition lesson](COMPOSITION_LESSON.md). It contains the exact diagnostic and task commands, a type legend, targeted worked examples, and seven separate tasks in [`composition/practice.rs`](../src/exercises/composition/practice.rs).

**Explain and change:** explain one move and one complete-path probability, then attempt the navigation transfer case. Compare regrouped computations by event probabilities, rather than vector order.

**Separate solution and next:** use [`composition/solutions.rs`](../src/exercises/composition/solutions.rs) after an attempt. When return and terminal behavior are clear, continue to the experience capstone. Numerical law checks are examples, not proofs for arbitrary Rust closures.

## Experience capstone: learn values from observations (25 minutes)

**Objective and prerequisite:** connect a TD target to repeated value updates. Bring Sessions 2 to 4; arithmetic tasks 3, 7, and 8 offer optional preparation.

**Predict and run:** follow the [experience lesson](EXPERIENCE_LESSON.md), first predicting one supplied update:

```bash
cargo run -- td-trace
cargo run -- learn-value --episodes 10000 --seed 7
```

**Read and explain:** [`experience.rs`](../src/experience.rs) separates the model-driven sampler from the observation-driven learner. Explain why the value table uses both state and steps remaining and why terminal or zero-horizon continuations do not bootstrap.

**Changed case and next:** compare several seeds and episode budgets using the lesson. Explain why the error can rise in a particular run even when the budget grows. This learner evaluates a fixed policy; it does not optimize boldness or implement Q-learning control. Record your attempt, then revisit a changed-context prompt later.

## Keep the vocabulary attached to code

| RL idea | Symbol | Rust location |
| --- | --- | --- |
| State / action | `s`, `a` | [`State`, `Action`](../src/model.rs) |
| Policy / world response | `π(a ∣ s)`, `P(s' ∣ s,a)` | [`Policy::possible_actions`, `DatingWorld::possible_next_states`](../src/model.rs) |
| Immediate reward | `r` | [`DatingWorld::reward_for`, `Transition::reward`](../src/model.rs) |
| Return / expectation | `G`, `E[G]` | [`discounted_return`](../src/planning.rs), [`expected_value`](../src/probability.rs) |
| Discount / remaining horizon | `γ`, `h` | `discount: f64`, `horizon: usize` |
| Exact / learned value | `V_h(s)` / estimate of `V_h(s)` | [`values.rs`](../src/values.rs), [`experience.rs`](../src/experience.rs) |

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

Use the [learning record](LEARNING_RECORD.md), the single assessment template for this project. Record your prediction, correctness, explanation, assistance, confidence, and exact next action. At the next available session, attempt a changed-context prompt without notes; repeat after a longer delay and record the actual interval. All learner outcomes remain pending until you attempt and assess them.
