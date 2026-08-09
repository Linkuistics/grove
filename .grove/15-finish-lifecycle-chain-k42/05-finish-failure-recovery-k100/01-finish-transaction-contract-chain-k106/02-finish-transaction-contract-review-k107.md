# finish-transaction-contract-review-k107

**Kind:** review-design
**Reviews:** finish-transaction-contract-k105

## Goal

Adversarially review `finish-transaction-contract-k105` and record concrete findings for its integration step.

## Context

- Binding artifacts: `docs/specs/config-driven-sessions.md` sections
  "Pre-commit transaction and recovery", "Crash and retry semantics", and
  "Scoped Git and Jujutsu commits"; ADR
  `task-tree-transactions-fail-closed`; glossary terms Finish transaction,
  Complete finish cycle, and Tree access lock.
- The producer's first in-session reviewer already disproved four assumptions;
  this scheduled review must attack the integrated corrections, not merely
  repeat the old findings.
- Test genuine commit proof when the generated finish leaf was never committed;
  unexpected Git/jj topology, message, or tree deltas; reserved-name collisions,
  foreign entries and symlinks; index restore/activation failure; atomic
  root-to-quarantine finalization; quarantine disposal failure; and disabled
  Git hooks.
- Review is inspection-only. Record findings here; the integration leaf owns
  every artifact change.

## Done when

- Findings have severity, an exact Git/native-jj/colocated-jj or filesystem
  failure sequence, and the threatened contract, or the task records an
  explicit no-finding result.
- The review proves that `Committed`, `Not committed`, and `Recovery pending`
  are mutually safe across every transition and that no ambiguous state can be
  selected, treated as fresh before commit proof, or rolled back over changed
  history.
- The review checks that post-commit quarantine is cleanup-only and cannot
  become a lifecycle receipt or enter the scoped deletion commit.
- No spec, ADR, glossary, source, or test artifact is changed.

## Result

Eleven findings against the integrated contract: three high, four medium, one
medium-low, three low. The four previously disproved assumptions are **not**
repeated; each finding below attacks the corrections themselves or a boundary
the corrections newly created.

Two structural conclusions frame them:

1. The three dispositions are **safe but not complete**. Every transition that
   the contract *classifies* is safe; the defect is that reachable repository
   states are classified `Recovery pending`, which the contract never exits
   (F1, and F2/F3/F8 as its entry points).
2. The **`Committed` predicate is Git-shaped**. Its "starts from the manifest's
   repository anchor" clause has no referent in native jj, where the deletion
   commit *is* the anchor change rather than a child of it (F3).

## Disposition safety analysis (Done-when 2)

Transitions proven safe by inspection:

- **No pre-commit window exposes task-root absence.** The witness is created
  inside `.grove/` before any evacuation, evacuation moves entries *into* it,
  and the root→quarantine rename is gated on commit proof. Every interruption
  point therefore leaves `.grove/` present and unwalkable, never fresh-shaped.
  The `Fresh tree` partial-scaffold recognizer cannot claim a witness-only root
  because recovery is ordered before format classification.
- **`Committed` is terminal in the right direction.** Once the exact commit is
  proven, no path restores the tree; rollback is unreachable from `Committed`.
- **`Not committed` → live tree is gated correctly.** Witness removal is last,
  after restoration and current-format verification, so a failed rollback
  degrades to a blocking witness rather than a half-tree.
- **`Recovery pending` never selects, commits, or rolls back.** It is inert.

Transitions **not** proven safe:

- **`Committed` / `Not committed` are not disjoint under native jj** (F3).
- **`Recovery pending` has no proven successor state** (F1). "Cannot *yet*
  prove" implies later provability that no mechanism supplies.
- **Driver-side recovery has no specified `Recovery pending` branch.** Spec
  ll. 733-740 gives the bare driver exactly two cases (roll back uncommitted;
  complete forward on proof). The third is absent from both spec and ADR.
- **The retry proof admits a commit from outside this transaction** (F4), which
  the test-seam list at l. 1078 already requires be impossible without the
  contract establishing it.

## Quarantine verdict (Done-when 3)

