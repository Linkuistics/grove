# 050-tui-server

**Kind:** planning

## Goal

Grill the design of a TUI navigator for the grove system — a UX layer
over the durable state (groves, inboxes, briefs, ADRs) defined by the
rest of this subtree. Initial scope target is **medium**: a per-repo
TUI with filesystem-watch refresh. The multi-repo evolution and any
"server" semantics are explicitly deferred — captured as a seed via the
very convention this subtree implements, as the first dogfooding
instance.

## Context

- The 010 and 020 ADRs (once written) define the state the TUI reads.
- The 030 SKILL update names the user-facing artifacts. The TUI's job
  is to make those navigable; it must not invent new state.
- `docs/research/seed-capture-prior-art.md` is **not** load-bearing
  here — none of the surveyed paradigms address visualisation. The TUI
  design space is unconstrained except by:
  - Walk-away-ability: the TUI is a reader/writer over the same files
    `ls` and `grep` see. If it disappears, nothing is lost.
  - Process correctness: every mutation goes through `grove` CLI verbs
    (the convention enforced by 040), not direct file edits.

## Done when

The grilling has settled enough that:

- Scope is fixed at **medium** (per-repo, filesystem-watch refresh,
  exit-when-you-exit). Server semantics (long-running, cross-repo,
  JSON-RPC for editor integration) are explicitly deferred.
- Stack choice is recorded (Ratatui is the default proposal unless the
  grilling surfaces a reason against). Rust-native, so it composes with
  the existing CLI crate without a runtime split.
- The view model is sketched: which entities appear (groves, leaves,
  inboxes, ADRs, briefs), what navigation moves exist, what (if any)
  write actions are exposed (likely just delegated drain/capture via
  the same CLI verbs).
- The multi-repo extension is **captured as a seed** at
  `<repo>/.grove-inboxes/inboxes/<follow-up-grove-name>.md` (via
  `grove inbox`, dogfooding the convention) with a short description of
  what the future grove would tackle: cross-repo navigation, a server
  daemon, editor integration, etc.
- This leaf is replaced by `050-tui-server/` with `BRIEF.md` and child
  leaves covering the v1 implementation. Or — if the grilling shows v1
  is small enough — by a single `050-tui-implement.md` work leaf.

## Notes

- **This leaf intentionally tests the convention before the rest of the
  subtree's child leaves are even retired.** The expected gesture, when
  the user says "yes the multi-repo / server piece is later", is for
  the LLM to run `grove inbox tui-multi-repo` (or similar) — the very
  CLI verb 040 is delivering. If 040 is not yet implemented at the time
  this leaf runs, the grilling notes the seed *content* in the brief
  and the seed write happens once 040 lands.
- The TUI is a navigation aid, not a tracker. Resist temptation to
  bolt on issue-status fields, completion percentages, or anything that
  would make the TUI's view drift from what's on disk. The grove
  philosophy is "what `ls` shows is the only state"; the TUI must obey.
- Server semantics are tempting because of LSP-style editor
  integration. The research artifact notes that as a potential
  follow-up. It is correctly deferred — the deferral is itself an
  example of seed capture.
