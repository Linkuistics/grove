# figure-contract-k18

## Goal

Answer, from the pilot's measure, whether walkthrough books get figures with
machinery behind them — a figure format, an asset convention and a validator
that knows what an asset is — or whether art stays hand-authored Markdown or is
dropped. Record the answer as an ADR.

## Context

- The art stage is new ground: the existing `ordinal-fs-tree` book has no
  diagrams, no figures and no captions anywhere, and the fragment validator has
  no concept of an asset. Format, convention and validation are all unbuilt
  (decision 11 of `plan-k1`).
- `measurement-report-k45`'s report, under the `pilot-measure-k26` node, is the
  evidence, and it is the whole reason this
  leaf comes after the pilot rather than before it. The root brief's *Notes*:
  expensive machinery ordered ahead of the measurement that would justify it
  cannot answer whether it was needed.
- The shared specification `walkthrough-books-spec-k20` deliberately left a hole
  for assets. Filling it — or recording that it stays empty — is this leaf's
  output.
- What machinery would actually cost, if it is earned: the fragment validator's
  scope is Markdown pages and byte-exact source reconstruction, and an asset is
  neither. It would need a fifth thing to know about, its own diagnostics in the
  `F0nn` contract, and a rule for what an unreferenced or missing asset means.

## Done when

- An ADR under `docs/adr/` records the decision and its trade-off, and names what
  would reopen it.
- If figures are kept: the shared specification gains the figure format, the
  asset convention, and what validation is obliged to check — stated precisely
  enough that `pipeline-skills-k28` can write the art kind's discipline against
  it and a later leaf can implement the validator support.
- If figures are dropped or stay unmachined: that is written down as the decision
  with the evidence behind it, and the art stage's fate at `pipeline-kinds-k27`
  follows from it.
- `bash scripts/check.sh` passes.

## Notes

**"Nothing is built" is a legitimate outcome and possibly the likely one.** A
design leaf can land an ADR that closes a path; that is why this is a design leaf
rather than a leaf that would have to be pruned.

**Do not implement validator support here.** If figures are kept, the
implementation is its own leaf — cut it, with its `--kind`, as your last act, and
place it where it precedes the book that first needs it.

## Decisions (running log)

**The measurable question is not "did art produce an asset".** It could not:
the preregistration's stage table charters art as *whatever figures, diagrams or
tables plain Markdown can carry, and nothing else*, so an absence of assets is a
construction of the charter rather than a reading of demand. The question the
pilot can answer is whether the art stage ever recorded a relation it declined to
draw **because Markdown could not carry it**. `4-art.md`'s `## Findings not
fixed` is that recording surface, it was used four times, and one of the four is
explicitly a case blocked by a rule (the `A2` in `source-index.md`, which the
book contract forbids fixing). So the surface demonstrably distinguishes *declined
on editorial judgement* from *forbidden*, and none of the four is ceiling-bound.
That is what converts the absence into evidence.

**No machinery is built.** No figure format, no asset convention in the manifest,
no validator concept of an asset. The measured demand is zero, the cost is not
incremental — the book contract's closed-directory rule is what makes `M101`
complete — and the cost of building later is additive behind a `schema` field
that exists for exactly that.

**The outcome is not "nothing", and that is the substantive finding.** The
pilot's `A1`/`A2` classes live in a frozen evaluation document that stops being
the live instrument the moment the pipeline is extracted as kinds. If nothing
lands in the book contract, `pipeline-skills-k28` has no durable rule to point the
art kind at. So the convention half of this leaf's question **is** answered
positively and at zero machinery cost: a `### Figures` subsection in the
specification's prose contract, which is author-and-reviewer applied by its own
framing.

**What reopens it is a recordable event, not a judgement.** An art session that
records a declined figure whose stated reason is the Markdown ceiling — rather
than an editorial judgement or a contract rule — is the trigger. Naming it that
way makes the reopening condition observable in the same surface that produced
this decision's evidence.

**The frozen evaluation documents are not edited.** `preregistration.md` and the
measurement `README.md` are the record of a measurement and say what was true
when they were written; their forward references to this leaf stay. The one
durable artifact carrying a stale forward reference is
`docs/specs/walkthrough-books.md`'s *Out of scope*, and it is rewritten to the
settled position.
