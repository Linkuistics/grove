# grove-flip-k28 — brief

## Goal

Increment 2 — **the flip**. grove's tree modules are deleted, grove supplies a
domain implementation of `EntryName`, and every verb that reads or mutates
`.grove/` runs through `ordinal-fs-tree`. Trees in flight are unaffected because
no on-disk name changes: the flip is a pure refactor, and the root brief says so
as a *premise*, which makes it falsifiable rather than assumed.

## Done when

- Every grove verb that touches the task tree runs through the library, and
  grove's own tree algebra is gone: `src/tree_id.rs`, `src/tree_read.rs`,
  `src/tree_grow.rs`, `src/tree_rename.rs`, and the algebra inside
  `src/tree_lifecycle.rs` and `src/tree_access.rs`. What survives is the part
  that was never algebra — the lifecycle around the tree, the transaction
  sentinels, migration, and the driver.
- grove's dependency line reads `default-features = false`, and something
  *checks* that the library's imposed dependency set is `libc` — the claim is
  worthless unheld by a test.
- grove's roughly 130 CLI-contract tests pass. Every test that had to change is
  recorded in *Findings* below with the reason it had to, because in a pure
  refactor a changed test is a finding rather than an adjustment.
- The four inherited questions are each answered by the leaf that owns them,
  with a record that outlives `.grove/` — an ADR where the answer constrains
  future work, `docs/formalism-findings.md` where a model was involved, and a
  doc comment where it is local.

## Decomposition

**Expand → migrate → contract**, because this is the wide refactor
`references/decompose.md` names as the exception to vertical slicing: grove's
tree layer has one shape and the library has another, and no single verb can
carry both. Every child leaves grove building and its tests green, because the
old modules stay in place until their last caller has gone.

| # | leaf | stage | what it flips |
|---|---|---|---|
| 01 | `domain-k29` | expand | grove's `EntryName` implementation, checked against `conformance`. Consumed by nothing yet. Owns questions **2** and **4**. |
| 02 | `refusals-k30` | expand | How grove renders the library's refusals, plus the refusal-**reachability table** for grove's verb set, written before any of them flips. Owns question **3**. |
| 03 | `reading-k31` | migrate | `pick`, `select`, `brief-chain`, `kind`, `resolve` and the read guard onto `fs::read` + `Snapshot`. |
| 04 | `marking-k32` | migrate | `leaf-retire`, `leaf-prune` onto `rewrite`. The first mutation, and the first rename. Owns question **1**. |
| 05 | `growing-k33` | migrate | `leaf-add`, `leaf-add-pair`, `leaf-insert` onto `append`, `append_many`, `insert`. |
| 06 | `promotion-k34` | migrate | `leaf-decompose` onto `promote`. |
| 07 | `lifecycle-k35` | migrate | `root-init`, `materialize-finish`, `transition-to-current`, `finish-commit` — grove-only lifecycle, rebuilt on the library's write path. |
| 08 | `migration-k36` | migrate | `tree_migrate` off `tree_id` and onto the domain name type. Migration is out of the library's surface but its *output* is a current-format tree. |
| 09 | `sweep-k37` | contract | Delete the dead modules and check the deletion the way `references/execute.md` requires a repo-wide claim to be checked. |

**Reading leads the migrate stage** because it is the largest consumer group and
the only one that has to solve path construction: the library's reading surface
returns no paths (`docs/ordinal-fs-tree/CLI.md`, *What `cli-k16` should watch*),
so grove builds them itself — the root's own spelling, plus each ancestor node's
rendered name, plus the entry's — in **one** place, which every later leaf then
uses. Do not answer this by adding a `path()` to the algebra; `cli-k16` refused
that and said why.

**Marking precedes growing** because it is the narrowest mutation that touches
every layer — one `by_key`, one `rewrite`, one rename — so question 1 is
answered against the simplest possible case rather than inside the sibling
shift.

**Migration is second-to-last** because `tree_migrate` is the last thing holding
`tree_id` alive, and the sweep cannot run until it lets go.

## The four inherited questions, and who owns each

