# ravel-extraction-k2

## Goal

Cut the smallest independently useful Ravel extraction increment from the
settled requirements. Sequence the necessary follow-up work as small verified
increments, keeping Grove usable throughout the transition.

## Context

- `plan-k1` records the human's decisions; resolve its handle to read the running
  log. The root brief incorporates the final caller-independence clarification.
- The existing launcher package contains its front, policy worker, SDK, scripts
  and command tests under `crates/harness-dispatch`. Its Grove adapter and
  Grove-specific examples/tests must remain caller-owned after extraction.
- `crates/keyed-launch` holds the existing internal runner. Grove currently uses
  that library locally and locates dispatch beside its own real executable via
  `crates/grove/src/dispatch.rs`. The target boundary is the `ravel` CLI only.
- `Taskfile.yml` and the existing release/build/install scripts own the reusable
  recipes to transfer. Preserve their working behavior and verification seams.
- Reuse `/Users/antony/Development/Ravel` and `Linkuistics/Ravel`. The first
  source-replacement increment must preserve the old revision and any local
  edits before replacing its active project. Recheck that checkout's state.
- Keep the existing configuration format. Copy the owner's current
  `~/.config/harness-dispatch` contents into `~/.config/ravel` in the first
  runnable increment, preserving choices and settings. Recheck the source and
  destination; update only necessary SDK or caller-owned adapter imports.

## Done when

- The next implementation leaf has a narrow, concrete result that can be built
  and demonstrated independently: the extracted Ravel CLI and matching worker
  operate with a generic caller, using Ravel's own source and tasks. Its first
  runnable installation uses the owner's copied configuration in the existing
  format.
- The necessary dependency order is explicit: independent Ravel delivery,
  Grove's executable-only consumption and packaging transition, then the
  controlled-session notice and continue/force-stop behavior as a separate
  small increment. Size or split those steps from actual code evidence.
- Each cut leaf names its observable acceptance checks through the agreed
  existing seams. Preserve cancellation, reaping, terminal behavior, records
  and live-session authority guards during the move.
- The handoff honors the human's boundaries: Ravel knows no Grove conventions;
  Grove knows only Ravel's CLI and parameters; Homebrew provides the executable;
  there is no cross-project Rust-library dependency.
- Only concrete next work is cut. The future agent-server remains a horizon
  note, and the old Ravel daemon design does not become a new specification.

## Notes

The requirements interview is settled. Synthesize it rather than repeating it.
Prefer the existing implementation and public command seams, with the minimum
ownership changes required by the caller-independent boundary. The root brief
already records the default continue ending, explicit force-stop and the
separate controlled-session increment; the exact implementation mechanics
belong to that increment's focused work.
