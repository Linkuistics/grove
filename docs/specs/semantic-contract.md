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

**A claim is not the checkable unit; an obligation is.** Many claims below carry
several independently reachable cases — two species mismatches, three session
endings, four handoff revalidations — and a single command answering to the
claim prefix can make the identifier look covered while every other case stays
unmodelled. So each claim whose content is **more than one** obligation
enumerates them, and each enumerated obligation carries its own permanent
sub-identity `TT-nn.x`, where `x` is a lower-case letter fixed for the life of
the obligation. The model spelling drops the separator: `TT-02.b` ↔ `TT_02b`.

The enumeration is written in one fixed shape, so extracting it is a match
rather than a parse. Under a multi-obligation claim:

```md
*Obligations*:
- `TT-02.a` — <what must hold>. *Witness*: <what must be reached>.
- `TT-02.b` — <what must hold>. *Witness*: <what must be reached>.
```

A claim with exactly **one** obligation carries no letters and keeps a bare
`*Witness*:` line; the claim identifier is then itself the unit. A claim
therefore has either zero sub-identities or at least two, and a claim that grows
a second obligation grows letters at the same moment. **This document is the
manifest**: the runner reads the obligation lines out of it and requires each
one to be answered, rather than trusting a mnemonic to imply that a case was
covered. **The manifest is the obligation lines outside fenced blocks**: the
shape above is documented by showing it, so the example is otherwise
indistinguishable from a real obligation and a reader extracting the manifest
would count it twice. Renumbering an obligation is a breaking change to every citation of it,
exactly as it is for a claim identifier, so a retired obligation's letter is
retired with it and never reused.

An obligation a model family cannot express is **declared, not dropped**: the
family's `README.md` records the identifier — claim or sub-identity — the reason
(inexpressible, abstracted away, outside bounds, or tool-limited) and what would
change the answer. A declared gap is **per family**: the other family is not
excused by it, and a gap declared on both sides is a finding about the catalogue
rather than a covered obligation, so the runner reports the count of
both-family gaps separately. The runner counts a declared gap as covered *for
the family that declared it* and reports it, so "not modelled" and "forgotten"
never look alike.

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

