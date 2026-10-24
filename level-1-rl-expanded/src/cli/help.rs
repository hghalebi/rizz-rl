//! The `--help` text: the user's view of every command and option.
//!
//! Question: what can a user type, and which options apply to which command?
//! Example: `--seed` applies only to `learn-value`; `--verbose` applies to every command.
//! Predict: which test keeps this text and `CommandName::accepts` in agreement?

pub(super) const HELP: &str = "A small laboratory for planning and learning policy values.
Planning commands evaluate a known model exactly; learn-value uses sampled episodes.

From the repository root:
  cargo run -- [COMMAND] [OPTIONS]

Commands:
  first-step  Follow action and next-state probabilities, then average rewards
  returns     Compute discounted return for the supplied rewards [1, 2, 4]
  evaluate    Score one policy by enumerating all represented trajectories
  sweep       Score all 11 boldness values from 0.0 to 1.0 (default command)
  compare     Compare trajectory enumeration with Bellman evaluation at H=0..H
  learn-value Estimate a fixed policy's value from sampled episodes
  td-trace    Follow a supplied deterministic TD update, including termination

Options:
  --boldness B  Finite number in [0, 1]; default 0.3 (evaluate, compare, learn-value)
  --horizon H   Integer in [0, 6]; default 5 (evaluate, sweep, compare, learn-value)
  --discount G  Finite number in [0, 1]; default 0.95 (returns, evaluate, sweep, compare, learn-value)
  --episodes N Integer in [1, 1000000]; default 10000 (learn-value)
  --seed S     Unsigned 64-bit integer; default 7 (learn-value)
  --verbose     Show algorithm traces on stderr
  --help, -h    Show this help

The horizon is bounded because exact trajectory enumeration grows rapidly.
Example: cargo run -- compare --horizon 4 --discount 0.5";
