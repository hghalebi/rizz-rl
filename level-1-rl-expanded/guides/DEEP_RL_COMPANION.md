# Deep RL companion: reference for the Rust lab

This reference explains the ideas implemented by the [probability operations](../src/probability.rs), [world model](../src/model.rs), and [exact planner](../src/planning.rs), then builds a bridge from that small exact planner toward the major families of reinforcement-learning techniques. Start with the shorter [learning path](LEARNING_PATH.md); use this file as a reference.

The separate `learn-value` command now learns fixed-policy values from sampled
transitions. Follow the [experience lesson](EXPERIENCE_LESSON.md) and
[`src/experience.rs`](../src/experience.rs) for that implemented TD(0) path.

The program itself remains intentionally small. The explanations are separated from the source so the code stays readable.

---

# How to read this companion

Section labels distinguish implemented ideas, derivations, and future extensions.

**IMPLEMENTED** means the idea exists directly in the Rust program.

**DERIVED** means the idea follows mathematically from what the program does, even if it is not named in the source.

**NEXT TECHNIQUE** means a complete implementation of that technique remains future work. An isolated target exercise does not by itself implement a training loop. These sections show which limitation motivates the technique.

This distinction matters:

```text
understand the current mechanism
          │
          v
observe its limitation
          │
          v
introduce the technique that solves that limitation
```

Do not try to memorize every acronym. Learn the causal ladder.

---

# 0. The complete mental model

The exact planning commands answer one question:

> If an agent follows a policy with a particular boldness, what discounted reward should it expect over the next five interaction steps?

The data flow is:

```text
Current state s
     │
     │ policy π(a | s)
     v
Distribution<Action>
     │
     │ transition model P(s' | s, a)
     v
Distribution<NextState>
     │
     │ reward R(s, a, s')
     v
Distribution<Transition>
     │
     │ repeat for horizon H
     v
Distribution<Trajectory>
     │
     │ discounted return G(τ)
     v
Distribution<TrajectoryScore>
     │
     │ expectation E[G]
     v
One expected-return score J(policy)
```

The policy search then does:

```text
boldness 0.0 ──evaluate──▶ score
boldness 0.1 ──evaluate──▶ score
boldness 0.2 ──evaluate──▶ score
...
boldness 1.0 ──evaluate──▶ score
                              │
                              v
                         keep maximum
```

## Precise classification

**IMPLEMENTED**

The exact planning path is:

```text
finite-horizon
+ tabular
+ stochastic-policy
+ model-based
+ exact-enumeration
+ policy-evaluation
+ grid-search optimization
```

The separate `learn-value` command implements finite-horizon, tabular TD(0)
evaluation of a fixed policy from sampled simulator transitions. Its learner
does not read the model's probabilities or exact values; those values are
computed afterward for comparison.

The following complete training methods remain future work:

```text
Monte Carlo estimation
Q-learning
policy-gradient training
actor-critic training
```

That distinction will become clearer throughout the guide.

---

# Part I — Probability machinery

# 1. `Distribution<T>` makes uncertainty explicit

**IMPLEMENTED**

A normal value says:

```text
next_state = Playful
```

A distribution says:

```text
possible next states:

Playful      with probability 0.55
Engaged      with probability 0.25
LowInterest  with probability 0.20
```

The type:

```rust
struct Distribution<T> {
    outcomes: Vec<(T, f64)>,
}
```

means:

```text
possible values of T
+
the weight/probability of every represented branch
```

Examples:

```text
Distribution<Action>
Distribution<State>
Distribution<Transition>
Distribution<Trajectory>
```

## Outcome versus event

A useful probability distinction:

```text
outcome = one represented branch
event   = a condition that may include several branches
```

For example, after the first interaction there are four trajectory branches:

```text
Ask   → Engaged
Ask   → LowInterest
Story → Engaged
Story → LowInterest
```

But the event:

```text
next state is Engaged
```

contains two branches:

```text
Ask   → Engaged
Story → Engaged
```

That is why `probability_of(&State::Engaged)` must sum all matching branches rather than taking only one.

---

# 2. Probability mass is a conserved quantity

**IMPLEMENTED**

A normalized probability distribution has total mass approximately equal to `1.0`.

```text
Σ probabilities = 1
```

Example:

```text
0.55 + 0.25 + 0.20 = 1.00
```

When the tree branches, probability moves into more leaves, but total mass should not change.

```text
before:
one root @ 1.0

after:
many leaves whose probabilities sum to 1.0
```

This creates an invariant:

> Branching may redistribute probability mass, but must not create or destroy it.

The program checks:

```rust
possible_trajectories.total_probability()
```

and warns if the result differs materially from `1.0`.

## Why only approximately?

The code uses `f64`.

Floating-point numbers cannot represent every decimal exactly. A mathematically exact total of `1.0` may become:

```text
0.9999999999999998
```

or:

```text
1.0000000000000002
```

So the code uses a tolerance:

```text
|mass - 1.0| ≤ 1e-9
```

This is a numerical-engineering issue, not an RL issue, but it affects RL implementations everywhere.

---

# 3. `certain`: deterministic behavior inside an uncertain pipeline

**IMPLEMENTED**

Some transitions have only one possible result:

```text
LowInterest + Space → Goodbye
```

Yet the surrounding system expects a `Distribution<State>`.

The program represents certainty as:

```text
Distribution<State>
└── Goodbye @ 1.0
```

The constructor is:

```rust
Distribution::certain(State::Goodbye)
```

Conceptually:

```text
T
│
│ place inside probability context
v
Distribution<T>
```

Functional-programming vocabulary:

```text
certain
≈ pure
≈ return
≈ lift a value into the context
```

The human meaning matters first:

> There is still a distribution, but it has exactly one branch and that branch is certain.

---

# 4. `map`: transform meaning without changing likelihood

**IMPLEMENTED**

Suppose:

```text
Distribution<State>

Engaged      @ 0.80
LowInterest  @ 0.20
```

A function turns one state into one transition:

```text
State ─────▶ Transition
```

`map` applies that function to every branch:

```text
Distribution<State>
        │ map
        v
Distribution<Transition>
```

The values change:

```text
State → Transition
```

The probabilities stay attached to their branches:

```text
Engaged      @ 0.80 → Transition A @ 0.80
LowInterest  @ 0.20 → Transition B @ 0.20
```

The invariant is:

```text
branch probability before map
=
branch probability after map
```

## The shape does not change

`map` preserves the number of branches.

```text
2 states
  │ map
  v
2 transitions
```

It changes labels on leaves, not the structure of the tree.

Memory hook:

> `map` changes what each leaf means; it does not create more uncertainty.

---

# 5. `and_then`: compose two uncertain steps

