# driving-original-scheme-example-k22

**Kind:** impl

## Goal

Reconcile `content/driving.md`'s worked research-leaf example, which names its
work items in the removed original scheme and by bare position.

## Context

`content/driving.md:55-61` cites `050-research-in-repo-issue-trackers.md` and
four downstream leaves as `060, 070, 080, 090`, plus a *"Synthesis for 060"*
section title. Every one of those is a three-digit, keyless original-scheme name
or a bare position — the two forms ADR *task-tree-scheme* §5 forbids in prose,
and `stale-module-headers-k14` already swept out of `src/` on exactly that
ground. This is **provisioned** methodology content: the binary sweeps `content/`
into every installed harness's global skill dir, so a session reads it and learns
a grammar the tool no longer writes.

Two things make this harder than `retire-help-node-path-k20`'s one-token fix, and
they are the actual work.

1. **The referents are dead.** That grove's `.grove/` was deleted at its finish
   cycle, so there is no current path to renumber to and no key to recover from
   the tree — `resolve` cannot help. The names survive only in commit messages,
   which is the case §5's rule was written for.
2. **The citation is the example.** The passage exists to be a *worked* example
   of when to insert a research leaf, and its force comes from the sequence — the
   leaf landing after the first ADRs and before four named downstream leaves. Cut
   the names and the paragraph becomes the abstract advice the bullets above it
   already give. So the choice is between recovering the handles from history,
   re-describing the sequence without positional names, or replacing the worked
   example with a live one; decide which, do not default.

## Done when

- No original-scheme or bare-position work-item reference remains in
  `content/driving.md`, and the worked example still earns its place (or is
  deliberately replaced, with the reason stated).
- The rest of `content/` is checked for the same class by grep, not by file list
  (`content/TASK-FORMAT.md:162` and `docs/specs/task-kind-taxonomy.md:223` are
  **not** instances — they quote `01-design` / `02-review` as the *rejected*
  shortening, where keylessness is the point).
- `cargo test` passes (the provisioning tests hash `content/`).

## Notes

Decided: **option 2, in the form `docs/driving-a-grove.md` already established**
for a different dead workstream — bare slug or subject, pointers anchored on
durable artifacts, plus a predates-the-scheme note. Option 1 is impossible (that
grove is pre-key; its commit messages are keyless too), option 3 rejected because
*any* `.grove/` is private and mortal, so a live example regenerates the defect.
The worked example keeps its force: the sequence survives as a relation, and the
four downstream leaves are now named by subject, read off the surviving survey's
`## Synthesis` headings. Class fix in ADR *task-tree-scheme* §5 + `CONTEXT.md`.
Full carry-forward in the root brief; `src-position-citations-k24` externalized.
