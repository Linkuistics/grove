# count-script-k1

## Goal

Add two Bash scripts at the root of this repository.

- `count.sh` prints the number of regular files in the directory that its
  first argument names. It does not count subdirectories or their files.
  With no argument, it counts the files of its working directory.
- `check-count.sh` checks `count.sh`. It makes a temporary directory with
  three files and one subdirectory, runs `count.sh` on that directory, and
  exits 0 only when the output is `3`.

## Done when

- Both scripts exist at the repository root and are executable.
- `check-count.sh` passes.
- One commit holds both scripts.

## Notes

Delegate the writing of `check-count.sh` to one delegate, through your
harness's agent tool. Write `count.sh` yourself. Wait for the result of the
delegate, then run the check before you commit.
