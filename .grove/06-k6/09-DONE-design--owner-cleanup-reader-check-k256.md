# owner-cleanup-reader-check-k256

## Goal

Make the owner design's statement about Claude Code's plugin-cache cleanup
true for the Claude Code version that the cutover uses.

## Context

- owner-inventory-reader-marker-k244 found that Claude Code 2.1.319's orphan
  cleanup reads `.in_use`. The section "Finding for the owner design" of
  `docs/owner-skill-delivery-evidence.md` records the evidence.
- The section "Resource lifetime" of `docs/specs/owner-skill-delivery.md`
  previously stated that cleanup ignored readers. That statement was false
  for 2.1.319.
- The executable is the only source for the check. The official plugin
  loading reference does not mention it.

## Done when

- "Resource lifetime" states the cleanup rule as the evidence shows it, and
  names the Claude Code version that it applies to.
- The design states whether the reader check changes any cutover rule. Its
  candidates are the freeze, "keep each version that a live reader uses
  registered and current", the reader evidence and the purge condition.
  Each rule that the design keeps has a reason that does not depend on an
  undocumented check.
- Each other section and decision record that repeats "ignores readers" is
  current. Search the claim, not a file list.

## Notes

The check protects only a session that wrote a marker. A Codex or Pi session
reads through links and writes no marker. A Claude Code version without the
check gives no protection. A conservative design can keep its rules and
correct only the statement. Record why.

## Decisions (running log)

- The installed Claude Code 2.1.319 executable shows the age and live-reader
  checks. The design records that version and links the executable evidence.
  An unreadable marker or an error during the liveness check also keeps the
  directory.
- Keep the freeze, versions registered and current, reader evidence and purge
  conditions. They preserve accepted bytes, absolute paths and native rollback,
  and they cover readers that write no marker. The undocumented check cannot
  replace those obligations. Resource lifetime records each independent reason.
- Reject relying on reader markers to permit updates or earlier purge. That
  option would depend on one version's check and would omit Codex and Pi
  readers. No switch transition, test seam or recovery-model property changes.
- Correct the purge wording and the existing final-session view together.
  The historical review remains history. No decision record repeats the false
  claim, and this correction changes no durable policy that needs a new record.

## Verification

- Prose checks pass with no hard findings. The manifest, source references,
  section anchors and served files pass their checks.
- The claim sweep finds the exact old assertion only in completed task
  history. A scratch control finds the wrong assertion and rejects a broken
  search pattern. The broader orphan-claim sweep also found the purge and
  evidence-summary wording that this leaf corrected.
- The spec, evidence, presentation sources, viewer and installed executable
  kept their digests throughout verification. The executable's digest matches
  the recorded evidence.
- Browser inspection could not run because the browser access host was
  unavailable. The served files match the edited sources. Their rendered
  layout remains unverified.
