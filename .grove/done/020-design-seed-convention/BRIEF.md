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

Nine leaves, ordered to minimise blocking. The retired four (010–040)
delivered the design's first pass. The live five (050–090) absorb the
four foundational concerns the original decomposition did not
anticipate — in the order needed to keep each downstream leaf
unblocked.

- `010-adr-inbox-model.md` — ADR for the inbox model + shared branch
  architecture (work, retired).
- `020-adr-cross-repo-handoff.md` — ADR for the cross-repo inbox rule
  (work, retired).
- `030-update-skill-and-glossary.md` — `content/SKILL.md` and the
  bundled-skill glossary updates (work, retired).
- `040-cli-implement-inbox.md` — `grove install` materialises the
  branch and worktree; `grove start`/`continue` drain; new
  `grove inbox` verb for capture (work, retired).
- `050-research-in-repo-issue-trackers.md` — survey prior in-repo
  issue-tracker / annotation systems (git-bug, Fossil tickets,
  bugs-everywhere, ditz, ticgit, artemis, git-appraise, Radicle COBs,
  Sapling/jj metadata, plus baseline TODO-comment and changelog
  conventions) for *failure modes*. Produces
  `docs/research/in-repo-issue-tracker-postmortems.md` with a
  per-system section and a synthesis section answering specific
  questions each downstream planning leaf needs (work).
- `060-sync-semantics-and-inbox-shape/` — was a planning leaf;
  decomposed into a node holding the agreed sync/shape decisions
  (`BRIEF.md`) plus three child work leaves. Decisions landed in
  ADR-0004 (shape) and ADR-0005 (sync) and the post-mortem research
  doc's "Findings adopted" section. Children: `010-cli-shape-and-capture.md`,
  `020-cli-drain-as-verb-and-bootstrap-fetch.md`,
  `030-cli-meta-remote-and-sync.md` (work — all three retired
  2026-05-28; node itself retired in the same session as 030).
- `070-grove-meta-rename-and-init.md` — rename the branch from
  `grove-inboxes` to `grove-meta` (worktree at `<repo>/.grove-meta/`)
  across CLI, ADRs, SKILL, prompts, and docs; add explicit
  `grove meta init` verb (idempotent), invoked internally by
  `install`/`update` and externally for pre-feature repos or
  worktree repair. The shape-change-coupling hedge from before 060
  landed is now resolved: ADR-0004's directory-of-files shape has
  shipped, so 070 sweeps the rename across the already-current shape
  (no shape change to bundle). The `grove meta` parent subcommand
  also already exists (introduced by 060/030 for `remote` and
  `sync`); 070 adds `init` as a sibling variant (work).
- `080-audit-llm-cli-boundaries.md` — planning task: audit
  `content/SKILL.md` and `content/prompts/*.md` for deterministic
  prose-coded steps that should be promoted to CLI verbs the LLM
  invokes; output a classified inventory and placement
  recommendations for each promotion. The featured example, drawn
  from this very subtree's planning history, is `grove leaf insert`
  for inserting a leaf at a given position with automatic renumbering
  of subsequent leaves (planning).
- `090-tui-server.md` — planning task: grill TUI scope (initial
  target is medium — per-repo, filesystem-watch; multi-repo
  evolution is a later seed candidate) and decompose (planning).

## Pointers

- Prior-art survey driving the design: `docs/research/seed-capture-prior-art.md`.
- Post-mortem survey of in-repo issue trackers (de-risks downstream
  planning leaves; cites concrete prior failures for each):
  `docs/research/in-repo-issue-tracker-postmortems.md`.
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
- **Why the leaves are numbered the way they are**: the subtree grew
  during a single planning session that surfaced four foundational
  concerns the original decomposition did not cover. Each was
  captured as a leaf when it surfaced; the numeric prefixes carry
  the resolved order, not the order of capture. The grilling session
  rolled the renumber by hand each time — explicit acknowledgement
  that this insert-and-renumber pattern is itself a strong CLI verb
  candidate (called out as the featured example in leaf 080).
  - The TUI leaf was originally numbered `050`.
  - When the rename concern surfaced (the branch `grove-inboxes` lied
    about the scope ADR 0002 already declared, and the branch had
    not yet shipped, so a clean rename with no migration code was
    possible), a rename + `grove meta init` leaf was inserted ahead
    of TUI. TUI moved to `060`. The rename leaf bundled the init
    verb because both touch the same code and ADR — one focused edit.
  - When the multi-machine sync concern surfaced (today's CLI does
    no fetch/push on `grove-meta`; multi-writer state is silently
    broken), a planning leaf for sync semantics + inbox shape was
    inserted ahead of the rename. Shape may change (single file vs
    directory of observation files for ff-conflict-less pushes); if
    so, the rename's ADR rewrite covers the shape change too. Rename
    moved to `060`, TUI moved to `080`.
  - When the prose-vs-CLI audit concern surfaced (deterministic
    LLM-driven steps in `SKILL.md` and the launcher prompts should
    be CLI verbs), a planning leaf was inserted between the rename
    and the TUI. It does not gate the rename — the rename's scope
    does not intersect the audit's reclassifications — so audit
    landed between rename and TUI rather than ahead of rename.
  - When the in-repo-tracker post-mortem research concern surfaced
    (prior tools have tried very similar architectures; their
    failure modes inform every downstream planning leaf), a research
    work leaf was inserted ahead of every live leaf — the research's
    findings de-risk the sync grilling, the audit grilling, and the
    TUI grilling. Each subsequent leaf shifted up by ten: sync to
    `060`, rename to `070`, audit to `080`, TUI to `090`.
- **The pause point.** This subtree has now absorbed every concern
  surfaced in the planning conversation that produced it. Subsequent
  additions, if any, should be deferred to a follow-up planning session
  (or captured as seeds) rather than continuing to extend this brief —
  the rolling-renumber overhead is what the audit leaf's featured CLI
  candidate exists to address; doing more by-hand renumbers before
  that candidate is implemented would be self-defeating.
- **060 outcome (2026-05-28).** The 060 planning leaf was grilled and
  decomposed in-place into `060-sync-semantics-and-inbox-shape/`
  (BRIEF + three child work leaves). The agreed model is recorded
  in ADR-0004 (shape — directory-of-observation-files) and ADR-0005
  (sync — local-first, opt-in remote, fetch-before-drain,
  push-best-effort with one auto-retry). The post-mortem research
  doc (`docs/research/in-repo-issue-tracker-postmortems.md`) gained
  a "Findings adopted" section pointing forward at the ADRs. A new
  bundled reference file `content/driving.md` captures the
  field-guide habits (commissioning research, grilling moves,
  WDYT/pushback discipline) so future groves repeat the pattern.
- **060 fully retired (2026-05-28).** All three children
  (010-cli-shape-and-capture, 020-cli-drain-as-verb-and-bootstrap-fetch,
  030-cli-meta-remote-and-sync) shipped and retired in the same
  session as the node itself. The `grove inbox add|drain|show` and
  `grove meta remote add|remove|list` / `grove meta sync` verbs now
  exist; capture writes per-observation files, drain is a two-phase
  CLI verb, and multi-machine users can opt into remote tracking
  with `grove meta remote add <url>` + cron `grove meta sync`. Leaf
  070 inherits a working `grove meta` parent subcommand and only
  needs to add `init` (plus the branch rename sweep).
