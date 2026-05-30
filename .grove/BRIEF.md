# grove-startup-confuses-the-LLM — brief

## Goal
Make a grove's **lifecycle state legible** at both ends of the seed/done axis,
where it is currently mis-signalled.

- **Start side (the originating confusion).** When `grove do <name>` opens a
  brand-new grove (worktree + branch exist, but no `.grove/` tree yet), the LLM
  has no documented procedure for bootstrapping the root. Fix: an explicit
  "bootstrap a new grove" procedure in the methodology and launcher prompt, plus
  a thin CLI affordance (`grove-llm root-init`) to scaffold the root so the LLM
  never improvises the first artifacts.
- **Finish side (added after the start-side plan settled).** The Complete finish
  cycle tears down the worktree and branch but orphans the grove's `grove-meta`
  inbox, so a *finished* grove shows up as a Seed in `grove status` / the TUI.
  Fix: finish also cleans up (or tombstones) the inbox. See leaf 030.

The two are symmetric lifecycle-legibility bugs: startup mis-signals a newborn
grove as **done**; finish mis-signals a dead grove as a **seed**.

## Done when
- The grove methodology (`content/SKILL.md`) and the `start.md` launcher prompt
  give an LLM an explicit, do-able procedure for a brand-new grove — no guessing
  about whether to `mkdir .grove`, hand-write the root `BRIEF.md`, or invent a
  first leaf.
- The chicken-and-egg between `grove-llm pick` (errors "grove root not found")
  and the tree-growing verbs (which need a root to exist) is resolved.
- The exact docs-vs-tooling split is decided during planning; scope is
  "docs/prompt fix + a thin CLI affordance".

## Primary evidence — the confusion, reproduced live this session
This grove was started by `grove do`, which launched the session with
`start.md`, whose entire content is: *"Start a new grove — use the grove skill's
start-a-new-grove flow."*

1. **No "start-a-new-grove" section exists.** `content/SKILL.md` documents the
   loop (Pick → Bootstrap → Execute → Decompose → Commit → Retire → Finish), and
   *every* step assumes `.grove/` already exists. There is no named flow for the
   empty-grove case the prompt points at.
2. **No verb creates the root.** `grove-llm` has `leaf-add`, `leaf-insert`,
   `leaf-decompose`, `leaf-retire` (grow), and `pick` / `brief-chain` (read) —
   but nothing to create the initial `.grove/` + root `BRIEF.md`. `grove-llm
   pick` on a fresh grove exits with `Error: grove root not found`.
3. **The LLM must improvise scaffolding.** Lacking a procedure, the model has to
   decide on its own to `mkdir .grove`, author `.grove/BRIEF.md`, and author a
   first `010-*.md` leaf — in formats it has to go read — with no signal that
   this is the intended path. (This brief is that improvisation.)
4. **A root brief with no leaves looks *finished*, not *new*.** Empirically
   confirmed this session: with `.grove/BRIEF.md` present and no leaves,
   `grove-llm pick` prints `no live leaves; this grove is done` and exits 0 — the
   exact signal that triggers the Complete finish cycle (delete `.grove/`, merge,
   tear down the worktree). A fresh grove is currently indistinguishable from a
   finished one, so root-scaffolding that stops at the root brief is actively
   dangerous: the next normal `pick` would propose tearing the grove down.

5. **A finished grove looks like a Seed.** The Complete finish cycle (ADR-0010)
   has five steps — promote, delete `.grove/`, merge, remove worktree, delete
   branch — none of which touch the grove's `grove-meta` inbox. By the glossary's
   own definition *Seed* includes "already finished", so the orphaned
   `inboxes/<name>/` (kept alive by its `.gitkeep`) renders in `grove status` /
   the TUI as a seed, indistinguishable from a not-yet-started one. The
   finish-side counterpart of item 4. (Leaf 030.)

## Pointers
- Launcher prompts: `content/prompts/start.md` (the content-free pointer),
  `continue.md`, `takeover.md`, `retire.md`.
- Methodology: `content/SKILL.md` (the loop; no fresh-grove section).
- The `grove-llm` / `grove` binary split: ADR-0006 (which surface a new verb
  belongs on).
- ADR-0009: `do` is the sole lifecycle entry verb — relevant to whether
  fresh-grove scaffolding is a `do`-time step or a separate verb.
- Glossary terms in play: this grove (workstream sense), Drain, Seed (see
  CONTEXT.md). A brand-new grove's inbox may have been a Seed.

## Decisions (running log)
- **Scope (settled):** docs/prompt fix *plus* a thin CLI affordance to scaffold
  the root; exact split decided during planning. (Rejected: docs-only, CLI-only,
  diagnose-first.) Rationale: the confusion is partly "no procedure" (docs) and
  partly "no tool to enact it" (CLI); fixing only one leaves the other gap.
- **CLI shape (settled):** a *new `grove-llm` verb*, not an extension of `grove
  do`, and not both. (Rejected: extend `grove do`'s new-grove path; belt-and-
  suspenders.) Rationale: symmetric with the existing tree-mutation verbs
  (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` — all `grove-llm`,
  working-tree-only, no commit, print the path); keeps `grove do` purely
  mechanical (lowest blast radius on lifecycle code, ADR-0009). `start.md` will
  be rewritten to name the verb as step 1.
- **What it creates (settled):** the root `.grove/BRIEF.md` stub *and* a first
  planning leaf stub — so `grove-llm pick` immediately returns work and the fresh
  grove enters the steady-state loop from step one. (Rejected: root brief only +
  harden `pick`; both.) Rationale: directly removes the empirically-confirmed
  "empty-but-briefed grove reports as *done*" trap (evidence item 4) without
  touching `pick`'s load-bearing Finish-trigger semantics.
- **Verb name (settled):** `grove-llm root-init`. (Rejected: `tree-init`; `init`;
  and `seed`/`bootstrap` — both collide with existing terms: *Seed* is a glossary
  term, *Bootstrap* names the read-context loop step.) Rationale: pairs with the
  existing error "grove **root** not found" — the error names the cure — and fits
  the `<noun>-<verb>` family.
- **Verb spec (recommended defaults, open to objection):**
  - Signature `grove-llm root-init [<first-leaf-slug>]`, default slug `plan` →
    creates `.grove/010-<slug>.md` (Kind: planning). Mirrors `leaf-add <slug>`.
  - Stubs are minimal section-header scaffolds from `BRIEF-FORMAT.md` /
    `TASK-FORMAT.md` with TODO prompts; the executing session fills them.
  - Refuses if `.grove/` already exists ("grove root already exists") — no clobber.
  - Working-tree change only, **no commit** — folded into the first session's
    commit, exactly like the other `grove-llm` tree verbs.
- **ADR (recommended):** raise **ADR-0011** capturing the fresh-grove-start
  contract (new verb over `grove do` extension; create-first-leaf over
  harden-`pick`). Passes all three tests: hard to reverse (the start contract
  binds every future new grove), surprising without context (why does a new grove
  auto-carry a planning leaf?), genuine trade-off (real alternatives rejected
  above).
- **Scope broadened (settled):** added the **finish side** — finish must clean up
  the grove's inbox so a finished grove stops showing as a Seed (leaf 030).
  (Rejected: seed it to a separate grove; grill the finish design now.) Rationale:
  thematic kinship — same seed/done legibility axis as item 4 — keeps both ends
  of the lifecycle fix in one workstream. The finish-side disposition-of-pending-
  observations question is left open for the 030 session to settle (recommended:
  refuse-and-instruct on pending, then remove the inbox dir), because Drain runs
  only at `grove do`, so finish can encounter un-triaged observations.
