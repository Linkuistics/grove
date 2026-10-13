# grove-extraction-release-k13

## Goal

Release the working Ravel extraction and Grove packaging increment part way
through this grove, preserving its live tree and original supervisor for k8.

## Context

- Requires grove-ravel-docs-k12's release-ready source, k6's tested legacy
  installation preservation and k11's independent installed Ravel release.
- The human chose a mid-grove release instead of two groves. This is an ordinary
  implementation checkpoint, not the driver-owned finish cycle. The eventual
  finish integrates/releases the later Grove-owned result-control increment.
- Follow `docs/RELEASING.md` and the project release tasks. Recheck main, default
  workspace and tap state; preserve unrelated work through jj.

## Done when

- Integrate only the extraction's project changes onto fetched current main
  through jj in an appropriate isolated/default-workspace change, resolving
  conflicts and running `bash scripts/check.sh` on the integrated result. Keep
  the live `.grove/` coordination tree and its commits in this session workspace;
  do not publish a live coordination tree on main or replace the workspace under
  its lease. Record the integrated source commit and reconcile the session
  branch with it so final finish can build on this release without replaying or
  dropping intervening main changes. Do not call finish-commit, record-teardown
  or remove `.grove/` to achieve a source-only integration.
- Preserve k6's original keg, canonical launch path and bare ending command
  before the Homebrew upgrade. Verify them without touching the live channel;
  disable automatic cleanup during the release and all later release steps
  while that driver remains live. Installing a new binary does not change the
  running driver. Preserve old config/store and front/worker pairing throughout.
- From the default colocated workspace, run `task release:minor`, with the
  validated preservation settings, and complete its tag, archives, main push,
  Homebrew tap and installed verification. Resume a partial cut/publication
  from the release recovery instructions rather than bumping again.
- Verify installed new Grove uses independently installed Ravel with fake
  agents, and the retained old installation can still launch/reap the next
  session with a temporary channel. Verify `harness-dispatch` still resolves
  on the inherited PATH for all remaining old-driver sessions.
- Record source/version/artifact and transition evidence, return to the original
  workspace, retire/commit this ordinary leaf, then use its original ending
  command as the last action. The remaining k8 leaf remain live and the
  driver's normal continuation proceeds; no successful teardown is recorded.

## Notes

k50's tested instrument and exact application/verification/recovery procedure
are in `docs/legacy-driver-transition.md`, with every original keg entry pinned
in `docs/legacy-driver-inventory.json`. Run the Taskfile `legacy:*` entry points;
prepare a fresh .jj/legacy-driver-22.1.0 plan and source its release-env.sh in
every release/recovery shell. Apply only absent stable-PATH harness-dispatch and
grove-llm links. Preserve them and the entire original canonical keg through final
finish's last signal; new-binary verification uses explicit new-keg paths.
Record the plan's absolute path in the root brief for final finish. k50 did not
apply compatibility links or alter the live installation. Its retained
target/k50/original-plan is preparation evidence, not an applied transition.

The human's mid-grove release request authorizes this checkpoint. It does not
authorize teardown or removing the original driver installation while live.
After the extraction release, public users get new Grove/Ravel, while this
already-running driver continues under its original launch/ending contract.

## Decisions (running log)

- Adopt the authorized mid-grove minor release; retain the live coordination
  tree, lease, original driver and ending contract throughout. No finish or
  teardown is part of k13.
- Fetched current main remains `89f947d5`, an ancestor of the accepted extraction
  source `e423bfe8`. Integrate its complete project tree excluding `.grove/`
  into a new default-workspace change on main, verify exact source equality,
  then reconcile this session onto the released main while retaining its tree.
- Apply the tested complete-keg preservation procedure before any upgrade;
  source its cleanup protections in every subsequent release/recovery shell.
- The 31 legacy controls and actual original-worker rehearsal passed. Prepared
  and applied `/Users/antony/Development/grove.extract-harness-dispatch-to-top-level/.jj/legacy-driver-22.1.0`;
  every original entry equals the recorded inventory and bare helper/dispatch
  resolve to its owned links. The release doctor passes with child PATH using
  Ravel's retained development Bun 1.4.2.
- Write concise extraction release notes in the integrated source before the
  cut, avoiding another harness launch solely to generate known release notes.
  This is the sole intended source difference from k58 before the version bump.
- The single read-only adversarial review found no issues with integration and
  two-parent session reconciliation, conditional on exact tree/coordination/
  ancestry checks. The initial equality check caught only old default-checkout
  development dependencies newly exposed by the removed crate's ignore rule;
  moved them intact to default `target/k13/retired-default-harness-dispatch`.
  Restored source then matched k58 exactly and contained no `.grove` entries.
- Integrated source is sealed at `96b6fd1e214dafee94a82f9a43ed8a2758406872`
  on fetched main `89f947d5`, with intentional release notes. All 18 checks,
  1,378 workspace cases, nine doctests and six final/all books pass; all 1,387
  runtime identities equal k58 individually and all 46,957 measured inputs
  retained their digests. Main contains no live coordination tree. Start the
  authorized minor release from its clean default-workspace successor; resume
  any subsequent partial cut at the documented boundary without bumping again.
