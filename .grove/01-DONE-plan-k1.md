# plan-k1

**Kind:** planning

## Goal

Adjust grove's prompts/methodology content so the driving LLM **aggressively
externalizes surfaced work into new leaves** instead of absorbing it into the
current session. grove's value is many small, low-context sessions (enabled by
the self-driving fresh-relaunch loop); that value is lost when one session
silently grows to cover work that should have become its own leaf.

## Context

- Files in scope: `content/SKILL.md` (Decompose step, the loop), `content/
  TASK-FORMAT.md`, `content/BRIEF-FORMAT.md`, `content/driving.md` ("runaway
  tree" anti-pattern), `content/grilling.md`, `content/prompts/*.md`.
- Distribution: `content/` is canonical; the binary embeds it and extracts to
  `~/.claude/skills/grove/` on `grove do`. Edit `content/`, rebuild/release —
  no hand-copy (see CONTEXT.md "Global skill provisioning").

## Done when

- The rule is settled (above), the root BRIEF.md captures it, and the tree is
  grown into work leaves. ✔ (this planning task)

## The rule we're encoding (settled)

> When work **surfaces mid-session**, default to **externalizing it as a new
> leaf** rather than absorbing it:
> - **A new concern** (you raise it, or a tangent appears) → `leaf-add` /
>   `leaf-insert`. Never do it inline.
> - **The current item proves bigger** than its brief assumed →
>   `leaf-decompose`; do only the first child in this session.
>
> Continue inline **only** while the work still serves *this leaf's stated
> goal* **and** fits one focused, low-context session. The bar is *"fits this
> session,"* not *"I can finish it."* (D3)

- **D4 — Placement (settled).** Skill + driving + a 1-line launcher prompt:
  authoritative rule in `SKILL.md` (Decompose step proactive; tiny "lazy ≠ few"
  clarification to constraint 4 — no new spine constraint, per constraint 7);
  positive habit in `driving.md` replacing the deleted runaway-tree bullet; one
  terse reminder line in `continue.md` (kept small). `TASK-FORMAT.md` light
  touch if needed.
- **D5 — Work breakdown (settled).** Two work leaves under the root, sequenced:
  **k2 methodology-content** (SKILL.md + driving.md + TASK-FORMAT.md — coupled
  prose, one session) then **k3 prompt-reminder** (continue.md one-liner, after
  k2 settles the canonical wording). A release is **out of scope** (the human
  cuts releases manually). No ADR planned.

## Notes / running decision log

- **D1 — Root problem (settled).** Not eager-planning. The failure mode is:
  when new work *surfaces* mid-session — the user adds an item, or the current
  item turns out to involve more work — the LLM keeps doing it inline rather
  than breaking it out into a new work item (`leaf-add` / `leaf-insert` /
  `leaf-decompose`). Intent: grove should *aggressively* optimize for many
  small low-context sessions; the fresh-session relaunch mechanism exists for
  exactly this, and is wasted when a session absorbs scope.
- **Tension to resolve:** `driving.md` "runaway tree" caution + constraint 4
  ("lazy and optional") could be *misread* as "don't add leaves." Need to
  distinguish speculative pre-decomposition (still discouraged) from
  externalizing concrete surfaced work (now encouraged).
- **D2 — Remove the "runaway tree" anti-pattern (settled).** Its evidence
  ("020 rolled its renumber four times") is **obsolete**: the v2 directory
  scheme made renumber a single `git mv` of sibling dirs rewriting *zero file
  contents*, and `leaf-insert` shipped. A growing tree of small well-specified
  leaves is grove *working as designed*, not a smell. Delete the bullet from
  `driving.md`; consider replacing it with a *positive* pattern ("externalize
  surfaced work eagerly"). "Lazy" (constraint 4) means **just-in-time**, not
  **few** — keep laziness, drop the count-phobia.
