# 020-design-seed-convention — brief

## Goal

Implement the agreed seed-capture convention: a shared coordination branch
(`grove-meta`) holds per-grove inbox files, observations are appended via
CLI verbs, and every grove session drains its own inbox at bootstrap. This
subtree delivers the ADRs that record the design, the SKILL/methodology
updates that make drain a bootstrap step, the CLI changes that enforce
process correctness, the branch rename and explicit `grove meta init`
verb, and a planning leaf for the TUI navigator.

## Done when

- Two ADRs exist under `docs/adr/`: the inbox model & `grove-meta` branch
  architecture; and the cross-repo inbox handoff rule.
- `content/SKILL.md` (the methodology source bundled into harnesses) names
  inbox drain as a bootstrap step, points at the `grove-meta` branch in
  its artifacts table, and reflects the glossary additions made during
  planning.
- The `grove` CLI materialises the `grove-meta` branch on `install`,
  performs drain on `grove start` and `grove continue`, exposes a capture
  verb (`grove inbox add`) for appending observations, and exposes an
  explicit `grove meta init` verb that creates/repairs the branch and
  worktree idempotently for repos that pre-date the feature or whose
  worktree has been removed.
- The TUI navigator has a planning leaf that has been grilled and either
  decomposed into work leaves or explicitly deferred (e.g. seeded for a
  future grove).
- All child leaves are retired into `done/`; this node retires when none
  remain live.

## The agreed model (in one screen)

- **Inbox** = markdown file at `<repo>/.grove-meta/inboxes/<name>.md` on
  the dedicated `grove-meta` branch.
- **Seed** = an inbox whose addressed grove has no worktree currently.
  Same file, same path; lifecycle state is the only distinction.
- **Capture**: LLM in any grove, finding an observation that belongs
  elsewhere, calls a `grove` CLI verb (not raw git/mv) to append to the
  appropriate inbox. Three destinations: new seed, existing seed, or a
  running grove's inbox — all identical write gestures.
- **Drain**: at every `grove start` and `grove continue`, the receiving
  grove's session triages its own inbox. For each entry: incorporate,
  defer, or reject (and possibly seed elsewhere). After triage the inbox
  file is committed empty; drained content lives in git history of the
  `grove-meta` branch.
- **Germination is a non-event**: when `grove start <name>` runs against
  a pre-existing seed, the seed file just becomes the new running grove's
  inbox at the same path. No migration, no consumption step.
- **Cross-repo**: a write from repo A to grove `Y` in repo B is a write
  to `<repo-B>/.grove-meta/inboxes/Y.md`. Requires B checked out locally
  with its `grove-meta` worktree present. Repo-path discovery is out of
  scope for v1.
- **The `grove-meta` branch is reserved more broadly** for shared
  cross-grove coordination data and repo-level metadata (grove-related
  or otherwise); inboxes are its first occupant, more may follow. The
  previously-considered name `grove-inboxes` was rejected because it
  narrowed the branch's scope to its first occupant.

## Decomposition

Six leaves, ordered to minimise blocking — ADRs first because they
record decisions the rest depends on; SKILL/glossary next because the
methodology must reflect the new bootstrap step before tooling is wired
to it; CLI implementation fourth; the branch rename + explicit
`grove meta init` verb fifth (settled during the planning that produced
the 060 leaf — see Notes); TUI planning last because its grilling
shouldn't gate the rest.

- `010-adr-inbox-model.md` — ADR for the inbox model + shared branch
  architecture (work, retired).
- `020-adr-cross-repo-handoff.md` — ADR for the cross-repo inbox rule
  (work, retired).
- `030-update-skill-and-glossary.md` — `content/SKILL.md` and the
  bundled-skill glossary updates (work, retired).
- `040-cli-implement-inbox.md` — `grove install` materialises the
  branch and worktree; `grove start`/`continue` drain; new
  `grove inbox` verb for capture (work, retired).
- `050-grove-meta-rename-and-init.md` — rename the branch from
  `grove-inboxes` to `grove-meta` (worktree at `<repo>/.grove-meta/`)
  across CLI, ADRs, SKILL, prompts, and docs; add explicit
  `grove meta init` verb (idempotent), invoked internally by
  `install`/`update` and externally for pre-feature repos or
  worktree repair (work).
- `060-tui-server.md` — planning task: grill TUI scope (initial target
  is medium — per-repo, filesystem-watch; multi-repo evolution is a
  later seed candidate) and decompose. Originally numbered `050`;
  renumbered when the rename leaf was inserted ahead of it so the
  TUI is designed against the final branch name (planning).

## Pointers

- Prior-art survey driving the design: `docs/research/seed-capture-prior-art.md`.
- Glossary terms in play (see `CONTEXT.md`): `Inbox`, `Seed`, `Drain`,
  `grove-meta branch` (renamed from the originally-considered
  `grove-inboxes branch`; the rejection rationale is recorded in the
  entry). `Germination` was considered and dropped — it reduces to
  `grove start`.
- ADRs to read: `docs/adr/0002-grove-meta-branch-and-inbox-model.md`
  (renamed by leaf 050; before leaf 050 lands, the file is still at
  `0002-grove-inboxes-branch-and-inbox-model.md`),
  `docs/adr/0003-cross-repo-inbox-handoff.md`. The existing
  `0001-install-and-update-create-commits.md` is unaffected.
- Methodology constraint that decided storage shape:
  `.claude/skills/grove/SKILL.md` constraint 6 (walk-away-able). Plain
  markdown files on a regular git branch survive tool loss.

## Notes

- **The convention is dogfoodable inside this very grove**: any "would be
  later" observation surfaced during a child leaf can itself be captured
  as a seed via the same convention being implemented. The TUI's
  multi-repo evolution is an obvious first instance.
- **No PRD**: the grilling did not reach a human-facing public-agreement
  moment beyond what the two ADRs record. If implementation surfaces a
  user-facing decision worth a PRD, the relevant leaf raises one.
- **No standalone convention doc**: the two ADRs + glossary entries +
  SKILL.md update together *are* the convention. Adding a separate
  `docs/seeds.md` would duplicate without earning its keep.
- **Why the rename came in mid-subtree (leaf 050)**: while planning the
  TUI grilling (the originally-`050` leaf), it became clear that the
  branch `grove-inboxes` lied about a scope ADR 0002 already declared
  ("reserved more broadly for cross-grove coordination data"). The
  branch had not yet shipped in any released version, so a clean
  rename with no migration code was possible. Doing it before the TUI
  is grilled means the TUI is designed against the final names. The
  same leaf adds the explicit `grove meta init` verb because the
  rename touches the same code and ADR — they belong in one focused
  edit.