**IMPLEMENTED**

The policy is uncertain:

```text
JustMet
├── Ask   @ 0.70
└── Story @ 0.30
```

The world is also uncertain:

```text
Ask
├── Engaged     @ 0.80
└── LowInterest @ 0.20
```

and:

```text
Story
├── Engaged     @ 0.60
└── LowInterest @ 0.40
```

`and_then` continues every action branch into every conditional world branch.

```text
JustMet
│
├── Ask @ 0.70
│   ├── Engaged     @ 0.80
│   └── LowInterest @ 0.20
│
└── Story @ 0.30
    ├── Engaged     @ 0.60
    └── LowInterest @ 0.40
```

The standard names are:

```text
and_then
≈ flat_map
≈ bind
```

The human meaning is:

> For every possible current outcome, continue with another probabilistic computation.

---

# 6. Conditional probability becomes complete-path probability

**IMPLEMENTED + DERIVED**

The world numbers are conditional probabilities.

```text
P(Engaged | JustMet, Ask) = 0.80
```

Read:

> Given that the current state is `JustMet` and the chosen action is `Ask`, the probability of `Engaged` is `0.80`.

The policy gives:

```text
P(Ask | JustMet) = 0.70
```

To calculate the complete branch:

```text
JustMet → Ask → Engaged
```

multiply:

```text
P(Ask and Engaged | JustMet)
=
P(Ask | JustMet)
×
P(Engaged | JustMet, Ask)
```

Numerically:

```text
0.70 × 0.80 = 0.56
```

This is the probability chain rule in its simplest operational form.

The code exposes it as:

```rust
let path_probability =
    probability_so_far * next_probability;
```

## Why multiplication, intuitively?

The first branch keeps `70%` of the mass.

The second branch keeps `80%` of that remaining mass.

```text
100%
 │ keep 70%
 v
70%
 │ keep 80% of 70%
 v
56%
```

---

# 7. Law of total probability: merge different paths to the same event

**DERIVED**

After one step:

```text
Ask   → Engaged  @ 0.56
Story → Engaged  @ 0.18
```

Both branches end in `Engaged`.

Therefore:

```text
P(Engaged after one step)
=
0.56 + 0.18
=
0.74
```

Similarly:

```text
P(LowInterest after one step)
=
0.14 + 0.12
=
0.26
```

Check:

```text
0.74 + 0.26 = 1.00
```

This is the law of total probability:

\[
P(s') = \sum_a P(a \mid s)P(s' \mid s,a)
\]

The program’s `probability_of` helper performs this kind of aggregation when several branches contain equal values.

## Important distinction

The trajectory distribution keeps branches separate:

```text
Ask → Engaged
Story → Engaged
```

A state distribution merges them conceptually:

```text
Engaged @ 0.74
```

Keeping full trajectories preserves history.

Merging by state enables dynamic programming later.

That tradeoff is central.

---

# 8. Expected value: collapse uncertainty into one comparison number

**IMPLEMENTED**

Suppose:

```text
Outcome A: probability 0.60, value +10
Outcome B: probability 0.40, value  -5
```

Expected value:

```text
0.60 × 10
+
0.40 × (-5)
=
4
```

Formal notation:

\[
E[X] = \sum_x P(x)X(x)
\]

The program implements:

```rust
weighted_sum += probability * value_of(outcome);
```

## Expected value is not a guaranteed outcome

An expectation of `4` does not mean the observed result will be exactly `4`.

It means:

> Across many repetitions under the same model, the average should approach `4`.

In the exact planner, expectation is also the rational comparison target chosen by the reward objective.

## Risk is hidden by a mean

These policies have the same expectation:

```text
A: 100% chance of +4

B: 50% chance of +100
   50% chance of -92
```

Operationally, they are very different.

Expected return is useful, but incomplete.

Later risk-sensitive techniques may also measure:

```text
variance
worst case
catastrophic-loss probability
quantiles
CVaR
hard safety constraints
```

---

# Part II — The reinforcement-learning model implemented by the code

# 9. The MDP tuple

**IMPLEMENTED + DERIVED**

A finite Markov decision process is often written:

\[
M = (S, A, P, R, \gamma)
\]

For this program:

```text
S = set of States
A = set of Actions
P = transition probabilities in DatingWorld
R = reward_for
γ = discount factor
```

The program also has:

```text
H  = finite horizon
s₀ = initial state JustMet
π  = policy parameterized by boldness
```

A fuller problem description is therefore:

```text
(S, A, P, R, γ, H, initial state, policy family)
```

The equation is just a compressed inventory of the system’s parts.

---

# 10. State: the information used to make a decision

**IMPLEMENTED**

States are:

```text
JustMet
Engaged
Playful
LowInterest
MutualInterest
Goodbye
```

A state is not necessarily the entire real world.

It is:

> The representation the agent and model treat as sufficient for the next decision.

This distinction is fundamental.

```text
complex reality
      │ state abstraction
      v
small decision-relevant representation
```

Good state design keeps relevant information while discarding irrelevant detail.

Bad state design removes information needed to predict outcomes or choose well.

---

# 11. The Markov property

**DERIVED**

The model uses:

```text
P(next_state | current_state, action)
```

It does not inspect the complete trajectory when predicting the next state.

This encodes the Markov assumption:

> Once the current state is known, older history provides no additional information needed for the next-step distribution.

Formally:

\[
P(s_{t+1} \mid s_t,a_t,s_{t-1},a_{t-1},\ldots)
=
P(s_{t+1} \mid s_t,a_t)
\]

## Why this assumption matters

It permits reuse:

```text
Every path arriving at Engaged
can share the same future-value calculation.
```

Without the Markov property, two identical visible states reached through different histories could have different futures.

## State aliasing

Suppose both interactions are labeled `Engaged`, but one has high trust and the other has tension.

```text
history A ─▶ Engaged
history B ─▶ Engaged
```

If their futures differ, the state representation has merged unlike situations.

This is called state aliasing.

The cure may be to enrich state:

```text
Engaged { trust: High }
Engaged { trust: Low }
```

or include memory/history.

---

# 12. Partial observability

**NEXT TECHNIQUE**

In real systems, the agent often cannot see the true state.

It sees observations:

```text
hidden world state
       │ emits
       v
observation
       │ used by
       v
agent
```

A partially observable Markov decision process, or POMDP, distinguishes:

```text
hidden state
observation
belief about hidden state
```

Common engineering responses include:

```text
explicit memory
history windows
recurrent neural networks
belief-state estimation
retrieval from previous interactions
sensor fusion
```

The current program intentionally ignores this complexity.

The lesson is:

> Before choosing an RL algorithm, ask whether your “state” is actually observable and sufficient.

