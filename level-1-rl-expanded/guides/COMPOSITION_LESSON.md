# Compose probabilities while learning Rust ownership

**Purpose:** carry an uncertain choice through a response model without losing
which values you own or how likely each complete path is.

Start with something small. Turning `"tea"` into its byte length gives `3`.
Doing this to a distribution changes each value and keeps its branch weight:

```text
("tea", 0.4)    -> (3, 0.4)
("coffee", 0.6) -> (6, 0.6)
```

This operation is `map`. Continuing one value into several possible outcomes
uses `and_then`: each complete path gets its starting weight multiplied by its
conditional weight. Paths that end at the same event contribute to the same
event probability. We will connect these concrete operations to the law names
later.

**Do now:** attempt [one response pipeline](#independent-pipeline). If a Rust
move or a probability step gets in the way, use the matching support below.
A short attempt is enough to find the next useful step. Stopping is allowed.

All commands run **from the repository root**. Edit
[`practice.rs`](../src/exercises/composition/practice.rs); its seven TODO tasks
are independent. Keep [solutions.rs](../src/exercises/composition/solutions.rs)
closed during an unaided attempt. The [learning path](LEARNING_PATH.md#composition-lesson-combine-the-pieces-20-to-40-minutes)
owns progression through the wider course.

<a id="4-independent-task-construct-the-pipeline"></a>
<a id="independent-pipeline"></a>
## Do now: construct one response pipeline

**Question:** if choices produce uncertain responses, how likely is each
response byte length? This is a transformation of outcomes, not an RL reward.

The case uses `P(ask) = 0.7`, `P(story) = 0.3`,
`P(engaged | ask) = 0.8`, and `P(engaged | story) = 0.6`.
The other response is `"quiet"`. Both labels are ASCII.

<a id="read-the-types-before-composing-them"></a>
The supplied `respond` function has type
`StringBranch = fn(String) -> Distribution<String>`: it consumes one owned
choice label and returns its weighted responses. The alias lives in
[`composition/mod.rs`](../src/exercises/composition/mod.rs). These fixtures use
named function pointers without captured local state.

1. **Predict:** sketch the types between choices, responses, and byte lengths.
   Write the probability of one complete path, then predict one final event.
2. **Attempt:** implement only `independent_pipeline` in
   [`practice.rs`](../src/exercises/composition/practice.rs), using the planner's
   `map` and `and_then`. Use the supplied response model; do not hard-code the
   fixture's final probabilities.
3. **Check:** run this one task.

   ```bash
   cargo test exercises::composition::practice::independent -- --exact --ignored
   ```

4. **Explain:** with references closed, identify one ownership transfer and one
   probability multiplication in your implementation.
5. **Change the case:** add a new prediction and assertion under `YOUR CASE` in
   the existing `independent` test wrapper in the same file. Keep its shared-case
   call. Run the same command again.

For your case, define a small choice distribution and a named response function
inside the wrapper. Use finite weights in `[0, 1]` that sum to one, for both the
choice distribution and each conditional response. Use
`cases::assert_same_events(&actual, &expected)` to
compare your predicted event probabilities. Shared
[`cases.rs`](../src/exercises/composition/cases.rs) contains acceptance checks
and expected outputs; leave it unchanged. Reading it counts as assistance for
an otherwise unaided attempt.

<details>
<summary>Hint: decide which step changes the number of branches</summary>

One choice can lead to several responses. One response label has one byte
length. Match each of those descriptions to an operation before writing the
pipeline. If the difficult part is owning the labels, use the
[ownership support](#ownership-support); if it is weighting paths, use the
[probability support](#probability-support).

</details>

**Stop here:** save a [restart note](LEARNING_RECORD.md). For example:
`Next: practice.rs, independent_pipeline; rerun the independent command above.`
After an explanation and a correct changed case, the default next activity is
[the navigation check](#navigation-check). No other TODO must be finished first.

<a id="5-changed-context-a-navigation-transfer-check"></a>
<a id="navigation-check"></a>
## Next: change the context to navigation

**Question:** how likely is arrival at `"goal"` when routes have uncertain
outcomes? North has probability `0.6` and south `0.4`. North reaches goal with
conditional probability `0.75`; south reaches it with probability `0.25`.
Their other destination is `"delay"`.

1. **Predict:** close the earlier implementation and calculate the total chance
   of goal before coding.
2. **Attempt:** implement `navigation_pipeline` in
   [`practice.rs`](../src/exercises/composition/practice.rs). The supplied travel
   function consumes a route and returns weighted destination labels. Return a
   distribution of whether each destination is exactly `"goal"`.
3. **Check:** run the navigation task.

   ```bash
   cargo test \
     exercises::composition::practice::transfer_navigation \
     -- --exact --ignored
   ```

4. **Explain:** describe what changed in the context and what stayed the same
   in the type flow and probability calculation.

**Stop here:** record any help used. Copying the earlier pipeline is useful
practice, but does not establish unaided transfer. On a later visit, try a new
case in the `transfer_navigation` wrapper and record the actual delay. When
ready to continue, try [the first law experiment](#law-experiments).

<a id="1-optional-diagnostic-assess-two-skills-separately"></a>
## Optional diagnostic: find the support you need

Use one probe if you cannot yet identify what is difficult. About five minutes
is a planning aid, not a speed requirement. These checks concern this lesson,
not readiness for the whole RL capstone.

**Rust probe:** inspect
[`ownership_move.rs`](../tests/fixtures/ownership_move.rs). Identify the
expression that transfers ownership and explain who owns the `String`
afterward. Predict how a borrowed parameter would differ. Then check:

```bash
cargo test --test ownership_compile_fail rejects_reuse_after_move -- --exact --nocapture
```

The test passes only when `rustc` rejects the deliberate mistake with `E0382`.
The fixture is outside normal Cargo targets. This validates the demonstration;
your explanation is a separate check. If needed, continue to
[ownership support](#ownership-support).

**Probability probe:** use the ask/story weights in the independent task.
Predict **ask and engaged**, then **engaged through either choice**. Explain
why those questions differ before checking:

```bash
cargo test exercises::composition::worked::probability_probe -- --exact --nocapture
```

If needed, continue to [probability support](#probability-support). If both
skills need support, take ownership first and probability afterward. If both
explanations are sound, return to [the independent task](#independent-pipeline).

<a id="2-worked-examples-inspect-only-the-support-you-need"></a>
<a id="3-completion-tasks-fill-one-meaningful-gap"></a>
<a id="ownership-support"></a>
## Optional Rust support: inspect, then consume

**Question:** can you inspect a distribution before consuming it with `map`?
The analogous worked example uses `"tea"` and `"coffee"`.

1. **Predict:** write their byte lengths and the number of stored branches.
2. **Observe:** run the ownership example, then inspect `ownership_demo` in
   [`worked.rs`](../src/exercises/composition/worked.rs).

   ```bash
   cargo test exercises::composition::worked::ownership_demo -- --exact --nocapture
   ```

   Find the borrow, the move, and the new owned result. Explain why inspecting
   the original distribution after `map` would fail. `String::len` measures
   bytes, not displayed characters.
3. **Complete:** close the example and fill the closure in `complete_ownership`
   in [`practice.rs`](../src/exercises/composition/practice.rs). Preserve the
   branch count and probabilities while returning each label's byte length.
4. **Check:** run completion A.

   ```bash
   cargo test \
     exercises::composition::practice::ownership_completion \
     -- --exact --ignored
   ```

<details>
<summary>Hint: inspect an owned label without duplicating its text</summary>

The closure receives one owned `String`. Consider which of its methods can
inspect the information requested by the output type.

</details>

**Stop here:** save a restart note, or return to
[the independent task](#independent-pipeline). If probability is also unclear,
the next useful step is [probability support](#probability-support).

<a id="probability-support"></a>
## Optional probability support: weight a complete path

**Question:** why is a response's conditional probability not yet the weight
of the whole path that reaches it?

1. **Predict:** using the ask/story inputs, draw the four complete branches.
   Keep one weight beside each path and distinguish paths from response events.
2. **Observe:** run the branching example, then inspect `branching_demo` in
   [`worked.rs`](../src/exercises/composition/worked.rs).

   ```bash
   cargo test exercises::composition::worked::branching_demo -- --exact --nocapture
   ```

   Explain why four stored branches can describe two distinct response events.
3. **Complete:** close the example and fill only the continuation in
   `complete_branching` in [`practice.rs`](../src/exercises/composition/practice.rs).
   Its `StringBranch` parameter already maps one owned choice to its conditional
   response distribution.
4. **Check:** run completion B.

   ```bash
   cargo test \
     exercises::composition::practice::branching_completion \
     -- --exact --ignored
   ```

<details>
<summary>Hint: compose uncertainty instead of nesting it</summary>

Which operation weights complete paths through a second distribution? A plain
value transformation can instead leave a distribution inside each outcome.

</details>

**Stop here:** return to [the independent task](#independent-pipeline), or save
that task and its command as your next action.

<details>
<summary>Optional combined worked example, after an attempt</summary>

Inspect the purpose labels in `combined_demo`: transform a choice, branch
through a response model, transform its result.

```bash
cargo test exercises::composition::worked::combined_demo -- --exact --nocapture
```

Byte lengths demonstrate a Rust transformation; they are not rewards and this
example does not learn a policy. Close the example before a new unaided attempt.

</details>

<a id="6-laws-predict-an-equivalence-then-construct-both-sides"></a>
<a id="law-experiments"></a>
## Laws: predict an equivalence, then construct both sides

Take one experiment per sitting if that helps. These tasks use the same finite
distributions as the planner, with small integers, duplicate outcomes, and a
zero-probability branch. Each result field is a separate experiment, so a
consuming operation needs its own input value.

### First experiment: transform values in one step or two

For a concrete starting value `2`, compare adding one and then doubling with
one transformation that adds one and doubles. Predict whether they agree.
Then ask whether doing nothing preserves the starting distribution.

`IntegerMap = fn(i32) -> i32` names one such transformation. `MapResults`
holds the fields `identity`, `sequential`, and `composed`.

1. Implement `construct_map_laws` in [`practice.rs`](../src/exercises/composition/practice.rs).
2. Run the map experiment.

   ```bash
   cargo test exercises::composition::practice::map_laws -- --exact --ignored
   ```

3. Add one predicted input under `YOUR CASE` in the `map_laws` wrapper and
   explain why the event probabilities agree.

After observing them, name these **functor identity and composition**. A map
changes values while preserving each stored branch's weight. Different
branches can end at the same event; their stored rows remain separate.

**Stop here.** Next, try the certainty experiment when you are ready.

### Second experiment: insert a certain step

Imagine starting certainly at `2` and then branching, or taking an existing
uncertain outcome and continuing certainly with that same value. Predict
what either inserted certainty changes.

`IntegerBranch = fn(i32) -> Distribution<i32>` names a weighted continuation.
`BindIdentityResults` has `left_identity` and `right_identity` fields.

1. Implement `construct_bind_identities` in [`practice.rs`](../src/exercises/composition/practice.rs).
2. Run the certainty experiment.

   ```bash
   cargo test exercises::composition::practice::bind_identities -- --exact --ignored
   ```

3. Add one predicted input in the `bind_identities` wrapper and explain the
   probabilities in both results.

Name these **left and right identity for bind**. Here `certain` is the unit
operation and `and_then` is bind.

**Stop here.** Next, try regrouping those continuations.

### Third experiment: regroup without reordering

Compare two consecutive branching steps with one continuation that performs
those same steps internally. The order stays fixed; only the grouping changes.
`AssociationResults` has `sequential` and `regrouped` fields.

1. Predict whether regrouping changes any complete path's weight.
2. Implement `construct_associativity` in [`practice.rs`](../src/exercises/composition/practice.rs).
3. Run the regrouping experiment.

   ```bash
   cargo test exercises::composition::practice::associativity -- --exact --ignored
   ```

4. Add one predicted input in the `associativity` wrapper and explain the
   resulting event probabilities.

Name this equivalence **associativity**. Composing functions of the shape
`A -> Distribution<B>` this way is called **Kleisli composition**.

The comparison helper sums duplicate events and checks every event occurring
on either side within absolute tolerance `1e-9`. It rejects invalid weights.
Vectors can differ in order, duplicates, or zero branches while representing
the same distribution. These selected tests illustrate laws; they do not prove
them for every input. The examples use pure, deterministic functions. Rust's
`Fn` bound does not guarantee purity, and floating-point arithmetic is not exact
real arithmetic. Do not extend this evidence to arbitrary side-effecting closures.

**Stop here:** save a [restart note](LEARNING_RECORD.md). The next course
activity is [learning values from observations](LEARNING_PATH.md#experience-capstone-learn-values-from-observations-25-minutes).

## After your attempt: separate solutions and next step

An unfinished selected task stops at its TODO. Implement that function; the
other TODOs can remain. A compiler error anywhere in the same target can block
this task, so start with the first diagnostic. An assertion names the failed
case and event. Zero selected tests means the attempt was not checked: recopy
the full selector. A pass is a reason to explain a changed case, not proof of
understanding by itself.

After an attempt or an explicit choice to use help, open only the matching
function in [the separate solutions](../src/exercises/composition/solutions.rs).
For example, this checks the independent reference answer:

```bash
cargo test exercises::composition::solutions::independent -- --exact
```

Replace `independent` with the exact task name for another solution. Reference
tests do not execute your attempt. For maintenance, this wider check runs all
reference, worked, and comparison tests, skipping the seven learner tasks:

```bash
cargo test exercises::composition::
```

Use the [learning record](LEARNING_RECORD.md) for a quick restart cue or an
optional detailed assessment. Keep correctness, explanation, help used, and
actual delay separate. If the same misconception remains after support, revise
the support and try a smaller case. Whether this lesson helps you is still
unverified until you actually attempt it.
