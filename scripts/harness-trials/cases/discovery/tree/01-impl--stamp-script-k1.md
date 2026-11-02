# stamp-script-k1

## Goal

Add a Bash script `stamp.sh` at the root of this repository. When it runs, it
writes the current UTC time in ISO 8601 form, such as `2026-11-03T04:05:06Z`,
to the file `STAMP` in its working directory. It takes no arguments.

## Done when

- `stamp.sh` exists at the repository root and is executable.
- One run of `stamp.sh` writes `STAMP` with one line in that form. Remove
  `STAMP` after the check, so that only the script is committed.
