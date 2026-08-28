# diagnostic-contract-k28


## Goal

Complete the stable fragment diagnostic record, evidence, suppression, ordering,
rendering, and internal-failure boundary.

## Context

- Builds on `fragment-engine-k26` and `ledger-and-pages-k27`.
- Follow the Deterministic diagnostics and CLI JSON/text sections of
  `docs/specs/ordinal-fs-tree-book.md` exactly.
- The producer's first adversarial pass found missing `source`/`related`
  evidence, incomplete `F008` mismatch data, noncanonical ordering and cycles,
  misleading cascades after structural invalidity, and no `I001` boundary.

## Done when

- Every record has the complete nullable JSON schema, ordered related
  locations, actionable remedy, and required code-specific evidence.
- `F001`, `F004`, `F005`, `F008`, `U001`, `U002`, and `I001` fixtures assert the
  exact evidence their classes require.
- Invalid roots suppress misleading byte cascades while unrelated roots
  continue.
- Text and JSON outputs are byte-identical across repeat runs and follow the
  full total-order key, with exact schema assertions.
- Deep or adversarial graphs cannot stack-overflow or expand exponentially;
  internal invariant failures become exit 3 and `I001`.
- Crate and repository verification pass.

## Notes

Do not absorb Markdown diagnostic codes here; `markdown-validation-k9` owns
that phase and extends the same envelope later.
