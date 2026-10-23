# Contributing to rizz-rl

You do not need to be an RL expert to contribute. A confusing sentence, a missing prerequisite, or an unclear error message is useful feedback. Small improvements that help someone take the next step are welcome.

Start with the [project overview](README.md). The [learning path](level-1-rl-expanded/guides/LEARNING_PATH.md) owns the lesson sequence; the [lab README](level-1-rl-expanded/README.md) owns commands and the source map.

## Choose one small contribution

- **Learning friction:** open an issue describing the page, step, and point where you got stuck. You do not need to propose a fix or share your exercise answer.
- **A bug:** include the command, expected behavior, actual output, and enough information to reproduce it. Search existing issues first.
- **A fix:** work on a branch in your fork, keep the change focused, run the checks below, and open a pull request. A small correction does not need a separate issue first.
- **A larger lesson or behavior change:** open an issue describing the learning objective and proposed scope before investing in implementation.

## Keep the learning experience intact

The maintained lab is in `level-1-rl-expanded/`. The root workspace runs it by default. Historical work in `scratch/` is separate from the maintained lessons.

- Put a concrete example and a prediction before optional theory. Give one clear next action and a stopping point. Follow the existing learning path rather than creating a parallel route.
- Keep answers separate from attempts. The shared lab has **17 unanswered starters**, ten arithmetic and seven composition tasks. Preserve their `todo!()` bodies and ignored tests in a contribution; keep reference implementations in the separate `solutions.rs` files. Your personal learning checkout can contain your own completed attempts.
- Preserve the shared acceptance cases. Add meaningful cases for a behavior change, and explain any proposed contract change. Do not weaken assertions to make a check pass.
- Keep exercise instructions, source comments, commands, and links consistent. The [arithmetic guide](level-1-rl-expanded/guides/EXERCISES.md) and [composition lesson](level-1-rl-expanded/guides/COMPOSITION_LESSON.md) describe the existing task conventions.
- Explain the scope of RL claims. Exact planning, fixed-policy value learning, and policy optimization are different activities. A passing test establishes covered software behavior, not learner mastery.

Use plain language and descriptive names. Prefer a small change to an existing explanation or module over new dependencies or another framework.

## Check your change

From the repository root, run:

```bash
python3 scripts/check.py
```

You need Python 3.9 or later, Rust/Cargo, rustfmt, and Clippy. See the [lab README](level-1-rl-expanded/README.md#check-the-lab) for the underlying commands and the [CI workflow](.github/workflows/ci.yml) for its Rust version.

The script checks formatting, strict Clippy, locked tests, local links and anchors, documented lesson commands, and exercise selectors. It also runs each starter separately and identifies its expected TODO failure. **An expected TODO failure is not a failing reference test.** Ordinary `cargo test` skips starters and runs the references. Unexpected failures still need investigation.

For documentation changes, also follow the changed instructions yourself. Confirm that a newcomer can find the input, run the command, interpret the result, and identify the next step. Automated links and tests cannot establish that an explanation is clear.

## Describe the result

In your pull request, explain what was confusing or broken, what changes for the learner, and which checks you actually ran. Include commands and results; if a check could not run, state the reason. Call out changed numerical behavior or exercise contracts explicitly.

Keep credentials, personal learning records, and unrelated generated files out of the contribution. Share only the output needed to understand the change.

## Work with other learners

Explain a correction without belittling the person who needed it. Questions about prerequisites and mistakes in an attempt are welcome. Keep feedback specific to the code, reasoning, or instructions.

Contribute material you have the right to share under the project's [MIT license](LICENSE). Credit external sources and preserve any required notices. If provenance is uncertain, describe it before adding the material.
