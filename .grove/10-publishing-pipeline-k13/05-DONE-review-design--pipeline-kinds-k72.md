# pipeline-kinds-k72

**Reviews:** pipeline-kinds-k27

## Goal

An adversarial read of the pipeline extraction: the two ADRs it landed, the
four disciplines it wrote into `pipeline-skills-k28`'s task file, and the shape
it gave `crate-books-k14`'s books. Findings only — no fixes, no skills authored.

## Context

- The artifacts: `docs/adr/the-editorial-pipeline-is-four-kinds.md`,
  `docs/adr/a-feedback-edge-is-forward-tree-growth.md`, the
  `## The four kinds` and `## The launch templates` sections of
  `pipeline-skills-k28`'s task file, and the pipeline paragraphs added to
  `.grove/11-crate-books-k14/BRIEF.md`. `pipeline-kinds-k27`'s own
  `## Decisions (running log)` records why each call went the way it did.
- The instrument that governs the set is
  `docs/evaluations/editorial-pipeline-pilot/preregistration.md` *Part 4*, and the
  evidence is `docs/evaluations/editorial-pipeline-pilot/README.md`. Both are
  frozen and neither is editable by anything downstream of them.
- **Nothing here has been implemented.** No skill exists yet, and the review's
  subject is a design and a work order, not shipped bytes.

## Done when

Each of these is read and answered, and every finding is written down with the
evidence that supports it.

- **Does the extraction match what the report licenses — exactly?** Not fewer,
  and above all not more. The verdict table fixes what each row permits, and an
  `Undetermined` licenses nothing; check that no stage was extracted, folded or
  dropped on a basis the report does not supply, and that nothing was smuggled in
  as a fifth kind under another name.
- **Is `draft` a kind the evidence supports, or a convenience?** It was never
  scored. It carries both merged charters and the structure-brief precondition,
  and the argument against reusing `impl` rests on those two facts. Try to break
  it.
- **Is the ADR set minimum and coherent?** Two records were written where one was
  possible. Argue the merge: do they reopen on different conditions, or is the
  feedback-edge record a section of the extraction record wearing a slug? Check
  every citation resolves and that neither record restates the other.
- **Is the feedback-edge mechanism actually performable with the verbs as they
  are?** Read `grove-llm leaf-add --help`, `leaf-insert --help` and
  `leaf-decompose --help` against the claims: that `leaf-decompose` grows exactly
  one child and needs `--kind draft`; that a lazy chain leaves nothing queued so
  a re-run is an append; that `leaf-insert` is needed only when a later sibling
  entry holds live work; that all three verbs refuse a kind with no launch
  template. A claim about the CLI that the CLI does not honour is the most
  expensive defect available here, because four skills are authored from it.
- **Does the filing survive `corpus-rules-have-one-owner.md`?** Run its placement
  function over each rule in the work order. Is `Occasion` honest for each? Does
  any rule appear in both the family file and a kind skill? Is the claim that the
  spine's `SKILL.md` gains nothing right under
  `restatement-declares-its-class.md`, or does some rule's load predicate fire
  from `SKILL.md` and so require a `trigger` sentence after all?
- **Are the disciplines document-general, or do they assume a walkthrough book?**
  The tokens are unprefixed and the skills install everywhere. Find every sentence
  that would be false, meaningless or unreachable in a repository with no
  `docs/specs/walkthrough-books.md`, no fragment validator and no `book-check`.
- **Two rules rest on judgement rather than evidence.** Mandatory stage
  membership — a stage may not be skipped because the previous session judged it
  would find nothing — and the one-re-run escalation bound. Neither was measured.
  Are they defensible, over-reach, or under-specified at the boundary?
- **What did the design fail to say?** The report names a limit the extraction has
  to live with (the fold is conditional on the structure brief) and a conflict
  no stage could close (`01-orientation.md#public-surface`, which belongs to
  whoever next revises a structure brief). Is each one carried somewhere a session
  will meet it, or has it been left in a frozen evaluation document that binds
  nothing?

## Notes

**Position matters here and is why this was inserted rather than appended.**
`pipeline-skills-k28` authors the skills from this design, so the walk must reach
this review — and any integration it cuts — before that leaf runs. An integration
cut here goes ahead of `pipeline-skills-k28` for the same reason.

**Do not author a skill, and do not fix a discipline you disagree with.** The
output is findings. If a charter cannot be written as a skill, that is the most
valuable finding on the list, and it is still a finding.

## Findings

Four findings survived the adversarial read. The remaining axes did not produce
a finding: the four-stage set matches the preregistered licensing table exactly;
`draft` earns a distinct token because its merged charters and hard structure-
brief precondition cannot bind generic `impl`; the two ADRs reopen on different
conditions and form a minimum coherent set with resolving citations; all three
grow verbs implement the configuration gate and tree mutation their design
requires; mandatory membership is a defensible consequence of asking a stage
with a different charter to predict its successor's result; and the operative
copy-edit/art/proof tests contain no walkthrough-specific path, validator, or
`book-check` dependency.

### F1 · High — A feedback re-run has no non-duplicating successor protocol

