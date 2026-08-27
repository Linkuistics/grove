# routing-and-prose-k73


## Goal

Land every disposition this node owns that costs no model work, and re-point
every inherited finding this node does **not** own to the sibling or model owner
that does — so that `finish-verdicts-k65` and `handoff-audit-k66` open on a tree
that already names them.

## Context

**The manifest is the boundary of this leaf.** `models/run.sh` reads its
obligation list out of `docs/specs/semantic-contract.md`, so every edit here must
leave `models/run.sh --list` printing the same count. An edit that moves it is by
definition `closed-set-additions-k74`'s and not this leaf's.

**Decided here:** items 8 (which Lifecycle actions are gated on the root's
classification), 19 (`EN-11` does not gate `TT-24.b`) and 25 (the runner has four
obligations, and the catalogue said three).

**Routed here:** items 26 – 36. Item 27 is already decided by
[`root-lifecycle-stays-with-its-receipt`](../../../../../docs/adr/root-lifecycle-stays-with-its-receipt.md);
the rest are re-pointed rather than answered.

**`docs/formalism-findings.md` is a log**, so it is appended to and never
rewritten: a disposition is recorded *beside* the finding, and the finding's own
prose stays byte-faithful because it was true when it was written.

## Done when

- Items 8, 19 and 25 are landed in `docs/specs/semantic-contract.md`, each
  verified manifest-neutral by `models/run.sh --list` before and after.
- Items 26 – 36 name their real owner in every live artifact — the catalogue,
  the four model files, the three model READMEs, `models/README.md` and the ADR
  set — and carry an appended disposition note in the findings log.
- `models/run-controls.sh` passes, which is the positive control that the
  catalogue edits did not break the manifest the runner parses out of it.

## Notes

Every decision this leaf settled is in its running log below, appended as each
one settled rather than reconstructed at the end.

## Decisions (running log)

### The manifest-neutral half is landed, and all three were verified neutral rather than assumed

`models/run.sh --list` printed `-- 128 obligations in scope`, exit 0, **before
and after** each of the three edits below. That is the check that a
"manifest-neutral" classification is a fact rather than an intention: the runner
reads its obligation list out of the catalogue, so an edit that accidentally
added, removed or re-shaped an obligation would move the count.

### 25 — the runner has four obligations, and the catalogue said three (manifest-neutral)

Landed in *`models/run.sh` is the one repository runner*. The fourth is Q4's
removal matrix, which the catalogue already states under *Q4 needs a matrix, not
a claim* (*a removable artifact with no row fails the run*) and which
`models/run.sh`'s own header numbers as 4. The section now states it as an
obligation of the runner and carries the rider the runner carries — that it
**rides with coverage assertion and has no flag of its own**, because a matrix
is owed only once a family's column has closed and `--no-coverage` already says
it has not.

**Why this was worth more than a numeral.** The catalogue is the source of truth
for what the runner owes, and obligation 4 is the one that makes the *ten
removable artifacts* a catalogue list rather than a README list. A section that
numbers three while the runner numbers four is a document that has stopped being
the manifest for one of its own obligations.

### 19 — `EN-11` does not gate `TT-24.b`, and the row has now been wrong three times (manifest-neutral)

`TT-24.b` is removed from `EN-11`'s controls column and the removal is annotated
in place, exactly as the same row already annotates `TT-16`. The evidence is a
**fired control**, not a reading: `wit_finding_EN_11_does_not_gate_TT_24b`
reaches `TT-24.b`'s witness in ~2% of traces with `hand-edit` gone, because
`EN-13` grants foreign entries at **any** name and `foreign-write` alone supplies
one. `TT-24.b`'s dependency is `EN-13`, where it is already listed — so the
correction removes a duplicate rather than orphaning a control.

**The generalisation is the disposition, and it is what the task file meant by
"worth more than the one-word fix".** The enumeration found a **third** instance
of one shape, and finding it is why the enumeration was run rather than the task
file's class list worked from: `crates/grove-finish/models/README.md` records
that `EN-08` names `FN-31.c` among the witnesses that become unreachable when
`crash` is removed, and that `FN-31.c`'s two land anyway. **I had classified that
site as prose and it is not** — it is the same defect in a different row.

So the catalogue now states the rule the three instances imply: **an
exercise-removal row's controls column is a claim of unreachability, and it
SHALL be established by running the removal rather than by reading the witness.**
It also names the two different failures the shape produces, because they have
different repairs and conflating them would fix the wrong artifact:

