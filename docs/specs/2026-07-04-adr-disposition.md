# ADR corpus disposition — approved keep / delete / merge

> **Status:** approved by the human at the required checkpoint (2026-07-04),
> produced by planning leaf `corpus-disposition-k5`.
> **Companion to:** `2026-07-04-adr-minimum-coherent-set-design.md` (Part 3 — the
> mandate). This file is the executable input for `corpus-rework-k6` (rename /
> delete / merge / edit) and `citation-reconcile-k7` (reconcile citations).
> **Snapshot caveat:** citations move as the rework lands — `k7` **re-greps**;
> do not trust the citation snapshots below as final.

## Result

**35 ADRs → 7 survivors.** The zellij→trellis→rmux TUI tower (0013–0030, 18 ADRs)
collapses to zero; the inbox/`grove-meta`, install-materialise, and `status`
clusters go with it. Every deleted ADR already carried a `superseded by ADR-0031`
(or a supersession chain) in its own `Status` line — the corpus already knew it
was dead. Under the new rule (*ADRs are the minimum coherent set describing the
current design; git holds the history*) the dead bodies are removed.

The 7 survivors are exactly the decisions that describe grove **as it is now**:
self-extension core + methodology, one `brew install grove` binary that provisions
the global skill, two binaries (`grove` / `grove-llm`), a self-driving shell loop,
the v2 directory task scheme, and the lifecycle verbs.

## KEEP — 7

Rename slug-only (`git mv NNNN-*.md <slug>.md`) and edit each to be
**self-contained and current-state**: strip its supersession/chronology framing,
delete references to now-deleted ADRs, and correct any stale verb names or
filenames. Philosophy/format per `linkuistics:decision-records`.

| Old # | Target slug | Current-state kernel + required edits |
|---|---|---|
| 0006 | `cli-binary-split` | Two-binary audience split (`grove` = human, `grove-llm` = LLM-driven), flat verb names on `grove-llm`, why-not-argv0-dispatch. **Edit:** verb enumeration is stale — the real sets are `grove {do, migrate, retire}` and `grove-llm {root-init, pick, brief-chain, resolve, leaf-add, leaf-insert, leaf-decompose, leaf-retire, complete}`; drop all inbox/install/uninstall/status/list/version/`meta` verbs and the "two migrated verbs" narrative. |
| 0009 | `do-is-sole-lifecycle-verb` | `grove do` is the sole lifecycle entry verb (collapses the old start/continue/finish cluster); `--start-point` lives on `do`; finishing is in-session. **Edit:** `takeover` no longer exists — the lifecycle surface is `do`/`migrate`/`retire`; drop the "note on superseded ADRs 0002/0005/0006/0007" block. |
| 0010 | `in-session-finish-cycle` | The 5-step, LLM-driven, state-checked finish cycle (promote → delete `.grove/` → merge → remove worktree → delete branch); why in-session not Rust; worktree-remove-before-branch-delete; plain `git merge`; single confirmation gate; resume from git state, no marker. **Edit:** present as 5 steps (the inbox step 0012 tried to add is gone); drop the "dead prompts/finish.md" release note if stale. |
| 0011 | `fresh-grove-start-contract` | `grove-llm root-init` scaffolds a fresh grove: root `BRIEF.md` **plus a first planning leaf** — a brief-only `.grove/` is indistinguishable from a finished grove (would mis-trigger the finish cycle). **Edit:** first-leaf name is `01-<slug>-k1.md` (v2), not `010-<slug>.md`. |
| 0031 | `self-extension-core-and-methodology` | **The defining current-state ADR.** grove = self-extension core (task tree as data, the walk, the grow verbs, two task kinds, loop skeleton) + retained methodology (grilling, driving habits, format guides), bundled in the global skill; machinery shed; distribution collapses to one `brew install grove` binary that provisions the global skill on launch. **Edit:** drop the "supersedes 0013–0030 tower" list and the transition/sequencing narrative; state what grove *is* + the (reversible-but-expensive) trade-off. Keep the pairing with `self-driving-loop`. |
| 0032 | `self-driving-loop` | The loop substrate is a thin, stateless, self-driving shell loop grove owns: `while grove-llm pick has work: launch claude; relaunch only on the `complete` signal`; foreground `claude` owns the TTY (native grilling/resize/Ctrl-C, no PTY wrapper); restart ≡ continuation by construction. **Edit:** distill the Archon-rejection to the essential rationale (the rejected alternative earns a line — it explains why grove owns a shell loop rather than an engine+DB); drop the PoC/dogfood/kill-realisation build history; drop the dead ADR-0028 reference. Evidence doc `docs/research/loop-substrate-options.md` stays. |
| 0035 | `task-tree-scheme` | **The current task-tree scheme.** A node is a directory `NN-<slug>-k<key>/` holding `BRIEF.md` + numbered children; `.grove/` is the root node; position `NN` is a mutable 2-digit per-level locator; `-k<key>` is the permanent stable id (`resolve` finds it across moves); done is a `DONE` infix in place; commit/prose name a work item by `<slug>-k<key>`, never by position/path. **Edit:** present the v2 directory scheme on its own terms; drop the "reversing ADR-0033" framing and the in-flight `[<key>]`→`-k<key>` / `00`-root amendment log. **Absorbs the two merges below.** |

## MERGE — 2 (folded into `task-tree-scheme`, then the files are deleted)

The physical outcome is deletion, but the disposition is **merge**: still-binding
content is folded into the survivor first so nothing is silently lost.

