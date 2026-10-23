# Plan and delivery: easier to start, understand, and return to

**Status: implemented and locally verified on 2026-10-22; learner evaluation pending.** The original review below describes `a7f2b17` on 2026-10-21. Its implementation stages are retained as the design rationale; the delivery record at the end states what was checked.

**Chosen approach:** keep [LEARNING_PATH](../LEARNING_PATH.md) as the single place that tells the learner what to do next. The return activity received an executable walkthrough, and the approved pattern was applied across the route. This technical walkthrough does not establish that the pilot improved an actual learner's experience.

The intended outcome is observable: you can identify the next action, attempt one small computation, explain a changed case, and resume after a break without reconstructing the whole lesson. Shorter text alone is not the acceptance criterion.

## Findings at the reviewed baseline

| Evidence at `a7f2b17` | Planned response |
| --- | --- |
| The [root README](../../../README.md) already has one starting link, and Cargo defaults to the maintained lab. | Preserve this entry point and workspace. |
| [Session 1](../LEARNING_PATH.md#session-1-follow-one-decision-15-minutes) sends the learner into three source files. | Show one local branch trace first; label cross-module reading as a later exploration. |
| [Session 2](../LEARNING_PATH.md#session-2-score-a-future-then-score-uncertainty-20-minutes) combines reward, return, expectation, three commands, and two implementations. | Separate return from expectation into two independently stoppable activities. |
| [Session 6](../LEARNING_PATH.md#session-6-write-the-small-pieces-yourself-25-minutes) introduces the first explicit implementation task. | Place the existing return and expectation exercises beside their concepts. |
| The [exercise introduction](../EXERCISES.md#attempt-one-exercise) demonstrates task 4, while the route starts practice at task 1. | Use the same first task in both places. |
| The [composition type legend](../COMPOSITION_LESSON.md#read-the-types-before-composing-them) precedes the first concrete exercise. | Show a tiny value transformation first and introduce each alias where needed. |
| The [attempt template](../LEARNING_RECORD.md#record-an-attempt) has thirteen fields; learner results remain pending. | Offer a three-line restart note by default and retain the full assessment as optional detail. |
| [Arithmetic assertions](../../src/exercises/arithmetic/cases.rs) report numbers without naming the failing input; instructions to add cases coexist with instructions to edit only one function. | Name failed cases and provide a clearly marked place for learner cases. |
| [Arithmetic demonstrations](../../src/exercises/arithmetic/worked.rs) print all ten worked answers in one test. | Make help selectable for one concept while preserving the complete reference demonstration. |

These were observed sources of possible friction, not measured difficulties experienced by this learner. The implementation preserves separate solutions, shared test cases, conceptual hints, the ownership diagnostic, and terminal-versus-horizon explanations. Sections 1 to 4 below retain the approved plan.

## 1. Pilot one focused activity

**Scope:** the discounted-return part of Session 2, exercise 1, and the restart note. Edit the existing learning path, exercise guide, and learning record. In arithmetic `cases.rs`, label the existing return failures; in `practice.rs`, mark where a learner can add a case inside the existing exercise-1 test wrapper. Preserve every shared assertion. This is the first implementation slice; do not reorganize the whole course before trying it.

At the top of the existing route, add a compact session index and a returning-learner instruction to open the last restart cue. Preserve one obvious default for a first visit.

Use one consistent card:

| Part | What the learner sees |
| --- | --- |
| Purpose | One sentence explaining the question this computation answers. |
| Predict | One small case to think through before running anything. |
| Do now | One named function, one file, and one copyable command for the current step. |
| Check | What the result means, including an expected unfinished TODO. |
| Explain | One changed case, with the solution closed. |
| Stop or continue | An exact restart cue and one default next activity. |

Each numbered step has one primary action. A card can contain several steps, but it must not present a menu of competing commands. Keep hints and deeper explanations after the immediate action, with clearly labeled optional links or disclosure blocks. Check both rendered Markdown and raw text; folded content is a convenience, not an answer-security boundary.

Offer a brief entry activity, such as predicting one boundary case, alongside the complete activity. Time estimates are adjustable planning aids. Stopping partway is valid; do not introduce countdowns, streaks, or speed-based success criteria.

The default restart note has three lines:

```text
Where I stopped: task and function.
What I observed or still do not understand: include help used, if any.
Next action: exact file, function, and command.
```

Keep the richer correctness, explanation, assistance, and actual-delay record available for deliberate assessment. A restart note does not count as a completed assessment.

**Acceptance:** one activity can be followed without reading the companion or choosing from the whole exercise bank; its command selects exactly the intended task; answers remain separate; stopping leaves a usable next action.

## 2. Put practice next to the idea

Once the pilot works, keep the concepts in their current dependency order and distribute the existing exercises through that route:

| Concept | Existing practice to connect |
| --- | --- |
| One action and one uncertain response | Trace one branch; optional probability-validation task 4. |
| Rewards across time | Return task 1 immediately after its explanation. |
| Alternative outcomes | Expectation task 2 immediately after its explanation. |
| Task endings and limited horizons | Preserve the boundary comparisons before introducing bootstrapping. |
| Exact Bellman evaluation | Backup task 10 after the smaller remaining-problem explanation. |
| Composition | Ownership and branching support as needed, then independent composition and law checks. |
| Learning values from observations | Target task 3, running mean task 7, and update task 8, each with its own purpose. |
| Optional control extension | Greedy selection 5, epsilon-greedy 6, and Q-learning target 9. Label these as building blocks beyond the implemented fixed-policy learner. |

Preserve all 17 task identities and their shared cases. Update cross-links and prerequisite text together when changing session headings. Keep policy comparison in the core route, with an explicit distinction between grid search and learning a policy.

Use the sequence **concrete example, prediction, small attempt, feedback, changed case**. Put the information needed for a prediction before the prompt: Session 1 should identify the exact policy and world match arms to inspect. When support is needed, offer an analogous worked example and then a partial completion. Close that support before assessing an independent attempt. Keep Rust ownership support separate from probability support until the learner is ready to combine them.

For category theory, begin with two concrete computations and ask whether their observable probabilities agree. Introduce `map`, composition, identity, and associativity terminology after the behavior is visible. Keep the limits of floating-point law tests explicit.

**Acceptance:** every core concept has nearby practice and one default next step; no session requires completing an entire exercise bank; reference tests are never presented as learner progress.

## 3. Make help and feedback local

In [EXERCISES](../EXERCISES.md) and [COMPOSITION_LESSON](../COMPOSITION_LESSON.md), keep contracts beside the task and use a consistent help order: conceptual hint, analogous worked example, then the separate solution after an attempt or explicit request.

Extend the pilot's named failure messages and learner-case locations to the other tasks. A message should identify the input or boundary, expected value, and actual value without revealing the implementation. Put learner-added assertions in the corresponding practice wrapper, with explicit editing instructions; leave shared acceptance cases intact. Check an intentionally incorrect return implementation in a disposable copy to confirm that feedback identifies the failed case.

Split the arithmetic worked demonstration into individually selectable examples, preserving the existing aggregate test and its checks. Do not automatically display unrelated answers. Keep direct Cargo commands as the default; consider a thin exercise command only if the pilot shows that selectors are a recurring obstacle.

Provide a short feedback guide near the first exercise:

| Observed result | Next action |
| --- | --- |
| Selected starter stops at its TODO | Implement that function; this is the expected starting state. |
| Compiler error | Read the first diagnostic and its file location; other code in the same target can block compilation. |
| Assertion mismatch | Compare the predicted value with the reported actual value and the task contract. |
| Zero selected tests | Recopy the complete selector; this does not verify the attempt. |
| Selected test passes | Explain the boundary, then try a changed case with the solution closed. |

Give each delayed prompt in the learning record a matching checking rubric and separately stored feedback, proposed as `guides/RETRIEVAL_SOLUTIONS.md`. Its current broad reference links do not resolve each changed case. Preserve answer-hidden attempts and distinguish a correct calculation from a sound explanation.

**Acceptance:** help stays on the selected concept; individual exercise and worked-example selectors run one test, while explicitly labeled aggregate checks retain their wider scope; a reference answer cannot silently stand in for a learner attempt.

## 4. Edit for clarity, then validate

Use consistent labels, short paragraphs, numbered actions, and stable terms such as reward, return, expected return, policy, and value. Introduce each symbol beside its meaning and Rust variable. Keep essential input contracts visible.

In source comments, retain explanations of purpose, invariants, and non-obvious Rust choices. Remove repeated narration where it hides the computation. The existing `map` and `and_then` diagrams are useful models: connect values, weights, and ownership in one small example. Avoid splitting modules solely to meet a line-count target.

Keep the companion and interactive labs as optional references reached through precise links. Do not add another starting document or copy the same lesson into several places. Use a small diagram only where it explains a relationship, with a text explanation alongside it.

After implementation, run `python3 scripts/check.py`, review affected command output, and inspect the rendered learner route. Retain existing correctness checks, task identities, numerical behavior, and separate answers. Changes to worked examples need focused tests of those examples, not tests that merely count headings or prose.

**Acceptance:** all automated checks pass; the learner can distinguish the main activity from optional detail; an actual attempt records explanation and transfer separately from test success.

## Evidence and strategy check

W3C's [short critical paths guidance](https://www.w3.org/WAI/WCAG2/supplemental/patterns/o5p02-short-paths/) recommends minimizing required steps and separating optional work. This supports the proposed single next action and optional detail. It is supplemental cognitive-accessibility guidance, not a WCAG conformance test.

The IES [instruction and study practice guide](https://ies.ed.gov/ncee/WWC/PracticeGuide/1), published in 2007, supports alternating worked examples with problem solving, connecting concrete and abstract representations, explanatory questions, and later retrieval. These are established educational recommendations. Their application to this Rust/RL course is a design hypothesis, not demonstrated ADHD-specific effectiveness. Sources checked on 2026-10-21.

The current strategy works when navigation, task size, and restart effort are the main obstacles. It is insufficient when prerequisite understanding is missing, feedback remains confusing, or the learner needs a different presentation format.

Two alternatives remain credible: split every lesson into small files, or build a book/site with guided navigation. Small files can shorten individual pages but increase switching and link upkeep. A site can add navigation controls but adds another toolchain. Start with the existing Markdown route because it already owns progression; reconsider these alternatives only after observing its limits.

**Premortem:** the pilot could fail because documentation still makes the learner choose the next step, the exercise harness exposes too much unrelated output, or shorter explanations remove the reasoning needed for transfer. Those failures belong respectively to navigation, feedback, and instructional content. Fix the owning part instead of adding more general prose.

**Smallest experiment:** implement only the return activity, then use one actual attempt and one later return to it. Observe whether the learner can locate the next action, run the intended test, explain a changed case, and resume from the note. Ask for a simple 1-5 effort rating plus what caused friction; do not treat the rating as mastery.

Persevere if action-finding and resumption become easier while explanation remains sound. Iterate if the route is clearer but one step still needs clarification. Pivot to prerequisite support or another presentation when repeated attempts show that navigation changes do not address the actual obstacle. A clearer page with equally difficult resumption or worse transfer would contradict the hypothesis that this redesign is sufficient. These observations guide the next change; they are not a controlled efficacy study.

## Delivery and verification

Implemented on **2026-10-22**:

- The route has a visible index, return cue, focused return/expectation activities, nearby exercises 1/2/10, and optional implementation depth. Composition introduces concrete behavior before local type explanations; experience supplies the trace inputs before prediction.
- Both tracks retain their 17 unanswered TODOs and shared acceptance cases. Failures identify the relevant case or event. Every existing practice wrapper has a place for learner-added assertions.
- Arithmetic has ten individually selectable worked demonstrations; the existing aggregate demonstration remains available. Delayed prompts have checking rubrics and separate [retrieval feedback](../RETRIEVAL_SOLUTIONS.md).
- The [learning record](../LEARNING_RECORD.md#quick-restart) defaults to a three-line restart cue. Its detailed assessment is optional and still distinguishes assistance, explanation, correctness, and actual delay.
- CLI suggestions are labeled optional, so the guide owns the default next step. Long commands use shell continuations for readable flags. The existing checker now recognizes indented CLI examples inside numbered steps.

Actual validation:

- `python3 scripts/check.py` passed formatting, strict Clippy, locked tests, links, CLI examples, exact selectors, and all 17 starter/solution pairs. The Rust suites passed **79 tests** with **17 starters ignored**; explicitly selecting those starters produced **17 expected TODO failures** and no completed learner attempts.
- After the final navigation and command-wrapping edits, the checker's documentation/exercise checks passed again: **416 local links and anchors**, **36 CLI commands**, and **38 exact test selectors**. No source changes followed the successful Rust checks.
- A disposable copy followed the return exercise from untouched TODO, to an intentionally wrong first-reward weight, to a corrected implementation plus a new negative-reward case. The wrong implementation reported the immediate-reward input with expected `3` and actual `1.5`; the corrected attempt passed. The real starter was left unanswered.
- All three Rust snippets in the separate retrieval feedback compiled with edition 2021 and warnings denied, and ran successfully. Parcel, sensor, and robot answers were also checked with exact rational arithmetic. A separate agent reviewed their explanations and assumptions.
- A temporary local Pandoc/GFM preview was inspected in the browser. The root link, return anchor, exercise link, and restart link worked; optional support was collapsed by default and opened with the keyboard. The initial preview exposed clipped command flags, which motivated the continuation-line adjustment. This checks this renderer, not every editor or GitHub layout.

The numerical algorithms, model, rewards, dependency versions, and solution implementations are unchanged. Existing source explanations were retained where they already explain the invariant clearly. During the 2026-10-22 implementation, the then-untracked `scratch/patch_practice.py` was neither executed nor modified; that validation did not include a commit, push, or hosted CI run.

The requested implementation is complete. The remaining experiment is one actual return activity and a later resumption, with an optional 1-5 effort rating and an independently explained changed case. Ease of study, ADHD usability, retention, and transfer are not established by the automated checks.
