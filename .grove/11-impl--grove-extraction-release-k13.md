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