---

# 13. Action: what the agent controls

**IMPLEMENTED**

Actions are:

```text
Ask
Story
Tease
Coffee
Space
Goodbye
```

The agent controls action selection.

It does not control the consequence.

```text
decision
   ≠
outcome
```

A good decision may get unlucky.

A poor decision may get lucky.

Therefore one observed transition is weak evidence about policy quality.

The evaluator considers the whole outcome distribution.

---

# 14. Legal actions and action masking

**IMPLEMENTED + PRODUCTION EXTENSION**

The policy only assigns probabilities to sensible actions for each state.

For example:

```text
LowInterest
├── Space
└── Goodbye
```

This is a form of action restriction.

In larger systems, action masking explicitly blocks illegal actions:

```text
all actions
   │ apply legality mask
   v
allowed actions
   │ renormalize
   v
policy distribution
```

Examples:

```text
cannot withdraw more money than balance
cannot deploy before approval
cannot send email without recipient
cannot transition from Deleted back to Active
```

The current world has a fallback that leaves the state unchanged for unsupported actions. That is pedagogically convenient but can hide bugs.

A production implementation should prefer:

```text
Result<Distribution<State>, IllegalAction>
```

or types that make illegal state/action combinations unrepresentable.

---

# 15. Policy: a rule for behavior

**IMPLEMENTED**

A policy maps state to action distribution:

\[
\pi(a \mid s)
\]

Read:

> Probability of choosing action `a` while in state `s`.

For `JustMet`:

```text
π(Ask | JustMet)   = 0.70
π(Story | JustMet) = 0.30
```

## Deterministic policy

```text
one state → one action with probability 1
```

## Stochastic policy

```text
one state → several actions with nonzero probability
```

The code uses a stochastic policy.

## Important nuance: stochasticity is not automatically exploration

The exact evaluation commands enumerate every policy branch.

The `learn-value` command instead samples actions from the same fixed policy to
learn its values. It does not change that policy to discover better actions.

Therefore a stochastic policy alone does not establish an exploration strategy
for policy improvement.

Exploration requires uncertainty about what is best and interaction to reduce that uncertainty.

---

# 16. Policy parameterization

**IMPLEMENTED**

The policy family is controlled by:

```rust
struct Policy {
    boldness: f64,
}
```

A parameterized policy is written:

\[
\pi_\theta(a \mid s)
\]

Here:

```text
θ = boldness
```

The parameter is converted into action probabilities.

```text
boldness
    │
    v
probability formulas
    │
    v
complete behavior policy
```

Modern policy networks use millions of parameters, but the conceptual structure is the same.

---

# 17. Linear and nonlinear parameter effects

**IMPLEMENTED**

In `Engaged`:

```text
tease = 0.15 + 0.35 × boldness
coffee = 0.30 × boldness²
```

The tease probability changes linearly.

```text
equal boldness increments
→ equal tease-probability increments
```

Coffee changes quadratically.

```text
low boldness: slow growth
high boldness: faster growth
```

This creates a curved behavior-to-performance relationship.

A parameter can have nonlinear downstream effects because it influences:

```text
action probability
→ next-state probability
→ later action availability
→ reward sequence
→ expected return
```

---

# 18. Transition model: the environment’s behavior

**IMPLEMENTED**

The world model is:

