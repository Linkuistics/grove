# refactor-grove-to-be-an-archon-workflow — brief

## Goal

Refactor grove down to its **irreducible self-extension core** (the self-extending
task tree + the loop that walks it) plus its **proven methodology**, and drive it
on a **self-driving loop** that automates the per-task fresh-context crank —
shedding grove's *machinery* (TUI, inbox/grove-meta, install/materialise). The
guiding directive: **less in grove** — meaning less *machinery*, not less *wisdom*
(030 D6).

**Substrate decided (was the open fork; now settled):** a thin, stateless,
grove-owned **self-driving shell loop**, **not** Archon — chosen on the 020 spike's
evidence (**ADR-0032**). The grove's name now misdescribes the outcome; kept as a
historical label (the spike reversing its own premise is the spike working).

## Done when

- The loop runs one grove task per fresh context until the tree is empty, hosting
  both work and grilling tasks, restart-safe. [substrate decided; built in 040]
- grove is shed to its core + methodology: TUI deleted (080); inbox/grove-meta +
  install machinery removed (090); distribution → a single global skill +
  `brew install grove` (070). The methodology is **retained**, not shed.
- The task-id scheme is migrated to flat dotted-decimal (050), wired live with a
  migrate-on-adoption flip (060 + 070, ADR-0034) — no transitional dual reader.

## Decomposition

Retired (in `done/`): `010-plan` (foundations D1–D8); `020-loop-substrate-spike`
(the cited options doc); `030-substrate-decision` (the substrate choice + this
leaf set).

Also retired (in `done/`): `040-substrate-wiring` (the loop driver + signal/kill +
interrupt semantics + PoC, ADR-0032); `050-dotted-decimal-numbering` (the flat
scheme + comparator + the eight new-format verbs, ADR-0033 — built isolated, now
wired live by 060); `060-backwards-compat-migration/` (node — **the engine**,
ADR-0034: new-format verbs live + `grove migrate` + migrate-on-adoption, all three
leaves built and fixture-tested; inert for this live tree, which still flips at 070).

Live leaves (this grove is still an old `NNN-slug` tree; the flip happens at 070):

- `070-global-skill-homebrew-distribution/` (node — **the flip**, ADR-0034):
  binary-provisioned global skill + new-scheme prose + `brew install grove` sole
  gesture + remove project-local mirrors + the live install/flip. Decomposed
  (5 leaves, safety-ordered — provision before removing mirrors; flip last +
  user-gated): `010-embed-skill-and-provision` → `020-new-scheme-prose` →
  `030-homebrew-sole-gesture` → `040-install-and-flip` (user-gated) →
  `050-remove-project-skill-mirrors`.
- `080-shed-tui` (work) — delete the rmux/ratatui TUI + Fleet.
- `090-shed-inbox-and-install-machinery` (work) — delete inbox/grove-meta +
  install/materialise.
- `100-complete-terminate-signal` (work) — `grove-llm complete --done` to end the
  loop cleanly (loop-runtime, position-independent; must land before release).
- `scheme-v2-directories` (node, **ADR-0035**) — reverse ADR-0033's flat structure
  to real directories (node = dir with `BRIEF.md` + numbered children); keeps the
  permanent `[key]`, reference-by-key, numeric order, done-in-place. Surfaced by
  dogfooding the 070/040 flip. Independent of 080/090/100; reorder ahead of them if
  v2 should precede the sheds.

Sequencing: 060 (engine) → 070 (flip — world goes new-format here) → 080/090 sheds
+ 100, now running **in the new world** (don't delete the old runtime before the
new one works). 100 is independent and may land any time before the release.

## Pointers

- Substrate evidence: `docs/research/loop-substrate-options.md` (020 spike).
- Decisions: **ADR-0031** (shed machinery, keep core + methodology) and
  **ADR-0032** (self-driving shell loop, not Archon). Full rationale: the retired
  `010-plan` (D1–D8) and `030-substrate-decision` (D1–D6) running logs in
  `.grove/done/`.
- grove's process-machinery history ("which complexity to own"): ADR-0028 (rmux
  substrate / trellis deletion) and the rmux glossary section in `CONTEXT.md`.

## Notes

### Settled decisions (condensed — full rationale in the retired running logs)

