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

**`growing-k33` (migrate).** The whole suite is **1250 passing**, up from
`marking-k32`'s 1236. **Not one integration-test assertion changed**: all ~130
CLI-contract tests passed untouched, first run, and the only edits to those files
are four prose headers whose claims about `git mv` stopped being true. That is
the strongest evidence this stage has produced for the *pure refactor* premise
and is worth stating as such — every one of those tests asserts what the tree
looks like afterwards, and the library computes the ordinals and keys grove's own
algebra did. Every change with teeth landed in **unit** tests, where the
machinery rather than the outcome was the subject. `src/task_grow.rs`
is the new module; `src/tree_grow.rs` keeps `leaf_add_unlocked` and the
destination guard, which the lifecycle verbs still write through.

- **The library allocates the key, and grove's content embeds it — this is the
  seam finding of the leaf, and no brief predicted it.** `NewEntry` takes its
  bytes *before* the library composes the name, and a grove leaf opens with the
  handle `# <slug>-k<key>`. So a content-carrying domain cannot render its
  content from the answer and must **predict** the allocation: `task_tree::next_key`
  is `max + 1` over the same snapshot the operation plans from, which is the
  library's own rule mirrored on the consumer's side. `tree_id::next_key` was
  supposed to die here; what actually happened is that its **rule** survived in a
  place the seam did not anticipate. The prediction is checked against the report
  and the verb refuses to claim success on a disagreement, because the silent
  failure is a leaf whose first line contradicts its own filename, permanently.
  Recorded in `docs/ARCHITECTURE.md#tree-access-lock` and asserted by
  `every_grow_verb_writes_a_handle_that_matches_its_own_filename`.
  **What a later flip leaf should take from it:** `promotion-k34` and
  `lifecycle-k35` both create leaves with bodies, so both inherit this, and the
  cheapest correct shape is to call `task_tree::next_key` and check the report —
  never to write the body after the operation, which would put a content write
  outside the guard and defeat the atomicity the operation just bought.

- **Two more rows of the inherited reachability table were wrong, and neither
  fell to the instrument that broke the last one.** `TargetNotNode` is **not**
  reachable: the row said *yes* and then named its own contradiction two lines
  later — *Grove keeps its own check in front of it* — and what settles it is that
  grove **cannot** drop that check, since `.grove/BRIEF.md` is an entry carrying
  no key and so can never be a target. `DestinationOccupied` is **not** reachable
  from any grow verb either: an append composes with `max + 1`, so nothing can
  already carry the name; and a shift's only possible occupant is the sibling one
  ordinal higher, which is itself a mover and has already vacated, because the
  renames run highest-first and `Plan::refusal` folds the plan through the
  snapshot in effect order. Entry 003's model result thus discharges a refusal as
  a side effect. The count of reachable variants falls from four to three, and
  the consequential change is that **no algebraic refusal reaches an operator
  from an ordinary argument any more** — `TargetNotNode` was the only one that
  did, and the collision `refusals-k30` weighed was its message. That record's
  decision is unchanged and now cheaper than it looked;
  `docs/adr/entry-name-is-the-only-seam.md` is reworked in place to say the
  pre-emption is *forced* rather than merely available, and
  `docs/ARCHITECTURE.md#library-refusals` carries both corrections.
  `docs/formalism-findings.md` entry 023 carries the protocol finding: entry
  022's own counterfactual (*write the fixture's filenames out in full*) would
  have caught **neither** of these, so what is repeatable is the transcription
  and not the instrument.

- **This leaf's own *Done when* is not fully met, and it could not be.**
  `tree_id::next_key`, `tree_grow::next_child_position` and `collect_all_names`
  still have callers: `tree_lifecycle`'s `root_init` and `materialize_finish`
  allocate leaves while holding **grove's own** exclusive guard, and grove cannot
  nest its lock inside the library's — the node brief's own established fact. So
  they keep the path-walking allocator until `lifecycle-k35`, and
  `leaf_add_unlocked` survives with them. `next_keys` did lose its last
  production caller and is now reached only through `next_key`. The clause was
  written expecting the grow flip to orphan the allocators; what orphans them is
  the lifecycle flip, and the two are different leaves for exactly the locking
  reason that made the migrate stage per-verb-group in the first place.

- **The `git mv` assertion the node brief pre-authorised does exist, and it is
  here.** `marking-k32` found none in `tests/leaf.rs` or `tests/jj_tree_verbs.rs`
  and said so; the assertion was in `tree_grow`'s own unit tests, where
  `insert_moves_the_index_entry_for_a_tracked_sibling` held that a tracked
  sibling's shift staged a rename. It is now
  `insert_leaves_a_tracked_siblings_index_entry_where_it_was` and asserts the
  opposite, which is question 1 arriving at the last verbs that answered it the
  other way (`docs/adr/grove-does-not-stage-its-own-renames.md`).
  `content/references/commit.md` widens *a mark it did not stage* to *a rename it
  did not stage — a `DONE` mark, a `leaf-insert` shift*, and
  `tests/jj_tree_verbs.rs`'s colocated section now names three verbs that hold
  for the stronger reason instead of two. `leaf-decompose` is the one left that
  still discriminates the dispatch.

