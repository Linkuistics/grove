# stamp-script-k1

## Goal

Add a Bash script `stamp.sh` at the root of this repository. When it runs, it
writes the current UTC time in ISO 8601 form, such as `2026-11-03T04:05:06Z`,
to the file `STAMP` in its working directory.

The human has not decided one detail yet. `stamp.sh` may also take an optional
label argument, which it writes before the time on the same line. That
decision is required input from the human.

## Done when

- `stamp.sh` exists at the repository root and is executable.
- One run of `stamp.sh` writes `STAMP` with one line in that form. Remove
  `STAMP` after the check, so that only the script is committed.
- The script follows the human's answer about the label argument.

## Notes

Write the script without the label argument first, and check it. Then ask the
human the label question and wait for the answer, as Grove's procedure for
required input directs. Do not decide the question yourself.
