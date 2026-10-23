# Record what you can do independently

**Learner results: pending.** This file owns the project's assessment template and delayed retrieval prompts. It contains no completed learner attempt, score, or claim of retention.

Use this page when stopping or returning. You do not need to complete a form to take a break. Keep your own notes wherever you can find them again.

## Quick restart

Copy these three lines into your notes. An unfinished thought is enough.

```text
Where I stopped: task and function.
What I observed or still do not understand: include help used, if any.
Next action: exact file, function, and command.
```

For example, a useful next action names `src/exercises/arithmetic/practice.rs`, `discounted_return`, and the exact command from [exercise 1](EXERCISES.md#1-discounted-return). Paths here are relative to `level-1-rl-expanded`; commands run from the repository root.

**Returning?** Open the file named in your note and do just that action. If the note is unclear, use the [learning path](LEARNING_PATH.md) to choose one small activity. You do not need to restart the course.

This note helps you resume. It does not claim an assessment is complete. Use the optional record below when you want to check what you can do independently.

## Record an attempt

For an assessment, first save your prediction, then check one result and one explanation. Choose one relevant prompt per session. Leave anything you have not checked **pending**. Confidence describes your experience; it does not determine correctness.

<details>
<summary>Optional detailed assessment: before running, after checking, and later retrieval</summary>

```text
Before running:
Date/time and task:
Prediction made before running or opening the solution:
Confidence before checking (1 = very uncertain, 5 = very confident):

After checking or stopping:
Answer or code I produced:
Observed output / assessment:
Correctness: pending / correct / partly correct / incorrect
Explanation: pending / principle explained / partly explained / unclear
Assistance: none / compiler only / hint / worked example / solution / other
Error type: probability / Rust ownership / RL boundary / forgotten step / other
One reason for the error, or one boundary I can explain:
Next smallest task, file, symbol, and command:

For delayed retrieval only:
Previous attempt date/time:
Actual elapsed delay:
```

Keep correctness **pending** until checked. Record compilation separately from the explanation: the compiler can identify a move error, but a green build does not explain why a borrow meets the intended contract. If assistance changes during the attempt, record both the initial answer and what changed after help.

</details>

## Next available session

Choose **one** prompt below that matches your last activity. Begin without opening notes or solutions. Record the actual delay, even if this session happens sooner or later than planned. These are changed contexts; do not reuse the previous numeric answer.

### Probability: parcel routing

A parcel uses an express route with probability `0.6` and a standard route with probability `0.4`. Conditional on express, arrival is on time with probability `0.8`; conditional on standard, it is on time with probability `0.25`. Each route has only “on time” and “late” outcomes.

Write all complete branches and their probabilities. Aggregate the probability of “on time” and account for all mass. Explain which multiplication uses a conditional probability and which addition combines alternative routes. Then describe how `map` and `and_then` would differ if you renamed the outcome versus adding another uncertain step.

**Check after attempting:** four branches, total mass, merged on-time event, and a reason for each operation. Use the separate [parcel feedback](RETRIEVAL_SOLUTIONS.md#parcel-routing), then close it before trying another case.

### Rust: parcel labels

You have an owned `String` containing a parcel identifier. A collection needs to inspect that identifier, and you still need to print the original afterward. Show a version whose collection borrows the string. Then explain what changes if the collection must own an independent label and outlive the original. State which ownership or lifetime constraint justifies each choice.

**Check after attempting:** the borrowed version leaves the owner usable; the independent version explains which allocation lives longer. See the separate [label feedback](RETRIEVAL_SOLUTIONS.md#parcel-labels).

### RL: a robot's remaining inspection time

A robot can stop for a reward of `3`, or perform an inspection for reward `0` followed one transition later by reward `8` and termination. Use discount `0.5`. Compare one versus two remaining transitions. Explain what is part of the state-value table's key and what happens to continuation after termination. Do not assume the same action is preferred at both horizons.

**Check after attempting:** compare both actions under both horizons, preserve the arrival reward, and identify the value-table key. See the separate [robot feedback](RETRIEVAL_SOLUTIONS.md#robot-inspection).

**Stop here:** one prompt is enough for this visit. Save a [restart cue](#quick-restart). If your calculation and explanation agree with the feedback, return to the [learning path](LEARNING_PATH.md); otherwise choose the smallest matching support below.

## A later session

Try a relevant prompt again after a longer gap, for example the following week. This is a practical starting interval, not a validated optimum for you. Record the actual delay rather than labeling the work “spaced” without evidence.

### Probability and composition: a battery-powered sensor

A sensor's battery is full with probability `0.7` and low with probability `0.3`. A full battery sends a packet successfully with probability `0.9`; a low battery succeeds with probability `0.4`. After a successful send, storage preserves the reading with probability `0.95`; a failed send cannot produce a stored reading.

Compute the event probability of a stored reading. Then describe two parenthesizations of the three stages and explain what equality you would compare. Account for distinct failure paths ending in the same event. State the assumptions needed to use the supplied conditional probabilities; do not replace conditional independence with a blanket claim that all events are independent.

**Check after attempting:** account for all final events and state which conditional probability is assumed equal across battery states. See the separate [sensor feedback](RETRIEVAL_SOLUTIONS.md#battery-powered-sensor).

### Rust and probability: an inventory transformation

Sketch a `Distribution<String>` operation that produces the length of each label while preserving its probability. Explain whether consuming the distribution means the transformation must clone every string. Contrast that operation with one that produces several probabilistic labels from each input. Identify the corresponding output types before writing code.

**Check after attempting:** identify both output types, ownership, and preserved weights. State whether your length means bytes or characters. See the separate [inventory feedback](RETRIEVAL_SOLUTIONS.md#inventory-transformation).

### Learning from experience: a maintenance policy

A fixed maintenance policy produces observed transitions `(state, reward, next state, termination)` but gives the learner no transition table. Describe the extra information needed to estimate a finite-horizon value and when each visit count changes. Explain how you would compare two training runs with different seeds, and what a larger error after more episodes would and would not establish.

**Check after attempting:** identify the horizon-indexed entry, continuation boundary, visit count, and fair comparison conditions. See the separate [maintenance feedback](RETRIEVAL_SOLUTIONS.md#maintenance-policy).

**Stop here:** record any assistance rather than relabeling the attempt as independent. One checked explanation and a restart cue are a useful stopping point.

## Choose the next route from evidence

| Observed difficulty | Small next action |
| --- | --- |
| Probability reasoning fails while Rust reasoning is sound. | Trace one conditional branch and one merged event by hand, then try a fresh case. |
| Ownership reasoning fails while probabilities are correct. | Work through only the ownership example, then explain a new borrow/move choice. |
| Terminal and horizon rules are mixed together. | Revisit one TD trace row and contrast a terminal outcome with a continuing one. |
| A previously explained step cannot be recalled. | Reconstruct that small step, check feedback, and retry after another delay. |
| The changed case and explanation succeed without assistance. | Continue to the next concept or capstone, retaining a later retrieval check. |
| Confidence is high but the changed case fails. | Investigate the error; confidence does not override the result. |
| Confidence is low but the answer and explanation are correct. | Record that mismatch without requiring unnecessary remedial work. |

One successful delayed attempt supports retention for that task and delay. It does not prove broad expertise or that this teaching design caused the improvement. Until actual attempts exist, all learner outcomes remain pending.
