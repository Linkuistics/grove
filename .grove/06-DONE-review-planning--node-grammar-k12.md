# node-grammar-k12

**Reviews:** node-grammar-k3

## Goal

Adversarially review the ordered implementation plan against the reviewed design
and the human's cutover requirements before any implementation starts.

## Context

- Find the producer commit by `node-grammar-k3`. Its artifact is the root brief
  and the six implementation bodies `distinguished-names-k6`, `node-files-k7`,
  `node-methodology-k8`, `node-documentation-k9`, `node-cutover-k10` and
  `grove-migration-k11`.
- Requirements are `plan-k1` and the root brief. The design at
  `node-grammar-k2` was reviewed at `node-grammar-k4` and integrated at
  `node-grammar-k5`. Use the current ADRs, specs and library model limits,
  including formalism finding 049; do not re-interview.
- The task's explicit scope keeps these working increments in one grove.
  `docs/RELEASING.md` supplies the release route, and the book manifests
  identify the source ownership that every source-changing leaf must honor.
- Parent discovery used targeted source reads after the graph CLI refused to
  start because of a generation conflict; there is no graph generation or
  coverage evidence to inherit. Re-establish graph availability or inspect
  source directly for claims about consumers.

## Review questions

- Does the library leaf deliver a usable, green API with every required consumer
  adaptation, while the Grove grammar leaf changes every coupled reader, writer,
  handle and fixture together? Is either too large for one focused session, and
  is there a smaller independently working boundary the plan missed?
- Are read-level validation and projected-final-level validation both covered,
  including generic duplicate refusal, root-specific policy, bare node creation,
  batches, optional promotion children and node rewrites? Do conformance samples
  have independent expected verdicts and tests for the model's stated gaps?
- Are the library/Grove content-write and recovery contracts preserved, including
  the accepted conditional interrupted-decompose advice and root classification
  after validation? Has the plan accidentally restored rejected machinery?
- Can each source leaf land with its affected books green, including adapters,
  reference-domain CLI changes, fixtures inside book roots, indexes and versioned
  manifest roots? Is the later documentation leaf confined to work that earlier
  green boundaries can actually leave for it?
- Is the human handoff executable when the installed old driver is already in
  memory, the release uses the default workspace and binaries are global? Can
  preparation, approval, stopping, publication, plugin refresh, this-tree rename,
  retirement and restart occur without a false completion or an incompatible
  relaunch? Are the other drivers kept stopped until their trees are verified?
- Does migration cover all known trees and newly discovered ones, preserve
  identity/content/order, handle interruptions and `FORMAT` without guessing,
  and verify actual installed-tree reads rather than only scratch fixtures?
  Is the no-product-migration constraint intact?

## Done when

- Findings are inspection-only, linked to the producer's commit and precise
  artifact coordinates, with a practical consequence and proposed correction.
- If actionable findings exist, insert `integrate-review-planning` using the
  bare slug `node-grammar` before the first following live sibling entry,
  presently `distinguished-names-k6`. Its body names this review's handle
  under `**Integrates:**`, rather than adopting a findings list as its charter.
  If there are no actionable findings, create no integration leaf.
- The review retires with implementation still after any integration it earned.

## Notes

The review consumes the plan, not unfinished implementation. No in-session
reviewer and no code changes are part of this leaf.

## Findings

Reviewed: commit `tpoponvo` (`091abec6`), *node-grammar-k3: plan node-file
implementation and cutover*, read against the working tree at that commit (the
working copy was empty on top of it). The artifact is the root brief and the six
`impl` bodies at positions 07–12; coordinates are `path:line` in that tree.
Inspection only: no build, test, check or runner was executed. Claims about the
machine were made by listing directories and reading files, never by running a
`grove` binary against a tree. This review's own `leaf-insert` afterwards
shifted the six implementation leaves from positions 07–12 to 08–13; the
handles cited below are unchanged, and the position prefixes in cited paths
are the producer's. Ordered by severity.

