# Structure review and implementation record

Reviewed and implemented: **2026-10-09**. Status: **implemented and locally verified**. This record covers the structural
cleanup of the existing teaching work. Learner explanation, independent transfer,
and delayed retention remain unverified until actual attempts are recorded.

## Outcome and boundaries

The maintained lab is now the root Cargo default. Both exercise tracks share
one command convention, the learning path owns progression, and Rust modules
follow the concepts they implement. The original scratch program remains
separately runnable. No model, reward, policy, dependency version, package name,
or edition was changed as part of this restructuring.

Existing teaching work and IDE settings were preserved. Only IDE paths affected
by the moves were adjusted. Validation was local; no push or hosted CI run was
performed for this implementation.

## Findings and delivered changes

The findings below describe the state before this restructuring.

| Prior problem | Delivered change | Evidence boundary |
| --- | --- | --- |
| Root `cargo test` selected the scratch package and ran zero tests. | [Root workspace](../../../Cargo.toml) defaults to the maintained lab, with scratch excluded. | Cargo metadata resolves the lab as the sole workspace/default member. |
| Arithmetic exercises were example targets; composition used binary unit tests. | Both now live under [exercises](../../src/exercises/mod.rs), with separate practice and solution files and shared acceptance cases. | All 17 old tasks map to corresponding exact selectors. |
| Ordinary lab tests omitted arithmetic reference tests. | Both reference tracks compile in the binary test target. | Default test inventory includes the original tests and one added checked arithmetic demonstration. |
| `main.rs` mixed concepts across 928 lines; a crate-wide allowance hid dead code. | A 21-line entry point wires probability, model, planning, CLI, values, and experience modules. | No blanket dead-code allowance remains; test-only helpers use `cfg(test)`. |
| Several documents competed to define the learning route and working directory. | [LEARNING_PATH](../LEARNING_PATH.md) owns progression, and commands use the repository root. | Local link and exact-selector checks validate navigation mechanically; actual ease of use needs learner evidence. |
| The two exercise tracks used different feedback conventions. | Cards identify objective, prerequisites, prediction, attempt, conceptual hint, explanation, changed case, solution, and next step. | Answers remain separate; [LEARNING_RECORD](../LEARNING_RECORD.md) owns the assessment template. |

## Current source map

```text
rizz-rl/
  Cargo.toml                   # virtual workspace, resolver 2
  Cargo.lock                   # existing lab resolution, moved unchanged
  README.md                    # orientation and Start here
  scripts/check.py             # standard Cargo checks plus navigation checks
  .github/workflows/ci.yml      # runs the same checks on pushes and pull requests
  scratch/original/            # preserved standalone package, edition 2024
  level-1-rl-expanded/
    Cargo.toml                 # unchanged lab package and edition 2021
    src/
      main.rs                  # entry point and module wiring
      cli.rs                   # parsing, tracing, presentation
      probability.rs           # Distribution, map, and_then, expectation
      model.rs                 # states, actions, policy, world, transitions
      planning.rs              # trajectories, returns, policy-grid search
      values.rs                # exact finite-horizon Bellman evaluator
      experience.rs            # sampling and fixed-policy TD(0)
      tests.rs                 # original core tests
      exercises/
        arithmetic/            # mod, practice, solutions, cases, worked
        composition/           # mod, practice, solutions, cases, worked
    tests/                     # CLI behavior and ownership compiler diagnostic
    guides/                    # route, exercises, lessons, optional references
```

Only the exercise tree and core test module are conditional on `cfg(test)`.
The binary keeps a small crate-internal API; no public library was introduced.
The arithmetic demonstration now runs as a checked worked test. Historical
reviews keep their existing paths, clearly labeled as historical evidence.
The companion remains an optional reference, with its anchors preserved.

## Decisions and alternatives

