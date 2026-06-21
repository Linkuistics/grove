# 34. grove flips to the new scheme by migrate-on-adoption, not a transitional dual reader

- Status: **accepted** (decided in `refactor-to-archon` 060, reopening its own
  scope)
- Date: 2026-06-21
- Deciders: Antony Blakey (with the 060 session)
- Supersedes: the **transitional dual-format reader** plank of **ADR-0033**
  (its "Transition" consequence and 050 running-log D5/D9's 060 hand-off) and
  the **"this grove finishes on the old scheme"** rollout decision in the
  refactor root `BRIEF.md`.
- Pairs with: **ADR-0031** (shed machinery, keep the core), **ADR-0032** (the
  self-driving loop), **ADR-0033** (the flat dotted-decimal scheme this flip
  adopts).

## Context

060 was scoped (ADR-0033's "Transition" consequence; the root BRIEF rollout
decision) as a **transitional dual-format reader** — the verbs would read *both*
the old `NNN-slug/` + `done/` directory format and the new flat dotted-decimal
format during a transition — plus a one-time `grove migrate`, with **this** grove
deliberately finishing on the *old* scheme and the new scheme shipping only at a
later release. The dual reader was a forever-tax-avoidance compromise: ephemeral
trees drain, so *permanent* dual-read was rejected, but *transitional* dual-read
was kept so in-flight groves (including this dogfood tree) kept working with zero
disruption.

On reopening 060, the user chose a simpler shape: have **`grove do` actively
migrate an old tree on adoption**. Once adoption migrates, every tree the verbs
ever see afterwards is new-format — so there is **no need for the verbs to read
the old format at all**, only for the migration to read it *once*. This removes
the dual-reader machinery ("less in grove") and lets this grove itself flip to
the new world, dogfooding the migration + the new loop + the global skill on a
real tree.

## Decision

1. **No transitional dual-format reader.** The live verbs (`pick`,
   `brief-chain`, `resolve`, `leaf-add`, `leaf-insert`, `leaf-decompose`,
   `leaf-retire`, `root-init`) become **new-format-only** — the 050 modules
   (`leaf_id` / `leaf_read` / `leaf_grow` / `leaf_lifecycle`) become *the* live
   verb path, replacing the old directory-based verbs. Old-format code shrinks to
   the parser the migration consumes once.
2. **`grove do` migrates on adoption.** When `grove do <name>` opens a grove
   whose `.grove/` is old-format, it runs the migration (a reviewable, committed
   git change) **before** driving. Idempotent on new-format trees. `grove
   migrate` also exists as an explicit human verb. The migration is **fixture-
   tested hard before it ever touches a real tree.**
3. **This grove is flipped, reversing the finish-old rollout decision.** After
   the engine (060) and the global-skill provisioning (070) are built, the new
   binary is installed; this grove's tree is migrated by adoption; the
   project-local skill mirrors are removed (this worktree, the main checkout, and
   the `grove-general-improvements` worktree) in favour of the binary-provisioned
   global skill; new-scheme prose lands in `content/`. The rest of the refactor
   (080 / 090 / 100) then runs **in the new world**. `grove-general-improvements`
   (also old-format) migrates by the same adoption mechanism on its next
   `grove do`.
4. **The flip is localized.** Every source change before the new-binary install
   is **inert for this grove** — it is driven by the *installed old* binary, which
   keeps reading this old tree until the install. The world flips only at the 070
   install step; `restart ≡ continuation` (ADR-0032) still holds, so a re-run of
   `grove do` re-derives state.

## Rationale

- **"Less in grove"** (the refactor's directive): a one-shot migration +
  new-format-only verbs is strictly less code than a dual reader threaded through
  *every* verb *plus* a migration — and it avoids a bimodal, flagless CLI spanning
  two argument grammars and two addressing models (directory-path vs dotted-id).
- The dual reader's whole value was "keep in-flight groves working." Migrate-on-
  adoption delivers the same outcome (the grove keeps working — after a one-time,
  reviewable rename) with **no permanent two-format surface**.
- **Dogfooding:** flipping this grove + `grove-general-improvements` exercises the
  migration, the self-driving loop, and the global skill on *real* trees before
  the public release — the strongest possible test, and squarely in grove's
  dogfood tradition (ADR-0031).
- **No sunk cost:** 050 built the new-format verbs as isolated, separately-tested
  code; this flip *uses* that work (wires it live), it does not waste it. The
  reversed "finish-old" decision is reversed on merit — the simpler shape is
  better — per the project's no-sunk-cost principle.

## Consequences

- **060 is rescoped** from "dual reader + migrate" to **the engine**: new-format
  verbs live + `grove migrate` + migrate-on-adoption, fixture-tested. Decomposed
  into `060/010-verbs-live`, `060/020-grove-migrate`, `060/030-migrate-on-adoption`.
- **070 absorbs the flip**: global-skill provisioning (binary embeds the skill and
  extracts it to `~/.claude/skills/grove/` on launch), new-scheme prose in
  `content/`, removal of the project-local skill mirrors, and the binary install /
  handoff.
- The new-scheme prose can land **incrementally**: because the global skill is
  binary-embedded and extracted on launch (070), it always matches the binary, so
  the inbox/TUI prose is cleaned as 080/090 land — each shed re-provisions a
  matching global skill.
- **Risk:** a migration bug could corrupt a live tree → mitigated by fixture-
  testing the migration before adoption touches any real tree, and by the
  migration being a single reviewable git commit (revertable). There is currently
  **no global skill** (`~/.claude/skills/grove/` is absent); removing the
  project-local mirrors therefore *requires* the 070 provisioning to land first,
  or every grove is left skill-less.
- ADR-0033's "Transition" consequence and the root BRIEF's rollout decision are
  **superseded** by this ADR.