### F1 — The cutover handoff names no actor per step, no approval channel, and stops the driving loop in two contradictory places (actionable, medium)

`.grove/11-impl--node-cutover-k10.md:33-36` states the handoff sequence as
"stop the driving loop and other active Grove loops; cut/publish and install
the release; refresh the plugin; rename this tree; verify it; re-run `grove`" —
the driving loop first. `:57-63` states the opposite shape: the session finishes
the whole cutover and "once the completed cutover is committed, return without
a signal", which is what stops the driving loop — last. Between them, `:59-61`
contemplates the human interrupting the agent, leaving the handle live "to
resume explicitly", without saying how.

What the driver actually does, read at `crates/grove-loop/src/loop_driver.rs`:
a session that returns with no token prints *session ended without a completion
signal … loop stopped* and returns `LoopOutcome::Stopped` (`:318-334`); a
Ctrl-C reaches the child's process group through the runner's escalation and
the loop returns `Interrupted` (`:308-311`). So "stop the driving loop" taken
first kills the cutover session mid-work; "resume explicitly" then means
re-running `grove`, which is the 20.2.0 driver picking `node-cutover-k10` again
into a fresh session that is once more *inside* a loop the sequence says must
already be stopped. The driving loop does not need stopping before the switch:
it is blocked in `wait` for the session's whole life, reads the tree only
between sessions (`:243-247`), and is unaffected by a binary replaced on disk.
Only a Relaunch token sends it back into the renamed tree, and that is refused
loudly at `transition_to_current`, not silently.

Two further gaps in the same handoff. First, the leaf's kind is `impl`, which
the shipped skill marks AFK, and the personal configuration launches it under
`codex … --ask-for-approval never`; the body says "approval concerns the
concrete publication and runtime switch" (`:36-38`) but names no channel by
which approval reaches a session with no human at the keyboard. The only one
available is: session ends without a signal, the human acts, the human re-runs
`grove`, and the *next* session — fresh context — must find evidence that
approval was given, which nothing in the body records. Second, the no-signal
rule at `:57` is stated unconditionally, but it is correct only for a session
the 20.2.0 driver launched; a `node-cutover-k10` session launched by the newly
installed driver after a human restart (the very case `:59-61` leaves live)
should retire and signal normally, or the human restarts the loop a third time
to reach migration. The mandate prompt carries "Grove version: N", so the rule
is checkable in-session.

One more coordinate the handoff should state. This working tree is a
*secondary* jj workspace (`jj workspace list`: `default: ../grove`); the
release is cut in the default workspace as `:13-16` and `docs/RELEASING.md`
require, and `.grove/` is tracked (thirteen files in `@`, none on `main`). So
moving `main` to the last code commit and `jj new main` in `~/Development/grove`
materialises a second working copy of this tree's `.grove/`, in the old
grammar, that the release-window preflight loop in `docs/RELEASING.md` will
list and the new build will refuse. The body should say which workspace renames
and that the other's `@` is moved onto the rename afterwards.