The root brief carries each with its evidence. This table is the assignment, and
it is the reason no leaf has to re-derive one.

| # | question | owner | what the answer must produce |
|---|---|---|---|
| 1 | the version-control-aware move is gone | `marking-k32` | a decision between re-staging after a library rename, accepting the changed `git status`, or something else — and whichever it is, the test changes it forces, named as such |
| 2 | grove's name grammar is not canonical | `domain-k29` | either a tightened grammar whose refusal names the canonical spelling, or a knowingly waived obligation with the waiver recorded |
| 3 | the library's refusals collide with grove's vocabulary | `refusals-k30` | a decision between printing verbatim, grove re-wording, and reopening `docs/adr/entry-name-is-the-only-seam.md` — the seam record names grove as the reopening condition |
| 4 | `default-features = false` on the dependency line | `domain-k29` | the manifest line **and** the check that the imposed dependency set is `libc` |

## What a flip leaf owes as regression evidence

grove's existing suites are this increment's **safety net, not its deliverable**:
`leaf`, `session_kind_tree`, `composition_verbs`, `leaf_ops`, `kind`,
`jj_tree_verbs`, `resolve`, `pick`, `root_init`, `brief_chain`, `tree_access`.

- Every flip leaf runs the **whole** suite, not the suites it thinks it touched.
  A pure refactor's blast radius is exactly what nobody predicted correctly.
- **A test that has to change is a finding.** Record it here, in *Findings*, with
  what changed and why the change was forced. A test edited quietly is the flip's
  premise being falsified without anyone noticing.
- **One exception is pre-authorised**, because it is question 1 by construction:
  the `git mv` assertions in `tests/leaf.rs` and `tests/jj_tree_verbs.rs` move
  when the rename primitive does. Pre-authorised means *expected*, not *unrecorded*
  — `marking-k32` still writes down what it changed them to.
- Where a leaf reaches for a formalism, it appends to `docs/formalism-findings.md`
  before it retires, against the fixed six-field format, counterfactual included.

## Findings

Each leaf appends here as it lands: changed tests, and anything the flip
falsified.

**`domain-k29` (expand).** No existing test changed, and none had to: the whole
suite went from 1206 passing to 1228, the twenty-two new ones being this leaf's
own. The flip's *pure refactor* premise is unfalsified so far, which is the least
this stage could produce and is worth stating as such.

- **Question 2 is answered by tightening**, and the record is
  `docs/adr/task-names-are-canonical.md`. The refusal names the canonical
  spelling, which is what makes the cost recoverable — a hand-typed `5-` halts
  the tree until one `mv`. Nothing on disk changes, so the premise holds.
- **Question 4 is answered**, and it is two claims rather than one:
  `tests/library_dependency.rs` holds that the library's transitive imposed set
  is exactly `libc` with the `cli` feature off, that grove's line actually asks
  for that (`uses_default_features: false`, read out of `cargo metadata`), and —
  the control — that the feature flag is doing observable work, since `clap` is
  in the graph without it.
- **One visibility widening, and no behaviour change with it.**
  `tree_access::FINISHING_PREFIX` and `PREPARING_FINISH_PREFIX` became
  `pub(crate)` so `task_name` classifies the sentinels from the same constants
  rather than a second spelling of them. `MIGRATION_TRANSACTION` already was.
- **The architecture's claim that a conforming domain can check itself against
  the seam *without reading the architecture* did not hold here, and the way it
  fails is worth the next domain's attention.** The kit reported conforming over
  a fixture of ten real `.grove/` names while this domain's canonicity check was
  disabled — its check is `format(parse(f)) == f` over the filenames it is
  handed, and a lenient grammar is invisible without a lenient sample. The kit's
  *not exercised* finding cannot see the gap either, because the other listings
  parsed. This is not a defect in the kit so much as a limit on what any coverage
  report can say, and `docs/formalism-findings.md` entry 020 carries the
  measurement and the counterfactual. **What a later flip leaf should take from
  it: a green kit is a statement about a fixture, so mutate the thing it checks
  before believing it.**

