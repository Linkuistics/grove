# 010-adr-inbox-model

**Kind:** work

## Goal

Write the ADR recording the inbox model and the `grove-inboxes` branch
architecture. The brief chain already states the agreed design; this
task captures *why* it was chosen — alternatives considered (per-grove
worktree inbox, default-branch storage, gitignored filesystem dir,
separate repo) and the trade-offs that picked this shape.

## Context

- `docs/research/seed-capture-prior-art.md` — Shortlist and
  Cross-cutting findings sections. The decision draws specifically on
  the gh-pages-style branch idea, the maildir "capture = drop a file"
  primitive, and the dangling-link semantics for naming.
- `CONTEXT.md` glossary entries `Inbox`, `Seed`, `Drain`,
  `grove-inboxes branch` — the ADR cites these rather than re-defining
  them.
- `docs/adr/0001-install-and-update-create-commits.md` — house style /
  format precedent.

## Done when

- `docs/adr/0002-grove-inboxes-branch-and-inbox-model.md` exists in the
  project's ADR style.
- The ADR covers: the inbox-as-shared-channel model, lifecycle parity
  (unborn / running / finished all addressable identically), the
  `grove-inboxes` branch as the storage mechanism, drain as a bootstrap
  step, and CLI-mediated access (no raw git for the LLM).
- The ADR explicitly names and rejects the four main alternatives:
  per-grove worktree inbox, default-branch storage, gitignored
  filesystem dir, separate repo for inboxes. One paragraph each.
- The ADR notes "walk-away-able" (SKILL constraint 6) as the binding
  constraint that ruled out paradigms storing items in opaque
  formats — git-bug, Fossil tickets, Taskwarrior, etc.
- No reference to `Germination` as a separate operation; the ADR states
  that `grove start` against an existing seed is the same code path as
  any session bootstrap.

## Notes

- The ADR is decision-grade prose, not a reference manual. Keep it
  focused on the *choice*; let `CONTEXT.md` and the SKILL.md update
  carry the definitions.
- The reservation of the branch for broader coordination data ("more
  may follow") should be noted but not over-specified — the branch's
  primary contract today is inboxes.
