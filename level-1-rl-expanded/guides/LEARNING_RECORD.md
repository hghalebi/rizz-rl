# Record what you can do independently

**Learner results: pending.** This file owns the project's assessment template and delayed retrieval prompts. It contains no completed learner attempt, score, or claim of retention.

Use it after [the composition lesson](COMPOSITION_LESSON.md), [an exercise](EXERCISES.md), or [the experience capstone](EXPERIENCE_LESSON.md). Choose one relevant prompt per short session. The aim is to identify the next useful task, not to accumulate completed checkboxes.

## Record an attempt

Copy this template into your own notes for each attempt. Record what happened, including incomplete work and help. Confidence describes your experience; it does not determine whether an answer is correct.

```text
Date/time and task:
Previous attempt date/time:
Actual elapsed delay:
Prediction made before running or opening the solution:
Answer or code I produced:
Observed output / assessment:
Correctness: pending / correct / partly correct / incorrect
Explanation: pending / principle explained / partly explained / unclear
Assistance: none / compiler only / hint / worked example / solution / other
Confidence before checking (1 = very uncertain, 5 = very confident):
Error type: probability / Rust ownership / RL boundary / forgotten step / other
One reason for the error, or one boundary I can explain:
Next smallest task, file, symbol, and command:
```

Keep correctness **pending** until checked. Record compilation separately from the explanation: the compiler can identify a move error, but a green build does not explain why a borrow meets the intended contract. If assistance changes during the attempt, record both the initial answer and what changed after help.

## Next available session

Begin without opening notes or solutions. Record the actual delay, even if this session happens sooner or later than planned. These are changed contexts; do not reuse the previous numeric answer.

### Probability: parcel routing

A parcel uses an express route with probability `0.6` and a standard route with probability `0.4`. Conditional on express, arrival is on time with probability `0.8`; conditional on standard, it is on time with probability `0.25`. Each route has only “on time” and “late” outcomes.

Write all complete branches and their probabilities. Aggregate the probability of “on time” and account for all mass. Explain which multiplication uses a conditional probability and which addition combines alternative routes. Then describe how `map` and `and_then` would differ if you renamed the outcome versus adding another uncertain step.

### Rust: parcel labels

You have an owned `String` containing a parcel identifier. A collection needs to inspect that identifier, and you still need to print the original afterward. Show a version whose collection borrows the string. Then explain what changes if the collection must own an independent label and outlive the original. State which ownership or lifetime constraint justifies each choice.

### RL: a robot's remaining inspection time

A robot can stop for a reward of `3`, or perform an inspection for reward `0` followed one transition later by reward `8` and termination. Use discount `0.5`. Compare one versus two remaining transitions. Explain what is part of the state-value table's key and what happens to continuation after termination. Do not assume the same action is preferred at both horizons.

After your attempt, check the relevant principle through the [composition lesson](COMPOSITION_LESSON.md) or the separately stored [exercise solutions](../src/exercises/arithmetic/solutions.rs). Those references support checking your reasoning; a different story does not necessarily have the same numeric answer. Ask for feedback on your recorded attempt when the explanation remains uncertain.

## A later session

Try a relevant prompt again after a longer gap, for example the following week. This is a practical starting interval, not a validated optimum for you. Record the actual delay rather than labeling the work “spaced” without evidence.

### Probability and composition: a battery-powered sensor

A sensor's battery is full with probability `0.7` and low with probability `0.3`. A full battery sends a packet successfully with probability `0.9`; a low battery succeeds with probability `0.4`. After a successful send, storage preserves the reading with probability `0.95`; a failed send cannot produce a stored reading.

Compute the event probability of a stored reading. Then describe two parenthesizations of the three stages and explain what equality you would compare. Account for distinct failure paths ending in the same event. State the assumptions needed to use the supplied conditional probabilities; do not replace conditional independence with a blanket claim that all events are independent.

### Rust and probability: an inventory transformation

Sketch a `Distribution<String>` operation that produces the length of each label while preserving its probability. Explain whether consuming the distribution means the transformation must clone every string. Contrast that operation with one that produces several probabilistic labels from each input. Identify the corresponding output types before writing code.

### Learning from experience: a maintenance policy

A fixed maintenance policy produces observed transitions `(state, reward, next state, termination)` but gives the learner no transition table. Describe the extra information needed to estimate a finite-horizon value and when each visit count changes. Explain how you would compare two training runs with different seeds, and what a larger error after more episodes would and would not establish.

After the unaided attempt, use the [composition lesson](COMPOSITION_LESSON.md) or [experience lesson](EXPERIENCE_LESSON.md) to inspect the relevant mechanism. Record any assistance rather than relabeling the attempt as independent.

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
