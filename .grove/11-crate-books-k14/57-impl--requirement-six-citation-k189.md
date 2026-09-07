# requirement-six-citation-k189

## Goal

Repair the two broken citations in `crates/grove-loop/src/session_config.rs`
— line 271's *the search precedence requirement 6 fixes*, which names no record
anywhere in this repository, and line 331's Markdown link to
`../docs/adr/untracked-configuration-delta.md`, which resolves from nowhere —
inside the file's frozen 358-line count, and reconcile the pages that reproduce
the changed bytes.

## Context

- **The defect, and it is an address rather than a claim.** `find_delta`'s doc
  comment argues that at the worktree root a caller that moved on would *invert
  the search precedence requirement 6 fixes*. **There is no requirement 6.**
  `grep -rn 'requirement 6'` over the repository returns the comment, the book
  page that adjudicates it, and this leaf's antecedents in `.grove/` — no
  record. `docs/specs/module-decomposition.md`, the document the phrase points
  at, numbers its **decisions** 1 to 11 and gives its four requirements names
  rather than numbers.
- **Read as *decision* 6 it still does not fit.** That decision moves the
  completeness quantifier and states that an overlay overrides and never
  supplies; it says nothing about which of two roots is searched first — at the
  revision the comment was written as much as today.
- **The likely target is `untracked-configuration-delta`.** That record fixes
  the search precedence and states this very absence rule in almost these
  words, and the same file cites it in prose twice elsewhere, at lines 15 and
  28. Check it
  is still the best target before retyping it: k157's and k158's finding was
  that a comment's cited section can hold less than the sentence leans on it
  for. This is the `loop-driver-kill-anchor-k176`, `lease-stale-reader-name-k169`
  and `prompt-rule-id-prefix-k174` shape — the rule the sentence describes is
  right and only its address is wrong.
- **The second defect is the same record, cited as a link instead of a path.**
  `delta_is_tracked`'s doc comment at line 331 cites *the untracked
  configuration delta* as a Markdown link to
  `../docs/adr/untracked-configuration-delta.md`. Every other citation in that
  block — six of them — is a backticked path in prose, which is this file's
  convention and is stable. The link resolves from nowhere: rustdoc emits the
  target verbatim, so from the rendered page it points inside the generated
  documentation tree where no `docs/adr/` exists, and read relative to the
  source file it would be `crates/grove-loop/docs/adr/`, which does not exist
  either. `cargo doc` reports nothing, because it checks intra-doc links and
  never an explicit URL target. The record it names is real and says what the
  comment says it says; only the address is broken. Adjudicated at
  `18-which-files.md`'s *And one more citation to adjudicate* and assigned to
  `template-source-read-count-k86` by `which-files-k166`; k86 externalised it
  here with its sibling rather than absorbing either.
- **No instrument sees it.** It is not an intra-doc link, so `cargo doc` is
  silent; not a Markdown link, so no link sweep reads it; not an `ADR <slug>`
  citation, so `every_adr_citation_names_a_decision_record` does not read it.
  It was found by enumerating the file's parenthesised and prose citations
  while `which-files-k166` drafted chapter 18.
- **One page reproduces the changed bytes** and must change in the same commit:
  `docs/walkthroughs/grove-loop/18-which-files.md`, fragments
  `«config-find-delta»` (lines 261-293) and the fence whose range contains 331.
  Two sections of that chapter adjudicate the two addresses in front of the
  reader — *One adjudication is owed here* and *And one more citation to
  adjudicate* — and both must be rewritten to describe the repaired citations
  rather than deleted; check `concept-index.md` for entries naming either, and
  `docs/specs/grove-loop-book-structure.md` for a chapter-18 pointer.
- **Cut at `template-source-read-count-k86`**, which the chapter had nominated
  to carry both fixes because it was already editing this file's comments within
  the frozen line counts. Neither was that leaf's stated goal, so the work was
  externalised here rather than absorbed, and k86 re-pointed both of the
  chapter's sentences at this leaf.

## Done when

- The comment names a record this repository carries, and no citation in
  `crates/grove-loop/src/session_config.rs` names something that resolves to
  nothing.
- **`crates/grove-loop/src/session_config.rs` is still exactly 358 lines**, and
  the comment still occupies its existing line span, so no ownership range,
  manifest `lines` value or fragment range moves. The substitution is longer
  than `requirement 6`, so the paragraph will need rewrapping *within* its
  existing span.
- `18-which-files.md` reproduces the new bytes and both of its adjudications
  are rewritten into explanations of the corrected wording rather than deleted.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is
  green, and every other book the commit touched is green too.
- `bash scripts/check.sh` passes.

## Notes

**The freeze holds.** One commit carries the source change, every affected
ledger and page, and a green validator run over every book it touched. The fix
changes no line count, so no ownership range moves.
