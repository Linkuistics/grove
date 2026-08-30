# delete-formal-models-k29

## Goal

Delete the formal-methods experiment's **apparatus** and the semantic contract it
checked. The campaign's value is the lessons it produced and the `linkuistics`
skills those lessons became; the runnable machinery has no consumer left and the
contract specifies a system this grove is actively dismantling.

## Context

This leaf was cut mid-`delete-migration-k6`, when rebasing that leaf onto `main`
surfaced ~38,000 lines of formal-modelling material that had landed there while
the refactor branch was off to the side (`finish-k93`, `model-led-development-k94`
through `-k98`). The human's decision, recorded at the moment it was taken: *"those
formal models were an experiment that can in fact be deleted — all that matters is
the lessons that were learnt whilst building them, and the skills generated
therefore."*

The boundary was settled the same way, against a stated alternative that took the
campaign's records too. It stands where it does because **the skills cite the
records for their authority**: `plugins/linkuistics/PROVENANCE.md` and four files
under `model-led-development/` and `doubt-driven-development/` bind their rules to
`docs/formalism-findings.md`, `docs/candidate-lessons.md` and
`docs/review-yield.md` — one through a public GitHub URL to `main`. A rule whose
evidence has been deleted is a rule nothing can falsify, which is the failure the
`doubt-driven-development` skill exists to prevent. So the evidence stays and the
machinery goes.

## Done when

- **Deleted**, with nothing left importing, running or citing them:
  - `models/` — `system/lifecycle.qnt`, `system/lifecycle.als`,
    `system/lifecycle-controls.qnt`, `system/README.md`, `run.sh`,
    `run-controls.sh`, `README.md` (~8,600 lines).
  - `crates/grove-task-tree/models/` — `task-tree.qnt`, `task-tree.als`,
    `task-tree-controls.qnt`, `README.md` (~7,600 lines).
  - `scripts/loop-record.py` and `scripts/review-yield.py` (~1,500 lines).
  - `docs/specs/semantic-contract.md` (~2,900 lines).
- **Kept, and their citations still resolve:** `docs/formalism-findings.md`,
  `docs/candidate-lessons.md`, `docs/review-yield.md`, `docs/loop-record.md`,
  `docs/preservation-baseline.md`, `docs/driving-a-checkable-loop.md`,
  `docs/results-of-formal-methods-trial.md`.
- Every reference into the deleted set is reconciled — `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table, `CONTEXT.md`, `CONTEXT-MAP.md` (which lists
  `semantic-contract` among the contexts' specs), the two `docs/` records that
  cite model files, and the skills. `tests/reference_navigation.rs` is the check
  that this is complete; run it rather than trusting a grep.
- `docs/results-of-formal-methods-trial.md` still reads truthfully once its
  subject is gone — it is the trial's *conclusion*, and a conclusion that points
  at deleted models needs its pointers turned into description.
- `cargo test` and `cargo clippy --all-targets` are clean.
- `CHANGELOG.md`'s `## Unreleased` records the removal, naming what was kept and
  why.

## Notes

**This is not a cutover leaf.** It changes no tree shape, no filename grammar and
no verb surface, so it neither rebuilds nor reinstalls, and it signals completion
normally. The root brief names the five leaves that may reinstall and this is not
one of them.

**Ordering: it runs after `delete-migration-k6` and before everything else.**
After k6 because k6 is already written and blocked only on a human-run install;
before the rest because the semantic contract describes the witness lifecycle,
the git lane, the finish transaction and the module boundaries that k7, k8, k9
and the store node all change — every one of those leaves would otherwise owe a
reconciliation to a document scheduled for deletion.

**The one file that needs judgement rather than deletion.**
`docs/specs/semantic-contract.md` is not apparatus: it is a specification of
grove's semantics, written to be model-checked. It goes because its checker goes
and because k6 already contradicts it in roughly 285 places, all of them the
format witness. Do not attempt to salvage it into a witness-free contract — that
is a design session's work, not this one's, and `spec-to-current-state-k23`
already owns what the surviving specs must say. If something in it turns out to
be load-bearing prose rather than a checkable claim, promote that paragraph into
`docs/ARCHITECTURE.md` where the current-state documentation lives, and say so.

