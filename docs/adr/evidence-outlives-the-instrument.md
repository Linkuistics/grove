# Evidence outlives the instrument

When a formal-modelling campaign is retired, the **apparatus** is deleted and the
**evidence** is kept. The models, their controls, their runner and the claim
catalogue they were checked against go; the records of what they found stay,
where anything still cites them for authority.

This repository ran such a campaign — two formalisms, 129 obligations across 258
`(family, obligation)` cells, over its own protocols. `delete-formal-models-k29`
deleted about 23,800 lines of it and kept seven documents under `docs/`, and
`delete-finish-models-k30` took the last directory — `crates/grove-finish/models/`,
16,400 lines carrying the `FN-` family — on the same terms.

## The trade-off

An instrument and its findings decay at opposite rates. The instrument is only
worth its maintenance while something runs it: a model nothing checks is a file
that goes stale silently, and a runner nobody invokes is a build dependency on a
toolchain nobody installs. Here the phase that would have consumed these models —
documentation, since about four in five material findings produced no executable
test — never ran, and the design they specify is being dismantled by the very
workstream that retired them. Nothing was left to run them for.

The findings decay far more slowly, and in this case not at all, because they had
already been distilled into two `linkuistics` skills that ship to other
repositories. Those skills bind every rule to a numbered entry in
`docs/formalism-findings.md` or an adjudication in `docs/candidate-lessons.md`,
one of them through a public URL. **A rule whose evidence has been deleted is a
rule nothing can falsify** — which is the exact failure the `doubt-driven-development`
skill exists to prevent, so deleting the evidence would have made the skills
unfalsifiable by their own standard.

So the line is drawn at citation, not at size or at subject matter: an artifact
goes if nothing outside the campaign cites it, and stays if something does. That
is checkable rather than a matter of taste, and `crates/grove/tests/reference_navigation.rs`
checks the half of it that is mechanical.

## The alternative, and why it was rejected

**Delete the records too**, on the ground that a workstream's own account of
itself is the least neutral document in the repository and that ~38,000 lines of
retrospective is a large thing to carry for a subject that no longer exists.

It was rejected because the skills would then assert rules with nothing behind
them. The distilled form is *shorter* than the evidence by design — that is what
distillation is — so it cannot stand in for it: a reader who doubts a rule needs
the entry that produced it, including the entries recording where the campaign's
own sessions were **wrong**, which `candidate-lessons.md` exists to hold. The
non-neutrality objection is real and is answered inside the documents rather than
by deleting them; `candidate-lessons.md` re-opened the model files and falsified
or weakened three of the six claims the producing sessions believed.

## What it costs, stated rather than hidden

Two kept records were **derived** from scripts that went with the apparatus, so
`docs/loop-record.md` and `docs/review-yield.md` can no longer be regenerated and
are labelled frozen. The loss is uneven and `review-yield.md` says so: its derived
half is re-extractable in principle, its judged half is not, because the
item-to-block classification table lived inside the deleted script. A record kept
for its authority that quietly stops being reproducible is worse than one that
says it has stopped.

Every record that cited a deleted file keeps its finding and loses its pointer —
the decision outlives the instrument, and `## What enforces it` names whatever
checks it now.
`docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md` is the worked
example, reworked in place at `delete-migration-k6` before this record existed.

**Some of those findings are then enforced by nothing, and saying so is part of
the rework.** `delete-finish-models-k30` reworked seven records and only one,
`success-is-proved-by-the-ticket-not-the-tree`, had a shipped test to name; the
rest bind a rule whose next consumer is a reader rather than a check. A
`## What enforces it` that names a live consumer where there is one and states
the absence where there is not is the honest form, and it is what stops a reader
inferring a check from the section's existence.

## What would reopen this

A consumer that needs to *re-run* rather than to *cite* — a repository adopting
`model-led-development` that wants the corpus as a worked example rather than as
evidence. The models are in this repository's history and are recoverable by
revision; what is not recoverable is a claim that they were ever green, which is
why the run results stayed in `docs/formalism-findings.md` rather than only in
the model READMEs.