Contract: the root brief's *meta-grove sequencing hazard* (`.grove/BRIEF.md:
155-162`), which calls this "a human moment", and planning decision 4. Practical
consequence: an AFK implementer reading `Done when` in order writes a handoff
whose first instruction destroys the session that has to execute the rest, or
publishes on its own authority because nothing tells it where approval lives.
Suggested: rewrite `:33-36` as a two-session shape — session one prepares,
records the concrete plan and an explicit *awaiting approval* entry in this
leaf's running log, and ends without a signal (that ending is the stop of the
driving loop); the human stops the *other* loops, records approval in the same
log (or performs the publication and install by hand from the recorded plan),
and re-runs `grove`; session two finds the approval entry, performs or verifies
publication, install, plugin refresh, rename and verification, retires the
leaf, and ends without a signal iff its mandate names a Grove version older
than the one it installed, otherwise with the ordinary `grove-llm complete`.

### F2 — The `FORMAT` inventory is wrong: three of the four other trees carry one, and the shipped driver skill still says the driver writes it (actionable, low)

`.grove/BRIEF.md:142` and `.grove/12-impl--grove-migration-k11.md:21,38` name
`grove.gh-issue-12/.grove/FORMAT` as "the known foreign leftover" and charter a
disposition for that one file. On this machine `FORMAT` exists at the root of
`APIAnyware.add-ocaml-target/.grove`, `Writegood/.grove` and
`grove.gh-issue-12/.grove`, each containing `session-kinds-v1`. Nothing in
`crates/grove-loop/src` or `crates/grove/src` writes that file any more
(`tree_lifecycle.rs:413-416`: the witness went with the migration), and this
grove's own root has none.

Consequence: `node-cutover-k10:30-31` ("expose malformed or unexpected shapes
before publishing") and `grove-migration-k11:35-36` ("unexpected shapes are
reported for specific recovery, not guessed at") will surface two `FORMAT`
files the plan has no disposition for, and a session that honours *not guessed
at* stops. Under the new grammar a root entry that starts with neither `_` nor
a digit is foreign and harmless to the reader, so the file's only cost is
noise; but the plan's rule is per-file and its inventory is short by two.
Suggested: state the disposition once for *any* root `FORMAT` (the same
contents test `k11:38-41` already gives), and correct the inventory in the
brief and in `k11`. Related, for `node-methodology-k8`: the shipped spine's
`plugins/grove/skills/grove/references/driver.md:65-66` still says the driver
creates "the root `BRIEF.md` stub … and the format witness"; the sentence is
one `k8` rewrites for the grammar anyway, and the witness clause should go
with it rather than survive into the new text.

### F3 — `node-files-k7` has no green intermediate seam, and its body does not say what a session that cannot finish does (actionable, low)

`node-files-k7` flips the parser, both handle constructors, the tree module,
every writer, the CLI help and every fixture in one commit, and owes two books.
The roots it changes are reconstructed byte for byte — `task_name.rs` (1714
lines), `task_tree.rs` (2038), `tree_lifecycle.rs` (2725), `task_grow.rs`,
`verbs.rs`, `cli.rs` — and the inline `#[cfg(test)]` modules in four of them
are inside the `grove-loop` book's corpus (only `task_grow/tests.rs` is
excluded), so fixture edits there are book edits too. Fifteen `grove-llm` test
files, two `grove-loop` test files and two `grove` test files spell the old
grammar by hand. That is the largest single session in the plan by some
distance.

The spine's remedy for a leaf that proves too big is `leaf-decompose` at a
working seam, and `distinguished-names-k6:67` invokes it. `k7`'s body does not,
correctly: with no dual-grammar reader (`plan-k1` decision 7, planning's own
*Context*) there is no child of `k7` that lands green — the parser and every
fixture flip together or `scripts/check.sh` is red. What the body should say
instead is the route that does exist: a session that reaches its bound without
finishing ends without retiring and without a signal, the working copy stays
snapshotted in `@`, the loop stops, and the next `node-files-k7` session
continues from `jj diff` rather than from the brief. Left unsaid, an
implementer either runs long or decomposes into a red child. For `k6` the
review question's "smaller independently working boundary" does exist and is
worth naming as the seam to decompose at if needed: supplied distinguished
names (trait signature, `promote` and `initialize` inputs, canonical-rendering
identity, every caller) land green on their own; read-level and projected-level
validation with the conformance samples is the second increment.

### Checked and found sound

Recorded so the integrator knows the review's scope, not as findings.

