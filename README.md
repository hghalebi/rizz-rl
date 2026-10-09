# Learn Hugging Face RL with Rust

A small Rust lab for understanding probability, exact planning, and value learning from observations. You can trace the computations by hand before connecting them to the Hugging Face Deep RL course.

**Start with [the learning path](level-1-rl-expanded/guides/LEARNING_PATH.md).** It gives you one session at a time, optional support for Rust and probability, and exercises with answers in separate files.

From this repository's root:

```bash
cargo run -- first-step
```

You need Rust and Cargo. The first build downloads dependencies; lessons then run locally without a model API, GPU, or Hugging Face account. The root Cargo workspace selects the reference lab by default.

Use the [lab README](level-1-rl-expanded/README.md) for commands, source locations, checks, and troubleshooting. Your original program is preserved separately in [scratch/original](scratch/original/README.md).

The lab evaluates and learns values for a fixed policy. It does not train a neural network or improve policy parameters. Its conversation states are invented teaching labels, not a validated model of people. The learning path connects these foundations to the official course and keeps learner progress separate from software test results.
