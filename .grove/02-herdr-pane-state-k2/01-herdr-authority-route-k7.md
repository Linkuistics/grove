# herdr-authority-route-k7

**Kind:** planning

## Goal

Settle how grove gets a state report **accepted** by herdr for a pane whose
session identity is already owned by the harness's own herdr integration — the
gate measured in this node's brief. Everything `02-report-plumbing-k8` builds
depends on the answer, so this is a route decision, not an implementation.

## Context

Read the node `BRIEF.md` first — it carries the live measurements and the exact
`state.rs` line numbers. The short version: `current_session_owner_conflicts`
vetoes any report whose `(source, agent)` differs from the pane's session-identity
owner, and herdr's claude/codex integrations claim exactly that ownership at
every SessionStart while contributing nothing to state. No unforked report gets
through, and one that lands *before* the owner appears latches the pane at a
stale state forever.

*herdr-optional-ui* named this: the fork option was rejected "for now" and
pre-committed to reopen **"if that turns out to be false"**. It has. This leaf
spends that reopening.

The user owns a herdr fork already — `AntonyBlakey/herdr`, with
`ogulcancelik/herdr` as `upstream`, checked out at `~/Development/herdr` and
currently level with `v0.7.5` on the files that matter. That materially changes
the cost of the fork route versus what planning assumed.

## Done when

A route is chosen, with the human, and recorded where it belongs — an ADR if it
clears the when-to-write bar, otherwise the node brief. `02-report-plumbing-k8`
is then rewritten to match the chosen route, since its shape follows from it.

## Notes

**Grilling agenda.** Candidate routes, roughly cheapest-first. Each is a real
option; the recommendation is the fourth.

1. **Do nothing unforked; ship the reporter disabled by default.** Honest, no
   herdr change, no risk of latching. But it delivers nothing the root brief's
   "Done when" asks for, so it is really a decision to abandon this node and
   let `05-herdr-grove-plugin-k5` carry the whole integration.
2. **Win the ownership race.** Have the driver claim the pane's session identity
   as `grove`/`grove` *before* launching the first harness session. Measured to
   work on a fresh pane — and it then vetoes the harness's own SessionStart
   report, permanently. Rejected on sight unless the human wants it: it is
   hostile to another integration, it silently breaks herdr's claude-session
   resume for that pane, and it depends on the mis-detection bug persisting.
   Loses outright if the pane ran an agent before `grove do`.
3. **Ask the user to uninstall the harness integration** (`herdr integration
   uninstall claude`). Then nothing owns the session and grove's reports land —
   measured. Costs herdr's session-resume for that harness, and is configuration
   guidance rather than a design. Viable as a documented escape hatch, weak as
   the primary route.
4. **Fork + upstream PR** (recommended). Two shapes, and the grill should pick
   between them rather than assume:
   - *Narrow*: add `("grove", "grove")` to the `full_lifecycle_hook_authority`
     allowlist at `src/detect/mod.rs:283`. Smallest possible diff, matches the
     seven existing pairs, plausibly acceptable upstream. But it grants grove
     **full** authority, which also suppresses screen detection entirely
     (`should_ignore_detected_state_under_full_lifecycle_hook`) — decide whether
     that is wanted or merely tolerated.
   - *General*: fix the gate itself, so a session-identity owner does not veto a
     differently-sourced *state* report. Arguably the real bug — session identity
     and lifecycle state are separate concerns that herdr conflates here — and it
     helps every third-party reporter, not just grove. Larger diff, better
     upstream story, needs care not to regress the takeover tests around
     `state.rs:4519-4670`.

**Questions the grill has to answer, whichever route wins:**

- Does grove want full lifecycle authority (screen detection off) or only
  precedence over it? The mis-detection bug argues for off — herdr is evaluating
  codex's manifest against a claude TUI on every pane measured. But off means a
  grove bug shows as a frozen pane with no fallback.
- What clears grove's authority when the loop stops? Measured: `release_agent`
  works while grove is the effective label, but cannot dislodge a foreign
  session owner. A driver that reports must also un-report on exit, or every
  `grove do` leaves a latched pane behind.
- Fork-only, or upstream PR too? If upstream, the *general* shape is the one
  worth proposing; if fork-only, the *narrow* one is a two-line carry.
- Does the mis-detection itself need fixing (MCP servers inheriting the
  harness's foreground process group)? It is upstream of everything here and
  affects far more than grove — possibly its own leaf under the root, not this
  node's business.

**Scope guard**: this leaf decides a route and records it. Writing the reporter
is `02-report-plumbing-k8`; touching herdr's screen manifests or the plugin is
neither.
