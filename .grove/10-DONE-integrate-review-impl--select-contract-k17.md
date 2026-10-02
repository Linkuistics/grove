# select-contract-k17

**Integrates:** select-contract-k16

## Goal

Triage the implementation review `select-contract-k16`, apply findings that
hold against the contract, and verify the resulting increment before
`owner-settings-k9` builds on it.

## Context

- The review's committed task file carries its findings, evidence, rulings and
  verification limits. Read it rather than treating its conclusions as the
  integration's charter.
- The reviewed producer is `select-contract-k8`, commit `086fd972`.
- `docs/specs/harness-selection-and-execution.md` is the contract; the root
  brief and the producer delimit what later implementation leaves own.

## Done when

- Every review finding has an evidence-backed disposition in this task's log.
- Any accepted fixes and meaningful regression coverage are implemented, and
  the increment's installed-layout smoke test and `bash scripts/check.sh`
  have current verification evidence.

## Notes

The review is inspection-only and ran no tests or builds. Integration owns
runtime confirmation and all post-fix verification.

## Decisions (running log)

**F1 is a real issue, reproduced (2026-10-02).** The review derived it from
source and did not run it. Run here on Linux, in a container, against the
pre-fix front cross-built from `086fd972`'s sources: with `agent` in a PATH
directory named `bin-\xff`, `inspect --json` exited 0 and reported
`command.executable` as `…/bin-\u{FFFD}/agent`. The fixture holds another
executable at that spelling, so a caller executing the report runs the wrong
file. APFS refuses a name that is not UTF-8 (`EILSEQ`), so the case cannot be
built on the development machine.

**The fix refuses at resolution, for `run` and `inspect` alike (2026-10-02).**
`Executable.path` is now a `String`, and `program::resolve` refuses an
executable file whose path is not UTF-8 as `program_unexecutable`, exit 126,
where it is found. The three lossy conversions of that path (inspection, the
launch record, the handoff notice) are gone with the type. Chosen over
refusing in inspection alone, which needs its own refusal and breaks the
specified rule that inspection resolves as `run` does. The file is refused
and not passed over for a later PATH entry: the caller's shell would have run
it, and a later match is another command.

**The trade-off, accepted visibly (2026-10-02).** `run` could exec such a
path before and now refuses it. What it wrote then was a launch record and a
handoff notice naming a different file, so the lost case was never recorded
correctly. A human who wants `run` to keep it gets it by moving the refusal
into `inspect` and restoring the lossy record field.

**The regression test has a subject only where the filesystem admits the
name (2026-10-02).** `tests/run.rs` builds the non-UTF-8 directory and
returns early on `EILSEQ`, so on macOS it asserts nothing. Seen on Linux
(aarch64, the same container): it fails against the pre-fix front with the
lossy report above, and passes against the fixed one, for a PATH name and
for a relative program in a non-UTF-8 cwd, under `inspect` and `run`. There
is no CI, so that Linux run is by hand; `scripts/check.sh` does not repeat it.

**The eight rulings stand (2026-10-02).** They are rulings, not findings, and
none asks for a change. Ruling 4's caveat, that `pathEntry` is not derivable
for a relative PATH entry, concerns a field the contract does not specify.

**Verification of the integrated increment (2026-10-02).** Run after the last
edit, with the changed sources, test, spec and README digested before and
after and unchanged. `bash scripts/check.sh`: all 12 principal checks pass.
`task release:smoke`: archives of the working copy for all three targets,
and the installed-layout smoke test passed on each (`aarch64-apple-darwin`
natively, both Linux targets under the pinned QEMU at their CPU and glibc
floors), 4 cases through both fronts. No further review is cut: the fix is
covered by the test seen to fail before it.