**Holds.** The quarantine is created only after commit proof, so it cannot
enter the scoped deletion commit; it lives in the VCS-administration control
directory, which is untracked in plain Git (`<canonical .git>/grove/`) and in
native and colocated jj (`.jj/grove/`); and no lifecycle path reads it —
rootless classification consults only task-root absence, and the retry proof is
VCS-side. Caveats recorded as F9 and F10, neither of which turns it into a
receipt.

## Findings

### F1 — `Recovery pending` is a terminal wedge with no specified exit — **High**

**Sequence (plain Git).** `finish-commit` writes the manifest with anchor
`HEAD = A`, evacuates, and runs `git commit --only`. The process is SIGKILLed
after the commit fails, *or* the commit succeeds and the result is lost. Before
the retry, any process in the same worktree commits or amends — a human in
another pane, an editor's autocommit, a CI hook, a sibling agent. `HEAD` is now
`C ≠ A` and `C` is not the handle-named deletion commit. Recovery: `Committed`
fails (not the exact immediate result), `Not committed` fails (`HEAD ≠ A`)
→ `Recovery pending`.

**Sequence (native jj).** Identical whenever any jj command creates a new
working-copy commit (`jj new`, a user `jj commit`, `jj squash`) so the recorded
change is no longer the working copy at the recorded parents.

**State reached.** `.grove/` contains only `FINISHING-finish-kN/`; every
ordinary reader and mutator refuses it; the entire task tree exists only under
the witness. Nothing in the contract ever makes the disposition provable later,
and no command, flag, or documented manual procedure moves the tree out of it.
ADR l. 48-51 states the intent — "keeps the witness unwalkable with an
actionable diagnostic" — but never says what action the diagnostic names. A
fail-closed state without a specified exit is fail-permanent.