- **A verb that reports on the tree it changed needs a second guard.**
  `leaf-insert` lints stray position-prefixed cross-references left stale by its
  own renumber, and the tree it must read is the one the shift **left** — a
  shifted node took its whole subtree's paths with it — while the mutation
  consumed the guard that could have shown it. So `task_grow::surface_cross_refs`
  reopens one. Two observations, deliberately, and the property preserved is the
  one that mattered: the output is written while the tree is held, so a hit
  naming a path is a path nothing has renamed underneath it.
  `leaf_insert_lints_cross_references_under_an_exclusive_lock_of_its_own` asserts
  the count is exactly two, so a later change moves a number rather than quietly
  contradicting the paragraph.

- **The lint's scan set narrowed, and it is a narrowing rather than a
  simplification.** It walked `.grove/` for every `.md` file; it walks the
  snapshot now, so what it reads is every leaf and every charter — the same set
  every other verb calls the tree — and a foreign `.md` a hand edit dropped in is
  no longer scanned. Grove writes no such file, and the alternative is a second,
  wider notion of *what is in the tree* than the reader has. The old fixture used
  exactly such a file (`NOTE.md`), so it moved into a real charter;
  `surface_scans_the_tree_and_not_the_directory` pins the new boundary in both
  directions.

- **`tree_read::resolve_unlocked` lost its last production caller here and was
  kept as a test oracle.** The grow verbs' `<parent>` / `<target>` resolution was
  it, so `resolve` is now entirely the library's. Rather than delete the
  path-walking resolver and take
  `both_readers_resolve_every_reference_form_identically` with it — the one
  direct check the *pure refactor* premise gets, and `reading-k31`'s best finding
  — it is `#[cfg(test)]` and labelled as an oracle. Both die in `sweep-k37`.
  **What a later flip leaf should take from it:** when a flip orphans the old
  implementation of a contract that still has a live equivalence test, gate it
  rather than delete it; the evidence is worth more than the lines.

- **Key exhaustion changed hands and grove took the library's message.** The task
  brief asked which survives, and the answer is the library's, unchanged
  (`docs/ARCHITECTURE.md#library-refusals`, clause 3): `Refusal::KeysExhausted`
  is planned from the same snapshot before any effect is built, so *nothing was
  created* remains true even though grove no longer says it. Ordinal exhaustion
  is a **gain** — `next_child_position` added one to a `u32` unguarded, and
  `Refusal::OrdinalsExhausted` refuses. Both are now the only two refusals grove
  can reach, and `a_level_at_the_last_ordinal_refuses_rather_than_wrapping` is
  the first test grove has ever had for the second.

