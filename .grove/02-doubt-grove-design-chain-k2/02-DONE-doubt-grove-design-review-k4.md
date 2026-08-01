# doubt-grove-design-review-k4

**Kind:** review-design

## Goal

Adversarially review the doubt/Grove composition design and try to disprove that
it satisfies the confirmed requirements without violating Grove's spine.

## Context

Review the artifact produced by `doubt-grove-design-k3` against the root brief.
Pay special attention to stable-key preservation, current-leaf identity after a
move, atomic failure, jj/git symmetry, chain-node semantics, stateless routing,
warning accuracy after configuration changes, and recursive-review escape
hatches. Produce findings only; do not fix the design.

## Done when

- Every root requirement is traced to a falsifiable design behavior or flagged
  missing.
- Counterexamples cover interrupted promotion, first/last sibling placement,
  already-chained leaves, wrong task kinds, same-target routing, restart, and
  integration/research exclusions.
- Findings are specific, severity-ranked, and recorded in this leaf for
  `doubt-grove-design-integrate-k5`.

## Notes

Assume the design author was overconfident. Do not validate or summarize.

## Findings

Artifact reviewed: `docs/specs/doubt-grove-review-mechanics.md` and
`docs/adr/grove-owns-escalated-review.md`, against the root brief's six
`Done when` clauses. Evidence is cited to the code the design says it builds on.

### Requirement trace

| Root `Done when` | Traced to | Verdict |
|---|---|---|
| R1 one consistent boundary across all kinds | spec §Behavior by session kind | **partial** — F2, F3, F11 |
| R2 atomic promotion, handle preserved, review next | spec §Atomic promotion, §Producer handoff | **partial** — F1, F5, F6, F7 |
| R3 one in-session reviewer; second need promotes | spec §Solution + table | **partial** — F2, F11 |
| R4 Grove owns routing; warn unless both differ | spec §Advisory target diversity | **partial** — F3, F4, F8, F9 |
| R5 tests cover the six areas | spec §Test seams | **partial** — F10 |
| R6 five surfaces tell one story | *nothing* | **missing** — F12 |

Counterexamples attempted and **not** found: first/last sibling placement (the
node reuses the producer's own position, so no sibling shifts, and rank-0
`BRIEF.md` ordering is unaffected); restart *after* a completed promotion (pick
correctly returns the producer at child `01`); the null-model branch (a `None`
model arises only from `launch.model_args.is_empty()`
(`src/loop_driver.rs:915`), i.e. it is a property of the harness, not of
configuration — the design's harness-scoped-default rule holds).

### F1 — an interrupted promotion inverts pick order (severity: high)

The spec orders the mutation as: create the brief-less node and the generated
leaves, *then* move the producer "as the final fallible mutation". Between those
two steps the tree is:

```text
05-sync-design-chain-k21/
  02-sync-design-review-k22.md      # live
  03-sync-design-integrate-k23.md   # live
05-sync-design-k12.md               # live, still outside
```

`sort_key` (`src/tree_id.rs:313`) ranks both position-`05` entries at `(1, 5, …)`
and tie-breaks on the name, where `05-sync-design-c…` < `05-sync-design-k…`. So
`pick` descends the node first and returns **`02-sync-design-review-k22.md`** —
the loop launches a review of an artifact whose producer has not run.

The spec disclaims only "power-loss transactions". This window does not need
power loss: the loop driver itself escalates to SIGKILL, Ctrl-C reaches the whole
foreground process group, and the harness may kill a tool call — all ordinary.
Rollback covers *reported* errors only, so none of these produce the "error
naming the exact paths" the spec promises; they produce a silently runnable
wrong-order tree.

The design also never states *why* move-last is mandatory: `roll_back`
(`src/tree_grow.rs:186`) is safe precisely because everything inside `node_dir`
was created by the run, so one recursive remove "can never eat a pre-existing
entry". Once the producer is inside, that argument fails and the rollback would
delete it. So the two safety properties conflict — rollback safety demands
move-last, crash residue demands move-first — and the spec analyses one and is
silent on the other. An unstated invariant is one a later step re-arms: the same
document adds post-move writes elsewhere (F5).

*What would satisfy this:* state the invariant, and close the window rather than
disclaim it — e.g. assemble the whole node outside `.grove/`'s parse namespace and
land it with one directory rename, or make the residue unpickable by construction.

### F2 — the canonical methodology still teaches the boundary this design replaces (severity: high)

R1 asks that the guidance "encode one consistent boundary". The spec's only
mechanism is a test seam asserting the skills "cover every row of the behavior
table" — a *coverage* assertion, which passes with a contradiction sitting
beside the covered rows. Three contradictions are live right now:

- `content/driving.md:404-423` teaches the doubt pass with "**Stop** … after
  three rounds" and step 4's fix-and-reconcile loop. Inside a picked leaf the
  design allows one reviewer and promotes on the first substantive actionable
  finding, so the three-round bound is dead text.
- `content/driving.md:437-441` gives a *different* escalation trigger — artifact
  too big for a subagent, different vendor wanted, findings more than a
  session's work — with no mention of the mechanical "second review need" trigger
  that the spec makes decisive.
- `content/driving.md:476-477` states "There is no retrofit verb: it has not come
  up often enough to earn one", and teaches the two-call hand retrofit. The
  design's entire premise is a retrofit verb. The spec never names this text.

Likewise `plugins/linkuistics/skills/doubt-driven-development/SKILL.md`: its
Verification checklist requires "At least one fresh-context review per
non-trivial **artifact**" and RECONCILE says actionable findings mean "Change it,
re-loop". A leaf making three non-trivial decisions cannot satisfy both that
checklist and the leaf-wide single-reviewer rule. The spec asserts the leaf-wide
unit but never says which of the doubt skill's own steps are suspended inside a
picked leaf.

*What would satisfy this:* name the files and the specific passages that change,
and make the seam contradiction-shaped (assert the old trigger text is gone), not
coverage-shaped.

### F3 — "is this session executing a picked leaf?" has no stated discriminator (severity: high)

Both the ADR and the spec hinge every rule on "executing the leaf returned by
Grove's pick step", and both explicitly reject the available proxy (`.grove/`
present in the checkout). Nothing in the design says how a session establishes
which side of that line it is on. This is the predicate the entire composition
branches on, and it is left to agent self-report.

