# ravel-history-migration-k16

## Goal

Make runnable Ravel use the owner's preserved configuration. Historical
continuity was explicitly rejected; the existing handle is retained.

## Context

Requires ravel-runtime-k15. Read its exact Ravel commit, current CLI docs and
root/ravel-cli-k3 contracts. Recheck source ~/.config/harness-dispatch and
destination ~/.config/ravel contents; prior inspection is not a lock.

## Done when

- Copy actual personal configuration, preserving source contents/destination
  edits, selection choices, bounds and grants. Relocate necessary SDK/caller
  imports. Remove obsolete destination storage settings; no history access.
- Typecheck the copied policy against Ravel's public SDK and validate applicable
  routing with temporary fake agents, using deterministic tools. Do not invoke
  a real model. The two-verb CLI has no inspect or init command.
- Document the selected configuration and validate that the new tool uses no
  old store. Open/copy/import no personal records; no snapshot or catch-up work.
- Commit project docs through jj in Ravel, record exact commit/validation here,
  retire, check/promote k3's complete contract and seal the coordination change.
  Preserve the old installed front and this session's original ending command.

## Notes

Publication/release preparation remain k4/k11. Source configuration and the
original driver remain intact. Any Grove-specific provenance is caller-owned;
Ravel offers no record lookup or durable store.
