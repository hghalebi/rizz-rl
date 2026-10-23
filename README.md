# rizz-rl

**Learn reinforcement learning in Rust, one awkward state transition at a time.**

Your agent can ask a question, tell a story, suggest coffee, or say goodbye. Its social life is a finite state machine. Conveniently, that makes the math inspectable.

In this lab, 3,448 possible trajectories and a Bellman evaluator with 15 cached subproblems produce the same expected return to six decimal places. You can inspect how each gets there.

Start with one decision. Work toward probability, rewards, exact planning, and learning values from experience, with Rust ownership and composition along the way. There are **17 exercises**, separate solutions, and a learning path that gives you one next action at a time.

[Start learning](level-1-rl-expanded/guides/LEARNING_PATH.md) · [Commands and source map](level-1-rl-expanded/README.md) · [Contribute](CONTRIBUTING.md) · [MIT license](LICENSE)

## Your first experiment

You need Git and [Rust with Cargo](https://rust-lang.org/tools/install/). CI checks Rust **1.95.0**. You should be comfortable reading a function, an enum, and a loop; the lessons introduce the probability as you need it.

Before running, make one prediction. The toy world starts at `JustMet`. The policy chooses `Ask` with probability `0.70`. Given that action, the world responds with `Engaged` with probability `0.80`.

**What is the probability of the complete path `JustMet -> Ask -> Engaged`?**

```bash
git clone https://github.com/hghalebi/rizz-rl.git
cd rizz-rl
cargo run --locked -- first-step
```

The first build downloads dependencies. After that, the lessons run locally. No GPU, model API, or Hugging Face account is needed.

<details>
<summary>After your prediction: compare with the output</summary>

These four rows come from `first-step`:

```text
  56.0%  JustMet --Ask / +1.0--> Engaged
  14.0%  JustMet --Ask / -2.0--> LowInterest
  18.0%  JustMet --Story / +1.0--> Engaged
  12.0%  JustMet --Story / -2.0--> LowInterest
Total probability mass: 1.000000
```

The first branch needs both events: `0.70 * 0.80 = 0.56`. The second probability is conditional on asking; it is not an assumption that actions and responses are independent.

</details>

Change just one number on paper: if `Ask` becomes `0.50`, while its response probabilities stay the same, what happens to that path?

**Next:** follow [the first learning activity](level-1-rl-expanded/guides/LEARNING_PATH.md#session-1-follow-one-decision-15-minutes). It supplies the remaining probabilities and a place to stop.

The conversation is a fictional state machine. Its probabilities, rewards, and labels are invented teaching devices, not measurements or advice about people. Please keep your actual coffee invitations out of the test suite.

## What you will build an intuition for

| Question | Experiment | Idea to carry away |
| --- | --- | --- |
| How likely is a complete path? | `first-step` | Multiply conditional branch probabilities. |
| What is a sequence of rewards worth? | `returns` | Discount rewards by how far away they are. |
| Can we score every possible future? | `evaluate` | Weight each trajectory's return by its probability. |
| Can we reuse the same future calculation? | `compare` | Cache values by state **and steps remaining**. |
| What changes when the policy changes? | `sweep` | Compare eleven specified candidates in the same world. |
| Can observations teach us a value? | `td-trace`, `learn-value` | Update an estimate from reward and estimated continuation. |

Use the [guided route](level-1-rl-expanded/guides/LEARNING_PATH.md) in order. The [command reference](level-1-rl-expanded/README.md#run-one-concept) is there when you want to explore.

## The same value, two ways to compute it

At boldness `0.3`, discount `0.95`, and horizon `5`, the lab can enumerate **3,448 stored trajectories**. The Bellman evaluator instead reuses **15 cached subproblems**, excluding base cases. Both report an expected return of **3.103947**.

```bash
cargo run --locked --quiet -- compare \
  --boldness 0.3 --discount 0.95 --horizon 5
```

Selected columns from the output:

```text
Horizon    Tree E[G]    Bellman V_h
      0     0.000000       0.000000
      1     0.220000       0.220000
      2     1.109402       1.109402
      3     2.000322       2.000322
      4     2.669438       2.669438
      5     3.103947       3.103947
```

One method follows complete trajectories. The other combines the next reward with the value of a smaller future. They evaluate the same model and policy; the full output exposes floating-point differences. These counts describe this example, not a speed benchmark.

Bellman saves the program from rehearsing the same small talk again. The cache still needs both the state and the time remaining: one chance to say something and five chances are different problems.

Later, [`learn-value`](level-1-rl-expanded/guides/EXPERIENCE_LESSON.md) estimates finite-horizon values from sampled episodes using TD(0). Its policy stays fixed. The lab does not train neural networks or learn a better policy; it gives you inspectable foundations for those next steps.

## Learn by making a prediction, then changing the case

The practice loop is small:

```text
Predict -> run or implement -> check -> explain a changed case
```

- **10 arithmetic exercises** cover returns, expectations, probability checks, TD updates, Bellman backups, and related control building blocks.
- **7 composition exercises** connect ownership with weighted branches, `map`, `and_then`, and tests of composition laws. This is a concrete route into category-theory ideas.
- Solutions live in separate files. Hints and worked demonstrations are optional, and each demonstration can be run on its own.

The borrow checker is also invited. It would like to know who owns the `String`.

Start with [exercise 1: discounted return](level-1-rl-expanded/guides/EXERCISES.md#1-discounted-return). It asks you to implement one function, then run exactly one test. An untouched starter fails at `todo!()` when selected; ordinary tests keep those 17 starters ignored.

If your attention or time runs out, leave a [three-line restart note](level-1-rl-expanded/guides/LEARNING_RECORD.md#quick-restart). On returning, follow the file, function, and command in your note. The route provides short activities, optional detail, and explicit stopping points. You can stop midway through an exercise.

## Choose what to read next

| You want to... | Open |
| --- | --- |
| Learn from the beginning | [Learning path](level-1-rl-expanded/guides/LEARNING_PATH.md) |
| Find a command or source file | [Lab README](level-1-rl-expanded/README.md) |
| Practice the calculations | [Arithmetic exercises](level-1-rl-expanded/guides/EXERCISES.md) |
| Connect Rust ownership and composition | [Composition lesson](level-1-rl-expanded/guides/COMPOSITION_LESSON.md) |
| Understand learning from observations | [Experience lesson](level-1-rl-expanded/guides/EXPERIENCE_LESSON.md) |
| Look up a concept | [Deep RL companion](level-1-rl-expanded/guides/DEEP_RL_COMPANION.md) |

This is an independent Rust learning project that connects to the [Hugging Face Deep RL Course](https://huggingface.co/learn/deep-rl-course/en/unit0/introduction). Follow that course for its deep-learning libraries, environments, and agent-training projects. The [course connections](level-1-rl-expanded/guides/LEARNING_PATH.md#connect-this-lab-to-hugging-face) explain where this lab fits.

## Help the next learner

A useful contribution can be one sentence that finally makes a concept click, a test that catches a mistaken assumption, or an issue describing where you lost the thread. Asking a question here requires no `--boldness` flag.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for small starting tasks and the review checklist. [Report a confusing lesson or a bug](https://github.com/hghalebi/rizz-rl/issues/new/choose). Include what you expected, what happened, and the command or passage involved. If this helped you understand a concept, share that experiment with someone learning it too.

To check the project locally:

```bash
python3 scripts/check.py
```

The checker needs Python 3.9+, Rust, rustfmt, and Clippy. It runs formatting, strict linting, Rust tests, lesson commands, local document links, and exercise selectors. [CI runs the same checker](https://github.com/hghalebi/rizz-rl/actions/workflows/ci.yml). Passing software checks and being able to explain a new example are separate achievements.

Maintained by [Hamze Ghalebi](https://github.com/hghalebi). Code and lesson text are available under the [MIT license](LICENSE). Dependencies and linked learning resources retain their own licenses. Historical experiments are kept in [scratch](scratch/README.md), outside the default learning route.
