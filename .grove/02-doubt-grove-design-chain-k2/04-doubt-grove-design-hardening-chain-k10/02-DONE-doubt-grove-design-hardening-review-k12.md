# doubt-grove-design-hardening-review-k12

**Kind:** review-design

## Goal

Adversarially review the hardened doubt/Grove design and try to disprove its
concurrency, interruption, receipt-freshness, and warning-payload guarantees.

## Context

Review the artifact produced by `doubt-grove-design-hardening-k11` against the
root brief and the seven source findings recorded there. Pay special attention
to concurrent promotion, process interruption versus power loss, VCS index
state, launch-window divergence, retries that cannot write a receipt, nullable
model identity, and missing relationships.

## Done when

- Every source finding is traced to a falsifiable behavior or remains open.
- Counterexamples cover two concurrent promoters, interruption at every
  mutation, producer restart under a different target, and every nullable-model
  pairing.
- Findings are severity-ranked and recorded in this leaf for
  `doubt-grove-design-hardening-integrate-k13`; do not fix the design.

## Notes

Assume the hardening author closed the obvious example while leaving a nearby
state transition undefined. Report issues only.

## Findings

Artifacts reviewed: `docs/specs/doubt-grove-review-mechanics.md` (22:01),
`docs/adr/promotion-transactions-fail-closed.md` (22:00),
`docs/adr/review-target-receipts.md` (21:59),
`docs/adr/grove-owns-escalated-review.md` (21:35), and the hardening session's
`CONTEXT.md` edits (21:43), against the root brief and the seven source findings
recorded in `doubt-grove-design-hardening-k11`. Claims about the code are cited
to the seam the design says it builds on; three are backed by executed
experiments, marked **[measured]**.

### Source-finding trace

| # (from `…-hardening-k11`) | Verdict | Where |
|---|---|---|
| 1 concurrent mutators race allocation | **closed for the happy path, recovery unreachable** | H4, H7 |
| 2 power-loss language / durability ordering | **closed in prose, falsified for plain Git** | H2 |
| 3 stale receipt survives a failed finisher | **reversed in spec/ADR, still stated the old way in the glossary** | H1, H9 |
| 4 launch context not bound to the routed leaf | **specified, unimplementable through the current peek** | H6 |
| 5 `null`-model equality across harnesses | **closed** | — |
| 6 Git/Jujutsu landing behavior | **specified for jj, asymmetric and unwitnessed for Git** | H2 |
| 7 warning payload without a relationship | **closed for the warning, reopened for the recovery diagnostic** | H5 |

### H1 — the glossary still states the receipt ordering the ADR rejects (severity: high)

`CONTEXT.md` is a canonical current-state surface, is read every session, and was
edited by the hardening session itself. Two of its entries state the **opposite**
ordering from the spec and ADR written eighteen minutes later:

- `CONTEXT.md:305-306` — "atomically rewrites that review **before** applying the
  producer's `DONE infix`".
- `CONTEXT.md:662-663` (`DONE infix`) — "Retiring a reviewed producer may
  **first** best-effort rewrite its sibling review's `Review target receipt`".