**An obligation's prefix is a crate assignment, and the rule that decides it is
[`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md).**
The runner sends every `TT_` command to `crates/grove-task-tree/models/`, every
`FN_` command to `crates/grove-finish/models/` and every `SY_` command to
`models/system/`, and refuses a command whose prefix disagrees with its
directory. So an obligation belongs to the scope that can execute its context,
ordered by the approved crate dependency direction; a clause a scope cannot check
stays in place only as a declared **cross-scope citation** to the obligation that
owns it, carrying that obligation's declared narrowings. A gap declared by
**both** families is the signal to apply the rule, not a place an obligation may
rest.

**`models/run.sh` is the one repository runner**, and it has four obligations
beyond running commands:

1. **Abort on a dead tool.** A tool that failed to launch reports what a tool
   that found nothing reports. Launch-failure output is a runner error, never a
   result. This is not hypothetical — the measurement host's default `java` is
   below Alloy 6's floor
   ([`docs/preservation-baseline.md`](../preservation-baseline.md) §1).
2. **Fail on zero work.** A model file no runner reaches, a command set that is
   empty, a witness that never lands, and a verification step that was skipped
   are each a runner failure that names itself.
3. **Assert obligation coverage in both directions, per family.** The unit is
   the pair `(family, obligation)`, where an obligation is a claim with no
   sub-identities or one lettered sub-identity of a claim that has them. For
   **each** of the two families independently: every obligation in this document
   must be answered by a property command plus each witness that obligation
   requires, or by that family's own declared gap; and every
   `TT_`/`FN_`/`SY_`-prefixed command in that family must name an obligation
   this document defines. One direction catches an obligation nobody modelled;
   the other catches a command answering to nothing.

   **Per family** is the load-bearing half. The node brief requires two
   independently constructed families that each cover all three scopes; a
   coverage rule satisfied by either family alone lets one of them omit a claim
   entirely and still report green, which is the divergence the comparison would
   then be measuring instead of the formalisms. The runner therefore reports a
   coverage matrix over `(family, obligation)` and fails on any empty cell that
   is not a declared gap, and the evidence record below is written per pair, not
   per claim.
4. **Assert Q4's removal matrix in both directions, per family.** Every
   removable artifact this document names has a row; every row names an artifact
   this document names and cites an obligation this document defines, or `none`,
   or is declared `abstracted`; and every row's evidence citation resolves. The
   rule is stated under *Q4 needs a matrix, not a claim* — *a removable artifact
   with no row fails the run* — and it is an obligation of the runner for the
   same reason obligation 3 is: it is what makes this document, rather than a
   README, the source of truth for what is owed.

   **It rides with coverage assertion and has no flag of its own.** A matrix is
   owed only once a family's column has closed, which is what the absence of
   `--no-coverage` already says; splitting the two would let a run assert
   coverage while excusing the matrix. The split that *is* made is the one
   coverage already makes: an artifact with no row is excused by
   `--no-coverage`, a broken row never is.

**A scope still being built is run as a NAMED SUBSET, not as an expected-red
whole.** While one family's column is empty the unqualified `models/run.sh` is
red, and that redness is the truth about the repository. What it must not become
is a colour anyone is told to ignore: a suite whose red is routinely explained
away has stopped being an instrument. So the phase's green while a column is
under construction is a **named invocation that asserts coverage over exactly
what exists** — `models/run.sh --scope <scope> --family <family>`, which
restricts the matrix to that family's cells and therefore still fails on an
empty one. `--no-coverage` stays what it is: the flag a scope uses *while its own
first family is mid-build*, and the model README says which obligations it claims
so far. The moment a family's column is complete, that scope's `--family` run is
green with coverage asserted, and the README's run line is updated to drop
`--no-coverage` — which is the visible signal that the column closed.

It delegates to the two existing `ordinal-fs-tree` runners rather than
absorbing them, which also gives it a positive control: those suites are known
green, so a repository run that reports them clean while finding nothing
anywhere else is reporting a broken instrument.

### Evidence, and where it is recorded

Per `(family, obligation)`, per run, the runner records: obligation identifier,
family, exact command, bound or trace limit, solver or backend, outcome, the
bound at which that obligation's witness **first** appears, and wall-clock. The
pair is the row key, so a claim with four obligations produces eight rows across
the two families and a missing one is visible as a missing row rather than as an
averaged-away absence. The witness bound is
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
`finish-verdicts-k65` must answer *keep*, *delete/replace* or *defer* with
evidence — and "the model is smaller" is explicitly not evidence. The catalogue
fixes which obligations decide each, so the answer is read off the models rather
than argued — but fixing that required separating two kinds of claim first.

**Strategy-neutral safety, separated from incumbent mechanics.** A question that
asks whether a mechanism should exist cannot be decided by claims that *are* that
mechanism: a candidate protocol would violate the catalogue by construction and
the question would answer itself. So every `FN-` claim carries a **class**, and
the classes are what the questions are decided against:

| class | meaning | may a candidate protocol contradict it? |
|---|---|---|
| **shared safety** | a property any admissible finish protocol must have, stated without naming a mechanism | no — contradicting one disqualifies the candidate |
| **incumbent mechanics** | how *today's* protocol achieves the shared safety properties | yes — this is what a candidate replaces |

A candidate is checked against the shared-safety claims **only**, at the bounds
the incumbent reached them at. The incumbent-mechanics claims stay in the
catalogue and stay checked for the incumbent; they are not evidence about a
candidate either way.

Each question therefore names the shared-safety claims a candidate must **retain**,
the incumbent mechanics it is allowed to **replace**, and the evidence that
classifies it. Where the deciding evidence is a reachability fact about an
incumbent transition (Q3), it is still read only while every retained claim is
green — the reachability answers *is this needed*, and the retained claims answer
*is the answer safe*.

| question | shared safety retained | incumbent mechanics at stake | what would classify it *delete/replace* |
|---|---|---|---|
| **Q1 — does the quarantine need to exist?** | `FN-20`, `FN-24`, `FN-27`, `TT-24` | `FN-19`, `FN-21`, `FN-31` | both candidate strategies — the incumbent quarantine handoff, and disposal-in-place under `relax_EN_03` — checked against the shared-safety claims, and disposal-in-place holding every one of them with each of `FN-24`'s obligations' witnesses reached at a bound no greater than the incumbent's |
| **Q2 — can the three dispositions become two?** | `FN-15`, `FN-25` | — | `FN-15.d`'s bounded-unreachability check passing for `Indeterminate` on a lane, at a bound strictly greater than the one at which `FN-15.b` and `FN-15.c` first land their witnesses, in **both** families |
| **Q3 — is the marker-replacement sub-transaction reachable?** | `FN-24` | `FN-31`, `FN-21`, `FN-22` | `FN-31.a`'s bounded-unreachability branch establishing that no state requires a *replace* rather than a create or a remove, at a bound strictly greater than the one at which `FN-21.a` and `FN-31.b` first land their witnesses, in **both** families. `FN-31.a`'s witness merely failing to land is a `defer`, for the reason Q2 gives |
| **Q4 — what does finish still owe the user?** | `FN-27`, `FN-28`, `SY-05` | every artifact and transition in the matrix below | a row of the removal matrix whose artifact or transition can be removed without breaking **any** shared-safety claim |

**Q2 needs an instrument, not an absent witness.** "The witness never landed" and
"no trace reaches it" are different statements, and only the second decides
anything: an unlanded witness is equally consistent with a bound too small, which
is the pre-registration's *scope trap*. `FN-15.d` therefore requires an explicit
**bounded-unreachability check** per lane — Alloy a `check` over the full scope
that no state carries the disposition, Quint an exhaustive run to the same depth
— and what it can decide is bounded by its own bound and says so: it is evidence
that the disposition is unreachable *within* B, never a proof that it is
unreachable. Delete/replace requires that evidence at a bound strictly greater
than the neighbours' first-witness bounds, in both families; anything weaker is
**defer**.

**Q4 needs a matrix, not a claim.** No claim in this catalogue classifies a
transition by *what it protects*, so Q4 cannot be read off one. What decides it
is an **artifact/transition removal matrix**, recorded in
`crates/grove-finish/models/README.md` by each family: one row per removable
artifact or transition — the reserved witness, the evacuation manifest, its ready
mark, the correlation ticket, the quarantine, the cleanup marker, the replace
transition, the index image, the recorded anchor, the deletion fingerprint —
naming the **first shared-safety obligation** its removal breaks under the
mutation discipline, or `none`. A row reading `none` in both families is Q4's
evidence for delete/replace; a row naming an obligation is evidence the artifact
is protecting the user rather than Grove. The matrix is a runner obligation like
any other: a removable artifact with no row fails the run.

A question whose deciding witness is never reached is **defer**, not delete: an
unreached witness is an absence of evidence, and the pre-registration's
*vacuous invariant* hazard is exactly the habit of reading one as the other. The
only exception is a check that positively establishes unreachability within a
stated bound, which is a different instrument and is named as one wherever it
decides something.

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

**Entry** — anything the walk **reaches** beneath the task root. Exactly one of:
a **charter**, the **format witness**, a **reserved witness**, a **task entry**
(a leaf or a node directory), or **foreign**.

*Reached*, not merely *beneath*: the walk descends into the task root and into
**node** entries, and into nothing else. A directory whose own name is outside
the task grammar is foreign, so grove never opens it — and a perfectly
well-formed task name inside one is therefore **not an entry at all**. It holds
no position on any level grove orders, its key is not part of the counter, and a
malformity in it does not stop the tree. Stated because "anything transitively
beneath" said the opposite and the difference is reachable: a level whose
positions begin at 2 because the level is a foreign directory's contents
satisfies no gaplessness rule, and `TT-06.b` finds it as a counterexample. Every
`TT-` claim that quantifies over "every directory" or "every entry" means the
reached ones.

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
| `PartialScaffold` | present, no format witness, and the contents are exactly a known proper subset of a fresh scaffold |
| `Legacy` | present, no format witness, and not a partial scaffold |
| `Foreign(found)` | the format witness holds something else |
| `Malformed(reason)` | current format, but a grammar or whole-tree invariant fails |
| `Current(Live)` | at least one live non-finish leaf |
| `Current(FinishOnly)` | the only live leaf is the finish sentinel |
| `Current(Spent)` | no live leaf at all |

A `Reserved` state may additionally carry a **blocked diagnosis** — see
[Outcomes](#outcomes) — which is what an interrupted transaction leaves behind
once recovery has run and could not settle it.

**`PartialScaffold` exists because `TT-20` and `SY-06` need somewhere to land.**
Root initialisation makes the format witness visible last, so an interruption
before it lands leaves a present root with no witness — which the classification
order would otherwise call `Legacy`, and legacy work would then be *completed* as
though Grove had scaffolded it. The state is therefore defined by an exact,
closed subset rather than by the absence of the witness:

> the task root exists; it contains no format witness; and its entries are a
> subset of `{the root charter, exactly one positioned entry, which is the first
> `requirements` leaf at position 1 with key 1}`, with the charter's and the
> leaf's bytes equal to what a fresh scaffold writes; and it contains nothing
> else.

Anything outside that subset — a second positioned entry, a differing byte, a
foreign entry, a node directory — is **not** a partial scaffold and falls
through to `Legacy`. The subset is what makes completion safe: every value the
completion would write is fixed in advance, so completing is a comparison
followed by at most one append, never an inference about someone else's tree.
`PartialScaffold` is ordered **before** `Legacy` and, per `SY-06`, is completed
before any format classification runs.

**`Malformed` carries a reason, not only an entry**, because not every malformity
is local to one entry. `TT-13` makes two individually well-formed live finish
leaves malform the *whole tree*, and an entry-shaped state cannot say that. The
reasons are closed and enumerated:

| reason | what failed |
|---|---|
| `MalformedEntry(entry)` | a task-shaped entry does not parse completely, or its session kind is absent or unknown (`TT-03`) |
| `SpeciesMismatch(entry)` | a name declares a species the on-disk entry is not (`TT-02`) |
| `PositionsNotGapless(dir)` | a directory's positions are not `1..n` without repetition or gap (`TT-06`) |
| `KeyReissued(key)` | one key names two entries (`TT-05`) |
| `MultipleLiveFinish` | more than one live finish leaf exists anywhere in the tree (`TT-13`) |
| `NodeWithoutCharter(dir)` | a node directory carries no charter |

Every reason is a whole-tree classification and stops every read and mutation,
which is `TT-03`'s rule generalised: the entry-local ones name the entry so the
operator can find it, and `MultipleLiveFinish` names the tree because that is
what is wrong. A model reaching a malformity not on this list is a finding
about the list, not a licence to invent a seventh.

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
| **Finish** | `allocate-finish-leaf`, `finish-commit`, `recover`, `dispose-quarantine`, `replace-cleanup-marker`, `reap-quarantine` | exclusive, plus the repository |
| **Lifecycle** | `acquire-lease`, `layout-preflight`, `open-epoch`, `launch`, `reap`, `close-epoch`, `release-lease` | lease, then epoch |
| **Environment** | `crash`, `hand-edit`, `foreign-write`, `topology-change`, `confirm` | none — these are the world's |

**Of the Lifecycle group, only `launch` is gated on the task root's
classification.** `acquire-lease`, `layout-preflight`, `open-epoch`, `reap`,
`close-epoch` and `release-lease` read and write no task tree, so none of them is
refused on an absent, legacy or malformed root. The table said nothing either
way and the silence was load-bearing: **gating `reap` makes `SY-05.a`
unwitnessable rather than false.** The driver runs the finish, proves the
deletion, the root becomes `Absent`, and the session that committed the teardown
ends — and a gated `reap` is then refused `RootAbsent`, so the loop never
collects that ending, never opens a new iteration, and never scaffolds the fresh
grove a missing task root *means*. A suite without a witness obligation reports
that as a green `SY-05.a` over a lifecycle that physically cannot happen.

**The two families filled the silence in opposite directions, which is how it
was found, and all three readings now agree.** Quint gated `reap` and its
`wit_SY_05a` never landed; Alloy's `doReap` reads no tree at all; and the shipped
reap path — `complete_post_reap_epoch_handoff` in
[`src/loop_driver.rs`](../../src/loop_driver.rs), whose operands are the launch
result, `invalidate_session_epoch()` and `complete::read_signal()` — touches no
task tree and classifies no root. `launch` is
the exception in both families for the same reason — it consumes a selection,
and selection is an Observation action that does read the tree — so gating it is
the tree read arriving one step earlier rather than a second gate.

`replace-cleanup-marker` is in the table rather than folded into
`dispose-quarantine` because [`TODO.finish_process.md`](../../TODO.finish_process.md)
Q3 asks whether *replacement* — as against creating a marker or removing one — is
reachable at all. A model that folds it away answers Q3 by construction, which is
the shape of a false-confidence incident rather than a finding. `FN-31` is its
claim; the [deliberate omission](#deliberate-omissions) covers only the marker's
byte layout.

**The groups belong to different scopes, so *action* owns none of them, and an
obligation quantified over it is read prefix-locally.** Observation and Tree
mutation are `grove-task-tree`'s, Finish is `grove-finish`'s, Lifecycle is the
application joint's, and Environment is the world's and admitted by each model
that needs it. A claim stated over *every action* is therefore not placeable as
one obligation: it becomes one obligation per scope, each ranging over **exactly
what that scope admits** and saying so in its own text — `SY-14`'s *no admitted
action*, and `TT-24.a`'s
([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md),
clause 4). The same reading governs every other group-spanning term this
vocabulary defines.

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
| `Refused(r)` | the action left nothing standing; the tree it returns is byte-identical to the tree it received |
| `Blocked(b)` | an effect of the action stands that it could neither complete nor undo, in a stable, recoverable state |

`Empty` and `Ambiguous` are outcomes rather than refusals because that is the
shipped contract and callers branch on it: selection on a spent grove and a
resolution that matched nothing are both successes that mutate nothing and
report nothing to standard output (`TT-15`).

**`Refused` and `Blocked` are separated by what the action leaves, never by
where it stopped**, and this sentence is the whole of the discriminator. An
action returns `Refused(r)` when the tree it hands back equals the tree it was
given — whether because no step ran at all, or because every step that ran was
undone. It returns `Blocked(b)` when an effect stands that the action could
neither carry to completion nor reverse. The operational content is the one
`FN-29` requires the operator to be able to read: **a refusal leaves nothing to
recover, and a block does.**

Three consequences, and each was a live mistake before it was written down.

- **The unit is the action, not the step and not the process.** A step's own
  frame condition — *this step mutated nothing* — is not the discriminator. The
  finish transaction publishes a witness, evacuates every entry and then attempts
  a commit; a stop at that commit is a stop by a step that moved nothing inside
  an action that has moved everything, and it is a `Blocked`. Equally, the
  driver's loop is many actions in one process, so an effect an *earlier*
  transition applied does not make a later transition's clean stop a block.
- **A rollback earns the refusal back.** `NotCommitted` restores the tree and
  ends `Refused` (`FN-29`) even though the action reached the commit, because
  what the caller is handed is the tree it gave. `FN-22`'s table applies exactly
  this rule to all ten of its rows, and applied it before the rule was stated.
- **An intermediate state is not a return.** `FN-22.f`'s successful quarantine
  return settles to `Reserved(Published)` and the attempt then completes as
  `Refused` from the restoration path; the outcome belongs to the attempt, not
  to the state it passed through.

**This was found, not designed, and the way it was found is the argument for
stating it.** `crates/grove-finish/models/finish.als` read the discriminator
step-locally and answered `FN-13` `Refused` with `treeSame` true of the step;
`crates/grove-finish/models/finish.qnt` read it action-locally and answered
`Blocked`. **Both were green**, because the catalogue supplied no predicate for
either to be wrong against. Where it happened to supply one in the neighbouring
sentence — `FN-17.b`'s *blocks rather than proceeds*, three lines below
`FN-16`'s *refused* — the two families agreed without ever discussing it. An
unstated discriminator is not a small omission in a catalogue two independent
families descend from; it is the one thing they cannot check each other on.
*See*: [`a-refusal-leaves-nothing-standing`](../adr/a-refusal-leaves-nothing-standing.md).

**A guard wait is not an outcome, and the omission is deliberate.** `TT-22`
serializes an observation against a mutation, and nothing in the closed set above
says what the *waiting* caller sees. It sees nothing: `src/tree_access.rs`
acquires with `flock` and no `LOCK_NB`, so the tree lock **blocks** and no
invocation ever returns while it is held. The set covers what a completed
invocation returns, and a wait is not a return; adding it here would put a
tree-level twin of `LeaseHeld` into the refusal list for a refusal Grove does not
produce. A model that needs the waiting state to be *observable* — because
otherwise a failed guard is an absent transition and `TT-22` is true by
construction — introduces it as an abstraction of its own and records it as one
([`crates/grove-task-tree/models/task-tree.als`](../../crates/grove-task-tree/models/task-tree.als),
`Deferred`).

**That paragraph states a rule and not a special case, and it governs every
situation in which Grove is never invoked at all.** The closed set covers what a
*completed invocation returns*. A situation in which no invocation happens is
therefore outside it by construction and gains no member — but a model that
leaves it as an **absent transition** makes the obligation about it true by
construction and unfalsifiable, which is the exact hazard `Each action is total`
exists to remove. So the model names the non-event, declares it as an
abstraction, and the catalogue gains nothing. There are exactly two such
situations and the second is `FN-01`'s: **Grove has no confirmation gate at
all.** Constraint 5 is *grove guides, it does not gate*; `finish_commit`'s own
contract is that "whether a human confirmed teardown is the calling finish
session's responsibility"
([`src/tree_lifecycle.rs`](../../src/tree_lifecycle.rs)), no Grove binary reads
standard input anywhere, and `EN-15` grants that Grove cannot verify a
confirmation. A transaction not entered for want of confirmation is a call that
was never made, so `FN-01`'s *produces no refusal at all* is literally true of
the product, and `crates/grove-finish/models/finish.qnt`'s `ONotEntered` is the
guard wait's rule applied a second time rather than a second finding about the
set.

**Refusal reasons**, closed:

`RootAbsent` · `FormatLegacy` · `FormatForeign` · `WitnessPending(class)` ·
`ReservedNameOccupied(entry)` · `Malformed(reason)` · `NotLive` ·
`AlreadyTerminal` · `ReservedKind` · `NotAnEntry` · `DestinationOccupied` ·
`LayoutUnsupported` · `LeaseHeld` · `EpochStale` · `NoTrackedDeletion` ·
`RootIdentityChanged` · `UnsupportedEntryType` · `DeletionNotCommitted` ·
`ConfigurationInvalid` · `GenerationContended`

**A reason names the question that was asked and answered no — never the gate
that asked it.** The catalogue already relies on this at `FN-05.a`, where an
unsupported layout and an unreachable quarantine operand are the *same* reason
because `SY-03` makes them one question asked at two gates. What follows, and is
the last three members' whole justification, is that **the set gains a member
exactly when a scope asks a question no member names** — and that reporting such
a case under the closest true member instead is not a smaller version of the
same fix, it is a different and worse one.

- **`DeletionNotCommitted`** — the deletion commit did not land, and the
  transaction rolled back. Distinct from `NoTrackedDeletion`, which is the
  preflight question *is there anything tracked to delete* (`FN-07`); this is
  the post-commit question *did the delete land*. `NoTrackedDeletion` and
  `RootIdentityChanged` are each **false** of a transaction whose fingerprint was
  fine and whose root never moved, so reporting under either is a lie no run
  could catch.
- **`ConfigurationInvalid`** — session configuration failed validation
  (`SY-04.b`,
  [`complete-session-configuration`](../adr/complete-session-configuration.md)).
  Shipped and operator-visible today, with its own diagnostics
  ([`src/session_config.rs`](../../src/session_config.rs)), and named by no
  existing reason.
- **`GenerationContended`** — the launch-generation handoff timed out against an
  operation already admitted under the previous epoch
  ([`one-live-driver-per-working-tree`](../adr/one-live-driver-per-working-tree.md)).
  Distinct from `EpochStale`, which is `SY-10.a`'s *mismatch* — the operation
  named a generation that is not live — where this is contention for one that is.

**Why all three arrive at once, and where the next one will come from.** The
seventeen were drawn over the questions the task-tree scope asks: preconditions
and guards on a tree. The set is swept by **three** scopes, and every member
added here is a question a *later* scope asks — a commit's disposition, a
configuration, a launch generation. That is the pattern under what the finish
scope recorded as three separate accidents, and it predicts the remaining gap
rather than merely listing the closed ones. **A fourth instance is already
recorded and is deliberately not decided here**: an ordinary operation meeting a
`PartialScaffold` has no reason either, and
`crates/grove-task-tree/models/task-tree.qnt` refuses `WitnessPending(RPreparing)`
as least-wrong while naming a reserved witness that is not there. It is
`task-tree-scope-k70`'s, because deciding it needs that leaf's own items — what
`PartialScaffold` *is*, and whether the state table gains a member for the
shipped *ambiguous partial root scaffold* refusal — settled first. **It inherits
a frozen set and a stated rule rather than an open question**, which is the whole
reason the vocabulary was decided ahead of the scopes.

**The rejected alternative is the one both families independently chose, twice.**
Report the case under the closest true member and keep it distinguishable with a
model-only observable — `Sys.why` in `crates/grove-finish/models/finish.als` and
its Quint counterpart. It is cheaper, it keeps the set small, and it is what a
model must do while the set is closed against it. Its cost is that **the reason
stops naming the question**: an operator told `WitnessPending` cannot learn from
it that the *repository*, not the filesystem, is what refused. A device a model
has to invent twice to say what an outcome could not is evidence about the
vocabulary, not about the model.
*See*: [`a-refusal-leaves-nothing-standing`](../adr/a-refusal-leaves-nothing-standing.md).

`WitnessPending` and `ReservedNameOccupied` are the two halves of one situation
and the split is deliberate: `WitnessPending` is an artifact at a reserved name
that Grove **can** prove is its own and can name the recovery for, while
`ReservedNameOccupied` is one it cannot classify at all. Telling an operator to
run a recovery against someone else's bytes is exactly the fail-closed violation
`TT-24` exists to prevent, so the second reason names the entry and no recovery.

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

**The partition is over `Blocked` outcomes and nothing else.** `FN-25` states it
about blocks, not about every unhappy result: a refusal is not a block and
carries a refusal reason instead, and reading `OwnershipConflict` onto a refusal
would make the partition neither disjoint nor exhaustive over anything.

**One artifact, three contexts, one decided outcome.** A foreign entry sitting at
a name Grove reserves is reachable from three places, and the catalogue fixes
each rather than letting a model choose (`TT-24`):

| context | outcome | checked by |
|---|---|---|
| an ordinary tree operation, before any transaction | `Refused(ReservedNameOccupied(entry))` — the tree is byte-identical | `TT-24.b` |
| inside a live finish or recovery transaction | the step stops and the artifact is untouched; **which** stop follows from what the action has left standing — `Refused(ReservedNameOccupied(entry))` while nothing of it stands, `Blocked(OwnershipConflict)` once something does | `FN-32`, `FN-29.b`, and the step's own obligation |
| the quarantine reaper (`FN-21`) | declines the entry, mutating nothing, and reports it; the sweep continues over entries the reaper *can* prove are Grove's | `FN-21.c` |

The three agree on what matters — nothing foreign is ever mutated, which is
`TT-24.a` — and differ only in what the caller can be told, which is a function
of how far the caller had already gone. **The rows are checked in two scopes**,
because two of the three contexts are `grove-finish`'s
([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md)).

**The second row said `Blocked(OwnershipConflict)` flatly, was then recorded as
underdetermined, and is now decided — by the rule above rather than by picking a
column.** *Inside a transaction* was never the discriminator, and this claim's
own preamble said so all along: the outcome is fixed by *how far the caller had
already gone*. What was missing was a predicate for *how far*, and
[§Outcomes](#outcomes)' opening now supplies it. The row therefore fixes no
single outcome and fixes something better — **a function of the step**, which is
what a claim quantified over every step of a transaction needs:

- **`FN-10.b`'s discard is a refusal.** Recovery meets unclassifiable content at
  the witness slot before it has moved anything, so the tree it hands back is the
  tree it received, and the closed set has named this exact case since before
  either model existed: `ReservedNameOccupied(entry)` — *an artifact at a
  reserved name Grove cannot classify at all*.
  `crates/grove-finish/models/finish.als` is right here, and
  `crates/grove-finish/models/finish.qnt`'s `SRemoveWitness` discard branch moves
  to the refusal.
- **A later step is a block.** A transaction that has published, evacuated or
  renamed and then meets such an artifact leaves an effect standing, so it stops
  `Blocked`. The diagnosis is `FN-25`'s own partition applied and not a new one:
  `OwnershipConflict`, because the artifact is by hypothesis one Grove **cannot**
  classify as its own.

**The two columns disagreed because they were answering two different questions
and neither knew it.** Alloy's `treeSame` is a step frame condition; Quint's
block test is an action-level one. Both are correct readings of *fails closed*,
which is all `FN-10.b` said, and both stayed green — the failure mode a suite
cannot report. The repair is not to pick a column but to state the predicate the
two were guessing at, and to give it an obligation so the next divergence is a
counterexample rather than two green runs: `FN-29.b`.

**The two diagnoses are a partition the catalogue introduces, and the shipped
implementation does not yet draw it.** Today's classification yields three
commit *dispositions* — `Committed`, `NotCommitted`, `Indeterminate` — and
gathers under one blocked state both of the cases above. The root brief requires
them distinguished; `FN-25` states the partition as a claim so the models decide
whether it is total, disjoint and reachable on every lane, and
`handoff-audit-k66` decides on that evidence whether the shipped diagnostic
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
therefore **mutated in at least one model**, in a named instance or scope of its
own, and each mutation has a stated expected result that the runner checks.

**Three controls, not one.** The mutations below are not all the same experiment,
and requiring all of them to "break a claim" is wrong for two of the three
classes — a capability added to test a cheaper protocol is supposed to leave the
safety claims standing. Every row therefore declares its class:

| class | what the mutation does | expected result |
|---|---|---|
| **premise-break** | removes a capability a claim's correctness rests on | a **named obligation fails**. If every obligation stays green, the assumption was carrying no weight and that is the finding. |
| **exercise-removal** | removes a dimension the models exercise — an action, a scope, a parameter | **named witnesses become unreachable**, and the property checks stay green. A witness that still lands was not exercising the dimension. |
| **counterfactual-capability** | *adds* a capability, to ask whether a cheaper protocol is admissible | every **shared-safety** obligation stays green, and the named **incumbent-mechanics** claims are deliberately out of scope for the candidate. A counterfactual capability is never required to falsify the safety property it exists to retain. |

A row's `controls` column names the obligations the expected result is stated
over. An empty `controls` column is not permitted: if no obligation depends on an
assumption, the assumption is either unnecessary or a claim is missing, and
either way it is a finding recorded in the experiment log rather than a blank
cell.

| id | assumption | class | mutated by | controls | expected result |
|---|---|---|---|---|---|
| `EN-01` | A same-directory rename is atomic with respect to namespace visibility. | premise-break | Quint — `relax_EN_01`, a rename observable half-applied | `FN-09.a`, `FN-19`, `FN-24.a`, `TT-20` | `FN-09.a` fails — a published witness is observable half-renamed — and `FN-24.a` fails with it, since the torn state is classifiable as two stable states |
| `EN-02` | A rename cannot cross a filesystem boundary. | exercise-removal | Alloy — a two-device scope | `FN-08` | with a single-device scope, `FN-08`'s witness — a layout that passes at lease acquisition and fails at the transaction's own operands — is unreachable; the property stays green |
| `EN-03` | There is no atomic recursive directory deletion. | counterfactual-capability | Quint — `relax_EN_03`, disposal as one step (this is Q1's counterfactual) | retained: `FN-20`, `FN-24`, `FN-27`, `TT-24`. replaced: `FN-19`, `FN-21`, `FN-31` | every retained obligation stays green under the candidate, at bounds no greater than the incumbent's; the replaced claims are not checked against the candidate and their failure under it is not evidence |
| `EN-04` | There is no atomic replacement of a file by a differently named directory. | counterfactual-capability | Alloy — the promotion structure, inherited from the delegated boundary | retained: `TT-07`, `TT-08`, `TT-09`. exercised: `TT-02.b` | with promotion atomic, `TT-07`, `TT-08` and `TT-09` stay green, and `TT-02.b`'s witness still lands by `hand-edit` (`EN-11`) rather than through a half-promoted entry — which records that `EN-04` buys step count, not safety, and that no claim in this catalogue depends on it |
| `EN-05` | No filesystem transaction can include a version-control commit. | counterfactual-capability | Quint — `relax_EN_05`, commit and evacuation as one step (this is Q2's counterfactual) | retained: `FN-03`, `FN-15`, `FN-24`, `FN-25`, `FN-27`. replaced: `FN-09`, `FN-11`, `FN-22` | every retained obligation stays green; `FN-15.d`'s bounded-unreachability check passes for `Indeterminate` on every lane, and `FN-25.b` is then exhaustive over one diagnosis — which is the evidence Q2 reads |
| `EN-06` | Locks are advisory: only cooperating processes are serialized, and a direct edit is outside the guarantee. | exercise-removal | Quint — `relax_EN_06`, a non-cooperating writer | `TT-21.b`, `TT-22` | removing the non-cooperating writer makes `TT-21.b`'s witness unreachable, while `TT-22`'s obligations stay green — which is exactly the content of the assumption, and the reason `TT-21` cannot claim to exclude one |
| `EN-07` | Two open descriptions of one directory do not share a lock. | premise-break | Alloy — a shared-lock scope | `SY-11.b`, `TT-22.b` | `SY-11.b` fails: the shared-lock scope reintroduces the cycle [`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md) records |
| `EN-08` | Interruption may occur between any two steps. Power loss, kernel failure and storage-cache loss are outside the contract. | exercise-removal | Both — `crash` is a first-class action; the mutation is its removal | `FN-09`, `FN-10`, `FN-24`, `FN-31.c`, `SY-12`, `TT-20`, `TT-23.b` | with `crash` removed, every named witness is unreachable and the run fails on zero work rather than reporting green |
| `EN-09` | A command's exit status is not a receipt: a result may be lost or arrive late. | exercise-removal | Alloy — a trace in which the result arrives after the classification | `FN-15.a` | removing the late-arrival trace makes `FN-15.a`'s witness — a lost result reported as failure while the exact commit exists — unreachable |
| `EN-10` | The names are the counter: key allocation reads the tree, and entries are never removed. | premise-break | Quint — `relax_EN_10`, an entry removed | `TT-05`, `TT-12` | `TT-05` fails: allocation re-issues a key a removed entry held |
| `EN-11` | Any well-formed tree is reachable by hand edit. | exercise-removal | Both — `hand-edit` is a first-class action; the mutation is its removal | `TT-02`, `TT-03`, `TT-13.c`, `TT-25` | with `hand-edit` removed, every witness that posits a tree Grove's own actions cannot build is unreachable. **`TT-16` was listed here and has been removed**: a resolved *terminal* entry is something Grove's own actions build — allocate, retire, resolve — so its witness never needed the assumption, and the Alloy run reaches it with `hand-edit` gone ([`crates/grove-task-tree/models/task-tree.als`](../../crates/grove-task-tree/models/task-tree.als), `witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit`). **`TT-24.b` was listed here and has been removed too, for the same reason and on a fired control**: its witness is an ordinary operation meeting a foreign entry at a name Grove reserves, and `EN-13` grants that foreign entries appear **at any name**, so `foreign-write` alone supplies one — the witness is reached in ~2% of traces with `hand-edit` gone (`crates/grove-task-tree/models/task-tree.qnt`, `wit_finding_EN_11_does_not_gate_TT_24b`). `TT-24.b`'s dependency is `EN-13`, where it is already listed |
| `EN-12` | A name renders as exactly one path component. | premise-break | Alloy — a rendering that escapes its level | `TT-01.a`, `TT-06` | `TT-01.a` fails: two spellings denote one entry, and the level's positions stop being a per-directory sequence |
| `EN-13` | Foreign entries may appear at any name and are not Grove's to delete. | premise-break | Quint — `relax_EN_13`, a sweep of a reserved namespace | `TT-04`, `TT-24.b`, `FN-21.b`, `FN-27` | `TT-04` fails in the task-tree scope and `FN-21.b`/`FN-27.a` in the finish scope: the sweep deletes bytes a refusal exists to preserve. **`TT-24.d` was listed here and is retired** — its content is `FN-21.c`'s, per the placement rule; the sweep is `grove-finish`'s action and the row's task-tree half is `TT-04`'s alone |
| `EN-14` | The working-tree root exists before the task root and outlives its deletion. | premise-break | Alloy — a scope in which the root itself is removed | `TT-22`, `SY-01`, `SY-05` | `SY-01` fails: ownership has nothing to be held on, so a second driver is admitted |
| `EN-15` | Confirmation is an operator input Grove cannot verify. | counterfactual-capability | Quint — `relax_EN_15`, a machine-attested confirmation | retained: `FN-01.a`, `FN-01.b` | **no obligation becomes stronger and none fails.** A machine attestation replaces nothing: `FN-01.a` still forbids running without confirmation and `FN-01.b` still refuses the deterministic guard as a substitute. A run in which some obligation *does* strengthen is the finding, because it would mean a claim was resting on the attestation rather than on the guard |
| `EN-16` | The three lanes differ in mechanism and agree on abstract outcome. | exercise-removal | Both — the lane is a model parameter; the mutation is collapsing it to one | `FN-15.b`, `FN-15.c`, `FN-15.d`, `FN-17`, `FN-25.c` | with one lane, `FN-25.c`'s per-lane witnesses are unreachable and `FN-17`'s working-copy-as-commit obligation has no instance; every `FN-` property stays green, which is what makes the collapse invisible without this control |