**`crates/grove-task-tree/models/` is inside a live crate.** Deleting the
directory must not disturb the crate; check `Cargo.toml` and the crate's own
`README` for references before removing it.

**One ADR already survived this deletion and is the model for the rest.**
`docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md` was reworked
in place by `delete-migration-k6`: its *decision* outlived the instrument that
found it, and its `## What enforces it` section now names product tests and says
plainly that the model controls were retired with the campaign. That is the shape
to reach for wherever a record cites a model — the finding stands, the citation
moves to whatever checks it now.

## Decisions (running log)

**`crates/grove-task-tree/` is not a live crate, and the note warning that it is
rests on a false premise.** The directory contains exactly one entry, `models/`,
and nothing else — no `Cargo.toml`, no `src/`. The workspace has one member,
`crates/ordinal-fs-tree` (`Cargo.toml`, `members = ["crates/ordinal-fs-tree"]`),
so `crates/grove-task-tree/` and its sibling `crates/grove-finish/` are
model-only directories that borrow the `crates/` path to say *which context these
obligations belong to*, which is what `docs/adr/obligations-follow-context-not-artifact.md`
argues placement should do. Deleting `crates/grove-task-tree/models/` therefore
empties the directory rather than disturbing a crate, and the directory goes with
it. The manifest check the note asks for was still run, and found nothing.

**`crates/grove-finish/models/` is the same campaign's apparatus, was omitted
from `## Done when`, and is externalised rather than absorbed.** The four files
there (`finish.qnt`, `finish.als`, `finish-controls.qnt`, `README.md`, 16,392
lines) carry the `FN-` obligation family, and `models/run.sh` — which this leaf
deletes — is their only runner (`run.sh:171` maps scope `finish` to that
directory). So this leaf strands them: apparatus with no runner and no consumer,
which is precisely what the goal sentence says to remove.

Three things say the omission is an oversight rather than the settled boundary.
The `## Context` puts the surfaced material at ~38,000 lines; the enumerated set
measures 23,757 (`models/` 11,735, `crates/grove-task-tree/models/` 7,630, the
two scripts 1,467, `semantic-contract.md` 2,925), and the shortfall is closed
almost exactly by these 16,392. The `## Done when` line counts are themselves
approximate and low (`models/` is stated at ~8,600 against an actual 11,735),
which is the signature of a list written from the campaign's records rather than
from a walk of `crates/`. And the note asserting `crates/grove-task-tree/` is a
live crate shows that walk did not happen.

It is still not absorbed here. The settled boundary — *apparatus goes, records
stay* — would put it in, but `decompose.md` is unconditional that work surfacing
mid-session is externalised, and the cost is not clerical: nine records under
`docs/adr/` cite these files as their evidence, and each needs the in-place rework
`## Notes` describes rather than a link repair. That is a leaf's worth of
judgement, and it is judgement about the **finish transaction**, whose own leaf
(`delete-finish-transaction-k8`) reworks the same records. So it is cut as a leaf
sequenced ahead of k8, where the two reworks meet once instead of twice.

**`docs/ordinal-fs-tree/models/` stays, and is not the same campaign.** It has
its own runners (`run-quint.sh`, `run-alloy.sh`) which `models/run.sh` documents
itself as *inheriting from* (`run.sh:77`, `:287`, `:616`) rather than owning, so
deleting `run.sh` leaves it runnable. It belongs to the `ordinal-fs-tree`
extraction, whose crate is a live workspace member whose tests cite those models
as standing authority (`crates/ordinal-fs-tree/tests/driving_a_tree.rs:403`,
`:611`, `tests/rewriting_on_disk.rs:6`). `semantic-contract.md` named it a
*delegated* boundary — something the contract consumed, not something it owned —
so the consumer's deletion does not reach it.

**Nothing in `semantic-contract.md` was load-bearing prose, so nothing was
promoted.** `## Notes` asked for the check and for it to be reported either way.
The document's thirty sections divide three ways and none of them leaves a gap.
Its `## Vocabulary` — *Identities*, *States*, *Actions*, *Outcomes* — is mirrored
term for term in `CONTEXT.md`, which is the glossary the map already names as
canonical, so the definitions survive where a reader looks for them. Its
`## Decisions`, `## Environment assumptions`, `## Deliberate omissions`,
`## Test seams` and the three `## Claims` families are apparatus: catalogue
mechanics, obligation identifiers, model paths and the runner's coverage rule,
all of which describe the instrument this leaf deletes.