**`refusals-k30` (expand).** No test changed and none could: this leaf touched no
`src/`, and the suite is 1228 passing, byte-identical to what `domain-k29` left.
The flip's *pure refactor* premise is untested here rather than unfalsified, and
the distinction is worth keeping — a design leaf is not evidence about a refactor.

- **Question 3 is answered: grove prints the library's refusals verbatim and
  re-words none of them**, because the reachability table shows the collision
  reaching an operator through exactly **one** ordinary argument — `leaf-add` /
  `leaf-add-pair` given a task file as `<parent>` — which grove already refuses
  itself and must keep refusing, since `.grove/BRIEF.md` carries no key and
  cannot be handed to the library as a target at all. The seam ADR is **not**
  reopened and **no library-side leaf is cut**. The records are
  `docs/ARCHITECTURE.md#library-refusals` (the decision, the table, the rule) and
  a rework of `docs/adr/entry-name-is-the-only-seam.md` in place, whose reopening
  clause was met by grove and did not fire; `CONTEXT-MAP.md` gains the two terms
  a library message uses that neither glossary mapped — *distinguished child* and
  *entry*. **No new ADR**: the AND test fails on *hard to reverse*, and `cli-k16`
  recorded the identical decision for the syllabus in `CLI.md` rather than in the
  record set.
- **This brief's own framing of the collision was wrong in its detail, and the
  correction is the finding.** Composing the message against grove's glossary
  clause by clause gives four of six clauses **true** — the library's *leaf* and
  *node* pick out exactly grove's **Leaf** and **Node directory** inside
  `.grove/`. What breaks is *which holds nothing* and, decisively, *promote it
  first*: an operation grove's verb set does not have, delivered to an operator
  who is an LLM and will try it. **The collision is in the verb the message
  names, not in the nouns it uses**, which is why pre-empting one refusal is
  cheaper than re-wording ten.
- **What every migrate leaf transcribes** is three clauses, and breaking one makes
  the table wrong rather than incomplete: resolve the argument to an entry and
  call **by key** against the same snapshot; **classify before calling**, using
  the library's own predicate read off the snapshot (*contents are `Some`*) and
  never a second one (*the path `is_dir`*); write **no second wording** for a
  condition the library states. Reading leaves get one more: no read verb can
  produce a `Refusal` at all, so `pick`, `brief-chain`, `kind` and `resolve` keep
  the diagnostics they already have — `CLI.md` had its read verbs *construct* a
  `TargetMissing` for want of a message of their own, and grove has one.
- **The table runs both ways** — a verb-side half naming each verb's library
  operation and the refusals it can reach, and a refusal-side half naming the
  argument that reaches each variant — so a migrate leaf reads its own row rather
  than re-deriving the set. Nine verbs of thirteen touch the algebra at all.
- **The count, so a migrate leaf knows what to expect.** Four `Refusal` variants
  of ten are reachable — `TargetNotNode`, `DestinationOccupied`, `KeysExhausted`,
  `OrdinalsExhausted` — and only the first from an ordinary argument. Two things
  that surprised: `NoOccupantAtOrdinal` is unreachable in **all three** of its
  messages, because `leaf-insert` names the *entry* whose slot is taken where the
  syllabus's `insert` names an ordinal; and the leaf brief's expectation that
  grove's hand-edited trees reach refusals the syllabus could not is **false** —
  hand-editing widens which trees reach a refusal, not which refusals are
  reachable. The argument surface was enumerated mechanically (17 clap-parsed
  fields across 13 verbs, no ordinal among them, with a positive control) rather
  than read, because a table seven leaves transcribe is worth the control.
- **No review chain is cut.** The table's own check is scheduled and stronger
  than an adversarial read: each migrate leaf transcribes its own rows into a
  suite and finds them wrong if they are, exactly as `cli-k16` confirmed
  `CLI.md`'s. `docs/formalism-findings.md` entry 021 carries the instruments, the
  measurement, and a falsifiable prediction stated in entry 017's terms.

