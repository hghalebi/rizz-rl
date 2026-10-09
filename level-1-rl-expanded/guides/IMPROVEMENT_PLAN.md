# Improvement plan: Rust, category theory, and RL

Status: approved for implementation on 2026-10-08. This document now tracks the
delivery as well as its acceptance criteria. It builds on the earlier
[pedagogy review](PEDAGOGY_REVIEW.md), which remains the source for the research
evidence and its limitations. Preparing materials does not complete a learner
pilot or demonstrate retention.

This is the pedagogical delivery record from **2026-10-08**. Current navigation
is owned by [LEARNING_PATH.md](LEARNING_PATH.md); the later workspace and source
reorganization is tracked in the [structure plan](maintenance/STRUCTURE_REVIEW_AND_PLAN.md).
Source links and operational commands here have been updated to their current locations.

## Implementation status

| Phase | Material or program delivery | Verification / learner evidence |
| --- | --- | --- |
| 1. Correct teaching contracts | Horizon-aware policy improvement, a numerical counterexample, and comparison labels are implemented. | Counterexample and CLI checks passed in the combined suite. |
| 2. Adapt support to the learner | [Composition lesson](COMPOSITION_LESSON.md), separate diagnostic routes, and staged tasks are implemented. | Worked/reference tasks and the ownership compile-fail check passed; no learner pilot recorded. |
| 3. Concrete composition laws | Reference and learner law tasks use the actual distribution implementation. | Numerical law checks passed, including duplicate and zero-probability branches; no claim of a general mathematical proof. |
| 4. Learn from observations | [Experience lesson](EXPERIENCE_LESSON.md), `td-trace`, and `learn-value` are implemented. | TD/sampling/CLI checks passed and nine seed/budget runs completed; [observed errors](EXPERIENCE_LESSON.md#4-compare-budgets-and-seeds-20-minutes) include a nonmonotonic case. Learner explanation pending. |
| 5. Observe delayed performance | [Learning record](LEARNING_RECORD.md) supplies changed-context prompts, delay tracking, and an assessment rubric. | Materials prepared; actual attempts, retention, and transfer remain pending. |

The table distinguishes implementation from learner outcomes. Update a check
only after it runs; do not mark the pilot complete from passing program tests.

Combined implementation checks on 2026-10-08: **66 tests passed and 17 starter
tests were intentionally ignored**. The passing tests comprised 44 unit tests,
11 CLI tests, one ownership compile-fail verifier, and ten arithmetic solution
tests. The ignored tasks comprised seven composition starters and ten arithmetic
starters. Formatting and strict Clippy checks across all targets passed.
The final documentation audit verified 117 local links and anchors, all 14
composition commands, and all nine distinct experience-lesson commands. The
seven selected composition starters failed only at their intended TODOs; the
first arithmetic filter selected one expected TODO failure. Reference commands
succeeded. These checks cover the expanded lab; the original scratch program, now in [scratch/original](../../scratch/original/README.md), was
preserved and is outside this implementation's validation scope.

## Target outcome and starting point

The learner should be able to predict a probabilistic computation, explain its
Rust ownership decisions, test its composition rules, and connect observed
transitions to a learned value estimate. Assume short sessions of roughly
20 to 30 minutes; use a diagnostic rather than assuming a Rust skill level.

Keep the existing exact planner, ten exercises, separate solutions, and shared
acceptance cases. The earlier exercise delivery verified 35 passing tests and
ten intentionally ignored starter tests. That is a historical baseline, not the
verification result for the new implementation or evidence of learner mastery.

Recommended learner sequence: **correctness repair -> one adaptive pilot ->
composition laws -> learning from experience -> adapt using learner evidence**.
The materials can be prepared together; choose the learner's next step from
actual attempts rather than requiring every stage or expanding the curriculum
before observing where help is needed.

## Phase 1: correct the teaching contracts

Change existing files:

- In [Lab 14](INTERACTIVE_LABS.md#lab-14-policy-improvement-depends-on-the-remaining-horizon), use
  `pi_new(h, s) = argmax_a Q(h, s, a)`. Explain that evaluation decreases `h`
  after each action. Keep the current fixed-policy planner unchanged.
- Add a tiny worked counterexample where a terminal action pays 2 immediately,
  while another pays 0 now and 5 at the next step. With discount 0.5, compare
  horizons 1 and 2. Include an executable numerical regression case.
- In [`cli.rs`](../src/cli.rs) (formerly `learning.rs`), rename the comparison columns to
  `Tree E[G]`, `Bellman V_h`, `absolute difference`, and `cached subproblems`.

Acceptance: the example demonstrates the horizon-dependent choice; CLI labels
distinguish values from counts; the existing default return and grid winner
remain unchanged. Update the relevant CLI expectation if its header is tested.

## Phase 2: pilot one lesson with gradually reduced support

Use probability composition as the pilot because it connects the three subjects
already present in the repository. The focused [composition guide](COMPOSITION_LESSON.md)
is linked from [the learning path](LEARNING_PATH.md).

The lesson offers five steps, with support chosen from demonstrated needs:

1. **Optional diagnostic:** predict one move error involving a `String`, explain
   borrowing versus consuming a value, and calculate one joint probability.
   Score Rust and probability separately. If both answers and explanations are
   sound, start at the independent exercise. If one reveals a gap, offer only
   its relevant worked and completion steps. If both reveal gaps, use the full
   sequence. Confirm this tentative routing with a new case; two probes do not
   establish readiness for the whole capstone. If the diagnostic is skipped,
   let the learner choose a challenge or a worked example and adjust afterward.
2. **Worked example:** explain a small `Distribution<String>` computation using
   purpose labels such as transform an outcome, branch, and weight a path.
3. **Completion exercise:** remove one meaningful step from a similar example.
   Offer a conceptual hint separately from the implementation.
4. **Independent exercise:** solve a comparable problem with less scaffolding.
   Keep its worked solution in a separate file and use shared contract cases.
5. **Transfer check:** change the context to a small navigation or queueing
   problem and ask the learner to explain which structure stays the same.

Keep runnable worked code, learner TODOs, and solutions visibly separate. Use
small internal lesson/test modules to access the actual distribution type; do
not create a second world model or expose a new public library solely for this
lesson. If a deliberately noncompiling ownership example is supplied, test that
it fails for the intended ownership reason and keep it outside normal targets.

Change the progression rule in `LEARNING_PATH.md`: advance after an unaided
explanation and a successful changed case. Keep confidence as a recorded signal,
not the deciding threshold. A learner who needs help should be offered the
smallest relevant example, not required to reread the entire companion.

Acceptance: each stage has one visible objective and one exact command; a
learner can attempt one task without implementing unrelated TODOs. Record the
prediction, observed result, assistance used, and explanation. This phase can
deliver working materials; whether the pilot helps this learner requires an
actual attempt and remains unverified until then.

## Phase 3: make the category-theory laws concrete

Extend the pilot with three small tasks, introducing vocabulary after the
observable behavior:

| Task | Observable claim | Terminology introduced afterward |
| --- | --- | --- |
| Map identity and composition | Mapping the identity changes nothing; two maps agree with one composed map. | Functor laws |
| Chaining with certainty | Adding a certain starting or ending step preserves the resulting probabilities. | Left and right identity for bind |
| Regrouping a probability computation | Two parenthesizations yield the same event probabilities. | Associativity and Kleisli composition |

Place reference law tests next to the existing private `Distribution` code or
in a test-only child module. Keep learner and reference implementations in
separate files. Compare aggregated probabilities across the union of outcomes,
including duplicate branches, with an explicit floating-point tolerance.

Acceptance: test normalized finite distributions, duplicate outcomes, and
zero-probability branches. The learner predicts an equivalence and constructs a
new example. Explain that selected numerical tests illustrate laws; arbitrary
Rust closures and floating-point arithmetic do not constitute a mathematical
proof of a lawful abstraction. No generic category-theory framework is needed.

## Phase 4: connect observations to a learned value table

Build one bounded capstone for **tabular TD(0) evaluation of the existing fixed
policy**. Learning an improved policy and Q-learning control are later steps.
Reuse the existing state, policy, world, and exact Bellman evaluator through a
small internal module, [`src/experience.rs`](../src/experience.rs).

The `td-trace` command exposes the mechanics with a supplied transition trace.
The `learn-value` command runs the sampler and learner. The
[experience lesson](EXPERIENCE_LESSON.md) links the two and distinguishes their
outputs from exact evaluation.

Keep responsibilities explicit:

- The sampler uses the fixed policy and world to produce observed transitions.
- The learner receives those observations and stores `V(steps_remaining, state)`.
  It has no access to transition probabilities or exact values.
- Each table entry has its own visit count. If the lesson uses learning rate
  `1 / visits`, use the count for that entry rather than a global episode count.
- True termination and zero remaining horizon remove continuation value for
  this finite-horizon objective. Initially add no separate interruption limit:
  every episode ends at termination or the selected horizon.
- Comparison code calls the existing exact evaluator using the same policy,
  initial state, horizon, and discount. Report initial-state error and visit
  counts; identify unvisited entries rather than presenting them as learned.

Bound episode count and horizon at CLI ingress. Use an established seeded RNG
implementation, checking and pinning its compatible dependency during this phase;
do not write a custom RNG. Test action and transition sampling with supplied
deterministic draws before depending on random experiments.

Acceptance has two distinct parts:

1. **Program behavior:** hand-calculated trace updates, terminal and horizon
   boundaries, independent visit counts, sampler boundary cases, repeatability
   for a fixed seed, and a complete CLI run pass their tests.
2. **Experiment evidence:** run several explicit seeds and episode budgets;
   report error against the exact initial-state value. Do not require error to
   decrease on every run, treat one favorable seed as accuracy proof, or claim
   convergence from a short demonstration.

Learner check: explain one table update and predict the effect of changing the
discount or terminal flag before rerunning. State clearly that the program
learns values for a fixed policy; it does not yet learn an improved policy.

## Phase 5: use delayed performance to choose further work

Provide short retrieval prompts for the next available study session and a
later session, for example the following week. These intervals are practical
starting points, not an experimentally established optimum for this learner.

The [learning record](LEARNING_RECORD.md) provides those prompts and a blank
record. No learner results are prefilled. Record the actual elapsed delay,
including when the next available session differs from the suggested timing.

Record correctness, explanation, and assistance separately from confidence.
Use changed contexts, not only the same function with different constants.
Distinguish a probability error, a Rust ownership error, an RL-boundary error,
and a forgotten step so the next task targets the actual difficulty.

Acceptance: at least one real delayed attempt is recorded before claiming
retention. Until learner evidence exists, mark results pending and keep further
curriculum expansion optional. Q-learning control and neural methods can follow
once the learner can explain the fixed-policy capstone independently.

## Strategy review: test the route before expanding it

The observable goal is an unaided prediction and explanation in a new case,
followed by a delayed attempt. The strategy is to repair misleading teaching
contracts, then pilot targeted support before adding more lessons. The review
identified a missing horizon argument in policy improvement and a confidence
threshold used for progression. Existing program tests validate examples, but
there is no recorded learner attempt establishing which skills need support.
That is the biggest remaining uncertainty.

Additional research checked on 2026-10-08 supports adaptive routing. A
[2025 meta-analysis of 60 experiments](https://www.pedocs.de/volltexte/2026/34113/pdf/Learn_and_Instr_2025_Tetzlaff_u.a._A_cornerstone_of_adaptivity.pdf)
found that the benefit of greater assistance depended on prior knowledge.
A [2026 experiment in collaborative diagnostic reasoning](https://link.springer.com/article/10.1007/s11412-026-09468-9)
found gains in declarative knowledge and process quality from self-explaining
worked examples, without corresponding gains in strategy application. Neither
study directly tests this Rust/RL curriculum. They support checking application
and adapting assistance, not assuming that more explanation improves transfer.

Two credible alternatives remain available:

| Route | When it works | When it does not work |
| --- | --- | --- |
| Targeted examples followed by independent composition | Ownership or probability is unfamiliar and the example addresses that gap. | Guidance repeats mastered material or hides a different prerequisite gap. |
| Challenge or capstone first, with help on demand | Independent probes succeed and the learner can explain their reasoning. | Rust mechanics consume attention needed to understand the RL objective. |
| Brief separate Rust and probability practice before recombining | Repeated integrated attempts obscure which skill is failing. | Isolated drills continue without returning to a shared computation. |

Premortem: the pilot fails if the lesson designer mistakes a syntax slip for an
RL misconception, the assessment accepts copied answers as transfer, or the
capstone compares different horizons or terminal rules. Own these boundaries:
score skills separately, record assistance on an unseen comparable case, and
keep the exact-value comparison contract in Phase 4.

Smallest learner experiment, supported by the materials but not yet run:

1. Spend roughly five minutes on separate ownership and probability probes.
   This is a timebox, not a speed requirement. Record answers and explanations.
2. Spend roughly fifteen minutes on the matching route, then attempt a new
   problem of comparable structural difficulty without notes or solutions.
3. At the next available session, attempt another changed case. Record the
   actual delay, correctness, explanation, and assistance used.

**Persevere** if immediate and delayed cases succeed independently. **Iterate**
on the smallest relevant example if one skill remains weak. **Pivot** to
challenge-first work if baseline probes already succeed, or temporarily split
the subjects if repeated attempts show interference between them.

The working hypothesis is that targeted support is sufficient for independent
transfer of this composition idea. A repeated critical misconception after the
targeted step and a comparable new case contradicts its adequacy for this
attempt. Baseline success instead contradicts the need for the full guided
sequence. Check fatigue and task difficulty before changing the wider course;
one successful pilot does not establish that the support caused the gain.

Phase 2 branches on separate demonstrated skills. The five phases, separate
solutions, and implementation boundaries remain the acceptance contract.
Curriculum effectiveness remains unverified pending learner evidence, even
after the material and program checks pass.

## Delivery and verification

Deliver each phase as a small, reviewable change. Preserve the original scratch
program and the separate solution convention. Keep the long companion as
reference material and update navigation instead of duplicating its theory.

For each implementation phase, run the relevant checks against the expanded lab:

```bash
cargo fmt --all -- --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
```

Also run the documented learner command, check intentional starter failures and
normal solution success, and validate changed documentation links. Formatting
and program tests validate the artifacts. The unaided and delayed checks
validate specific learner performances; neither replaces the other.

No new UI, service, database, course platform, or deep-learning framework is
needed for these phases. Use the status table to distinguish delivered material,
executed program checks, and learner evidence still to collect.