- **the row is wrong** — the witness has another route and never needed the
  assumption (`TT-16`, `TT-24.b`); correct the column and no claim changes;
- **the row is right and a family does not meet it** — the witness *posits* the
  state rather than reaching it, so removing the action leaves it landing
  (`FN-31.c`); a fact about that family's realisation, not about the assumption.

*A posited state and a reached one are not interchangeable for an assumption's
control*, however interchangeable they are for the claim the witness serves.
Telling the two apart is what running the removal buys and nothing else does.

**The three-instance claim was checked per family rather than asserted**, because
"found by both families" is the part that makes it a rule about the table rather
than about one column's habits. Alloy found two — `TT-16` via
`crates/grove-task-tree/models/task-tree.als`'s
`witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit`, and `FN-31.c` in
the `finish.als` section of `crates/grove-finish/models/README.md`. Quint found
one — `TT-24.b` via `crates/grove-task-tree/models/task-tree-controls.qnt`'s
`wit_finding_EN_11_does_not_gate_TT_24b`. Two rows, both families, three
instances.

**`FN-31.c` is not repaired here.** It is `finish-scope-k71`'s: the row now
states what it owes, and meeting it is finish-column model work.

### 8 — only `launch` is gated on the root's classification (manifest-neutral)

Landed in §*Actions*. The other six Lifecycle actions — `acquire-lease`,
`layout-preflight`, `open-epoch`, `reap`, `close-epoch`, `release-lease` — read
and write no task tree.

**The product half was re-verified here rather than inherited**, because this
leaf states it in a durable artifact and entry 048 stated it in a log. The
shipped reap path is `complete_post_reap_epoch_handoff` in `src/loop_driver.rs`,
whose three operands are the launch result, `invalidate_session_epoch()` and
`complete::read_signal()`; nothing on that path opens the task tree or
classifies a root. The catalogue now cites the function rather than the file, so
a later reader can check the claim in one hop.

**Neutral because both families already implement it and only the catalogue was
silent.** Verified rather than inherited: Quint's `touchesTree` now excludes
`ATAcquireLease`, `ATValidateConfig`, `ATOpenEpoch`, `ATCloseEpoch`,
`ATReleaseLease` and `ATReap`, and Alloy's `doAcquireLease`, `doOpenEpoch`,
`doCloseEpoch`, `doReleaseLease` and `doReap` are each `treeSame` with no root
classification in any guard. So no cell moves and no witness changes.

**The disposition is wider than entry 046 asked for, deliberately.** That entry
owed "a sentence saying that `reap`, `close-epoch` and `release-lease` read no
tree" — three of the seven. Stating three leaves the other four still silent, and
the silence is the defect rather than the three names: the catalogue is a closed
enumeration everywhere else, and a partial answer here would have to be extended
by the next model that guessed. The table now settles all seven.

**`launch` is the one that is gated, and it is gated in both families for the
same reason** — it consumes a selection, and `select` is an Observation action
that does read the tree. Alloy makes that explicit (`doLaunch` requires
`some p.sel`); Quint routes `ATLaunch` through `outcomeOn`. So the gate is the
tree read arriving one step earlier, not a second gate, and the two columns
agree despite looking different.

**Why the silence was load-bearing, recorded in the catalogue rather than only
here.** A gated `reap` makes `SY-05.a` **unwitnessable rather than false** — and
an unwitnessable claim is the failure mode a suite without a witness obligation
reports as green. That is the most transferable thing on this item, so it is
stated where a later reader of the `Actions` table meets it.

### One thing noticed and deliberately not decided here

The catalogue's Lifecycle group names `layout-preflight`; Quint models it as
`ATValidateConfig`, and `layout-preflight` appears **exactly once** in the whole
catalogue — in that table. Whether the two are one action, and whether the
catalogue owes a `validate-config` beside the preflight, sits directly against
items 15 – 17 (all `SY-04.b`) and is therefore `lifecycle-scope-k72`'s. Written
into that leaf's body rather than absorbed.

### The routing landed: 62 sites moved from a node to a named owner

Every one of items 26 – 36 is re-pointed, in the live artifacts rather than only
in the log. `docs/specs/semantic-contract.md` (4 sites), `models/README.md` (1),
`crates/grove-task-tree/models/{README.md,task-tree.qnt}` (5),
`crates/grove-finish/models/{README.md,finish.als}` (28),
`models/system/{README.md,lifecycle.als,lifecycle.qnt}` (22) and
`docs/adr/root-lifecycle-stays-with-its-receipt.md` (2) now name the leaf that
owes each: `finish-verdicts-k65`, `handoff-audit-k66`, `finish-scope-k71`,
`task-tree-scope-k70`, `lifecycle-scope-k72`, `closed-set-additions-k74`, or the
model owners.

