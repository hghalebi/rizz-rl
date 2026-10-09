# Commands and paths after the workspace cleanup

Updated **2026-10-09**. Run current commands from the repository root. The [lab README](../../README.md) owns operations; the [learning path](../LEARNING_PATH.md) owns the study route. This map helps you use older notes.

## Command equivalents

| Earlier command or convention | Current command or location |
| --- | --- |
| Root `cargo run` selected the original scratch package. | Root `cargo run` now selects the reference lab. Run historical scratch explicitly with `cargo run --manifest-path scratch/original/Cargo.toml`. |
| Lab commands required `--manifest-path level-1-rl-expanded/Cargo.toml`. | Use root `cargo run -- …`, `cargo test`, or `cargo clippy`. The explicit lab manifest still works inside the workspace. |
| Arithmetic practice used the `practice` example target and a short number filter. | Use a full selector from the [arithmetic catalog](../EXERCISES.md), with `--exact --ignored`. |
| Arithmetic solutions used the `solutions` example target. | Reference tests now run in ordinary `cargo test`; select one by replacing `practice` with `solutions` and omitting `--ignored`. |
| Composition test names began with `composition::`. | They now begin with `exercises::composition::`; use the [composition lesson](../COMPOSITION_LESSON.md). |
| Worked arithmetic output used a runnable example. | Use the `exercises::arithmetic::worked::demonstrations` test with output enabled. |

Examples of current exact commands:

```bash
cargo test exercises::arithmetic::practice::exercise_1_discounted_return -- --exact --ignored
cargo test exercises::arithmetic::solutions::exercise_1_discounted_return -- --exact
cargo test exercises::arithmetic::worked::demonstrations -- --exact --nocapture
cargo test exercises::composition::practice::map_laws -- --exact --ignored
```

## Source equivalents

| Former location | Current location |
| --- | --- |
| Root manifest, lockfile, and `src/main.rs` | [`scratch/original`](../../../scratch/original/README.md), excluded from the workspace |
| Lab `src/main.rs` contained most domain logic | [`probability.rs`](../../src/probability.rs), [`model.rs`](../../src/model.rs), and [`planning.rs`](../../src/planning.rs); `main.rs` is wiring |
| Lab `src/learning.rs` | [`cli.rs`](../../src/cli.rs) |
| `examples/practice.rs`, `examples/solutions.rs`, and their support cases | [`src/exercises/arithmetic`](../../src/exercises/arithmetic/mod.rs) |
| `src/composition.rs` and `src/composition/` | [`src/exercises/composition`](../../src/exercises/composition/mod.rs) |

Each exercise track keeps `practice.rs`, `solutions.rs`, `cases.rs`, and `worked.rs` separate. Case files contain expected values and count as assistance when assessing an unaided attempt. The 17 starters remain opt-in; a test filter does not isolate compilation from syntax or type errors elsewhere in its target.

Use `python3 scripts/check.py` for the current combined check. Dated test counts in earlier reviews are historical evidence, not the result of a new run. Learner attempts and retention remain separate from software verification.