**An exercise-removal row's controls column is a claim of unreachability, and it
SHALL be established by running the removal rather than by reading the witness.**
**The table above has now been wrong three times, across two of its rows and
found by both families** — which is a rule rather than three typos. `TT-16` and
`TT-24.b` were listed under `EN-11` and are each reached with `hand-edit` gone
(the first by Alloy's `task-tree.als`, the second by Quint's
`task-tree-controls.qnt`); and `EN-08` names `FN-31.c` among the witnesses that
become unreachable when `crash` is removed, while
[`crates/grove-finish/models/README.md`](../../crates/grove-finish/models/README.md)
records that its two land anyway. The shape is the same in all three — a witness
that *looks* like it needs the assumption because its prose describes a state an
operator or an interruption would produce — and the two failures it produces are
different, so both are named:

- **The row is wrong.** The witness has another route to it and never needed the
  assumption. `TT-16` and `TT-24.b` are this: correct the column, and nothing
  about either claim changes.
- **The row is right and a family does not meet it.** The witness *posits* the
  state rather than reaching it, so removing the action leaves it landing.
  `FN-31.c` is this, and it is a fact about that family's realisation rather
  than about the assumption. **A posited state and a reached one are not
  interchangeable for an assumption's control**, however interchangeable they are
  for the claim the witness serves — which is exactly what an exercise-removal
  exists to make visible, and is invisible without one.

