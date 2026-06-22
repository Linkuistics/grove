# 11-[23]-scheme-v2-directories — brief

**Kind:** planning (node)

## Goal

Reverse ADR-0033's flat-filename structure to **real directories** (ADR-0035),
keeping its durable ideas (permanent `[key]`, reference-by-key, numeric order,
renumber-on-reorder, done-in-place). A node becomes a directory holding a
`BRIEF.md` + numbered children; `.grove/` is itself the root node. This dissolves
the root-brief false-sibling problem, collapses the insert/reorder cascade to a
single `git mv` of a directory, and gives native file-manager tree navigation.

## Context

- **ADR-0035** is the decision and the source of truth for the grammar/rationale
  — read it first. **ADR-0033** (flat scheme, superseded *structure*), **ADR-0034**
  (migrate-on-adoption — the flip mechanism this reuses), **ADR-0031** (dogfood /
  no-sunk-cost) are the supporting chain.
- This grove is **on v1-flat now** (070/040 flipped it). v2 rides the *same*
  proven pipeline (embed → provision → migrate-on-adoption → loop); only the id
  grammar and on-disk shape change. The v1 flip is reused, not wasted.
- **Grammar (ADR-0035):** leaf `NN-[DONE-]<slug>-[<key>].md`; node dir
  `NN-<slug>-[<key>]/` with `BRIEF.md` inside; root brief `.grove/BRIEF.md`.
  Position `NN` = 2-digit zero-padded decimal, **per level**. `[key]` permanent,
  last token before `.md`/`/`. `DONE` is an infix right after the position
  (fixed-column, scannable). Pure decimal — the FS is case-insensitive and only
  low-ASCII digits collate robustly (probed: macOS/APFS/en_AU.UTF-8).

## Done when

- The dir-based scheme is built, fixture-tested, the live trees are re-migrated
  (v1-flat → v2-dirs), and the v2 binary + global skill are live and verified.
- `grove-llm` verbs speak only v2; the v1-flat reader shrinks to the migration's
  one-time input (alongside the still-needed old `NNN-slug` reader for any tree
  that never opened since 070/040).

## Decomposition

Ordered as the original scheme work was (id → read → grow/lifecycle → migrate →
prose → flip); the live re-flip is last and **user-gated**. Leaves are
reshapeable — this is a planning brief, not a contract.

- `11.1-[24]-id-grammar` — the v2 id model: parse/render node-dir names + leaf
  names (2-digit per-level position, `DONE` infix, `[key]` last), the per-level
  comparator, validation. Replaces `leaf_id`'s flat model.
- `11.2-[25]-read-verbs` — `pick` (recursive DFS over dirs), `brief-chain`
  (walk parent dirs collecting each `BRIEF.md`), `resolve` (search dirs by key).
- `11.3-[26]-grow-lifecycle-verbs` — `leaf-add`/`leaf-insert` (sibling renumber =
  `git mv` of dirs, subtree rides along), `leaf-decompose` (leaf file → node
  *dir*), `leaf-retire` (`DONE` infix in place), `root-init`.
- `11.4-[27]-migrate-v1-to-v2` — migration **v1-flat → v2-dirs** (and old
  `NNN-slug/` → v2 directly for any un-opened tree); fixture-tested hard before it
  touches a real tree; one reviewable commit; idempotent on an already-v2 tree.
- `11.5-[28]-prose-and-commit-convention` — rewrite `content/` (SKILL.md, format
  guides, prompts) to the dir scheme; add the **commit-naming instruction**:
  reference a work item by `<slug>-[<key>]`, never by position/path (ADR-0035 §5).
  Update `CONTEXT.md` glossary terms.
- `11.6-[29]-install-and-reflip-v2` (work, **USER-GATED**) — build + install the
  v2 binary; re-flip this grove (and `grove-general-improvements`) v1→v2 by
  adoption; verify the global skill + idempotency. Mirrors 070/040.

## Notes

- **Sequencing vs. the rest of the refactor:** this node currently sits after
  `08`/`09`/`10` (TUI/inbox sheds + terminate-signal). It is independent of them;
  if v2 should land *before* the sheds (so they re-provision matching v2 prose),
  `leaf-insert` it earlier. Operator's call.
- The `[key]`-bracket-is-a-shell-glob ergonomics wart (e.g. `ls 07-…-[14]*`
  fails) is a known open micro-question — flag it during 11.1's grammar work; an
  option is dropping the brackets in favour of a different stable-id delimiter.
- Migration source is **bimodal once** here (v1-flat OR old `NNN-slug/`), then the
  reader collapses to v2-only — same shape as ADR-0034's one-time old reader.
