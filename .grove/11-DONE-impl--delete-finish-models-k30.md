# delete-finish-models-k30

## Goal

Delete `crates/grove-finish/models/` — the last of the formal-methods campaign's
apparatus — and rework in place the nine decision records that cite it, so each
finding survives its instrument.

## Context

`delete-formal-models-k29` deleted the campaign's apparatus and the semantic
contract it checked. Its `## Done when` enumerated `models/`,
`crates/grove-task-tree/models/`, the two `scripts/` reporters and
`docs/specs/semantic-contract.md`, and omitted this directory.

k29 read the omission as an oversight rather than as the boundary, and its
`## Decisions (running log)` records the evidence: k29's `## Context` puts the
material the rebase surfaced at ~38,000 lines, the enumerated set measures
23,757, and these four files are 16,392 — the shortfall, almost exactly. The
same list's stated line counts run low against the files they name, and its one
note about `crates/` asserted a live crate that does not exist. Both are the
signature of a list written from the campaign's records rather than from a walk
of the tree.

So the boundary the human settled — *the apparatus goes, the records stay* —
puts this directory on the deleted side, and k29 externalised it rather than
absorbing it because the cost is not clerical. Nine records under `docs/adr/`
bind their arguments to these files as evidence, and each needs the in-place
rework k29's `## Notes` describes rather than a link repair.

**This leaf sits immediately before `delete-finish-transaction-k8`** because
that is the leaf which reworks the same records for the same subject. Running
them adjacently means the finish records are opened once, not twice; running
this one first means k8 never owes a reconciliation to models already scheduled
for deletion, which is the argument k29 itself was ordered on.

After k29 this directory is already stranded: `models/run.sh` was its only
runner (`run.sh:171` mapped scope `finish` here) and went with k29.

## Done when

- **Deleted:** `crates/grove-finish/models/` — `finish.qnt`, `finish.als`,
  `finish-controls.qnt`, `README.md` (16,392 lines). The directory holds nothing
  else, and `crates/grove-finish/` goes with it: it is not a crate, it is a
  model-only directory naming a context (`Cargo.toml`'s `members` lists only
  `crates/ordinal-fs-tree`).
- **Each citing record reworked in place, its finding kept.** The shape is
  `docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md`, reworked
  by `delete-migration-k6`: the decision outlives the instrument, and
  `## What enforces it` names whatever checks it now, saying plainly that the
  model controls were retired with the campaign. The nine:
  `finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`,
  `success-is-proved-by-the-ticket-not-the-tree.md`,
  `root-lifecycle-stays-with-its-receipt.md`,
  `a-shared-safety-claim-names-the-role-not-the-artifact.md`,
  `a-refusal-leaves-nothing-standing.md`,
  `a-closed-partition-is-over-outcomes-not-states.md`,
  `obligations-follow-context-not-artifact.md`, and the two others a sweep
  finds — enumerate rather than trust this list.
- **Kept, and their citations still resolve:** the campaign's records under
  `docs/` that k29 kept, in particular `docs/formalism-findings.md`,
  `docs/candidate-lessons.md` and `docs/driving-a-checkable-loop.md`, each of
  which links into these files.
- `CONTEXT.md`'s `crates/grove-finish/models/` reference reconciled.
- `cargo test` and `cargo clippy --all-targets` clean;
  `tests/reference_navigation.rs` is the completeness check, not a grep.
- `CHANGELOG.md`'s `## Unreleased` records the removal beside k29's.

## Notes

**Not a cutover leaf.** No tree shape, no filename grammar, no verb surface — it
neither rebuilds nor reinstalls, and it signals completion normally.

**`obligations-follow-context-not-artifact.md` is the one to read first.** Its
whole argument is about where an obligation family *lives*, stated over the
`FN-` / `TT-` / `SY-` split that this deletion completes the removal of. It may
turn out to be a record whose subject is wholly gone rather than one whose
citation moved; `ADR-FORMAT.md`'s rework guidance owns that call, and deleting a
record is a legitimate outcome there.

**Do not reopen the boundary.** Whether the campaign's records under `docs/`
survive was settled by the human, against a stated alternative that took them
too; k29's `## Context` carries the reasoning. This leaf moves apparatus only.

## Decisions (running log)

**Seven citing records under `docs/adr/`, not nine, and the enumeration is the
finding.** `## Done when` named seven and predicted two more a sweep would find.
The sweep was run whitespace-collapsed per file — k29's own correction, after a
line-based grep missed a citation straddling a line break — and over the **`FN-`
obligation family** as well as the four paths, because a record can bind these
models without naming a file. It returns exactly the seven named. The other
records carrying formal-methods vocabulary bind something else:
`a-lifecycle-claim-says-what-it-is-over` cites `models/system/` (deleted and
already reworked at k29), and `entry-name-is-the-only-seam`,
`entries-are-never-removed`, `task-names-are-canonical` and
`bulk-marks-are-not-atomic` cite `docs/ordinal-fs-tree/models/`, which survives.
The two extra were k29's arithmetic, not a class the tree holds. Two records
*were* touched beyond the seven — `evidence-outlives-the-instrument`, which names
this leaf in its own text, and nothing else — so the paragraph count is nine only
if the enumeration is of edits rather than of citations.