**`reading-k31` (migrate).** The whole suite is **1230 passing**, up from
`refusals-k30`'s 1228 — the two new ones are this leaf's own equivalence tests,
and no test was added to cover a hole. **Three existing tests had to change, and
none of them for a reason the flip could hide.** `pick`, `select`,
`brief-chain`, `kind` and `resolve` now read one snapshot under the library's
guard; `src/task_tree.rs` is the new module and `src/tree_read.rs` keeps only
`select_unlocked`, `resolve_unlocked` and `read_level`, which the verbs still
under grove's exclusive guard cannot reach the library from.

- **The premise is held directly rather than argued.** Both readers are live at
  once during this stage, which is the only window in which *pure refactor* is
  falsifiable by experiment instead of by inspection, so `tree_read`'s own tests
  now drive **both** over one fixture and assert they agree — thirteen reference
  forms for `resolve`, and the selected leaf for `select`. They pass, and they
  die with `tree_read`. **What a later flip leaf should take from it:** while a
  verb group is mid-flip you have two implementations of one contract for free;
  an equivalence test is cheaper than the review that would otherwise stand in
  for it, and it fails rather than reassures.
- **Changed test 1 — `pick_orders_numerically_not_lexically` made its point with
  a name grove never writes.** It seeded `2-impl-a-k1.md` against `10-…`, which
  `domain-k29`'s canonical grammar now refuses
  (`docs/adr/task-names-are-canonical.md`). Reseeded as `99-…` against `100-…`,
  which is the *stronger* case anyway: zero-padding is a minimum width, so three
  digits is where a lexical sort actually breaks, and the walk orders on the
  parsed ordinal rather than on the name. The old spelling was testing the
  lenient grammar's tolerance, not the ordering.
- **Changed tests 2 and 3 — the species-mismatch wording is now the domain's.**
  `pick_refuses_a_species_mismatch_at_a_task_shaped_name` (unit) and
  `a_task_shaped_entry_of_the_wrong_species_is_malformed_not_foreign`
  (`tests/session_kind_tree.rs`) asserted *declares a leaf* / *declares a node
  directory*, which was `tree_read::read_level`'s own sentence. The condition,
  the recovery advice and the blast radius are unchanged; the words are
  `task_name::TaskNameError::SpeciesMismatch`'s — *names a leaf* / *names a
  node*. This is `refusals-k30`'s *write no second wording* arriving as a test
  edit, and it is the shape every later flip leaf should expect its own message
  assertions to take.
- **One test that did not change but had to be re-aimed:**
  `llm_cli`'s *one CLI command observes the tree exactly once* counted
  `tree_access` acquisitions, and a flipped verb takes none. It now sums both
  counters. The property is about observations, not about which module took
  them, and stating it that way is what keeps it meaningful for the rest of the
  stage.
- **Path construction is `task_tree::entry_path`, and it is the only one.** The
  root's own spelling, each ancestor node's rendered name, the entry's — no
  `path()` on the algebra, exactly as `cli-k16` refused. Nothing canonicalises
  for output. Canonicalisation appears once, inside `leaf_entry`, purely to
  *compare* a caller's spelling of a leaf argument against the tree's, which is
  what the path-walking reader did too.
- **The pending-transaction refusals could not come from the grammar alone, and
  the reason is ordering rather than mechanism.** `task_name` does classify all
  three sentinels `Verdict::Reserved`, and that halt is live. But a tree with a
  pending migration is a tree mid-conversion, so its *other* names are legacy —
  task-shaped, no session kind, therefore `Malformed` — and the library reports
  whichever it meets first in sorted order, which is the legacy leaf. The
  operator would be told to fix a filename in a tree that needs migrating. So
  `task_tree::diagnose` re-states a **failed** read in grove's own precedence:
  root, pending, format, then the library's message. What holds the two spellings
  together is that `tree_access::refuse_pending_*` now raises `task_name`'s own
  `TaskNameError` — the identical value the library carries — so the pre-check
  and the halt cannot drift.
