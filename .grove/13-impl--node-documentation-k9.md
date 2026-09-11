# node-documentation-k9

## Goal

Leave the user guide, remaining repository documentation and book explanations
current and coherent for the implemented node-file grammar, ready to release.

## Context

- Depends on `node-methodology-k8`. Source-owning leaves have already updated
  the books they changed; this leaf finishes reader-facing prose and examples.
- Entry points: `docs/USAGE.md`, README and architecture summaries, guide
  coverage and book-structure specs, preservation notes, the two glossaries
  and context map, and every walkthrough's prose, indexes and manifest claims.
  Enumerate the relevant surface rather than treating these as a complete list.
- The ADR/spec/glossary decisions landed in `node-grammar-k2` and
  `node-grammar-k5`. Reconcile residual contradictions without duplicating the
  canonical grammar or rewriting the decisions' scope.

## Done when

- The guide explains the directory/file distinction, mandatory root and node
  files, node handles, decomposition, lookup, brief chains and actionable
  malformed-tree refusals. Examples are executable against the built binary.
  No text implies that a title is read from a body or retained in a directory.
- Every book's prose and examples agree with the implementation, including
  `overview` and books whose source roots did not change. Glossary anchors and
  existing links are preserved. Source-fragment corrections remain paired
  with their actual source; no book repair is deferred to release.
- Grammar-bearing statements, examples and summaries in the claimed surface
  are enumerated and classified. The sweep includes root files and dotfiles
  where relevant, with positive and cross-tree controls exercised on a known
  wrong subject. Do not blindly replace `_BRIEF.md`, `BRIEF-FORMAT.md` or
  reference-domain distinguished filenames.
- Shipped docs, ADRs, glossary prose and code comments state current naming
  only. Preserve intentional malformed-input fixtures and model witnesses as
  such; do not erase their ability to catch refusals. Process records remain
  in this task tree rather than becoming a durable migration guide.
- `CHANGELOG.md` describes the shipped behavior and requirement for a matching
  plugin/binary without narrating naming history. Any release-procedure edits
  agree with the human cutover and stopped-other-groves protocol.
- Guide examples are checked in isolated fixtures and `bash scripts/check.sh`
  passes, including final validation of every book. Any further source change
  carries its matching fragments, ledger and explanation in this commit.

## Notes

This increment is useful as the human-readable contract for the working binary
and plugin source. It installs nothing and renames no live task tree. Keep
temporary migration tooling and inventories outside the published codebase.
Insert any required review before `node-cutover-k10`; release consumes the
finished, checked corpus.
