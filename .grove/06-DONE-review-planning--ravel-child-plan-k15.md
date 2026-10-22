# ravel-child-plan-k15

**Reviews:** ravel-child-plan-k4

## Goal

Read the planned subtree of `ravel-child-plan-k4` adversarially. Report
findings and make no fixes.

## Context

- The producer is `ravel-child-plan-k4`. Find its commit from the handle. The
  planned leaves are `ravel-floor-k6` to `book-specs-k14`.
- The root brief: *Design* and *Requirements*. Every item of *Design* must
  belong to exactly one leaf.
- The specific doubts:
  - Each leaf must keep `bash scripts/check.sh` green on its own. The
    `grove-loop` and `overview` books reproduce whole source files, so a code
    change that lands without its chapter edit breaks `book-check`. Check the
    order of the leaves against this rule, and check that no leaf holds a
    stale quote.
  - The expand step keeps `keyed-launch` alive while two callers move. Check
    that the conversions of `Argv` between the leaves compile at every step.
    Check that the two latches never meet in one process.
  - `lifecycle-job-k9` is the largest leaf. Check whether it fits one session,
    and whether its seam is right if it must decompose.
  - The records are split across three late leaves. Check that no record that
    states the reversed decision belongs to no leaf. A sweep of the tree for
    `keyed-launch`, `keyed_launch`, "survivor", "surviving" and "group" is
    the instrument. Follow the controls that `references/execute.md` requires.
  - Check that each leaf names the Ravel tasks that it must pass.

## Done when

- The findings are written in this file, each with a severity and a
  `path:line` coordinate or a leaf handle.
- Cut an integration leaf only if a finding warrants action. Use
  `leaf-insert` at the first live sibling, as `references/decompose.md` directs.

## Notes

- The producer finished without a provider receipt, so this leaf carries no
  `**Creator:**` line.

## Findings

Reviewed producer commit `ead774d69285584a0dfbdb33f3d94fcd1fc71840`.
P1 means an intermediate leaf cannot meet its repository gate as planned.
P2 means a missing assignment or a contradictory completion condition.

### F1 — P1: The version floor omits its source-book update

**Coordinate:** `ravel-floor-k6`, Done when.

This leaf changes `crates/grove/src/dispatch/ravel.rs`, including its inline
resolver tests, but assigns no book update. The entire file is reproduced by
the `ravel-resolution` fragment at
`docs/walkthroughs/overview/06-standalone-invocations.md:517`. The overview
manifest includes all of `crates/grove/src/**/*.rs`, and the repository gate
checks final reconstruction of every book. Changing the resolver while leaving
this fragment unchanged therefore breaks the first leaf's gate. The plan must
assign the corresponding overview fragments and indexes to this leaf rather
than wait for `grove-run-job-k8`.

### F2 — P1: The Argv bridge also changes the overview corpus

**Coordinate:** `ravel-job-k7`, Done when, the Argv and book clauses.

The expand leaf moves the builder's return type and requires each old caller to
convert it at its call site. One caller is in
`crates/grove/src/standalone.rs:141`, and its launch consumes the value at line
182. That caller must change before the next leaf can compile. Its source is
reproduced at `docs/walkthroughs/overview/06-standalone-invocations.md:221`
and line 290, but the expand leaf assigns only the `grove-loop` book. The
overview changes are assigned to the following migration leaf. Thus the bridge
can preserve compilation while still breaking book reconstruction. Its overview
update must be part of the expand leaf too.

### F3 — P1: Two archive consumers still construct the old release layout

**Coordinate:** `ravel-floor-k6`, Context and Done when.

The release-input assignment names the resolver, pinning module and five smoke
scripts, but misses `scripts/test_ravel_target_inputs.py:18` and
`scripts/ravel-target-inputs.test.sh:22`. The former builds its nominally valid
archive under a hard-coded `ravel-v1.0.0` root. The latter checks and copies
hard-coded 1.0.0 prefixes and archive names, including lines 28–29 and 68–71.
Changing `scripts/ravel_target_inputs.py` to 1.1.0 makes its validator reject
the Python control's valid input, and makes the shell control look for absent
paths. `scripts/check.sh` runs the Python controls; `ravel:targets:controls`
runs both files. Neither test prints a version, so the leaf's clause about
fixtures that print 1.0.0 does not cover them. Assign both consumers to the floor
leaf. Its named target-controls task also requires `ARCHIVES`, as
`Taskfile.yml:119` states; setting `RAVEL` alone does not invoke it.

### F4 — P1: Book deletion precedes the removal of checked inbound links

**Coordinate:** `keyed-launch-removal-k11`, Done when; `book-specs-k14`, Notes.

