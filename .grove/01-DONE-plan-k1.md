# plan-k1

**Kind:** planning

## Goal

Invert grove's worktree ownership: the user (or their tooling) creates the
working tree and runs `grove do` from inside it; grove itself never creates —
and, at finish, never deletes — worktrees or branches. Worktree/branch
provisioning and teardown become optional convenience *utility* verbs, outside
the grove workflow per se.

## Context

Today `grove do <name>` creates `<repo>/.grove-worktrees/<name>/` on branch
`<name>` (src/repo.rs `create_grove_worktree`), re-attaches orphans, and the
in-session complete finish cycle merges, removes the worktree, and deletes the
branch. Motivation for the inversion: some tools work better when *they* create
the worktree and run the grove process from within that directory; grove's
canonical `.grove-worktrees/` layout gets in their way.

## Done when

Shared understanding reached; decisions logged; ADRs
(*do-is-sole-lifecycle-verb*, *in-session-finish-cycle*, others as touched)
reworked in place; tree grown with the implementation leaves.

## Decisions (running log)

1. **Inversion settled.** The user creates the working tree (any tool, any
   location) and runs grove from inside it. The grove name is the worktree
   name. `.grove-worktrees/` is **no longer canonical** — a grove's working
   tree can be anywhere.
2. **Verb shape: single argument-less `grove do`.** No name argument; run from
   inside the working tree. State dispatch stays: no `.grove/` → bootstrap,
   `.grove/` present → continue, no live leaves → propose finish. The
   one-verb principle of ADR *do-is-sole-lifecycle-verb* survives; the ADR is
   reworked in place (creation/attach dispatch arms go away).
3. **No topology utilities at all.** (Revised — earlier this session we
   sketched `grove create`/`grove remove` utility verbs, then eliminated
   them.) Grove ships zero worktree/branch handling; the user's own tooling
   owns the worktree lifecycle end-to-end (the user will use
   https://github.com/max-sixty/worktrunk). CLI surface shrinks to
   `do` / `migrate` / `retire`.
4. *(folded into 3)*
5. **Standard finish shrinks to: promote → delete `.grove/` in one focused
   commit → signal `complete --done`.** No merge, no worktree removal, no
   branch deletion. (Supersedes an earlier answer this session that kept the
   merge in-session — revised by the user at Q6.)
6. **Integration is outside grove entirely.** No grove verb merges anything;
   plain git/gh (or the user's worktree tooling) owns integration and
   teardown alike.
7. **`grove retire <node-path>`, run in-worktree.** Symmetric with
   argument-less `do`; the two-part `<name>/<node-path>` addressing dies with
   the canonical layout.
8. **Precondition: any git working tree.** Linked worktree or main checkout,
   any branch; grove never reads or constrains the branch anywhere in the
   loop — grove needs no branch awareness at all now.
9. **Smaller calls (recommended, not separately grilled):** `--start-point`
   dies with worktree creation; `--no-launch` stays on `do` as the no-exec
   test seam (provision + adoption-migrate + report, no harness exec);
   dropping `do`'s name argument is a breaking CLI change → next release is a
   major version bump.

## Notes

Grounding facts (verified this session): worktree layout is computed at
src/repo.rs:63-158 and consumed in src/launch.rs (do/retire) and
src/loop_driver.rs; harness stamps live at `<repo>/.grove-stamps/<name>`;
`tree_lifecycle::grove_name` is already worktree-basename; session name is
`<repo-basename>: <name> grove`; unwired `start`/`continue_grove` remnants in
launch.rs.
