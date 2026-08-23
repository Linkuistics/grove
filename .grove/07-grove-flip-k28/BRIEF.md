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