The removal leaf deletes the runner book and its structure spec. However,
`docs/ARCHITECTURE.md:45` links to the book's README, and
`docs/specs/grove-loop-book-structure.md:19` links to the deleted structure
spec. Those source records wait for `design-records-k12` and
`book-specs-k14`. The assertion that structure specs are ungated overlooks
`every_repository_markdown_reference_resolves` at
`crates/grove/tests/reference_navigation.rs:502`. It scans repository Markdown
and refuses missing targets. The removal leaf therefore leaves a red gate
until later siblings land. Checked inbound links must be reconciled with the
deletion, even if the rest of each document's rework remains later.

### F5 — P2: Current lifecycle explanations have no assigned owner

**Coordinate:** `lifecycle-job-k9`; `book-specs-k14`.

`docs/walkthroughs/overview/03-three-steps.md:814` still says a surviving Ravel
group stops continuation, and
`docs/walkthroughs/overview/04-proving-a-negative.md:263` describes group state
as a returned contract input. Neither chapter appears in the planned book
assignments: the lifecycle leaf names only the `grove-loop` book, and the
overview edits elsewhere name chapters 01, 05 and 06. In addition,
`docs/specs/user-guide-coverage.md:104` retains “no surviving group” in row L12,
while the final leaf's explicit coverage-spec condition changes only the home
of `reraise`. `crates/grove-loop/src/session_result.rs:26–30` also documents
the removed runner and survivor interpretation without an explicit source-edit
assignment. These are current descriptions, not dated run evidence. Name their
owners so the final result satisfies the requirement to rework each record that
states the reversed decision; pair source-comment changes with their book
fragments.

### F6 — P2: Intermediate sweep conditions require later siblings to finish

**Coordinate:** `keyed-launch-removal-k11`, Done when and Notes;
`design-records-k12`, Done when; `book-specs-k14`, Done when.

The removal leaf requires a whole-tree search to find the crate names only in
dated evidence and released changelog entries, while its Notes explicitly leave
those names in current records for the next three leaves. The design-records
leaf similarly requires no record to state the reversed decision while
`user-records-k13` still owns the obsolete usage and driver instructions and
`book-specs-k14` still owns the obsolete specs. These conditions cannot hold
at their planned boundaries. Even the final literal whole-tree condition
includes `.grove/`, whose task files intentionally discuss the removed crate.
State a stage-appropriate scope and classify deliberate remaining work at each
intermediate boundary. Reserve the final current-record sweep for the leaf
that can actually complete it, with explicit treatment of task notes and
retained history.

## Inspection evidence and limits

- The review ran no test, build, lint or format command. Gate failures above
  follow from the inspected source and task definitions, not from a test run.
- Graph discovery was attempted through `codebase-memory-mcp cli list_projects`.
  It refused startup because a pre-coordination or unverified generation was
  active. No project generation or graph coverage could be established. All
  source evidence above comes from direct reads and text searches.
- The text sweep used `rg -n --hidden -i
  'keyed-launch|keyed_launch|survivor|surviving|\bgroup\b' .`, excluding only
  `.git/`, `.jj/` and `target/`, with the complete output held outside the
  repository during inspection. Matched paths were enumerated before narrowing
  material findings. Runner code, its book and its consumers are migration or
  removal work; current records are rework candidates; dated audits, research,
  evaluation evidence and released history retain their evidence; unrelated
  uses of “group” and “surviving” do not state the reversed decision. This is a
  task-directed review, not a claim that every matched sentence is defective or
  that no further omission exists.
- The same search and exclusion flags found a seeded scratch match, then found
  none after the seed was changed to ordinary text. The production control was
  `crates/grove-loop/src/session_result.rs`; the retained-evidence control was
  `docs/ravel-caller-surface-audit.md`. Both were dirty. A deliberately broken
  pattern failed to find either known match, showing that these controls reject
  that broken instrument.
- The Argv conversion is otherwise a viable compilation bridge. The production
  invocations of `grove run` and the lifecycle select separate CLI branches, so
  their temporary runner latches need not operate together. Installed command
  and result tests live in separate integration-test binaries from the lease
  unit tests. No latch-conflict finding is established by the current plan.
- The named command and result tasks explicitly include their ignored installed
  cases. The lifecycle terminal cases and current standalone cases are ordinary
  tests reached by the named activation tasks. The new standalone terminal case
  must keep that reachability.
- The lifecycle leaf is broad, but size alone does not establish a defect. If
  it takes its optional lease-first split, the old loop runner still emits
  `keyed_launch::LaunchEvent` while the migrated lease expects the new type.
  That boundary needs an observer conversion, and the lease child must retain
  the crate dependency until the driver child removes its last uses. Treat the
  proposed seam as requiring that bridge, not as an independent type change.

## Decisions (running log)

- **Planning integration is needed (review, 2026-10-22).** The intermediate
  book and reference gates have actionable dependency gaps. Cut an
  `integrate-review-planning` leaf immediately before the first implementation
  leaf. The review makes no plan or implementation fixes; the integration
  session triages the findings from this committed review.
