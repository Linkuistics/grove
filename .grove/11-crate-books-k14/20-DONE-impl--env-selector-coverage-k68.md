# env-selector-coverage-k68

## Goal

Extend the crate's environment test so all four of the seam's repository
selectors are proved removed, rather than three of the four.

## Context

- `crates/jj-workspace/src/jj.rs:28-33` declares
  `REPOSITORY_SELECTORS: [&str; 4] = ["GIT_DIR", "GIT_WORK_TREE",
  "GIT_COMMON_DIR", "GIT_INDEX_FILE"]`, and `raw_output` calls `env_remove` for
  each before spawning the child.
- `resolution_ignores_repository_selection_and_temporary_directory_environment`
  (`crates/jj-workspace/tests/environment.rs:62`) sets `GIT_DIR`,
  `GIT_WORK_TREE`, `GIT_COMMON_DIR` and `TMPDIR` to point at a second, foreign
  workspace, then asserts that resolution from the intended tree still answers
  about the intended tree and that nothing was created in the foreign one.
  `GIT_INDEX_FILE` is the one member of the array the test never sets.
- Found while applying chapter 7's justified-subtraction test to refusal 3
  (`docs/walkthroughs/jj-workspace/07-what-jj-owns.md`, *3 · Ambient repository
  selection*), which distinguishes the mechanism being checked from the list
  being checked. The mechanism is checked — deleting the removal loop fails the
  existing test. This leaf closes the smaller of the two gaps.
- **The larger gap is not this leaf's.** Nothing goes red if jj or Git begins
  reading a fifth name, and no test can close that: it is the copy-versus-
  delegation failure the chapter names, and the argued position there is that the
  direction of error is cheap — removing a variable jj does not read costs
  nothing. Do not turn this into a discovery mechanism.

## Done when

- The environment test sets `GIT_INDEX_FILE` alongside the three selectors it
  already sets, so every member of `REPOSITORY_SELECTORS` is exercised.
- The addition is asserted rather than merely set: removing `GIT_INDEX_FILE` from
  the production array makes the test fail.
- `bash scripts/check.sh` passes.

## Notes

**This touches no frozen root.** `crates/jj-workspace/tests/` is evidence rather
than a book root (root brief, *The corpus, exactly*), so no fragment, ledger or
page moves and the cross-book one-commit rule does not bind here. It is placed
beside the other `jj-workspace` follow-ups for coherence, not because it is
blocked by them.

**Scope is one assertion.** If the work looks larger than that on arrival, the
thing that grew is the ambition rather than the defect — re-read the Context
before widening it.

## Decisions (running log)

- **The Context's premise is false, measured.** It states *the mechanism is
  checked — deleting the removal loop fails the existing test*. It does not.
  `Workspace::resolve` reaches `jj::raw_output` only through `main_repo_of`,
  which returns early unless `.jj/repo` is a **file**; both trees the test builds
  are colocated (`jj git init --colocate`), where `.jj/repo` is a directory. So
  the test spawns **no** jj at all and reaches the seam zero times. Verified by
  replacing the `for selector in REPOSITORY_SELECTORS { … }` loop with
  `let _ = REPOSITORY_SELECTORS;` and re-running: `1 passed`. None of the four
  selectors is proved removed today, not three of four.
- **`GIT_INDEX_FILE` is live against jj 0.45.1, and its effect is observable.**
  Controlled A/B over two fresh colocated pairs, one arm with the variable and
  one without, each snapshotting one new file through `jj file list`: with it
  set, the colocated index is written to the ambient path inside the foreign
  tree and `intended/.git/index` is never created; without it, the reverse. Both
  directions are assertable, and the intended-side one is the stronger.
- **`--ignore-working-copy` cannot carry the assertion.** Measured: `jj workspace
  root --name default --ignore-working-copy` under an ambient `GIT_INDEX_FILE`
  writes no index anywhere. So the gate's own call site cannot prove this member
  even when it does spawn; a snapshotting call (`is_tracked`) is what reaches it.
- **The test is extended, not added.** Chapter 7's *Final verification*
  transcript records `1 passed` for `environment.rs`, `31` for `workspace.rs`
  and *Thirty-two tests* in prose, and the pilot's proof stage settled the
  suite at twenty-nine interface tests. A new test would falsify all four
  counts for no gain, so the assertion goes inside
  `resolution_ignores_repository_selection_and_temporary_directory_environment`.