Telling the two apart is what running the removal buys, and nothing else does.

`EN-08`, `EN-11` and `EN-16` are *exercise-removal* rather than premise-break
because their negation is not a smaller world but a different one: a model with
no `crash` action, no `hand-edit` and one lane is the model this experiment
exists to avoid. Their control is unreachability, not failure — the properties
stay green, which is precisely why a green run under a collapsed dimension is the
false confidence the pre-registration names.

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
*Obligations*:
- `TT-01.a` — distinct filenames never denote one entry. *Witness*: a tree in
  which two spellings of one entry would otherwise both parse.
- `TT-01.b` — parse-then-render reproduces the input exactly, and any other
  spelling is refused naming the canonical one. The refusal is
  `Malformed(MalformedEntry(entry))`, and the canonical spelling is what the
  diagnostic carries: a non-canonical spelling is task-shaped and does not parse
  *completely*, which is that reason's own definition. Fixed here rather than
  left to each family, because a spelling refusal and an unknown-kind refusal
  are the same reason and a model that invents a second one has widened the
  closed set. *Witness*: a non-canonical spelling refused.
*Cites*: [`task-names-are-canonical`](../adr/task-names-are-canonical.md).

**`TT-02` — a name declares its species and must be it.** A task-shaped name
SHALL denote an on-disk entry of the species the name declares; a leaf name at a
directory, or a node name at a file, SHALL be malformed with reason
`SpeciesMismatch(entry)`.
*Obligations*:
- `TT-02.a` — a leaf name at a directory is malformed. *Witness*: that tree.
- `TT-02.b` — a node name at a file is malformed. *Witness*: that tree.

**`TT-03` — malformed halts, and never skips.** A task-shaped entry that does
not parse completely, or whose session kind is absent or unknown, SHALL stop
every read and mutation of the whole tree, naming the entry and the admissible
kinds.
*Witness*: a malformed node directory whose subtree holds live work — the case
where skipping would report a finished grove.