- The release task passed its own 18-check gate and cut local **22.2.0** at
  `a79cd5aa41`; all three tagged archives built. Target acceptance is now in
  progress, before pushes/publication. If interrupted, resume this existing
  cut using `docs/RELEASING.md`; never rerun `task release:minor` to bump again.
- Tagged native/QEMU acceptance passed before publication. Main and `v22.2.0`
  were pushed at `a79cd5aa41834b7425106dc9b9896e1fd17e03d9`; the GitHub Release
  exists at `https://github.com/Linkuistics/grove/releases/tag/v22.2.0`.
  The task is publishing tap formula `4daa4aef05ad` and proceeding to Homebrew
  installation/verification. Any interruption resumes those remaining steps.
- Publication and installation completed, but the task stopped at `brew test
  grove`: Homebrew's outer sandbox refused Ravel's nested `sandbox-exec` probe
  with `sandbox_apply: Operation not permitted`. No caller was selected and no
  output published. The task lock was removed. Resume installed verification
  after diagnosing the outer test environment; do not repeat the cut or
  publication. Original complete-keg and owned-link verification still passes
  after the real upgrade.
- Recovery design: Homebrew's formula test runs a fake lifecycle through the
  installed public caller/SDK/result boundary within its own outer sandbox;
  the release task then runs the existing actual installed-smoke instrument
  outside Homebrew, retaining confined standalone publication and all original
  lifecycle/import checks. Do not disable either sandbox, change runtime code,
  alter archives or replace the public tag. Fix the template, task and recovery
  documentation, verify the real formula/layout and `brew test`, then publish
  only the corrected source and same-version tap formula through jj.
- The real failed `brew test` is the red reproduction. The separate actual
  new-keg direct/symlink lifecycle and confined standalone checks already pass;
  the actual original-keg post-upgrade/reap rehearsal also passes. These real
  executable seams cover the narrow verification repair; no second reviewer is
  needed and the single leaf-wide allowance remains spent.
- The corrected template's actual private Homebrew install/test, Ruby syntax
  and release-pipeline controls pass. Source correction is sealed/pushed at
  `972fd3180f58ecbcc593c60a612c50301bebe0c4`; same-version tap correction is
  sealed/pushed at `bc73cb9f1aaf54632d8abadd537c070192286657`. Refreshing the
  tap and resuming real `brew test` plus actual installed-smoke verification.
  Release tag and all three archive digests remain unchanged. Downloaded public
  archives match built bytes and GitHub's per-asset SHA-256 values individually.
- Resumed real `brew test grove` and direct/symlink installed lifecycle plus
  confined standalone acceptance pass; explicit Homebrew-bin versions are
  Grove/helper 22.2.0 and independent Ravel 0.1.0. All 11 installed command/
  caller-asset files equal the published native archive. Old-keg complete-byte,
  backup, owned-link and inherited-PATH verification still passes.
- Reconciled this session's change onto both accepted extraction `e423bfe8`
  and released/corrected main `972fd318`. The merge conflicted in the corrected
  release guide and formula template; restored the released main project tree
  outside `.grove`, as the integration contract requires. No conflict remains;
  every project path equals main, every coordination path and byte survived,
  main is an ancestor, and k8 remains live. No replay, tree deletion, teardown,
  epoch change or replacement driver occurred.
- Verified official Homebrew test source at `eb56e2b1f7` byte for byte against
  the installed file. Its retired `master` URL returned 404, so mechanically
  pinned the citation without changing the test at source main
  `96a7d855e8c8b1e616cea0ddaefa69c5c8da571b` and tap main
  `20ee956baca0b9f97daf3eccd2d2413ece2a88b7`, both sealed/pushed. Reconciled the
  session onto final main again, retaining coordination bytes and evidence;
  outside `.grove` the sole session addition is the durable release evidence.
  Post-report reference navigation passes all 13 cases and all six books pass
  final/all validation; reporting input digests agree across that measurement.

## Handoff

All k13 criteria are met. Grove 22.2.0 is published/installed, all three tagged
archives and public/installed pins agree, and the initial nested-Homebrew
verification failure is recovered without another cut or archive replacement.
Final real Homebrew test, explicit new-bin versions/viewer help, installed
lifecycle/confinement/imports, old-worker launch/reap rehearsals and complete
original/backup/link verification pass. Final evidence references and all six
books pass with unchanged reporting inputs. No release step remains.

`docs/grove-extraction-release-evidence.md` carries exact URLs, digests, source
and tap commits, criteria, recovery and measurement limits. Complete logs and
instruments are in `target/k13`. Default Grove and tap have clean empty
successors on synchronized main; Ravel remains clean at its k11 handoff. Owner
configs, independent Ravel and original keg bytes were preserved.

This session's merged ancestry includes final main
`96a7d855e8c8b1e616cea0ddaefa69c5c8da571b` and accepted extraction `e423bfe8`.
The complete coordination tree and history survive; the sole new project
artifact beyond main is durable release evidence. k8 remains live. No parent
close, finish-commit, teardown, driver replacement or workspace deletion is
part of this checkpoint. Keep the applied plan at
`/Users/antony/Development/grove.extract-harness-dispatch-to-top-level/.jj/legacy-driver-22.1.0`
and re-source its release-env.sh for every later release/recovery/finish shell.
The original ending/helper links and entire canonical keg remain through final
finish's last original signal. Retire and seal this ordinary leaf, then end
with bare `harness-dispatch exit` as the final action.
