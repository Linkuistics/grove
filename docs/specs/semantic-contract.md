# The semantic contract

The tool-neutral account of what Grove *means* — the vocabulary, the actions,
the total outcome set, and the numbered claims that both the Alloy 6 and the
Quint model families check independently. It is an abstraction boundary, not a
description of the implementation: nothing below names a Rust module, a helper,
or a control-flow shape, and a reader who has never opened `src/` can check a
model against it.

It is also the **single common ancestor of both model families**, which is its
own hazard: two models built from one catalogue agree wherever the catalogue is
wrong, and agreement then reads as proof. [Environment
assumptions](#environment-assumptions) is the control — every assumption the
catalogue makes is enumerated there and relaxed in at least one model, so a
claim that survives only because an assumption was smuggled in has somewhere to
fail.

## Problem

Grove's task tree, its finish/recovery protocol, and the lifecycle that joins
them are specified today across a shipped implementation, the decision records,
an architecture document and a scoping note. Each is accurate; none of them is
*checkable*, and no two of them use quite the same words for a state.

Two model families are about to be built from this material by sessions that
must not read each other's work
([`docs/formalism-findings.md`](../formalism-findings.md), *Experiment 2 —
pre-registration*, **Independence protocol**). Written from prose, they would
diverge on vocabulary before they diverged on anything interesting, and every
later comparison — unique versus overlapping findings, counterexample quality,
synchronization cost — would be measuring that divergence rather than the
formalisms. The comparison needs one contract that both are stated against, and
it has to exist *before* either model does.

The contract has a second, longer life. The crates the implementation phase cuts
— methodology, task-tree semantics, workspace identity, finish/recovery — need
semantic boundaries that are stated in the domain's own terms rather than
inherited from where the code happens to sit today. What both models check is
what those crates must deliver.

## Solution

**Four layers, deliberately separated**, because each answers a different
question and mixing them is what makes the current material hard to check:

| layer | answers | owner |
|---|---|---|
| **Task tree** | what a tree *is*, and what an operation on one may do | `TT-` claims |
| **Finish / recovery** | how a grove ends, and what an interrupted ending leaves | `FN-` claims |
| **System lifecycle** | how sessions, exhaustion, finish, interruption and recovery join up | `SY-` claims |
| **Environment** | what the filesystem, the version control systems and the operator are assumed to do | `EN-` assumptions |

The first three carry **claims** — statements a model checks. The fourth carries
**assumptions** — statements a model *grants*, and which therefore have to be
listed and attacked separately, because an assumption is where a bounded green
run hides its real content.

**Every action is total.** No action in this contract "does nothing when its
guard is false": each returns exactly one member of a closed outcome set, and a
refusal is as much a result as a mutation. That is what lets a model check
*refusal* claims at all, and it is the property Quint's guarded actions and
Alloy's temporal transitions must both preserve.

**Every claim carries a witness obligation.** A claim whose situation is never
reached in a run is not checked, however green the run is — the
pre-registration names this the *vacuous invariant* hazard. So each claim below
names both what must hold and what must be reachable, and the runner fails a
model whose witness never lands.

## Decisions

### One catalogue, not three

The three scopes share identities, states and outcomes: a finish claim is stated
about the same tree the task-tree claims are about, and the lifecycle claims are
about the joint. Splitting the catalogue by scope would put the shared
vocabulary in one of the three, or in all three, and the second is the drift
this document exists to prevent. The scopes are sections, and the claim
identifiers carry the scope so a model file can still say which subset it covers.

### Claim identifiers, and how they reach a model

Each claim has a permanent identifier: `TT-nn`, `FN-nn`, `SY-nn` for claims,
`EN-nn` for assumptions. **The identifier is the cross-reference key** —
experiment entries, model commands, derived tests and findings all cite claims
by it, never by a section title or a line number.

Identifiers are hyphenated in prose and underscored in model identifiers, since
neither modelling language admits a hyphen in a name. The mapping is exactly
`TT-01` ↔ `TT_01`, and the naming convention follows the two runners this
repository already has rather than inventing a third:

| family | property command | witness command |
|---|---|---|
| Alloy 6 | `check TT_01_<mnemonic>` — must find **no** counterexample | `run witness_TT_01_<mnemonic>` — must find an instance |
| Quint | `inv_TT_01_<mnemonic>` — must hold | `wit_TT_01_<mnemonic>` — must be reached |

The `<mnemonic>` is the model author's, and nothing parses it. What is fixed is
the prefix, because that is what the runner matches to decide whether every
claim in this document is covered.

A claim a model family cannot express is **declared, not dropped**: the family's
`README.md` records the identifier, the reason (inexpressible, abstracted away,
outside bounds, or tool-limited) and what would change the answer. The runner
counts a declared gap as covered for the coverage assertion and reports it, so
"not modelled" and "forgotten" never look alike.

### Model paths and the runner

```text
models/run.sh                              the single repository entry point
models/README.md                           the map, and what each scope owns
models/system/lifecycle.als                system lifecycle, Alloy 6
models/system/lifecycle.qnt                system lifecycle, Quint
models/system/README.md
crates/grove-task-tree/models/             task-tree scope, both families
crates/grove-finish/models/                finish/recovery scope, both families
docs/ordinal-fs-tree/models/               the delegated boundary, unchanged
```

Component models sit beside the component they constrain, which is the root
brief's rule; the lifecycle models sit at `models/system/` because they
constrain the joint of several and belong to none. The `ordinal-fs-tree` models
stay where they are — its documents deliberately live under `docs/` while the
crate lives in this repository
([`CONTEXT-MAP.md`](../../CONTEXT-MAP.md)), and this contract does not reopen
that; it consumes the boundary rather than restating it.

The crate directories are created for their `models/` child before the crates
themselves exist. That is deliberate: the model is what the crate is cut
against, and a `models/` directory with no `Cargo.toml` beside it is invisible
to a workspace whose members are listed explicitly.

**`models/run.sh` is the one repository runner**, and it has three obligations
beyond running commands:

1. **Abort on a dead tool.** A tool that failed to launch reports what a tool
   that found nothing reports. Launch-failure output is a runner error, never a
   result. This is not hypothetical — the measurement host's default `java` is
   below Alloy 6's floor
   ([`docs/preservation-baseline.md`](../preservation-baseline.md) §1).
2. **Fail on zero work.** A model file no runner reaches, a command set that is
   empty, a witness that never lands, and a verification step that was skipped
   are each a runner failure that names itself.
3. **Assert claim coverage in both directions.** Every identifier in this
   document must be answered by a command or a declared gap in at least one
   family, and every `TT_`/`FN_`/`SY_`-prefixed command in either family must
   name an identifier this document defines. One direction catches a claim
   nobody modelled; the other catches a command answering to no claim.

It delegates to the two existing `ordinal-fs-tree` runners rather than
absorbing them, which also gives it a positive control: those suites are known
green, so a repository run that reports them clean while finding nothing
anywhere else is reporting a broken instrument.

### Evidence, and where it is recorded

Per claim, per run, the runner records: identifier, family, exact command,
bound or trace limit, solver or backend, outcome, the bound at which the
claim's witness **first** appears, and wall-clock. The witness bound is
separate from the check bound on purpose — the pre-registration's *scope trap*
hazard is a bound too small to reach the defect, and a claim whose witness first
lands at the bound it was checked at has no margin.

Per finding, the six experiment fields (Situation, Formalism, Caught or missed,
Cost, Counterfactual, Verdict) plus the pre-registration's four additions, in
`docs/formalism-findings.md` as entries 026 onward. Every material finding cites
the claim identifiers it affects; a finding that affects no claim is an
observation and stays in the entry's prose, which is the pre-registration's own
rule for what is countable.

Counterexamples are retained in the owning model's `README.md`, compact and
replayable: the command, the bound, the seed where a run is randomized, and the
trace trimmed to the transition that matters.

### What the models must be able to decide

[`TODO.finish_process.md`](../../TODO.finish_process.md) asks four questions that
`formal-synthesis-k16` must answer *keep*, *delete/replace* or *defer* with
evidence — and "the model is smaller" is explicitly not evidence. The catalogue
fixes which claims decide each, so the answer is read off the models rather than
argued:

| question | decided by | what would classify it *delete/replace* |
|---|---|---|
| **Q1 — does the quarantine need to exist?** | `FN-19`, `FN-20`, `FN-21`, `FN-24` | a disposal-in-place protocol under which every interruption still lands in exactly one stable state, with `FN-24`'s witness reached at the same bound |
| **Q2 — can the three dispositions become two?** | `FN-15`, `FN-25`, and the lane table | `Indeterminate` unreachable on a lane, shown by a witness that never lands under a bound where the neighbouring witnesses do |
| **Q3 — is the marker-replacement sub-transaction reachable?** | `FN-21`, `FN-22` | no reachable state in which disposal must *replace* rather than create or remove its marker |
| **Q4 — what does finish still owe the user?** | `FN-27`, `FN-28`, `SY-05` | a claim whose only content is protecting Grove's own intermediate artifacts, which drops with the artifact |

A question whose deciding witness is never reached is **defer**, not delete: an
unreached witness is an absence of evidence, and the pre-registration's
*vacuous invariant* hazard is exactly the habit of reading one as the other.

## Vocabulary

The terms below are the models' state. Where a term is already in
[`CONTEXT.md`](../../CONTEXT.md) this section uses it and does not redefine it;
where the catalogue needed a word the glossary did not have, the glossary gained
it rather than this document owning a second definition.

### Identities

**Working-tree root** — the directory Grove drives. It exists before the task
root and outlives its deletion, which is what makes it the only thing a lock can
be taken on across the whole lifecycle. Pinned by identity, not by path, for the
duration of any operation.

**Task root** — the tree's own directory beneath the working-tree root. Present
or absent; its absence is a first-class, load-bearing state (`SY-05`).

**Entry** — anything directly or transitively beneath the task root. Exactly one
of: a **charter**, the **format witness**, a **reserved witness**, a **task
entry** (a leaf or a node directory), or **foreign**.

**Position**, **permanent key**, **slug**, **session kind**, **outcome infix**,
**work-item handle** — as the glossary defines them. A name is the tuple of all
of these and nothing else; there is no metadata anywhere but the name, except
the freeform prose inside a leaf, which nothing parses.

**Finish handle** — the work-item handle of the finish leaf. Stable across the
whole attempt and across restarts.

**Finish-attempt identity** — the opaque value drawn once per launch that binds
one attempt to one still-live session. Two attempts on one handle are
distinguishable by it and by nothing else.

**Repository anchor** — the recorded starting topology, whose shape is
lane-specific and whose *role* is not: it is what a rollback must find unchanged
before it is licensed.

**Deletion fingerprint** — the expected, non-empty set of tracked paths the
deletion commit removes. Empty is refused before any mutation, because an
untracked tree has no focused commit that could record its finish.

**Entry digest** — a canonical, symlink-non-following, recursive digest of an
entry. The models treat it as an opaque equality; its construction is a
[deliberate omission](#deliberate-omissions).

**Evacuation manifest** — the record inside the witness: finish handle, attempt
identity, repository anchor, deletion fingerprint, and every evacuated entry's
type and digest, marked ready last.

**Correlation ticket** — the deletion commit's own message, naming the finish
handle and the attempt identity. This is the attempt's durable record, and it
sits in version-control history rather than in any artifact the transaction can
destroy — which is what lets a retry that has lost every local trace still tell
its own completed attempt from someone else's.

### States

**A leaf** is `Live`, `Done` or `Abandoned`. `Done` and `Abandoned` are
**terminal**. A **node** has no state: its done-ness is derived from the absence
of a live leaf anywhere beneath it, and it is never marked.

**A task root** is classified in a fixed order, and the order is itself a claim
(`TT-18`):

| state | meaning |
|---|---|
| `Absent` | no task root |
| `Reserved(Preparing)` | a finish witness built but not published |
| `Reserved(Published)` | a finish witness published, holding the evacuated entries |
| `Reserved(Migrating)` | a session-kind migration interrupted mid-flight |
| `Legacy` | present, no format witness |
| `Foreign(found)` | the format witness holds something else |
| `Malformed(entry)` | current format, but a task-shaped entry breaks the grammar |
| `Current(Live)` | at least one live non-finish leaf |
| `Current(FinishOnly)` | the only live leaf is the finish sentinel |
| `Current(Spent)` | no live leaf at all |

A `Reserved` state may additionally carry a **blocked diagnosis** — see
[Outcomes](#outcomes) — which is what an interrupted transaction leaves behind
once recovery has run and could not settle it.

`Reserved(Migrating)` is in the table because it is a state a tree can be in
today, and the models must be able to say what an ordinary reader does with one.
The approved breaking change removes migration itself; what survives it is the
refusal, and `TT-18`/`TT-19` are stated over the reserved *class* rather than
over its members so that removing one member changes no claim.

**Stable and transient.** A **stable** state is one an ordinary invocation may
observe and act on. A **transient** state exists only inside one operation, while
its exclusive guard is held, or between two filesystem steps of one transaction:
building a witness, evacuating, restoring, renaming to quarantine, disposing.

The load-bearing property, and the reason the distinction is in the vocabulary
rather than in a note, is that **no transient state may be observable as a
different stable state**. An evacuated tree is `Reserved(Published)` and never
`Malformed` or `Current(Spent)`; a task root whose deletion is not yet proven is
never `Absent`. `SY-05` is where that cashes out.

### Actions

Each action is **total**: it returns exactly one outcome, and a guard that fails
produces a named refusal rather than an absent transition.

| group | actions | guard |
|---|---|---|
| **Observation** | `select`, `resolve`, `brief-chain`, `kind` | shared |
| **Tree mutation** | `initialise-root`, `add-leaf`, `add-pair`, `insert-leaf`, `decompose-leaf`, `retire-leaf`, `prune` | exclusive |
| **Finish** | `allocate-finish-leaf`, `finish-commit`, `recover`, `dispose-quarantine`, `reap-quarantine` | exclusive, plus the repository |
| **Lifecycle** | `acquire-lease`, `layout-preflight`, `open-epoch`, `launch`, `reap`, `close-epoch`, `release-lease` | lease, then epoch |
| **Environment** | `crash`, `hand-edit`, `foreign-write`, `topology-change`, `confirm` | none — these are the world's |

The environment actions are what make the models behavioural rather than
structural. `crash` may occur between any two steps of any action; `hand-edit`
is how an arbitrary well-formed tree is *reached* rather than posited
(`EN-11`); `confirm` is the operator's, and Grove cannot verify it (`EN-15`).

<a id="outcomes"></a>
### Outcomes

The closed set. Every action returns exactly one.

| outcome | meaning |
|---|---|
| `Applied` | the mutation completed and is visible |
| `Reported(v)` | an observation returning a value |
| `Empty` | an observation matched nothing — **a success** |
| `Ambiguous(cs)` | an observation matched several — **a success** |
| `Refused(r)` | nothing happened; the tree is byte-identical |
| `Blocked(b)` | a transaction stopped part-way and left a stable, recoverable state |

`Empty` and `Ambiguous` are outcomes rather than refusals because that is the
shipped contract and callers branch on it: selection on a spent grove and a
resolution that matched nothing are both successes that mutate nothing and
report nothing to standard output (`TT-15`).

**Refusal reasons**, closed:

`RootAbsent` · `FormatLegacy` · `FormatForeign` · `WitnessPending(class)` ·
`Malformed(entry)` · `NotLive` · `AlreadyTerminal` · `ReservedKind` ·
`NotAnEntry` · `DestinationOccupied` · `LayoutUnsupported` · `LeaseHeld` ·
`EpochStale` · `NoTrackedDeletion` · `RootIdentityChanged` ·
`UnsupportedEntryType`

**Blocked diagnoses**, closed and exhaustive over blocks:

- **`RecoveryPending`** — a correlated Grove-owned attempt is incomplete. The
  artifact holding the transaction is provably Grove's, named by *this* finish
  handle and *this* attempt identity, and the outcome cannot yet be proven
  either way. The operator has two restorable exits and the diagnostic names
  both.
- **`OwnershipConflict`** — state is unrelated, ambiguous, or cannot be proved
  safe to mutate. An artifact sits at a name Grove reserves but Grove cannot
  classify it as its own; or the observed topology matches neither the recorded
  anchor nor the expected result; or an entry is of a type Grove refuses to
  touch.

**These two are a partition the catalogue introduces, and the shipped
implementation does not yet draw it.** Today's classification yields three
commit *dispositions* — `Committed`, `NotCommitted`, `Indeterminate` — and
gathers under one blocked state both of the cases above. The root brief requires
them distinguished; `FN-25` states the partition as a claim so the models decide
whether it is total, disjoint and reachable on every lane, and
`formal-synthesis-k16` decides on that evidence whether the shipped diagnostic
adopts two names. Nothing here changes product behaviour.

**Dispositions are not outcomes.** A disposition is the classification of the
*commit*, and it is an input to the outcome rather than the outcome itself:
`Committed` settles forward and yields `Applied`; `NotCommitted` rolls back and
yields `Refused`; `Indeterminate` yields exactly one `Blocked`.

<a id="environment-assumptions"></a>
## Environment assumptions

Everything the models **grant**. This list is the control for the
pre-registration's *agreement mistaken for proof* hazard: both families descend
from this one document, so an error smuggled in as an assumption produces two
models that agree and a comparison that means nothing. Each assumption is
therefore **relaxed in at least one model**, in a named instance or scope of its
own, and the relaxation must break a claim that names it. An assumption whose
relaxation breaks nothing was carrying no weight and is recorded as such.

| id | assumption | relaxed by |
|---|---|---|
| `EN-01` | A same-directory rename is atomic with respect to namespace visibility. | Quint — `relax_EN_01`, a rename observable half-applied |
| `EN-02` | A rename cannot cross a filesystem boundary. | Alloy — a two-device scope |
| `EN-03` | There is no atomic recursive directory deletion. | Quint — `relax_EN_03`, disposal as one step (this is Q1's counterfactual) |
| `EN-04` | There is no atomic replacement of a file by a differently named directory. | Alloy — the promotion structure, inherited from the delegated boundary |
| `EN-05` | No filesystem transaction can include a version-control commit. | Quint — `relax_EN_05`, commit and evacuation as one step |
| `EN-06` | Locks are advisory: only cooperating processes are serialized, and a direct edit is outside the guarantee. | Quint — `relax_EN_06`, a non-cooperating writer |
| `EN-07` | Two open descriptions of one directory do not share a lock. | Alloy — a shared-lock scope, which should reintroduce the deadlock `bulk-marks-are-not-atomic` records |
| `EN-08` | Interruption may occur between any two steps. Power loss, kernel failure and storage-cache loss are outside the contract. | Both — `crash` is a first-class action, so this assumption is *exercised* rather than relaxed; its complement is the omission list |
| `EN-09` | A command's exit status is not a receipt: a result may be lost or arrive late. | Alloy — a trace in which the result arrives after the classification |
| `EN-10` | The names are the counter: key allocation reads the tree, and entries are never removed. | Quint — `relax_EN_10`, an entry removed, which should re-issue a live key |
| `EN-11` | Any well-formed tree is reachable by hand edit. | Both — `hand-edit` is a first-class action; the relaxation is the *absence* of it, which is the pristine instance |
| `EN-12` | A name renders as exactly one path component. | Alloy — a rendering that escapes its level |
| `EN-13` | Foreign entries may appear at any name and are not Grove's to delete. | Quint — `relax_EN_13`, a sweep of a reserved namespace, which should delete bytes a refusal exists to preserve |
| `EN-14` | The working-tree root exists before the task root and outlives its deletion. | Alloy — a scope in which the root itself is removed |
| `EN-15` | Confirmation is an operator input Grove cannot verify. | Quint — `relax_EN_15`, a machine-attested confirmation, which should make no claim stronger |
| `EN-16` | The three lanes differ in mechanism and agree on abstract outcome. | Both — the lane is a model parameter, so every finish claim is checked three times |

`EN-08`, `EN-11` and `EN-16` are marked *exercised rather than relaxed* because
their negation is not a smaller world but a different one: a model with no
`crash` action, no `hand-edit` and one lane is the model this experiment exists
to avoid. Where an assumption is exercised, the control is that the claims which
depend on it must **fail** when the action is removed, and that is checked by
the same mutation discipline as everything else.

## The three lanes

One abstract outcome, three mechanisms. What every lane must produce is a single
revision whose only change is the deletion of the task tree, described by the
correlation ticket. What differs is what counts as the anchor, what licenses a
rollback, and what has to be protected on the way.

| | plain Git | native jj | colocated jj |
|---|---|---|---|
| **selects on** | a `.git` marker | a `.jj` marker | `.jj` wins over `.git` |
| **anchor** | the recorded head revision | the working-copy change identity, its parents, and the exact preflight commit | both, plus the user's index image |
| **scoping** | commit only the expected deletions at their original paths, excluding the witness | a fileset over the task root with the same exclusion | as native |
| **rollback licence** | the head still equals the anchor **and** the attempt-bound result is absent | the current working copy still carries the recorded change identity at the same parents **and** the result is absent | as native, plus the pre-snapshot index image restores |
| **exactness after restore** | the tree matches the manifest | the tree matches **and** the exact preflight commit is reproduced before the witness is removed | as native |
| **the partial-commit hazard** | — | a partial commit keeps the deletion in the change and moves the unselected witness into a **successor**, so success is the exact ticket-named *parent* of that successor, not merely the ticket appearing somewhere in history | as native |
| **hooks** | internal commits run with an empty hooks path | no Git hooks run | no Git hooks run |
| **index** | an index image is taken and restored on an uncommitted result | none | the user's index is backed up **before** the preflight snapshot can export into it, and the deletion-free success image activates only after the exact result is proven |

The lane is a **model parameter**, not three models: every `FN-` claim is
checked under all three, and a claim that holds on only some of them is a
finding rather than a lane-specific claim. That is what the root brief means by
symmetric lanes, and it is why `EN-16` is exercised rather than relaxed.

## Claims — task tree (`TT`)

Each row states what must hold and what must be reachable. The witness column is
an obligation, not a suggestion: a claim whose witness never lands is not
checked, whatever the check reported.

### Names and identity

**`TT-01` — a name has exactly one spelling.** Distinct filenames SHALL never
denote the same entry: parsing a name and rendering the result SHALL reproduce
the input exactly, and a name spelled any other way SHALL be refused, naming the
canonical spelling.
*Witness*: a tree in which a non-canonical spelling is refused, and a tree in
which two spellings of one entry would otherwise both parse.
*Cites*: [`task-names-are-canonical`](../adr/task-names-are-canonical.md).

**`TT-02` — a name declares its species and must be it.** A task-shaped name
SHALL denote an on-disk entry of the species the name declares; a leaf name at a
directory, or a node name at a file, SHALL be malformed.
*Witness*: each mismatch, separately.

**`TT-03` — malformed halts, and never skips.** A task-shaped entry that does
not parse completely, or whose session kind is absent or unknown, SHALL stop
every read and mutation of the whole tree, naming the entry and the admissible
kinds.
*Witness*: a malformed node directory whose subtree holds live work — the case
where skipping would report a finished grove.

**`TT-04` — foreign entries are ignored and preserved.** An entry outside the
task grammar SHALL neither be read as work nor mutated by any action.
*Witness*: a foreign entry surviving a mutation that renamed its siblings.

**`TT-05` — keys are unique, permanent and never reissued.** Allocation SHALL be
one past the maximum key over every entry in the tree, terminal entries
included, and no key SHALL ever be issued twice.
*Witness*: an allocation whose maximum comes from a terminal entry.
*Cites*: [`entries-are-never-removed`](../adr/entries-are-never-removed.md).

**`TT-06` — positions are per-directory and gapless.**
*Witness*: an insert that closes no gap and one that shifts every later sibling.

**`TT-07` — a shift preserves everything but position.** Insertion and
renumbering SHALL change positions only — never a key, slug, kind or outcome
infix, and never any file's bytes.
*Witness*: a shift across a directory containing every species.

**`TT-08` — decomposition preserves the key.** A leaf promoted to a node SHALL
keep its own key, and the promoted subtree's names and keys SHALL be untouched.
*Witness*: a promotion of a leaf whose key is the tree's maximum.

**`TT-09` — every mutation is one algebraic operation plus a domain
precondition.** No action SHALL move an entry outside an append, insert,
promotion or rewrite of the entry-name algebra; ordering, shifting and
allocation SHALL be properties of that algebra alone.
*Witness*: each of the four, reached.
*Cites*: [`entry-name-is-the-only-seam`](../adr/entry-name-is-the-only-seam.md).

**`TT-10` — no algebraic refusal reaches an operator from an ordinary
argument.** Every refusal an action can produce SHALL be one this catalogue
names, because the domain's own preconditions run in front of the algebra.
*Witness*: an argument that would reach an algebraic refusal, shown pre-empted.

### Selection

**`TT-11` — selection is a stateless pre-order walk.** It SHALL return the first
live leaf in depth-first pre-order over positions, and SHALL depend on no state
outside the tree.
*Witness*: a selection that descends a node before visiting a later sibling.

**`TT-12` — terminal entries are skipped, never removed.**
*Witness*: a walk crossing a wholly terminal node.

**`TT-13` — finish is reserved, not blocking.** The walk SHALL skip a live
finish leaf while any non-finish leaf is live, and SHALL return it when it is the
only live leaf. More than one live finish leaf SHALL be malformed.
*Witness*: a finish leaf at an earlier position than live ordinary work — the
case where the skip rule is the only thing preventing teardown.

**`TT-14` — selection is not a scheduler.** No dependency, priority or grouping
SHALL affect the order; the only mechanisms are position and terminality.
*Witness*: two orderings of the same work selecting differently.

**`TT-15` — an empty or ambiguous observation is a success.** Selection on a
spent tree, a resolution matching nothing, and a resolution matching several
SHALL each mutate nothing, refuse nothing, and be distinguishable from one
another by their reported value alone.
*Witness*: all three.

**`TT-16` — a resolved terminal entry is never mistaken for live.** A resolution
that matches a terminal entry SHALL report both the entry and its terminality.
*Witness*: one of each terminal state.

### Root identity and guarding

**`TT-17` — format is decided by the witness's content.** The classification
SHALL depend only on the format witness, never on any task entry's text.
*Witness*: a legacy tree whose slug text would otherwise read as a current kind.

**`TT-18` — classification order is fixed.** Reserved-witness classification
SHALL precede format classification, which SHALL precede any walk-derived
classification.
*Witness*: a tree carrying both a reserved witness and no format witness,
reported as the former.

**`TT-19` — a reserved witness refuses everything else.** While any reserved
witness exists, every observation and mutation except the matching recovery SHALL
refuse, naming the witness and the operation that can recover it.
*Witness*: a `Reserved(Preparing)` tree, whose ordinary entries are all still in
place and which therefore looks perfectly walkable.

**`TT-20` — the format witness lands last.** Root initialisation SHALL make the
format witness visible only after every other scaffolded entry, by an atomic
same-directory rename, so no reader observes a torn or premature marker.
*Witness*: an interruption before the witness lands, classified as a partial
scaffold rather than as a current tree.

**`TT-21` — one snapshot per operation.** Every classification an operation
makes SHALL be computed from a single listing taken under that operation's
guard.
*Witness*: a concurrent writer between two classifications, shown excluded.

**`TT-22` — guards are shared for observation and exclusive for mutation**, and
are taken on the working-tree root.
*Witness*: two concurrent observations admitted; an observation and a mutation
serialized.
*Cites*: [`task-tree-transactions-fail-closed`](../adr/task-tree-transactions-fail-closed.md).

**`TT-23` — a bulk mark validates before it moves, and converges.** A bulk mark
SHALL validate its whole plan against one snapshot before its first rename, and
re-running it after a partial application SHALL reach the same result.
*Witness*: a bulk mark interrupted mid-run, repaired by re-running it.
*Cites*: [`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md).

**`TT-24` — fail-closed ownership.** No action SHALL reset, merge, delete or
rewrite an entry it cannot prove is its own; where ownership cannot be proved the
outcome SHALL be a refusal or a block, never a mutation.
*Witness*: a foreign entry at a reserved name, refused rather than removed.

**`TT-25` — a node is never marked.** Done-ness SHALL be derived from the absence
of a live leaf beneath it, and no action SHALL write a node's state.
*Witness*: a node whose subtree is wholly terminal, and one that is not.

## Claims — finish and recovery (`FN`)

Every claim below is checked under all three lanes.

### Entry and intent

**`FN-01` — confirmation enables, and is never attested.** No step of the
transaction SHALL run without an operator confirmation, and the transaction SHALL
make no claim to have verified that one occurred. The deterministic guards it
*can* make — a live finish leaf, no live ordinary work — are separate and are
not a substitute.
*Witness*: a transaction refused for want of the deterministic guard, distinct
from one never entered for want of confirmation.

**`FN-02` — intent persists as the finish leaf.** Declining, or exiting without
completing, SHALL leave that leaf live and selectable, and SHALL write nothing
else.
*Witness*: a decline followed by a later successful attempt on the same handle.

**`FN-03` — the correlation ticket is the durable record.** The proof that a
given attempt completed SHALL be the deletion commit's own message, naming the
finish handle and the attempt identity, and SHALL survive the destruction of
every artifact the transaction owns.
*Witness*: a retry with no local trace of the attempt, settling forward on the
ticket alone.

**`FN-04` — an attempt binds to a live session.** A commit SHALL be accepted as
this attempt's result only when its attempt identity matches the currently live
session's; a ticket from an earlier attempt SHALL NOT settle a later one.
*Witness*: two attempts on one handle, the earlier ticket rejected by the later.

### Preflight

**`FN-05` — preflight mutates nothing.** Every precondition SHALL be established
before any tree mutation, and a failure SHALL leave the tree and the repository
byte-identical.
*Witness*: each precondition failing, with the tree unchanged.

**`FN-06` — the task root's identity is pinned and rechecked.** The task root
SHALL be opened as a no-follow directory whose identity is verified against the
entry in the guarded working-tree root, and rechecked at every later step, so a
mid-transaction swap is a refusal rather than a mutation applied elsewhere.
*Witness*: a swap between two steps, refused.

**`FN-07` — an untracked tree is refused before evacuation.** A deletion
fingerprint that is empty SHALL refuse, because no focused commit could record
that finish.
*Witness*: a wholly untracked tree.

**`FN-08` — the quarantine target is proved reachable before mutation.** The
same-device requirement SHALL be checked against the transaction's own rename
operands, and SHALL never be satisfied by an earlier lifecycle check.
*Witness*: a layout that passes at lease acquisition and fails here.

### The witness

**`FN-09` — build, then publish, in one atomic step.** The witness SHALL be
built under a preparing name and published by exactly one atomic rename. No
preparing witness SHALL ever hold an evacuated entry.
*Witness*: an interruption inside the build, and one immediately after
publication.

**`FN-10` — an unpublished witness is discardable.** Interruption before
publication SHALL be recoverable by discarding the witness, never by
interpreting its contents, and SHALL fail closed on any content it cannot
classify as its own.
*Witness*: a discard, and a refusal to discard unclassifiable content.

**`FN-11` — evacuation precedes deletion.** Every ordinary root entry SHALL be
inside the published witness, beneath a manifest that has been written and
verified, before any commit is attempted.
*Witness*: the interval between publication and commit, with the task root
present, unwalkable and holding every entry.

**`FN-12` — the manifest is complete and marked ready last.** It SHALL record the
finish handle, the attempt identity, the repository anchor, the deletion
fingerprint, and every evacuated entry's type and digest; an entry type it
cannot digest SHALL be refused before any mutation.
*Witness*: a refused entry type; a manifest interrupted before its ready mark.

**`FN-13` — the witness is never committed.** Every candidate committed tree
SHALL exclude the witness.
*Witness*: a commit attempted while the witness is tracked, refused.

### Commit and disposition

**`FN-14` — the commit is scoped.** It SHALL record exactly the expected
deletions at their original paths and no unrelated change; unrelated working-copy
work SHALL survive.
*Witness*: unrelated modified work present across a successful finish.

**`FN-15` — disposition is classified from evidence, not from exit status.** The
classification SHALL be derived from the recorded anchor, the expected
fingerprint and the exact immediate result.
*Witness*: a lost or late result reported as failure while the exact commit
exists — classified `Committed`.

**`FN-16` — rollback is licensed only by proof.** Restoration SHALL require the
recorded anchor to still hold **and** the attempt-bound result to be absent.
*Witness*: each half failing separately.

**`FN-17` — rollback is exact.** After restoration the tree SHALL match the
manifest, and on a working-copy-as-commit lane the exact recorded preflight
commit SHALL be reproduced before the witness is removed.
*Witness*: a restoration that reproduces the commit, and one that cannot, which
must block rather than proceed.

**`FN-18` — forward recovery never restores.** Once the exact commit is proven,
the tree SHALL never be reconstructed.
*Witness*: a proven commit reached after an interruption mid-evacuation.

### Handoff and cleanup

**`FN-19` — the root moves in one atomic rename.** A proven commit SHALL be
settled by renaming the whole task root — witness and evacuated tree intact —
into the quarantine in one step. No partial or empty task root SHALL ever be
observable.
*Witness*: an interruption immediately after the rename, leaving a complete
quarantine and an absent task root.

**`FN-20` — quarantine is garbage, never a receipt.** No classification SHALL
read the quarantine, or any control-directory artifact, as evidence that a
finish happened.
*Witness*: a quarantine present while the tree is classified fresh.

**`FN-21` — disposal is resumable and bounded to Grove's own.** Disposal SHALL be
re-enterable from any interruption, and a reaper SHALL touch only entries
carrying Grove's own cleanup manifest, and only when no matching in-tree witness
owns them.
*Witness*: a resumed disposal; a reaper declining an entry whose witness still
owns it; a reaper declining a foreign entry at a reserved name.
*Decides*: `TODO.finish_process.md` Q1 and Q3 — whether a *replace* step, as
against create or remove, is reachable at all.

**`FN-22` — the disposition is revalidated across every handoff.** It SHALL be
rechecked immediately before and after each filesystem handoff. A change after
restoration SHALL leave the witness blocking the restored tree; a change after
the quarantine rename SHALL return the quarantine atomically, and SHALL report
both the change and the quarantine if that return cannot complete.
*Witness*: each of the two, and the failed return.

### Recovery, refusal and the exits

**`FN-23` — recovery is idempotent.** Re-running recovery for the same handle
and attempt SHALL reach the same terminal state, and SHALL make no further change
once it has.
*Witness*: three consecutive recoveries, the second and third changing nothing.

**`FN-24` — every interruption lands in exactly one stable state.** From a crash
between any two steps of the transaction, the next invocation SHALL classify the
result into exactly one stable state, and never into a state that is
indistinguishable from a different one.
*Witness*: the full interruption sequence, one crash point per step.
*Decides*: `TODO.finish_process.md` Q1 — a cheaper protocol is admissible only if
this claim still holds under it, at the same bound.

**`FN-25` — a block is exactly one of the two diagnoses.** Every blocked state
SHALL be exactly one of `RecoveryPending` and `OwnershipConflict`; the two SHALL
be disjoint and jointly exhaustive over blocks, and each SHALL be reachable on
each lane.
*Witness*: each diagnosis, on each lane.
*Decides*: `TODO.finish_process.md` Q2.

**`FN-26` — history is never rewritten to clear a block.** A block SHALL stay
blocked and operator-restorable, naming the artifact holding the transaction,
the recorded and observed topology, and the two restorable exits.
*Witness*: a block whose diagnostic carries all four, and no trace in which
recorded history changes.

**`FN-27` — nothing unrelated is mutated, on any outcome.** Nothing outside the
task root, the reserved witness, the quarantine and the scoped commit SHALL
change — on success, on refusal, and on a block alike.
*Witness*: unrelated work intact across each of the three outcomes.

**`FN-28` — one successful exit.** A finish succeeds exactly when the exact
attempt-bound commit is proven and the task root is absent. Branch, bookmark and
worktree topology SHALL be unchanged, and no integration or removal SHALL be
performed. Best-effort cleanup that must be retried SHALL NOT make a proven
finish unsuccessful.
*Witness*: a success whose cleanup is still outstanding.
*See*: [Out of scope](#out-of-scope) — the second exit the root brief describes
is deliberately not modelled.

**`FN-29` — a refusal is a complete outcome.** `NotCommitted` SHALL leave the
grove exactly as it was, with the finish leaf live and selectable, and SHALL be
distinguishable by the operator from a block.
*Witness*: a refused attempt followed by a successful one.

**`FN-30` — internal commits run without operator hooks.** No user-supplied hook
SHALL run during an internal commit, because such a hook may mutate unrelated
working-tree bytes that no index image restores.
*Witness*: a hook that would have run, shown suppressed.

## Claims — system lifecycle (`SY`)

These are the joint. They are stated over sessions, exhaustion, finish,
interruption and recovery together, and they are what `models/system/` owns.

**`SY-01` — one live driver per working tree.** A second driver SHALL be refused
immediately rather than queued, and ownership SHALL be released by process death
as ordinarily as by return.
*Witness*: a refused second driver; a crashed driver whose successor proceeds.
*Cites*: [`one-live-driver-per-working-tree`](../adr/one-live-driver-per-working-tree.md).

**`SY-02` — the layout is proved before any tree exists.** An unsupported
workspace SHALL be refused at lease acquisition, before configuration validation
and before any observation, creation or mutation of the tree.
*Witness*: a refusal that leaves an empty working tree untouched.
*Cites*: [`supported-workspace-layouts`](../adr/supported-workspace-layouts.md).

**`SY-03` — a preflight is never a licence.** No later gate SHALL consult an
earlier layout check; each SHALL revalidate against its own operands.
*Witness*: a layout that changes between the two gates.

**`SY-04` — at most one lifecycle transition per iteration**, and full
configuration validation SHALL precede every one, so an invalid configuration
leaves the working tree byte-identical.
*Witness*: each transition, taken alone; an invalid configuration mutating
nothing.

**`SY-05` — task-root absence is the complete fresh-tree discriminator.** A
missing task root SHALL mean *start a new grove*, and SHALL never be read as
evidence about an earlier one. This inference is sound only because `FN-11`
and `FN-19` never expose an absent task root before the deletion is proven, and
the two claims SHALL be checked together.
*Witness*: a completed teardown whose driver never observed the signal, followed
by a fresh scaffold; and the absence of any trace in which an unproven deletion
exposes an absent root.

**`SY-06` — a fresh root carries a first live leaf.** Scaffolding SHALL produce
work, not only a charter, so a fresh grove is never indistinguishable from a
finished one; a partial scaffold SHALL be recognised as a subset and completed
before any format classification.
*Witness*: an interrupted scaffold, completed.

**`SY-07` — exhaustion yields exactly one finish leaf.** When no live leaf
remains the driver SHALL append or reuse exactly one driver-owned finish leaf,
and no session SHALL create one.
*Witness*: a reuse; a refused creation.

**`SY-08` — selection is authoritative once per iteration.** The driver SHALL
select exactly once per iteration and SHALL not recompute before launching, so a
leaf added during the launch window becomes the next iteration's work rather than
preempting the running one.
*Witness*: a leaf inserted during the launch window.

**`SY-09` — a session ends in exactly one of three ways.** Relaunch, done, or no
signal. No signal SHALL stop the loop, and SHALL never be inferred as done — not
even when that session committed a teardown.
*Witness*: all three, including the last with a proven teardown.

**`SY-10` — a stale session cannot act.** An ambient operation SHALL match the
live launch generation before it may touch the tree, and a contended generation
SHALL time out into a visible stop rather than a silent park.
*Witness*: a stale session refused; a timeout reported.

**`SY-11` — the guard order admits no cycle.** Lease, then launch generation,
then tree; and no path SHALL wait for a generation while holding a tree guard.
*Witness*: the full order, and the absence of any cycle within the bound.

**`SY-12` — restart is ordinary continuation.** From a crash at any lifecycle
point, the next invocation SHALL reach a stable state and either make progress or
refuse; it SHALL never silently repeat a completed effect.
*Witness*: one crash point per lifecycle step.

**`SY-13` — the loop makes progress.** From any stable state, a bounded sequence
of admitted actions SHALL reach either a live leaf to run or a terminal
disposition; no stable state SHALL be a sink from which neither is reachable.
*Witness*: the longest such sequence within the bound, and its length.

**`SY-14` — a blocked tree stays blocked until an operator acts.** No admitted
action SHALL clear a block, and every action on a blocked tree SHALL refuse
naming it.
*Witness*: an exhaustive sweep of the action set against a blocked tree.

<a id="deliberate-omissions"></a>
## Deliberate omissions

What the models abstract away, fixed here so that both families omit the *same*
things and a comparison is not measuring two different subjects. The
pre-registration names *the idealisation* — a detail omitted from the model is
where the bug lives — as a live hazard, so each omission below carries the
obligation that it be checked against the shipped behaviour **by hand** and the
result recorded in the experiment log.

| omitted | modelled instead as | why the omission is defensible |
|---|---|---|
| syscall choreography — descriptor-relative opens, exchanges, no-follow unlinks | one atomic step each, per `EN-01` | the ordering they implement is what the claims are about; the calls are how one platform provides it |
| the digest construction | an opaque equality on entries | `FN-12` needs digests to distinguish entries, not to be collision-resistant |
| byte-level file contents | an entry's identity and type only | no claim in this catalogue reads a leaf's prose |
| clocks, timeouts and retry counts | non-determinism | a bounded handoff wait is a liveness property of the *implementation*, not of the protocol |
| more than two cooperating processes | two | every guard claim is about mutual exclusion, which two processes exhibit |
| the marker-replacement protocol's own byte layout | a resumable disposal step **with an explicit replace transition** | Q3 asks whether replacement is reachable at all, so the transition must exist for the model to answer; only its encoding is omitted |
| power loss, kernel failure, storage-cache loss | not represented | `EN-08` — outside the contract, and representing them would make every claim false without making any of them wrong |
| the methodology corpus and session conduct | a session is a step that returns one of three endings | conduct is checked by delivery assertions, not by these models |

The sixth row is the one to watch. Abstracting the replace step *away* would
answer Q3 by construction, which is the shape of a false-confidence incident
rather than a finding.

## Test seams

**One seam: `models/run.sh`.** Every claim in this document is checked through
it, and it is the only new instrument this phase builds. Three existing rules
point the same way — prefer an existing seam, propose a new one at the highest
point, drive the count toward one — and the repository already runs models this
way, so the runner adopts the two `ordinal-fs-tree` runners' pass/fail
conventions rather than inventing a third and delegates to them rather than
absorbing them.

**The first model family to run builds it, and the second extends it.** A runner
is not a model, so building one jointly costs the independence protocol nothing —
what neither family may read before both are green is the other's *models*. The
runner is where the two families' results meet, which is exactly why it must
exist before either is finished rather than being assembled afterwards from two
scripts that already disagree.

Its obligations are stated under [Model paths and the
runner](#model-paths-and-the-runner): abort on a dead tool, fail on zero work,
and assert claim coverage in both directions. The third is what makes the
catalogue rather than the models the source of truth, and it is checkable rather
than asserted — a claim added here with no command anywhere fails the run, and so
does a command answering to no claim.

**Findings reach the product through the existing black-box binaries.** A
material finding yields a Rust test that fails against the pre-fix
implementation, and it lands in the suite that already covers that surface
rather than in a new one. This phase changes no product behaviour, so nothing
here proposes a new Rust seam; the crate-facing seams belong to
`formal-synthesis-k16`, once the models have shown where the boundaries actually
fall.

**Mutation is the control on both.** Per reported obligation: break the
mechanism, watch that specific claim fail, restore. A green suite that stays
green under a deliberate break is measuring nothing, and this is the only check
that distinguishes the two.

<a id="out-of-scope"></a>
## Out of scope

**A merge-and-remove finish exit.** The root brief asks finish to "make both
successful exits explicit: preserve the branch/bookmark, or merge and then
remove only the proved-owned branch/worktree". No such exit is modelled, and
`FN-28` states the single successful exit instead. Grove reads no branch or
bookmark, creates no working tree and performs no integration; the user owns
topology, and describing finish as merging anything version-control-topological
describes the cycle Grove replaced. `TODO.finish_process.md`, which the brief
names as the authoritative pointer for the required finish concerns, asks the
opposite question — how much of the machinery protects the repository as against
Grove's own intermediate artifacts. This was put to the human, who confirmed
today's contract. Reopening it is a brief change plus a rework of
[`task-tree-transactions-fail-closed`](../adr/task-tree-transactions-fail-closed.md)
and the glossary's `Complete finish cycle` entry, and it would add a
branch/bookmark surface to workspace identity; it is not an inline widening of a
model.

**Legacy migration as a capability.** The approved breaking change removes it.
`Reserved(Migrating)` remains in the state table because a tree can be in that
state today and the models must say what an ordinary reader does with one, but
`TT-18` and `TT-19` are stated over the reserved *class*, so removing a member
changes no claim. Nothing here models a migration's own transitions.

**Session conduct.** Whether a session obeys the methodology it is handed is not
a claim these models can make; the corpus has its own instrument for delivery,
and no green run here is evidence about conduct.

**Performance.** Not part of the preserved contract, and not a property any
claim above states.

**Rust structure.** No claim names a module, a type or a function. The crates
the implementation phase cuts are constrained by the claims they must satisfy,
not by where behaviour sits today — which is the whole point of stating the
contract before reshaping the code.