**Threatens.** `finish-failure-recovery-k100` Done-when 2 ("leaves a fail-closed
recoverable witness with an actionable diagnostic"): recoverable only in name.
Also the ADR's "Only the matching recovery path is admitted" — here no path is.

**Do not dispose by citing spec l. 1159-1161.** That out-of-scope item disclaims
*preventing* a stale process from committing directly. F1 is about *recovering*
from an ordinary user commit, which the preservation contract exists to
accommodate, and the contract does promise recoverability.

**Shape of the fix (integration leaf's call).** Either an explicit operator
recovery entry point that takes the human's decision as its authority — the
only input that can break the tie the repository no longer can — or an explicit,
documented statement that `Recovery pending` requires manual reconstruction
from the named witness, with the procedure written down.

### F2 — The witness can become tracked; the contract assumes it cannot — **High**

The exclusion of `FINISHING-<handle>` from the deletion commit is a *pathspec*
exclusion, which preserves the witness only while it is untracked. Nothing
keeps it untracked: no ignore entry, no `.git/info/exclude` write, no statement
of the requirement. Spec l. 617-619 already concedes that "a broad Git task
commit may pick up" the finish leaf; the same concession applied to the witness
is fatal.

**Sequence A (wedge).** Finish is interrupted with the witness present. A human
runs `git add -A && git commit` — routine, and the Tree access lock does not
exclude it. Retry: `HEAD ≠` anchor → F1 wedge, now with the task tree tracked
at witness-internal paths.

**Sequence B (phantom deletion).** The witness was tracked by an earlier broad
commit; the finish commit then succeeds. `Committed` requires the result to
leave "no tracked `.grove/`", but the pathspec deliberately excluded the tracked
witness → the successful commit classifies `Recovery pending`. Were the
predicate relaxed instead, the post-commit root→quarantine rename removes
tracked paths from the worktree without committing them, leaving a permanent
unstaged deletion of `.grove/FINISHING-…` in `git status` after the grove is
gone — violating "a successful finish leaves no `.grove/` in the integrated
history" in the opposite direction.

**Threatens.** The `Committed` predicate, the exclusion mechanism, and
`finish-failure-recovery-k100`'s preservation constraint.

### F3 — Native jj: `Committed` and `Not committed` are not disjoint — **High**

The anchor is defined as "jj's working-copy change identity and parent commit
identities **after its preflight snapshot**". `jj commit` finalizes that
working-copy change and starts a new one on top. So the deletion commit **is**
the anchor change, not a descendant of it.

Consequences, both live in the current text:

- `Committed` requires the commit to "start from the manifest's repository
  anchor". In Git that reads as parent == anchor. In native jj the deletion
  commit's parents are the anchor's *parents*; it does not start from the
  anchor. The predicate has no referent, so the adapter must invent one.
- `Not committed` requires that jj "still has the recorded working-copy change
  at the same parents". After a successful `jj commit`, the repository still
  *has* that change at those parents — as a committed revision. Read as a
  revset query on change id plus parents, this is **true after a successful
  teardown**. The first conjunct ("the exact commit is absent") is the only
  thing standing between that and a rollback over completed history — i.e. the
  precise failure the ADR rejects under "Treat a commit command's exit status
  as the boundary", now reachable through the anchor definition instead.

Compounding: the contract asserts "Unrelated changes and the evacuated witness
remain in the successor working-copy commit" for a *partial-fileset* commit,
without stating which of the two resulting changes retains the original change
id. Both the `Not committed` gate and the retry proof ("the committed parent of
the successor working-copy commit") key on that answer, and jj's assignment for
the paths variant is neither cited nor pinned to a version.

**Threatens.** Mutual exclusivity of the dispositions and, through it,
unrelated-work preservation.

### F4 — The retry proof drops every binding to this transaction — **Medium-high**

Path (b) in `Crash and retry semantics` is deliberately self-contained: message
names the handle exactly, delta from its own parent deletes only `.grove/`,
result has no tracked task root. None of the three distinguishes *this*
invocation's commit; the manifest that would bind it is gone by construction.

**Sequence.** Grove A finishes successfully; `HEAD` is its deletion commit
naming `finish-k9`. The working tree stays. A new grove B is initialized in the
same tree — spec l. 558-567 explicitly permits this and permits key reuse — and
grove B commits nothing (grove gates no commits; or `.grove/` was added to
`.gitignore` after grove A). Grove B is small and its finish leaf is also
allocated `finish-k9`. `finish-commit finish-k9` fails or its result is lost;
the confirmed session retries. All three predicates hold against grove A's
commit → idempotent success → the session runs `complete --done` → the loop
stops "cleanly" with grove B's complete live `.grove/` still on disk.

**Threatens.** "No match is a refusal and never signals `done`", and the framing
"narrow command-outcome verification under the current finish invocation" —
nothing scopes it to the current invocation. Spec l. 1078-1079 already *requires*
this case be covered ("include a prior completed grove with reused handles so an
older teardown commit cannot satisfy the new invocation") while the contract
supplies no predicate that achieves it.

**Note for integration.** The obvious repair — bind the proof to the repository
state observed at this `finish-commit` entry — is available even without the
manifest, since the entry-time observation is in the invocation's own memory.

### F5 — An untracked or ignored task root makes teardown permanently impossible — **Medium**

**Sequence.** A user adds `.grove/` to `.gitignore`, or simply never commits it
(grove gates no commits). Finish: evacuation succeeds; `git add -A -- .grove
':(exclude)…'` stages nothing; `git commit --only` has no change to record and
fails; `Not committed` → rollback → refusal. Every retry repeats identically.
The native-jj analogue is an ignored root.

The only contract text covering this — "an externally hand-constructed terminal
tree that has never existed in `HEAD` is refused because there is no deletion
for a focused finish commit to record" (l. 900-904) — frames a reachable,
user-created state as invalid input. It specifies no pre-evacuation validation
that would refuse without mutating, no diagnostic naming the cause, and no
supported way to complete the cycle. The test-seam entry at l. 1131 covers only
the *unborn-repository* case, not an ignored root in a born repository.

**Threatens.** The Complete finish cycle's guarantee that a confirmed teardown
completes; and, via F4, the same condition is what makes the false-positive
retry proof reachable.

### F6 — Same-device quarantine preflight is a layout precondition discovered only at teardown — **Medium**

The quarantine must be on the worktree's filesystem, and a workspace that
cannot supply one "refuses with the live tree unchanged". Safe, but the refusal
arrives after the entire workstream.

**Sequence.** A linked Git worktree whose `.git` gitfile resolves to
`<main>/.git/worktrees/<name>/grove/` — the exact directory Grove's own
control-directory rule mandates (l. 364-367) — with the main repository on a
different filesystem: an external volume, a network mount, or a container
bind-mount where only the worktree is mounted. Months of grove sessions run
fine; the first thing that fails is teardown, permanently, with no fallback.

Nothing validates this at `root-init`, at driver-lease acquisition (which
already resolves the same control directory and already fails early on an
unwritable one, per l. 1035), or at any earlier point.

**Threatens.** Usability of the loop's only routine human gate. The ADR records
cross-device refusal as a rejected *option* rather than as a supported-layout
constraint that must be surfaced early.

### F7 — Hook policy is asymmetric between the migration and finish commits — **Medium**

Hooks are disabled for the finish commit only. The migration commit — also
internal, also unattended inside bare `grove`, also promising to preserve
unrelated Git work, also rolling back from a witness — runs user hooks.

The ADR's justification is backend-neutral and applies verbatim to migration:
"a hook may mutate unrelated working-tree files even when it rejects the commit,
and Grove cannot safely snapshot and restore arbitrary user data."

**Sequence.** A `pre-commit` hook that formats or regenerates files — the most
common hook shape — fires during automatic legacy migration, rewrites unrelated
tracked files, and migration then fails and rolls back. `.grove/` is restored;
the hook's edits to unrelated files are not.

**Threatens.** `Fail-closed transaction and recovery` (migration) and the same
preservation promise the finish rule was written to protect. Either extend
hook suppression to the migration commit or state why migration is different.

### F8 — Colocated jj: preflight snapshot and index preservation are unordered — **Medium**

Two facts are stated and never ordered. The anchor is taken "after its preflight
snapshot"; the colocated adapter "preserves the user's Git index ... before
invoking `jj commit`". A colocated jj snapshot exports to Git, updating the
index and refs.

- If preservation happens after the snapshot, the "untouched index" Grove
  restores on `Not committed` is already jj-rewritten — a state the user never
  had.
- If it happens before, the anchor and the preserved index describe the same
  repository at two different moments, and a rollback restores an index that
  disagrees with the anchor the rollback was licensed against.

**Threatens.** `Not committed`'s third conjunct ("every repository-side
staging/index mutation has been restored, so restoring the task tree is safe")
and the colocated bullet's "restores the untouched index". The test seam at
l. 1064-1065 exercises the failures without pinning the order.

### F9 — Recursive quarantine disposal has no no-follow requirement — **Medium-low**

Evacuation is specified "without following symlinks". Disposal is only
"recursive disposal".

**Sequence.** A foreign root entry `.grove/scratch -> $HOME/notes` — foreign
entries are explicitly expected and explicitly preserved in the witness.
Evacuation moves the link itself. After commit proof the whole root is renamed
to quarantine. Recursive disposal, if it follows the link, deletes outside the
workspace.

**Threatens.** The "cleanup garbage" framing, which is only safe if disposal is
confined to the renamed subtree.

### F10 — Quarantine and auxiliary images have no disposal owner or bound — **Low**

"Stale auxiliaries and quarantines are cleanup candidates only" names no actor
and no trigger; the driver's specified cleanup covers only abandoned `signal-*`
files. A quarantine is a full copy of a task tree. Repeated
interrupted-then-recovered finishes accumulate complete task-tree copies under
`.git/grove/` or `.jj/grove/` with no reaper. Separately, "its exact path is
reported" is reported into a session that terminates immediately afterward, so
in practice no one reads it.

Not a safety defect — recorded because it is the practical weak point of the
otherwise-sound cleanup-only claim.

### F11 — The manifest's per-entry digest is undefined for directories — **Low**

Ordinary root entries are predominantly node *directories*. The manifest records
"every ordinary root entry's type, digest, and recovery location", and rollback
restores entries and then "verifies the complete current-format live finish
tree". What a digest means for a subtree — recursive content hash, nothing,
something else — is unspecified, and it is the one place the contract could
detect external tampering of evacuated content, which it currently neither
promises nor excludes. The test seam names "manifest tampering" (l. 1056)
without a digest definition to test against.
