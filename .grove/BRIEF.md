# adjust-prompts-to-encourage-small-workitem-breakdowns — brief

## Goal

Adjust grove's methodology content so the driving LLM **aggressively
externalizes surfaced work into new leaves** instead of absorbing it into the
current session. grove's value is many small, low-context sessions (enabled by
the self-driving fresh-relaunch loop); that value is lost when a session
silently grows to cover work that should have been its own leaf.

## The rule being encoded

When work **surfaces mid-session**, default to **externalizing it as a new
leaf** rather than absorbing it:
- **A new concern** (the human raises it, or a tangent appears) → `leaf-add` /
  `leaf-insert`. Never done inline.
- **The current item proves bigger** than its brief assumed → `leaf-decompose`;
  do only the first child in this session.

Continue inline **only** while the work still serves *this leaf's stated goal*
**and** fits one focused, low-context session. The bar is *"fits this session,"*
not *"I can finish it."*

## Done when

- The Decompose step in `SKILL.md` states the rule proactively (not just the
  reactive "when a leaf is too big"); constraint 4 clarifies *lazy = just-in-time,
  not few*.
- `driving.md`'s "runaway tree" anti-pattern is **removed** (its renumber-churn
  evidence is obsolete — v2 dirs + `leaf-insert` made renumber cheap), replaced
  by a positive "externalize surfaced work" habit.
- `continue.md` carries one terse every-session reminder (kept small).
- Terminology stays consistent across all touched surfaces.

## Decomposition

Two work leaves, sequenced (k2 settles the canonical wording k3 references):
- **k2 methodology-content** — `SKILL.md`, `driving.md`, `TASK-FORMAT.md` (light).
  Coupled prose; one focused session keeps terms consistent.
- **k3 prompt-reminder** — the one-line `continue.md` nudge, after k2's wording lands.

## Pointers

- Glossary terms in play: Node directory, Leaf, Permanent key, lazy decomposition
  (constraint 4), "runaway tree" (to be removed) — see `CONTEXT.md`.
- Distribution: `content/` is canonical; the binary embeds it and extracts to
  `~/.claude/skills/grove/` on `grove do`. Edit `content/`, rebuild/release — no
  hand-copy (CONTEXT.md "Global skill provisioning"). **A release is out of
  scope for this grove** — the human cuts releases manually.
- No ADR planned: this sharpens existing methodology guidance rather than making
  a hard-to-reverse, surprising trade-off. (Revisit only if k2 surfaces one.)

## Notes

The decomposition is itself a demonstration of the rule: split at the genuine
seam (methodology prose vs. launcher-prompt surface, with a real dependency),
not speculatively.