**One re-route was a correction rather than a transcription.** The node brief
sent item 28 — whether the catalogue gains a **general** form of *once the caller
grades an effect applied it never ungrades it* — to `finish-verdicts-k65`,
because it arrives through
[`root-lifecycle-stays-with-its-receipt`](../../../../../docs/adr/root-lifecycle-stays-with-its-receipt.md)
beside the ordinal successor question. Landing it showed the two are separable:
the ordinal verdict is a keep/defer/reject call, while **gaining the general form
adds an `FN-` claim and therefore a cell in both families**, which is scope work.
It is routed to `finish-scope-k71`, the ADR now says why the two do not travel
together, and the node brief and `finish-scope-k71`'s body carry the correction.

### The instrument said the routing was done and the CROSS-TREE control said it was not

The subject sweep came back with **31 sites, all in `docs/formalism-findings.md`**
— down from 93 — with the positive control (`cross-model-replay-k15`, 15) and the
negative control (`formal-synthesis-k99`, 0) both holding. Read alone that is a
finished job.

It was not. Running the **cross-tree control** — *the same pattern must still
find the class where it legitimately lives* — showed **27 sites naming
`closed-sets-k69`**, which this session had just turned into a **node**. The
decomposition reproduced, one level down, exactly the defect the routing was
fixing: a pointer naming a directory instead of an owner. Ten of the 27 were
owner-of-undecided pointers and are now `closed-set-additions-k74`'s; the other
17 were attributions of work done and now name this leaf. The same check then
found five more naming `catalogue-disposition-k64`, written by
`obligation-placement-k63` when that handle was still a leaf; all five are
`closed-set-additions-k74`'s and are re-pointed.

**The lesson is the one `references/execute.md` states and it paid out
immediately: a clean subject sweep is not the evidence.** Had this leaf checked
only that `formal-synthesis-k16` was gone, it would have retired having moved the
defect rather than removed it, and the three sweeps in the sibling leaves' `Done
when` would have inherited it. The node handles now find **zero** live sites —
`formal-synthesis-k16` outside the log, `closed-sets-k69`, and
`catalogue-disposition-k64` — while every leaf handle finds its own.

### The findings log is appended to, never rewritten

Seven entries that named a disposition as owed now carry a
`> **[disposed by ...]**` line beside the finding. The finding's own prose is
byte-unchanged, because it was true when it was written and the log is a record
of what each session found rather than of what was later decided. **So the 31
surviving sites are correct and are not a residue**: a later reader meeting entry
046's `reap` finding finds the catalogue's answer beside it without the log
having been edited to pretend the question was never open.

That has a consequence for this node's closing sweep, and it is written into
`lifecycle-scope-k72`: over the log, the clean condition is *every site carries a
disposition*, not *the name is gone*.

### No model re-run is owed, and that is established rather than assumed

Every hunk in the four model files is **inside a comment** — `//` and `///` lines
in the two `.qnt` files, `/* … */` blocks in the two `.als` files — so no model
semantics moved. Verified rather than eyeballed, and with the controls that make
it evidence:

| check | result |
|---|---|
| `models/run.sh --list` | `-- 128 obligations in scope`, before and after every catalogue edit |
| `quint typecheck models/system/lifecycle.qnt` | OK |
| `quint typecheck crates/grove-task-tree/models/task-tree.qnt` | OK |
| Alloy `run witness_FN_01a_…` on `finish.als` | exit 0, a 19,688-byte instance |
| Alloy `check FN_15d_…` on `finish.als` | exit 0, no counterexample |
| Alloy `run witness_SY_05a_a_completed_teardown_and_then_a_fresh_scaffold` on `lifecycle.als` | exit 0, an instance |
| **negative control** — the same command on a deliberately broken copy of `lifecycle.als` | **rejected**, `CompParser.syntax_error` |

**The negative control earned its place on the first use.** The first Alloy check
this session ran reported *"parsed (no syntax error)"* and had run nothing at
all: the host's default `java` is 16 and Alloy 6 needs 17+, so every invocation
died with `UnsupportedClassVersionError` and a grep for syntax errors read the
launch failure as clean. That is the catalogue's own runner obligation 1 — *a
tool that failed to launch reports what a tool that found nothing reports* —
happening to the session that was quoting it, and the broken-copy control is what
exposed it. The figures above are from `~/.local/share/jdk/amazon-corretto-21`,
which is what `models/run.sh`'s `pick_java` resolves and what the runner uses.