- **The library's invisible locking costs a consumer its contention
  diagnostic, and this is the seam finding of the leaf.** `ARCHITECTURE.md`
  (library) makes locking invisible deliberately: no try-variant, no timeout,
  `read` blocks. Nothing in that interface can say *someone else holds this*, and
  grove has always said it. `task_tree::announce_contention` buys it back
  outside the library — a non-blocking probe in the same mode on the same
  directory, one message, released — and it is a diagnostic and never a decision,
  so the window between its release and the library's acquisition costs a message
  and nothing else. **The write path will need the same and cannot reuse this
  one**: an exclusive probe must be released before the library's own exclusive
  acquisition or it deadlocks against itself. Recorded in
  `docs/ARCHITECTURE.md#tree-access-lock`, under *Two locks, one at a time*,
  along with the per-verb rule the nesting hazard forces.
- **`resolve` fits the seam without friction, and the leaf brief's worry was
  unfounded.** Slug lookup is a `walk` with grove's predicate over grove's own
  `Parts`, and ambiguity needs the whole walk rather than `find`'s first hit —
  which is a sentence of code, not a workaround. Nothing was added to the
  library and no `docs/formalism-findings.md` entry is owed: this leaf reached
  for no formalism, and the seam's narrowness was felt as *state the predicate
  yourself*, which is what it is for.
- **One tightening that is not a wording change.** `kind <leaf>` used to answer
  for **any** task-shaped file anywhere on disk — it parsed the filename and
  never checked containment. It now requires the leaf to be in the tree, because
  the answer comes from the snapshot. No test covered the old behaviour and no
  caller relied on it; it is recorded here because a flip that quietly narrows a
  verb is exactly what this section exists to catch.

**`marking-k32` (migrate).** The whole suite is **1236 passing**, up from
`reading-k31`'s 1230; six are this leaf's own. **Two existing tests had to
change** and both for the same reason — a message that is no longer grove's to
choose — and one of the two carried a finding that corrects this node's own
inherited table. `leaf-retire` and `leaf-prune` mark through `rewrite`, and
`src/tree_rename.rs` has no caller left from either.

- **Question 1 is answered: grove accepts the changed `git status` and stages
  nothing**, and the record is `docs/adr/grove-does-not-stage-its-own-renames.md`.
  What an operator sees between a grove verb and the commit is ` D` at the live
  name beside `??` at the marked one, where a `git mv` once showed a staged
  rename; the commit is unaffected **provided it stages the tree**, and a
  `git commit -a` records the deletion alone. Re-staging was rejected as the
  deleted primitive reassembled one layer up — it needs the same trackedness
  probe issue #3 was about, a jj branch of its own, and `git add` stages the
  file's *current content*, which `git mv` never did. All three outcomes are
  asserted in `tests/leaf_ops.rs` against a real git repo, because this working
  tree is jj and the whole question is about the git lane.
  `docs/ARCHITECTURE.md`'s *Version-control seam* and
  `content/references/commit.md` carry the consequence; the methodology sentence
  is one clause and it is the only reason the hazard is not silent.
- **The atomicity problem was accepted, not escalated**, and
  `docs/adr/bulk-marks-are-not-atomic.md` says what an operator does with a prune
  that stopped half way: run it again, because the marks are the state and an
  already-`ABANDONED` leaf is skipped silently. **What survived the change is the
  up-front validation** — the whole subtree is planned and checked against the
  *first* guard's snapshot before any rename, so the all-or-nothing property the
  suite has always held still holds against every precondition; only the window
  *between* guards is lost. `pruning_a_node_takes_one_guard_per_mark` asserts the
  count, so the cost is a number and not a paragraph.
- **This node's own reachability table had a wrong row, and the leaf that
  transcribed it is what found that out.** `DestinationOccupied` is **not**
  reachable from `leaf-retire` or `leaf-prune`. The occupying name must be
  exactly the name the mark would place, and an outcome infix and a key are both
  parts of one name — so the `DONE` twin the row names necessarily carries the
  live leaf's key, and there is no tree where the destination is taken and the
  key is not duplicated. Four variants are still reachable; every one of them is
  now a grow verb's. `docs/ARCHITECTURE.md#library-refusals` is corrected in
  place. **This is `refusals-k30`'s scheduled check firing as scheduled, and it
  fired on the first leaf to run it.**
