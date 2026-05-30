# 010-implement-root-init

**Kind:** work

## Goal
Ship the `grove-llm root-init` verb that scaffolds a brand-new grove's tree —
the root `.grove/BRIEF.md` stub plus a first planning leaf — so the LLM never
improvises the first artifacts and `grove-llm pick` yields work immediately.
Land the decision record (ADR-0011) and the glossary entry alongside the code.

## Context
- Read the root brief's running log: the verb spec and all four settled
  decisions live in `.grove/BRIEF.md` (`## Decisions (running log)`).
- ADR-0006 — the `grove-llm` vs `grove` binary split: this verb belongs on
  `grove-llm` (LLM-driven, working-tree-only).
- Sibling verbs to mirror for structure, output, and tests: `leaf-add`,
  `leaf-insert`, `leaf-decompose`, `leaf-retire`. All are working-tree-only, make
  no commit, and print the absolute created path(s) on stdout. Find their Rust
  module + test fixtures and follow that pattern.
- Templates to emit: section-header scaffolds matching the grove skill's
  `BRIEF-FORMAT.md` and `TASK-FORMAT.md`. (Note: the canonical format files ship
  under `content/` in this repo — locate them; do not hard-duplicate the prose.)

## Done when
- `grove-llm root-init [<first-leaf-slug>]` exists:
  - default slug `plan` → creates `.grove/010-<slug>.md` with `**Kind:**
    planning`, and `.grove/BRIEF.md` (stub with the BRIEF-FORMAT headers).
  - signature mirrors `leaf-add <slug>`; slug validated (lowercase ASCII,
    digits, dashes).
  - refuses with a clear error if `.grove/` already exists (no clobber).
  - working-tree change only — **no commit**; prints the absolute paths created.
- After `root-init`, `grove-llm pick` returns the new `010-<slug>.md` leaf (NOT
  "no live leaves; this grove is done"). Add a test asserting exactly this — it
  guards against the empty-but-briefed-looks-finished trap (root brief evidence
  item 4).
- Unit/integration tests cover: happy path, slug default, custom slug, the
  already-exists guard, and the pick-returns-the-leaf invariant.
- **ADR-0011** written (docs/adr/0011-*.md) recording the fresh-grove-start
  contract: new verb chosen over extending `grove do`; create-first-leaf chosen
  over hardening `pick`. Cite this grove's brief evidence.
- `CONTEXT.md` gains a glossary entry for the verb / fresh-grove-start concept
  (terse; glossary, not spec).
- `cargo build` + `cargo test` green; `grove-llm root-init --help` reads cleanly.

## Notes
- Leaf 020 (docs) depends on this leaf's *final* shipped behavior — keep the
  `--help` text and the created-stub shapes clean enough to quote there.
- Out of scope (explicitly rejected during planning): hardening `pick` to
  distinguish new-vs-done, and extending `grove do`. If a strong case to harden
  `pick` surfaces while implementing, capture it as a follow-up leaf rather than
  expanding this one.
