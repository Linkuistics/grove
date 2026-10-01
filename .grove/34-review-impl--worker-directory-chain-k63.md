# worker-directory-chain-k63

**Reviews:** worker-directory-chain-k61

## Goal

An adversarial, inspection-only read of `worker-directory-chain-k61`: the
worker's move to `/` before it loads anything, the owner-only start directory,
and the tests and documents that claim no directory above the worker's start
directory takes part in an evaluation. Produce findings, not fixes.

## Context

k61 was offered three repairs and took none of them. Its running log says why:
Bun moves the resolver's directory with `process.chdir`, so the worker can
start in the private directory and resolve from `/`. That choice was made
without the human, on the reasoning that no stated control is weakened.
`package-json-autoloading-k62` runs next and rests on it, so attack it first.

Read the spec's `#policy-authority` and the firing-configuration table under
`#test-seams`, *The worker's directory* in the runtime evidence,
`docs/adr/policy-evaluation-precedes-process-replacement.md`, and k61's
running log. The code is the statement above the embedded table in
`worker/src/main.ts`, the directory creation in `evaluate` in `src/worker.rs`,
the `unmoved` probe in `scripts/dispatch.sh`, and three tests: the package
case and the tripwires in `tests/hostile.rs`, the start-directory case in
`tests/worker.rs`, and the root-directory case in `tests/authority.rs`.

Specific doubts k61 could not settle from inside:

- **Half the argument is seen, not read.** That dotenv and bunfig are read
  only from the directory a process starts in rests on one probe result on one
  host. k61 read `run_env_loader` and did not trace its caller on a compiled
  executable's start path. Read the bun-v1.4.2 source for anything the runtime
  reads from the current directory lazily, after start. For any such file the
  private empty directory is no longer a control, and only a compile switch is.
- **Nothing may resolve before the move.** The move is a statement in the
  worker's main module. Check what Bun does with the start directory before
  that statement runs, with the switches as shipped and with package.json
  autoloading on, which k62 is about to enable. k61 saw a 4 GiB `package.json`
  in TMPDIR leave a selection unaffected in an experimental build, and no test
  holds that.
- **`/` is not empty and cannot hold a fixture.** A container image can carry
  `/node_modules`, `/package.json`, `/.env` or `/bunfig.toml`, and a process
  running as root can write there. k61 says the first two are already above
  every module in a file, and tests the last two only through a policy that
  moves into a hostile directory. Decide whether that stands for `/`.
- **The policy-facing change.** A policy's `process.cwd()` is now `/` and its
  relative writes fail or land there. Check the shipped examples, the SDK and
  the documents for anything that assumed the private directory.
- **The firing configuration departs from k61's done-when**, which named the
  shipped worker started under the planted directory. The `unmoved` probe
  replaces it. Check that the probe differs from the shipped build by the move
  alone, and that the case's three arms prove what its name says.
- **The two changed tripwires.** The cwd tsconfig tripwire's entry now moves
  the worker back into the caller's cwd. Check that it still watches the class
  it reports, and that the new moved-into tripwire could fail.
- **The mode.** The start directory is created `0o700`. Check the claim that
  no window exists, and what a restrictive umask or an unwritable TMPDIR does.
- **Linux is unmeasured for the control.** The installed smoke test runs no
  hostile fixture.

## Done when

- Each doubt above has a finding or a stated reason there is none, with the
  source or the observation behind it.
- The spec, README, ADR and runtime evidence have been read against the code
  and the tests, and every statement that is not true of them is a finding.
- Findings are recorded in this file. If any is actionable, an
  `integrate-review-impl` leaf is inserted ahead of
  `package-json-autoloading-k62`.

## Notes

Inspection only. A probe build or a scratch experiment is reading, not fixing.
