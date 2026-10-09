# Review: learning Rust, category theory, and RL together

Reviewed on **2026-10-08**. This is a selected evidence review of the project before the improvement plan was implemented, not an exhaustive literature review or a claim that this project has been evaluated with learners. Findings below describe that earlier state. See the [improvement plan and delivery status](IMPROVEMENT_PLAN.md) for subsequent changes and pending learner evidence.

**Recommended direction:** keep the small executable lab, then strengthen the transition from seeing a worked example to solving a different problem unaided. More reading or more passing tests alone would not demonstrate that transition.

Source links below point to current locations. Findings and test counts describe
the reviewed 2026-10-08 state, not current defects or current verification. The
[structure delivery record](maintenance/STRUCTURE_REVIEW_AND_PLAN.md) tracks the later reorganization.

## Historical repository findings

The review covers commit `91074f3` plus the ten-exercise expansion in the working
tree. Two independent agents reviewed the instructional code and learner flow;
the integration review checked their findings against the current files.

### 1. Correct the finite-horizon policy-improvement instruction

**Instructional correctness issue.** [Lab 13](INTERACTIVE_LABS.md#lab-13--build-qs-a)
defines `Q(h, s, a)`, but Lab 14 changes this to `pi_new(s) = argmax Q(s,a)`.
Dropping `h` can lead the learner to freeze an action that is best only at one
remaining horizon. The general finite-horizon improvement should use
`pi_new(h,s) = argmax_a Q(h,s,a)` and decrease `h` during evaluation. Alternatively,
the material must explicitly introduce a separate stationary-policy problem.
This concerns the optional proposed lab, not the running Bellman evaluator.

A small counterexample explains why: a terminal action pays 2 now; another
action pays 0 now and reaches a state where the next action pays 5. With
discount 1, the preferred initial action changes between horizons 1 and 2.
This is an illustrative hand calculation, not a benchmark of this project.

### 2. Assess an independent result before advancing

**Assessment gap.** The last paragraph of
[the restart instructions](LEARNING_PATH.md#save-a-restart-cue) allows advancement
at confidence 3 or higher. The explanation checks and next-day recall are useful,
but this threshold is not evidence of independent performance. Retain confidence
as a useful signal; pair it with an unaided explanation and a fresh boundary or
representation change. The actual user's retention and transfer remain unknown.

### 3. Connect Rust prerequisites to the code the learner must read

**Prerequisite and support gap.** Session 1 sends the learner into
[`Distribution<T>`, `map`, and `and_then`](../src/probability.rs), which consume owned
values through closures. Most [practice tasks](EXERCISES.md) operate on borrowed
slices and copyable numbers. They do not establish readiness to explain moving
a `String`, borrowing, or cloning one history for each branch.

Add an optional short diagnostic using `Distribution<String>`: predict a move
error, decide which value to borrow, and explain why branches need independent
histories. Let a successful learner skip this support. For an unfamiliar RL
operation, offer a purpose-labeled example and one partial completion before a
blank task. The current starter-to-solution jump offers no intermediate rung.
This is a design recommendation, not a measured diagnosis of this learner.

### 4. Make the category-theory connection executable

**Missing learning outcome.** The existing tests check concrete `map` and
`and_then` results, but the ten exercises contain no identity, composition, or
bind-law task. The operations provide a useful starting point for category theory.
Introduce terminology only after the learner predicts equivalent computations:

- `map(identity)` leaves observable probabilities unchanged.
- Mapping `f` and then `g` agrees with mapping their composition.
- `certain` acts as the identity when chaining probabilistic steps.
- Regrouping three compatible probabilistic steps preserves the resulting law.

Use types as the objects and appropriately constrained pure functions as arrows
in the teaching model; an individual RL state is a value of its state type.
Compare aggregated event probabilities, since branch order and duplicate entries
can differ. Work with valid finite distributions and tolerance-aware comparisons;
floating-point tests illustrate these laws and do not prove exact algebraic laws
for arbitrary Rust closures, invalid weights, panics, or side effects.

### 5. Add an integration task for learning from experience

**Curriculum gap, not a scope misrepresentation.** The project accurately labels
its planner as using a known model. The new exercises compute learning components
but do not connect observations, persistent estimates, action choices, and
evaluation. A learner can pass their arithmetic tests without knowing how those
pieces form an RL loop.

Use a tiny capstone: first update a value table from a supplied transition trace;
then collect sampled episodes and compare estimates with the exact evaluator.
Keep terminal handling and the horizon objective consistent. Fixed seeds support
reproducibility; multiple seeds and held-out evaluation are needed before drawing
performance conclusions. This capstone is proposed, not implemented.

### 6. Label displayed quantities precisely

**Small presentation issue.** The `compare` table in
[`cli.rs`](../src/cli.rs) (formerly `learning.rs`) labels an expected-return column
`trajectories`, while `evaluate` uses trajectories as a count. Prefer
`Tree E[G]`, `Bellman V_h`, and `cached subproblems` to distinguish values from
counts. The numerical comparison itself passes its tests.

### What already works

Preserve the compact predict/run/explain sessions, separate solutions, shared
acceptance cases, explicit terminal-versus-cutoff distinction, and the exact
tree/Bellman comparison. The model is clearly identified as invented. The long
companion is a reference; there is no evidence that reading all of it should be
a prerequisite for the small labs.

## What the research supports

The following table separates reported results from applications inferred for this repository. The cited studies do not directly establish effectiveness for an adult learning Rust, category theory, and RL together. Where a systematic review is used, it is explicitly identified as a synthesis rather than a new experiment.

| Evidence and date | Population and design | Reported finding | Limits and inferred application here |
| --- | --- | --- | --- |
| [Margulieux, Morrison, and Decker, 2020: subgoal-labeled programming examples](https://link.springer.com/article/10.1186/s40594-020-00222-7) | 265 introductory Java students, five course sections, semester-long quasi-experiment. | Subgoal instruction improved early quizzes and was associated with fewer failing or incomplete outcomes. Average exam performance was not significantly better. | Students were not individually randomized; an intervention instructor was a researcher. **Inference:** label the purpose of a few code steps, then check independent performance rather than assuming clearer comments guarantee mastery. |
| [Atkinson, Renkl, and Merrill, 2003: fading with self-explanation](https://www.researchgate.net/publication/200772684_Transitioning_From_Studying_Examples_to_Solving_Problems_Effects_of_Self-Explanation_Prompts_and_Fading_Worked-Out_Steps) | Two randomized probability-learning experiments: 78 university students and 40 high-school students. Original paper uploaded by author Alexander Renkl. | Removing worked steps progressively while prompting the underlying principle improved near and far transfer in these experiments. | Small, structured probability tasks; this does not establish broad transfer to software design. **Inference:** provide an optional partially completed intermediate task, and ask why the missing step is required before giving a blank implementation task. |
| [Agarwal, Nunes, and Blunt, 2021: classroom retrieval review](https://pdf.poojaagarwal.com/Agarwal_etal_2021_EDPR.pdf) | Systematic synthesis of 50 experiments across school, university, and medical classrooms; total reported sample 5,374. | Of 49 calculated effects, 57% indicated medium or large benefits from retrieval practice. | Varied tasks and comparison conditions; publication bias possible; only 6% of experiments were outside Western, educated, industrialized, rich, democratic settings. **Inference:** ask for a short reconstruction without notes before reopening the solution. This is a memory activity; rerunning Cargo is not. |
| [Cepeda et al., 2008: spacing and retention](https://labs.biology.ucsd.edu/rifkin/courses/bieb100/f14/Cepeda_et_al_2008_Psychological_Science_Spacing_Effects_in_Learning_A_Temporal_Ridgeline_of_Optimal_Retention.pdf) | 1,354 participants learned facts online; study gaps and final test delays varied, with final delays up to one year. | The best gap between study sessions depended on how long the information needed to be retained. Longer is not always better. | Factual recall, not programming or mathematical proof. **Inference:** revisit a solved concept after a delay. No particular daily or weekly schedule is established as optimal for this learner. |
| [Kestin et al., 2025: designed AI tutor](https://www.nature.com/articles/s41598-025-97652-6.pdf), published June 3 | Randomized crossover study, 194 eligible Harvard undergraduate physics students, two lessons over two weeks; AI study at home versus active learning in class. | The structured AI condition achieved higher immediate post-test scores; median recorded AI study time was 49 minutes versus an assumed 60 minutes of classroom learning. | Different settings and pacing; immediate topic tests, not demonstrated long-term retention or Rust competence. **Inference:** use targeted feedback and learner-controlled pace, then assess without assistance. The result does not show that any chatbot is a better teacher. |
| [Bastani et al., 2025: AI assistance and unaided performance](https://www.pnas.org/doi/10.1073/pnas.2422633122), published online June 25 | Classroom-randomized trial with nearly 1,000 Turkish high-school math students; four sessions comparing unrestricted-style GPT assistance, a tutor with instructional safeguards, and control. | Both AI conditions improved assisted practice. The unrestricted-style condition scored 17% lower than control on subsequent unaided exams; the tutor condition largely removed that harm but did not show a positive unaided effect. | One school, mathematics, and short-term exams. **Inference:** keep solutions available, record whether help was used, and assess later unaided work separately from assisted completion. |

### A relevant 2026 programming update

[Zhang et al., published January 13, 2026](https://link.springer.com/article/10.1186/s40594-025-00591-x), examined worked versus erroneous examples with and without self-regulation prompts in a four-week Python course. There were 95 eighth-grade participants, with three excluded from analysis. The class-level quasi-experiment found an interaction: worked examples benefited from scaffolding, while erroneous examples performed better than worked examples without it. Small samples and two classroom contexts limit causal confidence; cognitive load was not measured directly. The useful inference is to match support to the task, not to declare either debugging or worked examples universally superior.

The two AI trials above address different designs, settings, and assessments. Their results support examining the instructional interaction and the assessment boundary. They do not justify a universal claim that AI either improves or damages learning. No recommendation here depends on a 2026 preprint, inferred brain mechanism, or a prescribed learning style.

## Translate the evidence into a small teaching loop

These are **proposals**, not features already delivered or proven effective for this learner.

1. **Diagnose one prerequisite.** Before a lesson, ask one hand calculation and one small Rust tracing question. For example, identify the type carried by a distribution and compute one complete branch probability. A correct answer to one does not establish the other.
2. **Show a worked example with purpose labels.** Use labels such as “weight this outcome” and “combine alternatives,” rather than narrating every token. Explain the semantic reason for the step. This applies the subgoal evidence without assuming the learner needs every scaffold indefinitely. [Programming study](https://link.springer.com/article/10.1186/s40594-020-00222-7).
3. **Offer one intermediate completion task.** Keep most of a small example and remove one meaningful step. Ask which principle determines that step, then move to independent implementation when it can be explained. Retain the existing blank exercises for learners ready for them. [Fading and self-explanation study](https://www.researchgate.net/publication/200772684_Transitioning_From_Studying_Examples_to_Solving_Problems_Effects_of_Self-Explanation_Prompts_and_Fading_Worked-Out_Steps).
4. **Check a different case.** Change the story, boundary condition, or representation, not just one numeric constant. An example would move from conversation outcomes to a small queueing or navigation problem while preserving the probabilistic operation. Treat success as evidence for that transfer task, not unrestricted expertise.
5. **Retrieve after a delay.** On the next available study day, reconstruct the rule before reading notes; revisit it again the following week if useful. Those intervals are a practical starting proposal, not a research-derived optimum. Correct errors after the attempt. [Retrieval review](https://pdf.poojaagarwal.com/Agarwal_etal_2021_EDPR.pdf), [spacing study](https://labs.biology.ucsd.edu/rifkin/courses/bieb100/f14/Cepeda_et_al_2008_Psychological_Science_Spacing_Effects_in_Learning_A_Temporal_Ridgeline_of_Optimal_Retention.pdf).

A tutor can support this loop by responding to the learner's current attempt with one relevant explanation or hint. Full solutions remain available on request. Record assistance, then use a later unaided task to determine what can now be performed independently. This is an instructional proposal informed by the [AI assistance trial](https://www.pnas.org/doi/10.1073/pnas.2422633122), not an instruction to impose new access restrictions on the solutions.

## Assess what the learner can do

Use separate observations rather than a single completion score:

| Observation | What it supports | What it does not establish |
| --- | --- | --- |
| A Rust implementation passes the current tests. | The implementation handles the tested cases. | Who produced the solution, retention, or understanding of untested cases. |
| The learner explains a boundary in their own words. | Understanding expressed at that moment. | Reliable application after a delay or in another context. |
| The learner reconstructs a function without notes later. | Retention of that function or principle at that delay. | Ability to design a complete RL agent. |
| The learner solves a new problem and explains the mapping. | Transfer to that particular problem. | General transfer to arbitrary Rust, RL, or category-theory tasks. |
| Confidence is rated from 1 to 5. | The learner's perceived certainty or difficulty. | An objective mastery measure. |

For each attempt, record the date, task, prediction, actual outcome, assistance used, and one explanation or error. Use the next task to address the observed difficulty: probability reasoning, Rust mechanics, RL semantics, or recalling a previously understood step. This rubric is proposed here; it is not a validated psychological instrument.

## Proposed priority sequence

| Order | Proposed change | Observable acceptance criterion |
| --- | --- | --- |
| 1 | Repair Lab 14's time-dependent policy notation and the comparison headers. | A worked two-horizon example changes its greedy action when appropriate; every displayed column identifies a value or count. |
| 2 | Pilot one complete probability-composition lesson: optional Rust diagnostic, worked example, partial completion, independent task, delayed changed-context task. | Record unaided correctness, explanation, assistance used, and the delayed result separately; use observed errors to choose the next exercise. |
| 3 | Extend that pilot with the distribution laws and the tiny RL integration task. | The learner can explain equivalent compositions and connect an observed transition to a value update, while preserving the declared probability and horizon contracts. |

The default first change is the small correctness repair in order 1. Avoid
expanding the curriculum substantially until the pilot provides evidence about
where this learner needs support. These priorities are engineering and teaching
judgment informed by the sources, not outcomes established by the studies.

## Historical delivery and verification (2026-10-08)

The preceding exercise request is implemented: ten starter functions, ten
separate reference solutions with reasoning, shared acceptance cases, a practice
guide, and updated navigation. No training loop or category-theory lesson is
claimed as delivered. The review findings above remain proposed changes.

The checks below are the current workspace equivalents of the checks run against
the expanded manifest at review time. The historical results following them
are not a new execution report:

```bash
cargo fmt --all -- --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
```

Results: 35 tests passed, including all ten solution tests; ten unfinished
practice tests were intentionally ignored. Explicitly selecting the unfinished
practice suite produced ten expected TODO failures. Selecting the former example-target filter `exercise_1_`
produced exactly one expected failure. Both former example programs ran successfully. Those exercises now live in
internal modules; use the [command migration map](maintenance/COMMAND_MIGRATION.md)
for current commands.
Formatting, Clippy, and `git diff --check` passed. These observations validate
the delivered artifacts, not learner mastery. No learner attempt was supplied.

## Evidence boundary

The review uses original papers, author-hosted copies, publisher records, and an original systematic review. It is selective, not a preregistered or exhaustive search. Source dates describe publication; the 2026 Python study collected its classroom data in 2023, and the Bastani study ran in the 2023-2024 school year. Recent publication does not make the underlying intervention current in every respect.

The repository has executable teaching material and reference tests. No delayed learner scores or completed learner attempts were evaluated for this report. Consequently, the evidence supports design choices to try and observations to collect, not a claim that these materials have already improved the user's learning.
