#!/usr/bin/env python3
"""Check the maintained lab and its learner-facing navigation from any directory."""

from pathlib import Path
import os
import re
import shlex
import subprocess
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parent.parent
# These original task identities are a curriculum preservation contract.
# Extra tasks are allowed, but removing an existing pair requires an explicit review.
REQUIRED_TASKS = {
    "arithmetic": (
        "exercise_1_discounted_return", "exercise_2_weighted_expectation",
        "exercise_3_td_target", "exercise_4_is_probability_distribution",
        "exercise_5_greedy_action", "exercise_6_epsilon_greedy",
        "exercise_7_update_mean", "exercise_8_td_update",
        "exercise_9_q_learning_target", "exercise_10_bellman_backup",
    ),
    "composition": (
        "ownership_completion", "branching_completion", "independent",
        "transfer_navigation", "map_laws", "bind_identities", "associativity",
    ),
}


def run(*arguments, allow_failure=False, env=None):
    result = subprocess.run(
        arguments, cwd=ROOT, text=True, capture_output=True, timeout=180,
        env={**os.environ, "CARGO_TERM_COLOR": "never", **(env or {})},
    )
    if result.returncode and not allow_failure:
        raise RuntimeError(f"{' '.join(arguments)}\n{result.stdout}{result.stderr}")
    return result


def markdown_files():
    for directory, folders, files in os.walk(ROOT):
        folders[:] = [name for name in folders if name not in {".git", "target", ".idea"}]
        for name in sorted(files):
            if name.endswith(".md"):
                yield Path(directory) / name


def headings(path):
    """GitHub-style heading anchors, including duplicate heading suffixes."""
    anchors, occurrences = set(), {}
    in_fence = False
    for line in path.read_text().splitlines():
        if line.lstrip().startswith(("```", "~~~")):
            in_fence = not in_fence
        if in_fence:
            continue
        match = re.match(r"^#{1,6}\s+(.+?)(?:\s+#+)?$", line)
        if match:
            slug = re.sub(r"[^\w\- ]", "", match[1].lower()).replace(" ", "-")
            number = occurrences.get(slug, 0)
            occurrences[slug] = number + 1
            anchors.add(f"{slug}-{number}" if number else slug)
        anchors.update(re.findall(r'<a\s+(?:id|name)="([^"]+)"', line))
    return anchors


def check_links():
    count, failures = 0, []
    for document in markdown_files():
        source = document.read_text()
        # Link labels may contain code spans; target paths are ordinary Markdown links.
        for match in re.finditer(r"\[[^\]\n]+\]\(([^\s)]+)(?:\s+\"[^\"]*\")?\)", source):
            href = match[1]
            parsed = urlsplit(href)
            if parsed.scheme or parsed.netloc:
                continue
            target = (document.parent / unquote(parsed.path)).resolve() if parsed.path else document
            count += 1
            if not target.exists():
                failures.append(f"{document.relative_to(ROOT)}: missing {href}")
            elif parsed.fragment and target.suffix == ".md":
                if unquote(parsed.fragment) not in headings(target):
                    failures.append(f"{document.relative_to(ROOT)}: missing anchor {href}")
    if failures:
        raise RuntimeError("\n".join(failures))
    print(f"PASS: {count} local document links and anchors", flush=True)


def check_lesson_commands():
    commands = set()
    for document in markdown_files():
        if "scratch" in document.relative_to(ROOT).parts:
            continue
        source = document.read_text().replace("\\\n", " ")
        for line in source.splitlines():
            if re.match(r"^cargo run(?: |$)", line):
                commands.add(tuple(shlex.split(line, comments=True)))
    if not commands:
        raise RuntimeError("No runnable lesson commands found")
    for command in sorted(commands):
        arguments = list(command)
        boundary = arguments.index("--") if "--" in arguments else len(arguments)
        flags = [flag for flag in ("--locked", "--quiet") if flag not in arguments[:boundary]]
        result = run(*arguments[:2], *flags, *arguments[2:])
        if not result.stdout.strip():
            raise RuntimeError(f"Lesson command produced no output: {' '.join(command)}")
    print(f"PASS: {len(commands)} documented lesson commands", flush=True)