- **A duplicated key was silently marking the wrong entry, and the fix is the
  finding every later migrate leaf needs.** `rewrite` is called **by key**;
  `by_key` answers with whichever entry the walk reaches first, and walk order is
  one of `structure.als`'s recorded misses. So `leaf-retire` aimed **by path** at
  a live leaf rewrote its `DONE` twin onto its own name, changed nothing, and
  printed the twin's path as the retired leaf — a success aimed at the wrong
  entry. `task_tree::addressable_key` now refuses a key that names more than one
  entry, before any operation is called. **What a later flip leaf should take
  from it: clause 1's *resolve to an entry, then call by key* is sound only while
  keys are unique, and nothing enforces that on a hand-edited tree. Every verb
  that turns a path or a reference into a key wants this check.**
  `docs/formalism-findings.md` entry 022 carries the instrument and the
  counterfactual — a recorded model miss that had waited two leaves for a
  consumer that could feel it.
- **Changed test 1 — `prune_node_is_atomic_bails_clean_on_a_taken_destination`**
  is now `…_on_a_leaf_it_cannot_address`. Same fixture, same property (nothing
  renamed), different diagnosis and therefore a different sentence: the tree is
  refused for carrying two entries under key 5, which is what is actually wrong
  with it, rather than for a taken destination, which is a consequence.
- **Changed test 2, and the pre-authorised `git mv` assertions, are all prose.**
  There was no `git mv` *assertion* anywhere to move: no test ever checked that a
  mark staged an index entry. What the three named files carried was a claim in
  their headers, and each now says what is true of it. `tests/leaf_ops.rs`: the
  git repo is the marking verbs' *instrument* rather than their prerequisite, and
  four new tests use it as one. `tests/leaf.rs`: still true of `leaf-add` and
  `leaf-insert`, and no longer true of grove as a whole — said so, with the leaf
  that will finish it named. `tests/jj_tree_verbs.rs`: the four colocated
  assertions are **unchanged and still pass**, and two of them now hold for a
  second, stronger reason — the rename is plain on every lane, not plain because
  this one is jj — so they have stopped discriminating the dispatch and are kept
  as a guard against a verb growing a `git mv` of its own.
- **The write seam is `task_tree::write` / `reopen_write`, and the contention
  probe is now mode-aware.** `reading-k31` bought the waiting diagnostic back
  outside the library with a **shared** non-blocking probe; a write must probe
  **exclusively** or the message is swallowed whenever another reader holds the
  tree. A bulk mark announces once and takes its later guards through
  `reopen_write`, because the diagnostic is about the command's wait and not
  about each lock it happens to need.
- **Two messages improved and no test forced it, recorded because a flip that
  quietly changes a verb is what this section exists to catch.** A node
  directory handed to `leaf-retire` used to answer *cannot operate on a node
  directory (lifecycle verbs act on leaves)*, which came from a path check;
  it now answers *cannot retire a node (nodes are never marked done)*, from the
  snapshot. And the paths both verbs return are now built from the caller's own
  spelling of the root rather than from a canonicalised one, which is the
  library's rule and what `reading-k31` already did for the read verbs.
- **No formalism was reached for and entry 022 exists anyway.** The instrument
  was building an inherited table's fixture, which is entry 019's counterfactual
  run from the other end; the models were read only for their recorded misses.
  Worth distinguishing from `reading-k31`, which owed no entry at all.

## Pointers

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the seam, the seven obligations, the
  operation tables and the refusals. `CLI.md` matters more than it looks: its
  *What `cli-k16` found* is the only description of the library's error surface
  from a consumer's side.
- `docs/ordinal-fs-tree/CONTEXT.md` and grove's own `CONTEXT.md` — the two
  glossaries that **collide**. `CONTEXT-FORMAT.md` forbids one term living in
  two, which is why `ordinal-fs-tree` is a third bounded context.
- `docs/adr/entry-name-is-the-only-seam.md` — carries the vocabulary cost and the
  clause that names grove as the condition that would reopen it.
