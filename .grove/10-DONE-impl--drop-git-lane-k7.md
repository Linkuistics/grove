# drop-git-lane-k7

## Goal

Make jj the only lane. A non-jj working tree is refused **before any mutation**,
with the command that fixes it; every plain-git branch, and the colocated
git-index machinery that exists only because grove drives git plumbing, is
deleted.

## Context

- `docs/specs/module-decomposition.md`, decision 8 and `## Out of scope`
  (*"The plain-git lane. Dropped."*). The rejected alternative — narrowing the
  safety principle to *where the version control system can* — is what keeps the
  finish transaction alive on one lane and leaves the VCS seam the largest of the
  five modules; it is rejected, not open.
- `minimalism-k1`'s `## Deletion list`, *Reconciled* row 5: `repo.rs`'s git lane
  — `git_index_path`, the empty-internal-hooks-path rule, and every plain-git
  branch; **`vcs_of` survives as a precondition gate rather than a dispatch**.
- `src/finish_cleanup/auxiliary*` (1,257 + 960 + 1,634 test lines) — the
  colocated git-index backup, which `minimalism-k1` records exists *only*
  because grove drives git plumbing on a colocated repo: **jj has no index**.

## Done when

- `vcs_of` (or its successor) is a precondition gate: a working tree that is not
  jj-enabled is refused with `jj git init --colocate` named as the remedy, and
  nothing downstream branches on lane.
- No plain-git branch survives anywhere in `src/`. Verify by enumerating every
  git-shelling call site and classifying each, not by sweeping a pattern list —
  `references/execute.md`'s rule, including its positive and cross-tree controls.
- The git-index backup family is deleted.
- `docs/adr/jj-is-the-only-lane.md` is **added**: dropping plain Git is
  hard-to-reverse, surprising without the safety principle behind it, and carries
  a real rejected alternative. `ADR-FORMAT.md` governs its shape. Once the spec
  is rewritten to current state (`spec-to-current-state-k23`) there is no other
  record saying why, which is precisely the when-to-write test.
- `docs/adr/grove-does-not-stage-its-own-renames.md` is **amended**: the decision
  survives and gets simpler; its Git-lane consequences go. Its migration
  references go too — `delete-migration-k6` has already landed by then.
- `docs/USAGE.md`, `docs/ARCHITECTURE.md` and `docs/CONFIGURATION.md` no longer
  describe a git lane.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the precondition for both leaves after it: the VCS
seam cannot state *"a non-jj working tree is refused before any mutation"* as its
own guarantee while a git branch still exists behind it, and the finish
transaction's deletion is far smaller once the auxiliary family is already gone.

**Do not extract the crate here.** `extract-jj-workspace-k9` does that. This leaf
narrows `repo.rs`; it does not move it.

**The refusal is the product.** Principle 2 — an error that only reports
detection is unfinished. `TaskNameError` (`src/task_name.rs:314`) is the model:
every variant carries what is on disk *and* what it should be.

## Decisions (running log)

**Not a cutover leaf.** The standing note requires each remaining candidate to
re-derive the label rather than inherit it, and the test is whether the
*installed* build meets the tree this leaf leaves and fails. This leaf changes no
filename, no witness, no tree shape — its whole deliverable is source. Every cell
of the matrix is therefore the same tree the installed 19.3.0 already drives, so
no release is cut and the session signals completion normally.

**The gate is two functions, not a narrowed enum.** `Vcs` is deleted outright
rather than reduced to one variant: a one-variant enum still reads as a dispatch
and still invites a second arm. What replaces it is a probe,
`jj_workspace_root(path) -> Option<PathBuf>`, and a gate,
`require_jj_workspace(path) -> Result<PathBuf>`, which is the probe plus the one
refusal. Every former `match vcs_of(..)` becomes a straight line through one of
the two, and the Option form survives only where *absent* is a legitimate answer
rather than a refusal (`path_is_tracked`: nothing owns the file, so nothing
tracks it).

**`ControlMarker` goes with the lane.** Three variants existed because Git has
three markers resolving to three different control directories; jj has one, and
it is always `<workspace root>/.jj`. The type's stated reason for carrying the
marker alongside the paths — that re-deriving it at diagnostic time would walk
the ancestors a second time and could name a different marker — is a Git fact.
With one marker the derivation is a `join`, so the workspace-layout refusal
names it directly and the type is deleted.

