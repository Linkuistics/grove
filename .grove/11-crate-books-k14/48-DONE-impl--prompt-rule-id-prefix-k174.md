# prompt-rule-id-prefix-k174

## Goal

Repair one broken rule-id citation in `crates/grove-loop/src/prompt.rs` line 234
— `skill-stated-vcs-is-definitive`, which names no rule — inside the file's
frozen 245-line count, and reconcile the page that reproduces the changed bytes.

## Context

- **The defect.** `stated_vcs`'s doc comment argues that *do not probe for the
  version control* is a normative consequence of a value and so belongs to the
  methodology, and attributes the rule to *the spine's
  `skill-stated-vcs-is-definitive`*. The methodology's rule inventory is
  `plugins/grove/conformance/rules.tsv`, whose header states that the id is
  *unique across the whole file* over 169 rows. The rule is there as
  **`stated-vcs-is-definitive`**, owner `grove/SKILL.md`. No id in the file begins
  with `skill-`, and `grep -rn 'stated-vcs-is-definitive'` over the repository
  returns exactly two files: that inventory, and this comment.
- **The claim is true twice over and only the identifier is wrong**, which is the
  `paths-k142` and `lease-stale-reader-name-k169` shape. The rule exists, and the
  spine states it in prose at `plugins/grove/skills/grove/SKILL.md` — *The stated
  VCS is definitive. Do not re-derive which lane this working tree is on, and
  disregard a harness banner that disagrees.* So the repair is the prefix, not the
  sentence.
- **Check the intended grain before retyping the id.** The inventory's own header
  records that the owner column's grain *used to be skill-relative* and was
  changed once every kind gained a file called `SKILL.md`. So `skill-` may be
  residue of that earlier scheme rather than a typo, and the repair should say
  whether any other citation in the workspace still spells an id that way before
  assuming this is the only one.
- **No instrument in this repository sees it, and that is the reason it survived.**
  It is not an intra-doc link, so `cargo doc --no-deps --document-private-items`
  is silent — the crate's thirty warnings name `prompt.rs` once, at line 28. It is
  not a Markdown link, so no link sweep reads it. It is not an `ADR <slug>`
  citation, so `every_adr_citation_names_a_decision_record` does not read it
  either. It was found by enumerating the block's backticked tokens and resolving
  each one.
- **One page reproduces the changed bytes** and must change in the same commit:
  `docs/walkthroughs/grove-loop/19-the-core.md`, fragment `«core-stated-vcs»`
  (lines 223-245). Its *The value, and the clause that went to the skill* section
  adjudicates the id in front of the reader and must be rewritten to describe the
  repaired citation; check `concept-index.md` for the entry naming it.

## Done when

- The comment names an id that `plugins/grove/conformance/rules.tsv` carries, and
  no citation in `crates/grove-loop/src/` spells a rule id that the inventory does
  not hold.
- **`crates/grove-loop/src/prompt.rs` is still exactly 245 lines**, and the
  comment still occupies its existing line span, so no ownership range, manifest
  `lines` value or fragment range moves. Dropping a six-character prefix cannot
  lengthen the line.
- `19-the-core.md` reproduces the new bytes and its adjudication is rewritten to
  the repaired text.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  over the book at whatever slice it is proved at when this runs, and every other
  book the commit touched is green too.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** The bytes it
changes are reproduced by chapter 19, and the freeze rule requires one commit to
carry the source change, every affected ledger and page, and a green validator
run. Run it after `grove-loop-k123`'s last child has taken the book to green
`--final` validation, alongside `unresolved-doc-links-k151`, which repairs the
other broken address in this same root — line 28's `[`crate::methodology`]` — and
could reasonably be run in the same session. They are separate leaves because
k151's `Done when` is *no `unresolved link` warning*, which this defect does not
produce and that check would not close.

**Do not widen this into a sweep of the file's citations.** All eighteen were
enumerated at `the-core-k167` — every citation site in lines 1 to 245, its target
opened and its section read — and seventeen resolve and hold what the sentence
leans on them for. This was the only one that did not.

## Decisions (running log)

1. **The repair is the prefix alone.** `crates/grove-loop/src/prompt.rs:234` now
   cites `stated-vcs-is-definitive`, which
   `plugins/grove/conformance/rules.tsv` carries at line 76 owned by
   `grove/SKILL.md`. The sentence around it is untouched: the spine states the
   rule in prose, so the claim was sound and only the address was wrong — the
   `lease-stale-reader-name-k169` shape.
2. **The `skill-` prefix is not the inventory's recorded grain change, and the
   task file's caution was worth spending.** The tsv header's regrading is of the
   **owner** column (skill-relative → shipped-set-relative, once every kind owned
   a `SKILL.md`); it says nothing about the rule column, and 0 of the 169 ids
   begin with `skill-`. The prefix belongs to a different namespace the changelog
   still carries — the composed methodology's units, `skill-bootstrap`,
   `skill-do-not-pick-again`, `skill-finish-cycle`, from before `content/` was
   deleted at `delete-provisioning-k19` (jj `833c7a9b5760`). Their stems differ
   from today's rule ids for the same rules (`bootstrap-order`, `no-second-pick`),
   so the broken token was a unit prefix over a rule stem and named nothing under
   either scheme.
3. **Enumerated rather than assumed, and the first wording was wrong.** Every
   backticked kebab-case token under `crates/*/src` (64 of them) resolved against
   the 169 ids: **this was the only rule-id citation in any crate's `src/`**, and
   the only token anywhere in the workspace's `.rs` that becomes an id when a
   leading segment is dropped. A draft sentence on the page generalised that to
   *the workspace's Rust*, which is false — `composition_guidance.rs`,
   `commit_guidance.rs` and `plugin_fallback.rs` cite ten ids between them, all
   of which resolve. The page states the `src/` scope and the suites separately.
4. **No line moved.** Dropping six characters left `prompt.rs` at 245 lines and
   line 234 at 75 columns; no ownership range, manifest `lines` value or fragment
   range changed. The short line was left unreflowed — filling it would either
   collapse the four-line paragraph to three or push a line past eighty.
5. **Three surfaces, per the fan-out this campaign has measured.** The source
   literal; the `«core-stated-vcs»` fragment in
   `docs/walkthroughs/grove-loop/19-the-core.md` (the only surface `book-check`
   expands); and the prose that argued *from* the defect in that chapter's *The
   value, and the clause that went to the skill*, rewritten to past tense on
   chapter 16's `probe_lease_holder` precedent. `concept-index.md`'s entry keeps
   the concept and moves to past tense. No rendered-output sample and no other
   book reproduces these bytes, and no line count moved, so no assembly-chapter
   transcript is affected.
6. **Green.** `book-check --final --check all` passes over all six books
   (`grove-loop`: 13 files, 10,557 resolved lines), and `bash scripts/check.sh`
   reports *all 8 principal checks pass*.