- `docs/adr/entries-are-never-removed.md`.
- `docs/formalism-findings.md`, entries 004–019. Entry 017 measured what a second
  wording costs; entry 019's counterfactual is *write the reachability table at
  design time*.
- `src/tree_id.rs`, `src/tree_rename.rs` and the rest of grove's tree modules are
  **prior art and never authority**. `tree_id.rs`'s grammar is deliberately
  lenient and therefore breaks the library's canonicity obligation; a
  transcription of it is a bug.
- Glossary terms in play: *Leaf*, *Node directory*, *Brief*, *Handle* (grove's
  `CONTEXT.md`); *entry*, *leaf*, *node*, *ordinal*, *key*, *distinguished child*
  (`docs/ordinal-fs-tree/CONTEXT.md`). The first two names appear in both, meaning
  different things.

## Notes

**What the mapping already looks like**, so that no leaf spends a session
rediscovering it. grove's verbs land on the library's operations almost one to
one: `leaf-add` → `append`, `leaf-add-pair` → `append_many`, `leaf-insert` →
`insert`, `leaf-decompose` → `promote` (which optionally creates the first child
in the same unit, which is exactly what `leaf-decompose` promises), `leaf-retire`
and `leaf-prune` → `rewrite`, and `pick` / `select` / `brief-chain` / `resolve` →
`walk` / `by_key` / `ancestors` / `distinguished_chain`. grove's `BRIEF.md` is the
distinguished child. The library allocates keys and ordinals itself, so
`tree_id::next_key` and `next_keys` die rather than move.

**Five facts established while cutting this node.** Each was checked against the
source, and each is here so a leaf can start from it rather than find it.

- **grove has never written a lenient position.** Every on-disk name grove
  produces is rendered by `tree_id::Entry::name`, which formats the position
  `{:02}` — the grow verbs, the lifecycle verbs and `tree_migrate`'s output alike.
  So the only way a lenient spelling enters a tree is a hand edit, and grove's own
  methodology invites exactly that (`references/retire.md` offers *reorder by
  hand*). That is why question 2 needs no corpus-survey leaf of its own, and also
  why tightening is not free. Note that `{:02}` is a *minimum* width: position 100
  renders `100`, so the canonical rule is *zero-padded to at least two digits and
  no other leading zero*, not *exactly two digits*.
- **`FORMAT` is Foreign; the transaction sentinels are Reserved.** `Reserved`
  **halts**, so it is right for `MIGRATING-session-kinds`, `FINISHING-*` and
  `PREPARING-FINISH-*` — which is what `tree_access::refuse_pending_*` hand-writes
  today, and the flip gets it from the grammar instead. It is wrong for
  `.grove/FORMAT`, which is present in every healthy tree and must be skipped.
  Getting this backwards makes every grove command refuse every grove tree.
- **grove cannot nest its own lock inside the library's.** Both `flock` the
  directory *containing* the tree root, and two open file descriptions on one
  directory do not share a lock. So a verb uses one guard or the other, never
  both — which is what makes the migrate stage strictly per-verb-group.
- **A mutating method consumes the `WriteGuard`.** One guard is one operation. So
  `leaf-prune` on a node — which marks every live leaf in a subtree — becomes *N*
  operations under *N* separate guards, losing the atomicity `PruneResult` implies
  today. `marking-k32` owns that: either grove accepts it and says so, or the
  library wants a batched rewrite, which is a change to a checked library and an
  escalation rather than a quiet widening.
- **The consumer surface is small.** Outside the tree modules themselves, the call
  sites are `src/llm_cli.rs` (30), `src/loop_driver.rs` (4),
  `src/repo/migration_commit.rs` (2) and `src/finish_transaction.rs` (2). The
  work is in the modules, not in their callers.

**No review chain is cut here.** A chain is lazy by construction — a producer cuts
`review-*` as its own last act, only once its artifact exists and it judges an
adversarial read necessary (`references/decompose.md`). Cutting them in advance
would buy empty sessions for the leaves that turn out clean.

**The skill-distillation leaf is not in this node.** It sits at the grove root,
after this whole node, because it is the *second* experiment's deliverable rather
than flip work — see `formalism-skill-k38`.