\[
P(s' \mid s,a)
\]

It answers:

> Given state `s` and action `a`, how likely is each next state `s'`?

For example:

```text
Engaged + Tease
├── Playful     @ 0.55
├── Engaged     @ 0.25
└── LowInterest @ 0.20
```

The policy and world are separate boundaries:

```text
AGENT                            ENVIRONMENT

state ──π(a|s)──▶ action ──P(s'|s,a)──▶ next state
```

This separation supports clean debugging:

```text
bad action choice?
→ inspect policy

bad consequence prediction?
→ inspect world model

wrong objective?
→ inspect reward
```

---

# 19. Known model versus learned model

**IMPLEMENTED + NEXT TECHNIQUE**

The transition probabilities are manually written.

Therefore the model is known.

In many real systems, the transition model must be estimated from data:

```text
observed transitions
(s, a, s')
       │ estimation
       v
learned P̂(s' | s, a)
```

The hat means “estimated.”

A learned model introduces model error.

```text
true world P
     ≠
learned model P̂
```

When planning many steps ahead, small errors can compound.

```text
one-step prediction error
      │ repeated rollout
      v
large long-horizon error
```

This is a major challenge in model-based RL.

---

# 20. Reward: the objective actually optimized

**IMPLEMENTED**

The reward function is:

\[
R(s,a,s')
\]

It maps one transition to a number.

Examples:

```text
Engaged --Tease--> Playful        +3
Any state -------> MutualInterest +8
Engaged --Coffee--> LowInterest   -8
Any state -------> LowInterest    -2
```

The algorithm cannot infer unstated human values.

It optimizes the written signal.

```text
human intent
    │ encode
    v
reward function
    │ optimize
    v
behavior
```

Misalignment can happen at the encoding boundary.

---

# 21. Sparse reward and reward shaping

**IMPLEMENTED + DERIVED**

A sparse reward gives useful feedback rarely.

Example:

```text
all intermediate steps = 0
final success          = +8
```

The current program also rewards intermediate progress:

```text
Ask/Story → Engaged or Playful = +1
Tease → Playful                = +3
respect LowInterest            = +2
```

This is reward shaping.

It can make optimization easier by improving the learning signal.

But it may change the optimal behavior if designed carelessly.

## Reward hacking

An agent may maximize a proxy without satisfying intent.

```text
reward +1 for asking
      │
      v
ask forever
```

Production control requires more than one scalar reward:

```text
objective
+ hard constraints
+ safety policy
+ adversarial evaluation
+ observability
+ human review
```

---

# 22. Immediate reward versus long-term objective

**IMPLEMENTED**

An action can have:

```text
small reward now
large benefit later
```

or:

```text
large reward now
large cost later
```

RL exists largely because decisions affect future states and future options.

```text
action now
   │
   v
next state
   │ changes
   v
future action distribution
   │ changes
   v
future rewards
```

A greedy immediate-reward policy may be globally poor.

The return solves this by scoring the trajectory, not only the current step.

---

# 23. Terminal and absorbing states

**IMPLEMENTED**

`Goodbye` is terminal.

```text
Goodbye
   │
   └── no more meaningful interaction
```

The outer loop still has a fixed horizon, so terminal paths are carried forward unchanged.

Operationally:

```text
terminal path
→ same path
→ no new transition
→ no new reward
```

An absorbing state is one that cannot be left after entry.

`Goodbye` behaves like an absorbing endpoint.

## Why `MutualInterest` is not terminal

The policy selects `Goodbye` from `MutualInterest`, and the world then moves to `Goodbye`.

So:

```text
MutualInterest
      │ Goodbye
      v
Goodbye
```

This is a modeling choice.

Making `MutualInterest` terminal would change trajectory lengths and potentially returns.

---

# 24. Initial state and initial-state distribution

**IMPLEMENTED + NEXT EXTENSION**

The program always begins from:

```text
JustMet
```

This is a deterministic initial state.

A general RL problem may begin from an initial-state distribution:

\[
s_0 \sim \rho_0
\]

For example:

```text
70% JustMet
20% Engaged
10% LowInterest
```

Then policy value must average over starting states too.

The program could support this naturally by replacing `Distribution::certain(JustMet)` with a distribution over initial states.

---

# Part III — How the planner works

# 25. Transition

**IMPLEMENTED**

A transition records one local causal step:

```text
from_state --action / reward--> to_state
```

Rust:

```rust
struct Transition {
    from_state: State,
    action: Action,
    reward: f64,
    to_state: State,
}
```

It contains enough information to answer:

```text
Where were we?
What did the agent do?
What happened?
How was it scored?
```

---

# 26. Trajectory and episode

**IMPLEMENTED**

A trajectory is an ordered sequence of transitions:

```text
JustMet
   │ Ask / +1
   v
Engaged
   │ Tease / +3
   v
Playful
   │ Coffee / +8
   v
MutualInterest
```

Formal shorthand:

\[
\tau = (s_0,a_0,r_0,s_1,a_1,r_1,\ldots)
\]

An episode is usually a trajectory that begins at an initial condition and ends at a terminal state or time limit.

This program has finite-horizon paths, some of which terminate early.

---

# 27. Trajectory probability

**DERIVED**

A complete trajectory probability is the product of policy and transition probabilities at every step.

\[
P_\pi(\tau)
=
\prod_{t=0}^{T-1}
\pi(a_t \mid s_t)
P(s_{t+1} \mid s_t,a_t)
\]

Human translation:

```text
probability of full path
=
product of every required action branch
and every required environment branch
```

The program never writes this large product explicitly.

`and_then` builds it incrementally:

```text
probability so far × next branch probability
```

This is an important pattern:

> Local multiplication creates a globally correct path probability.

---

# 28. `ConversationPath`: state plus history

**IMPLEMENTED**

During expansion, each branch needs:

```text
current state
+
trajectory so far
```

That is:

```rust
struct ConversationPath {
    current_state: State,
    trajectory_so_far: Trajectory,
}
```

Why both?

The current state determines the next policy action distribution.

The trajectory stores rewards needed to calculate return later.

---

# 29. Branch independence and cloning

**IMPLEMENTED**

When one branch splits, every child needs its own history.

```text
shared past
   │
   ├── future A
   └── future B
```

If both children modified the same mutable vector, their histories would interfere.

The code clones the trajectory before appending each transition.

Invariant:

```text
changes inside one future branch
must not mutate another future branch
```

## Scalability consequence

Many trajectories share long prefixes, so cloning duplicates memory.

```text
same prefix copied thousands of times
```

This is acceptable for a tiny tutorial.

More scalable representations include:

```text
parent pointers
persistent vectors
shared immutable prefixes
dynamic programming
value functions
sampling
```

---

# 30. Rollout

**IMPLEMENTED**

A rollout means following a policy through an environment.

Two meanings are common.

## Sampled rollout

```text
sample one action
sample one next state
repeat
```

Produces one trajectory.

## Exact rollout tree

```text
enumerate every action
enumerate every next state
repeat
```

Produces all represented trajectories.

The program uses exact enumeration.

No random number generator appears.

---

# 31. Horizon

**IMPLEMENTED**

The horizon `H` is the number of future steps considered.

```text
H = 1 → immediate step only
H = 5 → five-step planning window
```

The horizon controls:

```text
how far consequences are visible
how many branches are generated
how expensive evaluation becomes
```

## Short-horizon failure

A short-horizon agent may prefer an action with immediate reward but delayed harm.

```text
reward now
   │ visible
   v
choose action

harm at H+1
   │ invisible
   v
ignored
```

This is planning myopia.

---

# 32. Combinatorial explosion

**IMPLEMENTED + DERIVED**

The number of stored paths grows:

```text
step 0:    1
step 1:    4
step 2:   26
step 3:  136
step 4:  688
step 5: 3448
```

The tree grows while mass remains `1.0`.

```text
number of representations ↑ rapidly
probability mass           = constant
```

If average branching factor is `b`, naive tree size grows roughly as:

\[
O(b^H)
\]

This exponential cost motivates many advanced techniques.

---

# 33. Discounted return

**IMPLEMENTED**

One reward scores one transition.

Return scores the whole trajectory.

\[
G(\tau)
=
r_0 + \gamma r_1 + \gamma^2 r_2 + \cdots
\]

Example:

```text
rewards: 1, 3, 8
γ = 0.95

G = 1 + 0.95×3 + 0.95²×8
  ≈ 11.07
```

The discount factor is:

```text
0 ≤ γ ≤ 1
```

## Interpretations of discounting

It can encode:

```text
preference for earlier reward
uncertainty in distant forecasts
finite effective planning horizon
mathematical convergence in continuing tasks
```

In this finite-horizon program, convergence is not the main reason.

Time preference is the clearest interpretation.

---

# 34. Effective horizon

**DERIVED**

Even without a hard horizon, discounting reduces the influence of distant rewards.

A rough effective horizon is on the order of:

\[
\frac{1}{1-\gamma}
\]

For `γ = 0.95`:

```text
1 / (1 - 0.95) = 20
```

This is only a rule of thumb, not an exact cutoff.

The program also has a hard horizon of `5`, so rewards beyond step five are completely absent regardless of discount.

```text
hard horizon H
= exact visibility boundary

discount γ
= gradual weighting inside that boundary
```

---

# 35. Policy evaluation

**IMPLEMENTED**

Policy evaluation asks:

> How good is this fixed policy under this fixed environment model and objective?

The program computes:

\[
J(\pi)
=
E_{\tau \sim \pi,P}[G(\tau)]
\]

Human translation:

```text
generate every trajectory induced by policy and world
calculate each discounted return
weight each return by trajectory probability
sum
```

The result is exact up to floating-point rounding because all branches are enumerated.

---

# 36. A complete one-step calculation

**DERIVED FROM THE PROGRAM**

From `JustMet`:

```text
P(Ask)   = 0.70
P(Story) = 0.30
```

World branches:

```text
P(Engaged | Ask)       = 0.80
P(LowInterest | Ask)   = 0.20

P(Engaged | Story)     = 0.60
P(LowInterest | Story) = 0.40
```

Joint path probabilities:

```text
Ask   → Engaged      0.70 × 0.80 = 0.56
Ask   → LowInterest  0.70 × 0.20 = 0.14
Story → Engaged      0.30 × 0.60 = 0.18
Story → LowInterest  0.30 × 0.40 = 0.12
```

One-step rewards:

```text
Ask   → Engaged      +1
Ask   → LowInterest  -2
Story → Engaged      +1
Story → LowInterest  -2
```

Expected one-step reward:

```text
0.56×(+1)
+ 0.14×(-2)
+ 0.18×(+1)
+ 0.12×(-2)

= 0.56 - 0.28 + 0.18 - 0.24
= 0.22
```

This first-step score is independent of boldness because the `JustMet` policy does not use the boldness parameter.

Boldness starts affecting later decisions after the interaction reaches `Engaged` or `Playful`.

This is a useful causal insight:

```text
parameter may not affect immediate action
but can affect later branches
and therefore total return
```

---

# 37. Objective as a function of the policy parameter

**DERIVED**

Let boldness be `b`.

The evaluated objective is:

\[
J(b) = E[G \mid \pi_b]
\]

The program samples this objective at eleven grid points:

```text
b ∈ {0.0, 0.1, ..., 1.0}
```

Approximate values:

```text
b     J(b)
0.0   2.791938
0.1   2.957257
0.2   3.061052
0.3   3.103947
0.4   3.088344
0.5   3.018149
0.6   2.898467
0.7   2.735256
0.8   2.534981
0.9   2.304278
1.0   2.049661
```

The best grid point is `0.3`.

## Why not maximum boldness?

Higher boldness increases opportunities for `Coffee`, but `Coffee` from `Engaged` has:

```text
15% MutualInterest
85% LowInterest
```

and failed `Coffee` receives a strong negative reward.

The expected cost eventually dominates the expected benefit.

This illustrates a general optimization pattern:

```text
increasing parameter
→ initially improves opportunity
→ later increases risk faster
→ interior optimum appears
```

---

# 38. Grid search

**IMPLEMENTED**

Grid search evaluates a finite set of candidate parameter values.

Strengths:

```text
simple
deterministic
easy to inspect
hard to implement incorrectly
```

Weaknesses:

```text
resolution limited by grid spacing
cost grows exponentially with parameter dimension
may miss a better value between grid points
```

One parameter with eleven values is easy.

Ten parameters with eleven values each would require:

```text
11¹⁰ ≈ 25.9 billion candidates
```

That is why high-dimensional policies need other optimization techniques.

---

# 39. Policy evaluation versus policy improvement

**IMPLEMENTED + DERIVED**

Policy evaluation:

```text
given policy π
→ calculate how good π is
```

Policy improvement:

```text
given policy scores or values
→ choose a better policy
```

The `sweep` command evaluates each candidate, then chooses the best grid point.

This is a primitive form of policy improvement.

It does not derive a new action choice state by state; it selects among a predefined policy family.

---

# Part IV — The technique ladder

The following techniques are not all implemented. Each one exists because a simpler approach hits a limitation.

# 40. Dynamic programming

**IMPLEMENTED: finite-horizon policy evaluation**

[`values::evaluate_value`](../src/values.rs) now evaluates the same policy by
memoizing `(steps_remaining, state)`. Run `cargo run -- compare` from the
repository root to compare its answer with full trajectory
enumeration. This computes state values using a known model; it does not learn
values from experience or implement action-value control.

## Problem it solves

The tree repeatedly recomputes futures from the same state.

```text
many histories
    │
    └── arrive at Engaged
             │
             └── identical future calculation
```

Dynamic programming stores and reuses state values.

## State-value function

\[
V_h^\pi(s)
=
E_\pi[G_t^{(h)} \mid S_t=s]
\]

Human meaning:

> Expected discounted return over at most `h` remaining steps, starting from state `s` and following policy `π`.

Here `G_t^{(h)}` stops after `h` rewards or at termination, whichever comes first.

## Action-value function

\[
Q_h^\pi(s,a)
=
E_\pi[G_t^{(h)} \mid S_t=s,A_t=a]
\]

Human meaning:

> Expected return if action `a` is taken now in state `s`, then policy `π` is followed for the remaining `h - 1` steps.

## Bellman expectation equation

\[
V_h^\pi(s)
=
\sum_a \pi(a \mid s)
\sum_{s'}P(s' \mid s,a)
\left[
R(s,a,s') + \gamma V_{h-1}^\pi(s')
\right]
\]

For `h >= 1`, this equation compresses the future tree.

```text
value of state
=
average over actions
    of
    average over next states
        of
        immediate reward + discounted future value
```

The full trajectory tree gives this equation its meaning.

For the finite-horizon program, time is part of the value:

```text
V(0, s) = 0
V(h, terminal) = 0
V(h, s) = sum over transitions p * [reward + gamma * V(h - 1, next)]
```

The first reward has weight 1. The horizon boundary assigns zero value to
rewards outside this particular planning objective. It does not declare the
environment terminal. If a sampled continuing task is merely cut off by a
time limit, TD learning can still bootstrap from its next-state value.

---

# 41. Policy iteration

**NEXT TECHNIQUE**

Policy iteration alternates:

```text
1. Policy evaluation
   calculate Vπ

2. Policy improvement
   choose actions that look better under Vπ

3. Repeat until stable
```

Diagram:

```text
policy π₀
   │ evaluate
   v
value Vπ₀
   │ improve
   v
policy π₁
   │ evaluate
   v
...
```

It assumes a known model and usually a manageable tabular state/action space.

For this lab's finite horizon, improve decisions separately for each `(h, s)`:
`pi_new(h, s) = argmax_a Q_h(s, a)`. The best action can depend on time remaining;
[Lab 14](INTERACTIVE_LABS.md#lab-14-policy-improvement-depends-on-the-remaining-horizon)
contains a numeric counterexample.

---

# 42. Value iteration

**NEXT TECHNIQUE**

Value iteration combines evaluation and improvement.

Bellman optimality equation:

This stationary notation is for a continuing task. A finite-horizon version
uses `V_h` on the left and `V_(h-1)` for the continuation value on the right.

\[
V^*(s)
=
\max_a
\sum_{s'}P(s' \mid s,a)
\left[
R(s,a,s')+\gamma V^*(s')
\right]
\]

Human meaning:

> The optimal value of a state equals the best action’s expected immediate reward plus discounted optimal future value.

Unlike the current parameterized policy search, value iteration can choose the best action separately in every state.

---

# 43. Monte Carlo evaluation

**NEXT TECHNIQUE**

## Problem it solves

Exact enumeration becomes too large or the environment is only available as a simulator/real interaction.

Monte Carlo samples trajectories:

```text
run episode 1 → return G₁
run episode 2 → return G₂
...
average returns
```

Estimate:

\[
V^\pi(s)
\approx
\frac{1}{N}\sum_{i=1}^N G_i
\]

## Properties

```text
does not require transition probabilities
uses complete sampled returns
unbiased under standard conditions
can have high variance
often waits until episode end
rare events may be missed
```

Comparison:

```text
Exact enumeration:
all branches, no sampling noise, expensive

Monte Carlo:
some branches sampled, noisy estimate, scalable
```

---

# 44. Temporal-difference learning

**IMPLEMENTED: finite-horizon evaluation of a fixed policy**

Run `cargo run -- td-trace` or `cargo run -- learn-value` from
the repository root. The [experience lesson](EXPERIENCE_LESSON.md) explains the
observations, per-entry visit counts, and exact comparison. The implementation
in [`src/experience.rs`](../src/experience.rs) keys values by `(h, state)` and
updates online using learning rate `1 / visits` for each entry.

## Problem it solves

Monte Carlo waits for the full return.

Temporal-difference learning updates from one transition.

TD(0) target with `h` steps remaining:

\[
r_t + \gamma V_{h-1}(s_{t+1})
\]

TD error:

\[
\delta_t
=
r_t + \gamma V_{h-1}(s_{t+1}) - V_h(s_t)
\]

Update:

\[
V_h(s_t)
\leftarrow
V_h(s_t) + \alpha \delta_t
\]

Human meaning:

```text
old estimate
+
learning rate × surprise
```

where surprise is:

```text
observed reward
+ estimated future value
- old estimate
```

## Bootstrapping

TD uses an estimate to update another estimate.

```text
V_h(s_t) updated using V_(h-1)(s_{t+1})
```

This is called bootstrapping.

Terminal and zero-horizon continuation values are fixed at zero; the arriving
reward still counts. This is the finite-horizon objective used by both learner
and exact evaluator. An external observation cutoff in a continuing task has
different semantics and need not disable bootstrapping.

Tradeoff:

```text
Monte Carlo:
real complete return, high variance, no bootstrap

TD:
partial target with estimate, lower variance, some bias
```

---

# 45. N-step returns and TD(λ)

**NEXT TECHNIQUE**

One-step TD looks ahead one reward.

Monte Carlo looks to the end.

N-step methods live between them:

\[
G_t^{(n)}
=
r_t + \gamma r_{t+1} + \cdots
+ \gamma^{n-1}r_{t+n-1}
+ \gamma^n V(s_{t+n})
\]

TD(λ) mixes multiple n-step returns.

The conceptual spectrum:

```text
TD(0)
short bootstrap
low delay

n-step
middle ground

Monte Carlo
full return
no bootstrap
```

---

# 46. SARSA

**NEXT TECHNIQUE**

SARSA learns action values from transitions:

```text
State
Action
Reward
next State
next Action
```

That sequence gives the name.

Update target:

\[
r + \gamma Q(s',a')
\]

SARSA is on-policy:

> It learns the value of the behavior policy actually being followed.

If the behavior explores cautiously, the learned values include the consequences of that exploration.

---

# 47. Q-learning

**NEXT TECHNIQUE**

Q-learning target:

\[
r + \gamma \max_{a'}Q(s',a')
\]

It is off-policy:

> It may behave with an exploratory policy while learning the value of a greedy target policy.

Comparison:

```text
SARSA target:
value of action actually selected next

Q-learning target:
value of best estimated next action
```

The exact planning commands can evaluate known-model expectations directly.
The implemented sampled learner estimates state values for a fixed policy;
SARSA and Q-learning control loops remain future work. The separate Q-learning
target exercise is only one building block of such a loop.

Q-learning becomes useful when:

```text
model unknown
interaction samples available
state/action table manageable
```

---

# 48. Expected SARSA

**NEXT TECHNIQUE**

Expected SARSA replaces a sampled next action with the policy’s expected next action value:

\[
r
+
\gamma
\sum_{a'}\pi(a' \mid s')Q(s',a')
\]

This is conceptually close to the exact planner's expectation over policy branches.

Difference:

```text
exact planner:
known transition model + exact tree

Expected SARSA:
sampled transition + expected next action value
```

---

# 49. On-policy versus off-policy

**NEXT TECHNIQUE**

## On-policy

Learn about the same policy used to collect data.

```text
behavior policy = target policy
```

Examples:

```text
SARSA
many policy-gradient methods
```

## Off-policy

Collect data with one policy and learn about another.

```text
behavior policy ≠ target policy
```

Examples:

```text
Q-learning
offline RL
importance-sampling methods
```

Why it matters:

```text
off-policy methods can reuse old or external data
but distribution mismatch can destabilize learning
```

The current exact planner has no data-collection policy because it enumerates the known model directly.

---

# 50. Exploration versus exploitation

**NEXT TECHNIQUE**

When the environment is unknown:

```text
exploit
→ choose action believed best

explore
→ choose action to learn more
```

The tension:

```text
always exploit
→ may remain confidently wrong

always explore
→ learns but wastes reward
```

Common exploration strategies:

```text
ε-greedy
softmax/Boltzmann action selection
upper-confidence bounds
Thompson sampling
entropy bonuses
intrinsic motivation
```

Important:

> A stochastic policy is not automatically performing useful exploration.

Exploration requires the stochasticity to reduce uncertainty or improve future decisions.

---

# 51. Contextual bandit

**NEXT TECHNIQUE**

A contextual bandit has:

```text
context/state
→ choose action
→ receive immediate reward
→ episode effectively ends
```

No long-term state transition matters.

If the dating example cared only about immediate reaction, it could be a contextual bandit.

It becomes RL because:

```text
actions change future state
and future opportunities
```

Bandit:

```text
action → reward
```

RL:

```text
action → next state → future actions → future rewards
```

---

# 52. Policy gradient

**NEXT TECHNIQUE**

## Problem it solves

Grid search does not scale to high-dimensional parameters.

Policy-gradient methods directly optimize:

\[
J(\theta)
=
E_{\tau \sim \pi_\theta}[G(\tau)]
\]

REINFORCE uses:

\[
\nabla_\theta J(\theta)
=
E[
G(\tau)
\nabla_\theta
\log \pi_\theta(a_t \mid s_t)
]
\]

Intuition:

```text
actions followed by high return
→ increase their log-probability

actions followed by low return
→ decrease their log-probability
```

The logarithm turns products of trajectory probabilities into sums and gives a tractable gradient estimator.

## Difficulty

Policy gradients can have high variance.

One lucky or unlucky trajectory may strongly affect the update.

This motivates baselines and critics.

---

# 53. Credit assignment

**NEXT TECHNIQUE**

A delayed reward creates a question:

> Which earlier actions deserve credit or blame?

Example:

```text
Ask
→ Story
→ Tease
→ Coffee
→ success +8
```

Was success caused by Coffee only?

Or did earlier actions create the necessary state?

This is temporal credit assignment.

Discounting, returns, value functions, TD errors, eligibility traces, and advantage estimates are all tools for distributing credit across time.

---

# 54. Actor-critic

**NEXT TECHNIQUE**

Actor-critic separates:

```text
Actor:
policy πθ(a | s)
chooses behavior

Critic:
value Vφ(s) or Qφ(s,a)
estimates quality
```

The critic provides a lower-variance learning signal to the actor.

Diagram:

```text
state
  │
  ├── actor ──▶ action
  │
  └── critic ─▶ value estimate

observed transition
  │
  v
TD error / advantage
  │
  ├── improve critic
  └── improve actor
```

The current code already separates policy from evaluator conceptually, making actor-critic easier to understand later.

---

# 55. Advantage function

**NEXT TECHNIQUE**

The advantage compares an action with the state’s normal baseline.

\[
A^\pi(s,a)
=
Q^\pi(s,a)-V^\pi(s)
\]

Human meaning:

> How much better or worse was this action than what the policy usually achieves from this state?

Why useful:

```text
raw return includes difficulty of the state
advantage isolates relative action quality
```

Actor-critic methods commonly use advantage estimates to reduce variance.

---

# 56. Entropy regularization

**NEXT TECHNIQUE**

A policy can collapse too early to one action.

Entropy measures spread:

\[
H(\pi(\cdot \mid s))
=
-\sum_a \pi(a \mid s)\log \pi(a \mid s)
\]

High entropy:

```text
probability spread across actions
```

Low entropy:

```text
nearly deterministic
```

An entropy bonus can encourage exploration or prevent premature certainty.

But too much entropy keeps the policy indecisive.

---

# 57. Model predictive control

**NEXT TECHNIQUE**

Model predictive control repeatedly:

```text
1. plan H steps ahead
2. execute only the first action
3. observe new state
4. replan
```

The exact planning commands evaluate futures from `JustMet` without executing actions.

An online agent could re-evaluate after every observation.

```text
plan
→ act once
→ observe
→ update state
→ plan again
```

This limits damage from model error because the plan is continuously corrected.

---

# 58. Monte Carlo tree search

**NEXT TECHNIQUE**

Exact enumeration explores every branch equally.

Monte Carlo tree search allocates more computation to promising branches.

Core cycle:

```text
selection
→ expansion
→ simulation
→ backup
```

It is useful when:

```text
tree enormous
generative simulator available
planning needed at decision time
```

It trades complete coverage for focused search.

---

# 59. Dyna

**NEXT TECHNIQUE**

Dyna combines:

```text
real experience
+
learned world model
+
simulated planning experience
```

Loop:

```text
act in real world
→ update value/policy
→ update model
→ generate imagined transitions
→ update value/policy again
```

The exact planning commands use a manually known model. The `learn-value`
command learns values from simulator observations, but does not learn a model
or add extra planning updates from one.

A Dyna version would learn `DatingWorld` probabilities from observed transitions and use that learned model for extra planning updates.

---

# 60. Offline RL

**NEXT TECHNIQUE**

Offline RL learns from a fixed dataset without further environment interaction.

```text
logged transitions
(s,a,r,s')
      │
      v
learn policy
```

Challenge:

The learned policy may choose actions poorly represented in the dataset.

This is distribution shift.

```text
dataset behavior distribution
             ≠
new policy distribution
```

Overestimating unseen actions is a major offline-RL failure mode.

---

# 61. Function approximation

**NEXT TECHNIQUE**

The current model is tabular:

```text
small finite list of states and actions
```

Large problems use functions:

```text
Vφ(s)
Qφ(s,a)
πθ(a|s)
Pψ(s'|s,a)
```

implemented by:

```text
linear models
decision trees
neural networks
transformers
```

Function approximation generalizes across similar states but introduces:

```text
approximation error
optimization instability
distribution shift
catastrophic forgetting
```

The “deadly triad” warning in RL combines:

```text
function approximation
+ bootstrapping
+ off-policy learning
```

This combination can diverge without care.

---

# 62. Online versus offline learning

**IMPLEMENTED: online value updates in a simulator; offline learning remains future work**

Online:

```text
agent acts
→ receives new data
→ updates after each observed transition
```

Offline:

```text
fixed historical dataset
→ train without new interaction
```

Online can adapt but creates safety and exploration risks.

Offline is operationally safer but constrained by dataset coverage.

The `learn-value` command performs online TD updates from new simulator
observations. The exact planning commands perform no learning updates, and
`td-trace` is a supplied arithmetic demonstration rather than offline RL training.

---

# Part V — Production and failure modes

# 63. Invalid probability values

**PRODUCTION EXTENSION**

`f64` can represent:

```text
negative values
values above 1
NaN
infinity
```

A production probability type should enforce:

```text
finite
0 ≤ p ≤ 1
```

A distribution constructor should enforce:

```text
all probabilities valid
total mass ≈ 1
```

The current code checks only mass after trajectory enumeration.

---

# 64. Parameter invariants

**IMPLEMENTED PARTIALLY**

Boldness is intended to satisfy:

```text
0 ≤ boldness ≤ 1
```

The expanded source uses a debug assertion.

A production design should use a validated constructor:

```text
Policy::try_new(boldness)
```

returning:

```text
Result<Policy, InvalidBoldness>
```

This turns a comment-level assumption into a program-enforced invariant.

---

# 65. Unsupported actions

**PRODUCTION EXTENSION**

The current fallback leaves the state unchanged.

This can hide mistakes.

Example:

```text
policy accidentally emits Coffee from Goodbye
world silently keeps Goodbye
```

A strict system should surface:

```text
IllegalAction {
    state,
    action
}
```

or prevent the pair at the type level.

Failing loudly at the boundary improves safety and debugging.

---

# 66. Reward-rule reachability

**DERIVED**

The reward table includes:

```text
LowInterest + Coffee → -12
```

The current policy never chooses Coffee from LowInterest.

This rule is therefore unreachable under the current policy family.

It may still be useful defensively if:

```text
policy changes
external action supplied
off-policy data evaluated
```

This distinction matters:

```text
world/reward model may define more behavior
than current policy can produce
```

Tests should cover both reachable and defensive rules intentionally.

---

# 67. Model error and compounding error

**PRODUCTION EXTENSION**

If transition probabilities are estimated incorrectly, the planner optimizes the wrong predicted world.

```text
incorrect model
→ incorrect future tree
→ incorrect expected return
→ incorrect policy choice
```

Longer horizons amplify error.

Mitigations:

```text
uncertainty estimates
short replanning horizon
real-world feedback
model ensembles
robust optimization
conservative policies
out-of-distribution detection
```

---

# 68. Reward misspecification

**PRODUCTION EXTENSION**

Even a perfect optimizer produces poor behavior if the reward is wrong.

Debugging order:

```text
1. Is the state representation sufficient?
2. Are legal actions correct?
3. Is the transition model calibrated?
4. Does reward encode real intent?
5. Is the evaluator measuring risk adequately?
6. Only then blame optimization.
```

This avoids treating every bad behavior as an “RL algorithm problem.”

---

# 69. Distribution shift

**PRODUCTION EXTENSION**

The model may be accurate for one population and wrong for another.

```text
training/model context
          ≠
deployment context
```

Policy optimization can also push behavior into regions where the model has little evidence.

```text
optimizer finds unusual high-score region
→ model uncertainty high
→ prediction unreliable
```

This is an optimizer’s curse.

Monitoring should include model confidence and coverage, not only expected return.

---

# 70. Risk-sensitive evaluation

**PRODUCTION EXTENSION**

Expected return alone may permit catastrophic tails.

Add metrics such as:

```text
P(severe loss)
minimum return
5th percentile return
variance
CVaR
constraint violation rate
```

A constrained objective might be:

```text
maximize expected return

subject to:

P(reward ≤ -8) ≤ 1%
```

This is closer to how production systems should treat safety-critical behavior.

---

# 71. Observability

**IMPLEMENTED + PRODUCTION EXTENSION**

The code traces:

```text
policy parameter
action probabilities
path count
probability mass
expected return
```

These events expose causal structure and invariants.

Production telemetry may also include:

```text
state occupancy
action frequency
terminal rate
reward components
model uncertainty
severe-loss probability
policy version
environment version
random seed
data provenance
```

Observability should not change the policy’s decisions.

---

# 72. Testing as executable understanding

**IMPLEMENTED IN EXPANDED SOURCE**

The tests prove conceptual invariants:

```text
certainty has mass 1
map preserves mass
and_then multiplies branches correctly
every policy distribution is normalized
first-step probabilities match hand calculation
best candidate remains boldness 0.3
```

A good pedagogical test answers:

> What conceptual promise would be broken if this assertion failed?

Tests are not only code protection. They are formalized explanations.

---

# 73. Reproducibility

**PRODUCTION EXTENSION**

Exact enumeration is deterministic.

Sampling-based RL additionally needs:

```text
random seeds
environment versions
policy checkpoints
dependency versions
dataset snapshots
evaluation protocols
confidence intervals
```

Without those, a reported improvement may be noise.

---

# Part VI — The shortest technique map

Use this table as orientation, not memorization.

| Technique | Main problem solved | Needs known model? | Learns from samples? | Relation to this program |
|---|---|---:|---:|---|
| Exact tree enumeration | Tiny planning problem | Yes | No | Implemented |
| Dynamic programming | Repeated state subproblems | Yes | No | Implemented finite-horizon evaluator |
| Monte Carlo | Model unavailable or tree too large | No | Yes | Samples trajectories |
| TD learning | Learn before episode ends | No | Yes | Implemented fixed-policy finite-horizon evaluator |
| SARSA | On-policy action-value learning | No | Yes | Learns actual behavior policy |
| Q-learning | Off-policy greedy action values | No | Yes | Learns target policy from exploratory data |
| Policy gradient | High-dimensional stochastic policy | No | Yes | Replaces grid search with gradients |
| Actor-critic | Reduce policy-gradient variance | No | Yes | Policy plus learned evaluator |
| MCTS | Huge planning tree | Generative model | Uses simulations | Selective tree expansion |
| Dyna | Combine learning and planning | Learns model | Yes | Learned version of `DatingWorld` |
| Offline RL | Learn from fixed logs | No | Fixed dataset | Must handle coverage shift |
| MPC | Replan after each real observation | Yes | Not necessarily | Executes only first planned action |

---

# Part VII — Active recall

Answer without looking back.

1. Why is `Distribution<State>` richer than `State`?
2. What invariant does total probability mass express?
3. Why does `map` preserve branch probability?
4. Why does `and_then` multiply probabilities?
5. How do two different paths to `Engaged` combine?
6. What does `π(a | s)` mean in plain English?
7. What does `P(s' | s,a)` mean?
8. Which boundary belongs to the agent, and which belongs to the world?
9. Why is stochastic policy not the same as exploration?
10. What assumption makes `Engaged` sufficient without the full trajectory?
11. What is state aliasing?
12. Why does reward design become a specification problem?
13. What is the difference between reward and return?
14. What is the difference between horizon and discount?
15. How is a trajectory probability constructed?
16. Why does exact enumeration become expensive?
17. What does `V_hπ(s)` summarize, and why does `h` matter?
18. What does Bellman decomposition reuse?
19. How does Monte Carlo differ from exact enumeration?
20. What does TD bootstrap?
21. What distinguishes SARSA from Q-learning?
22. What does on-policy versus off-policy mean?
23. Why can grid search handle one parameter but not millions?
24. What does a critic provide to an actor?
25. Why can expected return hide unacceptable risk?
26. What distinguishes the exact planning commands from the sampled `learn-value` command?
27. What would have to become unknown for Q-learning to make sense?
28. What would a Dyna version learn?
29. Why can a longer model rollout amplify model error?
30. Which invariant would you encode next in a production Rust type?

---

# Final compression

```text
Policy:
State ─────▶ Distribution<Action>

World:
(State, Action) ─────▶ Distribution<NextState>

Reward:
(State, Action, NextState) ─────▶ Number

and_then:
compose uncertain steps by multiplying path probabilities

Rollout:
grow the complete finite future tree

Return:
sum one trajectory’s discounted rewards

Expectation:
weight every trajectory return by its probability

Policy evaluation:
calculate J(π)

Grid search:
compare candidate π_b values

Next scalability step:
replace repeated tree expansion with values, sampling, or gradients
```

The one-sentence memory hook:

> The exact planner branches over choices and consequences, multiplies path probabilities, accumulates rewards, and averages the future tree into one policy score. The sampled learner instead updates value estimates from observed transitions.

The one diagram to remember:

```text
State
  │ π(a|s)
  v
Action
  │ P(s'|s,a)
  v
Next state + reward
  │ repeat
  v
Trajectory τ
  │ G(τ)
  v
Trajectory return
  │ Pπ(τ) × G(τ)
  v
Expected return J(π)
```
