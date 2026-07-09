# prd-to-spec-k4

**Kind:** planning

## Goal

Grill and decide: reframe grove's **PRD** as a **spec** — the user's rationale
(agreeing with upstream `to-spec`) is that what grove's planning increments
produce is *not* really a PRD. Land the decision (docs + methodology edits, or
follow-on work leaves), reworking the artifact story coherently.

## Context

- grove today: `docs/prd/` = "human-facing agreement checkpoints; committed,
  never retired" (SKILL.md artifacts table + PRD section); **and separately**
  `docs/specs/*-design.md` = "workstream-level technical design". A rename
  must reconcile these two rows — one artifact kind, two, merged?
- Upstream `to-spec/SKILL.md` (see report §G5): synthesises the conversation
  into a spec; explicitly "Do NOT interview the user" (grilling already
  happened); **sketches the test seams as a planning output** — "Use the
  highest seam possible… the ideal number is one. Check with the user that
  these seams match their expectations."
- Seam-sketching is folded into THIS leaf (plan-k1 decision): if grove's
  planning flow produces a spec, naming the agreed test seam belongs in it.

## Questions to grill

1. Is it a rename (`docs/prd/` → merged into `docs/specs/`?) or a reframe
   (grove keeps two artifact kinds with sharper names)? What happens to
   existing PRDs in consuming repos (migration? leave in place?).
2. What does grove's spec *contain* — the agreement checkpoint content plus
   named test seams? Adopt to-spec's "don't re-interview; synthesise" rule?
3. Which files change: `content/SKILL.md` (PRD section + artifacts table +
   loop prose), `grilling.md` (MAY-write-a-PRD line), glossary entry,
   launcher prompts? An ADR, or is this below the when-to-write bar?

## Done when

Decisions recorded (running log → durable docs), tree grown with the
implementing work leaf(s) or the edits done inline if they fit the session.

## Notes

Clean-cutover prose discipline applies: describe the new scheme on its own
terms; don't carry "formerly PRD" contrast through the docs (git holds
history).

## Decisions (running log)

### Facts established by exploration (not asked)

- `docs/prd/` has **never existed** in grove's own repo
  (`git log --all --diff-filter=A -- 'docs/prd/*'` is empty).
- It *is* used downstream, and every instance is a technical spec:
  `Stylepack/docs/prd/0001-stylepack-mvp.md` (Architecture overview, Pydantic
  v2 schemas, extraction strategy, build decomposition);
  `AppSpec/docs/prd/2026-06-26-appspec-toolkit.md` (boundary, substrate
  reconcile, decomposition, cross-grove seeds); `KeyEveryware-JSC/docs/prd/`
  (empty).
- The collision is **live**: AppSpec carries both `docs/prd/` and
  `docs/specs/{2026-04-18-app-spec-design,2026-06-27-spec-format-design}.md`.
- grove's own `docs/specs/` holds two files, both approved-by-human planning
  outputs of the finished `adr-coherent-set` grove — i.e. grove's specs already
  *are* its PRDs. One (`2026-07-04-adr-disposition.md`) declares itself "the
  executable input for `corpus-rework-k6` and `citation-reconcile-k7`".
- Naming is already three conventions: `SKILL.md` promises `*-design.md`;
  reality is `2026-07-04-adr-disposition.md`, `0001-stylepack-mvp.md`,
  `2026-06-26-appspec-toolkit.md`.

### Q1 — one artifact kind, or two? → **one: the spec**

Merge. The PRD row is deleted; `docs/specs/` is the single artifact a planning
increment produces at a genuine agreement point. Evidence: five real documents
exist across four repos and **none** honours the table's "human-facing
what/why" vs "workstream-level technical design" split — the distinction lives
only in `SKILL.md`, because no step ever had to test it. Upstream `to-spec`
reaches the same conclusion outright ("a spec (you may know this document as a
PRD)"). Accepted cost: grove no longer names a product-requirements artifact
for a cross-functional team; under constraint 4 such a repo adds `docs/prd/`
when it earns its place, without grove blessing a row it has never used.