- **Q1, library leaf.** `k6` lists the reader, planner, conformance kit,
  reference domain, syllabus CLI, every test domain and Grove's two adapters;
  the fixed-name factory it removes is `EntryName::distinguished()`
  (`crates/ordinal-fs-tree/src/name.rs:478`, implemented at
  `crates/grove-loop/src/task_name.rs:904`); Grove keeping its filename
  behaviour means its `validate_distinguished` permits zero or one `BRIEF.md`
  in `k6`, which is a domain policy and not a library compatibility surface.
  The syllabus CLI is in the `ordinal-fs-tree` book's corpus as a declared
  addition, so `k6`'s "expect `ordinal-fs-tree` and `grove-loop`" is right.
- **Q2.** `k6` covers read levels and projected final levels for append,
  batch, insert, initialization and its entries, promotion and its optional
  child, and rewrites; the library's independent at-most-one; root-versus-node
  placement; fixture-supplied expected verdicts; and a node-parts policy test
  for the gap finding 049 records.
- **Q3.** `k7` restates the ` — brief` heading rewrite as guarded, idempotent,
  custom-heading-preserving and non-rolling-back (`node-grammar-k5` decision
  3); the conditional interrupted-decompose advice with no observed-sibling
  claim and no second read (decision 7); and classification after validation
  (decision 5). No `migrate` verb, dual reader or automatic conversion appears
  in any body; converted copies in `k10` and the rename workflow in `k11` are
  explicitly non-shipping.
- **Q4, books.** Every book root the plan touches is in a manifest: `k6` →
  `ordinal-fs-tree` and `grove-loop`; `k7` → `grove-loop` and `grove-llm`
  (`cli.rs` help spells `BRIEF.md` at `:67-208`); `k8` normally none; `k9`
  prose only. The worry at `k10:70` about a version edit reaching a book is
  moot: crate manifests inherit `version.workspace = true`, and the v20.2.0
  cut (`lmznxvrxrllr`) touched only the root `Cargo.toml`, `Cargo.lock` and
  `CHANGELOG.md`, none of which is a root. `docs/USAGE.md` and `CONTEXT.md`
  are cited by every book only for anchors, which `k9` preserves.
- **Q5, remainder.** The epoch admission a new `grove-llm` meets inside a
  20.2.0-launched session compares only the ambient signal path against the
  `session.epoch` record (`driver_lease.rs:52-56,716-746`); no version is
  recorded, so a new binary retiring under the old driver is admitted unless
  `k7` changes that module. The plugin marketplace is GitHub
  `Linkuistics/grove` with `autoUpdate` on, so the refresh route is the push
  the release procedure already performs; `k10` leaves the route to be
  prepared rather than inferring it, which is right.
- **Q6.** `k11` re-enumerates at execution time, keeps drivers stopped until
  verification, compares identity, content digests, order and node-file bytes
  item by item, handles partial completion without double conversion, never
  infers a slug from a body, and verifies by opening each real tree with the
  installed binary. The deep case measured today is 1013 bytes at the same
  depth the brief measured, so the motivation still stands. Migration commits
  land in three different repositories — `grove.gh-issue-12` is a workspace of
  this repo, `APIAnyware.add-ocaml-target` of `../APIAnyware`, the other two
  are default workspaces — which `k11`'s "identifiable jj boundary" per
  workspace already accommodates.
- **Ordering.** Positions encode library → grammar → methodology → docs →
  cutover → migration; each body places any earned review before its
  dependent; the cutover is the last code leaf; migration alone signals.

## Decisions (running log)

**1. Three findings, all actionable; one integration leaf.** F1 corrects a
handoff an implementer cannot execute as written; F2 corrects a false
inventory a chartered *do not guess* rule will trip over; F3 names a
continuation route the largest leaf needs and a seam the library leaf can
decompose at. `integrate-review-planning` is inserted with the bare slug
`node-grammar` at `distinguished-names-k6`, the first following live sibling;
its body names this review's handle and not the findings.

**2. Inspection only, no in-session reviewer.** Nothing was built or run;
every source claim above is read off the tree at the producer's commit, and
every machine claim off directory listings and file contents.
