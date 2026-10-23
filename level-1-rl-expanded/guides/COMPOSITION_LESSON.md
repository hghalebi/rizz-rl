# Compose probabilities while learning Rust ownership

The goal is to explain and implement one small computation: transform an owned
value, continue it into possible outcomes, and preserve the weights of complete
paths. Use the actual [`Distribution`](../src/probability/distribution.rs) from the planner; its [`map` and `and_then`](../src/probability/composition.rs) are what you practice here.

Allow about five minutes for the optional diagnostic and fifteen minutes for
the route it suggests. These are timeboxes, not speed requirements. Passing
the material's reference tests does not establish that you have learned it.
Record your own prediction and explanation before looking at results.

All commands below run **from the repository root**. Learner code is in
[`src/exercises/composition/practice.rs`](../src/exercises/composition/practice.rs). It contains
seven isolated TODO tasks. The separate
[`solutions.rs`](../src/exercises/composition/solutions.rs) is explicitly a spoiler file.
[`worked.rs`](../src/exercises/composition/worked.rs) contains the optional guided examples.

The [learning path](LEARNING_PATH.md#composition-lesson-combine-the-pieces-20-to-40-minutes) places this lesson in the overall route. This guide owns its diagnostic, targeted support, and exact task commands.

## Read the types before composing them

The aliases and result types live in [`composition/mod.rs`](../src/exercises/composition/mod.rs). They name ordinary Rust function pointers and records; they are not new category-theory machinery.

| Type | Read it as |
| --- | --- |
| `StringBranch = fn(String) -> Distribution<String>` | Consume one owned label and return its weighted possible successors. |
| `IntegerBranch = fn(i32) -> Distribution<i32>` | Take one integer and return weighted successor integers. |
| `IntegerMap = fn(i32) -> i32` | Transform one integer into one integer. |
| `MapResults` | Keep identity, sequential, and composed experiments in separate fields. |
| `BindIdentityResults` | Keep the left-identity and right-identity experiments separately. |
| `AssociationResults` | Keep sequential and regrouped experiments separately. |

A `fn(...) -> ...` value is a function pointer; these fixtures pass named functions without captured local state. A `Distribution<T>` stores possible values of `T` with weights. Inspect each task's input and output types before choosing an operation. Consuming a distribution means that a separate experiment needs its own input value.

## 1. Optional diagnostic: assess two skills separately

**Ownership probe:** inspect
[`ownership_move.rs`](../tests/fixtures/ownership_move.rs). It deliberately
does not compile. Before running, identify the expression that transfers
ownership, explain who owns the `String` afterward, and explain how a borrowed
parameter would differ. Distinguish a syntax slip from an ownership mistake.

```bash
cargo test --test ownership_compile_fail rejects_reuse_after_move -- --exact --nocapture
```

The test succeeds only when `rustc` rejects the fixture with `E0382`. The broken
snippet is outside normal Cargo targets; the reference project remains buildable.
This verifies the demonstration, not your explanation.

**Probability probe:** `P(ask) = 0.7`, `P(story) = 0.3`,
`P(engaged | ask) = 0.8`, and `P(engaged | story) = 0.6`.
Predict the probability of **ask and engaged**, then the probability of
**engaged through either choice**. Explain why the two questions differ.

```bash
cargo test exercises::composition::worked::probability_probe -- --exact --nocapture
```

Record correctness and unaided explanation separately for each probe.

| Evidence from your attempt | Smallest useful route |
| --- | --- |
| Both predictions and explanations are sound | Start at the independent task in section 4. |
| Ownership needs support | Ownership example and completion A, then section 4. |
| Probability composition needs support | Branching example and completion B, then section 4. |
| Both need support | Both examples and completions, then section 4. |
| Diagnostic skipped | Choose a challenge or a worked example, then adjust from the attempt. |

This tentative route concerns this composition lesson, not readiness for the
whole RL capstone. If the combined task still obscures the difficulty, briefly
practice the weak skill separately before recombining it.

## 2. Worked examples: inspect only the support you need

**Ownership objective:** distinguish borrowing a distribution to inspect it
from consuming it with `map`. Run after predicting the output lengths:

```bash
cargo test exercises::composition::worked::ownership_demo -- --exact --nocapture
```

In `ownership_demo`, point to the borrow, the move, and the new owned result.
Explain why computing the original branch count after `map` would be a problem.
The strings here are ASCII; `String::len` measures bytes, not displayed characters.

**Probability objective:** distinguish a conditional response distribution
from the weights of complete paths. Predict before running:

```bash
cargo test exercises::composition::worked::branching_demo -- --exact --nocapture
```

Explain why four stored branches can represent only two distinct response
events. The count of stored branches is not the count of unique outcomes.

**Optional combined example:** inspect the purpose labels in `combined_demo`:
transform a choice, branch through a response model, transform its result.

```bash
cargo test exercises::composition::worked::combined_demo -- --exact --nocapture
```

Response byte lengths illustrate a Rust transformation. They are not rewards
and do not turn this example into a learning algorithm.

## 3. Completion tasks: fill one meaningful gap

**A: ownership.** In `complete_ownership`, complete the closure that returns
each label's byte length. Preserve the initial branch count and probabilities.

```bash
cargo test exercises::composition::practice::ownership_completion -- --exact --ignored
```

**B: branching.** In `complete_branching`, complete the continuation using the
supplied branch function. It receives one choice and returns its possible responses.

```bash
cargo test exercises::composition::practice::branching_completion -- --exact --ignored
```

Run only the task you are attempting. Untouched TODOs remain compilable and
fail only when selected. Filtering isolates test execution, not compilation:
a syntax or type error anywhere in the same target can block the selected task.

<details>
<summary>Optional conceptual hints for the completion tasks</summary>

- Ownership: the closure receives one owned `String`; ask what information its
  methods can inspect without duplicating the text.
- Branching: the supplied function already describes uncertainty conditional
  on one choice. Identify which operation composes distributions rather than
  merely putting a distribution inside each outcome.

</details>

## 4. Independent task: construct the pipeline

Without the worked code or solutions, implement `independent_pipeline`.
It receives weighted choices and a response function. Return the distribution
of response label byte lengths. Reuse the planner's `map` and `and_then`;
do not hard-code the supplied fixture's final probabilities.

Before coding, sketch the input and output type of each step. Then run:

```bash
cargo test exercises::composition::practice::independent -- --exact --ignored
```

Explain both a move and a probability multiplication without opening the
reference. Add a new choice distribution to the shared
[`cases.rs`](../src/exercises/composition/cases.rs) and predict its result. Those cases
contain expected numbers, so opening them counts as assistance when checking
an otherwise unaided attempt.

## 5. Changed context: a navigation transfer check

Close earlier implementations. In `navigation_pipeline`, weighted routes lead
through the supplied travel function to destination labels. Return the
distribution of whether the destination is exactly `"goal"`.

For the initial case, north has probability `0.6` and south `0.4`. North reaches
goal with conditional probability `0.75`; south reaches it with probability
`0.25`. Their other outcome is `"delay"`. Predict the total chance of goal.

```bash
cargo test exercises::composition::practice::transfer_navigation -- --exact --ignored
```

Explain what changed in the context and what stayed the same in the types and
probability structure. Passing by copying the earlier pipeline is not evidence
of unaided transfer. At the next available session, try another comparable
case without notes and record the actual delay and any help used. Use the
project's [learning record](LEARNING_RECORD.md) to distinguish correctness from explanation.

## 6. Laws: predict an equivalence, then construct both sides

These three tasks use small integers to keep arithmetic secondary. The
fixtures include normalized finite weights, duplicate outcomes, and a
zero-probability branch. Read the `MapResults`, `BindIdentityResults`, and
`AssociationResults` field names in [`composition/mod.rs`](../src/exercises/composition/mod.rs).
Each field holds a separate experiment using the existing distribution type.

**Map task:** `construct_map_laws` returns an identity transformation, two
successive transformations, and the same transformations composed in one step.
Predict which results agree before running:

```bash
cargo test exercises::composition::practice::map_laws -- --exact --ignored
```

After observing the equivalences, name them: **functor identity and composition**.
A map changes outcomes while preserving how their branches are weighted.

**Certainty task:** `construct_bind_identities` inserts a certain beginning
before a branch function, and a certain continuation after an input distribution.
Predict what should remain unchanged:

```bash
cargo test exercises::composition::practice::bind_identities -- --exact --ignored
```

Afterward, name these **left and right identity for bind**. Here `certain` is
the unit operation and `and_then` is bind.

**Regrouping task:** `construct_associativity` returns two successive branching
steps and a single continuation that groups those same steps internally.
Keep their order unchanged; only the grouping changes.

```bash
cargo test exercises::composition::practice::associativity -- --exact --ignored
```

Afterward, name the equivalence **associativity**. Composing functions of the
shape `A -> Distribution<B>` this way is called **Kleisli composition**. No
generic category-theory framework is needed to inspect the two computations.

For each law, add one small new input and explain its outcome probabilities.
The comparison helper aggregates each event across duplicate branches and
checks every event occurring on either side with absolute tolerance `1e-9`.
It also rejects invalid probabilities. Two vectors can differ in ordering,
duplicates, or zero branches while representing the same distribution.

These selected numerical tests illustrate laws; they are not a proof for all
inputs. The examples use pure, deterministic functions. Rust's `Fn` bound does
not guarantee purity, and floating-point arithmetic is not exact real arithmetic.
Do not infer laws for arbitrary closures with side effects from these examples.

## After your attempt: separate solutions and next step

After an attempt, open only the matching function in the [separate solutions](../src/exercises/composition/solutions.rs).
For example, verify the independent reference with:

```bash
cargo test exercises::composition::solutions::independent -- --exact
```

Replace `independent` with the exact task name when checking another solution.
Run all reference, worked, and comparison checks with:

```bash
cargo test exercises::composition::
```

Normal checks intentionally skip the seven learner tasks. Use the
[learning record](LEARNING_RECORD.md) for your prediction, observed result,
correctness, unaided explanation, assistance, and actual delay before revisiting.
Keep confidence as a signal rather than the progression gate.

Advance after an unaided explanation and a successful changed case. If the
same critical misconception remains after targeted support and a comparable
new case, revise that support; do not infer a fixed limitation in the learner.
Whether this lesson helps you remains unverified until you actually attempt it.
Return to the [learning path](LEARNING_PATH.md#experience-capstone-learn-values-from-observations-25-minutes)
for the next lesson, or use the learning record for a delayed changed-context check.
