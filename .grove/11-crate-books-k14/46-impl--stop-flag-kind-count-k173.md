# stop-flag-kind-count-k173

## Goal

Repair two stale session-kind counts in test comments — `crates/grove-loop/tests/prompt.rs`
line 289 and `crates/grove-llm/tests/kind.rs` line 2 — and reconcile the one book
page that adjudicates the first of them.

## Context

- **The defect in `tests/prompt.rs`.** `no_prompt_states_the_stop_flag`'s doc
  comment says a prompt naming `--done` *would hand the flag to the eighteen kinds
  it is not an ending for*. Enumerated today, `plugins/grove/skills/` holds
  twenty-four directories, twenty-three of them `grove-<kind>`, so the flag would
  reach **twenty-two**. Nineteen is the count `crates/grove-loop/src/prompt.rs`'s
  own header records twice — the nineteen-to-ten reference map and the
  nineteen-to-two ending map `prompt-names-the-kind-k18` deleted — so the comment
  was correct when written and the shipped set has grown by four since.
- **The second site is worse in two directions.** `crates/grove-llm/tests/kind.rs`
  line 2 says `kind` *separates one of the closed nineteen kinds from the leaf
  filename*. The count is stale the same way, and **`closed` is stale as a claim
  about the design**: `open-kind-k20` made a kind an open token, and
  `docs/adr/a-kind-is-an-open-token.md` is the record. The same ADR is already
  quoted against a sibling instance in
  `docs/adr/corpus-rules-have-one-owner.md` line 151, which parenthesises an
  earlier *all nineteen kinds* as a thing that was corrected.
- **Neither test is affected, and that is the shape of the defect.**
  `no_prompt_states_the_stop_flag` iterates `shipped_kinds()`, which strips the
  prefix off the plugin's own directory listing, so the assertion has been
  covering twenty-three kinds while its prose said eighteen. Only the comments are
  wrong.
- **Both files are `tests/` — evidence, not roots.** No book reproduces either
  byte, so no ownership range, manifest `lines` value or fragment range moves, and
  the freeze is not in play for the source change itself.
- **One page adjudicates the first site and must change with it**:
  `docs/walkthroughs/grove-loop/19-the-core.md`, the *Part 3 — one text for every
  kind* section, whose closing paragraph states the count as stale and names this
  leaf. Rewrite it to describe the repaired comment, and check
  `concept-index.md` for the entry naming it.
- **A count of the installed set does not belong in a comment at all**, which is
  the option worth weighing before retyping a number. Both sentences read
  correctly with the count removed — *the kinds it is not an ending for*, and *a
  session kind* — and a sentence with no count cannot go stale the next time the
  plugin grows. Prefer that shape unless the number is doing work the prose needs.

## Done when

- Neither comment states a count that the shipped set contradicts, and
  `kind.rs`'s no longer calls the set closed.
- `19-the-core.md`'s adjudicating paragraph describes the repaired comment, and
  `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  over the book at whatever slice it is proved at when this runs.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** Not for the
freeze — neither file is a root — but because chapter 19 adjudicates the first
site in front of the reader, and a fix landing before the page is finished would
leave the page describing a comment that no longer exists. Run it after
`grove-loop-k123`'s last child has taken the book to green `--final` validation,
alongside `unresolved-doc-links-k151`, `prompt-rule-id-prefix-k174`,
`unreachable-root-clause-k152`, `lease-stale-reader-name-k169` and
`grow-header-stale-helper-k154`.

**Do not widen this into a sweep of the workspace's kind counts.** These two were
found by enumerating what chapter 19's block and its evidence cite, not by a
search for the word *nineteen* — and a search is the wrong instrument here,
because the counts are spelled as English words and wrap across lines. If a sweep
is wanted it is its own leaf, and its unit is *every comment stating a cardinality
of the shipped kind set*, enumerated from the directory rather than from a
pattern.

## Decisions (running log)