**The sweep is an enumeration, and it found a class nobody listed.** The
`## Done when` clause asks for every git-shelling call site to be classified
rather than a pattern list swept. Enumerating `Command::new(` and every
occurrence of `git` across `src/` turned up the expected production sites — and
two *unit-test fixtures* a pattern list aimed at `Vcs::Git` would have missed:
`tree_lifecycle`'s and `task_grow`'s `git_grove()`, each standing up a plain-git
repository as the "instrument" for asserting that a rename reaches the index. A
plain-git repository is a tree this leaf teaches grove to refuse, and an index
assertion is a claim about a lane that no longer exists, so both fixtures move to
jj and their index assertions become claims about the committed revision. They
are unit tests calling library functions by path, so they never met the gate and
would have gone on passing indefinitely.

Final state, classified: `src/` spawns `jj` (six sites), the configured session,
`grove-llm`, `sh`, `stty` and — in one test — its own binary. The only remaining
literal `"git"` in `src/` is the argument to `jj git init` in four fixtures,
which is jj's own subcommand. Positive control: the same grep finds
`Command::new("jj")` in those same files. Cross-tree control: `git` still turns
up where it legitimately belongs — `tests/repo.rs`'s refusal fixtures and
`docs/adr/jj-is-the-only-lane.md`.

**Four ADRs this leaf does not own were corrected, not reworked.** The spec's
`## ADR reconciliation` assigns `supported-workspace-layouts` and
`task-tree-transactions-fail-closed` to k8, `one-live-driver-per-working-tree`
to k9 and `untracked-configuration-delta` to k10. Each nonetheless carries
clauses this leaf makes false today — a plain-Git row in the layout table, the
colocated index backup, the empty-hooks-path rule, `git ls-files` and the
`GIT_INDEX_FILE` hazard, an ignore-mutation argument about plain Git. The rule
that a record describes the design's *current state* binds now, so each false
clause is corrected in place and in this commit; the retirements and reworks
those leaves own are untouched.

**`content/references/commit.md` loses its Git boundary, though `## Done when`
names only the three `docs/` files.** The Commit step taught two boundaries and
told the session to determine which lane it was on. There is one, and a session
that followed the Git half in a colocated tree would commit behind jj's operation
log — so the file was shipping an instruction grove now refuses to make
reachable. The correction is contained: the boundary section, four assertions in
`tests/commit_guidance.rs`, and one clause in `CONTEXT-MAP.md`. The delivery-path
restructuring that k16–k19 own is untouched.

**"Refused before any mutation" is verified by enumerating entry points, not by
reading the gate.** The claim is about coverage, so a clean read of
`require_jj_workspace` proves nothing. Every `grove-llm` verb resolves its paths
through one helper, `llm_cli::grove_paths()`, whose first statement is
`repo::toplevel` — and `grove_paths()` is the *first* statement of all twelve
tree verbs (`root-init`, `pick`, `brief-chain`, `kind`, `resolve`, the five
`leaf-*` verbs, `finish-commit`), before any tree read or write. `complete` is
the one verb that does not call it, correctly: it writes a signal file whose path
comes from the environment and touches no tree, which is what lets it run after
teardown has removed `.grove/` entirely. Bare `grove` reaches the same refusal
through `driver_lease::acquire` → `repo::workspace_control`, which
`tests/workspace_layout.rs` already pins ahead of configuration validation and
ahead of any `.grove/` observation. Thirteen entry points, one exemption, and the
exemption mutates nothing.

**`docs/preservation-baseline.md` gets a note, not an edit.** It is a *measured*
record of v19.3.0 — captured transcripts and read facts — so rewriting its Git
rows would falsify a measurement rather than update a description. Its own
ledger admits exactly this case: *preserve unless a change explicitly records an
approved exception*. So the ledger item "abstract outcomes across Git, native jj,
and colocated jj workspaces" gains a standing note naming this leaf and the ADR,
and every measurement below it stays as taken.

**Two more records carried figures the deletion falsified.**
`finish-keeps-a-cleanup-layer-it-has-not-proved-forced` costed the layer at
10,366 lines across seven modules, two of which this leaf deletes and one of
which it cuts by four fifths; the table is re-measured at 5,690 and says
explicitly that the reduction came from dropping the lane rather than from
answering Q1 or Q4, so the deferral it records is unchanged. Its **Q3** —
*is the marker-replacement sub-transaction reachable?* — becomes **moot** rather
than newly answered: it was reachable, and its witness landed, but its only
caller was the colocated Git-index auxiliary, so the question has lost its
subject. `task-tree-transactions-fail-closed` loses three rejected options that
argued the shape of that auxiliary protocol and one that argued for suppressing
user Git hooks — rationale for code that no longer exists — and the two recovery
diagnostics that told an operator to *preserve the named attempt-bound auxiliary
evidence* now name what is actually there.
