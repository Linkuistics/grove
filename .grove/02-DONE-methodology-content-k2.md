# methodology-content-k2

**Kind:** work

## Goal

Encode the externalize-surfaced-work rule into grove's methodology content,
across the coupled prose surfaces, in one focused session so terminology stays
consistent. (See root BRIEF.md for the rule and its rationale.)

## Context

Three files, one coherent change:

1. **`content/SKILL.md`**
   - **Decompose step** (currently ~line 128, opens *"When a leaf is too big for
     one focused session…"*) — reframe from *reactive* to *proactive*: state the
     default-to-externalize rule with its **two triggers / two verbs** (new
     concern → `leaf-add`/`leaf-insert`, never inline; current item proves bigger
     → `leaf-decompose`, do only the first child here) and the **"fits this
     session," not "I can finish it"** bar. Preserve the existing mechanical verb
     instructions that follow — only the framing changes.
   - **Constraint 4 ("Lazy and optional")** in the spine — add a *brief*
     clarification that *lazy = just-in-time, not few* (a growing tree of small
     concrete leaves is fine). Keep it tight: constraint 7 is "one page of
     rules" — do **not** add a new spine constraint.

2. **`content/driving.md`**
   - **Remove** the "The runaway tree" anti-pattern bullet (~line 286). Its
     evidence ("020 rolled its renumber four times") is obsolete — v2 directory
     scheme made renumber a single `git mv` rewriting zero file contents, and
     `leaf-insert` shipped.
   - **Add** a positive habit/pattern in its place: externalize surfaced work
     eagerly; the two triggers; how to tell inline-vs-externalize. Check whether
     "The shortest version" closing paragraph should mention it.

3. **`content/TASK-FORMAT.md`** — light touch: the existing line *"A task too big
   for one focused session* is *a planning task — its job is to decompose, not to
   do"* is aligned already; strengthen/cross-reference only if it reads as
   reactive. Don't force an edit if it's already consistent.

## Done when

- SKILL.md Decompose step reads proactively with the two triggers + the bar.
- Constraint 4 clarifies lazy ≠ few (no new spine constraint added).
- driving.md "runaway tree" bullet gone; positive externalize habit present.
- TASK-FORMAT.md consistent (edited only if it was reactive).
- Wording is internally consistent and uses glossary terms (Leaf, leaf-add,
  leaf-insert, leaf-decompose, Permanent key).
- The canonical phrasing of the rule is settled here for k3 (continue.md) to
  point at.

## Notes

- `content/` is canonical; no hand-copy to any mirror. Building/releasing is out
  of scope (root BRIEF.md).
- If a genuinely hard-to-reverse trade-off surfaces while editing, raise an ADR
  (sparingly) — not expected here.