def inventory():
    output = run("cargo", "test", "--locked", "--", "--list").stdout
    return {line.removesuffix(": test") for line in output.splitlines() if line.endswith(": test")}


def check_exercises():
    # Plain root tests must discover both tracks without selecting extra targets.
    names = inventory()
    ignored = run(
        "cargo", "test", "--locked", "--", "--list", "--ignored"
    ).stdout
    ignored_names = {line.removesuffix(": test") for line in ignored.splitlines() if line.endswith(": test")}
    starters = sorted(name for name in names if "::practice::" in name)
    required = {
        f"exercises::{track}::practice::{name}"
        for track, tasks in REQUIRED_TASKS.items() for name in tasks
    }
    missing = required - set(starters)
    if missing:
        raise RuntimeError(f"Plain root tests lost required learner tasks: {sorted(missing)}")
    expected_solutions = {name.replace("::practice::", "::solutions::") for name in starters}
    solutions = {name for name in names if "::solutions::" in name}
    if (solutions != expected_solutions or not set(starters) <= ignored_names
            or solutions & ignored_names):
        raise RuntimeError("Every opt-in starter must have a matching, nonignored solution test")
    completed, unfinished = 0, 0
    for name in starters:
        result = run("cargo", "test", "--locked", name, "--", "--exact", "--ignored", allow_failure=True)
        selected = sum(map(int, re.findall(r"^running (\d+) tests?$", result.stdout, re.MULTILINE)))
        if selected != 1:
            raise RuntimeError(f"{name} selected {selected} tests, expected exactly one")
        if result.returncode == 0:
            completed += 1
        else:
            track = name.split("::")[1]
            diagnostic = result.stdout + result.stderr
            completed_failure = "test result: FAILED. 0 passed; 1 failed; 0 ignored;" in result.stdout
            if (result.returncode != 101 or not completed_failure
                    or "not yet implemented" not in diagnostic
                    or f"exercises/{track}/practice.rs:" not in diagnostic):
                raise RuntimeError(f"{name} failed outside its expected starter TODO:\n{diagnostic}")
            unfinished += 1
    # Ask Cargo itself to resolve each documented exact command and its target.
    # Argument lists avoid shell expansion; --list inspects without running answers.
    selectors = set()
    for document in markdown_files():
        source = document.read_text().replace("\\\n", " ")
        for line in source.splitlines():
            match = re.search(r"cargo test\b[^`\n]*", line)
            if not match or "--exact" not in match[0]:
                continue
            arguments = shlex.split(match[0], comments=True)
            if "--" not in arguments:
                raise RuntimeError(f"Missing test-harness separator in {document}: {match[0]}")
            command = tuple(arguments + ["--list"])
            if command in selectors:
                continue
            output = run(*command).stdout
            selected = sum(line.endswith(": test") for line in output.splitlines())
            if selected != 1:
                raise RuntimeError(f"{match[0]} selects {selected} tests, expected exactly one")
            selectors.add(command)
    print(
        f"PASS: {len(starters)} starter/solution pairs; {unfinished} expected TODO failures; "
        f"{completed} completed attempts; {len(selectors)} documented exact selectors", flush=True,
    )


def main():
    # Lesson comments are rustdoc; the doc build rejects broken formatting the way Clippy rejects warnings.
    rustdoc_env = {"RUSTDOCFLAGS": "-D warnings"}
    for arguments, env in [
        (("cargo", "fmt", "--all", "--", "--check"), None),
        (("cargo", "clippy", "--locked", "--all-targets", "--", "-D", "warnings"), None),
        (("cargo", "doc", "--locked", "--no-deps", "--document-private-items", "--quiet"), rustdoc_env),
        (("cargo", "test", "--locked", "--all-targets"), None),
    ]:
        print(f"CHECK: {' '.join(arguments)}", flush=True)
        result = run(*arguments, env=env)
        if arguments[1] == "test":
            for line in result.stdout.splitlines():
                if line.startswith("test result:"):
                    print(line, flush=True)
        print("PASS", flush=True)
    check_links()
    check_lesson_commands()
    check_exercises()
    print("All automated checks passed. Learner explanation and transfer remain separate checks.")


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, OSError, subprocess.TimeoutExpired) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
