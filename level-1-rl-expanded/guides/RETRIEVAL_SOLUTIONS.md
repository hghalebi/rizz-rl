# Feedback for delayed practice

**SPOILERS.** Open only the section matching a prompt you have attempted, or when you deliberately choose support. The questions live in [the learning record](LEARNING_RECORD.md). Reading feedback is useful work; it does not turn the preceding attempt into an unaided one.

Check both your result and your reason. If they differ, name the first step where they diverge. Close this file before another changed case.

## Parcel routing

The four complete paths are:

| Path | Probability |
| --- | --- |
| Express, on time | `0.6 * 0.8 = 0.48` |
| Express, late | `0.6 * 0.2 = 0.12` |
| Standard, on time | `0.4 * 0.25 = 0.10` |
| Standard, late | `0.4 * 0.75 = 0.30` |

They sum to `1`. On-time arrival has probability `0.48 + 0.10 = 0.58`.

Multiplication follows one complete path using a response conditional on its route. Addition merges mutually exclusive routes into the same event. This does not assume that arrival is independent of route.

`map` could rename each outcome while preserving its weight. `and_then` adds a conditional distribution after each current outcome, multiplying path weights. A rename can merge labels, but it does not create a new random step.

**If you got stuck:** draw just the express branch, then return to the [branching support](COMPOSITION_LESSON.md#2-worked-examples-inspect-only-the-support-you-need).

## Parcel labels

Borrow while the original owner is alive:

```rust
fn main() {
    let label = String::from("parcel-7");
    let labels: Vec<&str> = vec![label.as_str()];
    assert_eq!(labels[0], "parcel-7");
    println!("Original is still usable: {label}");
}
```

The vector owns its references, not the string data. The reference cannot outlive `label`. Making it a vector does not extend the borrowed string's lifetime.

For an independent label that outlives the original:

```rust
fn main() {
    let labels: Vec<String> = {
        let label = String::from("parcel-7");
        let copies = vec![label.clone()];
        println!("Original is still usable here: {label}");
        copies
    };
    assert_eq!(labels[0], "parcel-7");
}
```

Here cloning creates an independently owned string. If the original no longer needs to be used, moving it into the vector is another valid choice and avoids that clone. Explain the required lifetime and later use before choosing.

**If you got stuck:** use only the ownership example in the [composition support](COMPOSITION_LESSON.md#2-worked-examples-inspect-only-the-support-you-need).

## Robot inspection

The horizon counts transitions whose rewards belong to this task.

| Remaining transitions | Stop | Inspect | Better initial action |
| --- | --- | --- | --- |
| One | `3` | `0` | Stop |
| Two | `3` | `0 + 0.5 * 8 = 4` | Inspect |

The state alone cannot distinguish these two remaining problems. Use `(state, steps_remaining)` as the value-table key. The final reward `8` belongs to the transition into termination; continuation after that transition is zero. An external observation cutoff in a continuing task is a different boundary and does not by itself mean termination.

**If you got stuck:** write down only which rewards fit inside one remaining transition, then revisit [terminal and horizon behavior](LEARNING_PATH.md#session-3-find-the-end-of-the-episode-20-minutes).

## Battery-powered sensor

The stored-reading event combines two paths:

```text
full -> successful send -> stored: 0.7 * 0.9 * 0.95 = 0.5985
low  -> successful send -> stored: 0.3 * 0.4 * 0.95 = 0.1140
total stored: 0.7125
```

Send failure has mass `0.7 * 0.1 + 0.3 * 0.6 = 0.25`. Successful send followed by storage failure has mass `(0.7 * 0.9 + 0.3 * 0.4) * 0.05 = 0.0375`. These and stored success total `1`. If failures share one final label, aggregate their mass to `0.2875`.

Let `B` be the battery distribution, `send` return a conditional send distribution, and `store` return a conditional storage distribution. Compare:

```text
B.and_then(send).and_then(store)
B.and_then(|battery| send(battery).and_then(store))
```

Both must start from the same input distribution, constructed separately because these operations consume it. Compare final event probabilities, not vector order. This illustrates associativity for these pure, valid finite distributions, with floating-point tolerance.

Using `0.95` on both successful-send paths assumes that the storage-success probability after a successful send does not further depend on battery state. Sending still depends on battery state. If storage also depended on battery, retain that information and use the appropriate conditional probability in both groupings.

**If you got stuck:** verify the full-battery path alone before the [associativity exercise](COMPOSITION_LESSON.md).

## Inventory transformation

For ASCII labels `"bolt"` and `"nut"`, byte lengths are `4` and `3`. The branch weights do not change:

```rust
fn main() {
    let labels = Distribution {
        outcomes: vec![(String::from("bolt"), 0.6), (String::from("nut"), 0.4)],
    };
    let lengths: Distribution<usize> = labels.map(|label| label.len());
    assert_eq!(lengths.outcomes, vec![(4, 0.6), (3, 0.4)]);
}
```

This snippet uses the lab's `Distribution` from `src/probability.rs`; it is not a standalone public library import. `map` moves each `String` into the closure. The closure borrows it briefly to measure length, so it needs no clone. The original distribution has been consumed.

Several probabilistic labels per input require a function `String -> Distribution<String>` and `and_then`, yielding `Distribution<String>`. Each new branch weight multiplies the input weight by the conditional output weight. `String::len` counts UTF-8 bytes; a character-counting goal needs a different operation and a precise definition of character.

**If you got stuck:** draw `one label -> one length` beside `one label -> weighted labels`, then revisit [composition](COMPOSITION_LESSON.md).

## Maintenance policy

A finite-horizon observation needs its **remaining transition count**, the discount, and a clear distinction between actual task termination and a recording cutoff. The learner also needs a declared initialization for unseen entries. In this lab the value and visit count belong to `(state, steps_remaining)` under one fixed policy and discount.

For an observation at `(s, h)`, use the next estimate at `(next_state, h - 1)`. Continuation is zero at true termination or zero remaining horizon. Increment only the visited starting entry's count, once for the accepted observation. The learning rate `1 / visits` includes that new visit; it is not based on the total episode count or all states' visits.

Compare runs under the same policy, model, starting state, horizon, discount, and episode budget, changing the seed deliberately. Compare estimates with the same exact reference and inspect visit coverage. Repeat across seeds and budgets to see variability; an unvisited zero estimate is not learned evidence.

A larger error at a larger budget can occur in a finite sampled run. It neither proves that learning is impossible nor establishes that the implementation is correct. Inspect boundaries, visits, multiple runs, and the exact comparison. A reproducible seed establishes repeatability, not accuracy or convergence.

**If you got stuck:** inspect one row of the supplied [TD trace](EXPERIENCE_LESSON.md), then explain which entry receives its update.

Return to [your restart note](LEARNING_RECORD.md#quick-restart). Keep help used visible, and assess a fresh case with this file closed.
