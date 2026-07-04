# ADRs as a minimum coherent set — design seed

> **Status:** approved design, produced via a brainstorming session on 2026-07-04.
> **Purpose of this file:** seed for the `adr-coherent-set` grove. The first
> planning session should lift this into `docs/specs/2026-07-04-adr-minimum-coherent-set-design.md`
> on the grove branch, grow the task tree from the decomposition at the bottom,
> then drive the leaves. The design below is *decided* — this is an encode-and-execute
> planning pass, not a fresh grill. Re-open a decision only if execution surfaces a
> genuine contradiction.

## Motivation

ADRs currently accrete as an append-only chronology: sequential numbering, an
optional `superseded by ADR-NNNN` status, one new ADR per revised decision. The
result is design *history* masquerading as current documentation. grove's own
`docs/adr/` is the poster child — 35 ADRs, a large fraction of which document
dead designs (the zellij → trellis → rmux TUI saga, ADRs 0013–0030).

The new rule: **ADRs are a minimum coherent set describing the design's current
state.** This is grove's existing core tenet — *the tree is the only state; git
is the history* (SKILL.md constraint 1) — extended from the task tree to
`docs/adr/`. ADRs hold the present; git holds the past.

## The four settled decisions

1. **Placement.** The authoritative ADR philosophy lives in a new **linkuistics
   skill** (`../skills` repo), *not* bundled into grove. grove is allowed to
   **require the linkuistics plugin** as a prerequisite — self-containment is no
   longer a constraint. grove references the skill rather than carrying a copy.

2. **grove's ADR-FORMAT.md → a thin grove note.** It keeps only grove-specific
   placement conventions and defers philosophy/format/template to the linkuistics
   skill.

3. **Drop number-as-handle.** ADRs are identified and cited by **slug/title**,
   never by number. A sequential number *is* chronology — the exact history the
   new rule discards. Filenames become slug-only (`docs/adr/<slug>.md`).

4. **Full corpus rework now.** This grove also reworks grove's own 35-ADR
   `docs/adr/` down to a minimum coherent set — dogfooding the new rule — not just
   the guidance.

## Part 1 — new linkuistics skill: `decision-records`

Location: `../skills/plugins/linkuistics/skills/decision-records/SKILL.md`
(a peer of `codebase-design`, `cli-tool-design`). **Model-invoked** (no
`disable-model-invocation`). Follow `authoring-conventions`: description shape is
*capability + "Use when …"*; cite sources; progressive disclosure.

The skill is the single source of truth for the ADR philosophy:

- **Minimum coherent set** — as few ADRs as coherently describe the design's
  *current* state. Fewer, self-contained ADRs over many cross-referencing ones.
- **Current-state, not a changelog** — an ADR describes what *is* and why it
  binds, never the path taken to it. No `superseded by`, no decision history.
  **git is the history.**
- **Edit in place; merge / split / delete** to keep the set minimal and coherent
  as understanding changes. Reworking ADRs is normal maintenance, not an
  exceptional event.
- **Identity = slug / title, never a number.** A number encodes creation order,
  i.e. chronology — the thing we discard. Cite ADRs by slug/title.
- **Distill lessons learnt** — keep the *why*, the still-binding constraint, and
  the alternative rejected for a reason; discard the narrative of how the team
  arrived there.