**`obligations-follow-context-not-artifact` is kept.** The record delegated the
call here, and its own text is the case for deleting it: every artifact it names
is gone, so under `ADR-FORMAT.md`'s current-state rule it describes nothing that
is. Two facts decided it the other way. First,
`docs/driving-a-checkable-loop.md` — a kept record — classifies the campaign's
decision set and names **four** records as *about evidence rather than about
Grove*: this one, `a-shared-safety-claim-names-the-role-not-the-artifact`,
`a-closed-partition-is-over-outcomes-not-states` and
`a-lifecycle-claim-says-what-it-is-over`. All four have lost their subject; the
last lost it a leaf earlier, at k29, and **was kept**. Deleting this one alone
treats a sibling differently on no distinguishing ground and falsifies the
"four of which" sentence in a document this leaf is required to keep resolvable.
Second, `evidence-outlives-the-instrument` draws the line at **citation**, and
this record is cited from `CONTEXT-MAP.md`, from `docs/driving-a-checkable-loop.md`
and twice from `docs/formalism-findings.md` — once as the *recorded outcome* of a
hand-off that named the placement rule as owed. A hand-off whose outcome has been
deleted is the dangling authority that record exists to prevent.

What is genuinely open is one question about the **set**, not four about records:
whether four records whose subject is a retired campaign belong under `docs/adr/`
at all rather than beside the evidence under `docs/`. It applies to all four
identically and `spec-to-current-state-k23` already walks the whole ADR and spec
set as a checklist at the end of this run, so it is routed there rather than
answered here on one quarter of its subject.

**Only one of the seven has a shipped enforcer, and the six that do not say so.**
`success-is-proved-by-the-ticket-not-the-tree` gained a real `## What enforces it`:
`tests/finish_lifecycle.rs`'s `rootless_finish_retry_refuses_when_no_teardown_result_exists`,
`..._names_the_message_it_required_and_the_one_it_observed` and
`..._refuses_a_teardown_result_from_another_finish_attempt` decide a finish by the
teardown commit's attempt-bound message and refuse to read it off the tree —
which is both directions this record forbids. The other six bind a rule whose
next consumer is a reader; `a-closed-partition-is-over-outcomes-not-states` names
the one it has (`CONTEXT.md`'s **Recovery pending / Ownership conflict** entry),
and the rest state the absence. That honest form is now recorded in
`evidence-outlives-the-instrument` so the next rework does not infer a check from
the section's existence.

**Two of the seven were leaving open work that cannot be done, and now say so.**
`finish-keeps-a-cleanup-layer-it-has-not-proved-forced` reopens Q1 on a run
requiring `sweep-ownership-k81` and `alloy-candidate-k82` and directs a reopener
to `crates/grove-finish/models/README.md`; with no column, no mirror and no
`README.md`, Q1 is not reopenable on the terms it states. The `FN-24.a` / `FN-28`
Alloy repair `a-shared-safety-claim-names-the-role-not-the-artifact` leaves
outstanding is likewise not work anyone will do. Both were found by reading what
each record *asks a future session to do*, which the link repair would not have
surfaced.

**Prose paths stay, links go — k29's precedent, and the mechanical half is the
test.** Fourteen relative markdown links into the deleted files became plain
names; the backticked model paths in `docs/formalism-findings.md` stayed, because
that document already declares its paths *descriptions of where a finding was
made* and an entry saying a model refused something is accurate as history.
`tests/reference_navigation.rs`'s `every_repository_markdown_reference_resolves`
is the check, and it walks every `.md` in the tree rather than a named surface.

**The sweep's first positive control was broken and read clean everywhere.** The
zero-links result for `crates/grove-finish/` was first controlled against
`docs/ordinal-fs-tree/models/` — which returned **zero** as well, because nothing
links to it either, so the control proved only that the regex ran. Re-controlled
against `docs/ordinal-fs-tree/` (11 links) and against a path grep for
`ordinal-fs-tree/models` (8 files), both of which a legitimately surviving class
should return, the zero is a real zero.

**`CONTEXT.md`'s `crates/grove-finish/models/` reference does not exist**, so that
`## Done when` bullet rests on a false premise. What `CONTEXT.md` does carry is
its *Root and lifecycle semantics* section attributing the whole deletion to
`delete-formal-models-k29` while the `FN-` column was still on disk, and one
`FN-25.a` citation in the **Recovery pending / Ownership conflict** entry. The
section now says which leaf took which column, which is the reconciliation the
bullet was reaching for; `CONTEXT-MAP.md` gained the same, beside the catalogue's.

**No adversarial second pass was run on the delete.** `references/impl.md` asks
for one where a decision is hard to reverse, and this session was instructed not
to dispatch subagents. The mitigation is that neither half is hard to reverse: the
deleted files are recoverable by revision and `evidence-outlives-the-instrument`
says so as its own reopen condition, and keeping a record is undone by deleting
it. Recorded rather than quietly skipped.
