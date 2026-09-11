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

## Decisions (running log)

1. The documentation sweep covers tracked root files and dotfiles, docs, plugin
   prose, and source comments, with book fragments distinguished from prose.
   Published historical measurements (the v19.3.0 preservation transcript,
   release-tagged changelog entries, loop records and commit-pinned evaluation
   paths) remain evidence of their named revisions; they are not rewritten into
   fabricated measurements. Current-contract summaries and Unreleased describe
   only the implemented grammar. The preservation ledger will explicitly scope
   its naming assertions to that captured revision and link the current contract.
2. Graph verification is unavailable: `codebase-memory-mcp cli list_projects`
   refuses because a pre-coordination or unverified generation is active. Use
   direct source and executable fixtures; do not stop other sessions or claim
   graph coverage.
3. The user requested stopping the loop at this completion boundary to restart
   VSCode. After retirement and sealing, use `grove-llm complete --done` to stop
   the driver without selecting another task. This does not authorize finish
   teardown, release, installation or live-tree conversion.
4. The source change is documentation only: the `BRIEF` constant describes the
   root node file, paired byte-for-byte with its existing chapter-2 fragment.
   Source ranges and ledger line totals are unchanged. Test-module comments
   describe their actual fixtures; integration tests and the explicitly excluded
   `task_grow/tests.rs` are evidence, not reconstructed book roots.
5. The guide examples were exercised in a temporary jj repository using the
   built `target/debug/grove-llm`, isolated configuration and an empty signal
   channel. Initialization, pick, kind, root/nested chains, addition, insertion,
   decomposition, node key/slug/handle lookup, title edits, retirement and prune
   matched their expected paths. Missing root/node files, competing node files
   and misplaced `_BRIEF.md` refused with exit 1 and no partial chain.
6. The release-facing contract earns a separate `review-impl` before cutover.
   The review should challenge completeness beyond string replacement, including
   source/book prose agreement and the boundary drawn around revision evidence.
   No in-session reviewer is used beside that scheduled review.

## Verification evidence

- Enumerated filename/path tokens from 540 tracked text subjects, including
  root files and dotfiles, source and tests, docs, all six books and plugins;
  the inventory and subject digests are outside the repo under `/tmp/node-k9-*`.
  Classified occurrences as current prose/examples, source comments,
  source/fixtures, source fragments, model witnesses, format/provenance names
  and revision evidence. `BRIEF-FORMAT.md`, upstream `AGENT-BRIEF.md` and
  reference-domain names are not Grove node-file spellings.
- The remaining named-directory token in current source-comment candidates is
  `task_name.rs`'s intentional malformed `01-a-k1` diagnostic example. Historical
  captures are the explicit exception recorded above, not a clean sweep claim.
- A scratch subject using `_BRIEF.md` and `NN-k<key>/_topic.md` read clean;
  mutating it to `BRIEF.md` and `NN-<slug>-k<key>/` made the identical search
  fail that clean assertion. The same search read the guide clean and found
  the captured preservation transcript as its cross-tree control.
- Both glossary anchor sets match the parent commit. All six manifest corpus
  totals match actual source line counts; the book checks verify reconstruction.
- Final `bash scripts/check.sh` exited 0: all eight principal checks passed,
  including all 16 conformance controls, the locked workspace test suite and
  final validation of all six books with zero deferred lines. All 540 measured
  subjects matched their pre-validation SHA-256 digests afterward. Earlier
  interrupted runs were discarded, not credited as verification.
- Review `node-documentation-k22` is immediately before `node-cutover-k10`.
  The parent root still has live work; there is no node-close cascade.
