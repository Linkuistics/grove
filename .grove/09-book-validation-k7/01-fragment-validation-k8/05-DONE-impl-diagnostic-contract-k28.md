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

## Decisions (running log)

- Diagnostic values use one complete record shape for validation, invocation,
  and internal failures. Command-only null line and column values serialize
  from the record's zero sentinel; ordinary Markdown locations remain concrete.
- Graph reachability and expansion use iterative worklists. Byte expansion is
  bounded to the authoritative source length plus one byte, and structurally
  invalid roots are excluded while independent roots continue.
- Byte comparison emits at most one `F008` per source root. The record names the
  emitting literal fragment, owner, root-to-fragment path, first source byte and
  line, and expected/actual byte or EOF.
- CLI input enumeration is sorted before loading. The internal-failure boundary
  serializes panic-hook replacement so a caught invariant panic produces only
  the stable `I001` text or JSON record and no panic-hook stderr.
- `cargo fmt --check`, workspace clippy with warnings denied, and every
  `book-validation` test pass. The repository suite reaches only the pre-existing
  `relative/path` Markdown-reference failure owned by live sibling
  `reference-navigation-literal-k31`; two unrelated special-file setup tests
  are also denied by the execution sandbox before their assertions run.