### Q2 — spec lifecycle → **current-state, a minimum coherent set**

`docs/specs/` is the fewest specs that coherently describe the design's current
state — edited, merged, split in place; deleted when a spec describes nothing.
Same rule as `docs/adr/`, one grain coarser. It does **not** inherit the PRD
row's "committed, never retired": that clause is the ADR disease, and grove's
own two specs are already dead under it (an executed keep/delete/merge table,
and a design seed whose content is now promoted into
`linkuistics:decision-records`, `driving.md`, and the ADR set).

Two rules fall out:

- **Membership test.** *Would a session on an unrelated future grove need to
  read this?* Yes → a spec. No → it is a `BRIEF.md`, and it dies with `.grove/`.
  (`adr-disposition.md` self-describes as "the executable input for
  `corpus-rework-k6`" — that is a brief's job description.)
- **Grain rule vs ADRs.** An ADR records *one decision and its trade-off*; a
  spec describes *how an area works*. A spec **cites** ADRs, never restates them
  — otherwise the two sets disagree and neither binds.

Objection weighed: an agreement checkpoint you later edit is no longer what was
signed. Accepted — grove already made this trade for ADRs. The agreement is an
*event* (git holds it); the spec is the *design* (the tree holds it). A document
that lies about the present is worse than one that lost its provenance.

`Ephemeral — dies with the grove` was falsified by evidence, not preference:
AppSpec's toolkit doc scopes **cross-grove** seeds, so deleting it at one
grove's finish would destroy live input to another.

### Q3 — filename convention → **`docs/specs/<slug>.md`**

Slug-named; no date prefix, no `-design` suffix. The ADR convention one grain
up. A date orders a set by *when it was written*, which is meaningless for a set
describing *what is true now* — the argument that retired `0001-` from ADRs — and
a merged spec has two dates and room for one. Collapses the three live
conventions (`2026-07-04-adr-disposition.md`, `2026-04-18-app-spec-design.md`,
`0001-stylepack-mvp.md`) into one. `docs/workflows/finish.md` already writes it
this way in its own example commit (`docs/specs/rate-limiting.md`) two lines
after promising `*-design.md`.

### Q4 — cutover reach → **dogfood grove; leave downstream; CHANGELOG carries it**

- **Delete** `docs/specs/2026-07-04-adr-disposition.md` and
  `…-adr-minimum-coherent-set-design.md`. Verified dead: no file cites either;
  the rework landed (8 live ADRs); their content is promoted into
  `content/ADR-FORMAT.md`, `content/SKILL.md`, `content/driving.md`, and
  `linkuistics:decision-records`. Under Q2, a spec that describes nothing is
  deleted and git holds it.
- **Leave** `AppSpec/docs/prd/`, `Stylepack/docs/prd/`,
  `KeyEveryware-JSC/docs/prd/` in place — constraint 5 (grove guides, it does
  not gate). They migrate when a grove next drives those repos.
- **Propagation** is by an actionable `### Breaking` CHANGELOG entry naming the
  old directory, the new location, and the membership test. This is exactly how
  v9.0.0 shipped the ADR philosophy break *and* the `NNNN-` → slug rename, with
  no migration code.

Facts that decided it: `grove migrate` operates only on `.grove/`
(`src/tree_migrate.rs`); **no grove code reads `docs/prd/` or `docs/specs/`** —
sessions do. The `.grove/` migration exists because `pick`/`resolve` machine-read
that format and break on the old one; a docs directory read by nothing does not
earn the same machinery. Downstream *agents* cannot miss the new rules (the
binary re-extracts `content/` to `~/.claude/skills/grove/` on every `grove do`);
what they lack is a pointer from the orphan `docs/prd/` to its replacement, and
that is a five-minute human job across three repos, whose hard part — the
membership test — is judgement and not automatable anyway.

### Q5 — spec contents → **adapted `to-spec`, minus the decomposition**

Suggested shape (sections earn their place, constraint 3/4):
`## Problem` · `## Solution` · `## Decisions` · `## Test seams` · `## Out of scope`.

Three rules, adopted from `to-spec`:

- **Synthesise, never re-interview.** The grilling *is* the interview and it
  already happened; the spec synthesises the running decision log. A session that
  writes a spec by re-asking questions is running grilling twice.
- **Behavioural, not procedural.** No file paths, no line numbers, no code —
  they go stale. *Exception:* a prototype snippet that encodes a decision more
  precisely than prose can (state machine, schema, type shape). Stylepack's
  Pydantic schemas are that exception used correctly. Same rule G4 wants for
  briefs.
- **Cite ADRs, never restate them**, and use `CONTEXT.md`'s vocabulary. This is
  Q2's grain rule made operational.

Upstream's mandated "LONG, extremely extensive" user-story list is **dropped** to
optional: constraint 4, grove's domain has one actor, and an exhaustive
ceremonial list is what `driving.md`'s anti-patterns already reject.

**No `## Decomposition` section.** It is brief material — consumed by this
grove's leaves, dead when `.grove/` is deleted. All four real specs carry one
because all four were partly serving as briefs; that is Q2's confusion showing up
as a section rather than as a whole file.

### Q6 — where the shape lives → **`content/SPEC-FORMAT.md`**

A fifth reference file, symmetric with `BRIEF-` / `TASK-` / `CONTEXT-` /
`ADR-FORMAT.md`. `SKILL.md`'s `## Specs` section shrinks to the lazy rule plus a
pointer. Reasons: constraint 7's one-page budget; reference files are read
*lazily*, and a session needs the shape only when it is about to write a spec.

Decisive: `docs/adr/self-extension-core-and-methodology.md:10` already claims the
methodology carries "the CONTEXT / ADR / **PRD** format guides" — and no PRD
format guide has ever existed (`content/` ships four: ADR, BRIEF, CONTEXT, TASK).
The artifact was named in the ADR, given a row in the artifacts table and a
section in `SKILL.md`, and **never given a shape**. That absence is the direct
cause of five downstream files carrying three conventions. Writing
`SPEC-FORMAT.md` makes the ADR's line true for the first time.

### Q7 — seam-sketching → **spec section + `BRIEF.md` for the no-spec case**

- `SPEC-FORMAT.md` gains `## Test seams`, which **cites
  `linkuistics:codebase-design`** (it owns "placed at a clean seam, testable
  through it — Ousterhout depth + Feathers seams") rather than re-explaining
  seams. Same division of labour `ADR-FORMAT.md` has with `decision-records`:
  grove keeps placement, the skill keeps philosophy.
- Specs are lazy, so most increments write none. The agreed seams then go in the
  node's **`BRIEF.md`** (Pointers/Notes) — the brief chain is exactly how a
  node's settled design reaches its child work leaves, and it binds them without
  a new artifact. It dying with `.grove/` is correct: once tests exist at the
  seam, the tests are the record. Applies only when the increment covers code
  that will be tested.
- **"Check with the user that these seams match their expectations"** is a
  *grilling move*, not a document section → `grilling.md`, under "During the
  session".
- Rejected: seams only in specs (the enrichment would be dead on arrival);
  seams in each work leaf's `Done when` ("the ideal number is one" is a
  *cross-leaf* constraint — per-leaf choice is the proliferation it prevents).

### Q8 — ADR? → **no; correct `self-extension-core-and-methodology` in place**

Scores 2/3 on the when-to-write test. Surprising: yes. Real trade-off: yes (we
traded "what was signed" provenance for "the document does not lie"). Hard to
reverse: **no** — a prose convention, reversible by editing prose, every deletion
held by git. And "specs are current-state" is constraint 1 applied, which
`driving.md` already generalises to `docs/adr/`; extending it to `docs/specs/` is
consistent application, not surprise.

Two supports. **Precedent:** v9.0.0 was a strictly larger methodology break (ADR
philosophy externalised to the `linkuistics` plugin; `NNNN-` → slug) and added
*zero* ADRs — it edited an existing one and shipped a `### Breaking` CHANGELOG
entry. The last ADR *added* was `model-per-task-kind`, a CLI/loop behaviour
change. **Structure:** all 8 ADRs describe *grove-the-tool's* design; not one
records a *methodology rule* — those live in `content/`.
`self-extension-core-and-methodology` is the seam between the two, which is why
its line 10 (`CONTEXT / ADR / PRD format guides` — naming a file that has never
existed) is the one to fix.

### Q9 — inline or work leaf? → **inline, this session** (user's call)

Recommendation was a `[work]` leaf inserted at 05 (kind purity — mechanical prose
on the planning model is what `model-per-task-kind` exists to prevent; one
task = one focused commit). User chose inline, which this leaf's own `Done when`
explicitly permits. Consequence: one commit carries both the decision record and
the cutover.

## Change set (12 files)

**New** — `content/SPEC-FORMAT.md`.

**Edit (`content/`, the shipped methodology)**
- `SKILL.md` — mermaid node; constraint 4; Execute prose; artifacts table (drop
  the PRD row, rewrite the spec row); `## PRDs` → `## Specs`; reference-file list.
- `TASK-FORMAT.md` — the two PRD mentions.
- `BRIEF-FORMAT.md` — the `*-design.md` pointer glob; optional test-seams item.
- `grilling.md` — the seam-agreement move.

**Edit (repo docs)**
- `docs/concepts.md` — `## PRD` → `## Spec`.
- `docs/grove.md` — the PRD sentence.
- `docs/workflows/finish.md` — `docs/specs/<area>-design.md` → `<slug>.md`.
- `docs/adr/self-extension-core-and-methodology.md` — `PRD` → `SPEC`.
- `CONTEXT.md` — a **Spec** glossary entry.
- `CHANGELOG.md` — `### Breaking` entry under a new `## Unreleased` heading.

**Delete** — `docs/specs/2026-07-04-adr-disposition.md`,
`docs/specs/2026-07-04-adr-minimum-coherent-set-design.md`.

## Two departures from the change set, recorded

- **`content/BRIEF-FORMAT.md:41`** read `docs/adr/NNNN-*.md` — the same v9.0.0
  ADR-naming leftover as `concepts.md`, but *inside the exact Pointers block* this
  leaf rewrites. Corrected to `docs/adr/<slug>.md` rather than left two lines above
  a fresh `docs/specs/<slug>.md`, where it would have read as deliberate. Not a
  scope grab: a sweep found only two live instances, and the other one
  (`docs/concepts.md:55`) is externalized to `concepts-adr-refresh-k8`.
- **`CHANGELOG.md` gains a `## Unreleased` heading**, a convention the file has not
  had (entries have always landed at release, as `docs(changelog):` commits). The
  break is recorded while the reasoning is fresh rather than reconstructed at
  release time — `driving.md` names that reconstruction as an anti-pattern. The
  release cut renames the heading.

**Verified:** `cargo build` clean; 350 tests pass across 19 suites (exit 0).
`content/` is embedded with `include_dir!`, so `SPEC-FORMAT.md` ships with no
file-list to update.

## Surfaced, externalized (not absorbed)

`docs/concepts.md`'s **ADR** section is stale in three ways, all v9.0.0 leftovers
unrelated to this leaf: it says ADRs are `NNNN-slug.md` "numbered sequentially"
(they are slug-named); it points at `content/ADR-FORMAT.md` as "grove's preferred
shape" (now only a placement note); and it restates the when-to-write test that
now lives in `linkuistics:decision-records`. → its own leaf.