The work order makes two rules unconditional: every non-proof stage cuts its
normal successor as its last tree-growth act
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:101-109,168,187,212`),
and the stage discovering backward work cuts the earlier-stage re-run **plus all
later stages that must re-read the changed material**
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:120-128`).
Those instructions cannot both be followed once a feedback edge crosses more
than one stage.

For example, if `art` finds a `draft` defect, cutting only `draft` and the normal
next `proof` follows the ADR's call-order recipe
(`docs/adr/a-feedback-edge-is-forward-tree-growth.md:14-19`) but lets proof run
without a copy-edit or art re-read of the changed draft. Pre-cutting `draft →
copy-edit → art → proof` satisfies the re-read instruction, but the draft re-run
then cuts another copy-edit, that copy-edit cuts another art, and that art cuts
another proof. No rule tells a re-run that its successor is already queued, lets
it suppress the mandatory handoff, or tells it to insert one successor before an
already queued terminal stage. The ADR's claimed filesystem example exposes the
same omission: it shows a second draft between art and proof while claiming the
leaf sequence is the honest history, but contains neither required intermediate
re-read (`docs/adr/a-feedback-edge-is-forward-tree-growth.md:40-48`).

Proof is internally inconsistent at the same boundary: it may turn a hand-forward
entry into a re-run leaf, then says it cuts no next stage and that its retirement
closes a node with no live leaf
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:227-239`).
The CLI primitives are capable — append and insert behave as claimed — but the
design does not compose them into one ordered, terminating correction chain.

### F2 · High — The next editorial stage is not the scheduled review the allowance exempts

The family explicitly takes no `review-<stage>` leaves
(`docs/adr/the-editorial-pipeline-is-four-kinds.md:3-8`), yet the work order says
every stage except proof spends no in-session reviewer because the next
editorial producer qualifies under `references/execute.md`'s *already has a
review beside it* case
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:132-136`).
That owner says something narrower: the exemption is for a producer that
actually has a `review-*` leaf beside it
(`plugins/grove/skills/grove/references/execute.md:21-31`).

The semantic substitution fails too. The family requires every stage to change
only its own charter's class
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:96-100`):
copy-edit must not repair or even restructure draft's structural work, and art
reads only for figures. A fresh context reading the whole document against a
different charter is not an adversarial read of its predecessor's obligations.
The pipeline ADR's own reopen condition concedes the gap — a following stage
finding a defect outside its charter is what a review would catch and this
pipeline cannot (`docs/adr/the-editorial-pipeline-is-four-kinds.md:121-129`).

As written, the loaded path will carry two owners and two answers for the review
budget: the execute procedure grants the ordinary allowance, while the new
family file removes it. Either preserve the ordinary producer allowance, or
design and file an explicit editorial-family exception that really supplies the
same adversarial coverage and reconcile the execute owner; calling the next
different-charter producer a `review-*` leaf does neither.

### F3 · Medium — The structure-brief precondition has no document-facing interface

The report's most important limit is a **detailed human-authored structure
brief**, and the extraction correctly repeats that the fold is unsafe without
one (`docs/adr/the-editorial-pipeline-is-four-kinds.md:67-75`). The generic
`draft` work order, however, says only that “a structure brief” must exist and
that the session stops if one does not
(`.grove/10-publishing-pipeline-k13/07-impl--pipeline-skills-k28.md:142-153`).
It gives an installed skill no way to identify the artifact or distinguish it
from Grove's own node brief.

That distinction is operationally load-bearing: every decomposed document node
necessarily has a `BRIEF.md`, because `leaf-decompose` moves the old task body
there (`CONTEXT.md:736-744`), and bootstrap necessarily reads it
(`CONTEXT.md:383-384`). A naive existence test therefore passes for every draft,
including exactly the no-human-structure case that must stop. This campaign's
book tasks happen to name a separate structure task explicitly — for example
`overview-book-k30` names `overview-structure-k29`
(`.grove/11-crate-books-k14/04-impl--overview-book-k30.md:3-13`) — but the
unprefixed skill claims to be document-general and cannot depend on this grove's
task convention. Define how a document declares its human structure artifact,
and the minimum information that makes the precondition true, so the fold's
limit is an interface a session can actually test rather than a phrase a node
charter satisfies accidentally.

### F4 · Medium — The confirmed structure-standard conflict has no future owner

The frozen report records one conflict no editorial stage could close:
`01-orientation.md#public-surface` is simultaneously forbidden by the `D2`
catalogue rule and required by the human structure brief, independently confirmed
by the proof-only probe. It assigns the correction to whoever next revises
`docs/specs/jj-workspace-book-structure.md`
(`docs/evaluations/editorial-pipeline-pilot/README.md:587-597`).

That datum appears nowhere in the two ADRs, the producer decision log, the
pipeline-skills work order, or the crate-books brief — the bounded handoff
surfaces this review was asked to inspect. Those artifacts do carry the fold's
structure-brief limit, so the omission is not a general failure to read the
report; this one actionable conflict was left only in frozen evidence that no
future structure session is directed to read. Give it a durable owner a future
revision will meet — the structure artifact itself, a precisely placed leaf, or
another cited current-state surface — without rewriting the frozen report.
