# Open-source launch preparation

Prepared on **2026-10-23** for `hghalebi/rizz-rl`, which was private when preparation began. The maintainer subsequently authorized committing and pushing these changes, then making the GitHub repository public under MIT. This record captures the preparation evidence and publication procedure; check the [repository](https://github.com/hghalebi/rizz-rl) for its current state.

## Prepared in the repository

- The [root README](../../../README.md) introduces the project with a runnable prediction, hidden feedback, verified output, a learning map, and a contribution route. Its dry humor follows the actual fictional conversation model: small talk, coffee, and Rust ownership. Commands, expected output, and contracts stay literal.
- The [MIT license](../../../LICENSE) covers project code and lesson text. MIT was selected by the maintainer; the maintained Cargo package declares it. Dependencies and linked resources retain their own licenses.
- [Contribution guidance](../../../CONTRIBUTING.md), [issue templates](../../../.github/ISSUE_TEMPLATE/learning_friction.md), and a [pull request template](../../../.github/pull_request_template.md) invite specific feedback and preserve the unanswered exercises.
- Editor metadata is excluded from tracking; local editor files can remain on the maintainer's machine. [Historical experiments](../../../scratch/README.md) are explicitly outside the maintained curriculum.

The RL algorithms, rewards, exercise contracts, and dependencies are unchanged. The README distinguishes exact planning, fixed-policy TD value learning, and the neural-network training covered by the external course. Counts in the worked example are not speed measurements or evidence of learner outcomes.

## Verification record

Preparation starts from `a68575b`. Its hosted [Learning lab checks run](https://github.com/hghalebi/rizz-rl/actions/runs/39998015765) completed successfully. That run validates the baseline; checks of the preparation changes are recorded below.

- `first-step` produced branch probabilities 56%, 14%, 18%, and 12%, totaling 1.
- `evaluate` at boldness 0.3, discount 0.95, horizon 5 reported 3,448 trajectories and expected return 3.103947.
- `compare` at the same settings reported the same rounded value with 15 cached non-base subproblems. The maximum displayed difference through horizon 5 was approximately 3.553e-15.
- Gitleaks scanned all seven reachable commits with redaction enabled and reported no findings. This is a scanner result, not proof that every kind of sensitive content is absent.
- A separate read-only review found no personal learning records or home paths in the inspected historical text. Tracked editor files were also checked for personal paths and email-like values without displaying their contents.
- A clean copy containing only the intended release files passed `python3 scripts/check.py`: formatting, strict Clippy, **79 passing tests**, **292 local links and anchors**, **29 lesson commands**, and **37 exact test selectors**. All **17 starter/solution pairs** remained present; selected starters produced their expected TODO failures. Ignored local documents were excluded from this copy.
- Gitleaks also scanned that clean release copy and reported no findings. Cargo metadata resolved the maintained package's license as MIT.
- A local Pandoc/GFM browser preview showed the first answer collapsed by default and opened it with the keyboard. The preview checks this renderer, not GitHub's eventual public rendering.
- Before publication, a separate reviewer checked the accessible GitHub material: zero issues, pull requests, discussions, or Actions artifacts, and three successful workflow runs. All three logs (70,894 bytes) were scanned with Gitleaks without findings; additional checks found no home paths, email addresses, or private-key headers. This review covers runs 39998015765, 39958276134, and 39958274712, not subsequent runs.

After any further changes, rerun the affected checks before publication and scan the proposed commit. Repository scans and the separate GitHub-material review have different scopes; review newly created logs or artifacts as needed.

## Publication checklist

1. Confirm that the code and lesson text can be licensed under MIT, including any adapted examples and `zed-vim-cheat-sheet.txt`. The inspected history attributes commits to the maintainer, but authorship metadata alone does not establish provenance. Add credits or required notices for any third-party material.
2. Review Git history, author email exposure, and historical GitHub Actions logs/artifacts for anything intended to remain private. Removing a file from the current tree does not remove it from history. GitHub documents that [Actions history and logs become public](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/setting-repository-visibility).
3. Review the final diff, commit and push the preparation, then make the repository public. The maintainer authorized these actions on 2026-10-23. Verify the resulting remote commit and anonymous access to the README, license, issue chooser, and CI page after publication.

Consider enabling GitHub private vulnerability reporting before inviting public reports; no private reporting channel or response-time promise is claimed by these files.

## Suggested GitHub description and topics

Description:

> Learn reinforcement learning in Rust, one awkward state transition at a time. 17 exercises, separate solutions, Bellman planning, and TD value learning.

Topics:

```text
rust
reinforcement-learning
education
probability
dynamic-programming
temporal-difference-learning
category-theory
```

Use these values when applying the launch metadata; verify the resulting settings on GitHub.

## Short launch copy

Use after the public repository and its links have been checked:

> I built rizz-rl: a Rust reinforcement-learning lab where an agent attempts small talk and the borrow checker asks who owns the String. Start with one decision, then compare 3,448 trajectories with 15 cached Bellman subproblems. Both reach the same expected return to six decimal places. There are 17 exercises, separate solutions, and a path into TD value learning. Try the first prediction: https://github.com/hghalebi/rizz-rl

Share a specific experiment with an audience learning Rust or RL. Judge the next improvement by whether a new reader can run the first command and explain the changed case. Stars and traffic measure reach, not understanding.

## References behind the preparation

- [GitHub README guidance](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes): purpose, first use, help, and contribution entry points.
- [MIT license text and application guidance](https://choosealicense.com/licenses/mit/).
- [Cargo license metadata](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields).
