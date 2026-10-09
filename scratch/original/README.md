# Original scratch program

This is the original `rizz-rl` package, preserved as historical learner work. Its manifest, lockfile, and source were moved here without changing their contents. It is excluded from the root workspace; ordinary root `cargo run`, `cargo test`, and `cargo clippy` select the reference lab.

The scratch policy currently has total probability `0.8` in `LowInterest`. It also uses different rewards and policies from the reference lab. Treat it as an incomplete experiment, not a probability-conserving baseline. Its unused imports can produce warnings, including failures with strict Clippy.

To run this version explicitly from the repository root:

```bash
cargo run --manifest-path scratch/original/Cargo.toml
```

For the maintained learning route, return to the [learning path](../../level-1-rl-expanded/guides/LEARNING_PATH.md). For reference commands and verification, use the [lab README](../../level-1-rl-expanded/README.md).