- **`leaf-add-pair` got simpler, exactly as its brief predicted, and the measure
  is what went.** The up-front destination sweep, the `O_EXCL` claim per leaf,
  the per-run rollback list, the injected post-claim failure and its
  thread-local, and `next_keys` — all of them grove's reconstruction of what
  `append_many` and the one interpreter already do. The three tests that pinned
  that machinery were re-aimed at the outcome (`a_failed_run_leaves_the_next_call_a_clean_slate`,
  `an_unwritable_third_destination_refuses_the_whole_run`) or deleted with the
  seam they armed (`a_fill_that_fails_after_its_claim_unwinds_…`, whose window is
  now inside the library's own interpreter and fault-injected there). The
  destination guard itself stayed in `tree_grow`, with its three tests, because
  the lifecycle verbs still write through it.

- **Three refusal messages are now the resolver's rather than the appender's,
  and each says something more precise.** A `<parent>` path that does not exist
  was *not a node directory*; it is now *no entry matches …, tried as a path
  under the grove root and as a key/slug*, from clause 1's resolution, before
  anything is planned. A bare `notes/` directory was *not a node directory*; it
  is now *not a Grove leaf or node directory*, because the grammar disclaims the
  name and no walk reaches it. And the grove root handed to `leaf-insert` was
  *target is not a grove entry*; it is now *cannot insert at the grove root*,
  naming the verb that would work instead. No integration test asserted any of
  the three.

**`promotion-k34` (migrate).** The whole suite is **1257 passing**, up from
`growing-k33`'s 1250, and the seven are this leaf's own. **Not one existing test
changed** — not an assertion, not a name, not a fixture — which makes this the
second consecutive migrate leaf to leave the ~130 CLI-contract tests untouched
and the first to leave the *unit* tests untouched as well. Every existing
`leaf-decompose` test passed on the first run against `promote`. The only edits
to test files are three prose headers whose claims about `tree_rename` stopped
being true. `leaf_decompose` runs through `promote` with the first child in the
same unit; `src/tree_lifecycle.rs` keeps the verb, and
`resolve_leaf_file` and the lifecycle module's `canonical_grove_root` went with
the path-walking it did.

- **`src/tree_rename.rs` has no production caller left anywhere in grove.** This
  was the last one. Every entry that moves now moves inside an `ordinal-fs-tree`
  operation, so the version-control seam that used to live in that module is
  discharged by `docs/adr/grove-does-not-stage-its-own-renames.md` and nothing
  dispatches on the lane to rename. `sweep-k37` deletes the module; until then it
  survives with only its own tests, and `tests/jj_tree_verbs.rs`'s header says so
  rather than continuing to name it as a live seam. **Nothing in that file
  discriminates the jj dispatch any more** — the four colocated cases are now all
  guards against a verb *growing* a `git mv`, which is worth keeping and is a
  different claim from the one the file was written to make.

- **The row this leaf was sent to check is wrong, and what discharges it was
  written two leaves ago for something else.** `DestinationOccupied` is **not**
  reachable from `leaf-decompose`. The algebra reaches it perfectly well — a
  promotion composes the node as `(the leaf's own ordinal, the leaf's own key,
  node parts)`, so a hand-edited tree can occupy that name, which is exactly what
  the row said. But such an occupant is a *node carrying the leaf's key*, so the
  key is duplicated tree-wide and `task_tree::addressable_key` refuses before
  anything is planned. `marking-k32`'s finding retired a row nobody was looking at
  when it was written. The count of reachable variants falls from three to two,
  and both survivors are the grow verbs' keyspace and ordinal-space edges.
  `docs/ARCHITECTURE.md#library-refusals` carries the correction. **This is the
  fourth leaf to transcribe that table and the fourth to find a row wrong** —
  `refusals-k30`'s scheduled check has now fired on every leaf that had a row to
  write, four for four.

- **The check came with a positive control, because a reachability claim that
  cannot fail is worth nothing.** `library_promotion_refusal` calls `promote`
  directly on the same fixture, bypassing every precondition grove puts in front
  of it, and the library does refuse — with `PromoteNotLeaf`, as it happens,
  because `by_key` on that tree reaches the node twin first. *Which* refusal is
  not determined by anything: walk order on a duplicate-key tree is one of
  `structure.als`'s recorded misses, so the control accepts either wording. That
  it is undetermined is itself the argument for grove's check.

- **The recovery advice for the one tree state this library can damage was
  addressed to a reader nobody had, and this is the seam finding of the leaf.**
  `Error::FailedPartiallyRolledBack` states diagnosis and fix in as many words —
  *a node and a leaf sharing an ordinal and a key, with the node holding no
  distinguished child, is an interrupted promotion, and removing either half
  resolves it* — and `refusals-k30` decided grove prints it verbatim. What no
  record noticed is **who is holding that tree when it matters**: never the
  process that made it, which reported and exited, and always a later command, to
  which the library says nothing at all, key uniqueness being an obligation on the
  domain that no operation checks. So the only wording available is grove's — and
  grove's existing one was **wrong**. `addressable_key`'s general *give one of
  them a fresh key* would make two entities out of one entity caught
  mid-shape-change. It now recognises the exact signature and gives the library's
  own recovery instead. **This is not clause 3 broken**: there is no library
  wording in play, the one that exists having been printed by a process that has
  gone. `docs/ARCHITECTURE.md#interrupted-promotion` is the record, `CONTEXT-MAP.md`
  gains the *promote* / `leaf-decompose` row the two messages now share, and **no
  ADR** — the AND test fails on *hard to reverse*, exactly as it did for
  `refusals-k30`'s own decision. `docs/formalism-findings.md` entry 024 carries
  the measurement, and its counterfactual is the cheapest thing in that log per
  finding and is not a formalism at all: *read every error variant whose message
  describes a persistent state of the artifact, and ask which of your own commands
  can meet that state later.* Three of the library's nine `Error` variants do;
  two already had an owner.

- **`leaf-decompose` takes two guards, and the second is the retitle rather than
  a lint.** ` — brief` is grove's own edit to bytes the library moved verbatim and
  never read — it has no content model — so it cannot ride inside the unit, and
  `promote` consumes its guard. It takes one of its own through `reopen_write`
  rather than running on an unheld tree, so no cooperating command meets a node
  brief mid-retitle; the wait was already announced by the promotion.
  `decompose_takes_one_guard_for_the_promotion_and_one_for_the_retitle` asserts the
  count is exactly two. **What a later flip leaf should take from it:** a verb
  whose content edit follows its structural one is the same shape as
  `leaf-insert`'s lint, and the answer is the same — reopen, do not run unheld,
  and do not announce twice.

- **The intermediate state is real and no grove reader can see it, and that is
  held rather than argued.** A promotion has both the node and the leaf on disk
  sharing an ordinal and a key between its first two effects, which is why the
  library's invariants are about *quiescent* trees. Grove's readers cooperate for
  two reasons and `tests/tree_access.rs`'s
  `the_librarys_tree_lock_is_taken_from_exactly_one_module` checks the first by
  enumeration over `src/`: the library's `flock` is `task_tree`'s to take, one
  shared and one exclusive, so no snapshot exists that was not taken under it. The
  second is this node brief's own established fact — grove's surviving
  path-walking readers take `tree_access`, which `flock`s the *same* directory, so
  the two guards exclude rather than nest.

- **Two keys are predicted where `growing-k33` predicted one, and only one of
  them is an allocation.** `promoted` holds the library to both: the node's key
  must be the promoted leaf's own, which is what identity preservation *is* and
  what keeps the retitled `# <slug>-k<key>` handle true of the entry it now names;
  the child's must be the `task_tree::next_key` grove rendered into its template.
  A promotion allocates nothing for the node — the entity is unchanged — so
  `Refusal::KeysExhausted` reaches `leaf-decompose` through the **first child**
  alone, and `a_tree_at_the_last_key_refuses_the_promotion_rather_than_wrapping` is
  the first test grove has had for it on this verb. No ordinal is allocated past
  the end of any level, so `OrdinalsExhausted` is unreachable here.

- **The leaf's one in-session reviewer was spent on the corrected row, and it
  paid — not by breaking the claim but by finding the argument was weaker than
  written.** A promotion's **only** exposure to `DestinationOccupied` is its
  *first* effect: the two later destinations sit inside the directory the plan
  has just created, and `plan.rs::occupied` answers `false` for a
  `Level::Created` unconditionally, so no tree state at all can make them refuse.
  The row therefore rests on exactly one line of grove's code —
  `addressable_key`'s tree-wide twin scan — and `docs/ARCHITECTURE.md` now says
  so, with the four ways that line could be weakened named as what would reopen
  it. **A row whose whole support is one consumer-side check should say which
  check**, which is a rule the previous three corrections did not need and this
  one did.
  - The pass also found `crates/ordinal-fs-tree/src/ops.rs`'s promotion comment
    misleading, and it is **corrected in place**. Its true sentence — *a tree a
    failed rollback already damaged can reach the refusal* — sat in a paragraph
    about the two later destinations and read as being about them. Worth the
    two lines because the row above depends on which effect can refuse. **And
    the model does not settle it**: `wit_damagedTreeStrandsALaterOperation` is
    `outcome == RefusedDestinationOccupied and not(isInsert(…))`, which does not
    distinguish *which effect* refused, so the comment's citation could not
    support the reading it invited. Which effect it is comes from reading
    `occupied`. A recorded gap between what a witness proves and what a comment
    citing it claims, and the second of this node's findings to land there.
  - One finding was classified and **not** acted on: `plan.rs`'s
    `DestinationOccupied` doc naming *a duplicated key* and *a damaged tree* as
    two routes, which on the promote path are one. The variant is shared by every
    operation and the two routes are genuinely distinct for `insert`, so the
    sentence is right where it sits. Recorded as a visible trade-off rather than
    left unstated.

- **The verb's own `--help` text carried a claim that stopped being true, and it
  is the first user-visible string this stage has had to change.** `leaf-decompose`
  described the body's move as *`git mv`, or a plain rename in a jj-enabled tree*
  — a dispatch that no longer exists; it is a plain rename on every lane, staging
  nothing. Two prose claims went with it: `tests/jj_tree_verbs.rs`'s header, which
  named `tree_rename::rename_entry` as a live seam, and the untracked-leaf section
  of `src/tree_lifecycle.rs`'s tests, whose whole premise was that `git mv` had no
  index entry to move. `content/references/commit.md` gains `leaf-decompose` beside
  the `DONE` mark and the `leaf-insert` shift, and it is the one worth naming
  there: what an operator sees is a deletion beside an **untracked directory**,
  which reads less like a rename than either of the other two. **What a later flip
  leaf should take from it:** the pre-authorised exception this brief wrote was for
  `git mv` *assertions*, and three leaves have now found the claims living in prose
  and help text instead — grep the strings, not just the tests.

- **The three refusals `promote` owns are unreachable, and the sweep is what makes
  that a claim about the verb.** `no_promotion_refusal_reaches_an_operator_from_an_ordinary_argument`
  runs every argument that is an entry and not a live leaf — the grove root, the
  root brief, a node, a node's brief, a `DONE` leaf, an `ABANDONED` leaf, a
  `finish` leaf — and asserts none of the three library wordings appears.
  `PromoteNoDistinguished` needs no fixture: it is about the *domain*, and
  `TaskName::distinguished()` is `Some(BRIEF.md)`, asserted rather than assumed,
  the way `docs/ordinal-fs-tree/CLI.md`'s table does it.

**`lifecycle-k35` (migrate).** The whole suite is **1266 passing**, up from
`promotion-k34`'s 1257, and the nine are this leaf's own. **Two existing tests had
to change**, neither for a reason the flip could hide, and one of them is the
node's own instrument moving rather than a behaviour. `root-init`,
`materialize-finish`, `transition-to-current` and `finish-commit` are on the
library's write path; `src/tree_grow.rs` and `src/tree_read.rs` have **no
production caller left anywhere in grove** and are `#[cfg(test)]` until
`sweep-k37`.

- **The root's own creation takes both guards, one after the other, and this is
  the finding the leaf exists for.** The library locks the directory *containing*
  the root — so the lock spans the root's creation and its deletion, which is what
  the brief predicted `root-init` would need — but it still has to *reach* the
  root to snapshot it, so it cannot create one; and a `BRIEF.md` arrives only
  through `promote`, so it cannot create that either. Both are grove's, under
  grove's guard, and the first leaf is the library's, under its. **Nesting them is
  the deadlock this node established in its own Notes**, so the scaffold releases
  the first before taking the second, and `transition-to-current` was forced into
  the same shape because it called `root_init_unlocked` while holding the
  lifecycle guard. `docs/ARCHITECTURE.md#tree-access-lock` carries it under *The
  root's own creation takes both guards*. **What a later flip leaf should take
  from it:** the node brief's rule was *a verb uses one guard or the other, never
  both* — and the correct refinement is *never both at once*. A verb that needs
  what only grove can do and then what only the library can do is a verb in two
  phases, not a verb that has to pick one.

- **The window that release opens is the one `FORMAT` already made legible, and
  the brief's instruction to check rather than assume paid.** The task file asked
  whether `recover_partial_root_init` still recovers and whether the case it
  recovers from still exists. It does — and the case is now reachable *without
  anyone having died*, which it was not before: the tree exposed between the two
  phases is the root plus its charter and nothing else, which is exactly
  `partial_root_scaffold(ROOT_BRIEF)` in the migration transaction's own fixture.
  `phase_one_leaves_the_partial_root_recovery_completes` asserts the shape and
  then recovers it, so the two cannot drift. **The behaviour change is recorded
  rather than hidden**: a concurrent reader in that window is told the tree is
  legacy and must be migrated, where it used to block on grove's guard and then
  read a complete tree. It fails closed, the window is two lock acquisitions wide,
  and one bare `grove` repairs it.

- **Idempotency is load-bearing, not defensive, and the distinction has teeth
  here.** Completing a scaffold appends only when the snapshot holds no positioned
  entry. Appending unconditionally would not collide and would not refuse — the
  second first leaf lands at ordinal 2 with key 2, perfectly legally — so the
  failure mode is a *silently wrong tree* rather than an error.
  `a_scaffold_completed_by_a_recovery_leaves_the_original_nothing_to_add`
  sequences the exact race deterministically, which is better evidence than the
  in-session reviewer it was spent instead of.

- **`recover_partial_root_init_unlocked` deliberately does not go through the
  library.** It runs inside the session-kind migration transaction, which holds
  grove's exclusive guard, so reaching for the library's would be the nesting
  above; and it allocates nothing — ordinal, key, slug and bytes are all fixed,
  and every file is byte-compared before anything is written. It completes a
  scaffold; it does not grow a tree. What it *did* drop is `tree_id`: it composes
  and recognises the scaffold name through `TaskName` now, which is what orphaned
  `tree_grow`'s last lifecycle caller and with it `tree_id::next_key`,
  `next_child_position` and `collect_all_names` — the clause `growing-k33` could
  not meet and named this leaf for.

- **This node's reachability table survived transcription unchanged for the first
  time, and gained a row.** `root-init`'s predicted **none** is right — the root
  is not an entry and the level is empty. The addition is `materialize-finish`,
  the driver's own: an `append` at the root level, so it reaches `KeysExhausted`
  and `OrdinalsExhausted` exactly as `leaf-add` does, and from *no argument at
  all*, the verb taking none. Both are transcribed into tests
  (`a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping`,
  `a_root_level_at_the_last_ordinal_refuses_the_sentinel_rather_than_wrapping`).
  `refusals-k30`'s scheduled check is now **four corrections in five leaves**, and
  the fifth being clean is worth as much as the four: a check that had never
  passed would be measuring the transcriber rather than the table.

- **Changed test 1 — `finish_preflight_refuses_a_reserved_witness_collision_before_deletion`
  is now `finish_commit_refuses_…`, and the refusal moved a layer out.** On the
  library's guard a `FINISHING-*` name halts the tree as `Error::Reserved`,
  carrying `task_name`'s own wording, before `preflight_root`'s *reserved finish
  transaction path* is reached. That is clause 3 arriving as a test edit — one
  condition with one wording, where there were two — and it is the same shape
  `reading-k31` reported for the species-mismatch sentence. The preflight check is
  **kept**: it re-reads the root through its own `O_NOFOLLOW` descriptor rather
  than by path, so it is defence against a writer that ignored the lock rather
  than a duplicate of the guard.

- **`finish-commit` still classifies `.grove` itself before opening the tree, and
  a test is why.** `finish_preflight_refuses_a_symlinked_task_root_before_deleting_the_tree`
  failed the moment the guard came first: a symbolic link to a directory elsewhere
  is a root the library happily *follows and reads*, because every reader follows
  links, while a no-follow teardown must refuse it unfollowed. **What a later flip
  leaf should take from it:** the library's guard is the authority on the tree, and
  it is not the authority on what the caller's spelling of the root is allowed to
  be.

- **Changed test 2 — `the_librarys_tree_lock_is_taken_from_exactly_one_module`
  moved from 4 to 5, and it failed rather than reassured.** The count is that
  test's own control, and adding `task_tree::write_scaffold` moved it. Worth
  naming because of *how* it was found: the first full-suite run after the flip
  reported clean, and the count only failed once the binary was actually rebuilt —
  so the honest total came from measuring the parent revision in a second
  `jj workspace` with the same command, not from reading a summary line. **A test
  count is evidence only when both sides of the comparison were measured the same
  way.**

- **The dead-module claim has a positive control.** *No production caller of
  `tree_grow` or `tree_read` remains* is held by the compiler rather than by a
  grep: both modules are `#[cfg(test)]`, so a production reference does not
  compile. The control was run — a `crate::tree_read::read_level` reference added
  to `llm_cli` fails with `E0433`, and was reverted — because a clean build proves
  nothing unless a dirty one would have failed.

- **No formalism was reached for and none is owed**, which is `reading-k31`'s
  position rather than `marking-k32`'s. The one modelled fact this leaf leans on —
  that `append` composes `max + 1` from the snapshot, which is what makes the
  idempotency check sufficient rather than merely prudent — is entry 003's, and
  `growing-k33` already cited it. `sweep-k37` should read that as one more data
  point for the entry it may owe: three of six flip leaves have now reached for no
  model at all.

**`migration-k36` (migrate).** The whole suite is **1272 passing**, up from
`lifecycle-k35`'s 1266, and the six are this leaf's own. **Not one existing test
changed** — not an assertion, not a name, not a fixture, not a prose header —
which makes this the third consecutive migrate leaf to leave the ~130
CLI-contract tests untouched and the second to leave the unit tests untouched as
well. `tree_migrate` names no item from `tree_id` in production; the module's
recognisers are frozen and private, and its one renderer is `task_name`'s.

- **Two grammars meet in this module and they are deliberately not one — this is
  the seam finding of the leaf, and no brief predicted it.** Migration's *input*
  is a tree written before the live grammar existed and its *output* is a
  current-format tree, so its two halves carry **opposite** obligations. A
  recogniser must never narrow: its false negative classifies a real workstream
  `Format::Empty`, stamps `FORMAT` over it and loses the tree, which is the loss
  `Format`'s own doc comment exists for. A renderer must always track: what it
  writes is read by every verb. Sharing one rule between them satisfies either by
  breaking the other — and the sharing is the *tempting* move, because the rules
  are identical today and the module's own comment said so
  (*"the two grammars' slug rules agree, so there is no second validator to
  keep"*). So `is_legacy_slug` is a deliberate **copy** of `Slug::new`'s rule, and
  what holds the copy honest is a test rather than a call:
  `frozen_legacy_slug_rule_still_agrees_with_the_live_grammar` fails the day they
  part, and failing is the point — it is not a demand that the live grammar stay
  put, it is a demand that someone *decide* what a legacy tree spelled that way
  should do. **What a later flip leaf should take from it:** shared code between a
  legacy reader and a live writer is a coupling that looks like de-duplication,
  and the direction each side is allowed to move in is what tells them apart.

- **The `pure refactor` premise is measured here rather than argued, twice, and
  both instruments die with the sweep.** This leaf's *Done when* asks for
  byte-identical output, and the withdrawn reader is still compiled, so both
  implementations of both contracts are live at once — `reading-k31`'s
  equivalence-test finding applied to a *renderer* and a *matcher* instead of a
  reader. `both_renderers_spell_every_migrated_leaf_identically` drives
  `tree_id::Entry::name` and `TaskName`'s `Display` over one corpus, including
  shapes no fixture reaches (position 0 and 100, `u32::MAX` as a key, every
  outcome). `the_frozen_matchers_admit_exactly_what_the_withdrawn_reader_did`
  runs the frozen matchers against `tree_id::parse` and `tree_id::validate_slug`
  over a **generated** cross product — position form × infix × slug × key form ×
  suffix, five figures of names — rather than a listed corpus, because a listed corpus is a
  second copy of the transcriber's own idea of what matters, and this node's brief
  warns that a transcription of `tree_id` is a bug. Both carry their own control:
  the corpus asserts a floor on how many names it *admits*, since agreement
  between two functions that answer `None` to everything is not evidence.

- **The in-session reviewer was not spent, and what replaced it is stronger.**
  The claim with teeth was the hand-derived node matcher, which is exactly the
  narrow-claim-the-compiler-cannot-reach the allowance is for. A differential
  against the function it replaces settles the same claim by experiment, so the
  allowance went unspent and the four mutants below are what a reviewer would have
  had to argue about instead.

- **Two mutants survived, and they are equivalent rather than uncaught.** The
  matcher's `.md` guard and its outcome-infix guard can both be deleted without
  failing anything: a `.md` tail leaves the terminal key non-numeric and an
  uppercase infix leaves the slug outside the character set, so each is refused a
  line later anyway. The corpus is what establishes this rather than the argument
  — it crosses both features against everything and still agrees. Both guards are
  **kept and labelled redundant in place**, because they state rules a reader
  would otherwise derive, and because a later change to the key or slug rule would
  make them load-bearing without announcing it. The other two mutants failed as
  they should: admitting an empty slug, and perturbing the rendered ordinal (nine
  tests).

- **A lenient *node directory* survives migration, and the decision is recorded
  rather than left in the code.** Leaves are re-rendered — a legacy `5-task-k1.md`
  lands canonically as `05-impl-task-k1.md`, which is what `{:02}` always did and
  is now the domain type's rule rather than a second copy of it. Directories are
  the one entry migration never renames, because a planned file's source and
  destination share a parent and the migration transaction leans on that, so a
  hand-edited `5-node-k1/` passes through and is then refused by name with the
  `mv` in the message. Recognising it leniently and letting it halt **loudly**
  beats not recognising it, which loses the classification and the subtree with
  it. `docs/adr/task-names-are-canonical.md` is reworked in place — the migration
  case its corpus argument did not cover, and *repair the spelling during
  migration* added as a considered-and-rejected option with the invariant that
  rejects it. **No new ADR**: the AND test fails on *hard to reverse*, and the
  decision it turns on is already recorded.

- **`tree_id` is production-dead as of this leaf, and unlike `tree_grow` and
  `tree_read` the compiler cannot be made to say so.** The claim is held by a
  control run in both directions: gating the module `#[cfg(test)]` leaves
  `cargo build --lib` **clean**, and `cargo check --all-targets` then fails with
  `E0432` in exactly one place. That one place is `tests/session_kind_guidance.rs`,
  an *integration* test, which reaches `grove::tree_id` through the public API
  where the gate does not apply — so the module must stay `pub` until it is
  deleted. **What `sweep-k37` inherits is not a `use` line.** That file's oracle
  for whether a guidance example is a well-formed leaf **is** `tree_id::parse`,
  the lenient grammar, and its prose says that call is *"the same call `pick`,
  `resolve` and every … makes"* — false since `reading-k31`. Re-aiming the oracle
  at `task_name` is the deletion's real cost, and the **falsifiable prediction**
  is that doing so surfaces at least one guidance example the canonical grammar
  refuses and the lenient one accepted. That prediction is this leaf's answer to
  `promotion-k34`'s *grep the strings, not just the tests*.

- **Nothing to transcribe from this node's reachability table, and that is a
  statement rather than an omission.** `tree_migrate` calls no library operation
  at all — it is pure planning, and migration reaches the library only through
  `tree_lifecycle::transition_to_current`, whose rows `lifecycle-k35` already
  transcribed. `refusals-k30`'s scheduled check therefore stands where that leaf
  left it, four corrections in five leaves, and this leaf neither confirms nor
  corrects it.

- **No formalism was reached for and none is owed**, which is `reading-k31`'s and
  `lifecycle-k35`'s position. Four of seven flip leaves have now reached for no
  model at all — one more data point for the entry `sweep-k37` may owe, and the
  reading `lifecycle-k35` invited.

**`sweep-k37` (contract).** The whole suite is **1210 passing**, down from
`migration-k36`'s 1272: sixty-six tests died with the four modules and their two
differentials, and four are this leaf's own. **Three existing tests changed and
one unit helper moved**, none of them for a reason the flip could hide.
`src/tree_id.rs`, `src/tree_read.rs`, `src/tree_grow.rs` and `src/tree_rename.rs`
are deleted; `cargo build --lib` was clean the moment they went, which is the
linkage half of the claim held by the compiler with `lifecycle-k35`'s control
already run.

- **`tree_lifecycle` and `tree_access` needed no surgery, and that is a
  statement about the earlier leaves rather than about this one.** Neither held
  any algebra by the time this leaf opened: what they carry is the lifecycle,
  grove's own guard, the transaction sentinels and the grove-specific refusals,
  exactly as the task's Notes predicted. The only edit either needed was one
  unit-test helper that grew a leaf through the withdrawn appender under grove's
  guard and now calls `task_grow::leaf_add`, which is the composition production
  performs and — since grove cannot nest its lock inside the library's — the only
  one available.
- **`migration-k36`'s falsifiable prediction is false, and the reason it could
  not have fired is the leaf's best finding.** Re-aiming
  `tests/session_kind_guidance.rs`'s oracle from the lenient grammar to
  `TaskName::parse` surfaced **no** guidance example the canonical grammar
  refuses. But the lenient rule was encoded in two *more* places in that file —
  the candidate scanner and the shape explainer both matched a position of
  exactly two digits — so the sweep was structurally incapable of handing the
  parser the one class the tightening is about. Both are widened to a digit run,
  `the_candidate_scan_offers_every_position_width_to_the_parser` is the control,
  and the corpus is clean **measured** rather than clean by construction.
  **What a later leaf should take from it: when a predicate is tightened, hunt
  the test-side filters that were built against the loose one.** A duplicated
  leniency costs nothing while the parser is lenient and is silent in both
  directions the moment it is not — it neither fails nor reports what it skipped.
- **The deletion is checked by enumerate-then-classify, and it found
  thirty-three stale references in twelve files — not one of them a `use`
  line.** `tests/removed_surface.rs` gains a second subject on the method it
  already used for the removed launch environment: every module-shaped
  `tree_*` / `task_*` token under `src/` and `tests/` is enumerated, prose
  included, and classified against a live set read **off disk** against a listed
  withdrawn set. The positive control is the tokeniser finding a withdrawn name
  in a line that carries one; the cross-tree control is the same tokeniser
  finding every withdrawn name in `docs/` and the changelog, which both proves
  the instrument works and stops the table fossilising. A second test holds the
  disk-read live set equal to what `lib.rs` declares in both directions. **The
  compiler discharged the linkage claim and said nothing about the essays**, and
  the essays were the whole finding: a crate-root essay, the version-control seam
  paragraph, three test-file headers, and the module-ownership sentence in
  `docs/ARCHITECTURE.md` all argued about modules that no longer exist.
- **The summary-layer pass is a different reading of the claim, not a longer
  pattern list, and it is what reached the file in no tree at all.**
  `Cargo.toml`'s header said the library is what grove's tree modules *are being
  extracted into*, present tense; no sweep rooted at `src/` and `tests/` can ever
  reach a root manifest. Two of that pass's three findings were in files the
  enumerator does cover, under wordings it does not match — *surviving
  path-walking readers*, *verbs that have not flipped* — which is the honest
  limit of a token sweep: it finds names, and a stale claim need not contain one.
- **`docs/ARCHITECTURE.md` gains *The withdrawn tree algebra*** — what went, the
  three things that deliberately survive and why none of them is algebra, and how
  the deletion is checked — and its module-seams table, species-half paragraph,
  two-locks section and version-control seam are corrected. `CONTEXT-MAP.md`
  stops calling the library *being extracted*.
  `docs/specs/doubt-grove-review-mechanics.md`'s selection seam is re-pointed at
  `task_tree::selected`, whose composition survived the move unchanged in shape.
  Both ADRs naming a withdrawn module are reworked in place to current state
  rather than left predicting a deletion that has happened. **No new ADR and no
  CHANGELOG entry**: nothing was decided that was not already recorded, and this
  leaf changes nothing a reader runs — only `marking-k32` of the seven flip
  leaves earned an entry.
- **The withdrawn names are kept in `docs/` deliberately.** The cross-tree
  control asserts each is still found there, so tidying the deletion out of the
  durable record breaks the sweep rather than passing it. The failure message
  names that third cause explicitly, because it is the one a reader would
  otherwise fix in the wrong direction.
- **No formalism was reached for, which is `reading-k31`'s,
  `lifecycle-k35`'s and `migration-k36`'s position, and `docs/formalism-findings.md`
  entry 025 exists anyway** — because the task's Notes asked for the aggregate and
  four of seven is now a finding rather than an omission. The reading it records:
  a refactor **onto an already-checked library** inherits its models rather than
  needing new ones, and what it needs instead are consumer-side instruments —
  transcription suites, equivalence tests while both implementations are live,
  and sweeps over the surface the compiler cannot read. Three of those four
  model-free leaves reached for exactly one of those. Three routing rows added.
- **The equivalence tests died as scheduled, and that was the plan rather than a
  loss.** `both_readers_resolve_every_reference_form_identically`,
  `both_renderers_spell_every_migrated_leaf_identically` and
  `the_frozen_matchers_admit_exactly_what_the_withdrawn_reader_did` all needed two
  live implementations of one contract, and there is now one. What survives is
  `frozen_legacy_slug_rule_still_agrees_with_the_live_grammar`, which needs no
  second implementation because it holds the frozen rule against the **live**
  grammar. **What a later workstream should take from it:** the window in which a
  flip can be checked by experiment closes when the old side is deleted, so the
  evidence has to be spent while both are compiled — which is what every migrate
  leaf here did.

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