It is sharper than an omission, because the same design introduces
`GROVE_SESSION_HARNESS` — a variable that *is* set by `grove do` and by nothing
else — for an unrelated purpose, and never connects it. Meanwhile the natural
edge cases are unaddressed: a human interrupting a loop session to do unrelated
side work is still inside a `grove do` session; a nested `grove do` sees both.

### F4 — a present receipt is treated as proof, but the environment is inherited (severity: high)

The spec reasons from absence: "A missing harness means the producer is being
retired outside the loop." The converse — presence means *this* loop launched
*this* session — is what the receipt actually relies on, and it is false.

`src/launch.rs:225-256` states the rule this repo already learned: "the
environment is inherited by every descendant — so the authority is ambient unless
each spawn scopes it deliberately", with `LOOP_CONTROL_ENV` and
`scrub_loop_control_env` as the remedy, and "a stale, unrelated PID leaking into
a nested grove" named as the class of mistake. `GROVE_SESSION_HARNESS` /
`GROVE_SESSION_MODEL` are that class exactly. Concretely: this repo is a
meta-grove whose `cargo test` runs as a descendant of a live session
(`tests/env_hygiene.rs:1-13`), and a nested `grove do` or any tool that shells
out inherits the outer session's values.

The failure is worse than the one handled: an absent receipt yields
`uncheckable`, which is honest; a *stale inherited* receipt yields a confident,
wrong `diverse` or a spurious warning. The design adds neither variable to
`LOOP_CONTROL_ENV` nor to `tests/support/mod.rs:113 grove_env_names()`, and does
not say how a receipt is bound to the session that actually launched.

### F5 — `leaf-retire` gains a cross-file write with unspecified failure semantics (severity: medium)

"Before the producer receives its `DONE` infix, retirement writes the producer's
actual Grove launch target into the linked review task." Three consequences the
spec does not address:

- It contradicts the current-state glossary: **DONE infix** (`CONTEXT.md:592`)
  says `leaf-retire` leaves "its file contents … untouched". The design names no
  glossary edit.
- Retire runs after *every* leaf and is documented as mechanical bookkeeping that
  asks nothing. It can now fail on an advisory-only feature — a read-only file, a
  review leaf renamed by a concurrent `leaf-insert`. The spec guarantees the
  *warning* never blocks launch; it never says the *receipt write* never blocks
  retire. If it can, a diagnostic stalls the loop.
- Finding the linked review means a tree-wide scan of file **contents** for
  `**Reviews:** <handle>`, since position is explicitly not the relation. `pick`
  and `read_level` are documented as never reading contents
  (`src/tree_read.rs:35-44`). Two claimants (a re-run `leaf-add-chain` on the
  same stem, a copied leaf) leave the target unspecified.

### F6 — the already-reviewed refusal cannot see legacy or hand-cut chains (severity: medium)

Promotion "refuses … a producer already related to a scheduled review", and the
relation is metadata by design: "The metadata, never a filename suffix or mutable
position, is the machine-readable relation." §Compatibility then concedes that
chains built by an older binary or by hand carry no such metadata.

So: a producer sitting in a pre-existing `-chain` node is not detectably related,
and promotion nests a chain inside a chain — a review of a review, silently. The
`-chain` token cannot rescue it (ordinary slug text by rule), and the design does
not reach for the one structural signal available: the producer's parent being a
brief-less node, the same file test the Retire cascade already uses.

