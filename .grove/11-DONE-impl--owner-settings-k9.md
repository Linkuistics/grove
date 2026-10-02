# owner-settings-k9

## Goal

An owner sets the selection bound, the context budget, the record directory and
the policy's grants in one settings file with no flag passed. The bound's
ceiling admits minutes, and a policy can start a deciding agent and use its
answer.

## Context

- `docs/specs/harness-selection-and-execution.md`: *Owner settings*, the bounds
  table in *Bounded context*, *Dynamic dispatch*, and the command row of the
  test seams.
- Root brief, requirements 7 and 8.

## Done when

- Every command reads the settings file before any worker starts, as *Owner
  settings* states. A flag replaces a setting, `--policy-env` adds to the
  grants, a malformed file refuses, and inspection reports where each value
  came from.
- The whole-selection bound has a 600-second ceiling and keeps its 30-second
  default.
- At the command seam, a scripted stand-in started by a test policy receives
  the prompt in the caller's directory and its answer decides which command
  launches. A stand-in still running at the bound launches nothing.
- The installed-layout smoke test runs a policy that starts a child and uses
  its answer.
- The dispatch README documents the settings file. The README and the SDK's
  description of `prompt` say that keeping a deciding agent from executing the
  task is the policy owner's job.
- `bash scripts/check.sh` passes.

## Notes

- No agent policy and no model-calling policy ships, and no test calls a model.
- The settings are the same for every kind.

## Decisions (running log)

**Without an absolute HOME the settings file sets nothing (2026-10-02).** The
file is found from HOME alone, so an unset or relative HOME gives it no
location. Chosen over refusing: an invocation that names its policy and its
record directory with flags runs without HOME today, and the defaults it then
gets are the stricter ones, the short bound and no grant. Inspection shows
`default` for each.

**A value the file set reports `settings.json` as where it came from
(2026-10-02).** Bounds and the record directory already carry the input that
set them, so the file is one more label there: `from` in inspection and the run
record, and the `input` of a refusal such a bound causes. Grants keep their
`{ name, set }` shape and report no origin, because the specification asks for
one only on the bounds and the record directory.

**A malformed file refuses with exit 2 at stage `cli`, under two kinds of code
(2026-10-02).** A value its flag would refuse takes the flag's own code,
`malformed_input` or `excluded_grant`, as the specification's *refuses as the
flag would* reads. A file that cannot be read, is not a JSON object, holds an
unknown key, or holds a `stateDir` or `policyEnv` of the wrong shape is
`settings_invalid`. Either names the file as `source` and the key as
`location`. A bound is a JSON whole number, and a string of digits refuses.

**The file is checked whole by every command (2026-10-02).** `record show` and
`record observe` read only `stateDir`, but refuse a file whose `timeoutMs` is
out of range. Chosen over each command checking the keys it uses, which would
leave a broken bound unnoticed until the next launch.

**Remedies that say how to raise a bound name the setting beside the flag
(2026-10-02).** Once Grove passes no flag, an owner under Grove can act only on
the setting.

**The stand-in is started with `Bun.spawn` under `host.signal`, and the README
says to pass it (2026-10-02).** Seen on the pinned Bun 1.4.2 and documented at
https://bun.com/docs/runtime/child-process. With the signal removed from the
test policy, the stand-in outlived the timed-out selection and the case failed
on that assertion, so the front stopping its worker does not stop a child the
policy started. With `cwd` removed, the stand-in ran in `/` and the case failed
there. The stand-in receives the prompt as its one argument.

**The README's *Called from Grove* section still describes a command
definition (2026-10-02).** It is true until `lifecycle-launch-k12` changes the
launch, and `current-state-documents-k15` rewrites it.

**No review is cut, and the in-session reviewer was not spent (2026-10-02).**
Each claim here that a compiler cannot establish has a case at the command
seam: that only the file under HOME is read, beside the same bytes read there;
that a flag replaces a setting; that a setting is the bound that refuses; and
the two stand-in assertions, each seen to fail against a mutated policy. The
contract four leaves build on was `select-contract-k8`'s, and it had its
review.

**Verification (2026-10-02).** `task release:smoke` built archives of the
working copy for all three targets, and the installed-layout smoke test passed
on each: `aarch64-apple-darwin` natively, and both Linux targets under the
pinned QEMU at their CPU and glibc floors, 5 cases through both fronts, the
child-answer case among them. The working copy was one snapshot before and
after. One sentence was then added to the specification, on a HOME that cannot
place the file. `bash scripts/check.sh` was run after that, on one snapshot
before and after: all 12 principal checks pass. Only this entry is later.
