# adr-coherent-set — brief

## Goal
Reform how ADRs are kept: an ADR set is the **minimum coherent set describing
the design's current state**, not an append-only chronology. Extend grove's core
tenet — *the tree is the only state; git is the history* — to `docs/adr/`.
Ship the guidance **and** dogfood it by reworking grove's own 35-ADR corpus.

## Done when
- A `linkuistics:decision-records` skill owns the ADR philosophy/format (in `../skills`).
- grove's `content/` prose defers to it, gains the revisit-and-rework behavior,
  and declares the linkuistics prerequisite.
- grove's `docs/adr/` is reworked to a minimum coherent set, slug-named, with all
  citations reconciled.

## Decomposition
Grown from the approved spec into six execution leaves (run in this order — the
ordering encodes the dependencies):

1. `decision-records-skill-k2` (work) — author the linkuistics skill; commits in
   `../skills`. Independent; goes first so the grove prose can defer to it.
2. `grove-adr-note-k3` (work) — `content/ADR-FORMAT.md` → thin note; fix
   `grilling.md`. Points at k2.
3. `grove-process-prose-k4` (work) — `SKILL.md` revisit/rework behavior,
   `driving.md` subsection, `README.md` prerequisite. Points at k2.
4. `corpus-disposition-k5` (**planning**) — classify all 35 ADRs; **required
   human checkpoint** before any delete/merge. Gates k6 and k7.
5. `corpus-rework-k6` (work) — execute the approved keep/delete/merge; rename to
   slug-only. Depends on k5's approved table.
6. `citation-reconcile-k7` (work) — reconcile **all** ADR citations to final
   survivor slugs; convert the numbered in-prose citations (incl. those deferred
   from k3/k4). **Runs last** so slugs are final.

Dependency notes: k3/k4 defer their `ADR-NNNN`→slug conversions to k7 so nothing
cites a slug the corpus rework later changes; k6/k7 both wait on k5's approval.

## Pointers
- **Approved design spec (the mandate): `docs/specs/2026-07-04-adr-minimum-coherent-set-design.md`** — read this first; the design is *decided*.
- ADR philosophy target: the new `linkuistics:decision-records` skill.
- Corpus under rework: `docs/adr/` (35 ADRs).

## Notes
Four settled decisions (from the spec): (1) philosophy lives in a **linkuistics
skill**, grove may **require** the plugin — no self-containment constraint;
(2) grove's `ADR-FORMAT.md` becomes a **thin grove note**; (3) **drop
number-as-handle** — ADRs are cited by slug/title, filenames slug-only;
(4) **rework grove's corpus now**. Two repos, two commits: the skill lands in
`../skills`, everything else in `grove`.