| Old # | Folds into | What survives (must appear in `task-tree-scheme`) |
|---|---|---|
| 0033 | `task-tree-scheme` | The flat dotted-decimal *structure* is dead, but the **rationale** for still-current ideas is not in 0035: (a) prior-art adopted — OID/object-identifiers, materialized-path tree encoding, outline/legal section numbering, natural/version-sort comparator; (b) prior-art **rejected** — LexoRank (Jira) and the fractional-indexing family (Figma; Logoot/LSEQ/Treedoc), which give insert-without-renumber but a flat order with illegible keys → **this is why renumber-on-reorder is accepted**; (c) **integer key, not uuid** (single-writer tree — uuid buys nothing, costs legibility); (d) **filename `DONE` marker, not frontmatter** (keeps whole state in the filename so `pick` reads no contents and `ls` shows done-ness). Fold (a)–(d) into the survivor's rationale. |
| 0034 | `task-tree-scheme` | The current **migration rule**: `grove do` migrates an old-format tree to the current directory scheme **on adoption** (a reviewable single commit, idempotent on a current tree, fixture-tested before touching a real tree); **no transitional dual-format reader** — the live verbs are current-format-only; `grove migrate` is the explicit human verb; the migration accepts both the v1-flat `<dotted>-[<key>]-<slug>` format and the original `NNN-slug/` + `done/` format. Drop the refactor/dogfood/"this grove is flipped"/070-install transition story. |

## DELETE — 26 (superseded / dead; git holds the history)

`git rm` each. No live lesson survives deletion (the walk-away and hard-cutover
principles these invoke already live in SKILL.md's constraints and the surviving
lifecycle ADRs).

| Old # | Title | Dead because |
|---|---|---|
| 0001 | install/update create commits | install-materialise machinery removed (→ `self-extension-core-and-methodology`) |
| 0002 | grove-meta branch + inbox model | inbox subsystem removed |
| 0003 | cross-repo inbox handoff | inbox removed |
| 0004 | inbox as directory of files | inbox removed |
| 0005 | grove-meta sync semantics | inbox removed |
| 0007 | status is canonical visibility | `status`/`list`/`version` verbs removed |
| 0008 | install is idempotent | install removed |
| 0012 | finish cleans up inbox | inbox removed; finish cycle reverted to 5 steps |
| 0013 | v2 presentation is TUI behind boundary | TUI tower deleted |
| 0014 | harness backend is in-process pty | TUI tower |
| 0015 | harness substrate is zellij multiplexer | TUI tower |
| 0016 | dashboard surfaces are dumb proxies | TUI tower |
| 0017 | editor runs on proxy tty | TUI tower |
| 0018 | harness workspaces are zellij tabs | TUI tower |
| 0019 | nav opens workspaces itself | TUI tower |
| 0020 | fork zellij into trellis | TUI tower |
| 0021 | trellis hosting API | TUI tower |
| 0022 | constant nav + swapped content | TUI tower |
| 0023 | content-swap into suppressed slot | TUI tower |
| 0024 | embedded-tool exit observability | TUI tower |
| 0025 | fleet repo discovery manifest+scan | fleet view removed |
| 0026 | trellis is the only TUI | TUI tower |
| 0027 | TUI is config-only, no cwd anchor | TUI tower |
| 0028 | rmux substrate | TUI tower |
| 0029 | rendered history via rmux capture-pane | TUI tower |
| 0030 | grove bundles rmux daemon | TUI tower |

## Notes for `corpus-rework-k6`

- **Order:** merge-then-delete for 0033/0034 (fold content into `task-tree-scheme`
  before removing the files); rename survivors slug-only; edit each survivor to
  current-state. All in one focused commit (or a small, reviewable set).
- **Slug uniqueness:** the 7 target slugs above are unique within `docs/adr/`.
- **De-reference deleted ADRs inside survivors:** while editing, remove every
  reference a survivor makes to a now-deleted ADR (e.g. 0031's "supersedes
  0013–0030"; 0032's ADR-0028 pointer; 0009's superseded-ADR note). Survivor→
  survivor references become slug links (0010↔0009↔0011; 0031↔0032; 0035 internal).

## Notes for `citation-reconcile-k7` (runs last, after slugs are final)

- **Old-number → survivor-slug map:**
  `0006→cli-binary-split`, `0009→do-is-sole-lifecycle-verb`,
  `0010→in-session-finish-cycle`, `0011→fresh-grove-start-contract`,
  `0031→self-extension-core-and-methodology`, `0032→self-driving-loop`,
  `0033→task-tree-scheme`, `0034→task-tree-scheme`, `0035→task-tree-scheme`.
  Any citation to a **deleted** ADR (0001–0005, 0007, 0008, 0012, 0013–0030) is a
  defect to remove or re-point at a survivor.
- **Known in-prose numbered citations (re-grep `ADR-[0-9]` — do not trust this):**
  `content/TASK-FORMAT.md` (ADR-0035 §5 → task-tree-scheme, commit-naming),
  `content/SKILL.md` (ADR-0034+0035 → task-tree-scheme; ADR-0032 → self-driving-loop;
  ADR-0011 → fresh-grove-start-contract; ADR-0035 §5 → task-tree-scheme),
  `content/prompts/continue.md` (ADR-0035 §5 → task-tree-scheme). Reconcile across
  `content/`, `docs/`, `docs/research/`, `docs/workflows/`, and any `BRIEF.md`.
- **Flag (decide, don't silently do):** the Rust `--help` strings in `src/`
  (`grove --help`, `grove-llm --help`) cite `ADR-0006`, `ADR-0032`, `ADR-0035` by
  number. That surface is outside the design spec's stated citation scope
  (`content/` + `docs/`). Decide whether to convert those to slug or drop the
  numbered pointer — do not leave a dangling number.