**`TT-04` — foreign entries are ignored and preserved.** An entry outside the
task grammar SHALL neither be read as work nor mutated by any action, and
**neither shall anything beneath it**: a foreign directory is not descended
into, so its whole subtree is invisible to every read and untouched by every
mutation ([Identities](#identities)).
*Witness*: a foreign entry surviving a mutation that renamed its siblings; and a
canonical, known-kind task name inside a foreign directory that is not an entry,
holds no position, and contributes no key.

**`TT-05` — keys are unique, permanent and never reissued.** Allocation SHALL be
one past the maximum key over every entry in the tree, terminal entries
included, and no key SHALL ever be issued twice.
*Witness*: an allocation whose maximum comes from a terminal entry.
*Cites*: [`entries-are-never-removed`](../adr/entries-are-never-removed.md).

**`TT-06` — positions are per-directory and gapless.** Every **reached**
directory's positions SHALL be `1..n` with no repetition and no gap; a directory
that is not SHALL be malformed with reason `PositionsNotGapless(dir)`. A
directory the walk does not enter has no positions to be gapless, which is what
`TT-04` and [Identities](#identities) settle.
*Obligations*:
- `TT-06.a` — an append lands at `n+1` and closes no gap. *Witness*: that insert.
- `TT-06.b` — an insert at an occupied position shifts every later sibling.
  *Witness*: that insert.

**`TT-07` — a shift preserves everything but position.** Insertion and
renumbering SHALL change positions only — never a key, slug, kind or outcome
infix, and never any file's bytes. The byte clause is discharged by the **entry
digest** — the opaque equality [Identities](#identities) already defines — and
not by reading any file's contents, which the [deliberate
omissions](#deliberate-omissions) forbid. Stated here because the omission read
on its own removes the clause from both families, and a clause no model can
reach is not a claim.
*Witness*: a shift across a directory containing every species.

**`TT-08` — decomposition preserves the key.** A leaf promoted to a node SHALL
keep its own key, and the promoted subtree's names and keys SHALL be untouched.
*Witness*: a promotion of a leaf whose key is the tree's maximum.

**`TT-09` — every mutation is one algebraic operation plus a domain
precondition.** No action SHALL move an entry outside an append, insert,
promotion or rewrite of the entry-name algebra; ordering, shifting and
allocation SHALL be properties of that algebra alone.
*Obligations*:
- `TT-09.a` — append. *Witness*: reached.
- `TT-09.b` — insert. *Witness*: reached.
- `TT-09.c` — promotion. *Witness*: reached.
- `TT-09.d` — rewrite. *Witness*: reached.
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
only live leaf. More than one live finish leaf SHALL malform the whole tree.
*Obligations*:
- `TT-13.a` — the walk skips a live finish leaf while any non-finish leaf is
  live. *Witness*: a finish leaf at an earlier position than live ordinary work —
  the case where the skip rule is the only thing preventing teardown.
- `TT-13.b` — the walk returns the finish leaf when it is the only live leaf.
  *Witness*: that tree.
- `TT-13.c` — two live finish leaves anywhere in the tree classify the **tree**
  as `Malformed(MultipleLiveFinish)`, not either entry as malformed, and stop
  every read and mutation. *Witness*: two individually well-formed live finish
  leaves in different subtrees.

**`TT-14` — selection is not a scheduler.** No dependency, priority or grouping
SHALL affect the order; the only mechanisms are position and terminality.
*Witness*: two orderings of the same work selecting differently.

**`TT-15` — an empty or ambiguous observation is a success.** Selection on a
spent tree, a resolution matching nothing, and a resolution matching several
SHALL each mutate nothing, refuse nothing, and be distinguishable from one
another by their reported value alone.
*Obligations*:
- `TT-15.a` — selection on a spent tree reports `Empty`. *Witness*: reached.
- `TT-15.b` — a resolution matching nothing reports `Empty`. *Witness*: reached.
- `TT-15.c` — a resolution matching several reports `Ambiguous(cs)`. *Witness*:
  reached.

**`TT-16` — a resolved terminal entry is never mistaken for live.** A resolution
that matches a terminal entry SHALL report both the entry and its terminality.
*Obligations*:
- `TT-16.a` — a resolved `Done` entry. *Witness*: reached.
- `TT-16.b` — a resolved `Abandoned` entry. *Witness*: reached.

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
same-directory rename, so no reader observes a torn or premature marker. The
root it leaves behind on interruption SHALL classify as `PartialScaffold` — never
as `Current(*)` and never as `Legacy`.
*Witness*: an interruption before the witness lands, classified as
`PartialScaffold`.

**`TT-21` — one snapshot per operation.** Every classification an operation
makes SHALL be computed from a single listing taken under that operation's
guard. This is a claim about **internal consistency, not about excluding the
world**: `EN-06` grants only that cooperating processes are serialized, so
`hand-edit` and `foreign-write` may interleave at any point during an operation
and the operation may therefore act on a world that has already moved. What the
claim forbids is an operation drawing two classifications from two listings and
reaching a conclusion neither listing supports.
*Obligations*:
- `TT-21.a` — a cooperating writer between two classifications is excluded by the
  guard. *Witness*: that interleaving, shown serialized.
- `TT-21.b` — a **non-cooperating** writer interleaving mid-operation does not
  falsify the claim: every classification the operation made still comes from its
  one listing, and the operation's outcome is a refusal, a block, or a mutation
  that was licensed by that listing. *Witness*: a `foreign-write` landing between
  two classifications, with the operation's classifications still mutually
  consistent.

**`TT-22` — guards are shared for observation and exclusive for mutation**, and
are taken on the working-tree root.
*Obligations*:
- `TT-22.a` — two concurrent observations are admitted. *Witness*: reached.
- `TT-22.b` — an observation and a mutation are serialized. *Witness*: reached.
*Cites*: [`task-tree-transactions-fail-closed`](../adr/task-tree-transactions-fail-closed.md).

**`TT-23` — a bulk mark validates before it moves, and converges.** A bulk mark
SHALL validate its whole plan against one snapshot before its first rename, and
re-running it after a partial application SHALL reach the same result.
*Obligations*:
- `TT-23.a` — the whole plan is validated against one snapshot before the first
  rename. *Witness*: a plan whose later member is invalid, refused before the
  first rename lands.
- `TT-23.b` — re-running after a partial application converges on the same
  result. *Witness*: a bulk mark interrupted mid-run, repaired by re-running it.
*Cites*: [`bulk-marks-are-not-atomic`](../adr/bulk-marks-are-not-atomic.md).

**`TT-24` — fail-closed ownership.** No action SHALL reset, merge, delete or
rewrite an entry it cannot prove is its own. Which non-mutating outcome it
produces is **not** the model's choice: it is fixed by how far the caller had
already gone, per [Outcomes](#outcomes).
*Class*: shared safety.

**The claim spans every action group; its obligations do not, and that is the
placement rule rather than a weakening.** *Action* is partitioned by
§[Actions](#actions) across Observation, Tree mutation, Finish, Lifecycle and
Environment, so it owns no single scope and an obligation quantified over it is
read over **exactly what its own scope admits**
([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md),
clause 4) — `SY-14`'s *no admitted action* idiom, applied here. A property this
broad is therefore one obligation per scope, each checked where its actions
live: `TT-24.a` below for the observation and tree-mutation groups, `FN-32` for
a live transaction's steps, `FN-21.c` for the quarantine reaper. **No obligation
states it over the Lifecycle group**, and that is a declared limit of the
catalogue rather than a covered claim.
*Obligations*:
- `TT-24.a` — no **admitted** action mutates an entry whose ownership it cannot
  prove. The admitted set is this scope's — the observation and tree-mutation
  groups, `initialise-root` included — and both families' commands are executed
  against it and no other. *Witness*: a mutation attempted against an unprovable
  entry, shown not taken.
- `TT-24.b` — an ordinary tree operation meeting a foreign entry at a reserved
  name returns `Refused(ReservedNameOccupied(entry))`, leaving the tree
  byte-identical and naming no recovery. *Witness*: reached.

**The other two contexts are `FN-`'s, and the letters `c` and `d` are retired.**
The claim's other two contexts — inside a live transaction, and under the
quarantine reaper — name `grove-finish`'s actions and outcomes, and
`grove-finish` depends on `grove-task-tree` rather than the reverse, so an
obligation stated over them is not one this crate can deliver
([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md)).
They are `FN-32` and `FN-21.c`, which cite this claim back. Neither letter is
ever reused.

**`TT-24.a` does not reach those contexts, and an earlier revision of this
paragraph said it did.** It said `TT-24.a` was quantified over *every* action and
so still reached both contexts wherever a model admits them. That is false of
what is checked: Alloy's `TT_24a` quantifies over the task-tree file's own
transitions and Quint's reads a history flag only that file's steps set, so a
green `TT-24.a` is a statement about the observation and tree-mutation groups
and about nothing else. It is also incompatible with the placement rule the same
revision landed — an obligation naming the Finish and Lifecycle groups names
scopes above `TT-`, which clause 1 forbids. The prefix-local reading is the one
that survives; what the shared-safety register means by `TT-24` is the **claim**,
which its three per-scope obligations deliver between them. **A scope above may
not cite `TT-24.a` as evidence about an action `TT-` does not admit** — the one
row that did (`crates/grove-finish/models/README.md`, Q4-6) now reads `none`.

**`TT-25` — a node is never marked.** Done-ness SHALL be derived from the absence
of a live leaf beneath it, and no action SHALL write a node's state.
*Obligations*:
- `TT-25.a` — a node whose subtree is wholly terminal is derived done, unmarked.
  *Witness*: reached.
- `TT-25.b` — a node with a live leaf beneath it is derived live, unmarked.
  *Witness*: reached.

## Claims — finish and recovery (`FN`)

Every claim below is checked under all three lanes.

**Every `FN-` claim carries a class**, because [what the models must be able to
decide](#what-the-models-must-be-able-to-decide) turns on it: a question asking
whether a mechanism should exist cannot be decided by the claims that *are* that
mechanism. The register is here, in one place, so it cannot drift claim by claim:

| class | claims |
|---|---|
| **shared safety** | `FN-01`–`FN-07`, `FN-13`–`FN-18`, `FN-20`, `FN-23`–`FN-30`, `FN-32`, and `TT-24` |
| **incumbent mechanics** | `FN-08`, `FN-09`, `FN-10`, `FN-11`, `FN-12`, `FN-19`, `FN-21`, `FN-22`, `FN-31` |

Where a **shared-safety** claim names a concrete artifact — the correlation
ticket, the manifest, the recorded anchor — the artifact is the *incumbent
realisation of a role*, exactly as the repository anchor's entry in
[Identities](#identities) already says. A candidate protocol satisfies such a
claim by supplying the role, not by keeping the artifact, and Q4's removal matrix
is where each artifact's role is made to state itself.

### Entry and intent

**`FN-01` — confirmation enables, and is never attested.** No step of the
transaction SHALL run without an operator confirmation, and the transaction SHALL
make no claim to have verified that one occurred. The deterministic guards it
*can* make — a live finish leaf, no live ordinary work — are separate and are
not a substitute.
*Obligations*:
- `FN-01.a` — no step runs without an operator confirmation, and the transaction
  makes no claim to have verified one. *Witness*: a transaction never entered for
  want of confirmation.
- `FN-01.b` — the deterministic guards are not a substitute for confirmation.
  *Witness*: a transaction refused for want of the deterministic guard, distinct
  from the previous.

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
*Obligations*:
- `FN-05.a` — the preflight precondition set is **closed and exactly** this:
  confirmation absent (`FN-01`); no live finish leaf, or live ordinary work
  present (`SY-07`, `TT-13`); layout unsupported (`SY-02`); the quarantine target
  unreachable from the transaction's own operands (`FN-08`); task-root identity
  unverified (`FN-06`); an empty deletion fingerprint (`FN-07`); an entry type
  that cannot be digested (`FN-12`). No other precondition exists, and a model
  reaching an eighth is a finding about this list.
  *Witness*: each of the seven, reached — the enumeration is what makes the
  remaining two obligations quantified over something finite.
- `FN-05.b` — each member, failing, leaves the **tree** byte-identical.
  *Witness*: each of the seven, with the tree unchanged.
- `FN-05.c` — each member, failing, leaves the **repository** byte-identical.
  *Witness*: each of the seven, with the repository unchanged.

**The seven are not distinguishable by their outcomes, and a family answering
`FN-05.a` needs an observable of its own.** The first member produces no refusal
at all — the transaction is simply never entered (`FN-01`) — and two of the
remaining six, an unsupported layout and an unreachable quarantine operand, are
the *same* [refusal reason](#outcomes): the closed set holds exactly one reason
about the workspace's shape, `LayoutUnsupported`, and `SY-03` makes the two gates
one question asked twice — so the reason names the question rather than the gate.
Six reasons
therefore cannot witness seven members. A model that introduces a
precondition-naming observable to reach them is doing what this obligation
requires and records it as an abstraction; a model that reports six witnesses and
calls the set covered has lost a member. Whether the shipped diagnostic should
distinguish the two gates is `handoff-audit-k66`'s and is not settled here.

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
*Obligations*:
- `FN-09.a` — publication is exactly one atomic same-directory rename, and no
  reader observes it half-applied. *Witness*: an interruption immediately after
  publication.
- `FN-09.b` — no preparing witness ever holds an evacuated entry. *Witness*: an
  interruption inside the build.

**`FN-10` — an unpublished witness is discardable.** Interruption before
publication SHALL be recoverable by discarding the witness, never by
interpreting its contents, and SHALL fail closed on any content it cannot
classify as its own.
*Obligations*:
- `FN-10.a` — interruption before publication is recovered by discarding the
  witness, never by interpreting its contents. *Witness*: a discard.
- `FN-10.b` — content the discard cannot classify as Grove's own fails closed.
  *Witness*: a refusal to discard unclassifiable content.

**`FN-11` — evacuation precedes deletion.** Every ordinary root entry SHALL be
inside the published witness, beneath a manifest that has been written and
verified, before any commit is attempted.
*Witness*: the interval between publication and commit, with the task root
present, unwalkable and holding every entry.

**`FN-12` — the manifest is complete and marked ready last.** It SHALL record the
finish handle, the attempt identity, the repository anchor, the deletion
fingerprint, and every evacuated entry's type and digest; an entry type it
cannot digest SHALL be refused before any mutation.
*Obligations*:
- `FN-12.a` — the manifest records the finish handle, the attempt identity, the
  repository anchor, the deletion fingerprint, and every evacuated entry's type
  and digest, and is marked ready last. *Witness*: a manifest interrupted before
  its ready mark, recovered as not ready.
- `FN-12.b` — an entry type that cannot be digested is refused before any
  mutation. *Witness*: a refused entry type.

**`FN-13` — the witness is never committed.** Every candidate committed tree
SHALL exclude the witness.
*Witness*: a commit attempted while the witness is tracked, **blocked
`RecoveryPending`**.

**This witness said *refused* and the word was wrong**, by
[§Outcomes](#outcomes)' own discriminator. The commit is attempted only after
publication and evacuation (`FN-11`), so the tree at that moment holds every
entry inside the witness and is not the tree the action was given; an effect
stands that the action can neither complete nor undo, which is a `Blocked`. The
diagnosis is `RecoveryPending` and not `OwnershipConflict`: the artifact holding
the transaction is this attempt's own published witness, correlated by this
finish handle and this attempt identity, which is `FN-25`'s first arm word for
word.

**Three readings now agree and the odd one out was this sentence.**
`crates/grove-finish/models/finish.qnt` blocks here;
[`task-tree-transactions-fail-closed`](../adr/task-tree-transactions-fail-closed.md)
says a tracked witness "keeps the witness unwalkable as **Recovery pending**";
and the shipped post-commit verification rejects a result that still tracks
`.grove/` with the evacuation already done
([`src/repo/finish_commit.rs`](../../src/repo/finish_commit.rs)).
`crates/grove-finish/models/finish.als` refused, "because the catalogue is the
sole input to the formal phase" — which was the right method against the wrong
sentence, and is why the correction lands here rather than in that model's
judgement.

**It follows that the closed reason set gains nothing for a tracked witness.** A
block carries a diagnosis, not a reason, so `finish.als`'s `W8WitnessTracked`
stays what it always was — a model-only observable naming which branch was
taken — and the operator-facing question it raised, *can the diagnostic say the
repository rather than the filesystem is what stopped this*, is a diagnostic
question and is `handoff-audit-k66`'s, beside the other four.

### Commit and disposition

**`FN-14` — the commit is scoped.** It SHALL record exactly the expected
deletions at their original paths and no unrelated change; unrelated working-copy
work SHALL survive.
*Witness*: unrelated modified work present across a successful finish.

**`FN-15` — disposition is classified from evidence, not from exit status.** The
classification SHALL be derived from the recorded anchor, the expected
fingerprint and the exact immediate result.
*Obligations*:
- `FN-15.a` — the classification never reads an exit status as a receipt.
  *Witness*: a lost or late result reported as failure while the exact commit
  exists — classified `Committed`.
- `FN-15.b` — `Committed` is reachable. *Witness*: reached, on each lane.
- `FN-15.c` — `NotCommitted` is reachable. *Witness*: reached, on each lane.
- `FN-15.d` — `Indeterminate` is **either** reachable, with a witness, **or**
  positively established unreachable within a stated bound by an exhaustive
  check — an Alloy `check` over the full scope that no state carries the
  disposition, or a Quint exhaustive run to the same depth. A witness that merely
  fails to land satisfies neither branch and is a `defer`.
  *Witness*: reached on each lane, **or** the unreachability check's bound and
  result recorded per lane.
*Decides*: [`TODO.finish_process.md`](../../TODO.finish_process.md) Q2, jointly
with `FN-25`. What the unreachability branch can decide is bounded by its own
bound and never more.

**`FN-16` — rollback is licensed only by proof.** Restoration SHALL require the
recorded anchor to still hold **and** the attempt-bound result to be absent.
*Obligations*:
- `FN-16.a` — restoration is refused when the recorded anchor no longer holds.
  *Witness*: reached.
- `FN-16.b` — restoration is refused when the attempt-bound result is present.
  *Witness*: reached.

**Both letters say *refused* of a step declining, not of the action returning
`Refused(r)`**, and the distinction is [§Outcomes](#outcomes)'. A restoration
declined leaves the witness and the evacuated tree exactly where they were, so
the *action* blocks — `FN-17.b` says so three lines below, and `FN-22`'s table
gives the diagnosis. Both families read it that way without being told
(`witness_FN_16a_…` in `crates/grove-finish/models/finish.als` requires
`Sys.res = BlockedOutcome`), which is the same neighbouring-sentence effect that
kept them together here and let them diverge at `FN-13`, where no such sentence
stood. Nothing about either obligation changes; only what the word was doing.

**`FN-17` — rollback is exact.** After restoration the tree SHALL match the
manifest, and on a working-copy-as-commit lane the exact recorded preflight
commit SHALL be reproduced before the witness is removed.
*Obligations*:
- `FN-17.a` — after restoration the tree matches the manifest, and on a
  working-copy-as-commit lane the exact recorded preflight commit is reproduced
  before the witness is removed. *Witness*: a restoration that reproduces it.
- `FN-17.b` — a restoration that cannot reproduce it blocks rather than
  proceeds. *Witness*: reached.

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

**`FN-20` — a leftover artifact is garbage, never a receipt.** No classification
SHALL read the quarantine, or any other artifact the transaction owns, as
evidence that a finish happened; only the correlation ticket is that evidence
(`FN-03`). The quarantine is the incumbent realisation, and the role — *no
artifact a transaction leaves behind is a receipt for it* — is what a candidate
protocol must supply, whatever it leaves behind instead.
*Witness*: a leftover artifact present while the tree is classified fresh.
*Class*: shared safety, stated over the role rather than over the quarantine, so
Q1 can be decided against it.

**`FN-21` — disposal is resumable and bounded to Grove's own.** Disposal SHALL be
re-enterable from any interruption, and a reaper SHALL touch only entries
carrying Grove's own cleanup manifest, and only when no matching in-tree witness
owns them.
*Obligations*:
- `FN-21.a` — disposal is re-enterable from any interruption. *Witness*: a
  resumed disposal.
- `FN-21.b` — a reaper touches only entries carrying Grove's own cleanup
  manifest. *Witness*: a reaper declining an entry whose in-tree witness still
  owns it.
- `FN-21.c` — a reaper declines a foreign entry at a reserved name, mutating
  nothing (`TT-24.a`). *Witness*: reached. **This obligation is where the
  sweep's context landed**: it was also `TT-24.d`, which is retired, and the
  sweep is `grove-finish`'s action.
*Class*: incumbent mechanics — this claim is what Q1 asks about, so it is not
evidence about a candidate protocol.

**`FN-22` — the disposition is revalidated across every handoff.** There are
exactly **two** filesystem handoffs — the **restoration** and the **quarantine
rename** — and the disposition SHALL be rechecked immediately before and after
each, giving four revalidation points. Every observed disposition at every point
SHALL have a stated corrective action and a stated stable state, **including the
observations that settle successfully**: a corrective return that completes is
not an absence of a result.

| point | observed | corrective action | stable state after a **successful** action | outcome |
|---|---|---|---|---|
| before restoration | `NotCommitted` | proceed with the restoration | — (continues) | — |
| before restoration | `Committed` | do not restore; take the forward path (`FN-18`) | — (continues to the quarantine rename) | — |
| after restoration | `NotCommitted` (unchanged) | complete: remove the witness | task root `Current(*)`, matching the manifest, finish leaf live | `Refused` |
| after restoration | `Committed` | leave the witness blocking the restored tree | `Reserved(Published)` carrying `RecoveryPending` | `Blocked` |
| before quarantine rename | `Committed` | proceed with the rename | — (continues) | — |
| before quarantine rename | `NotCommitted` | do not rename; take the restoration path | — (continues to the restoration) | — |
| after quarantine rename | `Committed` (unchanged) | complete: dispose (`FN-21`) | task root `Absent`, quarantine holding the root | `Applied` |
| after quarantine rename | `NotCommitted` | return the quarantine atomically | `Reserved(Published)`, disposition `NotCommitted` — the exact pre-rename state, from which the restoration path runs | `Refused` |
| after quarantine rename | `Indeterminate` | return the quarantine atomically | `Reserved(Published)` carrying `RecoveryPending` | `Blocked` |
| any point | `Indeterminate` (except the row above) | no handoff is performed | `Reserved(Published)` carrying `RecoveryPending` | `Blocked` |
| after quarantine rename | any change, return cannot complete | report both the change and the quarantine | `Reserved(Published)` **and** a quarantine, both named in the diagnostic | `Blocked` |

The two rows the shipped material never distinguished are the last two `Committed`
departures, and they are not the same event: **`Committed -> NotCommitted`** is a
rollback that succeeds — the quarantine returns, the tree is restored, and the
attempt ends as a complete refusal with the finish leaf live (`FN-29`) — while
**`Committed -> Indeterminate`** cannot be settled either way and ends as a block.
Collapsing them would let a block be reported as a refusal, which is exactly the
distinction `FN-29` requires the operator to be able to make.

*Obligations*:
- `FN-22.a` — all four revalidation points are performed, and none is skipped.
  *Witness*: each of the four, reached.
- `FN-22.b` — before restoration, `Committed` diverts to the forward path and
  restores nothing. *Witness*: reached.
- `FN-22.c` — after restoration, `Committed` leaves the witness blocking the
  restored tree, `Blocked(RecoveryPending)`. *Witness*: reached.
- `FN-22.d` — after restoration, an unchanged `NotCommitted` settles to a
  manifest-matching `Current(*)` root with the witness removed and the finish leaf
  live, `Refused`. *Witness*: reached.
- `FN-22.e` — before the quarantine rename, `NotCommitted` diverts to the
  restoration path and renames nothing. *Witness*: reached.
- `FN-22.f` — after the quarantine rename, `Committed -> NotCommitted` returns the
  quarantine atomically, and a **successful** return settles to the exact
  pre-rename `Reserved(Published)` state from which the attempt completes as
  `Refused`. *Witness*: reached, with the returned tree byte-equal to the
  pre-rename tree.
- `FN-22.g` — after the quarantine rename, `Committed -> Indeterminate` returns
  the quarantine atomically, and a **successful** return settles to
  `Reserved(Published)` carrying `RecoveryPending`, `Blocked`. *Witness*: reached.
- `FN-22.h` — a return that cannot complete reports both the change and the
  quarantine and blocks. *Witness*: reached, with both named.
- `FN-22.i` — after the quarantine rename, an unchanged `Committed` settles to an
  absent task root with the quarantine holding it, `Applied`. *Witness*: reached.
- `FN-22.j` — `Indeterminate` observed at any point other than after the
  quarantine rename performs no handoff and blocks `RecoveryPending`. *Witness*:
  reached at each remaining point.
*Class*: incumbent mechanics.

**`FN-31` — the cleanup marker's replace transition exists and is reachable.**
Disposal SHALL have a `replace-cleanup-marker` transition distinct from creating
a marker and from removing one, and the models SHALL decide by reachability
rather than by construction whether it is needed.
*Obligations*:
- `FN-31.a` — there is a reachable source state from which disposal must
  *replace* an existing cleanup marker rather than create or remove one.
  *Witness*: that source state, with the marker present, owned by Grove, and
  carrying a value the next disposal step must supersede — **or**, if no such
  state is reachable, the bounded-unreachability check of `FN-15.d`'s form
  recorded at its bound, which is what makes Q3's *delete* answer evidence rather
  than an absent witness.
- `FN-31.b` — the replacement is atomic with respect to readers: no reader
  observes the marker absent, nor observes two markers. *Witness*: an observation
  interleaved with the replacement.
- `FN-31.c` — an interruption inside the replacement is resumable, and resumption
  reaches the same terminal state as an uninterrupted replacement (`FN-21.a`,
  `FN-23`). *Witness*: an interruption at each step of the replacement, resumed.
- `FN-31.d` — a replacement is never performed against a marker Grove cannot
  prove is its own (`TT-24.a`). *Witness*: a foreign marker, declined.
*Class*: incumbent mechanics.
*Decides*: [`TODO.finish_process.md`](../../TODO.finish_process.md) Q3.

### Recovery, refusal and the exits

**`FN-23` — recovery is idempotent.** Re-running recovery for the same handle
and attempt SHALL reach the same terminal state, and SHALL make no further change
once it has.
*Witness*: three consecutive recoveries, the second and third changing nothing.

**`FN-24` — every interruption lands in exactly one stable state.** From a crash
between any two steps of the transaction, the next invocation SHALL classify the
result into exactly one stable state, and never into a state that is
indistinguishable from a different one.

**Two premises this claim rests on, stated rather than smuggled.** "Between any
two steps" is only as strong as the step list, and `EN-08` grants interruption
between steps without saying what a step is. So: (i) the model's step list is a
**complete** set of crash boundaries for the transaction — every point at which a
persistent effect becomes visible is a step boundary in the model; and (ii) every
persistent effect **inside** one step is either atomic by `EN-01`, which grants
atomicity to a same-directory rename and to nothing else, or is decomposed into
steps of its own. A step whose persistent effect is neither is a modelling defect
and `FN-24.b` is what catches it. Neither premise is checkable by `FN-24.a`
alone, which is why it is not the whole claim.

*Obligations*:
- `FN-24.a` — from a crash at each step boundary, the next invocation classifies
  the result into exactly one stable state. *Witness*: the full interruption
  sequence, one crash point per step.
- `FN-24.b` — every step of the transaction has at most one persistent effect,
  and that effect is a same-directory rename (`EN-01`) or is itself decomposed.
  A step that is neither is declared, with what it would take to decompose it.
  *Witness*: the step list, enumerated, each step's persistent effect named.
*Class*: shared safety — this is the claim a candidate protocol is judged
against, and it names no artifact of the incumbent one.
*Decides*: [`TODO.finish_process.md`](../../TODO.finish_process.md) Q1 — a cheaper
protocol is admissible only if both obligations still hold under it, with
`FN-24.a`'s witnesses reached at a bound no greater than the incumbent's.

**`FN-25` — a block is exactly one of the two diagnoses.** Every `Blocked`
outcome SHALL carry exactly one of `RecoveryPending` and `OwnershipConflict`.
The partition is over **`Blocked` outcomes and nothing else**: a refusal is not a
block and carries a refusal reason instead, so a `Refused(ReservedNameOccupied)`
is outside this claim entirely and reading `OwnershipConflict` onto it would make
the partition neither disjoint nor exhaustive.
*Obligations*:
- `FN-25.a` — the two diagnoses are **disjoint**: no blocked state satisfies
  both. *Witness*: a state that nearly does — a Grove-owned, correlated artifact
  sitting at a name Grove also reserves — resolved to exactly one.
- `FN-25.b` — the two are **jointly exhaustive** over `Blocked`: no blocked state
  carries neither. *Witness*: an exhaustive sweep of the blocked states within
  the bound.
- `FN-25.c` — each diagnosis is reachable on **each** lane. *Witness*: each
  diagnosis, on each lane.
*Class*: shared safety.
*Decides*: [`TODO.finish_process.md`](../../TODO.finish_process.md) Q2, jointly
with `FN-15`.

**`FN-26` — history is never rewritten to clear a block.** A block SHALL stay
blocked and operator-restorable, naming the artifact holding the transaction,
the recorded and observed topology, and the two restorable exits.
*Witness*: a block whose diagnostic carries all four, and no trace in which
recorded history changes.

**`FN-27` — nothing unrelated is mutated, on any outcome.** Nothing outside the
task root, the reserved witness, the quarantine and the scoped commit SHALL
change — on success, on refusal, and on a block alike.
*Obligations*:
- `FN-27.a` — nothing unrelated changes on success. *Witness*: reached.
- `FN-27.b` — nothing unrelated changes on refusal. *Witness*: reached.
- `FN-27.c` — nothing unrelated changes on a block. *Witness*: reached.
*Class*: shared safety.

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
distinguishable by the operator from a block; and no action SHALL return
`Refused` while an effect of that action still stands.
*Obligations*:
- `FN-29.a` — `NotCommitted` leaves the grove exactly as it was, with the finish
  leaf live and selectable, and is distinguishable by the operator from a block.
  *Witness*: a refused attempt followed by a successful one.
- `FN-29.b` — every `Refused` outcome is returned with the tree byte-equal to the
  tree that action received, and an effect that stands and can be neither
  completed nor undone is `Blocked`. *Witness*: **both arms of the
  discriminator** — a stop that has applied nothing, refused; and a stop after an
  applied effect, blocked. The third state a refusal can be returned from — an
  effect applied and then **undone** — is `FN-29.a`'s witness, and the two
  obligations therefore cover the three between them rather than each covering
  part of one.

**`.b` is [§Outcomes](#outcomes)' discriminator, stated once where something can
check it, and it exists because nothing could.** Two independently built
families answered `FN-13` in opposite directions and **both stayed green**;
`FN-22`'s ten-row table applies the rule row by row and never names it. A rule
carried only by prose is a rule a model can contradict without a counterexample,
which is the failure this obligation removes.

**It is `grove-finish`'s alone, and that is placement rather than convenience**
([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md)).
`Blocked` is produced by the finish and recovery protocol and by nothing else —
§[Outcomes](#outcomes) scopes it to a transaction, and `FN-25`'s two diagnoses
are both about finish ownership. The task-tree scope has no block to be
distinguished from, and `models/system/` reads *blocked* as a mark already made
(`SY-13`, `SY-14`). So the claim stated over *every action* becomes **one**
obligation here rather than one per scope, because only one scope can execute
its context.
*Class*: shared safety. The discriminator is a property any admissible protocol
must have, not a fact about the incumbent's steps.

**`FN-30` — internal commits run without operator hooks.** No user-supplied hook
SHALL run during an internal commit, because such a hook may mutate unrelated
working-tree bytes that no index image restores.
*Witness*: a hook that would have run, shown suppressed.

**`FN-32` — a transaction never mutates an artifact it cannot prove is its
own.** While a finish or recovery transaction is live, an artifact sitting at a
name Grove reserves and carrying no proof that it is **this attempt's** SHALL be
left byte-identical by every step of the transaction, whatever outcome that step
produces.
*Class*: shared safety.
*Witness*: such an artifact, met by a transaction step, unchanged across it.

**Which reserved names those are is a fact about the protocol, not a modelling
choice.** A witness slot with no owner and a cleanup marker with no owner each
carry an ownership bit that a transaction step can read. The **quarantine name
does not**: a quarantine no cleanup marker yet authorises is a state the ordinary
forward path passes through between the root rename and the marker's creation, so
its presence proves nothing either way. Its own case — a quarantine met when no
transaction is live — is the reaper's, `FN-21.c`.

**This is `TT-24`'s second context, re-stated where the crate that delivers it
lives** ([`obligations-follow-context-not-artifact`](../adr/obligations-follow-context-not-artifact.md)).
It is stated as its own claim rather than left to the two obligations that carry
it today, and the reason is the class register: `FN-10.b` and `FN-31.d` are both
**incumbent mechanics**, so neither is evidence about a candidate protocol, while
fail-closed ownership inside a transaction is a property any admissible protocol
must have. It says nothing about **which** outcome the step produces, and
that is deliberate rather than unfinished: the outcome is a function of what the
action has left standing, not a constant, so [Outcomes](#outcomes)' *one
artifact, three contexts* decides it per step against `FN-29.b`. `FN-32` is the
conjunct every step shares whatever its outcome, which is why it stayed
separable from the outcome question and survived it.

**Q1's retained list does not yet name it, and extending that list is
`finish-verdicts-k65`'s.** `FN-32` is shared safety, so a candidate protocol must
supply it; whether Q1's own row gains it as a named retained claim is a change to
that question's evidence criteria and belongs to the child that answers Q1.

**`FN-32` is deliberately narrower than the reaper's `FN-21.c`, and the two must
stay separable.** `FN-21.c`'s subject is a sweep with no transaction live;
`FN-32`'s is a transaction step. A model that discharges both from one predicate
has made each other's mutations unable to kill anything, which is the hazard
`crates/grove-finish/models/finish.als` records beside `foreignAtReservedName`.

## Claims — system lifecycle (`SY`)

These are the joint. They are stated over sessions, exhaustion, finish,
interruption and recovery together, and they are what `models/system/` owns.

**`SY-01` — one live driver per working tree.** A second driver SHALL be refused
immediately rather than queued, and ownership SHALL be released by process death
as ordinarily as by return.
*Obligations*:
- `SY-01.a` — a second driver is refused immediately, never queued. *Witness*:
  reached.
- `SY-01.b` — ownership is released by process death as ordinarily as by return.
  *Witness*: a crashed driver whose successor proceeds.
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
*Obligations*:
- `SY-04.a` — at most one lifecycle transition occurs per iteration. *Witness*:
  each transition, taken alone.
- `SY-04.b` — full configuration validation precedes every transition, so an
  invalid configuration leaves the working tree byte-identical. *Witness*:
  reached.

The refusal is `Refused(ConfigurationInvalid)`. Both families had to name this
outcome and neither could from the closed set; the set now carries it
([Outcomes](#outcomes)).

**`SY-05` — task-root absence is the complete fresh-tree discriminator.** A
missing task root SHALL mean *start a new grove*, and SHALL never be read as
evidence about an earlier one. This inference is sound only because `FN-11`
and `FN-19` never expose an absent task root before the deletion is proven, and
the two claims SHALL be checked together.
*Obligations*:
- `SY-05.a` — a missing task root means *start a new grove* and is never read as
  evidence about an earlier one. *Witness*: a completed teardown whose driver
  never observed the signal, followed by a fresh scaffold.
- `SY-05.b` — no trace exposes an absent task root before the deletion is proven
  (`FN-11`, `FN-19`), so the inference in `SY-05.a` is sound. *Witness*: the
  exhaustive absence of such a trace within the bound.
  *Cross-scope citation*: the observation is `models/system/`'s and is stated
  over its own transitions; the two **steps** underneath are `FN-11`'s and
  `FN-19`'s and are answered in `crates/grove-finish/models/`. An
  `FN_`-prefixed command in `models/system/` would be a placement error, so
  *checked together* means each half is checked where its subject lives, not
  that one directory carries both.

**`SY-06` — a fresh root carries a first live leaf.** Scaffolding SHALL produce
work, not only a charter, so a fresh grove is never indistinguishable from a
finished one.
*Obligations*:
- `SY-06.a` — a completed scaffold carries a first live leaf, not only a charter.
  *Witness*: a fresh root, distinguishable from a spent one.
- `SY-06.b` — an interrupted scaffold classifies as `PartialScaffold` — by the
  exact known subset in [States](#states), never by the mere absence of the format
  witness — and is completed **before** any format classification runs, so a
  `Legacy` tree is never completed as though Grove had scaffolded it. *Witness*:
  an interrupted scaffold, completed; and a `Legacy` tree, refused rather than
  completed.
  *Cross-scope citation*: the classification, its exact subset and the **order**
  that puts `PartialScaffold` before `Legacy` are `TT-18`'s and `TT-20`'s, and
  `models/system/` has no classification step — it reads `partial` and `legacy`
  as marks already made. What is checked here is the ordering's consequence.
  **The citation carries its narrowing**: `TT-20`'s fourth conjunct is checked
  over an initialisation the world did not touch, declared in both families'
  READMEs, so this obligation inherits that narrowing and no more strength than
  it.

**`SY-07` — exhaustion yields exactly one finish leaf.** When no live leaf
remains the driver SHALL append or reuse exactly one driver-owned finish leaf,
and no session SHALL create one.
*Obligations*:
- `SY-07.a` — on exhaustion the driver appends or reuses exactly one
  driver-owned finish leaf. *Witness*: an append; a reuse.
- `SY-07.b` — no session creates one. *Witness*: a refused creation.

**`SY-08` — selection is authoritative once per iteration.** The driver SHALL
select exactly once per iteration and SHALL not recompute before launching, so a
leaf added during the launch window becomes the next iteration's work rather than
preempting the running one.
*Witness*: a leaf inserted during the launch window.

**`SY-09` — a session ends in exactly one of three ways.** Relaunch, done, or no
signal. No signal SHALL stop the loop, and SHALL never be inferred as done — not
even when that session committed a teardown.
*Obligations*:
- `SY-09.a` — **relaunch**: the loop continues with the next iteration.
  *Witness*: reached.
- `SY-09.b` — **done**: the loop ends. *Witness*: reached.
- `SY-09.c` — **no signal**: the loop stops, and is never inferred as done — not
  even when that session committed a teardown. *Witness*: reached, with a proven
  teardown.

**`SY-10` — a stale session cannot act.** An ambient operation SHALL match the
live launch generation before it may touch the tree, and a contended generation
SHALL time out into a visible stop rather than a silent park.
*Obligations*:
- `SY-10.a` — an ambient operation matching no live launch generation is refused
  before it touches the tree. *Witness*: a stale session refused.
- `SY-10.b` — a contended generation times out into a visible stop, never a
  silent park. *Witness*: a timeout reported.

**The visible stop is `Refused(GenerationContended)`, and the closed *outcome*
set gains nothing.** `models/system/lifecycle.als` introduced a seventh outcome
for it, `Stopped`, on the ground that it is neither a `Refused` nor a `Blocked`.
Half of that was right: it is not a `Blocked`, because §[Outcomes](#outcomes)
scopes blocks to an effect that stands and `FN-25`'s two diagnoses are both about
finish ownership. The other half rested on the reason set being closed against
it, which is a fact about the *reason* set and is now fixed there —
`models/system/lifecycle.qnt` reached that placement independently and is the
column that had it right.

**A word collision is what made the wider reading look necessary, and it is
worth naming because it cost a proposed widening of the most load-bearing closed
set in the catalogue.**
[`one-live-driver-per-working-tree`](../adr/one-live-driver-per-working-tree.md)
says the driver "stops `blocked`" on a post-reap invalidation timeout, and that
`blocked` is not this catalogue's `Blocked(b)` — it describes the *epoch
invalidation* being blocked. The shipped path is
`complete_post_reap_epoch_handoff` in
[`src/loop_driver.rs`](../../src/loop_driver.rs): it returns an error, the loop
stops, the completion signal is left unconsumed, and — per that ADR — "a timeout
performs no tree access or epoch rewrite". Nothing stands, so it is a refusal by
the discriminator, and no tree is left carrying a diagnosis for an operator to
recover.

**`SY-11` — the guard order admits no cycle.** Lease, then launch generation,
then tree; and no path SHALL wait for a generation while holding a tree guard.
*Obligations*:
- `SY-11.a` — every path takes lease, then launch generation, then tree, in that
  order. *Witness*: the full order, reached.
- `SY-11.b` — no path waits for a generation while holding a tree guard, and no
  cycle exists within the bound. *Witness*: the exhaustive absence of a cycle,
  with its bound.

**`SY-12` — restart is ordinary continuation.** From a crash at any lifecycle
point, the next invocation SHALL reach a stable state and either make progress or
refuse; it SHALL never silently repeat a completed effect.
*Witness*: one crash point per lifecycle step.

**`SY-13` — no stable state is a sink.** From any stable state there SHALL
**exist** a bounded sequence of admitted actions reaching either a live leaf to
run or a terminal disposition.

**This is existential reachability, and deliberately not a liveness property.**
Stating it as "the loop *will* reach one" would need a fairness or admission
premise the models have no grounds to grant — nothing here schedules the
operator, and `EN-15` says Grove cannot even verify a confirmation. What the
claim says instead is that no stable state is a dead end, which is checkable
without fairness and is the property the tree actually needs.

**`terminal disposition`** means an ending from which the loop has no further
admitted action of its own. There are exactly two, and a **block is one of
them**: `SY-14` says no admitted action clears a block, so the only exit is an
operator action, and operator actions are outside the admitted set by
construction. The two are therefore:

- a **proven successful finish** — the exact attempt-bound commit proven and the
  task root `Absent` (`FN-28`); and
- a **blocked tree** — any `Blocked` outcome's stable state, carrying exactly one
  diagnosis (`FN-25`) and two operator-restorable exits (`FN-26`).

A `Malformed(reason)` tree is **not** terminal for this property: it is a refusal
state that a hand edit reaches and a hand edit leaves, and folding it in would
let the claim be satisfied by a tree nobody can act on.

*Obligations*:
- `SY-13.a` — from every stable state, some bounded sequence of admitted actions
  reaches a live leaf or a terminal disposition. *Witness*: the longest such
  sequence within the bound, and its length.
- `SY-13.b` — no stable state within the bound is a sink. *Witness*: the
  exhaustive sweep of the stable states, with its bound.

**`SY-14` — a blocked tree stays blocked until an operator acts.** No admitted
action SHALL clear a block, and every action on a blocked tree SHALL refuse
naming it.
*Cross-scope citation*: *until an operator acts* names the two restorable exits,
which are `FN-26`'s and are answered in `crates/grove-finish/models/`. Operator
actions are outside the admitted set by construction (§[Actions](#actions),
`EN-15`), so at this scope the phrase is exactly *never, by anything the
lifecycle model has*. That is a **limit of the catalogue rather than a gap in a
scope**: the checkable halves are *no admitted action clears a block* (here) and
*the block names both exits* (`FN-26`), and no model claims the operator's own
act. `models/system/README.md` argued this and the argument is accepted; what it
lacked was the class and the citation.
*Obligations*:
- `SY-14.a` — no admitted action clears a block. *Witness*: an exhaustive sweep
  of the action set against a blocked tree.
- `SY-14.b` — every action on a blocked tree refuses, naming the block.
  *Witness*: the same sweep, each refusal naming it.

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
| the marker-replacement protocol's own byte layout | a resumable disposal step **with an explicit `replace-cleanup-marker` transition**, required by `FN-31` | Q3 asks whether replacement is reachable at all, so the transition must exist for the model to answer; only its encoding is omitted |
| power loss, kernel failure, storage-cache loss | not represented | `EN-08` — outside the contract, and representing them would make every claim false without making any of them wrong |
| the methodology corpus and session conduct | a session is a step that returns one of three endings | conduct is checked by delivery assertions, not by these models |

The sixth row is the one to watch. Abstracting the replace step *away* would
answer Q3 by construction, which is the shape of a false-confidence incident
rather than a finding — and a prose row is not an instrument, because the runner
checks obligations rather than reading this table. `FN-31` is what makes the row
enforceable: the transition is in the [action vocabulary](#actions), its
reachability is `FN-31.a`, and a family that omits it fails coverage.

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
`handoff-audit-k66`, once the models have shown where the boundaries actually
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
