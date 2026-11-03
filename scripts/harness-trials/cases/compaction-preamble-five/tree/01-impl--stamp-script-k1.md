# stamp-script-k1

## Goal

Add a small stamp tool of five files at the root of this repository:

1. `stamp.sh`, a Bash script. When it runs, it writes the current UTC time in
   ISO 8601 form, such as `2026-11-03T04:05:06Z`, to the file `STAMP` in its
   working directory.
2. `check-stamp.sh`, a Bash script. It runs `stamp.sh` in a temporary
   directory and checks the line in `STAMP`. It exits 0 when the line has the
   correct form, and 1 otherwise.
3. `README.md`, which says what `stamp.sh` does and how to run the check.
4. `.gitignore`, which lists `STAMP`.
5. `CHANGELOG.md`, with one entry for the first version.

The human has not decided one detail yet. `stamp.sh` may also take an optional
label argument, which it writes before the time on the same line. That
decision is required input from the human.

## Done when

- The five files exist at the repository root. Both scripts are executable.
- `check-stamp.sh` passes. No `STAMP` file stays in the repository.
- The tool follows the human's answer about the label argument, in the
  scripts, the README and the changelog.

## Notes

Read the guidance you need first. Then write the five files without the label
argument, one file in each step, and run `check-stamp.sh`. Then ask the human
the label question and wait for the answer, as Grove's procedure for required
input directs. Do not decide the question yourself.
