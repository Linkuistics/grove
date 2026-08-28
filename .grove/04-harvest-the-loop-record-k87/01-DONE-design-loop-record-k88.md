# loop-record-k88

## Goal

Land `docs/loop-record.md` — the durable, cited structural record of how this
grove's loop behaved — together with the **committed instrument that derives it**
and the cost account in raw form. One session per row, one row per session: kind,
node, position, key, outcome, the commit that carries it, and when.

The record is the parent's first and fourth `Done when` clauses. It is not the
yield answer and not the lesson verdicts; both are later children that read this
record rather than re-walking the tree.

## Context

**The deliverable is the instrument as much as the table.** *Derived, not
transcribed by hand, with the derivation reproducible* is a condition on the
artifact, so a script is owed and it is committed. `handoff-audit-k66` states the
local precedent in as many words — *the sweep that establishes it is one page of
Python worth re-running here rather than trusting* — and `k66` was itself a
`design` leaf. A design session writing its own instrument is the convention
here, not an exception to it.

**Read `.grove/` from history, not from the working tree.** This is the decision
that makes the artifact survive its subject. Grove's finish cycle deletes
`.grove/` wholesale, but it deletes it *from the tip, not from history*
(`references/commit.md` says so explicitly, and it is verified: the tree at this
grove's scaffolding commit lists thirty `.grove/` paths and their bodies read
back in full). So the instrument takes a **revision** and reads the tree at that
revision. Pointed at the working copy it still works today and stops working the
day the grove finishes; pointed at a pinned revision it works forever. The
rejected alternative — copying `.grove/` into `docs/` as raw data — buys the same
reproducibility for ~940 KB of duplicated process state and forks the record from
its source.

**Four ways the obvious instrument reads clean and lies.** All four are measured;
the node brief carries them in full and they are not restated here. The two that
shape the code rather than the invocation:

- The filename grammar is `NN-[DONE-|ABANDONED-]<kind>-<slug>-k<key>.md`, and
  **kind and slug are both hyphenated**, so the split is a longest-prefix match
  against the closed set of nineteen (`content/TASK-FORMAT.md`). Nothing else
  separates them. `TASK-FORMAT.md` also fixes the failure mode: an unknown or
  missing kind is *malformed and stops the operation*, never a silent degrade to
  `impl` — the instrument must inherit that strictness, because the grove
  immediately before this one used an entirely different grammar and would parse
  into plausible garbage.
- The join from a leaf to its commit is on the full `<slug>-k<key>` **handle**,
  which is what `references/commit.md` obliges every task commit message to
  carry, and never on the key alone.

**Controls, in both directions, and the instrument reports them rather than
asserting them.** Two counts read identically when the join is broken and when
the tree is simply irregular, so both are output, not swallowed:

- every parsed leaf either resolves to exactly one commit in the window or is
  listed as unjoined, with its handle;
- every commit in the window either resolves to exactly one leaf or is listed as
  unattributed, with its subject line.

Neither list is expected to be empty — framing commits carry no handle, and
abandoned leaves never ran — and that is the point: an instrument whose residue
is empty on the first run is more likely broken than perfect.

**Positive and negative controls.** The positive control is a handle known to be
present that must be found in the window (`experiment-baseline-k4`, the grove's
first executed leaf). The negative control is a handle known to be present in the
*commit store* but outside this workspace's ancestry, which must **not** be found
(`blinded-read-k27`, a different workstream sharing the same jj repository). A
broken instrument reads clean everywhere; clean-here plus dirty-there cannot be
produced by one.

**The report goes to `docs/`, and the counts are frozen before it is written.**
`finish-verdicts-k65` burned a handle by letting a sweep's report live inside the
sweep's own subject. This artifact is outside `.grove/` for that reason, and the
instrument reads a pinned revision rather than the working copy, so writing the
report cannot move the numbers the report states.

## Done when

- `docs/loop-record.md` exists and carries, derived rather than transcribed: one
  row per leaf in this grove — position, node path, kind, slug, key, outcome
  (live / `DONE` / `ABANDONED`), and the commit and timestamp that carry it —
  plus the node inventory, plus the up-front-plan-versus-grown-tree comparison
  the scaffolding commit makes available.
- The **cost account is present in raw form**: sessions, commits, and elapsed
  wall time per phase and for the grove as a whole, stated so the write-up can
  quote it without re-deriving it.
- The derivation script is committed, takes the revision to read as an argument,
  runs from a clean checkout, and **prints its own controls** — the positive
  control found, the negative control absent, and both unjoined residues
  enumerated rather than counted.
