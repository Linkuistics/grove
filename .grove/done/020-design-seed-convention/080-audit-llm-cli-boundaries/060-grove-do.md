# 060-grove-do

**Kind:** work

## Goal

Add `grove do <name>` to the **human** binary. The verb inspects
the state of the named grove and dispatches:

- No grove by that name → behave as `grove start <name>`.
- Grove exists with a live worktree → behave as
  `grove continue <name>`.
- Grove exists but worktree was removed (orphaned branch) or
  grove was already finished (branch merged + worktree gone but
  the name was once-known) → launch a harness session that
  explains the state to the user and offers a path forward
  (re-attach the worktree, finish properly, or start fresh).
  The verb itself only dispatches; the in-session LLM does the
  judgement.

The value is QoL: the same shell-history line works regardless
of whether the grove has been started yet. `make`'s "same command
for fresh-or-incremental" pattern, applied to grove.

## Context

- This verb lives on the **human** binary `grove`, not on
  `grove-llm`. The audience is the user at the terminal. The
  in-session LLM handles ambiguous states but does not invoke
  `grove do` itself.
- State detection: `grove status <name>` (existing verb)
  already computes most of what's needed — whether the worktree
  exists, whether the branch exists, whether the grove appears
  in `grove list`. Reuse that machinery; do not duplicate the
  detection logic.
- States the verb must handle (likely complete enumeration):
  1. **Unknown name** — no branch, no worktree, no entry in
     `grove list`. → dispatch to `grove start <name>`.
  2. **Live** — worktree at `<repo>/.grove-worktrees/<name>/`
     exists; branch exists. → dispatch to `grove continue
     <name>`.
  3. **Orphaned branch** — branch exists, worktree gone (user
     manually deleted the worktree). → launch a session that
     surfaces the state; the LLM can `grove start <name>`-like
     re-attach (decide during execution how this is shaped —
     probably a new internal helper that `grove do` shells
     into).
  4. **Finished** — branch merged into the default branch, no
     worktree. (Detection: `grove list` won't show it; the
     branch may or may not still exist.) → launch a session
     that explains "this grove was finished at commit X on
     date Y" and asks whether the user wants to start a new
     grove with the same name. May ultimately re-dispatch to
     `grove start <name>` if the user confirms.
- Naming collision: a user re-using a finished grove's name is
  fine — branches and worktrees are recreated from the default
  branch each `start`, so a fresh start works. But the user
  should be told *why* the verb didn't just continue.
- The verb is non-interactive itself. Dispatch happens by
  exec'ing into the existing `grove start|continue`
  implementations (or invoking their internal entry points).
  The interactive judgement happens in the launched session
  via the harness, which is already how `start`/`continue`
  work.

## Done when

- `grove do <name>` exists on the human binary's clap
  definition with the same `--harness` / `--no-launch` flags
  shape as `grove start` / `grove continue` (so the dispatch is
  shape-preserving).
- The verb correctly dispatches across the four states above.
  Tests cover at least the live (`continue`) and unknown
  (`start`) paths; orphaned and finished paths may be exercised
  via integration tests with seeded git state.
- `README.md`'s CLI reference includes `grove do`; the verb's
  description distinguishes it from `start`/`continue` by
  noting "use this when you don't remember which".
- The `content/SKILL.md` artifacts table is unchanged (the
  verb is not part of the LLM's loop). No prompt or skill
  surface needs updating — this is a pure human-CLI addition.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- Existing `grove status`, `grove list`, `grove start`, and
  `grove continue` implementations are the substrate. The verb
  is a thin dispatcher over them.
- Parent BRIEF (`../BRIEF.md`) Q6 records the rationale and the
  in-leaf scope. The verb is independent of the other five
  leaves in this subtree; it does not depend on `grove-llm`.

## Notes

- **The orphaned and finished branches are the interesting
  cases.** If they turn out to require non-trivial harness
  flow (the launched session needs to know it's in a "repair"
  mode rather than a fresh continue), consider a small
  internal launcher prompt at `content/prompts/repair.md`. Do
  not over-engineer this preemptively — the live and unknown
  cases are the common ones and should ship first.
- **Cherry-pickable.** This leaf has no dependency on the
  `grove-llm` scaffold (leaf 010) or any of the other LLM
  verbs. If a QoL win is wanted before the binary split lands,
  execute this leaf first.
- **No alias to `start` / `continue`.** Do not deprecate them
  — humans who already use `start`/`continue` should keep
  working unchanged. `grove do` is an *additional* affordance,
  not a replacement.