The one section that reads like current-state description, `## The three lanes`,
is already `docs/ARCHITECTURE.md`'s: the deep repository seam at
`ARCHITECTURE.md:1058` describes the Git, native-jj and colocated-jj mechanics,
lane selection, the index backup and the successor hazard. Promoting the spec's
table would have duplicated a description that already exists — and `drop-git-lane-k7`
falsifies half of it two leaves from now, so the copy would have arrived stale.

**The skills needed no reconciliation, which is the boundary's own evidence.**
`## Context` argued the boundary from the skills citing the records; the sweep
confirms they cite *only* records. Every path in `plugins/linkuistics` reaches
`docs/formalism-findings.md`, `docs/candidate-lessons.md` or
`docs/review-yield.md` — all kept, including the one public GitHub URL to `main`,
which resolves to `docs/review-yield.md`. Not one skill file names a `.qnt`, a
`.als`, `models/run.sh` or `docs/specs/semantic-contract.md`. The boundary was
drawn exactly where the citations stop.

**Two kept records are now unreproducible, and say so rather than hiding it.**
`docs/loop-record.md` and `docs/review-yield.md` are derived files whose headers
carried a `Regenerate:` command naming the scripts this leaf deletes. Keeping the
record while deleting its generator is what the `## Done when` asks for, but it
changes what the documents are, so both are relabelled **derived, and now
frozen**. The cost is asymmetric and is stated in `review-yield.md` rather than
smoothed over: its *derived* half is still checkable in principle, because block
structure at a pinned revision is a fact anyone can re-extract, while its
*judged* half is not — the item-to-block assignment table lived inside the
script, not in the document, so the classification is no longer written down
anywhere. A reader deciding how much weight the counts carry needs that.

**`CHANGELOG.md` was not rewritten, and one of its claims turned out to be
false.** The file is a ledger of what changed and when, so its historical
entries keep naming artifacts that no longer exist — the sweep's ~40 hits there
are correct as history and were left alone. But `## Unreleased`'s
*`docs/ARCHITECTURE.md` now owns the campaign records* entry claimed rows for
`loop-record.md` and `review-yield.md` that the table did not actually have.
This leaf added them, and the new entry says so rather than quietly closing the
gap.

**The decision earned an ADR, `evidence-outlives-the-instrument`.** It clears
`ADR-FORMAT.md`'s three-part test rather than only one leg of it: hard to
reverse, because the kept records are now the sole surviving evidence for rules
that ship to other repositories; surprising without context, because a reader
meeting ~30,000 lines of retrospective about models this repository does not have
will reasonably propose deleting them; and a real trade-off, because the
alternative — take the records too — was stated and refused on a ground that
generalises. The record carries the rejected alternative, the asymmetric cost the
two frozen documents pay, and a reopen condition. Without it the argument would
have lived only in `.grove/`, which `grove finish` deletes, and
`spec-to-current-state-k23` sweeps the whole ADR and spec set at the end of this
run with no way to tell a kept record from dead weight.

**A wrap-hidden citation is why the sweep was run twice.** The first sweep was
line-based and over paths, and it read clean on
`docs/adr/entries-are-never-removed.md`, which says *"The\nformal models take the
same premise"* — the phrase straddles a line break, so no line contains it. The
second sweep collapses whitespace per file and matches the **concept** rather
than the path list, which is what `references/execute.md` means by enumerating
the surface instead of sweeping a pattern list. It returned 112 sentences; all
but one classify as past-tense history in a record that now carries its
retirement paragraph, as `delete-finish-models-k30`'s to reconcile, or as
references to the surviving `docs/ordinal-fs-tree/models/`. The one real finding
was that wrap-hidden sentence, now naming which models it means — and it means
the surviving ones, so the deletion did not falsify it, only make it ambiguous.

The positive control for both sweeps was `docs/ordinal-fs-tree/models/`, a class
that legitimately survives: the same command and flags return 18 hits for it,
so a clean result elsewhere is not a broken instrument reading clean everywhere.
