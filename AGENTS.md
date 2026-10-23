# Agent guide

**This is a teaching project.** People read this code to learn Rust and reinforcement learning.

When clarity for a learner conflicts with brevity or production polish, choose clarity.

## Before you edit

1. Find the guide task that points at the code. Most are `**Read:**` lines in the [learning path](level-1-rl-expanded/guides/LEARNING_PATH.md).
2. Keep whatever that task asks the learner to find.
   - Session 2 asks which kind of weight "each loop" uses, so `discounted_return` and `expected_value` stay explicit loops.
   - Session 5 asks for "the `Option` representing an empty candidate set", so `best_policy` keeps its visible `Option`.
3. Keep teaching comments. When code changes, update the comment that explains it.
4. Write new comments the same way: a question, one idea per line, a small worked example, then a `Predict:` prompt.

## Keep the teaching shape

- Every lesson file opens with three `//!` lines: `Question:`, `Example:`, `Predict:`. Keep the example small enough to work on paper.
- The eleven checkpoints are listed in the [lab README](level-1-rl-expanded/README.md) under "Read the code in checkpoint order". A new checkpoint goes there too, with its file and session.
- Each learning-path session has one **Check yourself** test. The test name states the claim; `check.py` verifies that its `--exact` selector matches exactly one test, so renaming a test breaks CI until the guide is updated.

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

CI runs the same script with Rust 1.95.0. It checks formatting, Clippy, tests, document links, lesson commands, and exercise selectors.

## Where things are

- [Lab README](level-1-rl-expanded/README.md): commands, exit codes, source map, and checks.
- [Learning path](level-1-rl-expanded/guides/LEARNING_PATH.md): the learner's route, one session at a time.
- [Structure record](level-1-rl-expanded/guides/maintenance/STRUCTURE_REVIEW_AND_PLAN.md): past structural decisions and their evidence.
