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
- `pilot-measure-k26`'s report is the evidence, and it is the whole reason this
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
