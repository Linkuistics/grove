# 31. grove sheds its machinery to a self-extension core that keeps its methodology

- Status: **accepted** (decided in `refactor-to-archon` 030-substrate-decision;
  the machinery deletions are executed by leaves 080/090, the distribution by 070)
- Date: 2026-06-20
- Deciders: Antony Blakey (with grove `refactor-to-archon` 010-plan + 030-substrate-decision)
- Supersedes (at the thesis level — the per-ADR `Superseded` marking is the
  shed-TUI leaf, 080): the **rmux/ratatui TUI tower** (ADR-0013–0030) in its
  entirety. Those ADRs' *runtime* is deleted; nothing of the TUI is retained.
- Pairs with: **ADR-0032** (the loop substrate that replaces the shed runtime).

## Context

grove today is three things welded together: (1) a small **self-extension core**
(the `.grove/` task tree as data, the `pick` walk, the grow verbs, the two task
kinds, the loop skeleton); (2) a body of **proven methodology** (grilling, the
`driving.md` habits, the loop discipline, the CONTEXT/ADR/PRD format guides); and
(3) a large **runtime / machinery** layer (the rmux/ratatui TUI + Fleet tower of
ADR-0013–0030; the inbox / `grove-meta` branch subsystem; the
install/materialise-into-harness machinery with its cli/repo/worktree
`VERSION.md` drift model).

The refactor's directive is **"less in grove."** The original framing (010-plan
D2/D3) read that as "shed the *methodology* to third-party skills, keep only a
minimal self-extension core." Grilling (030, D6) found that conflated two very
different things. The methodology is **markdown** — nearly free to carry, proven
in practice, and *superior in many ways* to third-party collections (e.g.
addyosmani/agent-skills, from which `driving.md` already borrows two sections, in
grove's voice, with attribution). The machinery is **code** — expensive, brittle,
and largely a re-implementation of what a harness/substrate already provides.

## Decision

**"Less in grove" means less machinery, not less wisdom.** Concretely:

- **Survives — the self-extension core:** the task tree as data, the `pick` walk,
  the grow verbs (`root-init`/`leaf-add`/`leaf-insert`/`leaf-decompose`/
  `leaf-retire`/`brief-chain`), the two task kinds, the minimal loop skeleton.
- **Retained — grove's distinctive methodology:** grilling, the `driving.md`
  habits, the loop discipline, the CONTEXT/ADR/PRD format guides. **Bundled in the
  global skill**, not deleted. (This is also *less work* — nothing methodological
  is re-homed.)
- **Deleted — machinery:** the rmux/ratatui **TUI + Fleet** tower (leaf 080); the
  **inbox / `grove-meta`** subsystem (leaf 090); the
  **install/materialise + `VERSION.md`** drift machinery (leaf 090).
- **Shed to the substrate:** worktree lifecycle, fresh-context looping, and the
  human-as-scheduler crank — supplied by the self-driving loop of **ADR-0032**.
- **Defers to third-party (unchanged):** the *generic* engineering skills grove
  never had superior versions of — TDD / review / debugging → superpowers etc.
- **Distribution collapses** to a single **global skill** (bundling the full
  retained methodology) + one **`brew install grove`** (the binary provisions the
  global skill on launch, dissolving the `VERSION.md` drift model — leaf 070).

**Out of scope (explicitly deferred):** any pruning or modularisation of the
methodology itself. We remove *little to none* of the prompts in this refactor;
whether to later split grove's methodology into modular skills is a separate,
careful question for a future grove, not this one.

## Consequences

- The largest single code reduction is the **deletion of the TUI tower** (most of
  the present codebase). grove's surviving code is the tree verbs + the loop
  driver + the signal verb; its surviving *value* is the methodology, intact.
- The **cli/repo/worktree `VERSION.md` drift model evaporates**: one binary, one
  global skill it provisions, no per-worktree materialised copy to drift (leaf 070).
- **Sequencing is safety-first:** the sheds (080/090) run *after* the new loop
  (040) is proven — never delete the old runtime before the new one works.
- This grove **dogfoods** the change: it is itself an old `NNN-slug/` tree the new
  grove must keep reading during the transition (ADR-0032's leaves carry the
  backwards-compat + migration, 050/060).
- The decision is reversible in principle (the deleted machinery lives in git
  history) but expensive in practice; it clears the ADR bar as a real, durable
  trade-off. Rationale and the rejected aggressive-shed alternative are in the
  030 running log (D2/D3 → D6).
