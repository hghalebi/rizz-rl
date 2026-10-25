# Agent guide

**This is a teaching project.** People read this code to learn Rust and reinforcement learning.

When clarity for a learner conflicts with brevity or production polish, choose clarity.

## Before you edit

1. Find the guide task that points at the code. Most are the code links inside each session's optional-detail block in the [learning path](level-1-rl-expanded/guides/LEARNING_PATH.md).
2. Keep whatever that task asks the learner to find.
   - Session 2 asks for the weight sequence `1, γ, γ², ...` from `discount_weights`, and which kind of weight each function uses.
   - Session 5 asks why `reduce` returns an `Option` in `best_policy`.
3. Keep teaching comments. When code changes, update the card that explains it.
4. Write every new comment as a card (see below), never as a paragraph.

## Code shape

- A function that computes a value (a sum, a map, a selection, a repeated expansion) is an iterator pipeline that reads like its formula: `map`, `flat_map`, `filter`, `zip`, `fold`, `reduce`, `sum`, `collect`.
- A loop stays only where it performs effects in order: writing output rows, drawing random numbers, breaking early at a terminal state. Even then it iterates over a pipeline.
- Destructure tuples with full names, `(outcome, probability)`, never `.0` or `.1`; destructure newtypes, `let Discount(discount) = ..`.
- Small `Copy` types (`State`, `Action`, `Policy`, `DatingWorld`) are passed by value. Borrow slices, tables, and anything that is not `Copy`.
- Inside a private module, items are `pub`: the private `mod` line already keeps them inside the crate.
- Name an intermediate instead of writing `a + b * c` in one expression, for example `let discounted_future = discount * future_value;`. It reads like the formula, and Clippy would otherwise suggest `mul_add`, which changes rounding.
- Empty `f64` sums: `sum()` of an empty iterator is `-0.0`, which prints as `-0.000000`. Use `fold(0.0, ..)` where the input can be empty, as `discounted_return` does.
- Exception, on purpose: `src/exercises/` is where learners write loops by hand. Leave its loops alone.
- A pipeline a learner meets for the first time gets a stage-by-stage bridge from the loop it replaces, mapped to printed output. [Activity 2A](level-1-rl-expanded/guides/LEARNING_PATH.md#loop-to-pipeline) is the model.

## Comment card

Above every function, method, struct, and enum, as `///`:

```rust
/// <concept: what it computes, one line>
/// <formula or diagram, one line in backticks; longer diagrams go in a ```text fence>
/// <invariant: what stays true, one line>
/// Predict: <one prompt, only where the method had one>
```

- Domain meaning lives in `///` cards and `//!` file headers. Rust mechanics (ownership, `move`, `*` through a borrow, `FnMut`) get one inline `//` line, only where a learner would stall.
- Arrow style: `T @ p ──f──► U @ p`. The `// CHECKPOINT N` banners stay as anchors.
- Long explanations belong in `guides/DEEP_RL_COMPANION.md`, not in the code.

## Keep the teaching shape

- Every lesson file opens with three `//!` lines: `Question:`, `Example:`, `Predict:`. Keep the example small enough to work on paper.
- The eleven checkpoints are listed in the [lab README](level-1-rl-expanded/README.md) under "Read the code in checkpoint order". A new checkpoint goes there too, with its file and session.
- Each learning-path session's optional-detail block ends with one **Confirm** test whose name states the session's claim, with the command that runs it alone. It is a check after an attempt, never a prediction prompt. `check.py` verifies that every such exact selector matches one test, so renaming a test breaks CI until the guide is updated.
- Never state a number right before asking the learner to predict it. A prediction uses a changed case; worked results come afterward, labelled **Confirm**. A large regression test is never a prediction task: give one hand-sized case first, as Session 4 does.

## Keep files small

- One concept per file. Lesson files stay short: aim for about 120 lines or fewer.
- When a module has a folder, its module file is only a table of contents: reading order, `mod` lines, and `use` re-exports.
  Example: `model.rs` lists the files, and `model/policy.rs` holds the policy.
- Tests live in `<module>/tests.rs`, so moving code never renames a test. Guides quote some test names.
- When you add a file, add it to its module's reading order and to the [lab README](level-1-rl-expanded/README.md) source map.

## Do not change without review

| What | Why |
| --- | --- |
| `todo!()` in `src/exercises/*/practice.rs` | These are the learner's unfinished starters. |
| Test names in `REQUIRED_TASKS` in [check.py](scripts/check.py) | They are the curriculum's task list. |
| Printed numbers and wording | Guides and integration tests quote them. |

## Check your work

```bash
python3 scripts/check.py
```

CI runs the same script with Rust 1.95.0. It checks formatting, Clippy (default, pedantic, nursery, and cargo groups, all as errors), rustdoc, tests, document links, lesson commands, and exercise selectors.

A lint is fixed in the code, not silenced. The only exceptions are four `#[expect(..., reason = ...)]` attributes where a learner contract fixes the signature; add another only with the same kind of reason.

## Where things are

- [Lab README](level-1-rl-expanded/README.md): commands, exit codes, source map, and checks.
- [Learning path](level-1-rl-expanded/guides/LEARNING_PATH.md): the learner's route, one session at a time.
- [Structure record](level-1-rl-expanded/guides/maintenance/STRUCTURE_REVIEW_AND_PLAN.md): past structural decisions and their evidence.
