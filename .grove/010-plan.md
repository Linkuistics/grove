# 010-plan

**Kind:** planning

## Goal

Grill the two seeded nav-UX items to a shared design, then grow the tree:

1. **Sort/filter as a mode** — entering sort/filter is a discrete sub-modal
   with clear enter/exit keys, defined sort orders + filter dimensions, state
   ephemeral per session. The Fleet glossary entry already names the
   dimensions: fuzzy needle over `<repo>/<grove>`, inbox-pending toggle,
   lifecycle cycle, sort toggle; engaged ⇒ flat ranked list, idle ⇒ grouped.
2. **Inspect without harness** — a peek-detail path: read the highlighted
   grove's task tree / briefs / inbox from the nav without `grove do <name>`
   spawning its agent session.

## Context

- `src/tui/app.rs` `rebuild_detail` keys detail off the *focused pane's*
  grove target — peek requires pointing detail at a grove with no pane.
- `src/tui/nav.rs` lists **live** groves only and does not scroll; the
  lifecycle filter dimension implies surfacing non-live lifecycles too.
- Inbox observations (incorporated 2026-06-10): sort/filter precedent in the
  old codebase was a sub-modal, not three bare keys.

## Done when

- Both designs settled through grilling, decisions logged inline below.
- Tree grown with ordered work leaves; glossary updated inline as terms
  resolve; ADRs only if a decision clears the three-part bar.

## Notes

## Decisions (running log)

**Q1 — Scope (settled 2026-06-10).** Full deferred nav richness is in scope:
the two seeded items plus everything the Fleet glossary entry deferred —
grouped/collapsible repo headers for the idle shape, lifecycle-aware listing
(beyond live-only), nav scrolling, fuzzy ranked filtering, inbox/lifecycle
toggles. The engaged-filter ⇒ flat-ranked / idle ⇒ grouped distinction ships
in this grove.

**Q2 — Mode shape (settled 2026-06-10).** Inline live-filter mode, fzf-style:
a key enters the mode from Nav; the list re-ranks live as the needle is typed
and toggles are flipped; Enter accepts (filter stays engaged, normal nav keys
return for selection); Esc clears and exits the mode. The whichkey footer
shows the in-mode keys. The overlay configure-then-apply panel was rejected —
live feedback wins; the "sub-modal" precedent is honoured by the discrete
enter/exit, not by a separate panel.

**Q3 — Mode keys (settled 2026-06-10).** `/` enters the mode from Nav
(vim/fzf idiom). In-mode, printable chars edit the needle; Ctrl-i toggles
inbox-pending (Tab accepted as a legacy-terminal alias — Ctrl-i is
byte-identical to Tab there), Ctrl-l cycles lifecycle, Ctrl-s cycles sort.
Enter accepts, Esc clears-and-exits (per Q2).

**Q4 — Sort orders (settled 2026-06-10).** Ctrl-s cycles three orders:
**name** (default, stable) → **recency** (recently-active first, via a new
core per-grove field: last-commit timestamp on the grove branch, computed in
the already-concurrent `RepoView` scan) → **inbox** (pending-observation
count, descending — triage-needing groves float up). mtime-based recency was
rejected (any background file touch reorders the list).

**Q5 — Engaged-filter state in normal Nav (settled 2026-06-10).** The engaged
criteria render as a persistent one-line summary inside the nav (needle +
active toggles + sort order). Esc layers: first press clears the filter
(returning the list to grouped idle), second press returns focus to the pane.
`/` re-enters the mode with the existing needle preserved for editing.

**Q6 — Grouped-header UX (settled 2026-06-10).** Repo section headers are
selectable rows: j/k walk headers and groves alike; `h` collapses / `l`
expands the repo under the cursor (vim-fold idiom); Enter on a header toggles
its fold. Collapsed state is ephemeral per session. The lone header auto-hides
at N=1 (Fleet glossary entry).

**Q7 — Peek model (settled 2026-06-10).** Live preview: while Nav has focus,
the detail widget re-points to the *highlighted* grove as the cursor moves
(file-manager preview idiom) — reading a grove costs zero extra keystrokes
and never spawns a harness. Tab (or `l` on a grove row) moves focus into the
detail widget for scrolling/inbox grooming; Esc there returns to Nav (not the
pane — detail must remember it was entered from Nav). Leaving Nav back to a
pane re-points detail at the focused pane's grove, as today. The explicit
peek-key-only variant (as literally seeded) was rejected as strictly weaker.

**Q8 — Enter on a seed (settled 2026-06-10).** Enter on a seed row prompts a
y/n confirm, then runs `grove do <name>` — the seed becomes a live grove with
its harness pane open. Grove *creation* enters the TUI behind a confirm modal
(a new modal kind over the existing Focus::Modal machinery); the launch path
is the same `grove do` spawn the nav already uses for live groves.

**Q9 — Decomposition (settled 2026-06-10).** Six ordered work leaves, data →
model → engine → wiring: 020-recency-field, 030-nav-model, 040-filter-engine,
050-filter-mode, 060-detail-preview, 070-seed-start. Glossary gained **Filter
mode** and **Live preview** entries (inline, during grilling). No ADR raised —
every decision here is reversible UX, none clears the three-part bar. No PRD —
the running log + leaf files are the agreement record.