### One defect surfaced that is not this leaf's, and it went to the tree rather than inline

Validating the relative links this session's two `leaf-decompose` moves had
staled — three, all in the moved briefs, all fixed — surfaced **one broken link
in the durable set that predates this grove's disposition work**:
`docs/formalism-findings.md:3474` writes `](../adr/bulk-marks-are-not-atomic.md)`
from a file already inside `docs/`.

It was **not fixed here**: that file is a log this subtree appends to rather than
edits, and a link defect is not a catalogue disposition. It is written into
`handoff-audit-k66`, whose `Done when` already carries *every durable formal
artifact is reachable from its component owner* — a broken link is exactly that
obligation failing, so the concern has an owner already and needs no new leaf.

**Enumerated rather than reported as one instance**, because a single broken link
says nothing about whether there are twelve: every `](../…)` and `](./…)` link in
the 50 durable `.md`, `.als` and `.qnt` files under `docs/`, `models/`,
`crates/`, plus `CONTEXT.md`, `CONTEXT-MAP.md` and `README.md`, was resolved
against the filesystem, and that one is the only miss.

**And the checker was wrong first.** Its first run reported 28 broken links; 25
were `path#L1234` citations in a retired review leaf, where resolving the
fragment as part of the path turns a correct citation into a miss. Stripping the
fragment left 3 — the ones this session's moves had actually broken. A link
checker that over-reports is as useless as one that under-reports, and the
difference was visible only because the output was read rather than counted.

### The ADR set was reconciled and gains nothing — which is a conclusion, not a skipped step

`ADR-FORMAT.md`'s test is an **AND** of three: hard to reverse, surprising
without context, *and* the result of a real trade-off with a rejected
alternative. Each of this leaf's three decisions was put to it.

- **Item 8** comes closest and still fails the first leg. It is surprising — a
  reader expects `reap` to be gated like every other tree-adjacent action — and
  it has a genuine rejected alternative with a stated cost (gate it, and
  `SY-05.a` becomes unwitnessable while the loop can never scaffold the grove a
  proven teardown just made). But it is a **description of what the system does**,
  which both families and the shipped driver already implement, rather than a
  choice among live options; undoing it would break the product, which is a
  reason it is *right*, not evidence it is *hard to reverse*. Its full reasoning
  is in §*Actions*, where a reader meets the table it constrains.
- **Item 19's rule** fails on the same leg and on the third: reversing it costs
  one paragraph, and the alternative — populate an exercise-removal row by
  reading the witness's prose — is not a trade-off but the mistake, three times
  over. It belongs in the assumption table, which is where it went.
- **Item 25** is a numeral and a missing paragraph.

**And no existing record needed reworking.**
`obligations-follow-context-not-artifact` is untouched and still accurate;
`root-lifecycle-stays-with-its-receipt` gained only two corrected deferral
pointers and one sentence saying why its two deferred questions do not travel
together. The two records that items 3 and 6 will bear on —
`one-live-driver-per-working-tree` (the driver "stops `blocked`") and
`task-tree-transactions-fail-closed` (a tracked witness "keeps the witness
unwalkable as **Recovery pending**") — are deliberately **not** touched here:
both are evidence `closed-set-additions-k74` weighs, and pre-adjusting a record
that a later leaf must argue against would put this session's guess in the way
of that argument.

### Run line

```sh
models/run.sh --list          # -- 128 obligations in scope, exit 0
models/run-controls.sh        # -- runner controls: 10 passed, 0 failed, exit 0
```

`--list` was re-run after **every** catalogue edit and printed 128 each time,
which is what makes *manifest-neutral* a measurement rather than an intention.

**The controls run is against the finished catalogue and that was checked rather
than assumed.** Two earlier runs also passed 10/10, and both were discarded: each
had started before an edit that landed while it was running, so neither was
evidence about the tree being retired. The reported run was started only after
the last catalogue edit, and
`shasum docs/specs/semantic-contract.md` was `9cf430ab…` both when it started and
when it finished — the same file it read. A green control run against a file
that has since moved is exactly the *provenance* failure entry 049's own
whole-repository table was careful about, one grain smaller.

**No scope suite was re-run and none is owed.** Every model-file hunk is inside a
comment; both `.qnt` files typecheck and both edited `.als` files execute a real
command, with a deliberately broken copy rejected as the negative control. The
`(family, obligation)` matrix is untouched because the manifest is.