Against `docs/specs/doubt-grove-review-mechanics.md:207-210` ("It applies the
producer's `DONE` rename first. Only after that terminal transition succeeds…")
and `docs/adr/review-target-receipts.md:30-34`, where *Write the receipt before
applying `DONE`* is a **Rejected** option, rejected for causing source finding 3
exactly.

So the surface the loop actually reads still teaches the defect this leaf's
chain was commissioned to close, and it teaches it as current state, not as
history. File mtimes confirm the sequence: `CONTEXT.md` 21:43, the two ADRs
21:59/22:00, the spec 22:01 — the glossary was written first and never revisited
after the ordering flipped.

Same entry, second omission: `CONTEXT.md:300-306` describes `GROVE_SESSION_TARGET`
as carrying "the resolved worktree identity, harness, and nullable model
selector" and gates retirement on "that worktree matches and the producer is
still `Pick`'s answer" — **two** checks. The spec requires **three**
(`spec:246-252`): worktree, routed-leaf handle, factual pick. The routed-leaf
handle — the entire fix for source finding 4 — is absent from the glossary in
both the payload and the acceptance rule.

*What would satisfy this:* reconcile both `CONTEXT.md` entries to the ADR's
ordering and three-check rule before implementation reads either surface.

### H2 — the landing rename's atomicity boundary sits inside `git mv`, and Git's index has no fail-closed witness (severity: high)

The design's portable atomicity argument is one same-parent directory rename
(`spec:146-150`, `adr/promotion-transactions-fail-closed.md:8-13`). In a plain
Git tree that rename is not a syscall grove issues — it is `git mv`, dispatched
by `tree_rename::rename_entry` (`src/tree_rename.rs:41-50`) via
`is_tracked`, which succeeds for a directory holding any staged path. `git mv`
performs `rename(2)` and *then* writes the index. Those two steps are not atomic
together, and the design's own postconditions cover both:

- `spec:154-156` — "the index ends with only the final child path and no
  `PROMOTING-` path";
- `spec:381-382` — "after landing and rollback, assert Git's index has no
  `PROMOTING-` path".

**[measured]** Simulating an interruption in that window (disk rename applied,
index write lost) in a fresh Git repo produces:

```text
disk:    .grove/05-p-chain-k21/{01-p-k12.md,02-p-review-k22.md}   # complete chain
status:  RD .grove/05-p-k12.md -> .grove/PROMOTING-05-p-chain-k21/01-p-k12.md
         ?? .grove/05-p-chain-k21/
commit:  .grove/BRIEF.md
         .grove/PROMOTING-05-p-chain-k21/01-p-k12.md              # committed!
```

The producer session then commits the promotion (`spec:200-201`) and the
**reserved transaction prefix enters history** while all three real task files
stay untracked. No `PROMOTING-` directory remains on disk, so every grove reader
passes and the loop proceeds normally: this is precisely the invisible partial
state the ADR says reserving a visible prefix prevents
(`adr/promotion-transactions-fail-closed.md:17-21`).

The guarantee at `spec:184-188` is scoped to what a grove process observes after
"a filesystem call has returned". That scoping is honest for the filesystem and
silently untrue for the index, which the design elevates to a checked
postcondition two paragraphs earlier. Jujutsu is immune — a plain `fs::rename`
is one syscall and jj keeps no index — so the "Jujutsu-first VCS rule" the
hardening was asked to specify for source finding 6 is not symmetric; it is
sound on one branch and unwitnessed on the other, and the spec presents the two
as equivalent.

*What would satisfy this:* either move the landing rename off `git mv` (land
with `fs::rename` and reconcile the index afterwards, so the atomic step is a
syscall grove owns), or state that the Git index is outside the interruption
contract and delete the two index postconditions that say otherwise.

### H3 — the tree-access lock makes `.grove/BRIEF.md` mandatory, and it is not (severity: high)

`spec:131-139` and `adr/promotion-transactions-fail-closed.md:3-5` put the
advisory lock on "the open root `.grove/BRIEF.md`", entered by **every**
participating reader and mutator before it reads names.

**[measured]** Against the installed `grove-llm 16.4.0`, a `.grove/` with no root
`BRIEF.md` is fully functional today — `pick`, `brief-chain`, `kind`,
`leaf-add`, and `resolve` all exit 0 and behave normally. Nothing requires the
file: `pick` checks only `grove_root.is_dir()` (`src/tree_read.rs:46-48`),
`brief_chain` skips a missing level "silently" by documented contract
(`src/tree_read.rs:86-90, 141-147`), and `resolve_parent_node` had its
`BRIEF.md` requirement **deliberately removed** (`src/tree_grow.rs:477-484`).
Constraint 4 makes every brief lazy and optional; constraint 3 says nothing
validates these files.

So the design converts a currently-working tree into one where every verb fails,
and it does so at the one point grove is most insistent it must not fail: a
malformed or incomplete tree "must never jam" the unattended loop
(`src/tree_read.rs:270-273`, constraint 5). The two escapes are both closed by
the design's own text — creating the file contradicts "The process-scoped
advisory lock changes no artifact bytes"
(`adr/promotion-transactions-fail-closed.md:49-51`), and the `root-init` carve-out
(`spec:137-138`) is justified by "has no root brief to lock", which is a property
of the *verb*, not of the tree.

The reachable states are not exotic: a v1→v2 migrated tree, a hand-made
`.grove/` (constraint 6 says the folder must stand alone), a human deleting a
stub root brief they never filled in, or a checkout mid-rebase.

*What would satisfy this:* name a lock target whose existence is already an
invariant (the `.grove/` directory itself, opened as a directory descriptor), or
state the create-if-absent behavior and amend the ADR line that forbids it.

### H4 — recovery is unreachable through the design's own preconditions (severity: high)

Three statements cannot hold together:

1. `spec:105-107` — "Promotion accepts only a live, currently picked producer".
2. `spec:161-165` — while `PROMOTING-` exists, "`pick`, `kind`, `resolve`, key
   allocation, and every grow/retire verb refuse".
3. `spec:119-123` — a retry "resumes the exact pending transaction for that
   handle".

In the state that needs recovery, (2) makes `pick` refuse, so (1) can never be
satisfied, and `resolve` refuses too, so the handle in (3) cannot be turned into
a path. Worse, after the producer move the producer file lives *inside* the
staging directory, whose name fails `tree_id::parse` (`parse_position("PROMOTING")`
rejects on the first non-digit, `src/tree_id.rs:253-254, 301-306`), so
`read_level` treats it as foreign and never descends — the producer is not a live
leaf at any parsed path.

The spec orders the lock before the pending check (`spec:131-133`) and the lock
before pre-validation (`spec:141-143`), but never orders the **pending check
against the picked-producer gate**. That is the undefined transition. It is not
cosmetic: it decides whether a grove that was interrupted mid-promotion is
recoverable by the documented command or requires hand repair, and
`CONTEXT.md:651-653` promises the former.

*What would satisfy this:* state that promotion resolves a pending transaction
for the named handle **before** any liveness/pick validation, and that the
recovery branch resolves the handle by scanning inside the reserved directory
rather than through `resolve`.

### H5 — the fail-closed diagnostic must name a producer no reader is allowed to compute (severity: medium)

`spec:161-164` requires every refusing reader to name "the producer handle and
the recovery command". Neither source is available:

- The staging directory name yields the **node's** handle
  (`PROMOTING-05-sync-design-chain-k21` → `sync-design-chain-k21`), not the
  producer's `sync-design-k12`; they carry different keys by construction
  (`spec:74-79`).
- Before the producer move the handle exists only in the generated leaves'
  `**Reviews:**` line — file *contents*. `pick` is documented as never reading
  them (`src/tree_read.rs:44`), and the glossary restates it as an identity
  property (`CONTEXT.md:604`, "It reads no task file contents and carries no
  state"). `spec:449-451` re-affirms readers are unchanged.
- After the move the handle is the name of the child at position `01` — inferring
  it means trusting position, which the same document forbids for the warning
  payload (`spec:300-302`, "never infers a producer from sibling position").

This is source finding 7's defect re-created one layer down: the hardening fixed
the *warning* to say `producer=unknown` with a reason, and then wrote a second
diagnostic contract with the same uncomputable payload and no `unknown` fallback.

*What would satisfy this:* either give the reserved directory a name that carries
the producer handle (`PROMOTING-<producer-handle>-<final-node-name>/`), or drop
the producer from the reader diagnostic and name only the reserved path plus the
recovery command.

### H6 — the routed-leaf handle cannot come from "that exact routing peek" (severity: medium)

`spec:236-241` requires `GROVE_SESSION_TARGET` to carry "the stable handle of the
leaf used by that exact routing peek", and `spec:240-241` requires the launch
diagnostic to render "that retained value rather than performing a second pick".
That handle is the whole fix for source finding 4.

The routing peek is `grove-llm kind --with-harness`, a subprocess whose stdout
contract is one kind token plus an optional harness name
(`src/loop_driver.rs:1154-1204`). It emits no handle. The two ways to get one:

- **Extend the peek's output.** That is a `grove-llm` CLI contract change, and
  the design's Canonical surfaces list (`spec:426-430`) names
  `grove-llm --help`, `grove-llm leaf-promote-chain --help`, `docs/USAGE.md` and
  `docs/CONFIGURATION.md` — not the `kind` verb. It also lands in a protocol this
  codebase already treats as skew-prone: `resolve_kind` carries two explicit
  stale-binary guards (`src/loop_driver.rs:1174-1203`), `grove_llm_bin()` is
  overridable by `GROVE_LLM_BIN` (`src/loop_driver.rs:1221-1233`), and
  driver-version skew has its own named prior work
  (`src/loop_driver.rs:1239`, `driver-version-skew-k11`). A stale `grove-llm`
  printing two lines leaves the handle absent with no stated disposition; the
  safe answer is `uncheckable`, but nothing says so.
- **Walk the tree in-process**, as `picked_leaf` does today
  (`src/loop_driver.rs:1045-1050`) — which is a second pick at a different
  instant, exactly what `spec:240-241` forbids and exactly the divergence window
  the routed handle exists to close.

The design asserts a property whose only implementation is a contract change it
does not schedule.

*What would satisfy this:* add the peek's output contract to Canonical surfaces,
specify the added line, and state that its absence yields `uncheckable`.

### H7 — path-form and handle-form retries diverge after promotion, and the concurrent case takes the undefined branch (severity: medium)

`spec:65-67` accepts either "the absolute path returned by `grove-llm pick` or a
stable key/handle". `spec:119-121` grants idempotence only to "A retry by stable
producer **handle**".

Two concurrent promoters both peeked the same tree, so both hold the **path**
`…/05-sync-design-k12.md`. The lock serialises them correctly; the second then
finds that path gone (the producer is now `…/05-sync-design-chain-k21/01-sync-design-k12.md`)
and takes an unstated branch — a bare "no such path" from `resolve_leaf_file`, not
the `changed: false` the design designed for. So the design's own headline
concurrency scenario resolves through the one argument form its idempotence
clause excludes, and the divergence between the two accepted forms is never
stated.

The design's test seam for this (`spec:368-372`) asserts the waiter *waits* and
that keys stay unique — both of which hold — and asserts nothing about what the
waiter then returns.

*What would satisfy this:* state the path-form behavior after a completed
promotion (resolve the stale path to its relocated producer and report
`changed: false`, or refuse with the handle to retry by), and add it to the
concurrency seam.

### H8 — a legacy *flat* chain is invisible to both already-scheduled detectors (severity: medium)

`spec:112-115` detects an existing review two ways: stable relationship metadata
when present, and "an immediate parent with no `BRIEF.md`" as the compatibility
signal. `spec:437-438` concludes "A producer already inside a brief-less legacy
chain is not promotable even without metadata."

Both assume a legacy chain *is* a node. It was not. `CONTEXT.md:132-141` records
the reversal explicitly — "_Avoid_: 'a chain gets no node directory of its own' —
reversed" — so chains cut before that change are three **flat siblings** under an
ordinary decomposition node or the grove root. Such a producer's immediate parent
has a `BRIEF.md`, so the structural signal does not fire.

Nor does the metadata path, for a reason the spec creates itself: the only
relationship-scan it defines is retirement's, scoped to "the producer's sibling
tasks **inside its brief-less chain node**" (`spec:277-279`) — a scope that is
empty for a plain producer, which is the only kind promotion accepts. Promotion's
own lookup scope is never stated.

The same gap covers hand-cut chains, which `content/driving.md:476-477` still
teaches as the supported retrofit and `CONTEXT.md:152-155` still warns against by
name. Result: promotion nests a review chain inside an existing flat one, silently
scheduling a review of a review — the precise failure source finding 6 of the
*first* review pass identified, migrated to a shape the fix does not cover.

*What would satisfy this:* state promotion's relationship-scan scope (the
producer's whole sibling level, cardinality-checked), and correct the
Compatibility claim to cover flat legacy chains.

### H9 — receipt freshness rests on an ordering argument that ordinary hand-editing breaks (severity: medium)

`spec:210-213` derives the freshness guarantee purely from write ordering: "Under
this protocol a valid receipt is never present beside a live producer, so a prior
session whose terminal rename failed cannot leave an authoritative target for a
later finisher to preserve."

The derivation only constrains grove's own writes. Grove guides and does not gate
(constraint 5): "A task may be done by hand, reordered, or skipped." A human who
un-retires a producer — dropping the `DONE` infix, reverting the promotion commit,
`jj undo` — restores exactly the forbidden state, a live producer beside a valid
receipt naming the previous finisher's target.

What happens next is unstated. `spec:209-210` says retirement "makes a
best-effort atomic **rewrite**"; `CONTEXT.md:305-306` says it "atomically
rewrites"; neither says whether an existing receipt is overwritten or preserved.
If preserved, source finding 3 returns intact — a stale authoritative target that
a later finisher cannot displace. The test seam list (`spec:383-388`) covers zero
and duplicate claimants, an interrupted rewrite, a failed `DONE` rename, and a
failed post-`DONE` write — but no case where a receipt already exists.

*What would satisfy this:* state that retirement unconditionally replaces any
existing `**Producer launch:**` line, and add the pre-existing-receipt case to
the seam.

### H10 — the receipt/relationship agreement rule has no behavior and no oracle (severity: low)

`spec:259` and `spec:299` both require the receipt's `producer` field to agree
with the leaf's `**Reviews:**` declaration, and
`adr/review-target-receipts.md:24-26` repeats it. Nothing states what happens when
they disagree — `uncheckable`, warn-with-both, or trust the relationship — and no
entry in the seam list (`spec:383-388`) constructs the disagreement. A stated
constraint with no failure behavior and no test is a comment.

Adjacent, same shape: `leaf-decompose` of a `review-*` leaf moves its
`**Reviews:**` and `**Producer launch:**` lines into the new node's `BRIEF.md`
(`src/tree_lifecycle.rs:75-81`), and the generated first child inherits kind and
harness but no relationship (`src/tree_lifecycle.rs:83-96`). The launched leaf
then has neither, so the comparison degrades to `uncheckable`. Safe, but unstated.

### H11 — the tree-access lock is absent from the surfaces that must describe it (severity: low)

The lock is the most invasive mechanism the design introduces: it changes the
contract of every reader and mutator, gives `pick` a new blocking behavior, and
(per H3) a new failure mode. Canonical surfaces (`spec:412-430`) tells
`CONTEXT.md` to cover "Review chain, ownership discriminator, promotion
transaction, launch receipt, and the `DONE` side effect" — the lock is not in the
list, and `CONTEXT.md` contains no lock entry today.

Two concrete consequences the design also leaves open: acquisition "waits for the
current operation" (`spec:135-136`) with no bound and no "waiting for…"
diagnostic, so a human running `grove-llm pick` in a second terminal hangs
silently; and the driver's three per-iteration pick sites have three different
failure dispositions — `resolve_kind` bails the whole loop with a task-kind
diagnostic that would be a red herring for a pending transaction
(`src/loop_driver.rs:888-895`), while `picked_leaf` and `routed_leaf` degrade to
`None` and swallow the signal (`src/loop_driver.rs:1027-1038`). The design
specifies none of the three.

### Counterexamples attempted and not found

Recorded so `doubt-grove-design-hardening-integrate-k13` does not re-derive them:

- **Rollback dirtying the Git index.** **[measured]** `git mv` into the staging
  directory followed by the inverse `git mv` leaves `git status` completely
  clean, so `spec:176-178`'s "byte-for-byte" rollback claim holds for tracked
  producers in plain Git.
- **The happy-path landing corrupting the index.** **[measured]** Staging a
  tracked producer and landing the directory with `git mv` leaves the index
  holding only `…/05-sync-design-chain-k21/01-sync-design-k12.md`, with the
  generated leaves untracked and no `PROMOTING-` path — exactly `spec:154-156`.
  The defect in H2 is confined to interruption.
- **Worktree identity drifting through a symlinked invocation path.** Both sides
  resolve the worktree via `repo::toplevel(std::env::current_dir()…)`
  (`src/launch.rs:21`, `src/llm_cli.rs:765`), and `current_dir` returns the
  physical path, so `vcs_of`'s un-canonicalised ancestor walk
  (`src/repo.rs:30-42`) sees the same string on the driver and the session. No
  spurious `uncheckable`.
- **`null`-model identity being ambiguous.** `resolve_launch` yields
  `model: None` only from `launch.model_args.is_empty()`
  (`src/loop_driver.rs:915`), a property of the harness, so `default(<harness>)`
  is well-defined and `spec:261-266`'s equality rule is implementable as written.
- **Keys hidden inside the reserved directory being reissued.**
  `collect_names_into` skips foreign names and does not descend them
  (`src/tree_grow.rs:591-604`), so keys under `PROMOTING-` are invisible to
  `next_keys` — but the fail-closed scan refuses key allocation first
  (`spec:161-165`), which closes it. Correct as designed.
- **`leaf-insert` splitting a chain during promotion.** The node reuses the
  producer's own sibling position (`spec:80-81`) and the exclusive lock
  serialises the renumber, so no sibling shift can interleave.
