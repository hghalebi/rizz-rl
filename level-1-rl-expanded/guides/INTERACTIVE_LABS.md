# Optional extensions for the Rust RL lab

These optional extensions explore changes to the reference lab. They are a task bank, not the required learning route.

Start with the six short sessions in [LEARNING_PATH.md](LEARNING_PATH.md).
These longer labs remain optional extensions. All commands in this file run
from the repository root. Several labs ask you to implement new behavior;
their proposed features are not all present in the reference program.

The experiment controls now live in the CLI, so Labs 5 and 6 no longer require
editing source. For example, run `cargo run -- sweep --horizon 2 --discount 0.5`.
Use `cargo run -- --help` for supported commands and bounds. Lab 12 has a worked
implementation in [`src/values.rs`](../src/values.rs); reconstruct it yourself
before reading that file if you want an independent exercise.

Use the [source map](../README.md#source-map) to find the relevant module.
Record attempts and later retrieval in the [learning record](LEARNING_RECORD.md).
Use this loop for every experiment:

```text
1. Predict
2. Change one thing
3. Run
4. Observe
5. Explain the causal chain
6. Restore or commit
```

Do not skip prediction. Prediction is where learning happens.

Commands:

```bash
cargo test
cargo run
```

---

# Lab 0 — Prove the baseline before changing it

Expected baseline:

```text
horizon             = 5
discount            = 0.95
number of paths     = 3448
probability mass    ≈ 1.0
best boldness       = 0.3
expected return     ≈ 3.103947
```

Run:

```bash
cargo test
cargo run -- sweep
cargo run -- evaluate --boldness 0.3 --horizon 5
```

The sweep reports the best grid candidate and score; evaluation reports its
trajectory count and probability mass.

Mental model:

```text
baseline proof
      │
      v
safe experiment
      │
      v
causal comparison
```

Without a baseline, a changed output is merely different—not understood.

---

# Lab 1 — Calculate the first step manually

Before running, calculate:

```text
P(Ask → Engaged)
P(Ask → LowInterest)
P(Story → Engaged)
P(Story → LowInterest)
```

Use:

```text
P(complete branch)
=
P(action | state)
×
P(next state | state, action)
```

Expected:

```text
0.56
0.14
0.18
0.12
```

Then calculate state probabilities:

```text
P(Engaged)     = 0.56 + 0.18 = 0.74
P(LowInterest) = 0.14 + 0.12 = 0.26
```

Finally calculate expected immediate reward:

```text
0.56×1 + 0.14×(-2) + 0.18×1 + 0.12×(-2)
= 0.22
```

Concepts:

```text
conditional probability
joint probability
law of total probability
expected value
```

---

# Lab 2 — Make probability mass fail intentionally

Change one policy probability:

```rust
State::JustMet => Distribution {
    outcomes: vec![
        (Action::Ask, 0.70),
        (Action::Story, 0.40),
    ],
},
```

Now total action mass is:

```text
1.10
```

Predict:

```text
What will first-step transition mass become?
Will later mass remain wrong?
Will path count change?
```

Run the program and tests.

Then restore `0.30`.

Lesson:

> Probability normalization is not formatting. It is a structural invariant that propagates through every future layer.

---

# Lab 3 — Understand `map`

Inside the test `map_changes_values_but_preserves_probability_mass`, change:

```rust
numbers.map(|number| number * 2)
```

to:

```rust
numbers.map(|number| number * number)
```

Predict:

```text
Which fields change?
Which fields stay exactly the same?
Does path count change?
```

Core invariant:

```text
map changes values
map preserves tree shape
map preserves probabilities
```

---

# Lab 4 — Understand `and_then`

Use this tiny tree:

```text
A @ 0.25
B @ 0.75
```

Let both branch again:

```text
A → X @ 0.40
A → Y @ 0.60

B → X @ 0.10
B → Y @ 0.90
```

Calculate all complete paths:

```text
A→X
A→Y
B→X
B→Y
```

Then calculate:

```text
P(X)
P(Y)
```

Only after hand calculation, encode it with `Distribution::and_then`.

Lesson:

```text
and_then preserves history as separate paths
probability_of can later merge equal outcomes
```

---

# Lab 5 — Horizon sensitivity

Run the same sweep with each horizon from 1 through 5:

```bash
cargo run -- sweep --horizon 1
cargo run -- sweep --horizon 2
cargo run -- sweep --horizon 3
cargo run -- sweep --horizon 4
cargo run -- sweep --horizon 5
```

Use `evaluate` to obtain trajectory counts, matching the horizon and boldness
reported by each sweep. For example, the horizon-one sweep ties and selects
boldness zero:

```bash
cargo run -- evaluate --horizon 1 --boldness 0
```

Record:

| Horizon | Number of trajectories | Best boldness | Expected return |
|---:|---:|---:|---:|
| 1 | | | |
| 2 | | | |
| 3 | | | |
| 4 | | | |
| 5 | 3448 | 0.3 | 3.103947 |

Before running, predict:

```text
At horizon 1, can boldness matter?
```

Answer:

Boldness does not affect the `JustMet` action distribution, so at horizon 1 every candidate should have the same expected return.

This is a powerful causal check.

```text
parameter influences only later states
+
short horizon cannot see later states
=
parameter appears irrelevant
```

Concept:

```text
planning horizon
delayed effects
myopia
```

---

# Lab 6 — Discount sensitivity

Try:

```bash
cargo run -- sweep --discount 0.0
cargo run -- sweep --discount 0.5
cargo run -- sweep --discount 0.95
cargo run -- sweep --discount 1.0
```

Record best boldness and return.

Predict before running:

```text
γ = 0.0
→ only first reward matters
→ boldness should not matter

γ = 1.0
→ later rewards receive full weight
→ measure how the policy ranking changes; sensitivity need not increase
```

Distinguish:

```text
horizon
= which rewards exist in evaluation

discount
= how strongly visible future rewards count
```

---

# Lab 7 — Reward sensitivity

Change failed Coffee from:

```rust
(State::Engaged, Action::Coffee, State::LowInterest) => -8.0,
```

to:

```rust
-3.0
```

Predict:

```text
Will the optimal boldness move upward or downward?
Why?
```

Then try:

```text
-15.0
```

The causal chain:

```text
Coffee downside changes
→ expected value of bold action changes
→ J(boldness) curve changes
→ optimum moves
```

This is reward sensitivity analysis.

In production, a policy that changes dramatically after a small reward change is fragile.

---

# Lab 8 — Terminal-state semantics

Current terminal state:

```rust
matches!(self, Self::Goodbye)
```

Change it temporarily to:

```rust
matches!(self, Self::MutualInterest | Self::Goodbye)
```

Predict:

```text
Will path count fall?
Will successful trajectories become shorter?
Will expected return change?
Why might the change be zero or nonzero?
```

This lab exposes:

```text
terminal-state definition
reward timing
episode length
```

A terminal state is a modeling choice, not merely a Rust enum variant.

---

# Lab 9 — Measure severe-loss probability

Expected return hides tail risk.

Add:

```rust
fn contains_severe_loss(
    trajectory: &Trajectory,
) -> bool {
    trajectory
        .iter()
        .any(|transition| transition.reward <= -8.0)
}
```

Then inside policy evaluation calculate:

```rust
let severe_loss_probability: f64 =
    possible_trajectories
        .outcomes
        .iter()
        .filter(|(trajectory, _)| {
            contains_severe_loss(trajectory)
        })
        .map(|(_, probability)| probability)
        .sum();
```

Print it for every policy.

Create a table:

| Boldness | Expected return | P(severe loss) |
|---:|---:|---:|
| 0.0 | | |
| 0.1 | | |
| ... | | |

Question:

> Would you still choose boldness `0.3` if severe-loss probability had to remain below a fixed threshold?

Concepts:

```text
risk-sensitive control
chance constraint
tail risk
multi-metric evaluation
```

---

# Lab 10 — Measure variance

For each trajectory return `G`, calculate:

\[
Var(G) = E[G^2] - E[G]^2
\]

Rust shape:

```rust
let expected_return =
    expected_value(&trajectories, |trajectory| {
        discounted_return(trajectory, discount)
    });

let expected_squared_return =
    expected_value(&trajectories, |trajectory| {
        let value =
            discounted_return(trajectory, discount);

        value * value
    });

let variance =
    expected_squared_return
        - expected_return * expected_return;
```

Now compare policies with similar means but different variance.

Concept:

```text
mean tells central performance
variance tells spread
```

---

# Lab 11 — Replace exact enumeration with Monte Carlo

Goal:

```text
estimate expected return by sampling
instead of enumerating all branches
```

You need a function that samples one outcome from a distribution.

Conceptual Rust signature:

```rust
fn sample<T: Clone>(
    distribution: &Distribution<T>,
    random_unit: f64,
) -> T
```

where:

```text
0 ≤ random_unit < 1
```

Then:

```text
sample action
sample next state
append transition
repeat horizon
```

Run many episodes:

```text
N = 10
N = 100
N = 10_000
```

Compare the sample mean with exact expected return.

Observe:

```text
small N
→ noisy estimate

large N
→ estimate approaches exact value
```

Add multiple seeds and confidence intervals.

Concepts:

```text
Monte Carlo estimation
sampling error
law of large numbers
variance
reproducibility
```

---

# Lab 12 — Build a state-value function

Instead of storing every trajectory, define:

```text
V(step, state)
=
expected future discounted return
from this state with this many steps remaining
```

Recursive idea:

```text
V(0, state) = 0

V(h, state)
=
Σa π(a|state)
Σs' P(s'|state,a)
[
    R(state,a,s')
    + γ V(h-1,s')
]
```

Implement memoization keyed by:

```text
(steps_remaining, state)
```

This is finite-horizon dynamic programming.

Compare:

```text
exact trajectory enumeration result
versus
memoized value result
```

They should match.

Then compare work performed.

Concepts:

```text
Bellman expectation equation
overlapping subproblems
memoization
state-value function
```

---

# Lab 13 — Build `Q(s, a)`

Define:

```text
Q(h, s, a)
=
expected return after taking action a now,
then following the policy
```

Equation:

```text
Q(h,s,a)
=
Σs' P(s'|s,a)
[
    R(s,a,s') + γV(h-1,s')
]
```

Then:

```text
V(h,s)
=
Σa π(a|s)Q(h,s,a)
```

Print Q-values for actions in `Engaged`.

Question:

> At each horizon, which action has the highest Q-value, and why can the stochastic policy still assign probability to others?

Concepts:

```text
state value
action value
policy averaging
greedy improvement
```

---

# Lab 14: Policy improvement depends on the remaining horizon

Using the finite-horizon Q-values from Lab 13, create a deterministic greedy policy:

```text
π_new(h,s) = argmax_a Q(h,s,a)
```

Here `h` is the number of transitions remaining. The greedy decision may differ
at the same state when there is less time left. Decrease `h` after every step;
store decisions by `(h, state)` and use a consistent tie rule.

Before implementing the policy, calculate this separate deterministic example
with discount `0.5`:

| Action now | Rewards | Return with h=1 | Return with h=2 |
| --- | --- | ---: | ---: |
| TakeNow | 2, then terminate | 2 | 2 |
| Wait | 0 now, 5 next, then terminate | 0 | 0 + 0.5 * 5 = 2.5 |

With one step left, TakeNow wins. With two steps left, Wait wins. A single
decision stored only under the state cannot represent both answers. This is a
counterexample to dropping `h`, not a claim that every state changes its best
action at every horizon. It uses invented reward traces, not `DatingWorld`.

Run its numeric check from the repository root:

```bash
cargo test values::tests::the_greedy_action_can_depend_on_remaining_horizon -- --exact
```

Then evaluate your improved policy using the same remaining-horizon convention.

Compare with the one-parameter boldness family.

Question:

> Can a policy that chooses separately for each `(h, state)` outperform every policy expressible by one global boldness number?

It has more freedom than the one-parameter family. Measure the return rather
than assuming a strict improvement: the best returns can also tie.

Concept:

```text
policy-class limitation
expressiveness
policy improvement
```

---

# Lab 15 — Learn the transition model from synthetic experience

Hide the real probabilities from the agent.

Generate sampled transitions using `DatingWorld`, then count:

```text
count(s,a,s')
count(s,a)
```

Estimate:

```text
P_hat(s'|s,a)
=
count(s,a,s') / count(s,a)
```

Compare learned estimates after:

```text
10 samples
100 samples
10_000 samples
```

Then plan with the learned model.

Observe:

```text
few samples
→ inaccurate model
→ inaccurate policy evaluation

more samples
→ better model
→ better planning
```

Concepts:

```text
model learning
maximum-likelihood estimate
epistemic uncertainty
compounding model error
```

---

# Lab 16 — TD(0) value learning

Remove access to exact future probabilities.

Maintain:

```text
V(State)
```

For each observed transition:

```text
TD target =
reward + γV(next_state)

TD error =
target - V(state)

V(state) += α × TD error
```

Run repeated episodes.

For a true terminal next state, the target is just the reward. A time limit
in a continuing task is different: retain the next-state bootstrap.

To compare against this project's finite-horizon dynamic-programming values,
learn `V(steps_remaining, State)` and bootstrap from
`V(steps_remaining - 1, next_state)`, with zero value at zero remaining steps.
A single stationary `V(State)` is a different objective and should not be
expected to match every horizon in `cargo run -- compare`.

Concepts:

```text
bootstrapping
learning rate α
online updates
bias/variance tradeoff
```

---

# Lab 17 — SARSA versus Q-learning

Maintain:

```text
Q(State, Action)
```

SARSA target:

```text
r + γQ(next_state, next_action)
```

Q-learning target:

```text
r + γ max_action Q(next_state, action)
```

Use an ε-greedy behavior policy.

For both methods, set the bootstrap term to zero on true termination. Match
the time convention from Lab 16 if comparing to finite-horizon values.

Compare learned policies.

Ask:

```text
Which method learns the behavior policy’s risk?
Which method learns toward a greedy target policy?
```

Concepts:

```text
on-policy
off-policy
exploration
action-value control
```

---

# Lab 18 — Policy gradient intuition

Keep boldness as the policy parameter.

Instead of grid search:

```text
sample trajectories
estimate whether increasing boldness
made high-return actions more likely
update boldness gradually
```

Do not rush to full calculus.

First log:

```text
chosen action
probability under current boldness
trajectory return
```

Then study the score-function idea:

```text
high return
×
gradient of log action probability
```

Concepts:

```text
objective J(θ)
stochastic gradient
credit assignment
variance
baseline
```

---

# Lab 19 — Actor-critic intuition

Split responsibilities:

```text
Actor:
Policy { boldness }

Critic:
estimated V(State)
```

The critic estimates whether an observed result was better or worse than expected.

```text
advantage
≈ actual result - expected baseline
```

Use that signal to update the actor.

Concepts:

```text
actor
critic
TD error
advantage
variance reduction
```

---

# Lab 20 — Receding-horizon planning

Instead of evaluating once from `JustMet`:

```text
1. evaluate from current observed state
2. choose/execute first action
3. observe actual next state
4. evaluate again
```

This is a simplified model-predictive-control loop.

Concepts:

```text
plan
act
observe
replan
```

It is useful when the model is imperfect or the world changes.

---

# Final learning ladder

```text
Exact probability tree
        │ tree too large
        v
Dynamic programming or sampling
        │ model unknown
        v
Monte Carlo / TD / Q-learning
        │ policy high-dimensional
        v
Policy gradient / actor-critic
        │ model can be learned
        v
Dyna / model-based RL
        │ partial observability
        v
memory / belief state / recurrent policy
        │ safety and tail risk matter
        v
constrained and risk-sensitive RL
```

The one rule for every lab:

> Change one causal variable at a time, predict the consequence, and demand evidence before claiming understanding.
