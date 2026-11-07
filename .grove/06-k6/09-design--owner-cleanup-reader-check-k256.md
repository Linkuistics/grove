# owner-cleanup-reader-check-k256

## Goal

Make the owner design's statement about Claude Code's plugin-cache cleanup
true for the Claude Code version that the cutover uses.

## Context

- owner-inventory-reader-marker-k244 found that Claude Code 2.1.319's orphan
  cleanup reads `.in_use`. The section "Finding for the owner design" of
  `docs/owner-skill-delivery-evidence.md` records the evidence.
- The section "Resource lifetime" of `docs/specs/owner-skill-delivery.md`
  states that the cleanup is time-based and ignores readers. That statement
  is false for 2.1.319.
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