A root workspace with the lab as default removes the accidental scratch target
while retaining the lab's existing paths. Resolver `2` preserves its edition-2021
resolution behavior. The scratch package retains its separate lock and edition.
See the official [Cargo workspace reference](https://doc.rust-lang.org/cargo/reference/workspaces.html).

Keeping both packages in place and adding only aliases would retain the root
command trap. Keeping scratch as a workspace member would include its historical
warnings in workspace-wide checks. Moving the whole lab to the root would change
nearly every lesson path without improving its algorithms. The selected layout
addresses the concrete ambiguity with fewer teaching-path changes.

Test-only exercise modules let both tracks use internal code, including the
actual `Distribution`, without exporting an API solely for exercises. Cargo's
[target reference](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
describes this unit-test access. Test filtering isolates execution, not
compilation: a syntax or type error anywhere in the target can still block a
selected task. Untouched TODO bodies remain safe until explicitly selected.

## Commands and task migration

Run commands from the repository root. Use [EXERCISES](../EXERCISES.md) for all
ten arithmetic task names and [COMPOSITION_LESSON](../COMPOSITION_LESSON.md) for
all seven composition task names.

The [command migration guide](COMMAND_MIGRATION.md) owns the old-to-new path
and selector mapping. Task numbers and names remain unchanged.

A representative attempt and its separately selected answer:

```bash
cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
cargo test exercises::arithmetic::solutions::exercise_1_discounted_return -- --exact
cargo test exercises::composition::practice::map_laws -- --exact --ignored
cargo test exercises::composition::solutions::map_laws -- --exact
```

Run all automated checks with `python3 scripts/check.py`. It invokes the normal
Rust test harness, checks local Markdown links and anchors, runs documented CLI commands, verifies matching
starter/solution names, runs each starter with an exact selector, and checks
documented exact selectors against the real inventory. An untouched starter's
TODO is expected and reported separately. A completed attempt must pass its
shared cases. Other failures, including zero selected tests, fail the check.

The [CI workflow](../../../.github/workflows/ci.yml) uses the same command and
Rust 1.95.0, matching the local validation toolchain. Checkout is pinned to a
verified v7 commit and disables persisted credentials; its supported inputs are
in the official [checkout documentation](https://github.com/actions/checkout).
The workflow was checked locally with `actionlint`; hosted execution is pending.

## Verification record

Before editing, the current source, five preservation hashes, all 83 test names,
and 15 CLI outputs were captured outside the repository. The CLI cases cover
first-step, returns at discount 0.5, evaluate, compare, sweep, td-trace, and
learn-value at 100, 1,000, and 10,000 episodes with seeds 7, 42, and 2026.

- All 15 post-migration CLI outputs match the captured output byte for byte.
- All 83 original test names survive under the documented module mapping.
  The only added case is the arithmetic worked demonstration, which checks the
  values printed by the former example runner.
- The existing numerical tests retain the default exact return
  `3.103947335269`, best grid boldness `0.3`, and 3,448 trajectories, together with
  terminal rewards, horizon-zero behavior, visit counts, and the E0382 check.
- The original scratch source, manifest, and lockfile match the captured bytes;
  the maintained manifest is unchanged and its lock moved unchanged to the root.
- Plain root `cargo test --locked` and all-target checks both passed: **67 passed,
  17 intentionally ignored starters**. Formatting and warning-denied Clippy pass.
- `python3 scripts/check.py` passed **162 local links/anchors**, **34 documented
  lesson commands**, **28 documented exact test commands**, and all **17**
  starter/solution pairs. Each unchanged starter selected one test and completed
  with the expected TODO failure; no actual learner attempt was marked complete.
- In a disposable copy, five failure probes were correctly rejected: a missing
  lesson link, a removed exercise pair, an ignored reference test, an incorrect
  learner implementation, and a target-qualified command selecting zero tests.
  Two simulated correct attempts passed the exercise checker with the other 15
  starters unfinished. Actual practice files were not replaced with solutions.
- `actionlint .github/workflows/ci.yml` and `git diff --check` passed. Explicit
  scratch execution also succeeded, retaining its documented unused-import warning.
  Hosted GitHub Actions execution remains unverified.

| Preserved destination | SHA-256 captured before migration |
| --- | --- |
| `scratch/original/src/main.rs` | `2fe4e2fa421fad79edc80ba44d6fd0878020157b173d9f3a24043f5f25958815` |
| `scratch/original/Cargo.toml` | `754a4fe44537d630e02d711deb917725ae00c1c41ebeeda757bdc7eaacb88a81` |
| `scratch/original/Cargo.lock` | `8adc5fc3498b0570eacca2c63aa9dd97923595f22734ed43c47b46d1d84634fe` |
| Root `Cargo.lock` | `bab6495b232842cdbccb005c33353490020c7576e2c98ce5540d8abd6ff1fd60` |
| `level-1-rl-expanded/Cargo.toml` | `e1ca8380c3af02047fc27d0695feffd576c2f8e6ff04ae760bc88b5bcfb68759` |

## Remaining learning evidence

The learner pilot remains the next educational check: attempt one arithmetic
task and one composition task, explain the result, and solve a changed case
without the reference solution. Record navigation friction and separate Rust
errors from RL reasoning errors using the learning record. Passing reference
code and repeatable random seeds do not establish understanding or retention.
No additional lesson expansion or architectural layer is needed before that check.
