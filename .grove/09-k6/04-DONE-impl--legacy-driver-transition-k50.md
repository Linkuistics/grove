# legacy-driver-transition-k50


## Goal
Prepare and test safe preservation of the original installed 22.1.0 driver
through k13's mid-grove upgrade, without changing the live installation here.

## Context

The parent's exact complete-keg, cleanup, canonical-front, stable-PATH and
original grove-llm obligations are authoritative. The current session always
ends with its original harness-dispatch exit. Ravel is not its replacement.

## Done when

- Reusable preparation/refusal tests record original paths/digests, complete keg contents, automatic-cleanup suppression, non-overwriting stable-PATH link ownership and post-driver cleanup. Preserve original admission/teardown executable too.
- Isolated upgrade/unlink uses actual original 22.1.0 front/worker to launch/reap a next fake session/write its old temporary store; bare exit signals only the temporary legacy channel. Missing worker/keg or existing link refuses safely. Independent Ravel context remains isolated.
- Hand k13 exact application/verification/recovery instructions and a tested instrument; do not alter Homebrew installation or real PATH/config/history/authority. Retain original bytes through final finish's last signal.

## Notes

## Decisions (running log)

- Implement the parent-approved complete-keg preservation as a Python stdlib
  instrument with separate prepare/apply/verify/cleanup verbs. Preparation is
  read-only against the installation, inventories every file/directory/link and
  copies the complete keg as recovery evidence. Application preserves the
  canonical original path by requiring both Homebrew cleanup protections and
  creates only absent stable-PATH links for harness-dispatch and grove-llm.
- Pin bare grove-llm to the original helper as well as bare harness-dispatch,
  because subsequent old-driver sessions still need original admission and
  teardown. New-version checks use explicit new-keg paths. Cleanup removes only
  matching owned links after explicit driver-ended acknowledgement; it never
  removes the keg or changes Homebrew/config/history.
- Test first through subprocess CLI controls, then rehearse with actual copied
  22.1.0 front/worker and independent Ravel in temporary HOME/config/store/channel
  contexts. Keep test artifacts/logs in target/k50 and durable application,
  verification and recovery instructions in docs/legacy-driver-transition.md.
- The one fresh reviewer identified target-only link ownership as insufficient:
  a same-target owner replacement passed verify/cleanup. A failing subprocess
  regression demonstrated it. Record the created symlink's device/inode and
  nanosecond change/modification times in the plan's application receipt and
  require that identity before verify/cleanup. This behavioral correction is
  conclusively covered by the new control; no second reviewer is needed.
- The review's keg-retention finding was a contract ambiguity: the instrument
  never removes a keg, while the parent explicitly permits post-driver cleanup.
  Clarify that a later Homebrew cleanup is a separate operator action after the
  original driver has ended and the new installation works. No cleanup runs now.
- Full task check's first run identified two concrete causes: new absolute
  original-front exports needed ReleaseTooling classification in the existing
  exhaustive GROVE_* inventory, and host Bun had advanced to 1.4.3. Classify the
  release-shell names without changing runtime admission. Run remaining checks
  with the already-preserved pinned Ravel/target/tools/bun 1.4.2 on a child-only
  PATH; do not change host runtime or relax the project's 1.4.2 contract.

## Implementation plan

1. Create scripts/test_legacy_driver_transition.py with complete-keg, cleanup
   protection, link collision/ownership, changed/missing bytes and PATH controls;
   observe the missing instrument fail before implementation.
2. Implement scripts/legacy_driver_transition.py and Taskfile prepare/apply/
   verify/cleanup/controls/rehearsal entry points. Add deterministic controls to
   task check; the actual-original installation rehearsal remains explicit.
3. Add scripts/legacy_driver_rehearsal.py: copy the complete original keg,
   simulate prefix unlink/upgrade, launch the next fake session by canonical old
   path, test bare exit and old store/reaping, helper admission/teardown and
   independent Ravel context isolation. Verify refusals and frozen subjects.
4. Record exact original paths/digests and full logs, write k13's durable
   procedure, run full task check once after edits, reconcile and retire k50,
   seal its jj change and use original harness-dispatch exit last.

## Delivery and verification

`docs/legacy-driver-transition.md` is k13's exact application/verification/
recovery/post-driver cleanup procedure. `docs/legacy-driver-inventory.json`
pins all 56 original entries (42 file digests) and the complete original paths.
Taskfile legacy:prepare/apply/verify/cleanup/controls/rehearsal provide the
tested instrument. Prepare records/copies all actual keg contents; apply
requires both cleanup protections and absent stable-PATH original dispatch/
helper links; its identity receipt preserves even same-target replacements.
Cleanup touches only matching links after driver-ended acknowledgement.

31 controls, Ruff and strict Pyright pass. The actual 22.1.0 copied front/worker
rehearsal passes scratch upgrade/unlink, canonical launch, bare exit to only its
temporary legacy channel, execution-confirmed old SQLite record/export and
process/descendant reaping. Nested installed Ravel 0.1.0 supplies only its own
opaque result/context; wrong ending commands cannot end the other session.
Original helper version/no-loop/stale-admission behavior, real worker/keg
refusals and post-reap owned cleanup pass. No second Grove driver or admitted
finish/teardown was launched. System-only original dynamic linkage is recorded.

Final task check passes all 15 principal checks, 1,378 workspace cases, nine
doctests and six final/all books. All 15,788 frozen input subjects match
individually, including original/Ravel installations, config, old history,
adjacent checkouts, tools, dependencies and archive inputs. Complete logs,
failed controls/initial checks, final rehearsal/store/ending/context records,
strict typecheck, one review and per-item digest comparison are in target/k50;
the durable procedure names every instrument and its measurement boundaries.

Preparation evidence target/k50/original-plan was not applied. Real
~/.local/bin/harness-dispatch and grove-llm remain absent; original canonical
22.1.0 paths/bytes and all real configuration/history/authority stay intact.
k13 prepares/applies a fresh durable plan, sources release-env.sh in every
release/recovery shell and records its absolute path for final finish. Keep
complete original bytes and links through final finish's last signal. No
Homebrew operation, installation/publication, integration, historical import
or teardown ran. k52/k51 and k6 remain live; this session ends through its
original harness-dispatch exit after retirement and sealing.