### F7 — the kind gate rests on the reader that is designed to degrade (severity: medium)

Promotion "refuses … `research`, `combine-research`, `review-*`,
`integrate-review-*`". `read_kind` (`src/tree_read.rs:279-311`) maps a missing,
empty, or unrecognised `**Kind:**` token to **`Kind::Impl`** with a warning —
deliberately, so a typo cannot jam the unattended loop. A `review-design` leaf
whose kind line reads `reveiw-design` therefore presents as a producer and is
promotable. The spec states the refusal in kinds without saying which of the two
readers (`Kind::parse`'s write-side gate, or the degrading read) supplies them.

### F8 — under default configuration the diversity warning fires on every review (severity: medium)

Warn if *either* axis matches. The harness axis matches whenever no
`GROVE_<KIND>_HARNESS` / `GROVE_<FAMILY>_HARNESS` override is set, because
`harness_override` (`src/loop_driver.rs:672`) then falls through to the stamped
harness for both producer and review. That is the default state, and it is also
the permanent state for anyone who has one harness installed.

So the warning is not an exception signal, it is a constant. This repo already
rejects that pattern explicitly, in the `work`-alias rationale at
`src/tree_read.rs:275-278`: a diagnostic that fires on every task file of every
live grove "train[s] the operator to ignore this diagnostic".

The model axis does not soften it: whenever harnesses match the warning fires
regardless of model, so the model comparison can only decide the case where the
harnesses already differ — where a shared exact selector string across two
vendors is rare. The design presents two axes; one carries essentially all the
behavior.

Note this traces to R4's own wording ("warns unless both … differ"), so the fix
may be a requirement question rather than a design patch — flagging for
`doubt-grove-design-integrate-k5` to escalate rather than silently redesign.

### F9 — the warning is emitted where a TUI harness is about to erase it (severity: medium)

"emitted once immediately before an actual review spawn." The next thing that
happens is `Command` spawn of a full-screen harness (`src/loop_driver.rs:321`).
The precedent line — `grove: launching {} (model: {})` at
`src/loop_driver.rs:314` — is one line an operator may catch; a multi-part
warning naming two handles, two targets, the matching axis and a configuration
pointer is not. Combined with F8 (it fires on every review by default), the
feature is plausibly always-on and never-read. The design names no other delivery
surface.

### F10 — two test seams cannot catch the failures they are written for (severity: medium)

- Atomicity: "Inject a failure at every create, write, and rename point" covers
  *reported* failures, which is the path rollback already handles. Nothing
  exercises the interrupted (unreported) residue, and no seam asserts `pick`'s
  answer on a residual tree — which is where F1 lives.
- Documentation: "Assert the canonical Grove and doubt skills cover every row of
  the behavior table" is satisfiable while `driving.md` still teaches the
  superseded trigger and the three-round loop (F2). Coverage cannot detect
  contradiction.

### F11 — the reviewer-counting unit is ambiguous, and the spec and glossary disagree (severity: low)

`CONTEXT.md:105-107` says a "**multi-axis review** belongs in the tree". The
spec's table says only "at most one reviewer for the entire picked leaf" and
never mentions the doubt skill's **diverse-lens** pattern (N reviewers, one named
axis each, for a single decision). Under the spec's wording, is diverse-lens one
reviewer or N? Two canonical surfaces, two answers, on a rule whose whole content
is a count.

Related, unstated placement: an `integrate-review-*` leaf must "externalise
substantial redesign as a new producer review chain", but not *where*.
`leaf-add-chain <parent>` appends behind every unrelated live sibling;
`leaf-add-chain <chain-node>` runs it next. The spec leaves the choice — and
therefore whether the redesign happens before or long after the work that needed
it — to the session.

### F12 — R6's surfaces are untraced, and the ADR set is not minimal-coherent for the decisions actually made (severity: medium)

R6 names five surfaces. The glossary is done (`CONTEXT.md:103-124, 155-167`).
The spec names none of the other four: no methodology file (`content/SKILL.md`,
`content/driving.md`, `content/TASK-FORMAT.md`), no architecture section
(`docs/ARCHITECTURE.md#task-kind-taxonomy` and the routing sections the brief
points at), no CLI-help/`docs/USAGE.md`/`docs/CONFIGURATION.md` entry for the new
verb or the two new environment variables. `tests/help_surfaces.rs` will force a
`--help` description to exist but asserts nothing about the prose docs.

Separately, the ADR set records one decision (the doubt/Grove division) while the
design makes a second of the same weight: **persisting a launch fact into the
task tree** rather than recomputing it. The spec argues the point in prose
("without creating a cache, signal payload, route ledger, or other state outside
`.grove/`") — but a receipt is state, and it is state that is neither a task nor a
brief. That is hard to reverse (it changes what a task file is for), surprising,
and a real trade-off, which is the when-to-write test. Deciding it in a spec
paragraph leaves the ADR set describing less than the current design.