- **When to write** — the 3-part test (hard to reverse · surprising without
  context · the result of a real trade-off). All three must hold. This lives here
  now (deduped out of grove's `ADR-FORMAT.md` and `grilling.md`).
- **Minimal template** — a title + 1–3 sentences (context, decision, why).
  Optional `Considered options` / `Consequences` only when they earn their place.
  No `Status: superseded` framing.

Provenance note: the surviving good material originates from
`mattpocock/skills` (MIT) — preserve attribution consistent with how grove's
bundled copies cite it.

Also update the linkuistics plugin manifest keywords / description if a new
skill warrants it (light touch).

## Part 2 — grove methodology prose (the `content/` files)

grove embeds `content/` into the binary and extracts it to
`~/.claude/skills/grove/`; these are the files a grove session reads. Changes:

- **`content/ADR-FORMAT.md` → thin grove note.** Keep: `docs/adr/<slug>.md`
  naming (slug-only, no number), per-context placement under `CONTEXT-MAP.md`,
  lazy creation, and the brief-chain curation rationale ("a session reads three
  ADRs, not fifty"). Remove: the template, the 3-part test, the numbering/superseded
  machinery. Add a pointer: *for the ADR philosophy, format, and when-to-write
  test, use the `linkuistics:decision-records` skill.*

- **`content/grilling.md`** — "Offer ADRs sparingly": collapse the duplicated
  3-part test to a one-line reference to the skill. Update the file-tree examples
  (`0001-event-sourced-orders.md`, `0002-postgres-for-write-model.md`) to
  slug-only names.

- **`content/SKILL.md`** — (a) add the **revisit-and-rework** behavior to the
  Plan and Retire steps: as understanding changes, rework the ADR set *in place*
  (merge / split / delete) to keep it minimal and coherent, and reconcile the
  BRIEFs; never append a superseding ADR. (b) Update the Artifacts table row for
  ADRs (`docs/adr/<slug>.md`; current-state, minimum coherent set). (c) Update
  the Reference-files list entry for `ADR-FORMAT.md`. (d) Convert this file's own
  `ADR-NNNN` self-citations to slug form (see reconciliation below). (e) Note the
  linkuistics-plugin prerequisite.

- **`content/driving.md`** — add a short field-guide subsection: reworking ADRs
  and BRIEFs as understanding shifts (edit in place, keep the set minimal, fix
  dangling citations). Align the existing "when research retires into ADRs"
  material with edit-in-place. Convert its ADR citations to slug form.

- **`content/TASK-FORMAT.md`, `content/BRIEF-FORMAT.md`,
  `content/prompts/continue.md`, `content/prompts/retire.md`** — convert
  in-prose `ADR-NNNN` citations to slug/title; align any ADR-related language
  (e.g. "raises ADRs sparingly") with the skill. `BRIEF-FORMAT.md`'s "read three
  ADRs not fifty" curation stays valid.

- **`README.md`** — declare the linkuistics-plugin prerequisite (grove now
  depends on `linkuistics:decision-records`).

The 6 known in-prose numbered citations to reconcile (grep `ADR-[0-9]` across
`content/`): `TASK-FORMAT.md:55` (ADR-0035 §5), `SKILL.md:76` (ADR-0034,
ADR-0035), `SKILL.md:78` (ADR-0032), `SKILL.md:94` (ADR-0011), `SKILL.md:180`
(ADR-0035 §5), `prompts/continue.md:1` (ADR-0035 §5). Each maps to a surviving
ADR's new slug (determined in Part 3). Re-grep after the corpus rework — do not
trust this snapshot.

## Part 3 — grove `docs/adr/` corpus rework

Rework grove's 35 ADRs into a minimum coherent set describing grove's *current*
design. Judgment-heavy; do it as its own leaf(s) with a review checkpoint.

**Method:**
1. Establish grove's *current* architecture from `README.md`, `content/SKILL.md`,
   `CONTEXT.md` (if present), and the live ADRs (0028–0035 and any others still
   describing current state).
2. Classify each of the 35: **keep** (describes current state → rename slug-only,
   edit to be self-contained and current-state), **delete** (superseded / dead →
   git holds it), or **merge** (a live lesson folded into a surviving ADR).
3. Surviving set = the minimum that coherently explains grove's current design,
   its still-binding constraints, and the lessons worth keeping. Expect the
   0013–0030 TUI tower to collapse dramatically.
4. Rename survivors to slug-only (`git mv NNNN-slug.md slug.md`).
5. Reconcile **every** citation to surviving slugs — across `content/`, `docs/`,
   `docs/research/`, `docs/workflows/`, and any BRIEFs. A merge/delete that leaves
   a dangling `ADR-NNNN` reference is a defect, not acceptable collateral.

**Checkpoint (required):** present the proposed keep / delete / merge
**disposition table for all 35 ADRs** to the human for approval **before** any
delete or merge. Pruning is judgment-heavy; a live constraint must not be
silently dropped.

## Cross-cutting

- **Two repos, two commits/PRs.** The `decision-records` skill lands in
  `../skills` (its own git repo); everything else in `grove`. Keep the commits
  separate — the grove worktree is a worktree of the grove repo; editing
  `../skills` is just editing another repo on disk, committed independently.
- **Slug uniqueness.** ADR slugs must be unique within a `docs/adr/` directory
  (they are now the handle).

## Risks / follow-ups (not blocking)

- **Release.** grove embeds `content/` in the binary; the prose changes reach
  installed users only after a rebuild + release (`scripts/release-*.sh`). Flag
  at Finish; the release itself is out of scope for this grove.
- **Plugin distribution.** The linkuistics skill reaches users when the skills
  plugin repo is published/updated — separate from grove's brew release.
- **Dependency is documentation-level**, not install-time enforced. Enforcement
  (grove checking the plugin is present) would be separate work; note it, don't
  build it.

## Proposed decomposition (for the planning session to grow the tree)

A suggested shape — the planning session finalizes it:

1. **`decision-records-skill`** (work) — author the linkuistics skill; commit in
   `../skills`.
2. **`grove-adr-note`** (work) — rewrite `content/ADR-FORMAT.md` to the thin note
   + update `grilling.md`'s duplicated test and file-tree examples.
3. **`grove-process-prose`** (work) — SKILL.md revisit/rework behavior, driving.md
   subsection, artifacts/reference-file updates, README prerequisite.
4. **`corpus-disposition`** (planning) — classify all 35 ADRs; produce the
   disposition table; get human approval at the checkpoint.
5. **`corpus-rework`** (work) — execute keep/delete/merge, rename to slug-only.
6. **`citation-reconcile`** (work) — reconcile all ADR citations across
   `content/` and `docs/` to surviving slugs; convert the 6 numbered in-prose
   citations. Do this *after* the corpus settles so slugs are final.

Leaves 5 and 6 depend on 4's approved disposition. Leaf 2/3's citation edits
should be finalized in 6 (or note the dependency) so nothing cites a slug that
the rework later changes.