- The record states, in its own voice, **what it does not establish**: it counts
  sessions and commits, not effort, tokens, or wall-clock attention; a commit's
  timestamp bounds a session, it does not measure one; and a leaf that never ran
  has a body and no commit by construction.
- Nothing under `.grove/` is edited beyond this leaf's own body and the node
  brief this session already wrote.
- The next child is cut, with the yield question's *specific* instrument hazard
  written into its body rather than a goal sentence.

## Notes

**Do not answer the yield question here, and do not adjudicate a lesson here.**
Both are the next children's, and both would be cheaper to do badly in this
session than to do properly in their own. This leaf produces the enumeration they
stand on and stops.

**Elapsed time between commits is an upper bound on a session and nothing more.**
Sessions in this grove were driven by a loop that launched them back to back, so
the gap between consecutive task commits is close to a session's wall time — but
a gap that spans a night is a human going to bed, not a session running for nine
hours. State the distribution and the total; do not report a mean as if it were a
per-session cost.

## Decisions (running log)

**The leaf was decomposed rather than run long, and the trigger was measured
rather than assumed.** The parent's condition was *if the derivation turns out to
need its own instrument with its own controls, it is a child and not a section.*
Four ways the obvious instrument reads clean and lies were found before any code
was written — a shared four-workspace commit store with colliding key spaces,
seven prior groves in the same directory, a superseded filename grammar, and
findings recorded in the integration body rather than the review's. The node
brief carries all four.

**`.grove/` is read at a revision, and the pin is a change id.** Reading the
working tree would make the artifact stop working the day finish deletes its
subject; copying `.grove/` into `docs/` would duplicate ~940 KB of process state
and fork the record from its source. So the script takes `--rev`. The pin is a jj
**change** id rather than a commit id, because in jj the working copy *is* a
commit: writing this document rewrites the commit it is being written into, so a
commit-id pin would be stale the instant it was saved. That is the
self-measurement hazard in miniature, and it also made the derivation
non-idempotent until it was fixed — running the script twice produced two
different files. It is now verified to reproduce itself byte-for-byte.

**No ADR.** The revision-pinning decision fails `ADR-FORMAT.md`'s AND test on the
first clause: `--rev` is an argument, so the choice is a one-line change and not
hard to reverse. It is recorded where it lands — in the script's docstring, in
the generated document's own prose, and here.

**Three claims the first draft asserted and the instrument now derives.** Each
read as plausible and was wrong: the node child-count omitted child *nodes*, so
`alloy-models-k6` reported zero children; the node origin column collapsed *was a
node at scaffold* with *was a planned leaf that got decomposed*, hiding the
decomposition rate the table exists to expose; and "every review earned an
integration" was inferred from two totals that happened to agree rather than
checked as a pairing. The pairing is now matched in both directions.

**A pre-existing test failure was fixed, and it is named here because it is not
this leaf's work.** `cargo test --workspace` failed on
`every_repository_markdown_reference_resolves` —
`docs/formalism-findings.md:3474` wrote `](../adr/…)` from a file already inside
`docs/`. It is the exact defect `handoff-audit-k66` enumerated and never fixed,
because `k66` was abandoned. The file is untouched by this session otherwise, the
broken form occurs exactly once, and the same file uses the correct `](adr/…)`
form elsewhere. Fixed rather than externalised, because leaving it would land
this commit on a red suite and make the next session unable to tell who broke it.
The suite is now green across the workspace.

**The next child is `review-yield-k89`, and it is cut with the hazard that
decides its answer.** Findings are recorded in at least six notations across the
nine chains, three chains record them *only* in the integration body, one
integration enumerates nothing at all, and the word *findings* is overloaded
inside a single review body. A file-scoped counter would report a decaying yield
curve — the opposite of the truth. All of that is written into `k89`'s body
rather than left for it to rediscover.

**A record generated inside its own history must be keyed on identifiers the act
of writing does not perturb, and it took two failures to find them all.** The
document was pinned and its commits named by commit id, which meant: writing the
file rewrote the commit, which changed the id, which changed the file. Sealing
the commit and regenerating did not converge — it oscillated. Two changes fix it,
and both generalise past this artifact:

- **Name commits by jj change id, not commit id.** A change id survives every
  amend, squash and rebase; a commit id is rewritten by all three.
- **Read the *author* timestamp, not the committer's.** jj resets the committer
  timestamp on every rewrite, so a cost account built on it drifts a minute
  further from the truth each time the commit is touched. The author timestamp
  records when the work was done, which is also the number a cost account wants.

Verified rather than argued: regenerating from the sealed commit now leaves the
working copy clean. `python3 scripts/loop-record.py --rev wzmyysov` is a no-op
against its own output.