- **The mutation matrix, one name at a time against jj 0.45.1.** Dropping
  `GIT_INDEX_FILE` from `REPOSITORY_SELECTORS` → **FAIL**. Dropping `GIT_DIR`,
  `GIT_WORK_TREE` or `GIT_COMMON_DIR` → **PASS**. Unmutated control → **PASS**.
  So the seam's mechanism is now checked, and checked by exactly one member:
  jj does not follow the other three when opening the backend of a workspace it
  was pointed at, and the crate spawns no Git-aware child that would. Those
  three are unassertable here, not merely unasserted.
- **The task file predicted no page would move; that prediction is wrong.** Five
  passages across three chapters state this test's scope, and the change
  falsifies each — two of them were already false before it. `tests/` is not a
  book root, so no fragment, ledger or line count moves and nothing here breaks
  the freeze; the corrections are prose only, and they land in the same commit
  as the test.
- **Chapter 7's residue row 3 changes hands rather than closes.** The claim it
  carried — `GIT_INDEX_FILE` is removed and no test sets it — is now asserted.
  What the repair exposes is the residue underneath: the other three names.
  Seven rows stay seven, on the precedent row 5 set at `jj-owned-names-k65`.
- **The leaf's one in-session reviewer was spent, and returned nine findings.**
  Classified per `references/execute.md`: **seven valid and actionable**, all
  applied — a summary paragraph at `07:394-397` that reads off the verdict table
  and still said row 3 answers question 3 "with nothing at all" (the finding
  landed against a section and did not reach the roll-up, which is the named
  failure mode); a claim of mine that "no command this crate runs is Git-aware",
  which the same page's own definition and the frozen source comment both
  contradict — jj follows `GIT_INDEX_FILE`, so jj *is* the Git-aware child; an
  incoherent sentence asserting a mutation "cannot be written" and then reporting
  its result; the load-bearing overreach below; the fixture defect below; chapter
  2 describing `GIT_INDEX_FILE` as pointed at the foreign repository when it is
  pointed at a path inside it that does not exist; and a hand-over sentence
  saying the retired row had called those three names *tested* when it said
  *set*. **One is now its own leaf** (finding 9, `EnvGuard` and `set_var`:
  `env-guard-set-var-soundness-k185`). **One was reclassified rather than
  applied**: the reviewer called `!ambient_index.exists()` decoration, which is
  right about the prose overselling it and wrong that the assertion should go —
  it is the direct statement of the property, so it stays and the prose now says
  which assertion discriminates.
- **The central claim was an overreach, and is now stated at the strength it was
  measured at.** Three green mutations show *this test cannot detect those three
  names*, not *jj ignores them* — a jj that followed `GIT_DIR` to write a ref or
  read a config would leave every assertion green. The narrower claim was checked
  separately (running the crate's commands with all three pointed at a foreign
  colocated repository alters nothing in it), and that command-scoped statement is
  what the three pages now make. The mechanism offered for it was wrong too: jj's
  cwd walk is what `current_dir` already pins, and it does not explain why the
  Git layer beneath jj honours `GIT_INDEX_FILE` and not `GIT_DIR`. The pages no
  longer offer a mechanism they cannot support.
- **The fixture needed a second control, and the control has been seen to fail.**
  `is_tracked`'s snapshot is what writes the index, and `snapshot.auto-track`
  decides whether the snapshot takes the new file — so a developer whose own
  configuration sets it to `none()` would see the test go red with the scrub
  intact, which is exactly the non-specific red signal the colocated-fixture
  control exists to rule out. The test now writes a config and pins `JJ_CONFIG`
  at it. Measured, with a hostile `snapshot.auto-track = "none()"` in the ambient
  environment: **with** the pin the test passes, **without** it the test fails at
  the index assertion. Precedent: `workspace.rs`'s `native` fixture pins
  `git.colocate=false` for the same reason.
- **Two validator findings became leaves rather than fixes here.**
  `book-validation` silently drops any Markdown link whose label is hard-wrapped
  (`markdown.rs:686-689`) — so two same-page links in `06-refusal.md` have never
  been checked — and `resolve_local` resolves a bare `#anchor` to the book
  *directory*, which is why the same link fails on one line and passes on two.
  Both are `wrapped-link-labels-unchecked-k186`; this leaf's own link uses the
  explicit `03-subprocess-seam.md#the-premise` form meanwhile.
- **No second reviewer, and no `review-impl` leaf.** Every applied fix narrows a
  claim toward what was measured, which is the safe direction, and the one
  substantive change — the `JJ_CONFIG` pin — is covered by an executable seam
  rather than by judgement: the mutation matrix still discriminates
  (`GIT_INDEX_FILE` red, the other three and the unmutated control green), and
  the pin's own control has been watched to fail. `references/execute.md` exempts
  a fix conclusively covered by an executable test seam from forcing escalation.