From `010-plan` (foundations): **D1** Archon = the workflow-engine. **D2/D3**
end-state = replace the runtime, keep the self-extension brain, shed aggressively.
**D4/D5** task ids → flat dotted-decimal, version-sort comparator,
mark-done-in-place. **D6/D7** execution = a continuous fresh-context loop,
resume-safe by construction, the engine (not the human) turns the crank. **D8**
substrate reopened → the 020 spike.

From `030-substrate-decision` (the substrate, decided on the spike's evidence):
**substrate = self-driving shell loop, NOT Archon** (ADR-0032; Archon's
`interactive` fails gate D, the restart hypothesis is refuted, DB walk-away cost,
a ~10-week-old rewrite). **Native foreground `claude`**; an out-of-band `grove-llm`
signal triggers an external kill (lean: self-spawned delayed killer); `pick` is
the loop condition; **relaunch is opt-in** so interrupts stay stopped; restart ≡
continuation. **Distribution = `brew install grove` sole gesture**, one binary that
provisions the global skill (dissolves `VERSION.md` drift). **Backwards-compat =
transitional dual-format + one-time `grove migrate`, then drop** (ephemeral trees
drain). **"Less in grove" = less machinery, not less wisdom — the methodology is
RETAINED** (ADR-0031, D6).

### Rollout (REVERSED 2026-06-21 — ADR-0034)

**Superseded.** The original rollout ("this grove finishes on the *old* scheme;
do NOT switch the active binary or migrate this grove; ship the new scheme only at
a later release; build 060 as a transitional dual-format reader") is **reversed**
by **ADR-0034**. New rollout:

**We flip this grove into the new world.** There is **no transitional dual-format
reader** — `grove do` **migrates an old tree on adoption** (a reviewable, committed
git change) and then drives it new-format, so the verbs are **new-format-only**
(the 050 modules become the live verb path) and the only surviving old-format code
is the parser the migration consumes once. Sequence: build the **engine** (060 —
verbs-live + `grove migrate` + migrate-on-adoption, fixture-tested *hard* before it
touches a real tree), then the **flip** (070 — binary-provisioned global skill +
new-scheme prose in `content/` + remove the project-local skill mirrors + install
the new binary). All source changes before the new-binary install are **inert for
this grove** (it is driven by the *installed old* binary until then); the world
flips at exactly the 070 install step, after which the next `grove do` migrates
this tree and the rest of the refactor (080/090/100) runs **in the new world**.
`grove-general-improvements` (also old-format) flips by the same adoption mechanism
on its next `grove do`. Dogfood per ADR-0031, reversed on merit (no sunk cost):
050's isolated verbs are now *used*, not wasted.

### Dead code to sweep (after 060, for 080/090)

060 fully unwired the old verb path. These now-dead modules are **not** the TUI
(080) or inbox/install (090), so a shed must claim them explicitly — fold into 090
or a dedicated cleanup leaf:

- `src/pick.rs`, `src/brief_chain.rs`, `src/root_init.rs`, `src/leaf_ops.rs` —
  dead (only comment references remain in `repo_view.rs`/`migrate.rs`). Delete
  wholesale, and drop their `pub mod` lines + their old-format integration tests
  (rewritten in 060/010 to drive the new surface).
- `src/leaf.rs` — delete the old-format reader/grower (`add`, `insert`,
  `surface_cross_refs`, `write_template`, the old `NNN-slug`/`done` parsing &
  header rewriting). **Keep `split_prefix`** (now the old-format reader that
  `migrate.rs` consumes once) and **`Kind`** (live: the new grow/lifecycle verbs
  + `llm_cli`). `migrate.rs` itself is shed-able only once no old tree can exist.

### ADRs

- **ADR-0031** — grove sheds its machinery to a self-extension core that keeps its
  methodology.
- **ADR-0032** — the loop substrate is a self-driving shell loop, not an Archon
  workflow.
- **ADR-0033** — task ids are flat dotted-decimal positions with permanent keys
  (the scheme 050 built).
- **ADR-0034** — grove flips to the new scheme by migrate-on-adoption (no
  transitional dual reader); this grove is itself flipped. Reverses the rollout.
- **ADR-0035** — the task tree is real **directories** with stable-keyed names,
  reversing ADR-0033's *flat* structure (keeps the `[key]`, numeric order,
  done-in-place). Built by node `scheme-v2-directories`.
