# ravel-cli-k3 — brief

## Goal

Deliver the independent Ravel process manager/router, its two-verb CLI and
matching private worker, using the owner's copied configuration.

## Context

Reuse `/Users/antony/Development/Ravel` and Linkuistics/Ravel. The old project is
preserved under `retired/actor-daemon-2026-10-12`; read `docs/retired-project.md`.
Source front/worker and build/install instruments remain in this Grove checkout
under `crates/harness-dispatch`; Ravel already owns its local runner copy.
The root brief and Ravel's `docs/session-cli.md` carry the human's current scope.
They replace the original history/store and broader CLI requirements.

## Done when

- Retired revision and owner README edit remain remotely retrievable. The active
  tree contains only replacement sources/documentation. Advance ordinary jj
  descendant commits; development version is 0.0.0, first minor cut is 0.1.0.
- Ravel's root Taskfile builds/checks/installs a matching front/compiled worker.
  Preserve pinned Bun/dependencies, source identity pairing, hostile probes,
  applicable notices and local runner coverage. Direct and symlinked bin/ravel
  work from a temporary prefix with no host runtime or Grove.
- Public verbs are only start-session and end-session RESULT. Routing passes the
  caller's prompt unchanged. Ending uses the enclosing process context; after
  termination/reaping start-session returns the opaque supplied value. Caller
  result semantics, end-session instructions and durable state stay caller-owned.
- Remove the SQLite store/dependency, record/observation/export/lookup features,
  historical migration fixtures and store-related SDK/worker requests. Ravel
  creates no persistent run history, including through explicit old storage
  settings. Preserve generic policy/context/routing features needed for launches.
- Generic SDK/examples/sample and CLI/inspection-free output contain no embedded
  Grove adapter, task conventions or caller-specific fields. Rewrite a useful
  generic routing sample. Enumerate/classify caller references across the active
  tree including dotfiles; use the spine's positive/cross-tree controls. Retired
  source documentation and authority-scrub names are explicit exceptions.
- k16 copies the actual ~/.config/harness-dispatch configuration into
  ~/.config/ravel, preserving source/destination edits and owner choices. Change
  necessary imports and remove obsolete destination storage settings. Typecheck
  against the shipped SDK and validate without launching models. No store copy,
  snapshot, importer or catch-up is part of this work.
- Transfer and reconcile executable spec, generic process/runtime ADR content
  and Bun evidence/instruments. Keep source citations and pinned-runtime upgrade
  controls attached to their real implementation. Grove copies remain until k12.
- Existing real-worker/fake-agent seams cover routing, opaque result round trips,
  empty/multiline values, nested/stale context, cancellation/reaping, terminal
  cleanup and confinement. Remove cases solely for discarded features. Tests
  use temporary configuration/channels and scrub all live authority variables.
  Verify the session's real channel stays absent before the final signal.
- Deterministic Taskfile checks save full logs and return concise summaries;
  run focused checks while editing and the full relevant suite at handoff.

## Public rename contract

| Surface | Contract |
|---|---|
| Front / verbs | ravel start-session; ravel end-session RESULT |
| SDK import | ravel/sdk; generic routing/context only |
| Private layout | libexec/ravel/ravel-policy, resolved from the front's real path |
| Configuration | ~/.config/ravel/{policy.ts,settings.json}; .ravel-choice |
| Session completion | One opaque result argument, returned to the start-session caller after reaping |
| Process context | end-session discovers its enclosing session; no explicit target argument |
| Prompt | Caller text is unchanged; caller supplies ending instructions |
| Persistence | None; no state directory, record verbs or historical lookup |

Preserve applicable selection/context/confinement parameters, with no obsolete
storage or third-verb compatibility surface. The original installed dispatch
remains usable for the live driver; keep its old ending command and guards.

## Decomposition

1. ravel-retired-project-k14 preserves the old revision and retrieval docs.
2. ravel-runtime-k15 delivers the smaller generic pair, docs and installed checks.
3. ravel-history-migration-k16 now copies configuration only; its handle is
   retained, but the human explicitly removed historical continuity.

k4 requires the complete runnable/configured handoff. No package release here.

## Decisions (running log)

**Human scope correction during k18.** Historical continuity is not useful.
Ravel is a process manager/router with exactly start-session and end-session;
the latter's opaque argument becomes the former's result. The caller supplies
ending instructions and owns result semantics/provenance. Removed the general
importer prototype, Python toolchain, history migration and Ravel prompt-injection
work. Retain process/runtime mechanisms and efficient existing test seams.

## Testing runner

The human selected cargo nextest. Rust test tasks use nextest and preserve its
reporting UI; formatting/lint diagnostics may be saved with short summaries.
Use focused filters during edits and the full relevant suite at handoff. The
nextest runner replaces cargo test for runtime suites; retain doctest checks
when the source contains executable documentation examples.
